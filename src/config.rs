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

pub fn create_configuration_file() -> File {
    File::create("config.json").expect("Unable to create configuration file")
}

pub fn write_api_key_to_config_file(configuration: &APIConfiguration) {
    if !config_file_exists() {
        let _ = create_configuration_file();
    }
    if api_key_flag_was_passed(&configuration) {
        let config_file = File::options()
            .write(true)
            .open("config.json")
            .expect("Unable to open configuration file");
        serde_json::to_writer_pretty(config_file, configuration).expect("Invalid configuration");
    }
}

pub fn api_key_flag_was_passed(configuration: &APIConfiguration) -> bool {
    !configuration.api_key.eq("default")
}

pub fn deserialize_configuration_file() -> Option<JSONData> {
    if config_file_exists() {
        let config_file = fs::read_to_string("config.json");

        match config_file {
            Ok(config_file_content) => {
                let config_file_data = serde_json::from_str::<JSONData>(&config_file_content).unwrap();
                Some(config_file_data)
            }

            Err(err) => panic!("Error while trying to read configuration file: {}", err),
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {}
