use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

type JSONData = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct APIConfiguration {
    pub api_key: String,
}

pub fn config_file_exists(file_path: &PathBuf) -> bool {
    file_path.is_file()
}

pub fn create_configuration_file(file_path: &PathBuf) -> File {
    File::create(file_path).expect("Unable to create configuration file")
}

pub fn write_api_key_to_config_file(
    config_file_path: &PathBuf,
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

pub fn deserialize_configuration_file(config_file_path: &PathBuf) -> Option<JSONData> {
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
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_deserializer() {
        let config_file_path = "src/tests/config_example.json";
        let expected_result =
            HashMap::from([("api_key".to_string(), "SOME_RANDOM_API_KEY".to_string())]);
        assert_eq!(
            deserialize_configuration_file(config_file_path),
            Some(expected_result)
        )
    }

    #[test]
    fn test_if_config_file_exists() {
        assert_eq!(config_file_exists("src/tests/config_example.json"), true);
    }

}
