use std::fs;
use clap::Parser;
use reqwest::Error;

mod pastebin;
mod config;

use config::{APIConfiguration, write_api_key_to_config_file, deserialize_configuration_file};

#[derive(Parser, Debug)]
#[clap(author = "PasteBinGen", version, about = "A simple CLI for writing PasteBin texts.", long_about = None)]
pub struct Args {
    /// PasteBin API key
    #[clap(short, long, default_value = "default", required = false)]
    pub api_key: String,

    /// Path to file that you want to upload
    #[clap(short, long, required = true)]
    pub path: String,

    /// Syntax highlighting options
    #[clap(short = 'f', long, required = false, default_value = "text")]
    pub paste_format: String,

    /// Paste name
    #[clap(short = 'n', long, required = false, default_value = "untitled")]
    pub paste_name: String,
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    let config = APIConfiguration {
        api_key: args.api_key,
    };

    let mut config_file_path = match dirs::home_dir() {
        None => {
            eprintln!("Missing home directory");
            std::process::exit(1);
        },
        Some(path) => path,
    };
    config_file_path.push("config.json");

    write_api_key_to_config_file(&config_file_path, &config);

    let config_file = match deserialize_configuration_file(&config_file_path) {
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
        Ok(f) => f,
    };

    let api_key = match config_file.get("api_key") {
        Some(key) => key,
        None => {
            eprintln!("Missing API key");
            std::process::exit(1);
        }
    };

    let file_data = match fs::read_to_string(args.path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };

    let res = pastebin::post(
        api_key,
        &file_data,
        &args.paste_format,
        &args.paste_name
    );
    println!("{}", res.await?.text().await?);
    Ok(())
}
