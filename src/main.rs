use std::env;
use crate::devices::exec_broadlink_command;


mod core;
mod devices;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.is_empty() || args.len() < 3 {
        log::error!("No arguments provided");
        return;
    }

    let device: String = args[1].clone();
    let command: String = args[2].clone();
    let verbose: bool = args.len() > 3 && args[3] == "-v";

    if verbose {
        env::set_var("RUST_LOG", "debug");
    } else {
        env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    log::debug!("Device: {}", device);
    log::debug!("Command: {}", command);

    let config: core::Config = core::read_config_file();

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
