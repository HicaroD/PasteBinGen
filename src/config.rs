use serde::{Serialize, Deserialize};
use std::fs::File;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct APIConfiguration {
    pub api_key: String,
}

pub fn config_file_exists(path: &str) -> bool {
    return Path::new(path).exists()
}

pub fn create_configuration_file(configuration: &APIConfiguration){
    let config_file = File::create("config.json").expect("Can't create file");
    serde_json::to_writer_pretty(config_file, configuration).expect("Invalid configuration");
}
