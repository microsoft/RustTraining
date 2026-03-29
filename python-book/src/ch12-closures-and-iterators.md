## Rust Closures vs Python Lambdas

> **What you'll learn:** Multi-line closures (not just one-expression lambdas), `Fn`/`FnMut`/`FnOnce` capture semantics,
> iterator chains vs list comprehensions, `map`/`filter`/`fold`, and `macro_rules!` basics.
>
> **Difficulty:** 🟡 Intermediate

### Python Closures and Lambdas
```python
# Python — lambdas are one-expression anonymous functions
double = lambda x: x * 2
result = double(5)  # 10

# Full closures capture variables from enclosing scope:
def make_adder(n):
    def adder(x):
        return x + n    # Captures `n` from outer scope
    return adder

add_5 = make_adder(5)
print(add_5(10))  # 15

# Higher-order functions:
numbers = [1, 2, 3, 4, 5]
doubled = list(map(lambda x: x * 2, numbers))
evens = list(filter(lambda x: x % 2 == 0, numbers))
```

### Rust Closures
```rust
// Rust — closures use |args| body syntax
let double = |x: i32| x * 2;
let result = double(5);  // 10

// Closures capture variables from enclosing scope:
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n    // `move` transfers ownership of `n` into the closure
}

let add_5 = make_adder(5);
println!("{}", add_5(10));  // 15

// Higher-order functions with iterators:
let numbers = vec![1, 2, 3, 4, 5];
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).copied().collect();
```

### Closure Syntax Comparison
```text
Python:                              Rust:
─────────                            ─────
lambda x: x * 2                      |x| x * 2
lambda x, y: x + y                   |x, y| x + y
lambda: 42                           || 42

# Multi-line
def f(x):                            |x| {
    y = x * 2                            let y = x * 2;
    return y + 1                         y + 1
                                      }
```

### Closure Capture — How Rust Differs
```python
# Python — closures capture by reference (late binding!)
funcs = [lambda: i for i in range(3)]
print([f() for f in funcs])  # [2, 2, 2] — surprise! All captured the same `i`

# Fix with default arg trick:
funcs = [lambda i=i: i for i in range(3)]
print([f() for f in funcs])  # [0, 1, 2]
```

```rust
// Rust — closures capture correctly (no late-binding gotcha)
let funcs: Vec<Box<dyn Fn() -> i32>> = (0..3)
    .map(|i| Box::new(move || i) as Box<dyn Fn() -> i32>)
    .collect();

let results: Vec<i32> = funcs.iter().map(|f| f()).collect();
println!("{:?}", results);  // [0, 1, 2] — correct!

// `move` captures a COPY of `i` for each closure — no late-binding surprise.
```

### Three Closure Traits
```rust
// Rust closures implement one or more of these traits:

// Fn — can be called multiple times, doesn't mutate captures (most common)
fn apply(f: impl Fn(i32) -> i32, x: i32) -> i32 { f(x) }

// FnMut — can be called multiple times, MAY mutate captures
fn apply_mut(mut f: impl FnMut(i32) -> i32, x: i32) -> i32 { f(x) }

// FnOnce — can only be called ONCE (consumes captures)
fn apply_once(f: impl FnOnce() -> String) -> String { f() }

// Python has no equivalent — closures are always Fn-like.
// In Rust, the compiler automatically determines which trait to use.
```

***

## Iterators vs Generators

### Python Generators
```python
# Python — generators with yield
def fibonacci():
    a, b = 0, 1
    while True:
        yield a
        a, b = b, a + b

# Lazy — values computed on demand
fib = fibonacci()
first_10 = [next(fib) for _ in range(10)]

# Generator expressions — like lazy list comprehensions
squares = (x ** 2 for x in range(1000000))  # No memory allocation
first_5 = [next(squares) for _ in range(5)]
```

### Rust Iterators
```rust
// Rust — Iterator trait (similar concept, different syntax)
struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.a;
        self.a = self.b;
        self.b = current + self.b;
        Some(current)
    }
}

// Lazy — values computed on demand (just like Python generators)
let first_10: Vec<u64> = Fibonacci::new().take(10).collect();

// Iterator chains — like generator expressions
let squares: Vec<u64> = (0..1_000_000u64).map(|x| x * x).take(5).collect();
```

***

## Comprehensions vs Iterator Chains

This section maps Python's comprehension syntax to Rust's iterator chains.

### List Comprehension → map/filter/collect
```python
# Python comprehensions:
squares = [x ** 2 for x in range(10)]
evens = [x for x in range(20) if x % 2 == 0]
names = [user.name for user in users if user.active]
pairs = [(x, y) for x in range(3) for y in range(3)]
flat = [item for sublist in nested for item in sublist]
```

```mermaid
flowchart LR
    A["Source<br>[1,2,3,4,5]"] -->|.iter\(\)| B["Iterator"]
    B -->|.filter\(\|x\| x%2==0\)| C["[2, 4]"]
    C -->|.map\(\|x\| x*x\)| D["[4, 16]"]
    D -->|.collect\(\)| E["Vec&lt;i32&gt;<br>[4, 16]"]
    style A fill:#ffeeba
    style E fill:#d4edda
```

> **Key insight**: Rust iterators are lazy — nothing happens until `.collect()`. Python's generators work similarly, but list comprehensions evaluate eagerly.

```rust
// Rust iterator chains:
let squares: Vec<i32> = (0..10).map(|x| x * x).collect();
let evens: Vec<i32> = (0..20).filter(|x| x % 2 == 0).collect();
let names: Vec<&str> = users.iter()
    .filter(|u| u.active)
    .map(|u| u.name.as_str())
    .collect();
let pairs: Vec<(i32, i32)> = (0..3)
    .flat_map(|x| (0..3).map(move |y| (x, y)))
    .collect();
let flat: Vec<i32> = nested.iter()
    .flat_map(|sublist| sublist.iter().copied())
    .collect();
```

### Dict Comprehension → collect into HashMap
```python
# Python
word_lengths = {word: len(word) for word in words}
inverted = {v: k for k, v in mapping.items()}
```

```rust
// Rust
let word_lengths: HashMap<&str, usize> = words.iter()
    .map(|w| (*w, w.len()))
    .collect();
let inverted: HashMap<&V, &K> = mapping.iter()
    .map(|(k, v)| (v, k))
    .collect();
```

### Set Comprehension → collect into HashSet
```python
# Python
unique_lengths = {len(word) for word in words}
```

```rust
// Rust
let unique_lengths: HashSet<usize> = words.iter()
    .map(|w| w.len())
    .collect();
```

### Common Iterator Methods

| Python | Rust | Notes |
|--------|------|-------|
| `map(f, iter)` | `.map(f)` | Transform each element |
| `filter(f, iter)` | `.filter(f)` | Keep matching elements |
| `sum(iter)` | `.sum()` | Sum all elements |
| `min(iter)` / `max(iter)` | `.min()` / `.max()` | Returns `Option` |
| `any(f(x) for x in iter)` | `.any(f)` | True if any match |
| `all(f(x) for x in iter)` | `.all(f)` | True if all match |
| `enumerate(iter)` | `.enumerate()` | Index + value |
| `zip(a, b)` | `a.zip(b)` | Pair elements |
| `len(list)` | `.count()` (consumes!) or `.len()` | Count elements |
| `list(reversed(x))` | `.rev()` | Reverse iteration |
| `itertools.chain(a, b)` | `a.chain(b)` | Concatenate iterators |
| `next(iter)` | `.next()` | Get next element |
| `next(iter, default)` | `.next().unwrap_or(default)` | With default |
| `list(iter)` | `.collect::<Vec<_>>()` | Materialize into collection |
| `sorted(iter)` | Collect, then `.sort()` | No lazy sorted iterator |
| `functools.reduce(f, iter)` | `.fold(init, f)` or `.reduce(f)` | Accumulate |

### Key Differences
```text
Python iterators:                     Rust iterators:
─────────────────                     ──────────────
- Lazy by default (generators)       - Lazy by default (all iterator chains)
- yield creates generators            - impl Iterator { fn next() }
- StopIteration to end               - None to end
- Can be consumed once               - Can be consumed once
- No type safety                      - Fully type-safe
- Slightly slower (interpreter)       - Zero-cost (compiled away)
```

***


<!-- ch12a: Macros -->
## Why Macros Exist in Rust

Python has no macro system — it uses decorators, metaclasses, and runtime
introspection for metaprogramming. Rust uses macros for compile-time code generation.

### Python Metaprogramming vs Rust Macros
```python
# Python — decorators and metaclasses for metaprogramming
from dataclasses import dataclass
from functools import wraps

@dataclass              # Generates __init__, __repr__, __eq__ at import time
class Point:
    x: float
    y: float

# Custom decorator
def log_calls(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        print(f"Calling {func.__name__}")
        return func(*args, **kwargs)
    return wrapper

@log_calls
def process(data):
    return data.upper()
```

```rust
// Rust — derive macros and declarative macros for code generation
#[derive(Debug, Clone, PartialEq)]  // Generates Debug, Clone, PartialEq impls at COMPILE time
struct Point {
    x: f64,
    y: f64,
}

// Declarative macro (like a template)
macro_rules! log_call {
    ($func_name:expr, $body:expr) => {
        println!("Calling {}", $func_name);
        $body
    };
}

fn process(data: &str) -> String {
    log_call!("process", data.to_uppercase())
}
```

### Common Built-in Macros
```rust
// These macros are used everywhere in Rust:

println!("Hello, {}!", name);           // Print with formatting
format!("Value: {}", x);               // Create formatted String
vec![1, 2, 3];                          // Create a Vec
assert_eq!(2 + 2, 4);                  // Test assertion
assert!(value > 0, "must be positive"); // Boolean assertion
dbg!(expression);                       // Debug print: prints expression AND value
todo!();                                // Placeholder — compiles but panics if reached
unimplemented!();                       // Mark code as unimplemented
panic!("something went wrong");         // Crash with message (like raise RuntimeError)

// Why are these macros instead of functions?
// - println! accepts variable arguments (Rust functions can't)
// - vec! generates code for any type and size
// - assert_eq! knows the SOURCE CODE of what you compared
// - dbg! knows the FILE NAME and LINE NUMBER
```

## Writing a Simple Macro with macro_rules!
```rust
// Python dict() equivalent
// Python: d = dict(a=1, b=2)
// Rust:   let d = hashmap!{ "a" => 1, "b" => 2 };

macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut map = std::collections::HashMap::new();
            $(map.insert($key, $value);)*
            map
        }
    };
}

let scores = hashmap! {
    "Alice" => 100,
    "Bob" => 85,
    "Charlie" => 90,
};
```

## Derive Macros — Auto-Implementing Traits
```rust
// #[derive(...)] is the Rust equivalent of Python's @dataclass decorator

// Python:
// @dataclass(frozen=True, order=True)
// class Student:
//     name: str
//     grade: int

// Rust:
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Student {
    name: String,
    grade: i32,
}

// Common derive macros:
// Debug         → {:?} formatting (like __repr__)
// Clone         → .clone() deep copy
// Copy          → implicit copy (only for simple types)
// PartialEq, Eq → == comparison (like __eq__)
// PartialOrd, Ord → <, >, sorting (like __lt__ etc.)
// Hash          → usable as HashMap key (like __hash__)
// Default       → MyType::default() (like __init__ with no args)

// Crate-provided derive macros:
// Serialize, Deserialize (serde) → JSON/YAML/TOML serialization
//                                  (like Python's json.dumps/loads but type-safe)
```

### Python Decorator vs Rust Derive

| Python Decorator | Rust Derive | Purpose |
|-----------------|-------------|---------|
| `@dataclass` | `#[derive(Debug, Clone, PartialEq)]` | Data class |
| `@dataclass(frozen=True)` | Immutable by default | Immutability |
| `@dataclass(order=True)` | `#[derive(Ord, PartialOrd)]` | Comparison/sorting |
| `@total_ordering` | `#[derive(PartialOrd, Ord)]` | Full ordering |
| JSON `json.dumps(obj.__dict__)` | `#[derive(Serialize)]` | Serialization |
| JSON `MyClass(**json.loads(s))` | `#[derive(Deserialize)]` | Deserialization |

---

## Exercises

<details>
<summary><strong>🏋️ Exercise: Derive and Custom Debug</strong> (click to expand)</summary>

**Challenge**: Create a `User` struct with fields `name: String`, `email: String`, and `password_hash: String`. Derive `Clone` and `PartialEq`, but implement `Debug` manually so it prints the name and email but redacts the password (shows `"***"` instead).

<details>
<summary>🔑 Solution</summary>

```rust
use std::fmt;

#[derive(Clone, PartialEq)]
struct User {
    name: String,
    email: String,
    password_hash: String,
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("User")
            .field("name", &self.name)
            .field("email", &self.email)
            .field("password_hash", &"***")
            .finish()
    }
}

fn main() {
    let user = User {
        name: "Alice".into(),
        email: "alice@example.com".into(),
        password_hash: "a1b2c3d4e5f6".into(),
    };
    println!("{user:?}");
    // Output: User { name: "Alice", email: "alice@example.com", password_hash: "***" }
}
```

**Key takeaway**: Unlike Python's `__repr__`, Rust lets you derive `Debug` for free — but you can override it for sensitive fields. This is safer than Python where `print(user)` might accidentally leak secrets.

</details>
</details>

***

