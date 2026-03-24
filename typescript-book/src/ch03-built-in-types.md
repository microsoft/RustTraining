# Built-in Types

## Type System Philosophy

TypeScript's type system is *structural* and *optional* — types are checked by shape, and you
can escape with `any`. Rust's type system is *nominal* and *mandatory* — every value has exactly
one concrete type, determined at compile time, and there is no escape hatch.

## Scalar Types

### Numbers

🟦 **TypeScript** — one `number` type (64-bit float) plus `bigint`:
```typescript
const count: number = 42;
const price: number = 9.99;
const big: bigint = 9007199254740993n;
```

🦀 **Rust** — explicit integer and float sizes:
```rust
let count: i32 = 42;          // signed 32-bit integer
let price: f64 = 9.99;        // 64-bit float
let small: u8 = 255;          // unsigned 8-bit
let big: i128 = 9_007_199_254_740_993;
let arch: usize = 42;         // pointer-sized unsigned (like size_t)
```

| TypeScript | Rust equivalents |
|-----------|-----------------|
| `number` | `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`, `f32`, `f64` |
| `bigint` | `i128`, `u128`, or the `num-bigint` crate |

⚠️ **Common pitfall**: Integer overflow panics in debug mode and wraps in release mode.
TypeScript silently loses precision. Use `checked_add`, `saturating_add`, or `wrapping_add` for
explicit control.

### Booleans

Identical concept, slightly different syntax:

```typescript
const done: boolean = true;
```

```rust
let done: bool = true;
```

### Characters

🟦 TypeScript has no character type — single characters are just `string`:
```typescript
const ch: string = "A";
```

🦀 Rust has `char`, a Unicode scalar value (4 bytes):
```rust
let ch: char = 'A';        // single quotes
let emoji: char = '🦀';
```

## Strings

This is where things diverge the most.

🟦 **TypeScript** — one `string` type, immutable, UTF-16 internally:
```typescript
const greeting: string = "hello";
const name: string = "world";
const message: string = `${greeting}, ${name}!`;
```

🦀 **Rust** — two primary string types:

| Type | Owned? | Mutable? | Where? | Analogy |
|------|--------|----------|--------|---------|
| `String` | Yes | Yes (if `mut`) | Heap | Like a `StringBuilder` |
| `&str` | No (borrowed) | No | Stack/heap/static | Like a read-only view |

```rust
let greeting: &str = "hello";                       // string literal → &str
let name: String = String::from("world");           // heap-allocated String
let message: String = format!("{greeting}, {name}!");
```

💡 **Key insight**: Think of `&str` as a *window* into string data owned by someone else, and
`String` as a string that *you* own and can grow. Most functions take `&str` as input (flexible)
and return `String` as output (owned).

```rust
// Accepts any string-like input
fn shout(s: &str) -> String {
    s.to_uppercase()
}

let owned = String::from("hello");
shout(&owned);     // &String auto-coerces to &str
shout("hello");    // &str directly
```

## The Unit Type — Rust's `void`

🟦 TypeScript:
```typescript
function log(msg: string): void {
    console.log(msg);
}
```

🦀 Rust:
```rust
fn log(msg: &str) {
    // return type is `()` (unit) — implied when omitted
    println!("{msg}");
}
```

The unit type `()` is a real type with exactly one value: `()`. It's like TypeScript's `void`
but can be stored in variables, used as generic parameters, etc.

## Type Inference

Both languages have inference, but Rust's is more powerful because types are never erased:

```rust
let x = 42;                // inferred as i32
let y = 3.14;              // inferred as f64
let names = vec!["a", "b"]; // inferred as Vec<&str>

// Sometimes you need turbofish syntax to help the compiler:
let parsed = "42".parse::<i32>().unwrap();
// or
let parsed: i32 = "42".parse().unwrap();
```

## Type Aliases

🟦 TypeScript:
```typescript
type UserId = number;
type Callback = (data: string) => void;
```

🦀 Rust:
```rust
type UserId = u64;
type Callback = fn(data: &str);
// or for closures:
type Callback = Box<dyn Fn(&str)>;
```

## Arrays and Tuples

### Fixed-size arrays

```rust
let rgb: [u8; 3] = [255, 128, 0];   // fixed size, known at compile time
let zeros = [0u8; 100];              // 100 zeros
```

### Tuples

🟦 TypeScript:
```typescript
const pair: [string, number] = ["age", 30];
const [key, value] = pair;
```

🦀 Rust:
```rust
let pair: (&str, i32) = ("age", 30);
let (key, value) = pair;              // destructuring
```

## Never Type

🟦 TypeScript:
```typescript
function fail(msg: string): never {
    throw new Error(msg);
}
```

🦀 Rust:
```rust
fn fail(msg: &str) -> ! {
    panic!("{msg}");
}
```

The `!` (never) type means the function never returns. Used for `panic!`, infinite loops, and
`process::exit`.

🏋️ **Exercise**: Write a function `describe(value: f64) -> String` that returns `"positive"`,
`"negative"`, or `"zero"`. Use type inference where possible.
