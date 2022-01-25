use clap::*;

///
#[derive(Parser, Debug)]
#[clap(author = "PasteBinGen", version, about = "A simple CLI for writing PasteBin texts.", long_about = None)]
pub struct Args {
    /// PasteBin API key
    #[clap(short, long, default_value = "default")]
    pub api_key: String,
}

pub fn get_command_line_arguments() -> Args {
    Args::parse()
}
