use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short, long, help = "AWS region", default_value = "eu-west-1")]
    pub region: String,
    #[arg(short, long, help = "AWS SSO start url")]
    pub start_url: String,
    #[arg(short, long, help = "Max number of workers (spawned tasks/threads)")]
    pub workers: usize,
    // #[arg(short, long, help = "Log level: error, warn, info, debug , trace", default_value="warn")]
    // pub log_level: String,
    // #[command(subcommand)]
    // command: Option<Commands>,
}

// #[derive(Subcommand)]
// enum Commands {
//     Start {
//         #[arg(short, long, help = "AWS region", default_value = "eu-west-1")]
//         region: String,
//         #[arg(short, long, help = "AWS SSO start url")]
//         start_url: String,
//     },
//     Version {
//         /// lists test values
//         #[arg(short, long)]
//         list: bool,
//     },
// }

pub fn argparse() -> Cli {
    let cli = Cli::parse();

    // Provide some validations with some flags
    // TO DO

    // Return cli
    cli
}
