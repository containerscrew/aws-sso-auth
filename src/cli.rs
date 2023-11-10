use clap::{Parser, Subcommand};
use tracing::info;
use crate::commands::config;
use crate::commands::config::{CONFIG_FILE_PATH, Configuration, read_config_file};
use crate::utils::{config_file_exists};

#[derive(Parser)]
#[clap(about = "aws-sso-auth", version = "0.0.1", author = "Daniels info@containerscrew.com", about = "Get your ~/.aws/credentials using AWS SSO and your external IDP", arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Set your configuration for all your AWS SSO portals. Will be saved in ~/.aws/aws-sso-auth.json
    Config {
        #[arg(short = 's', long = "start-url", help = "AWS start URL endpoint. Example: https://XXXXXX.awsapps.com/start", required = true)]
        start_url: String,
        #[arg(short = 'r', long = "aws-region", help = "AWS region where you have configured SSO", required = true, default_value = "us-east-1")]
        aws_region: String,
        #[arg(short = 'p', long = "profile-name", help = "The name with which you want to save the configuration", required = true)]
        profile_name: String,
        #[arg(short = 'o', long = "overwrite", help = "Overwrite the config file. Create a new empty file", default_value = "false", required = false)]
        overwrite: bool,
    },
    /// Start fetching your AWS credentials
    Start {
        #[arg(short = 'p', long = "profile-name", help = "The name with which you want to fetch the credentials", required = true)]
        profile_name: String
    },
}

pub fn argparse() -> Cli {
    let cli = Cli::parse();

    // Provide some validations with some flags
    // TO DO

    match &cli.command {
        Some(Commands::Config { start_url, aws_region, profile_name , overwrite}) => {
            let config: Configuration = Configuration::new(start_url.to_string(), aws_region.to_string(), profile_name.to_string());

            // Check if config file exists
            config_file_exists(CONFIG_FILE_PATH);

            // Write configuration
            config.write_config_file();
            info!("Configuration file saved!")
        }
        Some(Commands::Start { profile_name }) => {
            read_config_file()
        }
        None => {}
    }

    // Return cli
    cli
}
