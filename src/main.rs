use std::env;
use clap::Parser;
mod definition;
mod devices;

use crate::definition::{Cli, Config, DeviceType, get_all_device_commands};
use crate::definition::get_all_devices;
use crate::devices::exec_broadlink_command;


#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    set_log_level(cli.debug);
    env_logger::init();

    let config: Config = definition::read_config_file(cli.config);

    if cli.suggestion.is_some() {
        let suggestions = cli.suggestion.clone().expect("Error parsing suggestions");
        match suggestions.len() {
            0 => {
                println!("{}", get_all_devices(&config));
                std::process::exit(1);
            },
            1 => {
                println!("{:?}", get_all_device_commands(&config, suggestions[0].clone()));
                std::process::exit(1);
            },
            _ => {
                std::process::exit(1);
            }
        }


    }


    let device: String = cli.device_name.clone();
    let command: String = cli.device_command.clone();
    log::debug!("Device: {}", device);
    log::debug!("Command: {}", command);

    let device_type: DeviceType = definition::get_device_type(&config, &device);

    match device_type {
        DeviceType::Broadlink => {
            exec_broadlink_command(&config, &device, command).await;
        },
        DeviceType::SwitchBot => {
            println!("SwitchBot");
        }
    }

}

fn set_log_level(debug: bool) {
    if debug {
        env::set_var("RUST_LOG", "debug");
    } else {
        env::set_var("RUST_LOG", "info");
    }
}
