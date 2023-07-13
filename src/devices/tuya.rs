use crate::definition::{Config, Device};

pub async fn exec_tuya_command(config: &Config, device: &Device, command: String) {
    log::debug!(
        "Executing Tuya command {:?}, {:?}, {}",
        config,
        device,
        command
    );
}
