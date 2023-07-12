use std::fs::File;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use shellexpand::tilde;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DeviceType {
    Broadlink,
    SwitchBot,
    Tuya,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub broadlink: Broadlink,
    pub switchbot: Switchbot,
    pub tuya: Tuya,
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
pub struct Switchbot {
    pub base_url: String,
    pub token: String,
    pub secret: String,
    pub nonce: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tuya {
    pub api_key: String,
    pub api_secret: String,
    pub api_region: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Device {
    pub name: String,
    pub device_type: DeviceType,
    pub commands: Vec<Command>,

    #[serde(default)]
    pub switchbot_device_id: String,
    #[serde(default)]
    pub switchbot_device_name: String,
    #[serde(default)]
    pub switchbot_device_type: String,
    #[serde(default)]
    pub tuya_id: String,
    #[serde(default)]
    pub tuya_ip: String,
    #[serde(default)]
    pub tuya_key: String,
    #[serde(default)]
    pub tuya_ver: String,
    #[serde(default)]
    pub tuya_mac: String,
}
//
// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct BroadlinkDevice {
//     pub name: String,
//     pub device_type: DeviceType,
//     pub commands: Vec<Command>,
// }
//
// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct SwitchbotDevice {
//     pub name: String,
//     pub device_type: DeviceType,
//     pub commands: Vec<Command>,
//     pub switchbot_device_id: String,
//     pub switchbot_device_name: String,
//     pub switchbot_device_type: String,
// }
//
// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct TuyaDevice {
//     pub name: String,
//     pub device_type: DeviceType,
//     pub commands: Vec<Command>,
//     pub tuya_id: String,
//     pub tuya_ip: String,
//     pub tuya_key: String,
//     pub tuya_ver: String,
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Command {
    pub name: String,
    pub code: String,
}

// impl Device for BroadlinkDevice {
//     fn match_type(self);
// }

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

pub fn get_device(config: &Config, device_name: &String) -> Device {
    let device = config
        .devices
        .iter()
        .find(|&d| &d.name == device_name)
        .cloned()
        .unwrap_or_else(|| {
            log::error!("Device {:?} not found in config file", device_name);
            std::process::exit(1);
        });

    log::debug!("Device: {:?}", device);
    return device;
}
