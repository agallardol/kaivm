use clap::{error::ErrorKind, ArgMatches, Command, Error};
use reqwest::get;
use serde::Deserialize;

use crate::{
    config::config_manager::ConfigManager, utils::utils::get_shinkai_node_binaries_versions_url,
};

#[derive(Deserialize)]
struct Versions {
    pub latest: String,
    pub versions: Vec<String>,
}

pub async fn list_remote(
    command: &Command,
    _sub_matches: &ArgMatches,
    mut _config_manager: ConfigManager,
) {
    println!("⌛ Fetching versions from remote...\n");
    let versions_definition_response = get(get_shinkai_node_binaries_versions_url()).await;
    let mut versions_definition = match versions_definition_response {
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

    for version in versions_definition.versions.iter() {
        if version == &versions_definition.latest {
            println!("🏷️ {} (✨latest)", version);
        } else {
            println!("🏷️ {}", version);
        }
    }
    println!("\n💡 Install any of these versions running:");
    println!("\t 'kaivm install [version]'");
}
