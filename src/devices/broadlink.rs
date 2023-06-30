use base64::{engine::general_purpose, Engine as _};
use hex;
use crate::core::{Broadlink, Command, Config};


const BROADLINK_COMMAND_ENDPOINT: &str = "/command/send/";

fn get_broadlink_request_parameters(config: &Broadlink, command: Command) -> [(&'static str, String); 4] {

    let decoded_command = general_purpose::STANDARD.decode(command.code).expect("Unable to decode command");
    let hex_command = hex::encode(decoded_command);

    let params = [
        ("type", config.device_type.to_owned()),
        ("host", config.device_ip.to_owned()),
        ("mac", config.device_mac.to_owned()),
        ("command", hex_command)
    ];

    return params;
}

pub async fn exec_broadlink_command(config: &Config, device: &String, command: String) {

    let broadlink_device = config.devices
        .iter()
        .find(|&d| &d.name == device)
        .cloned()
        .expect("Unable to find device");

    let broadlink_command = broadlink_device.commands
        .iter()
        .find(|&c| c.name == command)
        .cloned()
        .expect("Unable to find command");

    let broadlink_request_url = format!("{}{}", config.broadlink.manager_url, BROADLINK_COMMAND_ENDPOINT);
    let broadlink_request_parameters = get_broadlink_request_parameters(&config.broadlink ,broadlink_command);

    reqwest::Client::new()
        .get(&broadlink_request_url)
        .query(&broadlink_request_parameters)
        .send()
        .await
        .expect("Internal Server Error");
}