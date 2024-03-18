use clap::{ArgMatches, Command};

use crate::config::config_manager::{ConfigManager, ShinkaiNodeEnv};

fn print_config(shinkai_node_env: &ShinkaiNodeEnv) {
    let options_reflection = serde_json::to_value(shinkai_node_env).unwrap();
    for (key, value) in options_reflection.as_object().unwrap() {
        let env_key = key.clone();
        let env_value = value.as_str().unwrap_or_default().to_string();
        println!(
            "\t{}: {}",
            env_key,
            env_value
        );
    }
}

pub fn env(_command: &Command, sub_matches: &ArgMatches, mut config_manager: ConfigManager) {
    let mut config = config_manager.get_config();

    if !sub_matches.args_present() {
        println!("Current config:");
        print_config(&config.shinkai_node_env.unwrap());
        return;
    }

    let existing_port = config
        .shinkai_node_env
        .as_ref()
        .unwrap()
        .node_api_port
        .as_ref()
        .unwrap()
        .clone();
    let existing_node_storage_path = config
        .shinkai_node_env
        .as_ref()
        .unwrap()
        .node_storage_path
        .as_ref()
        .unwrap()
        .clone();
    let existing_unstructured_server_url = config
        .shinkai_node_env
        .as_ref()
        .unwrap()
        .unstructured_server_url
        .as_ref()
        .unwrap()
        .clone();
    let existing_embeddings_server_url = config
        .shinkai_node_env
        .as_ref()
        .unwrap()
        .embeddings_server_url
        .as_ref()
        .unwrap()
        .clone();
    let existing_first_device_needs_registration_code = config
        .shinkai_node_env
        .as_ref()
        .unwrap()
        .first_device_needs_registration_code
        .as_ref()
        .unwrap()
        .clone();
    let existing_initial_agent_names = config
        .shinkai_node_env
        .as_ref()
        .unwrap()
        .initial_agent_names
        .as_ref()
        .unwrap()
        .clone();
    let existing_initial_agent_urls = config
        .shinkai_node_env
        .as_ref()
        .unwrap()
        .initial_agent_urls
        .as_ref()
        .unwrap()
        .clone();
    let existing_initial_agent_models = config
        .shinkai_node_env
        .as_ref()
        .unwrap()
        .initial_agent_models
        .as_ref()
        .unwrap()
        .clone();
    let existing_initial_agent_api_keys = config
        .shinkai_node_env
        .as_ref()
        .unwrap()
        .initial_agent_api_keys
        .as_ref()
        .unwrap()
        .clone();
    let existing_starting_num_qr_devices = config
        .shinkai_node_env
        .as_ref()
        .unwrap()
        .starting_num_qr_devices
        .as_ref()
        .unwrap()
        .clone();
    config.shinkai_node_env = Some(ShinkaiNodeEnv {
        node_api_port: Some(
            sub_matches
                .get_one::<String>("node_api_port")
                .cloned()
                .unwrap_or(existing_port),
        ),
        node_storage_path: Some(
            sub_matches
                .get_one::<String>("node_storage_path")
                .cloned()
                .unwrap_or(existing_node_storage_path),
        ),
        unstructured_server_url: Some(
            sub_matches
                .get_one::<String>("unstructured_server_url")
                .cloned()
                .unwrap_or(existing_unstructured_server_url),
        ),
        embeddings_server_url: Some(
            sub_matches
                .get_one::<String>("embeddings_server_url")
                .cloned()
                .unwrap_or(existing_embeddings_server_url),
        ),
        first_device_needs_registration_code: Some(
            sub_matches
                .get_one::<String>("first_device_needs_registration_code")
                .cloned()
                .unwrap_or(existing_first_device_needs_registration_code),
        ),
        initial_agent_names: Some(
            sub_matches
                .get_one::<String>("initial_agent_names")
                .cloned()
                .unwrap_or(existing_initial_agent_names),
        ),
        initial_agent_urls: Some(
            sub_matches
                .get_one::<String>("initial_agent_urls")
                .cloned()
                .unwrap_or(existing_initial_agent_urls),
        ),
        initial_agent_models: Some(
            sub_matches
                .get_one::<String>("initial_agent_models")
                .cloned()
                .unwrap_or(existing_initial_agent_models),
        ),
        initial_agent_api_keys: Some(
            sub_matches
                .get_one::<String>("initial_agent_api_keys")
                .cloned()
                .unwrap_or(existing_initial_agent_api_keys),
        ),
        starting_num_qr_devices: Some(
            sub_matches
                .get_one::<String>("starting_num_qr_devices")
                .cloned()
                .unwrap_or(existing_starting_num_qr_devices),
        ),
    });
    config_manager.write_config(&config);

    println!("Config updated successfully:");
    print_config(&config.shinkai_node_env.unwrap());
}
