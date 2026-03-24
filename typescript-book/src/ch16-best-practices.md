# Best Practices

## Idiomatic Rust for TypeScript Developers

### Prefer `&str` over `String` in Function Parameters

```rust
// ❌ Requires callers to allocate a String
fn greet(name: String) -> String { format!("Hello, {name}!") }

// ✅ Accepts both &str and &String
fn greet(name: &str) -> String { format!("Hello, {name}!") }
```

### Use `impl Into<T>` for Flexible APIs

```rust
fn connect(host: impl Into<String>, port: u16) -> Connection {
    let host = host.into();
    // ...
}

connect("localhost", 8080);              // &str
connect(String::from("localhost"), 8080); // String
```

### Prefer Iterators over Index Loops

```rust
// ❌ C-style loop
for i in 0..items.len() {
    process(&items[i]);
}

// ✅ Iterator
for item in &items {
    process(item);
}
```

### Use `clippy` Religiously

`cargo clippy` catches hundreds of common mistakes and anti-patterns. Run it on every commit,
like you'd run ESLint. Add it to CI:

```yaml
- run: cargo clippy -- -D warnings
```

### Derive What You Can

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Config { /* ... */ }
```

Derive `Debug` on almost everything. Add `Clone`, `PartialEq` when needed. This is free code
that saves time.

### Use `todo!()` for Incremental Development

```rust
fn complex_algorithm(data: &[f64]) -> Vec<f64> {
    todo!("implement after the basic structure works")
}
```

`todo!()` compiles but panics at runtime — perfect for sketching out a program's structure
before filling in the details. Like writing `throw new Error("TODO")` in TypeScript.

### Prefer `expect()` over `unwrap()`

```rust
// ❌ Panics with an unhelpful message
let port = env::var("PORT").unwrap();

// ✅ Panics with context
let port = env::var("PORT").expect("PORT environment variable must be set");
```

### Use Type-State Patterns

Instead of runtime checks for invalid state transitions, encode valid states in the type
system (see Chapter 10's phantom types section).

### Error Messages That Help

```rust
// ❌ Generic error
return Err("invalid input".into());

// ✅ Specific, actionable error
return Err(format!(
    "expected port between 1 and 65535, got {port}"
).into());
```

🏋️ **Exercise**: Take a Rust program you've written during this book and run `cargo clippy`
on it. Fix every warning and note the patterns clippy teaches you.
