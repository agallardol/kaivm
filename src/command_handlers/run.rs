use clap::ArgMatches;

use crate::{config::config_manager::ConfigManager, utils::utils::get_version_binary_file_path};
use std::process::{Command, Stdio};

pub fn run(_command: &clap::Command, _sub_matches: &ArgMatches, config_manager: ConfigManager) {
    let config = config_manager.get_config();
    if config.current.is_none() {
        let error = clap::Error::raw(
            clap::error::ErrorKind::InvalidValue,
            "No current Shinkai Node version is set. Please use 'svm use <VERSION>' to set a current version.",
        );
        error.exit();
    }
    let shinkai_node_binary_file_path = get_version_binary_file_path(&config.current.unwrap());
    if !std::path::Path::new(&shinkai_node_binary_file_path).exists() {
        let error = clap::Error::raw(
            clap::error::ErrorKind::InvalidValue,
            format!(
                "Shinkai Node '{}' is not installed. Please use 'svm install <VERSION>'",
                shinkai_node_binary_file_path
            ),
        );
        error.exit();
    }

    let mut binding = Command::new(&shinkai_node_binary_file_path);
    let command = binding.stdout(Stdio::inherit());

    let options_reflection = serde_json::to_value(config.shinkai_node_env).unwrap();
    for (key, value) in options_reflection.as_object().unwrap() {
        let env_key = key.to_uppercase();
        let env_value = value.as_str().unwrap_or_default().to_string();
        println!("starting with {}:{}", env_key.clone(), env_value.clone());
        command.env(env_key, env_value);
    }

    let mut child = command
        .spawn()
        .expect("Failed to execute Shinkai Node");

     let exit_status = child.wait().expect("Failed to wait on child");


    if !exit_status.success() {
        let error = clap::Error::raw(
            clap::error::ErrorKind::Io,
            format!("Shinkai Node process exited with error: {}", exit_status),
        );
        error.exit();
    }
}
