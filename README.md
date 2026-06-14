# Concurrency Benchmark Visualization
> **Under high contention, how does resource-access latency vary across concurrency primitives?**

*This idea is important, as synchronization overhead can drive latency just as much as resource contention when understanding throughput.*

To better understand `tokio-rs`'s scheduler along with other async constructs and concurrency primitives in Rust, this project serves to be a visualization of end-to-end latency when using different methods of accessing shared data (under various conditions) such as: 
 - `mpsc::channel` with a `oneshot::channel` following the actor model (message passing)
 - `Mutex`
 - `RWLock` (read-optimized mutex)
 - `Sempahore` (maintains n permits to some associated data)

End-to-end latency being defined by: 

$$T_{\text{response received}} - T_{\text{request sent}}$$ 

which implictly includes tokio's scheduler waking and scheduling tasks, etc.&mdash; which is fine, as all users of the system would feel this latency when characterizing system responsiveness.

*Disclaimer: By no means is this an academic report of any kind. This is simply a project done to understand tokio in more technical depth and what goes on under the hood when choosing a concurrency primitive.*

# Usage

All tests are ran in Rust and piped through `stdout` to Python for data analysis and visualization.
```
./benchmark | python stats.py <output filename NOT including extension>
```
TODO: detail env vars

# Test Environment

For all tests, these parameteres are kept constant to ensure results are comparable:
| Parameter | Value |
| - | - |
| Worker Thread Count | 16 |
| Request Count | 10,000 |

# Dependencies

*See [setup](/docs/setup.md) if you already have a valid version of Rust and Python installed.*

Rust environment:
- *rustc 1.91.0 (f8297e351 2025-10-28)
- *cargo 1.91.0 (ea2d97820 2025-10-10)
- tokio 1.52.3 (full features)
- Dependencies listed in the [Cargo.toml](/Cargo.toml) (specifics in the [Cargo.lock](/Cargo.lock))

Python environment:
- *Python 3.12.4
- `matplotlib` 3.10.9 (and nested dependencies in [requirements.txt](/requirements.txt))

\* *later versions may also work however, not verified*

# Findings
See [results.md](/docs/results.md)

*`results.md` includes images; moved to prevent long README*

# Motivation
I was working on a project in Rust and usually, if an implementation feels way too convoluted/complicated&mdash;then it isn't the right way to implement it. Therefore, I found myself looking into the actor model with [actix](https://actix.rs/docs/actix/actor/). No offense to them but, this was also very complicated as types are disjoint for an actor, and there is a lot of boilerplate, blah blah blah&mdash;I wanted to know if writing all this was worth it. 

Essentially my project's old setup could be summarized as "every user fights to update their own state in the collection" (mutex), whereas using the actor model and an `mpsc::channel`, the setup could be summarized as "every user sends in their update and the actor acknowledges it in the collection". I chose actor over mutexes because the use case read more as "users ***update*** state whereas the server ***owns*** the state". Otherwise, with a mutex, it would read as ***everyone*** owns the state and they fight over who gets to update it. 

Furthermore, this easily prevents a soft-lock because under the right conditions, a single user with a copy of `Arc<Mutex<SharedState>>` could keep this memory from being dropped even though the server has dropped the original `Arc<...>`.

# References
Links to docs.rs documentation for the aforementioned tokio constructs:
- [mpsc::channel](https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html)
- [oneshot::channel](https://docs.rs/tokio/latest/tokio/sync/oneshot/index.html)
- [Semaphore](https://docs.rs/tokio/latest/tokio/sync/struct.Semaphore.html)
- [RwLock](https://docs.rs/tokio/latest/tokio/sync/struct.RwLock.html)
- [Mutex](https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html)
- [Barrier](https://docs.rs/tokio/latest/tokio/sync/struct.Barrier.html)
