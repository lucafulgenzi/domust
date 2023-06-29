use std::env;

mod core;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.is_empty() || args.len() < 3 {
        println!("No arguments provided");
        return;
    }

    let device: String = args[1].clone();
    let command: String = args[2].clone();
    let config: core::Config = core::read_config_file();
    exec_broadlink_command(&config, device, command).await;
}


async fn exec_broadlink_command(config: &core::Config, device: String, command: String) {

    let broadlink_device = config.devices
        .iter()
        .find(|&d| d.name == device)
        .cloned()
        .expect("Unable to find device");

    let broadlink_command = broadlink_device.commands
        .iter()
        .find(|&c| c.name == command)
        .cloned()
        .expect("Unable to find command");

    let broadlink_request_url = format!("{}/command/send/", config.broadlink.manager_url);
    let broadlink_request_parameters = core::get_broadlink_request_parameters(&config.broadlink ,broadlink_command);

    reqwest::Client::new()
        .get(&broadlink_request_url)
        .query(&broadlink_request_parameters)
        .send()
        .await
        .expect("Internal Server Error");
}