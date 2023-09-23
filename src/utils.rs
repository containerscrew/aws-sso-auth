use std::io;
use configparser::ini::Ini;
use log::{error};
use crate::lib::AccountCredentials;

pub fn open_browser_url(url: &String) {
    // From the device authorization, open the URL in the browser
    if webbrowser::open(&*url).is_ok() {
        println!("Web browser opened correctly!")
    } else { panic!("Problems with WebBrowser!!!") }
}

pub fn read_user_input() {
    println!("Type ENTER to continue...");
    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_input).unwrap();
}

pub fn write_configuration(all_credentials: Vec<AccountCredentials>) {
    //Start configparser to write data
    let mut configuration = Ini::new_cs();
    let mut aws_credentials_file = String::from("");

    match home::home_dir() {
        Some(path) => aws_credentials_file = format!("{}/.aws/credentials", path.display()),
        None => error!("Impossible to get your home dir!"),
    }

    for creds in all_credentials {
        configuration.set(&format!("{}@{}", creds.account_name , creds.role_name), "region", Some("eu-west-1".parse().unwrap()));
        configuration.set(&format!("{}@{}", creds.account_name , creds.role_name), "aws_access_key_id", Option::from(creds.aws_access_key_id));
        configuration.set(&format!("{}@{}", creds.account_name , creds.role_name), "aws_secret_access_key", Option::from(creds.aws_secret_access_key));
        configuration.set(&format!("{}@{}", creds.account_name , creds.role_name), "aws_session_token", Option::from(creds.aws_session_token));
        configuration.write(&aws_credentials_file).expect("Can't write configuration file");
    }
}
