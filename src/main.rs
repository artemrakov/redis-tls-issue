use std::thread::sleep;
use std::time::{Duration, Instant};

use bb8_redis::{bb8, RedisConnectionManager};

#[tokio::main]
async fn main() {
    println!("Connecting to redis");
    // let client = Client::open("rediss://127.0.0.1:6379#insecure").unwrap();

    let manager = RedisConnectionManager::new("rediss://127.0.0.1:6379#insecure").unwrap();
    let pool = bb8::Pool::builder()
        .max_size(10)
        .min_idle(3)
        .build(manager)
        .await
        .unwrap();

    println!("Connected to redis. Starting the loop");
    let value = vec![0; 20 * 1024 * 1024]; // 20MB value

    // let mut con = client.get_multiplexed_tokio_connection().await.unwrap();
    let mut con = pool.get().await.unwrap();
    let _: Result<(), redis::RedisError> = redis::cmd("SET")
        .arg("key")
        .arg(value)
        .query_async(&mut *con)
        .await;

    for i in 1..10 {
        let mut con = pool.get().await.unwrap();
        let start_time = Instant::now();

        let value: Option<Vec<u8>> = redis::cmd("GET")
            .arg("key")
            .query_async(&mut *con)
            .await
            .unwrap();

        let duration = start_time.elapsed().as_millis();
        println!(
            "LoopNumber: {}, RedisGet latency_ms={}, value.is_some()={}",
            i,
            duration,
            value.is_some()
        );

        sleep(Duration::from_secs(1))
    }
}
