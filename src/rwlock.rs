use std::{sync::Arc, time::Duration};

use tokio::{
    sync::{Barrier, RwLock},
    time::Instant,
};

use crate::test_harness::{collect_latencies, spawn_n_tasks};

pub async fn burst_rwlock_test(n: u32, read_chance: u8) -> Vec<Duration> {
    rwlock_test(n, read_chance, Some(Arc::new(Barrier::new(n as usize + 1)))).await
}

pub async fn gradual_rwlock_test(n: u32, read_chance: u8) -> Vec<Duration> {
    rwlock_test(n, read_chance, None).await
}

// RWLock is the same as a mutex (FIFO) but with n readers (technically not physically)
// Therefore, readers acquire a permit immediately if no writers have the lock.
// Otherwise, they block until the writer releases.
// TL;DR - it's a regular mutex during write but Semaphore with Semaphore::MAX permits when reading
// this does have overhead that is why it has a separate test
// 
// `read_chance` is an int from 0..=100 representing the percent chance to perform a read call
// aka 0 means only writes, 100+ means only reads
async fn rwlock_test(n: u32, read_chance: u8, barrier: Option<Arc<Barrier>>) -> Vec<Duration> {
    let mutex = Arc::new(RwLock::new(0));
    let barrier_clone = barrier.clone();
    let tasks = spawn_n_tasks(n, move || {
        let mutex = mutex.clone();
        let barrier = barrier_clone.clone();
        let read_only = rand::random_range(1..=100) <= read_chance;
        async move {
            if let Some(barrier) = barrier {
                barrier.wait().await; // block until all n tasks have started
            }
            let start = Instant::now();
            if read_only {
                drop(mutex.read().await);
            } else {
                drop(mutex.write().await);
            }
            start.elapsed()
        }
    });
    if let Some(barrier) = barrier {
        barrier.wait().await; // release all tasks
    }
    collect_latencies(tasks).await
}
