use std::fs::File;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use base64::{engine::general_purpose, Engine as _};
use hex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DeviceType {
    Broadlink
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub broadlink: Broadlink,
    pub devices: Vec<Device>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct  Broadlink {
    pub manager_url: String,
    pub device_type: String,
    pub device_ip: String,
    pub device_mac: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Device {
    pub name: String,
    pub device_type: DeviceType,
    pub commands: Vec<Command>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Command {
    pub name: String,
    pub code: String
}

pub fn read_config_file() -> Config {
    let file_path = "config.yaml";

    let config_file = File::open(file_path).expect("Unable to open config file");
    let config: Config = serde_yaml::from_reader(config_file).expect("Unable to parse config file");

    return config;
}


pub fn get_broadlink_request_parameters(config: &Broadlink, command: Command) -> [(&'static str, String); 4] { //TODO: create a struct for this

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