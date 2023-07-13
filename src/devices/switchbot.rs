use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use base64::engine::general_purpose;
use base64::Engine;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use ring::hmac::{self, Key, HMAC_SHA256};

use crate::definition::{Config, Device};

pub async fn exec_switchbot_command(config: &Config, device: &Device, command: String) {
    log::debug!(
        "Executing Switchbot command {:?}, {:?}, {}",
        config,
        device,
        command
    );

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

    log::debug!("t: {}", t);
    let string_to_sign = token + &*t + &*nonce;
    log::debug!("String to sign: {}", string_to_sign);
    let bytes_to_sign = string_to_sign.as_bytes();

    let result_bytes = sign_message(bytes_to_sign, &*config.switchbot.secret.as_bytes());

    let sign = general_purpose::STANDARD.encode(result_bytes);
    log::debug!("Sign: {}", sign);

    let mut switchbot_headers = HeaderMap::new();
    switchbot_headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_str("application/json; charset=utf8").unwrap(),
    );
    switchbot_headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&*config.switchbot.token).unwrap(),
    );
    switchbot_headers.insert("sign", HeaderValue::from_str(&*sign).unwrap());
    switchbot_headers.insert("t", HeaderValue::from_str(&*t).unwrap());
    switchbot_headers.insert("nonce", HeaderValue::from_str(&*nonce).unwrap());

    let mut base_request_url: String = config.switchbot.base_url.to_string() + "/v1.1/devices/";
    base_request_url += &*device.switchbot_device_id;
    base_request_url.push_str("/commands");

    log::debug!("Switchbot request URL: {}", base_request_url);
    log::debug!("Switchbot request headers: {:?}", switchbot_headers);
    log::debug!("Switchbot request data: {:?}", data);

    let res = reqwest::Client::new()
        .post(base_request_url)
        .json(&data)
        .headers(switchbot_headers)
        .send()
        .await
        .unwrap_or_else(|e| {
            log::error!("Error sending request to switchbot hub: {:?}", e);
            std::process::exit(1);
        });
    log::debug!("Switchbot response: {:?}", res);
}

fn sign_message(message: &[u8], secret: &[u8]) -> Vec<u8> {
    let key = Key::new(HMAC_SHA256, secret);
    let signature = hmac::sign(&key, message);
    signature.as_ref().to_vec()
}
