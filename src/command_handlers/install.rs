use crate::{
    config::config_manager::ConfigManager,
    utils::utils::{
        get_shinkai_node_binary_url, get_version_binary_file_path, get_version_folder_path,
    },
};
use clap::{error::ErrorKind, ArgMatches, Command};
use futures_util::stream::StreamExt;
use std::io::Write;

pub async fn install(command: &Command, sub_matches: &ArgMatches, _config_manager: ConfigManager) {
    let version = sub_matches.get_one::<String>("VERSION").expect("required");

    let version_folder_path = get_version_folder_path(version);
    if !std::path::Path::new(&version_folder_path).exists() {
        if let Err(e) = std::fs::create_dir_all(&version_folder_path) {
            let error = clap::Error::raw(
                ErrorKind::Io,
                format!("❌ Failed to create version directory: {}", e),
            )
            .with_cmd(command);
            error.exit();
        }
    }

    let url = get_shinkai_node_binary_url(version);
    let target_path = get_version_binary_file_path(version);

    let response = match reqwest::get(&url).await {
        Ok(resp) => {
            if resp.status() != 200 {
                let mut error = clap::Error::raw(
                    ErrorKind::InvalidValue,
                    format!("❌ The specified version '{}' is not available for download. Please check the version number and try again.", version.clone()),
                ).with_cmd(command);
                error.insert(
                    clap::error::ContextKind::InvalidArg,
                    clap::error::ContextValue::String("VERSION".to_owned()),
                );
                error.exit();
            }
            resp
        }
        Err(e) => {
            let error =
                clap::Error::raw(ErrorKind::Io, format!("❌ Failed to download file: {}", e))
                    .with_cmd(command);
            error.exit();
        }
    };

    let total_size = response.content_length().unwrap_or(0);
    let total_size_mb = total_size as f64 / 1024.0 / 1024.0;

    println!("⏳ Pulling version:{} size:{:.2}MB", version, total_size_mb);
    let mut file = match std::fs::File::create(&target_path) {
        Ok(f) => f,
        Err(e) => {
            let error = clap::Error::raw(
                ErrorKind::Io,
                format!("❌ Failed to create file at {}: {}", target_path, e),
            )
            .with_cmd(command);
            error.exit();
        }
    };
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.expect("Error while reading chunk");
        file.write_all(&chunk).expect("Error while writing to file");
        downloaded += chunk.len() as u64;
        let percentage = if total_size > 0 {
            downloaded as f64 / total_size as f64 * 100.0
        } else {
            0.0
        };
        print!("\r⬇️  Downloading {:.2}%", percentage);
        std::io::stdout().flush().unwrap();
    }
    println!(
        "\n✅ Version {} has been downloaded and stored at 📂 {}",
        version, target_path
    );

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = std::fs::metadata(&target_path).expect("Failed to read metadata");
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o755); // rwxr-xr-x
        std::fs::set_permissions(&target_path, permissions).expect("Failed to set permissions");
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        use winapi::um::winnt::FILE_ATTRIBUTE_READONLY;
        let mut options = std::fs::OpenOptions::new();
        options.write(true).attributes(FILE_ATTRIBUTE_READONLY);
        let file = options.open(&target_path).expect("Failed to open file");
        file.sync_all().expect("Failed to sync file");
    }

    let mut config_manager = ConfigManager::new();
    let mut config = config_manager.get_config();
    if config.current.unwrap_or_default().is_empty() {
        config.current = Some(version.clone());
        config_manager.write_config(&config.clone());
    }
}
