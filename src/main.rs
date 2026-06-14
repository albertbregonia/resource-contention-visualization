mod test_harness;
mod semaphore;
mod mutex;
mod rwlock;

#[tokio::main(worker_threads=16)]
async fn main() {}
