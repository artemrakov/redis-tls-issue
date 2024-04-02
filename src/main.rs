use std::io::Read;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::{fs::File, io::BufReader};

use redis::{Client, ClientTlsConfig, Commands, TlsCertificates};

#[derive(Clone)]
pub struct RedisClient {
    pool: r2d2::Pool<Client>,
}

impl RedisClient {
    pub fn new(url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::open(url).unwrap();
        let pool = r2d2::Pool::builder()
            .max_size(10)
            .min_idle(Some(5))
            .build(client)
            .unwrap();

        let redis = RedisClient { pool };
        Ok(redis)
    }

    pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>, redis::RedisError> {
        let mut con = self.pool.get().unwrap();
        let value: Vec<u8> = redis::cmd("GET").arg(key).query(&mut con).unwrap();

        Ok(Some(value))
    }

    pub fn set(&self, key: &str, value: Vec<u8>) -> Result<(), redis::RedisError> {
        let mut con = self.pool.get().unwrap();

        redis::cmd("SET").arg(key).arg(value).query(&mut con)?;

        Ok(())
    }
}

fn main() {
    println!("Connecting to redis");
    let client = RedisClient::new("rediss://127.0.0.1:6379#insecure").unwrap();

    println!("Connected to redis. Starting the loop");

    let value = vec![0; 20 * 1024 * 1024]; // 20MB value
    client.set("key", value).unwrap();

    for i in 1..10 {
        println!("This is loop number {}", i);
        let start_time = Instant::now();

        let result = client.get("key").unwrap();
        println!("Is some: {}", result.is_some());

        let end_time = start_time.elapsed().as_millis();
        println!("Redis get latency_ms={}", end_time);

        sleep(Duration::from_secs(1))
    }
}
