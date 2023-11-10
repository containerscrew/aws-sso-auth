use std::{io};
use std::fs::File;
use std::path::Path;
use colored::Colorize;
use log::{error, info};


pub fn print_banner() {
    let banner = r#"
                                                  |    |
    ,---.. . .,---.   ,---.,---.,---.   ,---..   .|--- |---.
    ,---|| | |`---.---`---.`---.|   |---,---||   ||    |   |
    `---^`-'-'`---'   `---'`---'`---'   `---^`---'`---'`   '

    Author: github.com/containerscrew
    License: GNU AFFERO GENERAL PUBLIC LICENSE V3
    Description: Fetch your local ~/.aws/credentials using AWS SSO
"#;

    println!("{}", banner.truecolor(255, 165, 0));
}

pub fn open_browser_url(url: &String) {
    // From the device authorization, open the URL in the browser
    if webbrowser::open(&*url).is_ok() {
        info!("Web browser opened correctly!")
    } else { error!("Problems with WebBrowser") }
}

pub fn read_user_input() {
    info!("Type ENTER to continue...");
    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_input).unwrap();
}

pub fn config_file_exists(path: &str) {
    // This function checks if config file ~/.aws/aws-sso-auth.json exists
    // If not, will try to create a new one
    let expanded_path = extend_path(path);
    let directory_path = Path::new(&expanded_path);

    match directory_path.metadata() {
        Ok(metadata) => {
            if metadata.is_file(){
                info!("Config file exists: {}", &path);
            }
        },
        Err(_) => {
            info!("Config file don't exists {}.", &expanded_path);
            // If config file don't exists, try to create a new one
            // match File::create(&expanded_path) {
            //     Ok(_) => info!("File {} created", &expanded_path),
            //     Err(err) => error!("Can't create file. {}", err),
            // }
        }
    }
}

pub fn extend_path(path: &str) -> String {
    shellexpand::tilde(path).to_string()
}


// pub fn write_configuration(all_credentials: Vec<AccountCredentials>) {
//     //Start configparser to write data
//     let mut configuration = Ini::new_cs();
//     let mut aws_credentials_file = String::from("");
//
//     match home::home_dir() {
//         Some(path) => aws_credentials_file = format!("{}/.aws/credentials", path.display()),
//         None => error!("Impossible to get your home dir!"),
//     }
//
//     for creds in all_credentials {
//         configuration.set(&format!("{}@{}", creds.account_name , creds.role_name), "region", Some("eu-west-1".parse().unwrap()));
//         configuration.set(&format!("{}@{}", creds.account_name , creds.role_name), "aws_access_key_id", Option::from(creds.aws_access_key_id));
//         configuration.set(&format!("{}@{}", creds.account_name , creds.role_name), "aws_secret_access_key", Option::from(creds.aws_secret_access_key));
//         configuration.set(&format!("{}@{}", creds.account_name , creds.role_name), "aws_session_token", Option::from(creds.aws_session_token));
//         configuration.write(&aws_credentials_file).expect("Can't write configuration file");
//     }
// }
