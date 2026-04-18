## Async Programming: C# Task vs Rust Future

> **What you'll learn:** Rust's lazy `Future` vs C#'s eager `Task`, the executor model (tokio),
> cancellation via `Drop` + `select!` vs `CancellationToken`, and real-world patterns for concurrent requests.
>
> **Difficulty:** 🔴 Advanced

C# developers are deeply familiar with `async`/`await`. Rust uses the same keywords but with a fundamentally different execution model.

### The Executor Model

```csharp
// C# — The runtime provides a built-in thread pool and task scheduler
// async/await "just works" out of the box
public async Task<string> FetchDataAsync(string url)
{
    using var client = new HttpClient();
    return await client.GetStringAsync(url);  // Scheduled by .NET thread pool
}
// .NET manages the thread pool, task scheduling, and synchronization context
```

```rust
// Rust — No built-in async runtime. You choose an executor.
// The most popular is tokio.
async fn fetch_data(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

// You MUST have a runtime to execute async code:
#[tokio::main]  // This macro sets up the tokio runtime
async fn main() {
    let data = fetch_data("https://example.com").await.unwrap();
    println!("{}", &data[..100]);
}
```

### Future vs Task

| | C# `Task<T>` | Rust `Future<Output = T>` |
|---|---|---|
| **Execution** | Starts immediately when created | **Lazy** — does nothing until `.await`ed |
| **Runtime** | Built-in (CLR thread pool) | External (tokio, async-std, etc.) |
| **Cancellation** | `CancellationToken` | Drop the `Future` (or `tokio::select!`) |
| **State machine** | Compiler-generated | Compiler-generated |
| **Size** | Heap-allocated | Stack-allocated until boxed |

```rust
// IMPORTANT: Futures are lazy in Rust!
async fn compute() -> i32 { println!("Computing!"); 42 }

let future = compute();  // Nothing printed! Future not polled yet.
let result = future.await; // NOW "Computing!" is printed
```

```csharp
// C# Tasks start immediately!
var task = ComputeAsync();  // "Computing!" printed immediately
var result = await task;    // Just waits for completion
```

### Cancellation: CancellationToken vs Drop / select!

```csharp
// C# — Cooperative cancellation with CancellationToken
public async Task ProcessAsync(CancellationToken ct)
{
    while (!ct.IsCancellationRequested)
    {
        await Task.Delay(1000, ct);  // Throws if cancelled
        DoWork();
    }
}

var cts = new CancellationTokenSource(TimeSpan.FromSeconds(5));
await ProcessAsync(cts.Token);
```

```rust
// Rust — Cancellation by dropping the future, or with tokio::select!
use tokio::time::{sleep, Duration};

async fn process() {
    loop {
        sleep(Duration::from_secs(1)).await;
        do_work();
    }
}

// Timeout pattern with select!
async fn run_with_timeout() {
    tokio::select! {
        _ = process() => { println!("Completed"); }
        _ = sleep(Duration::from_secs(5)) => { println!("Timed out!"); }
    }
    // When select! picks the timeout branch, the process() future is DROPPED
    // —  automatic cleanup, no CancellationToken needed
}
```

### Real-World Pattern: Concurrent Requests with Timeout

```csharp
// C# — Concurrent HTTP requests with timeout
public async Task<string[]> FetchAllAsync(string[] urls, CancellationToken ct)
{
    var tasks = urls.Select(url => httpClient.GetStringAsync(url, ct));
    return await Task.WhenAll(tasks);
}
```

```rust
// Rust — Concurrent requests with tokio::join! or futures::join_all
use futures::future::join_all;

async fn fetch_all(urls: &[&str]) -> Vec<Result<String, reqwest::Error>> {
    let futures = urls.iter().map(|url| reqwest::get(*url));
    let responses = join_all(futures).await;

    let mut results = Vec::new();
    for resp in responses {
        results.push(resp?.text().await);
    }
    results
}

// With timeout:
async fn fetch_all_with_timeout(urls: &[&str]) -> Result<Vec<String>, &'static str> {
    tokio::time::timeout(
        Duration::from_secs(10),
        async {
            let futures: Vec<_> = urls.iter()
                .map(|url| async { reqwest::get(*url).await?.text().await })
                .collect();
            let results = join_all(futures).await;
            results.into_iter().collect::<Result<Vec<_>, _>>()
        }
    )
    .await
    .map_err(|_| "Request timed out")?
    .map_err(|_| "Request failed")
}
```

<details>
<summary><strong>🏋️ Exercise: Async Timeout Pattern</strong> (click to expand)</summary>

**Challenge**: Write an async function that fetches from two URLs concurrently, returns whichever responds first, and cancels the other. (This is `Task.WhenAny` in C#.)

<details>
<summary>🔑 Solution</summary>

```rust
use tokio::time::{sleep, Duration};

// Simulated async fetch
async fn fetch(url: &str, delay_ms: u64) -> String {
    sleep(Duration::from_millis(delay_ms)).await;
    format!("Response from {url}")
}

async fn fetch_first(url1: &str, url2: &str) -> String {
    tokio::select! {
        result = fetch(url1, 200) => {
            println!("URL 1 won");
            result
        }
        result = fetch(url2, 500) => {
            println!("URL 2 won");
            result
        }
    }
    // The losing branch's future is automatically dropped (cancelled)
}

#[tokio::main]
async fn main() {
    let result = fetch_first("https://fast.api", "https://slow.api").await;
    println!("{result}");
}
```

**Key takeaway**: `tokio::select!` is Rust's equivalent of `Task.WhenAny` — it races multiple futures, completes when the first one finishes, and drops (cancels) the rest.

</details>
</details>

### Spawning Independent Tasks with `tokio::spawn`

In C#, `Task.Run` launches work that runs independently of the caller. Rust's equivalent is `tokio::task::spawn`:

```rust
use tokio::task;

async fn background_work() {
    // Runs independently — even if the caller's future is dropped
    let handle = task::spawn(async {
        tokio::time::sleep(Duration::from_secs(2)).await;
        42
    });

    // Do other work while the spawned task runs...
    println!("Doing other work");

    // Await the result when you need it
    let result = handle.await.unwrap(); // 42
}
```

```csharp
// C# equivalent
var task = Task.Run(async () => {
    await Task.Delay(2000);
    return 42;
});
// Do other work...
var result = await task;
```

**Key difference**: A regular `async {}` block is lazy — it does nothing until awaited. `tokio::spawn` launches it on the runtime immediately, like C#'s `Task.Run`.

### Pin: Why Rust Async Has a Concept C# Doesn't

C# developers never encounter `Pin` — the CLR's garbage collector moves objects freely and updates all references automatically. Rust has no GC. When the compiler transforms an `async fn` into a state machine, that struct may contain internal pointers to its own fields. Moving the struct would invalidate those pointers.

`Pin<T>` is a wrapper that says: **"this value will not be moved in memory."**

```rust
// You'll see Pin in these contexts:
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
    //           ^^^^^^^^^^^^^^ pinned — internal references stay valid
}

// Returning a boxed future from a trait:
fn make_future() -> Pin<Box<dyn Future<Output = i32> + Send>> {
    Box::pin(async { 42 })
}
```

**In practice, you almost never write `Pin` yourself.** The `async fn` and `.await` syntax handles it. You'll encounter it only in:
- Compiler error messages (follow the suggestion)
- `tokio::select!` (use the `pin!()` macro)
- Trait methods returning `dyn Future` (use `Box::pin(async { ... })`)

> **Want the deep dive?** The companion [Async Rust Training](../../async-book/src/ch04-pin-and-unpin.md) covers Pin, Unpin, self-referential structs, and structural pinning in full detail.

***


