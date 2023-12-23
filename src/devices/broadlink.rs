use base64::{engine::general_purpose, Engine as _};

use crate::definition::{Broadlink, Command, Config, Device};

static BROADLINK_COMMAND_ENDPOINT: &str = "/command/send/";

fn get_broadlink_request_parameters(config: &Option<Broadlink>, command: Command) -> [(&'static str, String); 4] {
    let decoded_command = general_purpose::STANDARD
        .decode(command.code)
        .unwrap_or_else(|e| {
            log::error!("Error decoding command: {:?}", e);
            std::process::exit(1);
        });
    log::debug!("Decoded command: {:?}", decoded_command);

    let hex_command = hex::encode(decoded_command);
    log::debug!("Hex command: {}", hex_command);

    let param_type = config.clone().unwrap_or_else(|| {
        log::error!("No broadlink config");
        std::process::exit(1);
    }).device_type;

    let param_host = config.clone().unwrap_or_else(|| {
        log::error!("No broadlink config");
        std::process::exit(1);
    }).device_ip;

    let param_mac = config.clone().unwrap_or_else(|| {
        log::error!("No broadlink config");
        std::process::exit(1);
    }).device_mac;

    let params = [
        ("type", param_type.to_owned()),
        ("host", param_host.to_owned()),
        ("mac", param_mac.to_owned()),
        ("command", hex_command),
    ];
    log::debug!("Request parameters: {:?}", params);

    params
}

pub async fn exec_broadlink_command(config: &Config, broadlink_device: &Device, command: String) {
    let broadlink_command = broadlink_device
        .commands
        .iter()
        .find(|&c| c.name == command)
        .cloned()
        .unwrap_or_else(|| {
            log::error!("Command {:?} not found in config file", command);
            std::process::exit(1);
        });
    log::debug!("Broadlink command: {:?}", broadlink_command);

    let broadlink_manager_url = config.broadlink.clone().unwrap_or_else(|| {
        log::error!("No broadlink config");
        std::process::exit(1);
    }).manager_url;
    let broadlink_request_url = format!( "{}{}", broadlink_manager_url, BROADLINK_COMMAND_ENDPOINT );
    log::debug!("Broadlink request URL: {}", broadlink_request_url);

    let broadlink_request_parameters =
        get_broadlink_request_parameters(&config.broadlink, broadlink_command);
    log::debug!(
        "Broadlink request parameters: {:?}",
        broadlink_request_parameters
    );

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
