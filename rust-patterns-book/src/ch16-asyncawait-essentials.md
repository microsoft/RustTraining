# 16. Async/Await Essentials 🔴

> **What you'll learn:**
> - How Rust's `Future` trait differs from Go's goroutines and Python's asyncio
> - Tokio quick-start: spawning tasks, `join!`, and runtime configuration
> - Common async pitfalls and how to fix them
> - When to offload blocking work with `spawn_blocking`

## Futures, Runtimes, and `async fn`

Rust's async model is *fundamentally different* from Go's goroutines or Python's `asyncio`.
Understanding three concepts is enough to get started:

1. **A `Future` is a lazy state machine** — calling `async fn` doesn't execute anything;
   it returns a `Future` that must be polled.
2. **You need a runtime** to poll futures — `tokio`, `async-std`, or `smol`.
   The standard library defines `Future` but provides no runtime.
3. **`async fn` is sugar** — the compiler transforms it into a state machine that
   implements `Future`.

```rust
// A Future is just a trait:
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

// async fn desugars to:
// fn fetch_data(url: &str) -> impl Future<Output = Result<Vec<u8>, Error>>
async fn fetch_data(url: &str) -> Result<Vec<u8>, reqwest::Error> {
    let response = reqwest::get(url).await?;  // .await yields until ready
    let bytes = response.bytes().await?;
    Ok(bytes.to_vec())
}
```

### Tokio Quick Start

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust,ignore
use tokio::time::{sleep, Duration};
use tokio::task;

#[tokio::main]
async fn main() {
    // Spawn concurrent tasks (like lightweight threads):
    let handle_a = task::spawn(async {
        sleep(Duration::from_millis(100)).await;
        "task A done"
    });

    let handle_b = task::spawn(async {
        sleep(Duration::from_millis(50)).await;
        "task B done"
    });

    // .await both — they run concurrently, not sequentially:
    let (a, b) = tokio::join!(handle_a, handle_b);
    println!("{}, {}", a.unwrap(), b.unwrap());
}
```

### Async Common Pitfalls

| Pitfall | Why It Happens | Fix |
|---------|---------------|-----|
| Blocking in async | `std::thread::sleep` or CPU work blocks the executor | Use `tokio::task::spawn_blocking` or `rayon` |
| `Send` bound errors | Future held across `.await` contains `!Send` type (e.g., `Rc`, `MutexGuard`) | Restructure to drop non-Send values before `.await` |
| Future not polled | Calling `async fn` without `.await` or spawning — nothing happens | Always `.await` or `tokio::spawn` the returned future |
| Holding `MutexGuard` across `.await` | `std::sync::MutexGuard` is `!Send`; async tasks may resume on different thread | Use `tokio::sync::Mutex` or drop the guard before `.await` |
| Accidental sequential execution | `let a = foo().await; let b = bar().await;` runs sequentially | Use `tokio::join!` or `tokio::spawn` for concurrency |

```rust
// ❌ Blocking the async executor:
async fn bad() {
    std::thread::sleep(std::time::Duration::from_secs(5)); // Blocks entire thread!
}

// ✅ Offload blocking work:
async fn good() {
    tokio::task::spawn_blocking(|| {
        std::thread::sleep(std::time::Duration::from_secs(5)); // Runs on blocking pool
    }).await.unwrap();
}
```

> **Comprehensive async coverage**: For `Stream`, `select!`, cancellation safety,
> structured concurrency, and `tower` middleware, see our dedicated
> **Async Rust Training** guide. This section covers just enough to read and
> write basic async code.

### Spawning and Structured Concurrency

Tokio's `spawn` creates a new asynchronous task — similar to `thread::spawn` but
much lighter:

```rust,ignore
use tokio::task;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // Spawn three concurrent tasks
    let h1 = task::spawn(async {
        sleep(Duration::from_millis(200)).await;
        "fetched user profile"
    });

    let h2 = task::spawn(async {
        sleep(Duration::from_millis(100)).await;
        "fetched order history"
    });

    let h3 = task::spawn(async {
        sleep(Duration::from_millis(150)).await;
        "fetched recommendations"
    });

    // Wait for all three concurrently (not sequentially!)
    let (r1, r2, r3) = tokio::join!(h1, h2, h3);
    println!("{}", r1.unwrap());
    println!("{}", r2.unwrap());
    println!("{}", r3.unwrap());
}
```

**`join!` vs `try_join!` vs `select!`**:

| Macro | Behavior | Use when |
|-------|----------|----------|
| `join!` | Waits for ALL futures | All tasks must complete |
| `try_join!` | Waits for all, short-circuits on first `Err` | Tasks return `Result` |
| `select!` | Returns when FIRST future completes | Timeouts, cancellation |

```rust,ignore
use tokio::time::{timeout, Duration};

async fn fetch_with_timeout() -> Result<String, Box<dyn std::error::Error>> {
    let result = timeout(Duration::from_secs(5), async {
        // Simulate slow network call
        tokio::time::sleep(Duration::from_millis(100)).await;
        Ok::<_, Box<dyn std::error::Error>>("data".to_string())
    }).await??; // First ? unwraps Elapsed, second ? unwraps inner Result

    Ok(result)
}
```

### `Send` Bounds and Why Futures Must Be `Send`

When you `tokio::spawn` a future, it may resume on a different OS thread.
This means the future must be `Send`. Common pitfalls:

```rust,ignore
use std::rc::Rc;

async fn not_send() {
    let rc = Rc::new(42); // Rc is !Send
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    println!("{}", rc); // rc is held across .await — future is !Send
}

// Fix 1: Drop before .await
async fn fixed_drop() {
    let data = {
        let rc = Rc::new(42);
        *rc // Copy the value out
    }; // rc dropped here
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    println!("{}", data); // Just an i32, which is Send
}

// Fix 2: Use Arc instead of Rc
async fn fixed_arc() {
    let arc = std::sync::Arc::new(42); // Arc is Send
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    println!("{}", arc); // ✅ Future is Send
}
```

> **Comprehensive async coverage**: For `Stream`, `select!`, cancellation safety,
> structured concurrency, and `tower` middleware, see our dedicated
> **Async Rust Training** guide. This section covers just enough to read and
> write basic async code.

> **See also:** [Ch 5 — Channels](ch05-channels-and-message-passing.md) for synchronous channels. [Ch 6 — Concurrency](ch06-concurrency-vs-parallelism-vs-threads.md) for OS threads vs async tasks.

> **Key Takeaways — Async**
> - `async fn` returns a lazy `Future` — nothing runs until you `.await` or spawn it
> - Use `tokio::task::spawn_blocking` for CPU-heavy or blocking work inside async contexts
> - Don't hold `std::sync::MutexGuard` across `.await` — use `tokio::sync::Mutex` instead
> - Futures must be `Send` when spawned — drop `!Send` types before `.await` points

---

### Exercise: Concurrent Fetcher with Timeout ★★ (~25 min)

Write an async function `fetch_all` that spawns three `tokio::spawn` tasks, each
simulating a network call with `tokio::time::sleep`. Join all three with
`tokio::try_join!` wrapped in `tokio::time::timeout(Duration::from_secs(5), ...)`.
Return `Result<Vec<String>, ...>` or an error if any task fails or the deadline
expires.

<details>
<summary>🔑 Solution</summary>

```rust,ignore
use tokio::time::{sleep, timeout, Duration};

async fn fake_fetch(name: &'static str, delay_ms: u64) -> Result<String, String> {
    sleep(Duration::from_millis(delay_ms)).await;
    Ok(format!("{name}: OK"))
}

async fn fetch_all() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let deadline = Duration::from_secs(5);

    let (a, b, c) = timeout(deadline, async {
        let h1 = tokio::spawn(fake_fetch("svc-a", 100));
        let h2 = tokio::spawn(fake_fetch("svc-b", 200));
        let h3 = tokio::spawn(fake_fetch("svc-c", 150));
        tokio::try_join!(h1, h2, h3)
    })
    .await??;

    Ok(vec![a?, b?, c?])
}

#[tokio::main]
async fn main() {
    let results = fetch_all().await.unwrap();
    for r in &results {
        println!("{r}");
    }
}
```

</details>

***

