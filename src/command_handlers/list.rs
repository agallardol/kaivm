use std::fs;

use clap::{ArgMatches, Command};

use crate::{config::config_manager::ConfigManager, utils::utils::get_versions_folder_path};

pub fn list(_command: &Command, _sub_matches: &ArgMatches, mut _config_manager: ConfigManager) {
    println!("⌛ Fetching installed versions...\n");
    let versions_path = get_versions_folder_path();
    match fs::read_dir(versions_path) {
        Ok(entries) => {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_dir() {
                    let folder_name = path.file_name().unwrap().to_str().unwrap();
                    println!("🏷️ {} (📂 {})", folder_name, path.display());
                }
            }
        }
        Err(_) => println!("❌ No versions installed"),
    }
}
