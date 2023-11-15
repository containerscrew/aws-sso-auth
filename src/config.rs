use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

// fn read_json_file(file_path: &str) -> io::Result<HashMap<Profile, i32>> {
//     let mut file = File::open(file_path)?;
//     let mut content = String::new();
//     file.read_to_string(&mut content)?;
//
//     let data: HashMap<Profile, i32> = serde_json::from_str(&content)?;
//     Ok(data)
// }
//
// fn write_json_file(file_path: &str, data: &HashMap<Profile, i32>) -> io::Result<()> {
//     let json_content = serde_json::to_string_pretty(data)?;
//     let mut file = File::create(file_path)?;
//     file.write_all(json_content.as_bytes())?;
//     Ok(())
// }
//
// pub fn create_config_file(start_url: &String, aws_region: &String, profile_name: &String) -> Result<(), Box<dyn std::error::Error>> {
//     let profile = Profile{
//         profile_name: profile_name.to_string(),
//         config: Config {
//             start_url: start_url.to_string(),
//             aws_region: aws_region.to_string(),
//         },
//     };
//
//     write_config_file(&profile, "test.test.json")
// }
//
// fn write_config_file(data: &Profile, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let mut data = if Path::new(file_name).exists() {
//         read_json_file(file_name).expect("Can't read json file!")
//     } else {
//         HashMap::new()
//     };
//
//     Ok(())
// }
