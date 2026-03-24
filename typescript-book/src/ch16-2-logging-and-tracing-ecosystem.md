# Logging and Tracing Ecosystem

## From console.log to Structured Logging

| TypeScript | Rust | Purpose |
|-----------|------|---------|
| `console.log` | `println!` | Quick debugging |
| `winston` / `pino` | `tracing` + `tracing-subscriber` | Structured logging |
| `morgan` (HTTP) | `tower-http::trace` | Request logging |

## The `tracing` Crate

`tracing` is the de facto standard for logging and instrumentation in Rust:

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

```rust
use tracing::{info, warn, error, debug, trace, instrument};

#[instrument]
fn process_order(order_id: u64, customer: &str) {
    info!(order_id, customer, "processing order");

    if order_id == 0 {
        warn!("suspicious order ID");
    }

    debug!(order_id, "order validated");
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("my_app=debug")
        .init();

    process_order(42, "Alice");
}
```

Output:
```
2025-03-24T10:30:00.000Z DEBUG my_app: order validated order_id=42
2025-03-24T10:30:00.000Z  INFO my_app: processing order order_id=42 customer="Alice"
```

## Log Levels

Same concept as in TypeScript logging libraries:

| Level | Use |
|-------|-----|
| `error!` | Something broke |
| `warn!` | Something is wrong but recoverable |
| `info!` | Normal operations |
| `debug!` | Detailed diagnostic information |
| `trace!` | Very verbose, per-iteration data |

## Structured Fields

```rust
info!(
    user_id = 42,
    action = "login",
    ip = "192.168.1.1",
    "user logged in"
);
```

## The `#[instrument]` Attribute

Automatically creates a span with function arguments:

```rust
#[instrument(skip(password))]   // skip sensitive fields
async fn authenticate(username: &str, password: &str) -> Result<Token, AuthError> {
    // All logs inside this function are automatically tagged with username
    info!("authenticating");
    // ...
}
```

🏋️ **Exercise**: Add `tracing` to one of your capstone project's modules. Log at appropriate
levels and use `#[instrument]` on async functions.
