# From and Into Traits

## Type Conversions in Rust

TypeScript has implicit coercion (`"5" + 3 === "53"`) and explicit conversion (`Number("5")`).
Rust has no implicit coercion — all conversions are explicit, and the `From`/`Into` traits are
the idiomatic way to do them.

## `From<T>` — Construct Self from T

```rust
// String from &str
let s: String = String::from("hello");

// i64 from i32 (infallible widening)
let big: i64 = i64::from(42_i32);

// Vec<u8> from &str
let bytes: Vec<u8> = Vec::from("hello");
```

### Implementing From for Your Types

```rust
struct Email(String);

impl From<String> for Email {
    fn from(s: String) -> Self {
        Email(s)
    }
}

impl From<&str> for Email {
    fn from(s: &str) -> Self {
        Email(s.to_string())
    }
}

let email = Email::from("alice@example.com");
```

## `Into<T>` — Convert Self into T

`Into` is the reciprocal of `From`. When you implement `From<A> for B`, you automatically get
`Into<B> for A`. Use `Into` in function signatures to accept flexible input:

```rust
fn send_email(to: impl Into<Email>, body: &str) {
    let email: Email = to.into();
    // ...
}

send_email("alice@example.com", "Hello!");    // &str → Email
send_email(String::from("bob@b.com"), "Hi!"); // String → Email
```

## `TryFrom` / `TryInto` — Fallible Conversions

When conversion can fail, use the `Try` variants:

```rust
use std::num::TryFromIntError;

struct Port(u16);

impl TryFrom<i32> for Port {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 || value > 65535 {
            Err(format!("port out of range: {value}"))
        } else {
            Ok(Port(value as u16))
        }
    }
}

let port = Port::try_from(8080)?;  // Ok(Port(8080))
let bad = Port::try_from(-1);      // Err("port out of range: -1")
```

## `AsRef<T>` and `AsMut<T>` — Cheap Reference Conversions

Used for functions that accept either owned or borrowed types:

```rust
fn read_file(path: impl AsRef<std::path::Path>) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}

read_file("config.toml");                        // &str
read_file(String::from("config.toml"));           // String
read_file(std::path::PathBuf::from("config.toml")); // PathBuf
```

## `ToString` and `Display`

Implement `Display` to get `ToString` for free:

```rust
use std::fmt;

struct Temperature(f64);

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}°C", self.0)
    }
}

let t = Temperature(36.6);
println!("{t}");           // "36.6°C"
let s: String = t.to_string(); // also "36.6°C"
```

🏋️ **Exercise**: Create a `Celsius` and `Fahrenheit` struct. Implement `From<Celsius>` for
`Fahrenheit` and vice versa. Use the formula: F = C × 9/5 + 32.
