use current_platform::{COMPILED_ON, CURRENT_PLATFORM};
use tracing::info;

use crate::utils::print_banner;

mod utils;

mod cli;

mod commands;
mod config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(3) // more threads can't provide AWS API 429 responses
        .enable_time()
        .enable_io()
        .thread_name("aws-sso-auth")
        .build()
        .unwrap()
        .block_on(async_main())
}

async fn async_main() -> Result<(), Box<dyn std::error::Error>> {
    // Default banner
    print_banner();

    // Initialize global logging
    tracing_subscriber::fmt()
        .with_thread_names(true)
        .with_max_level(tracing::Level::INFO)
        .init();

    // Platform information
    info!(platform = CURRENT_PLATFORM, compiled_on = COMPILED_ON);

    // Command line flags
    cli::argparse();

    // // AWS Config
    // let config = lib::init_config("eu-west-1").await;
    //
    // // AWS SSOIDC CLIENT
    // let ssoidc_client = lib::ssoidc_client(&config).await;
    //
    // // AWS SSO CLIENT
    // let sso_client = lib::sso_client(&config).await;
    //
    // // Register device and get client id and client secret
    // let device_credentials: lib::DeviceClientCredentials = lib::get_register_device_credentials(&ssoidc_client).await.expect("Can't register device!");
    //
    // // Get device user&device codes and verification url
    // let device_auth_credentials: lib::DeviceAuthCredentials = lib::get_device_authorization_credentials(&ssoidc_client, &device_credentials, "https://d-93671e0715.awsapps.com/start").await?;
    //
    // // Open browser with verification URL
    // utils::open_browser_url(&device_auth_credentials.verification_url);
    //
    // // To continue this program, user must accept the approval in the browser, without this we can't continue
    // // That's why we need to pause the program until the user press Enter
    // utils::read_user_input();
    //
    // // Generate token
    // let token = lib::generate_token(&ssoidc_client, &device_credentials, &device_auth_credentials).await.expect("Can't regenerate token :(");
    //
    // // Get account list using the previous generate token
    // let account_list = lib::get_account_list(&sso_client, &token).await.expect("Can't get account list");
    //
    // let mut all_credentials: Vec<lib::AccountCredentials>  = vec![];
    //
    // let start = Instant::now();
    //
    // let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(5));
    // let mut join_handles = Vec::new();
    //
    // for account in account_list {
    //     let permit = semaphore.clone().acquire_owned().await.unwrap();
    //     let sso_client = sso_client.clone();
    //     let token = token.clone();
    //
    //     join_handles.push(tokio::spawn(async move {
    //         let account_name = &account.account_name.unwrap();
    //         let account_credentials = lib::get_account_credentials(
    //             &sso_client,
    //             &account.account_id.unwrap(),
    //             &token,
    //             &account_name)
    //             .await.expect("Can't get account credentials");
    //         drop(permit);
    //         return account_credentials;
    //
    //     }));
    // }
    //
    // for handle in join_handles {
    //     let account_credential = handle.await.unwrap();
    //     all_credentials.extend(account_credential);
    // }
    //
    // println!("Writing data to file");
    // utils::write_configuration(all_credentials);
    //
    // println!("Credentials downloaded in {:?}s", start.elapsed().as_secs());
    Ok(())
}
