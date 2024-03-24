use clap::{error::ErrorKind, ArgMatches, Command, Error};
use reqwest::get;
use serde::Deserialize;

use crate::{
    config::config_manager::{self, ConfigManager},
    utils::utils::get_shinkai_node_binaries_versions_url,
};

#[derive(Deserialize)]
struct Versions {
    pub latest: String,
    pub versions: Vec<String>,
}

pub async fn list_remote(
    command: &Command,
    _sub_matches: &ArgMatches,
    mut config_manager: ConfigManager,
) {
    println!("⌛ Fetching versions from remote...\n");
    let versions_definition_response = get(get_shinkai_node_binaries_versions_url()).await;
    let versions_definition = match versions_definition_response {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<Versions>().await {
                    Ok(versions) => versions,
                    Err(e) => {
                        let error =
                            Error::raw(ErrorKind::Io, format!("❌ Error parsing versions: {}", e))
                                .with_cmd(command);
                        error.exit();
                    }
                }
            } else {
                let error = Error::raw(
                    ErrorKind::Io,
                    format!("❌ Failed to fetch versions. Status: {}", response.status()),
                )
                .with_cmd(command);
                error.exit();
            }
        }
        Err(e) => {
            let error = Error::raw(ErrorKind::Io, format!("❌ Error fetching versions: {}", e))
                .with_cmd(command);
            error.exit();
        }
    };

    let current_version = config_manager
        .get_config()
        .current
        .unwrap_or("".to_string());

    for version in versions_definition.versions.iter() {
        let current_version_tag = if &current_version == version {
            "(👉current)".to_string()
        } else {
            "".to_string()
        };
        let latest_version_tag: String = if &versions_definition.latest == version {
            "(✨latest)".to_string()
        } else {
            "".to_string()
        };
        println!("🏷️ {} {}{}", version, latest_version_tag, current_version_tag);
    }
    println!("\n💡 Install any of these versions running:");
    println!("\t 'kaivm install [version]'");
}
