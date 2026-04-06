# 5. Channels and Message Passing 🟢

> **What you'll learn:**
> - `std::sync::mpsc` basics and when to upgrade to crossbeam-channel
> - Channel selection with `select!` for multi-source message handling
> - Bounded vs unbounded channels and backpressure strategies
> - The actor pattern for encapsulating concurrent state

## std::sync::mpsc — The Standard Channel

Rust's standard library provides a multi-producer, single-consumer channel:

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a channel: tx (transmitter) and rx (receiver)
    let (tx, rx) = mpsc::channel();

    // Spawn a producer thread
    let tx1 = tx.clone(); // Clone for multiple producers
    thread::spawn(move || {
        for i in 0..5 {
            tx1.send(format!("producer-1: msg {i}")).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Second producer
    thread::spawn(move || {
        for i in 0..5 {
            tx.send(format!("producer-2: msg {i}")).unwrap();
            thread::sleep(Duration::from_millis(150));
        }
    });

    // Consumer: receive all messages
    for msg in rx {
        // rx iterator ends when ALL senders are dropped
        println!("Received: {msg}");
    }
    println!("All producers done.");
}
```

> **Note:** `.unwrap()` on `.send()` is used for brevity. It panics if the receiver has been dropped. Production code should handle `SendError` gracefully.

**Key properties**:
- **Unbounded** by default (can fill memory if consumer is slow)
- `mpsc::sync_channel(N)` creates a **bounded** channel with backpressure
- `rx.recv()` blocks the current thread until a message arrives
- `rx.try_recv()` returns immediately with `Err(TryRecvError::Empty)` if nothing is ready
- The channel closes when all `Sender`s are dropped

```rust
// Bounded channel with backpressure:
let (tx, rx) = mpsc::sync_channel(10); // Buffer of 10 messages

thread::spawn(move || {
    for i in 0..1000 {
        tx.send(i).unwrap(); // BLOCKS if buffer is full — natural backpressure
    }
});
```

> **Note:** `.unwrap()` is used for brevity. In production, handle `SendError` (receiver dropped) instead of panicking.

### crossbeam-channel — The Production Workhorse

`crossbeam-channel` is the de facto standard for production channel usage. It's faster than `std::sync::mpsc` and supports multi-consumer (`mpmc`):

```rust,ignore
// Cargo.toml:
//   [dependencies]
//   crossbeam-channel = "0.5"
use crossbeam_channel::{bounded, unbounded, select, Sender, Receiver};
use std::thread;
use std::time::Duration;

fn main() {
    // Bounded MPMC channel
    let (tx, rx) = bounded::<String>(100);

    // Multiple producers
    for id in 0..4 {
        let tx = tx.clone();
        thread::spawn(move || {
            for i in 0..10 {
                tx.send(format!("worker-{id}: item-{i}")).unwrap();
            }
        });
    }
    drop(tx); // Drop the original sender so the channel can close

    // Multiple consumers (not possible with std::sync::mpsc!)
    let rx2 = rx.clone();
    let consumer1 = thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            println!("[consumer-1] {msg}");
        }
    });
    let consumer2 = thread::spawn(move || {
        while let Ok(msg) = rx2.recv() {
            println!("[consumer-2] {msg}");
        }
    });

    consumer1.join().unwrap();
    consumer2.join().unwrap();
}
```

### Channel Selection (select!)

Listen on multiple channels simultaneously — like `select` in Go:

```rust,ignore
use crossbeam_channel::{bounded, tick, after, select};
use std::time::Duration;

fn main() {
    let (work_tx, work_rx) = bounded::<String>(10);
    let ticker = tick(Duration::from_secs(1));        // Periodic tick
    let deadline = after(Duration::from_secs(10));     // One-shot timeout

    // Producer
    let tx = work_tx.clone();
    std::thread::spawn(move || {
        for i in 0..100 {
            tx.send(format!("job-{i}")).unwrap();
            std::thread::sleep(Duration::from_millis(500));
        }
    });
    drop(work_tx);

    loop {
        select! {
            recv(work_rx) -> msg => {
                match msg {
                    Ok(job) => println!("Processing: {job}"),
                    Err(_) => {
                        println!("Work channel closed");
                        break;
                    }
                }
            },
            recv(ticker) -> _ => {
                println!("Tick — heartbeat");
            },
            recv(deadline) -> _ => {
                println!("Deadline reached — shutting down");
                break;
            },
        }
    }
}
```

> **Go comparison**: This is exactly like Go's `select` statement over channels.
> crossbeam's `select!` macro randomizes order to prevent starvation, just like Go.

### Bounded vs Unbounded and Backpressure

| Type | Behavior When Full | Memory | Use Case |
|------|-------------------|--------|----------|
| **Unbounded** | Never blocks (grows heap) | Unbounded ⚠️ | Rare — only when producer is slower than consumer |
| **Bounded** | `send()` blocks until space | Fixed | Production default — prevents OOM |
| **Rendezvous** (bounded(0)) | `send()` blocks until receiver is ready | None | Synchronization / handoff |

```rust
// Rendezvous channel — zero capacity, direct handoff
let (tx, rx) = crossbeam_channel::bounded(0);
// tx.send(x) blocks until rx.recv() is called, and vice versa.
// This synchronizes the two threads precisely.
```

**Rule**: Always use bounded channels in production unless you can prove the
producer will never outpace the consumer.

### Actor Pattern with Channels

The actor pattern uses channels to serialize access to mutable state — no mutexes needed:

```rust
use std::sync::mpsc;
use std::thread;

// Messages the actor can receive
enum CounterMsg {
    Increment,
    Decrement,
    Get(mpsc::Sender<i64>), // Reply channel
}

struct CounterActor {
    count: i64,
    rx: mpsc::Receiver<CounterMsg>,
}

impl CounterActor {
    fn new(rx: mpsc::Receiver<CounterMsg>) -> Self {
        CounterActor { count: 0, rx }
    }

    fn run(mut self) {
        while let Ok(msg) = self.rx.recv() {
            match msg {
                CounterMsg::Increment => self.count += 1,
                CounterMsg::Decrement => self.count -= 1,
                CounterMsg::Get(reply) => {
                    let _ = reply.send(self.count);
                }
            }
        }
    }
}

// Actor handle — cheap to clone, Send + Sync
#[derive(Clone)]
struct Counter {
    tx: mpsc::Sender<CounterMsg>,
}

impl Counter {
    fn spawn() -> Self {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || CounterActor::new(rx).run());
        Counter { tx }
    }

    fn increment(&self) { let _ = self.tx.send(CounterMsg::Increment); }
    fn decrement(&self) { let _ = self.tx.send(CounterMsg::Decrement); }

    fn get(&self) -> i64 {
        let (reply_tx, reply_rx) = mpsc::channel();
        self.tx.send(CounterMsg::Get(reply_tx)).unwrap();
        reply_rx.recv().unwrap()
    }
}

fn main() {
    let counter = Counter::spawn();

    // Multiple threads can safely use the counter — no mutex!
    let handles: Vec<_> = (0..10).map(|_| {
        let counter = counter.clone();
        thread::spawn(move || {
            for _ in 0..1000 {
                counter.increment();
            }
        })
    }).collect();

    for h in handles { h.join().unwrap(); }
    println!("Final count: {}", counter.get()); // 10000
}
```

> **When to use actors vs mutexes**: Actors are great when the state has complex
> invariants, operations take a long time, or you want to serialize access
> without thinking about lock ordering. Mutexes are simpler for short critical sections.

> **Key Takeaways — Channels**
> - `crossbeam-channel` is the production workhorse — faster and more feature-rich than `std::sync::mpsc`
> - `select!` replaces complex multi-source polling with declarative channel selection
> - Bounded channels provide natural backpressure; unbounded channels risk OOM

> **See also:** [Ch 6 — Concurrency](ch06-concurrency-vs-parallelism-vs-threads.md) for threads, Mutex, and shared state. [Ch 15 — Async](ch16-asyncawait-essentials.md) for async channels (`tokio::sync::mpsc`).

---

### Exercise: Channel-Based Worker Pool ★★★ (~45 min)

Build a worker pool using channels where:
- A dispatcher sends `Job` structs through a channel
- N workers consume jobs and send results back
- Use `std::sync::mpsc` with `Arc<Mutex<Receiver>>` for a shared work queue

<details>
<summary>🔑 Solution</summary>

```rust
use std::sync::mpsc;
use std::thread;

struct Job {
    id: u64,
    data: String,
}

struct JobResult {
    job_id: u64,
    output: String,
    worker_id: usize,
}

fn worker_pool(jobs: Vec<Job>, num_workers: usize) -> Vec<JobResult> {
    let (job_tx, job_rx) = mpsc::channel::<Job>();
    let (result_tx, result_rx) = mpsc::channel::<JobResult>();

    let job_rx = std::sync::Arc::new(std::sync::Mutex::new(job_rx));

    let mut handles = Vec::new();
    for worker_id in 0..num_workers {
        let job_rx = job_rx.clone();
        let result_tx = result_tx.clone();
        handles.push(thread::spawn(move || {
            loop {
                let job = {
                    let rx = job_rx.lock().unwrap();
                    rx.recv()
                };
                match job {
                    Ok(job) => {
                        let output = format!("processed '{}' by worker {worker_id}", job.data);
                        result_tx.send(JobResult {
                            job_id: job.id, output, worker_id,
                        }).unwrap();
                    }
                    Err(_) => break,
                }
            }
        }));
    }
    drop(result_tx);

    let num_jobs = jobs.len();
    for job in jobs {
        job_tx.send(job).unwrap();
    }
    drop(job_tx);

    let results: Vec<_> = result_rx.into_iter().collect();
    assert_eq!(results.len(), num_jobs);

    for h in handles { h.join().unwrap(); }
    results
}

fn main() {
    let jobs: Vec<Job> = (0..20).map(|i| Job {
        id: i, data: format!("task-{i}"),
    }).collect();

    let results = worker_pool(jobs, 4);
    for r in &results {
        println!("[worker {}] job {}: {}", r.worker_id, r.job_id, r.output);
    }
}
```

</details>

***

