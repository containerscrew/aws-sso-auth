use std::time::Instant;
use env_logger::{Env};
use current_platform::{COMPILED_ON, CURRENT_PLATFORM};
use tokio::runtime::Builder;

mod lib;
mod utils;

mod argparse;

fn main() ->  Result<(), Box<dyn std::error::Error>> {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .max_blocking_threads(1)
        .enable_time()
        .enable_io()
        .thread_name("testing")
        .build()
        .unwrap()
        .block_on(async_main())
}

async fn async_main() -> Result<(), Box<dyn std::error::Error>> {
    // Logging
    env_logger::Builder::from_env(Env::default().default_filter_or("error")).init();

    // Command line flags
    let cli = argparse::argparse();

    // Platform information
    println!("Currently platform: {}. Compiled in {}", CURRENT_PLATFORM, COMPILED_ON);

    // AWS Config
    let config = lib::init_config(&cli.region).await;

    // AWS SSOIDC CLIENT
    let ssoidc_client = lib::ssoidc_client(&config).await;

    // AWS SSO CLIENT
    let sso_client = lib::sso_client(&config).await;

    // Register device and get client id and client secret
    let device_credentials: lib::DeviceClientCredentials = lib::get_register_device_credentials(&ssoidc_client).await.expect("Can't register device!");

    // Get device user&device codes and verification url
    let device_auth_credentials: lib::DeviceAuthCredentials = lib::get_device_authorization_credentials(&ssoidc_client, &device_credentials, &cli.start_url).await?;

    // Open browser with verification URL
    utils::open_browser_url(&device_auth_credentials.verification_url);

    // To continue this program, user must accept the approval in the browser, without this we can't continue
    // That's why we need to pause the program until the user press Enter
    utils::read_user_input();

    // Generate token
    let token = lib::generate_token(&ssoidc_client, &device_credentials, &device_auth_credentials).await.expect("Can't regenerate token :(");

    // Get account list using the previous generate token
    let account_list = lib::get_account_list(&sso_client, &token).await.expect("Can't get account list");

    let mut all_credentials: Vec<lib::AccountCredentials>  = vec![];

    let start = Instant::now();

    let tasks:Vec <_> = account_list
        .into_iter()
        .map(|account| {
            let sso_client = sso_client.clone(); // Clone the sso_client
            let token = token.clone(); // Clone the token
            tokio::spawn(async move {
                let account_name = &account.account_name.unwrap();
                println!("Fetching credentials for {}", &account_name);
                let account_credentials = lib::get_account_credentials(
                    &sso_client,
                    &account.account_id.unwrap(),
                    &token,
                    &account_name)
                    .await.expect("Can't get account credentials");
                account_credentials
            })
        })
        .collect();

    for task in tasks {
        let account_credential = task.await.unwrap();
        all_credentials.extend(account_credential);
    }

    println!("Writing data to file");
    utils::write_configuration(all_credentials);

    println!("Credentials downloaded in {:?}s", start.elapsed().as_secs());
    Ok(())
}
