mod redis;
use std::{fs::File, process::Command};

use clap::Parser;
use log::*;
use redis::RedisComms;
use rocket::form::name;
use simplelog::*;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct WakeServerArgs {
    // IP Address of redis server
    #[arg(short, long)]
    redis_ip: String,
}

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
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

    let args = WakeServerArgs::parse();

    info!("Args: {:#?}", args);

    let mut redis_comms = redis::RedisComms::new(args.redis_ip);
    let clients = redis_comms.get_all_clients().unwrap();
    info!("Clients: {:?}", clients);

    loop {
        for client in clients.clone() {
            debug!("Checking for wake for {}", client);
            let should_wake = redis_comms.check_for_wake(client.clone()).unwrap();
            if should_wake {
                info!("Should wake {}", client);
                wake_client(&mut redis_comms, client);
            }
        }
        // sleep for 1s
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn wake_client(comms: &mut RedisComms, name: String) {
    let mac = comms.get_client_mac("test_client".to_string());
    match mac {
        Ok(mac) => {
            info!("Got mac: {}", mac);
            let wol = wakey::WolPacket::from_string(&mac, ':').unwrap();
            if wol.send_magic().is_ok() {
                println!("Sent the magic packet.");
            } else {
                println!("Failed to send the magic packet.");
            }
        }
        Err(e) => {
            error!("Error: {}", e);
        }
    }
}
