mod command_line_parser;
mod config;

fn main() {
    let args = command_line_parser::get_command_line_arguments();

    let config = config::APIConfiguration {
        api_key: args.api_key,
    };

    config::create_configuration_file(&config);
}
