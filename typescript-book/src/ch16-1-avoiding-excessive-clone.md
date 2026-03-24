# Avoiding Excessive clone()

## The Problem

When learning Rust, it's tempting to `.clone()` everything to silence the borrow checker. This
works but defeats Rust's zero-cost philosophy — every clone is a heap allocation.

## When clone() Is Fine

- **Small, infrequently-cloned data** — a configuration string cloned once at startup.
- **Prototyping** — get it working first, optimize later.
- **Shared ownership semantics** — when you genuinely need independent copies.
- **Arc::clone()** — this only increments a reference count, not a deep copy.

## When to Avoid clone()

### Pattern 1: Borrow Instead of Clone

```rust
// ❌ Clones the entire string just to read it
fn print_name(user: &User) {
    let name = user.name.clone();
    println!("{name}");
}

// ✅ Borrow it
fn print_name(user: &User) {
    println!("{}", user.name);
}
```

### Pattern 2: Take Ownership When You Need It

```rust
// ❌ Clone then discard original
fn process(items: &Vec<String>) {
    let owned = items.clone();
    consume(owned);
}

// ✅ Take ownership directly
fn process(items: Vec<String>) {
    consume(items);
}
```

### Pattern 3: Use Cow for Maybe-Owned Data

`Cow` (Clone-on-Write) borrows when possible and only clones when mutation is needed:

```rust
use std::borrow::Cow;

fn normalize(input: &str) -> Cow<'_, str> {
    if input.contains(' ') {
        Cow::Owned(input.replace(' ', "-"))  // allocates only when needed
    } else {
        Cow::Borrowed(input)                  // zero-cost borrow
    }
}
```

### Pattern 4: Return References from Methods

```rust
struct Database {
    records: Vec<Record>,
}

impl Database {
    // ❌ Clones the record
    fn find(&self, id: u64) -> Option<Record> {
        self.records.iter().find(|r| r.id == id).cloned()
    }

    // ✅ Returns a reference
    fn find(&self, id: u64) -> Option<&Record> {
        self.records.iter().find(|r| r.id == id)
    }
}
```

## Profiling Clone Usage

Use `cargo clippy` to find unnecessary clones:
```bash
cargo clippy -- -W clippy::redundant_clone
```

🏋️ **Exercise**: Review a Rust program you've written and identify every `.clone()` call.
For each, determine if it can be replaced with a borrow, ownership transfer, or `Cow`.
