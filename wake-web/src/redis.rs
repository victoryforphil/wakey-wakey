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

    pub fn send_wake(&mut self, name: String) -> redis::RedisResult<()> {
        let wake_key = format!("wake/{}/wake", name);
        debug!("Sending wake key: {}", wake_key);
        self.con.set(wake_key, "1")
    }

    pub fn send_shutdown(&mut self, name: String) -> redis::RedisResult<()> {
        let shutdown_key = format!("wake/{}/shutdown", name);
        debug!("Sending shutdown key: {}", shutdown_key);
        self.con.set(shutdown_key, "1")
    }
}
