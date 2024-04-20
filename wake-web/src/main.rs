mod redis;
use std::{fs::File, process::Command, sync::{Arc, Mutex}};

use clap::Parser;
use log::*;

use redis::RedisComms;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct WakeServerArgs {
    // IP Address of redis server
    #[arg(short, long)]
    redis_ip: String,
}


#[macro_use]
extern crate rocket;


#[get("/<name>/<action>")]
fn wake(name: String, action: String, redis_comms: &rocket::State<Arc<Mutex<RedisComms>>>) -> String {
    log::info!("Received request for {} to {}", name, action);
    let mut redis_comms = redis_comms.lock().unwrap();
    match action.as_str() {
        "wake" => {
            log::info!("Waking up {}", name);
            redis_comms.send_wake(name).unwrap();
            "OK".to_string()
        }
        "shutdown" => {
            log::info!("Shutting down {}", name);
            redis_comms.send_shutdown(name).unwrap();
            "OK".to_string()
        }
        "mac" => {
            log::info!("Getting MAC address for {}", name);
            redis_comms.get_client_mac(name).unwrap()
           
        }
        
        _ => "Invalid action".to_string(),
    }
}

#[get("/clients")]
fn get_clients(redis_comms: &rocket::State<Arc<Mutex<RedisComms>>>) -> String {
    let mut redis_comms = redis_comms.lock().unwrap();
    let clients = redis_comms.get_all_clients().unwrap();
    clients.join("\n")
}


#[launch]
fn rocket() -> _ {
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

    log::info!("Args: {:#?}", args);
    let redis_comms = redis::RedisComms::new(args.redis_ip.clone());
    let redis_comms = std::sync::Arc::new(Mutex::new(redis_comms));
    rocket::build()
        .manage(redis_comms)
        .mount("/", routes![wake, get_clients])
    
}
