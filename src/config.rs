use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;

type JSONData = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct APIConfiguration {
    pub api_key: String,
}

pub fn config_file_exists() -> bool {
    use std::path::Path;
    Path::new("config.json").exists()
}

pub fn write_api_key_to_config_file(configuration: &APIConfiguration) {
    let config_file = File::create("config.json").expect("Can't create file");
    serde_json::to_writer_pretty(config_file, configuration).expect("Invalid configuration");
}

pub fn api_key_flag_was_passed(configuration: &APIConfiguration) -> bool {
    !configuration.api_key.eq("default")
}

pub fn create_configuration_file(configuration: &APIConfiguration) {
    if api_key_flag_was_passed(configuration){
        write_api_key_to_config_file(configuration);
    }
}

pub fn deserialize_configuration_file() -> JSONResponse {
    let config_file = fs::read_to_string("config.json");

    match config_file {
        Ok(config_file_content) => {
            let config_file = serde_json::from_str::<JSONResponse>(&config_file_content).unwrap();
            config_file
        }

        Err(err) => panic!("Error while trying to read configuration file: {}", err),
    }
}
