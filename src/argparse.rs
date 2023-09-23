use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short, long, help = "AWS region", default_value = "eu-west-1")]
    pub region: String,
    #[arg(short, long, help = "AWS SSO start url")]
    pub start_url: String,
    // #[arg(short, long, help = "Log level: error, warn, info, debug , trace", default_value="warn")]
    // pub log_level: String,
}

pub fn argparse() -> Cli {
    let cli = Cli::parse();

    // Provide some validations with some flags
    // TO DO

    // Return cli
    cli
}
