# Rust Option and Result key takeaways

> **What you'll learn:** Idiomatic error handling patterns — safe alternatives to `unwrap()`, the `?` operator for propagation, custom error types, and when to use `anyhow` vs `thiserror` in production code.

- ```Option``` and ```Result``` are an integral part of idiomatic Rust
- **Safe alternatives to `unwrap()`**:
```rust
// Option<T> safe alternatives
let value = opt.unwrap_or(default);              // Provide fallback value
let value = opt.unwrap_or_else(|| compute());    // Lazy computation for fallback
let value = opt.unwrap_or_default();             // Use Default trait implementation
let value = opt.expect("descriptive message");   // Only when panic is acceptable

// Result<T, E> safe alternatives  
let value = result.unwrap_or(fallback);          // Ignore error, use fallback
let value = result.unwrap_or_else(|e| handle(e)); // Handle error, return fallback
let value = result.unwrap_or_default();          // Use Default trait
```
- **Pattern matching for explicit control**:
```rust
match some_option {
    Some(value) => println!("Got: {}", value),
    None => println!("No value found"),
}

match some_result {
    Ok(value) => process(value),
    Err(error) => log_error(error),
}
```
- **Use `?` operator for error propagation**: Short-circuit and bubble up errors
```rust
fn process_file(path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(path)?; // Automatically returns error
    Ok(content.to_uppercase())
}
```
- **Transformation methods**:
    - `map()`: Transform the success value `Ok(T)` -> `Ok(U)` or `Some(T)` -> `Some(U)`
    - `map_err()`: Transform the error type `Err(E)` -> `Err(F)`
    - `and_then()`: Chain operations that can fail
- **Use in your own APIs**: Prefer `Result<T, E>` over exceptions or error codes
- **References**: [Option docs](https://doc.rust-lang.org/std/option/enum.Option.html) | [Result docs](https://doc.rust-lang.org/std/result/enum.Result.html)

# Rust Common Pitfalls and Debugging Tips
- **Borrowing issues**: Most common beginner mistake
    - "cannot borrow as mutable" -> Only one mutable reference allowed at a time
    - "borrowed value does not live long enough" -> Reference outlives the data it points to
    - **Fix**: Use scopes `{}` to limit reference lifetimes, or clone data when needed
- **Missing trait implementations**: "method not found" errors
    - **Fix**: Add `#[derive(Debug, Clone, PartialEq)]` for common traits
    - Use `cargo check` to get better error messages than `cargo run`
- **Integer overflow in debug mode**: Rust panics on overflow
    - **Fix**: Use `wrapping_add()`, `saturating_add()`, or `checked_add()` for explicit behavior
- **String vs &str confusion**: Different types for different use cases
    - Use `&str` for string slices (borrowed), `String` for owned strings
    - **Fix**: Use `.to_string()` or `String::from()` to convert `&str` to `String`
- **Fighting the borrow checker**: Don't try to outsmart it
    - **Fix**: Restructure code to work with ownership rules rather than against them
    - Consider using `Rc<RefCell<T>>` for complex sharing scenarios (sparingly)

## Error Handling Examples: Good vs Bad
```rust
// [ERROR] BAD: Can panic unexpectedly
fn bad_config_reader() -> String {
    let config = std::env::var("CONFIG_FILE").unwrap(); // Panic if not set!
    std::fs::read_to_string(config).unwrap()           // Panic if file missing!
}

// [OK] GOOD: Handles errors gracefully
fn good_config_reader() -> Result<String, ConfigError> {
    let config_path = std::env::var("CONFIG_FILE")
        .unwrap_or_else(|_| "default.conf".to_string()); // Fallback to default
    
    let content = std::fs::read_to_string(config_path)
        .map_err(ConfigError::FileRead)?;                // Convert and propagate error
    
    Ok(content)
}

// [OK] EVEN BETTER: With proper error types
use thiserror::Error;

#[derive(Error, Debug)]
enum ConfigError {
    #[error("Failed to read config file: {0}")]
    FileRead(#[from] std::io::Error),
    
    #[error("Invalid configuration: {message}")]
    Invalid { message: String },
}
```

Let's break down what's happening here. `ConfigError` has just **two variants** — one for I/O errors and one for validation errors. This is the right starting point for most modules:

| `ConfigError` variant | Holds | Created by |
|----------------------|-------|-----------|
| `FileRead(io::Error)` | The original I/O error | `#[from]` auto-converts via `?` |
| `Invalid { message }` | A human-readable explanation | Your validation code |

Now you can write functions that return `Result<T, ConfigError>`:

```rust
fn read_config(path: &str) -> Result<String, ConfigError> {
    let content = std::fs::read_to_string(path)?;  // io::Error → ConfigError::FileRead
    if content.is_empty() {
        return Err(ConfigError::Invalid {
            message: "config file is empty".to_string(),
        });
    }
    Ok(content)
}
```

> **🟢 Self-study checkpoint:** Before continuing, make sure you can answer:
> 1. Why does `?` on the `read_to_string` call work? (Because `#[from]` generates `impl From<io::Error> for ConfigError`)
> 2. What happens if you add a third variant `MissingKey(String)` — what code changes? (Just add the variant; existing code still compiles)

## Crate-Level Error Types and Result Aliases

As your project grows beyond a single file, you'll combine multiple module-level errors into a **crate-level error type**. This is the standard pattern in production Rust. Let's build up from the `ConfigError` above.

In real-world Rust projects, every crate (or significant module) defines its own `Error`
enum and a `Result` type alias.  This is the idiomatic pattern — analogous to how in C++
you'd define a per-library exception hierarchy and `using Result = std::expected<T, Error>`.

### The pattern

```rust
// src/error.rs  (or at the top of lib.rs)
use thiserror::Error;

/// Every error this crate can produce.
#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),          // auto-converts via From

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),     // auto-converts via From

    #[error("Invalid sensor id: {0}")]
    InvalidSensor(u32),                  // domain-specific variant

    #[error("Timeout after {ms} ms")]
    Timeout { ms: u64 },
}

/// Crate-wide Result alias — saves typing throughout the crate.
pub type Result<T> = core::result::Result<T, Error>;
```

### How it simplifies every function

Without the alias you'd write:

```rust
// Verbose — error type repeated everywhere
fn read_sensor(id: u32) -> Result<f64, crate::Error> { ... }
fn parse_config(path: &str) -> Result<Config, crate::Error> { ... }
```

With the alias:

```rust
// Clean — just `Result<T>`
use crate::{Error, Result};

fn read_sensor(id: u32) -> Result<f64> {
    if id > 128 {
        return Err(Error::InvalidSensor(id));
    }
    let raw = std::fs::read_to_string(format!("/dev/sensor/{id}"))?; // io::Error → Error::Io
    let value: f64 = raw.trim().parse()
        .map_err(|_| Error::InvalidSensor(id))?;
    Ok(value)
}
```

The `#[from]` attribute on `Io` generates this `impl` for free:

```rust
// Auto-generated by thiserror's #[from]
impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        Error::Io(source)
    }
}
```

That's what makes `?` work: when a function returns `std::io::Error` and your function
returns `Result<T>` (your alias), the compiler calls `From::from()` to convert it
automatically.

### Composing module-level errors

Larger crates split errors by module, then compose them at the crate root:

```rust
// src/config/error.rs
#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("Missing key: {0}")]
    MissingKey(String),
    #[error("Invalid value for '{key}': {reason}")]
    InvalidValue { key: String, reason: String },
}

// src/error.rs  (crate-level)
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]               // delegates Display to inner error
    Config(#[from] crate::config::ConfigError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
pub type Result<T> = core::result::Result<T, Error>;
```

Callers can still match on specific config errors:

```rust
match result {
    Err(Error::Config(ConfigError::MissingKey(k))) => eprintln!("Add '{k}' to config"),
    Err(e) => eprintln!("Other error: {e}"),
    Ok(v) => use_value(v),
}
```

### C++ comparison

| Concept | C++ | Rust |
|---------|-----|------|
| Error hierarchy | `class AppError : public std::runtime_error` | `#[derive(thiserror::Error)] enum Error { ... }` |
| Return error | `std::expected<T, Error>` or `throw` | `fn foo() -> Result<T>` |
| Convert error | Manual `try/catch` + rethrow | `#[from]` + `?` — zero boilerplate |
| Result alias | `template<class T> using Result = std::expected<T, Error>;` | `pub type Result<T> = core::result::Result<T, Error>;` |
| Error message | Override `what()` | `#[error("...")]` — compiled into `Display` impl |


