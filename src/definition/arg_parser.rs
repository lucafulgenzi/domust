use std::path::PathBuf;
use clap::{Parser};

#[derive(Parser)]
#[command(version, name = "Domust", author, about)]
pub struct Cli {

    pub device_name: String,
    pub device_command: String,

    /// Path to config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Activate debug mode
    #[arg(short, long)]
    pub debug: bool,
}