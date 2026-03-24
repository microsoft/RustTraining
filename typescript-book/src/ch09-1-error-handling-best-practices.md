# Error Handling Best Practices

## Library vs Application Errors

### Libraries: Use typed errors with `thiserror`

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("connection failed: {0}")]
    Connection(String),
    #[error("query failed: {0}")]
    Query(String),
    #[error("record not found: {table}/{id}")]
    NotFound { table: String, id: String },
}
```

### Applications: Use `anyhow` for convenience

```rust
use anyhow::{bail, ensure, Context, Result};

fn run() -> Result<()> {
    let port: u16 = std::env::var("PORT")
        .context("PORT env var not set")?
        .parse()
        .context("PORT must be a valid number")?;

    ensure!(port > 0, "PORT must be positive, got {port}");

    if port == 80 {
        bail!("port 80 requires root privileges");
    }

    Ok(())
}
```

## Error Context Chains

`anyhow` provides `.context()` which wraps errors with additional information, creating a
chain similar to JavaScript's `Error.cause`:

```rust
std::fs::read_to_string("config.toml")
    .context("failed to read config file")
    .context("during application startup")?;

// Error output:
// during application startup
// Caused by:
//   0: failed to read config file
//   1: No such file or directory (os error 2)
```

## The `?` in `main()`

```rust
fn main() -> Result<()> {
    let config = load_config("app.toml")?;
    start_server(config)?;
    Ok(())
}
```

When `main()` returns `Result`, Rust prints the error and exits with code 1 on `Err`.

## Mapping Errors

```rust
// Map error type
let count: i32 = input.parse()
    .map_err(|_| AppError::InvalidInput("expected a number".into()))?;

// Map to Option
let maybe: Option<i32> = input.parse().ok();

// Map from Option to Result
let value = maybe_value.ok_or(AppError::MissingField("name"))?;
let value = maybe_value.ok_or_else(|| expensive_error())?;
```

🏋️ **Exercise**: Define a custom error enum for a URL shortener service with variants for
invalid URLs, duplicate slugs, and database errors. Use `thiserror` for the library and
`anyhow` in the application layer.
