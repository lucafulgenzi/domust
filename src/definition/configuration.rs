use std::fs::File;
use std::path::PathBuf;
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

pub fn read_config_file(config_path: Option<PathBuf>) -> Config {

    let config_file_path = config_path.unwrap_or_else(|| {
        let mut path = PathBuf::new();
        path.push(tilde("~/.config/domust/config.yaml").to_string());
        path
    });
    log::debug!("Config file path: {:?}", config_file_path);

    let config_file = File::open(config_file_path.clone()).unwrap_or_else(|_| {
        log::error!("No config file found in {:?}", config_file_path);
        std::process::exit(1);
    });
    log::debug!("Config file: {:?}", config_file);

    let config: Config = serde_yaml::from_reader(config_file).unwrap_or_else(|e| {
        log::error!("Error reading config file: {:?}", e);
        std::process::exit(1);
    });
    log::debug!("Config: {:?}", config);

    return config;
}

pub fn get_device_type(config: &Config, device_type: &String) -> DeviceType {
    let device = config.devices
        .iter()
        .find(|&d| &d.name == device_type)
        .cloned()
        .unwrap_or_else(|| {
            log::error!("Device {:?} not found in config file", device_type);
            std::process::exit(1);
        });

    log::debug!("Device: {:?}", device);
    return device.device_type;
}

