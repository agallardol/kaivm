pub fn get_full_path(sub_path: &String) -> String {
    let base_path = dirs::home_dir().expect("Failed to get home directory");
    base_path
        .join(format!(".kaivm/{}", sub_path))
        .to_str()
        .unwrap()
        .to_string()
}

pub fn get_arch() -> String {
    if cfg!(target_arch = "aarch64") && cfg!(target_os = "macos") {
        "aarch64-apple-darwin".to_string()
    } else if cfg!(target_arch = "x86_64") && cfg!(target_os = "windows") {
        "x86_64-pc-windows-msvc".to_string()
    } else if cfg!(target_arch = "x86_64") && cfg!(target_os = "linux") {
        "x86_64-unknown-linux-gnu".to_string()
    } else {
        panic!("Unsupported architecture or operating system");
    }
}

pub fn get_versions_folder_path() -> String {
    get_full_path(&"versions".to_string())
}

pub fn get_version_binary_file_path(version: &String) -> String {
    let version_folder_path = get_version_folder_path(version);
    let version_binary_file_path = format!("{}/shinkai-node-{}", version_folder_path, version);
    version_binary_file_path
}

pub fn get_version_folder_path(version: &String) -> String {
    let version_folder_path = get_versions_folder_path();
    format!("{}/{}", version_folder_path, version)
}

pub fn get_default_shinkai_node_storage_path() -> String {
    get_full_path(&"default-storage".to_string())
}

pub fn get_config_folder_path() -> String {
    get_full_path(&"config".to_string())
}

pub fn get_config_file_path() -> String {
    let config_file_path = format!("{}/config.toml", get_config_folder_path());
    config_file_path
}

pub fn get_shinkai_node_binary_url(version: &String) -> String {
    let url = format!(
        "https://download.shinkai.com/shinkai-node/binaries/{}/shinkai-node-{}{}",
        get_arch(),
        version,
        if cfg!(target_os = "windows") {
            ".exe"
        } else {
            ""
        },
    );
    url
}

pub fn get_shinkai_node_binaries_versions_url() -> String {
    "https://download.shinkai.com/shinkai-node/binaries/versions.json".to_string()
}
