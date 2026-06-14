use std::time::Duration;

use futures::future::JoinAll;
use tokio::task::JoinHandle;

pub fn spawn_n_tasks<F, Fut>(n: u32, f: F) -> JoinAll<JoinHandle<Duration>>
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Duration> + Send + 'static,
{
    futures::future::join_all((0..n).map(|_| tokio::spawn(f())).collect::<Vec<_>>())
}

pub async fn collect_latencies(handles: JoinAll<JoinHandle<Duration>>) -> Vec<Duration> {
    handles.await.into_iter().map(|r| r.expect("failed to join on task")).collect()
}
