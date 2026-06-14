use std::{sync::Arc, time::Duration};

use tokio::{
    sync::{Barrier, mpsc, oneshot},
    time::Instant,
};

use crate::{
    channel::TestRequestType,
    test_harness::{collect_latencies, spawn_n_tasks},
};

type ActorSenderHandle = mpsc::UnboundedSender<TestRequestType>;

pub async fn unbounded_channel_test(n: u32, spike: bool) -> Vec<Duration> {
    if spike {
        test(n, Some(Arc::new(Barrier::new(n as usize + 1)))).await
    } else {
        // Without the barrier, the actor is already active
        // Therefore, many requests are able to resolve immediately, guaranteeing that the queue will never reach a length of `n`
        // However, tokio still has to perform work on when to park/unpark tasks (suspend, wake) contributing to latency
        test(n, None).await
    }
}

async fn test(n: u32, barrier: Option<Arc<Barrier>>) -> Vec<Duration> {
    let handle = spawn_actor(barrier.clone());
    let tasks = spawn_n_tasks(n, move || {
        call_actor_await_response(handle.clone(), barrier.clone())
    });
    collect_latencies(tasks).await
}

// spawns a dedicated tokio task for handling the receiver end of the mpsc::channel
// and returns the corresponding Sender<> handle
// this is usually called first in tests because we want to replicate a server ready waiting for requests
// `log_contention` is a feature flag to show the max queue length.
// it is disabled by default to keep tests truly fair (despite marginal impact)
fn spawn_actor(barrier: Option<Arc<Barrier>>) -> ActorSenderHandle {
    let (sender, mut receiver) = mpsc::unbounded_channel::<TestRequestType>();
    tokio::spawn(async move {
        #[cfg(feature = "log_contention")]
        let mut max_contention = 0;
        if let Some(barrier) = barrier {
            barrier.wait().await;
        }
        while let Some(reply) = receiver.recv().await {
            reply.send(()).unwrap();
            #[cfg(feature = "log_contention")]
            {
                let contention = receiver.len();
                if contention > max_contention {
                    max_contention = contention;
                }
            }
        }
        #[cfg(feature = "log_contention")]
        // not an actual error, simply a workaround as we pipe stdout
        eprintln!(
            "[unbounded_channel] max contention experienced: {}",
            max_contention
        );
    });
    sender
}

// creates a oneshot to have the actor respond on
// given a Barrier, this task will wait until the green light to send in requests
// and then start the timer to simulate the end-to-end request
// including time to send and time to receive response
// the channel is never full (unbounded) therefore all the time is spend waiting on receiving
async fn call_actor_await_response(
    actor_handle: ActorSenderHandle,
    barrier: Option<Arc<Barrier>>,
) -> Duration {
    let (tx, rx) = oneshot::channel();
    if let Some(barrier) = barrier {
        barrier.wait().await; // wait until all tasks have spawned
    }
    let start = Instant::now();
    actor_handle.send(tx).unwrap();
    rx.await.unwrap();
    start.elapsed()
}
