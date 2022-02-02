mod api_helper;
mod command_line_parser;
mod config;
mod file_handler;

use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = command_line_parser::get_command_line_arguments();

    let config = config::APIConfiguration {
        api_key: args.api_key,
    };

    config::write_api_key_to_config_file(&config);

    let config_file = config::deserialize_configuration_file().unwrap();
    let api_key = config_file.get("api_key").unwrap();

    let file_data = file_handler::get_file_as_string(args.path);
    let res = api_helper::post_pastebin(
        api_key.to_string(),
        file_data,
        args.paste_format,
        args.paste_name,
    );
    println!("{}", res.await?.text().await?);
    Ok(())
}
