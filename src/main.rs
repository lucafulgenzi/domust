use std::env;
use clap::Parser;
mod definition;
mod devices;

use crate::definition::{Cli, Config, DeviceType};
use crate::devices::exec_broadlink_command;


#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    set_log_level(cli.debug);

    let device: String = cli.device_name.clone();
    let command: String = cli.device_command.clone();

    env_logger::init();

    log::debug!("Device: {}", device);
    log::debug!("Command: {}", command);

    let config: Config = definition::read_config_file(cli.config);
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