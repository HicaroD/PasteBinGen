use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::Path;

type JSONData = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct APIConfiguration {
    pub api_key: String,
}

pub fn config_file_exists(file_path: &'static str) -> bool {
    Path::new(file_path).exists()
}

pub fn create_configuration_file(file_path: &'static str) -> File {
    File::create(Path::new(file_path)).expect("Unable to create configuration file")
}

pub fn write_api_key_to_config_file(
    config_file_path: &'static str,
    configuration: &APIConfiguration,
) {
    if !config_file_exists(&config_file_path) {
        let _ = create_configuration_file(&config_file_path);
    }
    if api_key_flag_was_passed(&configuration) {
        let config_file = File::options()
            .write(true)
            .truncate(true)
            .open(config_file_path)
            .expect("Unable to open configuration file");
        serde_json::to_writer_pretty(config_file, configuration).expect("Invalid configuration");
    }
}

pub fn api_key_flag_was_passed(configuration: &APIConfiguration) -> bool {
    !configuration.api_key.eq("default")
}

pub fn deserialize_configuration_file(config_file_path: &'static str) -> Option<JSONData> {
    if config_file_exists(&config_file_path) {
        let config_file = fs::read_to_string(config_file_path);

        match config_file {
            Ok(config_file_content) => {
                let config_file_data =
                    serde_json::from_str::<JSONData>(&config_file_content).unwrap();
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
