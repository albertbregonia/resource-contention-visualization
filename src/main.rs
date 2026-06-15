use std::env;

use serde::Deserialize;
use tokio::fs;

use crate::{
    channel::channel_test, mutex::mutex_test, rwlock::rwlock_test, semaphore::semaphore_test,
    unbounded_channel::unbounded_channel_test,
};

mod channel;
mod mutex;
mod rwlock;
mod semaphore;
mod test_harness;
mod unbounded_channel;

// when serialized test_type is one of:

// {"Semaphore": {"permits": 100}}
// {"RwLock": {"read_chance": 100}}
// {"Channel": {"buffer_size": 100}}
// "UnboundedChannel"
// "Mutex"
#[derive(Deserialize)]
enum TestType {
    Mutex,
    Semaphore { permits: usize },
    RwLock { read_chance: u8 },
    Channel { buffer_size: usize },
    UnboundedChannel,
}

#[derive(Deserialize)]
struct Config {
    test_type: TestType,
    task_count: u32,
    spike: bool,
}

const JSON_CONFIG_PATH_KEY: &str = "CONFIG_PATH";
const JSON_CONFIG_DEFAULT_PATH: &str = "./config.json";

// as this is not production code and simply a test structure
// `unwrap()/expect()` is used extensively as many cases
// can guarantee it does not `unwrap()`` an error
#[tokio::main(worker_threads = 16)]
async fn main() {
    let config_path =
        env::var(JSON_CONFIG_PATH_KEY).unwrap_or(JSON_CONFIG_DEFAULT_PATH.to_string());
    let config = fs::read(config_path)
        .await
        .expect("Failed to read JSON config file at given path");
    let config = serde_json::from_slice::<Config>(&config)
        .expect("Failed to deserialize file as JSON config");
    let mut dataset = match config.test_type {
        TestType::Mutex => mutex_test(config.task_count, config.spike).await,
        TestType::Semaphore { permits } => {
            semaphore_test(config.task_count, permits, config.spike).await
        }
        TestType::RwLock { read_chance } => {
            rwlock_test(config.task_count, read_chance, config.spike).await
        }
        TestType::Channel { buffer_size } => {
            channel_test(config.task_count, buffer_size, config.spike).await
        }
        TestType::UnboundedChannel => unbounded_channel_test(config.task_count, config.spike).await,
    };
    dataset.sort(); // data needs to be sorted first before histogram
    for v in dataset.into_iter().map(|d| d.as_secs_f64() * 1_000_000.0) {
        println!("{v}"); // convert to us and pipe to python
    }
}
