mod network_info;
mod redis;
use std::{fs::File, process::Command};

use clap::Parser;
use log::*;
use simplelog::*;

use crate::network_info::NetworkInfo;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct WakeClientArgs {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,
    // IP Address of redis server
    #[arg(short, long)]
    redis_ip: String,
    //Shutdown commmand (default to shutdown)
    #[arg(short, long, default_value = "shutdown")]
    shutdown: String,
}

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create("wake-client.log").unwrap(),
        ),
    ])
    .unwrap();

    let args = WakeClientArgs::parse();
    let net_info = NetworkInfo::fetch();
    info!("Args: {:#?}", args);
    info!("MAC Address: {}", net_info.mac);
    info!("Sending mac adress to redis server at {}", args.redis_ip);
    let mut redis_comms = redis::RedisComms::new(args.name.clone(), args.redis_ip);
    match redis_comms.submit_mac_address(args.name, net_info) {
        Ok(_) => info!("MAC Address submitted successfully"),
        Err(e) => error!("Error submitting MAC Address: {}", e),
    }

    match redis_comms.subscribe_to_shutdown() {
        Ok(_) => info!("Subscribed to shutdown"),
        Err(e) => error!("Error subscribing to shutdown: {}", e),
    }

    loop {
        match redis_comms.should_shutdown() {
            Ok(true) => {
                info!(
                    "Received shutdown signal, running command: {}",
                    args.shutdown
                );
                // Execute shutdown command in shell
                let output = if cfg!(target_os = "windows") {
                    Command::new("cmd")
                        .args(["/C", &args.shutdown.clone()])
                        .output()
                } else {
                    Command::new("sh").arg("-c").arg(&args.shutdown).output()
                };
                match output {
                    Ok(output) => {
                        info!("Shutdown command executed successfully");
                        info!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                        info!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                    }
                    Err(e) => error!("Error executing shutdown command: {}", e),
                }
            }
            Ok(false) => {
                info!("No shutdown signal received");
            }
            Err(e) => error!("Error checking for shutdown signal: {}", e),
        }
        //sleep 1 second
    }
}
