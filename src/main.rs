mod command_line_parser;
mod config;

fn main() {
    let args = command_line_parser::get_command_line_arguments();

    let config = config::APIConfiguration {
        api_key: args.api_key,
    };

    config::create_configuration_file(&config);
    let config_file = config::deserialize_configuration_file();
    let api_key = config_file.get("api_key");

    match api_key {
        Some(value) => println!("{}", value),
        None => println!("There is no value in this key!"),
    }
}
