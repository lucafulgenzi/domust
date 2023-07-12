use std::env;
use std::path::PathBuf;

use clap::Parser;

use crate::core::Command;
use crate::devices::{exec_broadlink_command, exec_switchbot_command, exec_tuya_command};

mod core;
mod devices;

static RUST_LOG: &str = "RUST_LOG";

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

    if env::var(RUST_LOG).is_err() {
        if cli.debug {
            env::set_var(RUST_LOG, "debug");
        } else {
            env::set_var(RUST_LOG, "info");
        }
    }

    let device_name: String = cli.device_name.clone();
    let command: String = cli.device_command.clone();

    env_logger::init();

    log::debug!("Device: {}", device_name);
    log::debug!("Command: {}", command);

    let config: core::Config = core::read_config_file(cli.config);
    let device = core::get_device(&config, &device_name);

    device
        .commands
        .iter()
        .find(|&d: &&Command| d.name == command)
        .cloned()
        .unwrap_or_else(|| {
            log::error!("Command {:?} not found in config file", command);
            std::process::exit(1);
        });

    match device.device_type {
        core::DeviceType::Broadlink => {
            exec_broadlink_command(&config, &device, command).await;
        }
        core::DeviceType::SwitchBot => {
            exec_switchbot_command(&config, &device, command).await;
        }
        core::DeviceType::Tuya => {
            exec_tuya_command(&config, &device, command).await;
        }
    }
}
