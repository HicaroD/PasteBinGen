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

impl APIConfiguration {
    fn is_default_key(&self) -> bool {
        self.api_key == "default"
    }
}

pub fn write_api_key_to_config_file(config_file_path: &Path, configuration: &APIConfiguration) {
    if !configuration.is_default_key() {
        let config_file = File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(config_file_path)
            .expect("Unable to open configuration file");
        serde_json::to_writer_pretty(config_file, configuration).expect("Invalid configuration");
    }
}

pub fn deserialize_configuration_file(
    config_file_path: &Path,
) -> Result<JSONData, std::io::Error> {
    let config_file = fs::read_to_string(config_file_path)?;
    let config_file_content = serde_json::from_str::<JSONData>(&config_file)?;
    Ok(config_file_content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_deserializer() {
        let config_file_path = Path::new("src/tests/config_example.json");
        let expected_result = HashMap::from([("api_key".to_string(), "SOME_RANDOM_API_KEY".to_string())]);
        assert_eq!(
            deserialize_configuration_file(config_file_path).unwrap(),
            expected_result
        )
    }
}
