use clap::ArgMatches;

use crate::{config::config_manager::ConfigManager, utils::utils::get_version_binary_file_path};
use std::process::{Command, Stdio};

fn ensure_ollama_models(initial_agent_models: &Vec<String>) {
    let ollama_list = Command::new("ollama").arg("list").output();
    if ollama_list.is_err() {
        let error = clap::Error::raw(
            clap::error::ErrorKind::Io,
            "ollama is not installed. Please install ollama to proceed: https://ollama.com/download",
        );
        error.exit();
    }
    let ollama_list_output_text = String::from_utf8(ollama_list.unwrap().stdout).unwrap();
    for agent_model in initial_agent_models {
        if let Some(model) = agent_model.strip_prefix("ollama:") {
            if !ollama_list_output_text.contains(&model.to_string()) {
                println!("ollama model '{}' is not installed.", model);
                println!("auto installing it using 'ollama pull {}'...", model);
                let ollama_pull = Command::new("ollama").arg("pull").arg(model).output();
                let ollama_pull_is_error = if let Ok(output) = ollama_pull {
                    let output_text = String::from_utf8(output.stderr).unwrap_or_default();
                    output_text.contains("Error:")
                } else {
                    true
                };
                if ollama_pull_is_error {
                    let error = clap::Error::raw(
                    clap::error::ErrorKind::Io,
                    format!("\nError installing ollama model '{}'.\nPlease ensure you can install it manually using 'ollama pull {}'", model, model),
                    );
                    error.exit();
                }
            }
        }
    }
}
pub fn run(_command: &clap::Command, _sub_matches: &ArgMatches, config_manager: ConfigManager) {
    let config = config_manager.get_config();
    if config.current.is_none() {
        let error = clap::Error::raw(
            clap::error::ErrorKind::InvalidValue,
            "No current Shinkai Node version is set. Please use 'kaivm use <VERSION>' to set a current version.",
        );
        error.exit();
    }
    let shinkai_node_binary_file_path = get_version_binary_file_path(&config.current.unwrap());
    if !std::path::Path::new(&shinkai_node_binary_file_path).exists() {
        let error = clap::Error::raw(
            clap::error::ErrorKind::InvalidValue,
            format!(
                "Shinkai Node '{}' is not installed. Please use 'kaivm install <VERSION>'",
                shinkai_node_binary_file_path
            ),
        );
        error.exit();
    }
    let initial_agent_models = config
        .shinkai_node_env
        .clone()
        .unwrap()
        .initial_agent_models
        .unwrap()
        .split(',')
        .map(|s| s.to_string())
        .collect();
    ensure_ollama_models(&initial_agent_models);

    let mut binding = Command::new(&shinkai_node_binary_file_path);
    let command = binding.stdout(Stdio::inherit());

    let options_reflection =
        serde_json::to_value(config.shinkai_node_env.unwrap().clone()).unwrap();
    println!("Running Shinkai Node with envs:");
    for (key, value) in options_reflection.as_object().unwrap() {
        let env_key = key.to_uppercase();
        let env_value = value.as_str().unwrap_or_default().to_string();
        println!("\t{}:{}", env_key.clone(), env_value.clone());
        command.env(env_key, env_value);
    }

    let mut child = command.spawn().expect("Failed to execute Shinkai Node");

    let exit_status = child.wait().expect("Failed to wait on child");

    if !exit_status.success() {
        let error = clap::Error::raw(
            clap::error::ErrorKind::Io,
            format!("Shinkai Node process exited with error: {}", exit_status),
        );
        error.exit();
    }
}
