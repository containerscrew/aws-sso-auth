use current_platform::{COMPILED_ON, CURRENT_PLATFORM};
use tracing::{error, info};

use crate::utils::print_banner;

mod utils;

mod cli;

mod commands;

mod logger;
fn main() {
    // Default banner
    print_banner();

    // Platform information
    info!(
        message = "running in",
        platform = CURRENT_PLATFORM,
        compiled_on = COMPILED_ON
    );

    // Command line flags control all the program flow
    match cli::argparse() {
        Ok(_) => {} // nothing to print
        Err(err) => {
            error!("Error parsing clap CLI {}", err)
        }
    }
}
