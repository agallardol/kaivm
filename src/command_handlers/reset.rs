use std::fs;

use clap::{ArgMatches, Command};

use crate::config::config_manager::ConfigManager;

pub fn reset(_command: &Command, sub_matches: &ArgMatches, config_manager: ConfigManager) {
    let confirm = sub_matches.get_one::<String>("confirm");
    let node_storage_path = config_manager
        .get_config()
        .shinkai_node_env
        .unwrap()
        .node_storage_path
        .unwrap()
        .clone();
    if confirm.is_none() || confirm.unwrap().is_empty() {
        println!("🚨 It will delete all files inside node_storage_path folder");
        println!("\t📂 {}", node_storage_path);
        println!("🚨 Also, you won't be able to connect with your current keys so you will need to connect again all your apps and devices");
        println!("\n❓ Are you sure you want to reset your Shinkai Node?");
        println!(
            "\t💡 If yes, execute this command again using 'kaivm node reset --confirm=\"true\"'"
        );
        return;
    }
    match fs::remove_dir_all(node_storage_path.clone()) {
        Ok(_) => {
            println!("\n✅ Your Shinkai Node storage was resetted");
        }
        Err(_) => {
            let error = clap::Error::raw(
            clap::error::ErrorKind::Io,
            format!("❌ Failed to reset the Shinkai Node storage. Are you sure the folder exists? (📂 {})", node_storage_path),
        );
            error.exit();
        }
    }
}
