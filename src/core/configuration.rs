use std::fs::File;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use shellexpand::tilde;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DeviceType {
    Broadlink,
    SwitchBot,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub broadlink: Broadlink,
    pub devices: Vec<Device>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Broadlink {
    pub manager_url: String,
    pub device_type: String,
    pub device_ip: String,
    pub device_mac: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Device {
    pub name: String,
    pub device_type: DeviceType,
    pub commands: Vec<Command>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Command {
    pub name: String,
    pub code: String,
}

pub fn read_config_file() -> Config {
    let config_file_path = tilde("~/.config/domust/config.yaml"); // TODO: search for .yaml and .yml

    let config_file = File::open(config_file_path.to_string()).expect("Unable to open config file");
    let config: Config = serde_yaml::from_reader(config_file).expect("Unable to parse config file");

    return config;
}

pub fn get_device_type(config: &Config, device_type: &String) -> DeviceType {
    let device = config.devices
        .iter()
        .find(|&d| &d.name == device_type)
        .cloned()
        .expect("Unable to find device");

    return device.device_type;
}

