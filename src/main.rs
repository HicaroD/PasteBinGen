mod config;

fn main() {
    let config_file_path = "config.json".to_string();

    if config::config_file_exists(&config_file_path){
        println!("Config file exists!");
    }

    else {
        let config = config::APIConfiguration {
            api_key: "something".to_string(),
        };

        config::create_configuration_file(&config);
    }
}
