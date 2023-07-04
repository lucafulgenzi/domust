use std::path::PathBuf;
use clap::{Parser};

#[derive(Parser)]
#[command(version, name = "Domust", author, about)]
pub struct Cli {

    #[arg(default_value_t = String::from(""))]
    pub device_name: String,

    #[arg(default_value_t = String::from(""))]
    pub device_command: String,

    /// Path to config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[arg(short, long, value_parser, num_args = 0.., value_delimiter = ' ')]
    pub suggestion: Option<Vec<String>>,

    #[arg(short, long)]
    pub debug: bool,
}