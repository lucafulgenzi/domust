use std::path::PathBuf;
use clap::{Parser};
use std::env;

mod core;
mod devices;
use crate::devices::exec_broadlink_command;

#[derive(Parser)]
#[command(version, name = "Domust", author, about)]
struct Cli {

    device_name: String,
    device_command: String,

    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long)]
    debug: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if cli.debug {
        env::set_var("RUST_LOG", "debug");
    } else {
        env::set_var("RUST_LOG", "info");
    }

    let device: String = cli.device_name.clone();
    let command: String = cli.device_command.clone();

    env_logger::init();

    log::debug!("Device: {}", device);
    log::debug!("Command: {}", command);

    let config: core::Config = core::read_config_file(cli.config);
    let device_type = core::get_device_type(&config, &device);

    match device_type {
        core::DeviceType::Broadlink => {
            exec_broadlink_command(&config, &device, command).await;
        },
        core::DeviceType::SwitchBot => {
            println!("SwitchBot");
        }
    }

}
