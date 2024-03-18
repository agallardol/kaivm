use clap::{ArgMatches, Command};

use crate::config::config_manager::ConfigManager;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn version(_command: &Command, _sub_matches: &ArgMatches, config_manager: ConfigManager) {
    let current_shinkai_node_version = if config_manager.get_config().current.is_some() {
        config_manager.get_config().current.unwrap()
    } else {
        "unset".to_string()
    };

    println!("kaivm: {}", VERSION);
    println!("Shinkai Node: {}", current_shinkai_node_version);
}
