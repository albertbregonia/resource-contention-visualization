use std::{sync::Arc, time::Duration};

use tokio::{
    sync::{Barrier, Semaphore},
    time::Instant,
};

use crate::test_harness::{collect_latencies, spawn_n_tasks};

pub async fn semaphore_test(n: u32, permits: usize, spike: bool) -> Vec<Duration> {
    test(
        n,
        permits,
        spike.then_some(Arc::new(Barrier::new(n as usize + 1))),
    )
    .await
}

// the advantage of a semaphore is normally n access (permits) to a resource
// - with 1 permit: it becomes a mutex (a mutex internally uses a semaphore with 1 permit)
// - with 0 permits: it just deadlocks and hangs
// ref: https://docs.rs/tokio/latest/src/tokio/sync/mutex.rs.html#357-374
//
// - If the permit count is exhausted, the task suspends until it is able to obtain one: adding to its latency
// - The latency returned also includes scheduler latency
// - Semaphore is fair (FIFO) when trying to obtain a permit
// ref: https://docs.rs/tokio/latest/tokio/sync/struct.Semaphore.html
async fn test(n: u32, permits: usize, barrier: Option<Arc<Barrier>>) -> Vec<Duration> {
    let mutex = Arc::new(Semaphore::new(permits));
    let barrier_clone = barrier.clone();

    let tasks = spawn_n_tasks(n, move || {
        let mutex = mutex.clone();
        let barrier = barrier_clone.clone();
        async move {
            if let Some(barrier) = barrier {
                barrier.wait().await; // block until all n tasks have started
            }
            let start = Instant::now();
            drop(mutex.acquire().await);
            start.elapsed()
        }
    });
    if let Some(barrier) = barrier {
        barrier.wait().await; // release all tasks
    }
    collect_latencies(tasks).await
}
