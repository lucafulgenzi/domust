use base64::{engine::general_purpose, Engine as _};
use hex;
use crate::definition::{Broadlink, Command, Config};


const BROADLINK_COMMAND_ENDPOINT: &str = "/command/send/";

fn get_broadlink_request_parameters(config: &Broadlink, command: Command) -> [(&'static str, String); 4] {

    let decoded_command = general_purpose::STANDARD.decode(command.code).unwrap_or_else(|e| {
        log::error!("Error decoding command: {:?}", e);
        std::process::exit(1);
    });
    log::debug!("Decoded command: {:?}", decoded_command);

    let hex_command = hex::encode(decoded_command);
    log::debug!("Hex command: {}", hex_command);

    let params = [
        ("type", config.device_type.to_owned()),
        ("host", config.device_ip.to_owned()),
        ("mac", config.device_mac.to_owned()),
        ("command", hex_command)
    ];
    log::debug!("Request parameters: {:?}", params);

    return params;
}

pub async fn exec_broadlink_command(config: &Config, device: &String, command: String) {

    let broadlink_device = config.devices
        .iter()
        .find(|&d| &d.name == device)
        .cloned()
        .unwrap_or_else(|| {
            log::error!("Device {:?} not found in config file", device);
            std::process::exit(1);
        });
    log::debug!("Broadlink device: {:?}", broadlink_device);

    let broadlink_command = broadlink_device.commands
        .iter()
        .find(|&c| c.name == command)
        .cloned()
        .unwrap_or_else(|| {
            log::error!("Command {:?} not found in config file", command);
            std::process::exit(1);
        });
    log::debug!("Broadlink command: {:?}", broadlink_command);

    let broadlink_request_url = format!("{}{}", config.broadlink.manager_url, BROADLINK_COMMAND_ENDPOINT);
    log::debug!("Broadlink request URL: {}", broadlink_request_url);

    let broadlink_request_parameters = get_broadlink_request_parameters(&config.broadlink ,broadlink_command);
    log::debug!("Broadlink request parameters: {:?}", broadlink_request_parameters);

    reqwest::Client::new()
        .get(&broadlink_request_url)
        .query(&broadlink_request_parameters)
        .send()
        .await
        .unwrap_or_else(|e| {
            log::error!("Error sending request to broadlink manager: {:?}", e);
            std::process::exit(1);
        });
}