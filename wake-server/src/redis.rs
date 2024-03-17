use clap::builder::Str;
use log::{debug, info};
use redis::{Commands, RedisError};

pub struct RedisComms {
    pub redis_ip: String,
    pub con: redis::Connection,
}

impl RedisComms {
    pub fn new(redis_ip: String) -> RedisComms {
        info!("Connecting to redis server at {}", redis_ip);
        let client = redis::Client::open(format!("redis://{}", redis_ip)).unwrap();
        let mut con = client.get_connection().unwrap();

        RedisComms { redis_ip, con }
    }

    pub fn get_client_mac(&mut self, name: String) -> redis::RedisResult<String> {
        self.con.get(format!("wake/{}/mac", name))
    }

    pub fn get_all_clients(&mut self) -> redis::RedisResult<Vec<String>> {
        self.con.smembers("wake/clients")
    }

    pub fn check_for_wake(&mut self, name: String) -> redis::RedisResult<bool> {
        // Check for /wake/{name}/wake existence
        let wake_key = format!("wake/{}/wake", name);
        debug!("Checking for wake key: {}", wake_key);
        let wake: bool = self.con.exists(wake_key.clone()).unwrap();
        if wake {
            // Remove the key
            debug!("Removing wake key: {}", wake_key);
            return self.con.del(wake_key);
        }
        Ok(wake)
    }
}
