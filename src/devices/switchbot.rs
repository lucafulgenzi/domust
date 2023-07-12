use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use base64::engine::general_purpose;
use base64::Engine;
use hmac::{Hmac, Mac};
use reqwest::header::HeaderMap;
use sha2::Sha256;

use crate::core::{Config, Device};

type HmacSha256 = Hmac<Sha256>;

pub async fn exec_switchbot_command(config: &Config, device: &Device, command: String) {
    log::debug!(
        "Executing Switchbot command {:?}, {:?}, {}",
        config,
        device,
        command
    );
    check_device_options(device);

    let mut data = HashMap::new();
    data.insert("command", command);
    data.insert("parameter", "default".parse().unwrap());
    data.insert("commandType", "command".parse().unwrap());

    let token: String = config.switchbot.token.parse().unwrap();
    let nonce: String = config.switchbot.nonce.parse().unwrap();
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string();

    let string_to_sign = token + &*t + &*nonce;
    let key = string_to_sign.as_bytes();
    let mut mac = HmacSha256::new_from_slice(key).unwrap();
    mac.update(string_to_sign.as_ref());
    let result = mac.finalize();
    let result_bytes = result.into_bytes();
    let sign = general_purpose::STANDARD.encode(result_bytes);

    let mut switchbot_headers = HeaderMap::new();
    switchbot_headers.insert(
        "Content-type",
        "application/json; charset=utf8".parse().unwrap(),
    );
    switchbot_headers.insert("Authorization", config.switchbot.token.parse().unwrap());
    switchbot_headers.insert("sign", sign.parse().unwrap());
    switchbot_headers.insert("t", t.parse().unwrap());
    switchbot_headers.insert("nonce", nonce.parse().unwrap());

    let mut base_request_url: String = config.switchbot.base_url.to_string() + "/v1.1/devices/";
    base_request_url += &*device.switchbot_device_id;
    base_request_url.push_str("/commands");
    reqwest::Client::new()
        .post(&config.switchbot.base_url)
        .json(&data)
        .headers(switchbot_headers)
        .send()
        .await
        .unwrap_or_else(|e| {
            log::error!("Error sending request to broadlink manager: {:?}", e);
            std::process::exit(1);
        });
}

fn check_device_options(device: &Device) {
    let dev = device.clone();
    check_existence(dev.switchbot_device_id, "No switchbot device id");
    check_existence(dev.switchbot_device_name, "No switchbot device name");
    check_existence(dev.switchbot_device_type, "No switchbot device type");
}

fn check_existence(value: String, error_message: &str) {
    if Some(value) == None {
        log::error!("{}", error_message);
        std::process::exit(1);
    }
}
