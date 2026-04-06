## Exceptions vs `Result<T, E>`

> **What you'll learn:** Why Rust replaces exceptions with `Result<T, E>` and `Option<T>`,
> the `?` operator for concise error propagation, and how explicit error handling
> eliminates hidden control flow that plagues C# `try`/`catch` code.
>
> **Difficulty:** 🟡 Intermediate
>
> **See also**: [Crate-Level Error Types](ch09-1-crate-level-error-types-and-result-alias.md) for production error patterns with `thiserror` and `anyhow`, and [Essential Crates](ch15-1-essential-crates-for-c-developers.md) for the error crate ecosystem.

### C# Exception-Based Error Handling
```csharp
// C# - Exception-based error handling
public class UserService
{
    public User GetUser(int userId)
    {
        if (userId <= 0)
        {
            throw new ArgumentException("User ID must be positive");
        }
        
        var user = database.FindUser(userId);
        if (user == null)
        {
            throw new UserNotFoundException($"User {userId} not found");
        }
        
        return user;
    }
    
    public async Task<string> GetUserEmailAsync(int userId)
    {
        try
        {
            var user = GetUser(userId);
            return user.Email ?? throw new InvalidOperationException("User has no email");
        }
        catch (UserNotFoundException ex)
        {
            logger.Warning("User not found: {UserId}", userId);
            return "noreply@company.com";
        }
        catch (Exception ex)
        {
            logger.Error(ex, "Unexpected error getting user email");
            throw; // Re-throw
        }
    }
}
```

### Rust Result-Based Error Handling
```rust
use std::fmt;

#[derive(Debug)]
pub enum UserError {
    InvalidId(i32),
    NotFound(i32),
    NoEmail,
    DatabaseError(String),
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserError::InvalidId(id) => write!(f, "Invalid user ID: {}", id),
            UserError::NotFound(id) => write!(f, "User {} not found", id),
            UserError::NoEmail => write!(f, "User has no email address"),
            UserError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl std::error::Error for UserError {}

#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub email: Option<String>,
}

pub struct UserService {
    users: Vec<User>,  // Simulated database
}

impl UserService {
    fn database_find_user(&self, user_id: i32) -> Option<User> {
        self.users.get(user_id as usize).cloned()
    }

    pub fn get_user(&self, user_id: i32) -> Result<User, UserError> {
        if user_id <= 0 {
            return Err(UserError::InvalidId(user_id));
        }
        
        // Simulate database lookup
        self.database_find_user(user_id)
            .ok_or(UserError::NotFound(user_id))
    }
    
    pub fn get_user_email(&self, user_id: i32) -> Result<String, UserError> {
        let user = self.get_user(user_id)?; // ? operator propagates errors
        
        user.email
            .ok_or(UserError::NoEmail)
    }
    
    pub fn get_user_email_or_default(&self, user_id: i32) -> String {
        match self.get_user_email(user_id) {
            Ok(email) => email,
            Err(UserError::NotFound(_)) => {
                log::warn!("User not found: {}", user_id);
                "noreply@company.com".to_string()
            }
            Err(err) => {
                log::error!("Error getting user email: {}", err);
                "error@company.com".to_string()
            }
        }
    }
}
```

```mermaid
graph TD
    subgraph "C# Exception Model"
        CS_CALL["Method Call"]
        CS_SUCCESS["Success Path"]
        CS_EXCEPTION["throw Exception"]
        CS_STACK["Stack unwinding<br/>(Runtime cost)"]
        CS_CATCH["try/catch block"]
        CS_HIDDEN["[ERROR] Hidden control flow<br/>[ERROR] Performance cost<br/>[ERROR] Easy to ignore"]
        
        CS_CALL --> CS_SUCCESS
        CS_CALL --> CS_EXCEPTION
        CS_EXCEPTION --> CS_STACK
        CS_STACK --> CS_CATCH
        CS_EXCEPTION --> CS_HIDDEN
    end
    
    subgraph "Rust Result Model"
        RUST_CALL["Function Call"]
        RUST_OK["Ok(value)"]
        RUST_ERR["Err(error)"]
        RUST_MATCH["match result"]
        RUST_QUESTION["? operator<br/>(early return)"]
        RUST_EXPLICIT["[OK] Explicit error handling<br/>[OK] Zero runtime cost<br/>[OK] Cannot ignore errors"]
        
        RUST_CALL --> RUST_OK
        RUST_CALL --> RUST_ERR
        RUST_OK --> RUST_MATCH
        RUST_ERR --> RUST_MATCH
        RUST_ERR --> RUST_QUESTION
        RUST_MATCH --> RUST_EXPLICIT
        RUST_QUESTION --> RUST_EXPLICIT
    end
    
    style CS_HIDDEN fill:#ffcdd2,color:#000
    style RUST_EXPLICIT fill:#c8e6c9,color:#000
    style CS_STACK fill:#fff3e0,color:#000
    style RUST_QUESTION fill:#c8e6c9,color:#000
```

***

### The ? Operator: Propagating Errors Concisely
```csharp
// C# - Exception propagation (implicit)
public async Task<string> ProcessFileAsync(string path)
{
    var content = await File.ReadAllTextAsync(path);  // Throws on error
    var processed = ProcessContent(content);          // Throws on error
    return processed;
}
```

```rust
// Rust - Error propagation with ?
fn process_file(path: &str) -> Result<String, ConfigError> {
    let content = read_config(path)?;  // ? propagates error if Err
    let processed = process_content(&content)?;  // ? propagates error if Err
    Ok(processed)  // Wrap success value in Ok
}

fn process_content(content: &str) -> Result<String, ConfigError> {
    if content.is_empty() {
        Err(ConfigError::InvalidFormat)
    } else {
        Ok(content.to_uppercase())
    }
}
```

### `Option<T>` for Nullable Values
```csharp
// C# - Nullable reference types
public string? FindUserName(int userId)
{
    var user = database.FindUser(userId);
    return user?.Name;  // Returns null if user not found
}

public void ProcessUser(int userId)
{
    string? name = FindUserName(userId);
    if (name != null)
    {
        Console.WriteLine($"User: {name}");
    }
    else
    {
        Console.WriteLine("User not found");
    }
}
```

```rust
// Rust - Option<T> for optional values
fn find_user_name(user_id: u32) -> Option<String> {
    // Simulate database lookup
    if user_id == 1 {
        Some("Alice".to_string())
    } else {
        None
    }
}

fn process_user(user_id: u32) {
    match find_user_name(user_id) {
        Some(name) => println!("User: {}", name),
        None => println!("User not found"),
    }
    
    // Or use if let (pattern matching shorthand)
    if let Some(name) = find_user_name(user_id) {
        println!("User: {}", name);
    } else {
        println!("User not found");
    }
}
```

### Combining Option and Result
```rust
fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b != 0.0 {
        Some(a / b)
    } else {
        None
    }
}

fn parse_and_divide(a_str: &str, b_str: &str) -> Result<Option<f64>, ParseFloatError> {
    let a: f64 = a_str.parse()?;  // Return parse error if invalid
    let b: f64 = b_str.parse()?;  // Return parse error if invalid
    Ok(safe_divide(a, b))         // Return Ok(Some(result)) or Ok(None)
}

use std::num::ParseFloatError;

fn main() {
    match parse_and_divide("10.0", "2.0") {
        Ok(Some(result)) => println!("Result: {}", result),
        Ok(None) => println!("Division by zero"),
        Err(error) => println!("Parse error: {}", error),
    }
}
```

***


<details>
<summary><strong>🏋️ Exercise: Build a Crate-Level Error Type</strong> (click to expand)</summary>

**Challenge**: Create an `AppError` enum for a file processing application that can fail due to I/O errors, JSON parse errors, and validation errors. Implement `From` conversions for automatic `?` propagation.

```rust
// Starter code
use std::io;

// TODO: Define AppError with variants:
//   Io(io::Error), Json(serde_json::Error), Validation(String)
// TODO: Implement Display and Error traits
// TODO: Implement From<io::Error> and From<serde_json::Error>
// TODO: Define type alias: type Result<T> = std::result::Result<T, AppError>;

fn load_config(path: &str) -> Result<Config> {
    let content = std::fs::read_to_string(path)?;  // io::Error → AppError
    let config: Config = serde_json::from_str(&content)?;  // serde error → AppError
    if config.name.is_empty() {
        return Err(AppError::Validation("name cannot be empty".into()));
    }
    Ok(config)
}
```

<details>
<summary>🔑 Solution</summary>

```rust
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Validation: {0}")]
    Validation(String),
}

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(serde::Deserialize)]
struct Config {
    name: String,
    port: u16,
}

fn load_config(path: &str) -> Result<Config> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&content)?;
    if config.name.is_empty() {
        return Err(AppError::Validation("name cannot be empty".into()));
    }
    Ok(config)
}
```

**Key takeaways**:
- `thiserror` generates `Display` and `Error` impls from attributes
- `#[from]` generates `From<T>` impls, enabling automatic `?` conversion
- The `Result<T>` alias eliminates boilerplate throughout your crate
- Unlike C# exceptions, the error type is visible in every function signature

</details>
</details>


