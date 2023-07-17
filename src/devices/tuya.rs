use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;
use std::time::SystemTime;

use log::log;
use rust_tuyapi::tuyadevice::TuyaDevice;
use rust_tuyapi::{Payload, PayloadStruct};
use serde_json::json;

use crate::definition::{Config, Device};

pub async fn exec_tuya_command(config: &Config, device: &Device, command: String) {
    log::debug!(
        "Executing Tuya command {:?}, {:?}, {}",
        config,
        device,
        command
    );

    let status: bool;
    if command.contains("on") {
        status = true;
    } else if command.contains("off") {
        status = false;
    } else {
        log::error!("Unknown command: {}", command);
        std::process::exit(1);
    }

    let mut dps = HashMap::new();
    dps.insert("1".to_string(), json!(status));
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;

    let payload = Payload::Struct(PayloadStruct {
        dev_id: device.tuya_id.to_string(),
        gw_id: None,
        uid: Some(device.tuya_id.to_string()),
        t: Some(current_time),
        dp_id: None,
        dps: Some(dps),
    });

    let mut version = "ver".to_string();
    version.push_str(&device.tuya_ver);

    log::debug!("Tuya version: {}", version);
    log::debug!("Tuya device id: {}", device.tuya_id);
    log::debug!("Tuya device ip: {}", device.tuya_ip);
    log::debug!("Tuya device key: {}", device.tuya_key);
    log::debug!("Tuya payload: {:?}", payload);

    // let tuya_device = TuyaDevice::create_with_transport(
    //     &*version,
    //     Some(&*device.tuya_key),
    //     IpAddr::from_str(&device.tuya_ip).unwrap(),
    //     Transport::TCP(6668),
    // ).expect("Cannot create Tuya device");

    let tuya_device = TuyaDevice::create(
        &*version,
        Some(&*device.tuya_key),
        IpAddr::from_str(&device.tuya_ip).unwrap(),
    )
    .expect("Cannot create Tuya device");

    tuya_device.set(payload, 0).expect("Cannot set Tuya device");
}
