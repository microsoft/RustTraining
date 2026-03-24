# Testing Patterns

## Built-in Test Framework

Unlike TypeScript (where you pick jest, vitest, mocha, etc.), Rust has testing built into
`cargo`. No installation, no configuration.

🟦 **TypeScript (vitest)**
```typescript
import { describe, it, expect } from "vitest";
import { add } from "./math";

describe("add", () => {
    it("adds two numbers", () => {
        expect(add(2, 3)).toBe(5);
    });
});
```

🦀 **Rust**
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_numbers() {
        assert_eq!(add(2, 3), 5);
    }
}
```

Run with: `cargo test`

## Test Assertions

| vitest / jest | Rust | Purpose |
|--------------|------|---------|
| `expect(x).toBe(y)` | `assert_eq!(x, y)` | Equality |
| `expect(x).not.toBe(y)` | `assert_ne!(x, y)` | Inequality |
| `expect(condition).toBeTruthy()` | `assert!(condition)` | Boolean check |
| Custom message | `assert!(cond, "msg: {}", val)` | With context |

## Testing for Errors

🟦 **TypeScript**
```typescript
expect(() => parsePort("abc")).toThrow();
```

🦀 **Rust**
```rust
#[test]
fn parse_invalid_port() {
    let result = parse_port("abc");
    assert!(result.is_err());
}

#[test]
#[should_panic(expected = "out of bounds")]
fn panics_on_invalid_index() {
    let v = vec![1, 2, 3];
    let _ = v[99];
}
```

## Integration Tests

Place files in a `tests/` directory at the project root:

```
my-project/
├── src/
│   └── lib.rs
├── tests/
│   └── integration_test.rs
└── Cargo.toml
```

```rust
// tests/integration_test.rs
use my_project::add;

#[test]
fn integration_add() {
    assert_eq!(add(10, 20), 30);
}
```

Each file in `tests/` is compiled as a separate crate, so it only sees your public API.

## Doc Tests

Rust can test code examples in documentation comments:

```rust
/// Adds two numbers.
///
/// # Examples
///
/// ```
/// assert_eq!(my_crate::add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

`cargo test` runs these doc examples as tests. This ensures your documentation never goes stale.

🏋️ **Exercise**: Write a `divide(a: f64, b: f64) -> Result<f64, String>` function with tests
for the happy path, division by zero, and a doc test.
