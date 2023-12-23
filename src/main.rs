use std::env;
use clap::Parser;

use crate::definition::{Cli, get_all_device_commands, get_all_devices, Command, Config};
use crate::devices::{exec_broadlink_command, exec_switchbot_command};

mod definition;
mod devices;

static RUST_LOG: &str = "RUST_LOG";


#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    env_logger::init();
    set_log_level(cli.debug);

    let device_name: String = cli.device_name.clone();
    let command: String = cli.device_command.clone();

    log::debug!("Device: {}", device_name);
    log::debug!("Command: {}", command);

    let config: definition::Config = definition::read_config_file(cli.config);

    if cli.suggestion.is_some() {
        check_completion(&config, cli.suggestion.unwrap());
    }

    let device = definition::get_device(&config, &device_name);

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
        definition::DeviceType::Broadlink => {
            exec_broadlink_command(&config, &device, command).await;
        }
        definition::DeviceType::SwitchBot => {
            exec_switchbot_command(&config, &device, command).await;
        }
    }
}

fn set_log_level(debug: bool) {
    if env::var(RUST_LOG).is_err() {
        if debug {
            env::set_var(RUST_LOG, "debug");
        } else {
            env::set_var(RUST_LOG, "info");
        }
    }
}

fn check_completion(config: &Config, suggestions: Vec<String>) {
    match suggestions.len() {
        0 => {
            println!("{}", get_all_devices(config));
            std::process::exit(1);
        }
        1 => {
            println!("{}", get_all_device_commands(config, suggestions[0].clone()));
            std::process::exit(1);
        }
        _ => {
            std::process::exit(1);
        }
    }
}
