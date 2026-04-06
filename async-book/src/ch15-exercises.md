## Exercises

### Exercise 1: Async Echo Server

Build a TCP echo server that handles multiple clients concurrently.

**Requirements**:
- Listen on `127.0.0.1:8080`
- Accept connections and echo back each line
- Handle client disconnections gracefully
- Print a log when clients connect/disconnect

<details>
<summary>🔑 Solution</summary>

```rust
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Echo server listening on :8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("[{addr}] Connected");

        tokio::spawn(async move {
            let (reader, mut writer) = socket.into_split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(0) => {
                        println!("[{addr}] Disconnected");
                        break;
                    }
                    Ok(_) => {
                        print!("[{addr}] Echo: {line}");
                        if writer.write_all(line.as_bytes()).await.is_err() {
                            println!("[{addr}] Write error, disconnecting");
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("[{addr}] Read error: {e}");
                        break;
                    }
                }
            }
        });
    }
}
```

</details>

---

### Exercise 2: Concurrent URL Fetcher with Rate Limiting

Fetch a list of URLs concurrently, with at most 5 concurrent requests.

<details>
<summary>🔑 Solution</summary>

```rust
use futures::stream::{self, StreamExt};
use tokio::time::{sleep, Duration};

async fn fetch_urls(urls: Vec<String>) -> Vec<Result<String, String>> {
    // buffer_unordered(5) ensures at most 5 futures are polled
    // concurrently — no separate Semaphore needed here.
    let results: Vec<_> = stream::iter(urls)
        .map(|url| {
            async move {
                println!("Fetching: {url}");

                match reqwest::get(&url).await {
                    Ok(resp) => match resp.text().await {
                        Ok(body) => Ok(body),
                        Err(e) => Err(format!("{url}: {e}")),
                    },
                    Err(e) => Err(format!("{url}: {e}")),
                }
            }
        })
        .buffer_unordered(5) // ← This alone limits concurrency to 5
        .collect()
        .await;

    results
}

// NOTE: Use Semaphore when you need to limit concurrency across
// independently spawned tasks (tokio::spawn). Use buffer_unordered
// when processing a stream. Don't combine both for the same limit.
```

</details>

---

### Exercise 3: Graceful Shutdown with Worker Pool

Build a task processor with:
- A channel-based work queue
- N worker tasks consuming from the queue
- Graceful shutdown on Ctrl+C: stop accepting, finish in-flight work

<details>
<summary>🔑 Solution</summary>

```rust
use tokio::sync::{mpsc, watch};
use tokio::time::{sleep, Duration};

struct WorkItem {
    id: u64,
    payload: String,
}

#[tokio::main]
async fn main() {
    let (work_tx, work_rx) = mpsc::channel::<WorkItem>(100);
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    // Spawn 4 workers
    let mut worker_handles = Vec::new();
    let work_rx = std::sync::Arc::new(tokio::sync::Mutex::new(work_rx));

    for id in 0..4 {
        let rx = work_rx.clone();
        let mut shutdown = shutdown_rx.clone();
        let handle = tokio::spawn(async move {
            loop {
                let item = {
                    let mut rx = rx.lock().await;
                    tokio::select! {
                        item = rx.recv() => item,
                        _ = shutdown.changed() => {
                            if *shutdown.borrow() { None } else { continue }
                        }
                    }
                };

                match item {
                    Some(work) => {
                        println!("Worker {id}: processing item {}", work.id);
                        sleep(Duration::from_millis(200)).await; // Simulate work
                        println!("Worker {id}: done with item {}", work.id);
                    }
                    None => {
                        println!("Worker {id}: channel closed, exiting");
                        break;
                    }
                }
            }
        });
        worker_handles.push(handle);
    }

    // Producer: submit some work
    let producer = tokio::spawn(async move {
        for i in 0..20 {
            let _ = work_tx.send(WorkItem {
                id: i,
                payload: format!("task-{i}"),
            }).await;
            sleep(Duration::from_millis(50)).await;
        }
    });

    // Wait for Ctrl+C
    tokio::signal::ctrl_c().await.unwrap();
    println!("\nShutdown signal received!");
    shutdown_tx.send(true).unwrap();
    producer.abort(); // Cancel the producer task

    // Wait for workers to finish
    for handle in worker_handles {
        let _ = handle.await;
    }
    println!("All workers shut down. Goodbye!");
}
```

</details>

---

### Exercise 4: Build a Simple Async Mutex from Scratch

Implement an async-aware mutex using channels (without using `tokio::sync::Mutex`).

*Hint*: Use a `tokio::sync::mpsc` channel with capacity 1 as a semaphore.

<details>
<summary>🔑 Solution</summary>

```rust
use std::cell::UnsafeCell;
use std::sync::Arc;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

pub struct SimpleAsyncMutex<T> {
    data: Arc<UnsafeCell<T>>,
    semaphore: Arc<Semaphore>,
}

// SAFETY: Access to T is serialized by the semaphore (max 1 permit).
unsafe impl<T: Send> Send for SimpleAsyncMutex<T> {}
unsafe impl<T: Send> Sync for SimpleAsyncMutex<T> {}

pub struct SimpleGuard<T> {
    data: Arc<UnsafeCell<T>>,
    _permit: OwnedSemaphorePermit, // Dropped on guard drop → releases lock
}

impl<T> SimpleAsyncMutex<T> {
    pub fn new(value: T) -> Self {
        SimpleAsyncMutex {
            data: Arc::new(UnsafeCell::new(value)),
            semaphore: Arc::new(Semaphore::new(1)),
        }
    }

    pub async fn lock(&self) -> SimpleGuard<T> {
        let permit = self.semaphore.clone().acquire_owned().await.unwrap();
        SimpleGuard {
            data: self.data.clone(),
            _permit: permit,
        }
    }
}

impl<T> std::ops::Deref for SimpleGuard<T> {
    type Target = T;
    fn deref(&self) -> &T {
        // SAFETY: We hold the only semaphore permit, so no other
        // SimpleGuard exists → exclusive access is guaranteed.
        unsafe { &*self.data.get() }
    }
}

impl<T> std::ops::DerefMut for SimpleGuard<T> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: Same reasoning — single permit guarantees exclusivity.
        unsafe { &mut *self.data.get() }
    }
}

// When SimpleGuard is dropped, _permit is dropped,
// which releases the semaphore permit — another lock() can proceed.

// Usage:
// let mutex = SimpleAsyncMutex::new(vec![1, 2, 3]);
// {
//     let mut guard = mutex.lock().await;
//     guard.push(4);
// } // permit released here
```

**Key takeaway**: Async mutexes are typically built on top of semaphores. The semaphore provides the async wait mechanism — when locked, `acquire()` suspends the task until the permit is released. This is exactly how `tokio::sync::Mutex` works internally.

> **Why `UnsafeCell` and not `std::sync::Mutex`?** A previous version of this
> exercise used `Arc<Mutex<T>>` with `Deref`/`DerefMut` calling `.lock().unwrap()`.
> That doesn't compile — the returned `&T` borrows from a temporary `MutexGuard`
> that's dropped immediately. `UnsafeCell` avoids the intermediate guard, and the
> semaphore-based serialization makes the `unsafe` sound.

</details>

---

### Exercise 5: Stream Pipeline

Build a data processing pipeline using streams:
1. Generate numbers 1..=100
2. Filter to even numbers
3. Map each to its square
4. Process 10 at a time concurrently (simulate with sleep)
5. Collect results

<details>
<summary>🔑 Solution</summary>

```rust
use futures::stream::{self, StreamExt};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let results: Vec<u64> = stream::iter(1u64..=100)
        // Step 2: Filter evens
        .filter(|x| futures::future::ready(x % 2 == 0))
        // Step 3: Square each
        .map(|x| x * x)
        // Step 4: Process concurrently (simulate async work)
        .map(|x| async move {
            sleep(Duration::from_millis(50)).await;
            println!("Processed: {x}");
            x
        })
        .buffer_unordered(10) // 10 concurrent
        // Step 5: Collect
        .collect()
        .await;

    println!("Got {} results", results.len());
    println!("Sum: {}", results.iter().sum::<u64>());
}
```

</details>

---

### Exercise 6: Implement Select with Timeout

Without using `tokio::select!` or `tokio::time::timeout`, implement a function that races a future against a deadline and returns `Either::Left(result)` or `Either::Right(())` on timeout.

*Hint*: Build on the `Select` combinator from Chapter 6 and the `TimerFuture` from the same chapter.

<details>
<summary>🔑 Solution</summary>

```rust,ignore
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

pub enum Either<A, B> {
    Left(A),
    Right(B),
}

pub struct Timeout<F> {
    future: F,
    timer: TimerFuture, // From Chapter 6
}

impl<F: Future + Unpin> Timeout<F> {
    pub fn new(future: F, duration: Duration) -> Self {
        Timeout {
            future,
            timer: TimerFuture::new(duration),
        }
    }
}

impl<F: Future + Unpin> Future for Timeout<F> {
    type Output = Either<F::Output, ()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Check if the main future is done
        if let Poll::Ready(val) = Pin::new(&mut self.future).poll(cx) {
            return Poll::Ready(Either::Left(val));
        }

        // Check if the timer expired
        if let Poll::Ready(()) = Pin::new(&mut self.timer).poll(cx) {
            return Poll::Ready(Either::Right(()));
        }

        Poll::Pending
    }
}

// Usage:
// match Timeout::new(fetch_data(), Duration::from_secs(5)).await {
//     Either::Left(data) => println!("Got data: {data}"),
//     Either::Right(()) => println!("Timed out!"),
// }
```

**Key takeaway**: `select`/`timeout` is just polling two futures and seeing which completes first. The entire async ecosystem is built from this simple primitive: poll, Pending/Ready, Waker.

</details>

***

