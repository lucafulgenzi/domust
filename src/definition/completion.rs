use crate::definition::Config;

pub fn get_all_devices(config: &Config) -> String {
    let mut devices: String = String::new();

    for device in &config.devices {
        devices.push_str(&device.name);
        devices.push_str(" ");
    }
    return devices;
}

pub fn get_all_device_commands(config: &Config, device_name: String) -> String {
    let mut commands: String = String::new();

    for device in &config.devices {
        if device.name == device_name {
            for command in &device.commands {
                commands.push_str(&command.name);
                commands.push_str(" ");
            }
        }
    }
    return commands;
}