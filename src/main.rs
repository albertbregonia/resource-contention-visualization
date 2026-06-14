mod channel;
mod mutex;
mod rwlock;
mod semaphore;
mod test_harness;
mod unbounded_channel;

// as this is not production code and simply a test structure
// `unwrap()` is used extensively as many cases
// can guarantee it does not `unwrap()`` an error
#[tokio::main(worker_threads = 16)]
async fn main() {}
