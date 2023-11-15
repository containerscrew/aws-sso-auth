use std::os::unix::prelude::PermissionsExt;

use crate::commands::config;
use crate::commands::config::{Configuration, CONFIG_FILE_PATH};
use crate::utils::config_file_exists;
use clap::{Parser, Subcommand};
use tracing::info;

#[derive(Parser)]
#[clap(
    about = "aws-sso-auth",
    version = "0.0.1",
    author = "Daniels info@containerscrew.com",
    about = "Get your ~/.aws/credentials using AWS SSO and your external IDP",
    arg_required_else_help = true
)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Set your configuration for all your AWS SSO portals. Will be saved in ~/.aws/aws-sso-auth.json
    Config {
        #[arg(
            short = 's',
            long = "start-url",
            help = "AWS start URL endpoint. Example: https://XXXXXX.awsapps.com/start",
            required = true
        )]
        start_url: String,
        #[arg(
            short = 'r',
            long = "aws-region",
            help = "AWS region where you have configured SSO",
            required = true,
            default_value = "us-east-1"
        )]
        aws_region: String,
        #[arg(
            short = 'p',
            long = "profile-name",
            help = "The name with which you want to save the configuration",
            required = true
        )]
        profile_name: String,
    },
    /// Start fetching your AWS credentials
    Start {
        #[arg(
            short = 'l',
            long = "log-level",
            help = "Log level. Print all info when you download credentials!",
            default_value = "info",
            required = false
        )]
        log_level: String,
    },
}

pub fn argparse() -> Cli {
    let cli = Cli::parse();

    // Provide some validations with some flags
    // TO DO

    match &cli.command {
        Some(Commands::Config {
            start_url,
            aws_region,
            profile_name,
        }) => {
            let config: Configuration = Configuration::new(
                start_url.to_string(),
                aws_region.to_string(),
                profile_name.to_string(),
            );

            // Write configuration
            config.write_config_file();
        }
        Some(Commands::Start { log_level }) => {
            // Check if config file exists
            config_file_exists(CONFIG_FILE_PATH);

            // Read data from file
        }
        None => {}
    }

    // Return cli
    cli
}
