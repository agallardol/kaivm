use clap::{arg, Arg, Command};
use config::config_manager::ConfigManager;

mod command_handlers;
mod config;
mod utils;
mod tests;

fn cli() -> Command {
    Command::new("kaivm")
        .about("A Shinkai Node versioning CLI")
        .subcommand_required(true)
        .subcommand(Command::new("list").short_flag('l').long_flag("list").long_flag_alias("ls").about("List all Shinkai Node installed versions"))
        .subcommand(
            Command::new("install")
                .short_flag('i')
                .about("Install a specific version of Shinkai Node")
                .arg(arg!(<VERSION> "The version to install"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("use")
                .short_flag('u')
                .about("Use a specific version of Shinkai Node")
                .arg(arg!(<VERSION> "The version to use"))
                .arg_required_else_help(true),
        )
        .subcommand(Command::new("version").short_flag('v').about("Get current Shinkai Node version"))
        .subcommand(
            Command::new("node")
                .short_flag('n')
                .long_flag("shinkai-node")
                .subcommand_required(true)
                .about("Manage Shinkai Node")
                .subcommand(Command::new("run").about("Run current Shinkai Node version"))
                .subcommand(
                    Command::new("env").about("Set variables for current Shinkai Node")
                                .arg(Arg::new("node_api_port").long("node_api_port").required(false).help("Specifies the port on which the Shinkai Node API will run."))
                                .arg(Arg::new("node_storage_path").long("node_storage_path").required(false).help("Defines the file system path where the Shinkai Node will store its data."))
                                .arg(Arg::new("unstructured_server_url").long("unstructured_server_url").required(false).help("The URL of the unstructured server that the Shinkai Node will communicate with."))
                                .arg(Arg::new("embeddings_server_url").long("embeddings_server_url").required(false).help("The URL of the embeddings server used by the Shinkai Node for processing data."))
                                .arg(Arg::new("first_device_needs_registration_code").long("first_device_needs_registration_code").required(false).help("Determines whether the first device connecting to the Shinkai Node requires a registration code for authentication. Accepts 'true' or 'false'."))
                                .arg(Arg::new("initial_agent_names").long("initial_agent_names").required(false).help("A comma-separated list of initial agent names to be registered with the Shinkai Node."))
                                .arg(Arg::new("initial_agent_urls").long("initial_agent_urls").required(false).help("A comma-separated list of URLs for the initial agents, corresponding to the names provided."))
                                .arg(Arg::new("initial_agent_models").long("initial_agent_models").required(false).help("A comma-separated list of models for the initial agents, corresponding to the names provided."))
                                .arg(Arg::new("initial_agent_api_keys").long("initial_agent_api_keys").required(false).help("A comma-separated list of API keys for the initial agents, corresponding to the names provided."))
                                .arg(Arg::new("starting_num_qr_devices").long("starting_num_qr_devices").required(false).help("The initial number of QR devices that should be supported by the Shinkai Node."))
                                .arg(Arg::new("node_port").long("node_port").required(false).help("Specifies the port on which the Shinkai Node will run."))
                                .arg(Arg::new("node_ws_port").long("node_ws_port").required(false).help("Specifies the WebSocket port for the Shinkai Node."))
                                .arg(Arg::new("unstructured_server_api_key").long("unstructured_server_api_key").required(false).help("The API key for the unstructured server that the Shinkai Node will communicate with."))
                                .arg(Arg::new("embeddings_server_api_key").long("embeddings_server_api_key").required(false).help("The API key for the embeddings server used by the Shinkai Node for processing data."))
                                .arg(Arg::new("job_manager_threads").long("job_manager_threads").required(false).help("The number of threads to be used by the job manager within the Shinkai Node."))
                                .arg(Arg::new("global_identity_name").long("global_identity_name").required(false).help("The global identity name for the Shinkai Node."))
                )
                .subcommand(
                    Command::new("reset").about("Reset your Shinkai Node storage")
                                .arg(Arg::new("confirm").long("confirm").short('c').help("Confirm you want to reset your Shinkai Node.")
                            )
                        )
        )
}

#[tokio::main]
async fn main() {
    let config_manager = ConfigManager::new();
    let command = cli();
    let matches = command.get_matches();
    match matches.subcommand() {
        Some(("list", sub_matches)) => {
            command_handlers::list::list(&cli(), sub_matches, config_manager)
        }
        Some(("install", sub_matches)) => {
            command_handlers::install::install(&cli(), sub_matches, config_manager).await
        }
        Some(("use", sub_matches)) => {
            command_handlers::r#use::r#use(&cli(), sub_matches, config_manager)
        }
        Some(("version", sub_matches)) => {
            command_handlers::version::version(&cli(), sub_matches, config_manager)
        }
        Some(("node", sub_matches)) => match sub_matches.subcommand() {
            Some(("run", sub_matches)) => {
                command_handlers::run::run(&cli(), sub_matches, config_manager)
            }
            Some(("env", sub_matches)) => {
                command_handlers::env::env(&cli(), sub_matches, config_manager)
            }
            Some(("reset", sub_matches)) => {
                command_handlers::reset::reset(&cli(), sub_matches, config_manager)
            }
            _ => {
                let error = clap::Error::raw(
                    clap::error::ErrorKind::UnknownArgument,
                    "Command not found.",
                );
                error.exit();
            }
        },
        _ => {
            let error = clap::Error::raw(
                clap::error::ErrorKind::UnknownArgument,
                "Command not found.",
            );
            error.exit();
        }
    }
}
