use crate::cli;
use clap::{error::ErrorKind, Command};

// Utility function to create a test CLI instance
fn test_cli() -> Command {
    cli()
}

#[test]
fn test_cli_about() {
    let cmd = test_cli();
    assert_eq!(
        cmd.get_about().unwrap().to_string(),
        "A Shinkai Node versioning CLI"
    );
}

#[test]
fn test_cli_subcommands() {
    let cmd = test_cli();
    let subcommands: Vec<_> = cmd
        .get_subcommands()
        .map(|sc| sc.get_name().to_string())
        .collect();
    assert_eq!(
        subcommands,
        vec!["list", "install", "use", "version", "node"]
    );
}

#[test]
fn test_cli_install_requires_version_arg() {
    use clap::error::ErrorKind;

    let cmd = test_cli();
    let result = cmd.try_get_matches_from(vec!["kaivm", "install"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(
        err.kind(),
        ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
    );
}

#[test]
fn test_cli_use_requires_version_arg() {
    let cmd = test_cli();
    let result = cmd.try_get_matches_from(vec!["kaivm", "use"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(
        err.kind(),
        ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
    );
}

#[test]
fn test_cli_node_subcommand_run() {
    let cmd = test_cli();
    let result = cmd.try_get_matches_from(vec!["kaivm", "node", "run"]);
    assert!(result.is_ok());
    let matches = result.ok().unwrap();
    assert!(matches
        .subcommand_matches("node")
        .unwrap()
        .subcommand_matches("run")
        .is_some());
}

#[test]
fn test_cli_node_subcommand_env() {
    let cmd = test_cli();
    let result = cmd.try_get_matches_from(vec!["kaivm", "node", "env", "--node_api_port", "9550"]);
    assert!(result.is_ok());
    let matches = result.ok().unwrap();
    let env_matches = matches
        .subcommand_matches("node")
        .unwrap()
        .subcommand_matches("env")
        .unwrap();
    assert_eq!(
        env_matches.get_one::<String>("node_api_port"),
        Some(&"9550".to_string())
    );
}

#[test]
fn test_cli_node_subcommand_reset() {
    let cmd = test_cli();
    let result = cmd.try_get_matches_from(vec!["kaivm", "node", "reset"]);
    assert!(result.is_ok());
    let matches = result.ok().unwrap();
    assert!(matches
        .subcommand_matches("node")
        .unwrap()
        .subcommand_matches("reset")
        .is_some());
}

// Additional tests based on follow-up prompt
#[test]
fn test_cli_version_command() {
    let cmd = test_cli();
    let result = cmd.try_get_matches_from(vec!["kaivm", "version"]);
    assert!(result.is_ok());
}

#[test]
fn test_cli_list_command() {
    let cmd = test_cli();
    let result = cmd.try_get_matches_from(vec!["kaivm", "list"]);
    assert!(result.is_ok());
}

#[test]
fn test_cli_install_with_version() {
    let cmd = test_cli();
    let result = cmd.try_get_matches_from(vec!["kaivm", "install", "v0.5.3"]);
    assert!(result.is_ok());
    let matches = result.ok().unwrap();
    assert_eq!(
        matches
            .subcommand_matches("install")
            .unwrap()
            .get_one::<String>("VERSION"),
        Some(&"v0.5.3".to_string())
    );
}

#[test]
fn test_cli_use_with_version() {
    let cmd = test_cli();
    let result = cmd.try_get_matches_from(vec!["kaivm", "use", "v0.5.3"]);
    assert!(result.is_ok());
    let matches = result.ok().unwrap();
    assert_eq!(
        matches
            .subcommand_matches("use")
            .unwrap()
            .get_one::<String>("VERSION"),
        Some(&"v0.5.3".to_string())
    );
}

// Follow-up tests for additional coverage
#[test]
fn test_cli_node_env_with_multiple_args() {
    let cmd = test_cli();
    let result = cmd.try_get_matches_from(vec![
        "kaivm",
        "node",
        "env",
        "--node_api_port",
        "9550",
        "--node_storage_path",
        "/path/to/storage",
        "--unstructured_server_url",
        "http://localhost:8080",
        "--embeddings_server_url",
        "http://localhost:8081",
        "--first_device_needs_registration_code",
        "true",
    ]);
    assert!(result.is_ok());
    let matches = result.ok().unwrap();
    let env_matches = matches
        .subcommand_matches("node")
        .unwrap()
        .subcommand_matches("env")
        .unwrap();
    assert_eq!(
        env_matches.get_one::<String>("node_api_port"),
        Some(&"9550".to_string())
    );
    assert_eq!(
        env_matches.get_one::<String>("node_storage_path"),
        Some(&"/path/to/storage".to_string())
    );
    assert_eq!(
        env_matches.get_one::<String>("unstructured_server_url"),
        Some(&"http://localhost:8080".to_string())
    );
    assert_eq!(
        env_matches.get_one::<String>("embeddings_server_url"),
        Some(&"http://localhost:8081".to_string())
    );
    assert_eq!(
        env_matches.get_one::<String>("first_device_needs_registration_code"),
        Some(&"true".to_string())
    );
}

#[test]
fn test_cli_node_env_with_invalid_arg() {
    let cmd = test_cli();
    let result = cmd.try_get_matches_from(vec!["kaivm", "node", "env", "--invalid_arg", "value"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::UnknownArgument);
}
