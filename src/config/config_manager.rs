use serde::{Deserialize, Serialize};
use std::fs;
use toml;

use crate::utils::utils::{get_config_file_path, get_config_folder_path, get_default_shinkai_node_storage_path};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub current: Option<String>,
    pub shinkai_node_env: Option<ShinkaiNodeEnv>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ShinkaiNodeEnv {
    pub node_api_port: Option<String>,
    pub node_storage_path: Option<String>,
    pub unstructured_server_url: Option<String>,
    pub embeddings_server_url: Option<String>,
    pub first_device_needs_registration_code: Option<String>,
    pub initial_agent_names: Option<String>,
    pub initial_agent_urls: Option<String>,
    pub initial_agent_models: Option<String>,
    pub initial_agent_api_keys: Option<String>,
    pub starting_num_qr_devices: Option<String>,
}

pub struct ConfigManager {
    config: Config,
}

impl ConfigManager {
    pub fn new() -> Self {
        let config = Self::init_config();
        ConfigManager { config }
    }

    fn init_config() -> Config {
        if !std::path::Path::new(&get_config_file_path()).exists() {
            std::fs::create_dir_all(get_config_folder_path()).expect("Failed to create config directory");
            std::fs::File::create(get_config_file_path()).expect("Failed to create config.toml file");
            let default_config = Self::default_config();
            let default_config_toml = toml::to_string(&default_config).unwrap();
            fs::write(get_config_file_path(), default_config_toml)
                .expect("Failed to write to config.toml");
            return default_config;
        }
        let config_string =
            fs::read_to_string(get_config_file_path()).expect("Failed to read config.toml");
        let config: Config = toml::from_str(&config_string).expect("Failed to parse config.toml");
        config
    }

    pub fn default_config() -> Config {
        Config {
            shinkai_node_env: Some(ShinkaiNodeEnv {
                node_api_port: Some("9550".to_string()),
                node_storage_path: Some(get_default_shinkai_node_storage_path().to_string()),
                unstructured_server_url: Some("https://public.shinkai.com/x-un".to_string()),
                embeddings_server_url: Some("https://public.shinkai.com/x-em".to_string()),
                first_device_needs_registration_code: Some("false".to_string()),
                initial_agent_names: Some("ollama_mistral".to_string()),
                initial_agent_urls: Some("http://localhost:11434".to_string()),
                initial_agent_models: Some("ollama:mistral".to_string()),
                initial_agent_api_keys: Some("".to_string()),
                starting_num_qr_devices: Some("0".to_string()),
            }),
            current: None,
        }
    }

    pub fn get_config(&self) -> Config {
        self.config.clone()
    }

    pub fn write_config(&mut self, config: &Config) -> Config {
        let config_toml = toml::to_string(config).unwrap();
        let config_path = get_config_file_path();
        fs::write(config_path, config_toml).expect("Failed to write to config.toml");
        self.config = config.clone();
        self.config.clone()
    }
}
