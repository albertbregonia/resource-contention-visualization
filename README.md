# Concurrency Benchmark Visualization
To better understand `tokio-rs`'s scheduler along with other async constructs and concurrency primitives, this project serves to be a visualization of end-to-end latency when using different methods of accessing shared data (under various conditions) such as: 
 - `mpsc::channel` with a `oneshot::channel` following the actor model (message passing)
 - `Mutex`
 - `RWLock` (read-optimized mutex)
 - `Sempahore` (maintains n permits to some associated data)


End-to-end latency being defined by: 

$$T_{\text{response received}} - T_{\text{request sent}}$$ 

which implictly includes tokio's scheduler waking and scheduling tasks, etc.&mdash; which is fine, as all users of the system would feel this latency when characterizing system responsiveness.

*Disclaimer: By no means is this an academic report of any kind. This is simply a project done to understand tokio-rs in more technical depth and what goes on under the hood when choosing a concurrency primitive.*

# Motivation

# Test Environment

# Findings
