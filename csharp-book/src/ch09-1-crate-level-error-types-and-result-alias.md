## Crate-Level Error Types and Result Aliases

> **What you'll learn:** The production pattern of defining a per-crate error enum with `thiserror`,
> creating a `Result<T>` type alias, and when to choose `thiserror` (libraries) vs `anyhow` (applications).
>
> **Difficulty:** 🟡 Intermediate

A critical pattern for production Rust: define a per-crate error enum and a `Result` type alias to eliminate boilerplate.

### The Pattern
```rust
// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Validation error: {message}")]
    Validation { message: String },

    #[error("Not found: {entity} with id {id}")]
    NotFound { entity: String, id: String },
}

/// Crate-wide Result alias — every function returns this
pub type Result<T> = std::result::Result<T, AppError>;
```

### Usage Throughout Your Crate
```rust
use crate::error::{AppError, Result};

// Assumes a database pool is available, e.g.:
// async fn get_user(pool: &PgPool, id: Uuid) -> Result<User>
// Here we show the pattern with `pool` as shorthand.
pub async fn get_user(id: Uuid) -> Result<User> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_optional(&pool)
        .await?;  // sqlx::Error → AppError::Database via #[from]

    user.ok_or_else(|| AppError::NotFound {
        entity: "User".into(),
        id: id.to_string(),
    })
}

pub async fn create_user(req: CreateUserRequest) -> Result<User> {
    if req.name.trim().is_empty() {
        return Err(AppError::Validation {
            message: "Name cannot be empty".into(),
        });
    }
    // ...
}
```

### C# Comparison
```csharp
// C# equivalent pattern
public class AppException : Exception
{
    public string ErrorCode { get; }
    public AppException(string code, string message) : base(message)
    {
        ErrorCode = code;
    }
}

// But in C#, callers don't know what exceptions to expect!
// In Rust, the error type is in the function signature.
```

### Why This Matters
- **`thiserror`** generates `Display` and `Error` impls automatically
- **`#[from]`** enables the `?` operator to convert library errors automatically
- The `Result<T>` alias means every function signature is clean: `fn foo() -> Result<Bar>`
- **Unlike C# exceptions**, callers see all possible error variants in the type


### thiserror vs anyhow: When to Use Which

Two crates dominate Rust error handling. Choosing between them is the first decision you'll make:

| | `thiserror` | `anyhow` |
|---|---|---|
| **Purpose** | Define structured error types for **libraries** | Quick error handling for **applications** |
| **Output** | Custom enum you control | Opaque `anyhow::Error` wrapper |
| **Caller sees** | All error variants in the type | Just `anyhow::Error` — opaque |
| **Best for** | Library crates, APIs, any code with consumers | Binaries, scripts, prototypes, CLI tools |
| **Downcasting** | `match` on variants directly | `error.downcast_ref::<MyError>()` |

```rust
// thiserror — for LIBRARIES (callers need to match on error variants)
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("File not found: {path}")]
    NotFound { path: String },

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub fn read_config(path: &str) -> Result<String, StorageError> {
    std::fs::read_to_string(path).map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => StorageError::NotFound { path: path.into() },
        std::io::ErrorKind::PermissionDenied => StorageError::PermissionDenied(path.into()),
        _ => StorageError::Io(e),
    })
}
```

```rust
// anyhow — for APPLICATIONS (just propagate errors, don't define types)
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let config = std::fs::read_to_string("config.toml")
        .context("Failed to read config file")?;

    let port: u16 = config.parse()
        .context("Failed to parse port number")?;

    println!("Listening on port {port}");
    Ok(())
}
// anyhow::Result<T> = Result<T, anyhow::Error>
// .context() adds human-readable context to any error
```

```csharp
// C# comparison:
// thiserror ≈ defining custom exception classes with specific properties
// anyhow ≈ catching Exception and wrapping with message:
//   throw new InvalidOperationException("Failed to read config", ex);
```

**Guideline**: If your code is a **library** (other code calls it), use `thiserror`. If your code is an **application** (the final binary), use `anyhow`. Many projects use both — `thiserror` for the library crate's public API, `anyhow` in the `main()` binary.

### Error Recovery Patterns

C# developers are used to `try/catch` blocks that recover from specific exceptions. Rust uses combinators on `Result` for the same purpose:

```rust
use std::fs;

// Pattern 1: Recover with a fallback value
let config = fs::read_to_string("config.toml")
    .unwrap_or_else(|_| String::from("port = 8080"));  // default if missing

// Pattern 2: Recover from specific errors, propagate others
fn read_or_create(path: &str) -> Result<String, std::io::Error> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let default = String::from("# new file");
            fs::write(path, &default)?;
            Ok(default)
        }
        Err(e) => Err(e),  // propagate permission errors, etc.
    }
}

// Pattern 3: Add context before propagating
use anyhow::Context;

fn load_config() -> anyhow::Result<Config> {
    let text = fs::read_to_string("config.toml")
        .context("Failed to read config.toml")?;
    let config: Config = toml::from_str(&text)
        .context("Failed to parse config.toml")?;
    Ok(config)
}

// Pattern 4: Map errors to your domain type
fn parse_port(s: &str) -> Result<u16, AppError> {
    s.parse::<u16>()
        .map_err(|_| AppError::Validation {
            message: format!("Invalid port: {s}"),
        })
}
```

```csharp
// C# equivalents:
try { config = File.ReadAllText("config.toml"); }
catch (FileNotFoundException) { config = "port = 8080"; }  // Pattern 1

try { /* ... */ }
catch (FileNotFoundException) { /* create file */ }        // Pattern 2
catch { throw; }                                            // re-throw others
```

**When to recover vs propagate:**
- **Recover** when the error has a sensible default or retry strategy
- **Propagate with `?`** when the *caller* should decide what to do
- **Add context** (`.context()`) at module boundaries to build an error trail

---

## Exercises

<details>
<summary><strong>🏋️ Exercise: Design a Crate Error Type</strong> (click to expand)</summary>

You're building a user registration service. Design the error type using `thiserror`:

1. Define `RegistrationError` with variants: `DuplicateEmail(String)`, `WeakPassword(String)`, `DatabaseError(#[from] sqlx::Error)`, `RateLimited { retry_after_secs: u64 }`
2. Create a `type Result<T> = std::result::Result<T, RegistrationError>;` alias
3. Write a `register_user(email: &str, password: &str) -> Result<()>` that demonstrates `?` propagation and explicit error construction

<details>
<summary>🔑 Solution</summary>

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegistrationError {
    #[error("Email already registered: {0}")]
    DuplicateEmail(String),

    #[error("Password too weak: {0}")]
    WeakPassword(String),

    #[error("Database error")]
    Database(#[from] sqlx::Error),

    #[error("Rate limited — retry after {retry_after_secs}s")]
    RateLimited { retry_after_secs: u64 },
}

pub type Result<T> = std::result::Result<T, RegistrationError>;

pub fn register_user(email: &str, password: &str) -> Result<()> {
    if password.len() < 8 {
        return Err(RegistrationError::WeakPassword(
            "must be at least 8 characters".into(),
        ));
    }

    // This ? converts sqlx::Error → RegistrationError::Database automatically
    // db.check_email_unique(email).await?;

    // This is explicit construction for domain logic
    if email.contains("+spam") {
        return Err(RegistrationError::DuplicateEmail(email.to_string()));
    }

    Ok(())
}
```

**Key pattern**: `#[from]` enables `?` for library errors; explicit `Err(...)` for domain logic. The Result alias keeps every signature clean.

</details>
</details>

***


