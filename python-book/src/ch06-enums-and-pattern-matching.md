## Algebraic Data Types vs Union Types

> **What you'll learn:** Rust enums with data vs Python `Union` types, exhaustive `match` vs `match/case`,
> `Option<T>` as a compile-time replacement for `None`, and guard patterns.
>
> **Difficulty:** 🟡 Intermediate

Python 3.10 introduced `match` statements and type unions. Rust's enums go further —
each variant can carry different data, and the compiler ensures you handle every case.

### Python Union Types and Match
```python
# Python 3.10+ — structural pattern matching
from typing import Union
from dataclasses import dataclass

@dataclass
class Circle:
    radius: float

@dataclass
class Rectangle:
    width: float
    height: float

@dataclass
class Triangle:
    base: float
    height: float

Shape = Union[Circle, Rectangle, Triangle]  # Type alias

def area(shape: Shape) -> float:
    match shape:
        case Circle(radius=r):
            return 3.14159 * r * r
        case Rectangle(width=w, height=h):
            return w * h
        case Triangle(base=b, height=h):
            return 0.5 * b * h
        # No compiler warning if you miss a case!
        # Adding a new shape? grep the codebase and hope you find all match blocks.
```

### Rust Enums — Data-Carrying Variants
```rust
// Rust — enum variants carry data, compiler enforces exhaustive matching
enum Shape {
    Circle(f64),                // Circle carries radius
    Rectangle(f64, f64),        // Rectangle carries width, height
    Triangle { base: f64, height: f64 }, // Named fields also work
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle(w, h) => w * h,
        Shape::Triangle { base, height } => 0.5 * base * height,
        // ❌ If you add Shape::Pentagon and forget to handle it here,
        //    the compiler refuses to build. No grep needed.
    }
}
```

> **Key insight**: Rust's `match` is **exhaustive** — the compiler verifies you handle
> every variant. Add a new variant to an enum and the compiler tells you exactly which
> `match` blocks need updating. Python's `match` has no such guarantee.

### Enums Replace Multiple Python Patterns

```python
# Python — several patterns that Rust enums replace:

# 1. String constants
STATUS_PENDING = "pending"
STATUS_ACTIVE = "active"
STATUS_CLOSED = "closed"

# 2. Python Enum (no data)
from enum import Enum
class Status(Enum):
    PENDING = "pending"
    ACTIVE = "active"
    CLOSED = "closed"

# 3. Tagged unions (class + type field)
class Message:
    def __init__(self, kind, **data):
        self.kind = kind
        self.data = data
# Message(kind="text", content="hello")
# Message(kind="image", url="...", width=100)
```

```rust
// Rust — one enum does all three and more

// 1. Simple enum (like Python's Enum)
enum Status {
    Pending,
    Active,
    Closed,
}

// 2. Data-carrying enum (tagged union — type-safe!)
enum Message {
    Text(String),
    Image { url: String, width: u32, height: u32 },
    Quit,                    // No data
    Move { x: i32, y: i32 },
}
```

```mermaid
flowchart TD
    E["enum Message"] --> T["Text(String)<br>🏷️ tag=0 + String data"]
    E --> I["Image { url, width, height }<br>🏷️ tag=1 + 3 fields"]
    E --> Q["Quit<br>🏷️ tag=2 + no data"]
    E --> M["Move { x, y }<br>🏷️ tag=3 + 2 fields"]
    style E fill:#d4edda,stroke:#28a745
    style T fill:#fff3cd
    style I fill:#fff3cd
    style Q fill:#fff3cd
    style M fill:#fff3cd
```

> **Memory insight**: Rust enums are "tagged unions" — the compiler stores a discriminant tag + enough space for the largest variant. Python's equivalent (`Union[str, dict, None]`) has no compact representation.
>
> 📌 **See also**: [Ch. 9 — Error Handling](ch09-error-handling.md) uses enums extensively — `Result<T, E>` and `Option<T>` are just enums with `match`.

```rust
fn process(msg: &Message) {
    match msg {
        Message::Text(content) => println!("Text: {content}"),
        Message::Image { url, width, height } => {
            println!("Image: {url} ({width}x{height})")
        }
        Message::Quit => println!("Quitting"),
        Message::Move { x, y } => println!("Moving to ({x}, {y})"),
    }
}
```

***

## Exhaustive Pattern Matching

### Python's match — Not Exhaustive
```python
# Python — the wildcard case is optional, no compiler help
def describe(value):
    match value:
        case 0:
            return "zero"
        case 1:
            return "one"
        # If you forget the default, Python returns None silently.
        # No warning, no error.

describe(42)  # Returns None — a silent bug
```

### Rust's match — Compiler-Enforced
```rust
// Rust — MUST handle every possible case
fn describe(value: i32) -> &'static str {
    match value {
        0 => "zero",
        1 => "one",
        // ❌ Compile error: non-exhaustive patterns: `i32::MIN..=-1_i32`
        //    and `2_i32..=i32::MAX` not covered
        _ => "other",   // _ = catch-all (required for open-ended types)
    }
}

// For enums, NO catch-all needed — compiler knows all variants:
enum Color { Red, Green, Blue }

fn color_hex(c: Color) -> &'static str {
    match c {
        Color::Red => "#ff0000",
        Color::Green => "#00ff00",
        Color::Blue => "#0000ff",
        // No _ needed — all variants covered
        // Add Color::Yellow later → compiler error HERE
    }
}
```

### Pattern Matching Features
```rust
// Multiple values (like Python's case 1 | 2 | 3:)
match value {
    1 | 2 | 3 => println!("small"),
    4..=9 => println!("medium"),    // Range patterns
    _ => println!("large"),
}

// Guards (like Python's case x if x > 0:)
match temperature {
    t if t > 100 => println!("boiling"),
    t if t < 0 => println!("freezing"),
    t => println!("normal: {t}°"),
}

// Nested destructuring
let point = (3, (4, 5));
match point {
    (0, _) => println!("on y-axis"),
    (_, (0, _)) => println!("y=0"),
    (x, (y, z)) => println!("x={x}, y={y}, z={z}"),
}
```

***

## Option for None Safety

`Option<T>` is the most important Rust enum for Python developers. It replaces
`None` with a type-safe alternative.

### Python None

```python
# Python — None is a value that can appear anywhere
def find_user(user_id: int) -> dict | None:
    users = {1: {"name": "Alice"}}
    return users.get(user_id)

user = find_user(999)
# user is None — but nothing forces you to check!
print(user["name"])  # 💥 TypeError at runtime
```

### Rust Option

```rust
// Rust — Option<T> forces you to handle the None case
fn find_user(user_id: i64) -> Option<User> {
    let users = HashMap::from([(1, User { name: "Alice".into() })]);
    users.get(&user_id).cloned()
}

let user = find_user(999);
// user is Option<User> — you CANNOT use it without handling None

// Method 1: match
match find_user(999) {
    Some(user) => println!("Found: {}", user.name),
    None => println!("Not found"),
}

// Method 2: if let (like Python's if (x := expr) is not None)
if let Some(user) = find_user(1) {
    println!("Found: {}", user.name);
}

// Method 3: unwrap_or
let name = find_user(999)
    .map(|u| u.name)
    .unwrap_or_else(|| "Unknown".to_string());

// Method 4: ? operator (in functions that return Option)
fn get_user_name(id: i64) -> Option<String> {
    let user = find_user(id)?;     // Returns None early if not found
    Some(user.name)
}
```

### Option Methods — Python Equivalents

| Pattern | Python | Rust |
|---------|--------|------|
| Check if exists | `if x is not None:` | `if let Some(x) = opt {` |
| Default value | `x or default` | `opt.unwrap_or(default)` |
| Default factory | `x or compute()` | `opt.unwrap_or_else(\|\| compute())` |
| Transform if exists | `f(x) if x else None` | `opt.map(f)` |
| Chain lookups | `x and x.attr and x.attr.method()` | `opt.and_then(\|x\| x.method())` |
| Crash if None | Not possible to prevent | `opt.unwrap()` (panic) or `opt.expect("msg")` |
| Get or raise | `x if x else raise` | `opt.ok_or(Error)?` |

---

## Exercises

<details>
<summary><strong>🏋️ Exercise: Shape Area Calculator</strong> (click to expand)</summary>

**Challenge**: Define an enum `Shape` with variants `Circle(f64)` (radius), `Rectangle(f64, f64)` (width, height), and `Triangle(f64, f64)` (base, height). Implement a method `fn area(&self) -> f64` using `match`. Create one of each and print the area.

<details>
<summary>🔑 Solution</summary>

```rust
use std::f64::consts::PI;

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => PI * r * r,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle(b, h) => 0.5 * b * h,
        }
    }
}

fn main() {
    let shapes = [
        Shape::Circle(5.0),
        Shape::Rectangle(4.0, 6.0),
        Shape::Triangle(3.0, 8.0),
    ];
    for shape in &shapes {
        println!("Area: {:.2}", shape.area());
    }
}
```

**Key takeaway**: Rust enums replace Python's `Union[Circle, Rectangle, Triangle]` + `isinstance()` checks. The compiler ensures you handle every variant — adding a new shape without updating `area()` is a compile error.

</details>
</details>

***


