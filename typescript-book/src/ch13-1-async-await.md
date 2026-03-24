# Async/Await — From Promises to Futures

## The Core Difference

TypeScript's `async`/`await` and Rust's `async`/`await` look almost identical but work
fundamentally differently.

| | TypeScript | Rust |
|--|-----------|------|
| Async primitive | `Promise<T>` | `Future<Output = T>` |
| Runtime | Built into V8/Deno/Bun | You choose: `tokio`, `async-std`, `smol` |
| Execution | Eager — calling an `async` function starts it | Lazy — calling an `async` function returns an inert Future |
| Concurrency | Single-threaded event loop | Multi-threaded by default (tokio) |
| Cancellation | Not built in (AbortController is opt-in) | Dropping a Future cancels it |

## Side-by-Side

🟦 **TypeScript**
```typescript
async function fetchUser(id: number): Promise<User> {
    const response = await fetch(`/api/users/${id}`);
    const user = await response.json();
    return user;
}
```

🦀 **Rust** (with `tokio` + `reqwest`)
```rust
async fn fetch_user(id: u64) -> Result<User, reqwest::Error> {
    let user: User = reqwest::get(format!("/api/users/{id}"))
        .await?
        .json()
        .await?;
    Ok(user)
}
```

## Setting Up an Async Runtime

Unlike TypeScript where the runtime is always there, Rust needs you to start one:

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust
#[tokio::main]
async fn main() {
    let result = fetch_user(42).await;
    println!("{result:?}");
}
```

`#[tokio::main]` is a macro that wraps your `main` in a tokio runtime. It expands roughly to:

```rust
fn main() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let result = fetch_user(42).await;
        println!("{result:?}");
    });
}
```

## Futures Are Lazy

💡 **This is the most important difference.** In TypeScript, calling an `async` function
immediately starts executing it. In Rust, it only returns a `Future` — nothing runs until
you `.await` or spawn it.

```rust
let future = fetch_user(42);  // nothing happens yet!
let user = future.await;       // NOW it runs
```

## Concurrent Execution

🟦 **TypeScript** — `Promise.all`:
```typescript
const [user, posts] = await Promise.all([
    fetchUser(42),
    fetchPosts(42),
]);
```

🦀 **Rust** — `tokio::join!`:
```rust
let (user, posts) = tokio::join!(
    fetch_user(42),
    fetch_posts(42),
);
```

## Spawning Tasks

🟦 **TypeScript** — fire and forget:
```typescript
// In TypeScript, just calling an async function without await starts it
fetchUser(42); // runs in the background (but you lose error handling)
```

🦀 **Rust** — explicit spawn:
```rust
let handle = tokio::spawn(async {
    fetch_user(42).await
});

// Later:
let user = handle.await.unwrap();
```

## `select!` — Racing Futures

Like `Promise.race` but more powerful:

```rust
use tokio::select;

select! {
    user = fetch_user(42) => println!("got user: {user:?}"),
    _ = tokio::time::sleep(Duration::from_secs(5)) => println!("timeout!"),
}
```

## Streams — Async Iterators

TypeScript has `AsyncIterable`. Rust's equivalent is `Stream` (from `tokio-stream` or
`futures` crate):

🟦 **TypeScript**
```typescript
for await (const chunk of response.body) {
    process(chunk);
}
```

🦀 **Rust**
```rust
use tokio_stream::StreamExt;

let mut stream = tokio_stream::iter(vec![1, 2, 3]);
while let Some(value) = stream.next().await {
    println!("{value}");
}
```

## Common Async Patterns

### Timeout

```rust
use tokio::time::{timeout, Duration};

match timeout(Duration::from_secs(5), fetch_user(42)).await {
    Ok(Ok(user)) => println!("got user: {user:?}"),
    Ok(Err(e)) => println!("request failed: {e}"),
    Err(_) => println!("timed out"),
}
```

### Retry with Backoff

```rust
let mut delay = Duration::from_millis(100);
for attempt in 1..=3 {
    match fetch_user(42).await {
        Ok(user) => return Ok(user),
        Err(e) if attempt < 3 => {
            eprintln!("attempt {attempt} failed: {e}, retrying...");
            tokio::time::sleep(delay).await;
            delay *= 2;
        }
        Err(e) => return Err(e),
    }
}
```

🏋️ **Exercise**: Write an async function that fetches 3 URLs concurrently with `tokio::join!`
and returns the first successful result, or all errors if all fail.
