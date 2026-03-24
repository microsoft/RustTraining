# Error Handling

## Philosophy: Errors Are Values, Not Exceptions

In TypeScript, errors are *thrown* and *caught*. Any function can throw at any time, and there's
no way to know from the signature whether it will. In Rust, errors are *returned* as values.
The function signature tells you exactly what can go wrong.

🟦 **TypeScript** — error path is invisible:
```typescript
function readConfig(path: string): Config {
    const data = fs.readFileSync(path, "utf-8"); // might throw
    return JSON.parse(data);                      // might also throw
}
```

🦀 **Rust** — error path is explicit:
```rust
fn read_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&data)?;
    Ok(config)
}
```

## `Result<T, E>`

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Working with Result

```rust
let result: Result<i32, String> = "42".parse::<i32>()
    .map_err(|e| e.to_string());

// Pattern matching
match result {
    Ok(n) => println!("parsed: {n}"),
    Err(e) => println!("error: {e}"),
}

// Combinator methods (similar to Promise chaining)
let doubled = "42".parse::<i32>()
    .map(|n| n * 2)           // transform Ok value
    .unwrap_or(0);            // provide default on Err
```

## The `?` Operator — Rust's try/catch Replacement

The `?` operator is the most important error-handling tool. It:
1. Unwraps `Ok(value)` and returns the value.
2. On `Err(e)`, converts the error (via `From`) and returns it from the current function.

```rust
fn fetch_user_name(id: u64) -> Result<String, Box<dyn Error>> {
    let response = reqwest::blocking::get(format!("/users/{id}"))?;
    let user: User = response.json()?;
    Ok(user.name)
}
```

This replaces the TypeScript pattern of:
```typescript
try {
    const response = await fetch(`/users/${id}`);
    const user = await response.json();
    return user.name;
} catch (e) {
    throw new Error(`Failed to fetch user: ${e}`);
}
```

## `panic!` — The Unrecoverable Error

`panic!` is for bugs, not expected errors. It's analogous to throwing an uncaught exception
that crashes the process.

```rust
// Use panic for programming errors / invariant violations
fn first_element(v: &[i32]) -> i32 {
    if v.is_empty() {
        panic!("called first_element on an empty slice");
    }
    v[0]
}
```

### When to panic vs return Result

| Situation | Use |
|----------|-----|
| Invalid user input | `Result` |
| File not found | `Result` |
| Network failure | `Result` |
| Index out of bounds (your bug) | `panic!` |
| Invariant violation | `panic!` |
| Prototype / example code | `.unwrap()` (panics on Err) |

## `unwrap()` and `expect()`

```rust
// .unwrap() — panics with a generic message if Err
let port: u16 = env::var("PORT").unwrap().parse().unwrap();

// .expect() — panics with your message if Err
let port: u16 = env::var("PORT")
    .expect("PORT must be set")
    .parse()
    .expect("PORT must be a valid number");
```

💡 **Key insight**: In production code, avoid `.unwrap()`. Use `?` to propagate errors, or
`.expect("reason")` if you're certain the value exists and want a clear message if you're wrong.

## Converting Between Error Types

The `?` operator uses the `From` trait to convert between error types:

```rust
use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Parse(ParseIntError),
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self { AppError::Io(e) }
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self { AppError::Parse(e) }
}

fn load_count(path: &str) -> Result<i32, AppError> {
    let text = std::fs::read_to_string(path)?;  // io::Error → AppError
    let count = text.trim().parse::<i32>()?;     // ParseIntError → AppError
    Ok(count)
}
```

## Crates That Help

- **`anyhow`** — for applications. Provides `anyhow::Result<T>` that wraps any error.
- **`thiserror`** — for libraries. Derives `Error` + `Display` + `From` for custom error enums.

```rust
// With anyhow (application code):
use anyhow::{Context, Result};

fn load_config(path: &str) -> Result<Config> {
    let text = std::fs::read_to_string(path)
        .context("failed to read config file")?;
    let config = toml::from_str(&text)
        .context("failed to parse config")?;
    Ok(config)
}
```

🏋️ **Exercise**: Write a function that reads a file, parses each line as an integer, and
returns the sum. Use `?` for error propagation and provide meaningful error context.
