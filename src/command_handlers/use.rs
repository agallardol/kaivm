use clap::{ArgMatches, Command};

use crate::{config::config_manager::ConfigManager, utils::utils::get_version_binary_file_path};

pub fn r#use(_command: &Command, sub_matches: &ArgMatches, mut config_manager: ConfigManager) {
    let version = sub_matches.get_one::<String>("VERSION").expect("required");

    let shinkai_node_binary_file_path = get_version_binary_file_path(version);
    if !std::path::Path::new(&shinkai_node_binary_file_path).exists() {
        let error = clap::Error::raw(
            clap::error::ErrorKind::InvalidValue,
            format!("The specified version '{}' is not installed. Please check the version number or install it using 'kaivm install {}'.", version, version),
        );
        error.exit();
    }

    let mut config = config_manager.get_config();
    config.current = Some(version.clone());
    config_manager.write_config(&config.clone());

    println!("\nCurrent version set to {}", version);
}
