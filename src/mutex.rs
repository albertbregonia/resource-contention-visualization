use std::{sync::Arc, time::Duration};

use tokio::{
    sync::{Barrier, Mutex},
    time::Instant,
};

use crate::test_harness::{collect_latencies, spawn_n_tasks};

pub async fn mutex_test(n: u32, spike: bool) -> Vec<Duration> {
    // this test spawns n tasks, uses a barrier (if spike) to have them all wait until all tasks are spawned,
    // and then once released, all tasks start their timer to lock the mutex
    // this is high contention / request spike simulation
    //
    // in a normal system, requests usually come in gradually therefore (spike=false, without barrier) is a more realistic test
    // however, latency is driven by contention and this test ensures contention is lower
    test(n, spike.then_some(Arc::new(Barrier::new(n as usize + 1)))).await
}

// Mutex is FIFO ref: https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html.
// Therefore, the Nth task suspends until N-1th task unlocks: adding to its latency
// The latency returned also includes scheduler latency
// Meaning, when the N-1th task unlocks, it merely signals to wake up the Nth task
// but the Nth task still has to wait until the scheduler runs it
async fn test(n: u32, barrier: Option<Arc<Barrier>>) -> Vec<Duration> {
    let mutex = Arc::new(Mutex::new(0)); // 0 is just unused shared data
    let barrier_clone = barrier.clone();

    let tasks = spawn_n_tasks(n, move || {
        let mutex = mutex.clone();
        let barrier = barrier_clone.clone();
        async move {
            if let Some(barrier) = barrier {
                barrier.wait().await; // block until all n tasks have started
            }
            let start = Instant::now();
            drop(mutex.lock().await);
            start.elapsed()
        }
    });
    if let Some(barrier) = barrier {
        barrier.wait().await; // release all tasks
    }
    collect_latencies(tasks).await
}
