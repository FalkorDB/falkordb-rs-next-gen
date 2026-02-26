//! Thread pool for parallel query execution.
//!
//! This module provides a global thread pool used to execute queries off
//! the Redis main thread. This prevents long-running queries from blocking
//! Redis command processing.
//!
//! ## Architecture
//!
//! ```text
//! Redis Main Thread                Thread Pool
//!       │                              │
//!   GRAPH.QUERY ───spawn()───→  [Worker 1] → execute query
//!       │                       [Worker 2]
//!   (continues)                 [Worker N]
//!       │                              │
//!   BlockedClient ←────────────── result
//! ```
//!
//! ## Thread Affinity
//!
//! Jobs can optionally specify a worker index for affinity (useful when
//! a query needs to run on the same thread as related work).

use std::thread::{self, JoinHandle};

use crossfire::{Tx, spsc::List};
use once_cell::sync::OnceCell;

/// A closure that can be sent to a worker thread.
type Job = Box<dyn FnOnce() + Send + 'static>;

/// A pool of worker threads for executing jobs.
struct ThreadPool {
    workers: Vec<JoinHandle<()>>,
    sender: Vec<Tx<List<Job>>>,
}

// SAFETY: ThreadPool is stored in a global OnceCell and accessed from any thread.
// The workers Vec<JoinHandle> is only read after initialization (never mutated).
// The sender Vec<Tx> uses crossfire's lock-free SPSC channels which are thread-safe
// for sending from multiple threads (each send targets a specific index).
unsafe impl Sync for ThreadPool {}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let mut workers = Vec::with_capacity(size);
        let mut sender = Vec::with_capacity(size);
        for _ in 0..size {
            let (tx, rx) = crossfire::spsc::unbounded_blocking::<Job>();
            sender.push(tx);
            let worker = thread::spawn(move || {
                while let Ok(job) = rx.recv() {
                    job();
                }
            });
            workers.push(worker);
        }
        Self { workers, sender }
    }

    pub fn spawn<F>(
        &self,
        job: F,
        idx: Option<usize>,
    ) where
        F: FnOnce() + Send + 'static,
    {
        let idx = idx.unwrap_or_else(|| rand::random::<u32>() as usize) % self.workers.len();
        self.sender[idx]
            .send(Box::new(job))
            .expect("thread pool worker channel closed — worker thread terminated unexpectedly");
    }
}

static GLOBAL_THREAD_POOL: OnceCell<ThreadPool> = OnceCell::new();

pub fn spawn<F>(
    job: F,
    idx: Option<usize>,
) where
    F: FnOnce() + Send + 'static,
{
    GLOBAL_THREAD_POOL
        .get_or_init(|| ThreadPool::new(num_cpus::get()))
        .spawn(job, idx);
}
