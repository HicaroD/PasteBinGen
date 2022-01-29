use clap::*;

#[derive(Parser, Debug)]
#[clap(author = "PasteBinGen", version, about = "A simple CLI for writing PasteBin texts.", long_about = None)]
pub struct Args {
    /// PasteBin API key
    #[clap(short, long, default_value = "default", required = false)]
    pub api_key: String,

    /// Path to file that you want to upload
    #[clap(short, long, required = true)]
    pub path: String,
}

pub fn get_command_line_arguments() -> Args {
    Args::parse()
}
