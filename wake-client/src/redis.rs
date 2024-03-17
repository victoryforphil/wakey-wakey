use clap::builder::Str;
use log::info;
use redis::Commands;

use crate::network_info::NetworkInfo;

pub struct RedisComms {
    pub name: String,
    pub redis_ip: String,
    pub con: redis::Connection,
}

impl RedisComms {
    pub fn new(name: String, redis_ip: String) -> RedisComms {
        info!("Creating RedisComms for {}", name);
        info!("Connecting to redis server at {}", redis_ip);
        let client = redis::Client::open(format!("redis://{}", redis_ip)).unwrap();
        let mut con = client.get_connection().unwrap();

        RedisComms {
            name,
            redis_ip,
            con,
        }
    }

    pub fn submit_mac_address(
        &mut self,
        name: String,
        mac: NetworkInfo,
    ) -> redis::RedisResult<String> {
        self.con.set(format!("wake/{}/mac", name), mac.mac)
    }

    pub fn subscribe_to_shutdown(&mut self) -> redis::RedisResult<()> {
        info!("Subscribing to wake/{}/shutdown", self.name);

        Ok(())
    }

    pub fn should_shutdown(&mut self) -> redis::RedisResult<bool> {
        let mut sub = self.con.as_pubsub();
        sub.subscribe(format!("wake/{}/shutdown", self.name))?;
        let msg = sub.get_message()?;
        let payload: String = msg.get_payload()?;
        Ok(payload == "true")
    }
}
