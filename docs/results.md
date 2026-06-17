# Overview
Different concurrency primitives allow access to shared resources in various ways. In the context of async Rust, tokio-rs and its corresponding scheduler: when a task (green thread) is suspended waiting for access, each primitive has a distinct method to determine task readiness for said resource. Therefore, this affects when the task is woken up, added to the runnable queue, and eventually executed by a worker thread (OS thread). As a result, this coordination between the concurrency primitive and the scheduler drive resource-access latency.

Therefore, we are looking to understand:
- Given 16 tokio worker threads (OS threads),
- Given 10,000 tasks (green threads) all requesting to access a shared resource,
- When using various concurrency primitives,
- How long does it take the system to access the requested resource?

**In other words, under high contention, how does resource-access latency vary across concurrency primitives?**

This idea is important, as synchronization overhead can drive latency just as much as resource contention when understanding throughput.

### Invariants:
1. Little to no work is actually performed when obtaining access to shared memory.
    - This is done to ensure latency variance is due to concurrency primitives affecting task readiness, scheduler behavior, and subsequently observed latency.
    - Actor (channel) responds with `()` as soon as the request is received
    - Mutex / RwLock and Semaphore all `drop(..)` their locks/permits as soon as they obtain them
    - The latency of these operations are considered negligible
2. All tokio concurrency primitives are FIFO for their waiters but not for the scheduler.
    - Tokio's scheduler optimizes work and reorders the queue of runnable tasks.
    - For mutexes, even though the Nth task may be woken up, they may get scheduled later in favor of other tasks [not accessing the mutex] therefore driving *up* latency.
    - For channels, even though the receiver may handle the Nth task later than the N-1th task, the scheduler may execute the Nth task first, seeing the response earlier, driving *down* latency.
    - In our case, there are no other tasks to reorder in favor of more "meaningful work". Therefore, we shouldn't see this issue but if it did, ultimately, it contributes to the observed latency when using tokio.

# Channel

| Bounded Channel @ buffer_size=10_000 | Unbounded Channel @ buffer_size=inf. | Bounded Channel @ buffer_size=16 |
| - | - | - |
| ![mpsc::channel @ buffer_size=10_000](./histograms/channel.gif) | ![mpsc::unbounded_channel @ buffer_size=inf.](./histograms/unbounded.gif) | ![mpsc::channel buffer_size=16](./histograms/small_buffer_channel.gif) |

# Mutex

![sync::Mutex](./histograms/mutex.gif)

# RwLock

| RwLock @ 50% Read 50% Write | RwLock @ 0% Read 100% Write | RwLock @ 100% Read 0% Write | RwLock @ 99% Read 1% Write |
| - | - | - | - |
| ![sync::RwLock @ 50% Read 50% Write ](./histograms/rwlock.gif) | ![sync::RwLock @ 0% Read 100% Write](./histograms/writeonly_rwlock.gif) | ![sync::RwLock @ 100% Read 0% Write](./histograms/readonly_rwlock.gif) | ![sync::RwLock @ 99% Read 1% Write](./histograms/read99_rwlock.gif) | 

# Semaphore

| Semaphore @ permits=1 | Semaphore @ permits=n=10_000 | 
| - | - |
| ![sync::Semaphore @ permits=1](./histograms/semaphore.gif) | ![sync::Semaphore @ permits=n=10_000](./histograms/max_semaphore.gif) |