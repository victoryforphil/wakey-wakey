use clap::builder::Str;
use log::{debug, info};
use redis::{Commands, RedisError};

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
        let _res: Result<String, RedisError> = con.sadd("wake/clients".to_string(), name.clone());
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
        Ok(())
    }

    pub fn should_shutdown(&mut self) -> redis::RedisResult<bool> {
        // Check for /wake/{name}/wake existence
        let shutdown_key = format!("wake/{}/shutdown", self.name);
        debug!("Checking for wake key: {}", shutdown_key);
        let wake: bool = self.con.exists(shutdown_key.clone()).unwrap();
        if wake {
            // Remove the key
            debug!("Removing shutdown key: {}", shutdown_key);
            return self.con.del(shutdown_key);
        }
        Ok(wake)
    }
}
