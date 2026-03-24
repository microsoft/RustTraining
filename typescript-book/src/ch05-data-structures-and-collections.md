# Data Structures and Collections

## Structs — Rust's Interfaces-Made-Real

In TypeScript, you define shape with `interface` or `type`, but the runtime object is just a
plain JavaScript object. In Rust, `struct` defines both the shape *and* the memory layout.

🟦 **TypeScript**
```typescript
interface User {
    name: string;
    age: number;
    email: string;
}

const alice: User = { name: "Alice", age: 30, email: "alice@example.com" };
```

🦀 **Rust**
```rust
struct User {
    name: String,
    age: u32,
    email: String,
}

let alice = User {
    name: String::from("Alice"),
    age: 30,
    email: String::from("alice@example.com"),
};
```

### Field Shorthand

Both languages support shorthand when variable names match field names:

```typescript
const name = "Alice";
const user = { name, age: 30 };  // shorthand
```

```rust
let name = String::from("Alice");
let user = User { name, age: 30, email: String::from("a@b.com") };
```

### Struct Update Syntax (Spread)

🟦 TypeScript:
```typescript
const updated = { ...alice, age: 31 };
```

🦀 Rust:
```rust
let updated = User { age: 31, ..alice };
```

⚠️ **Pitfall**: This *moves* fields from `alice` that aren't `Copy`. After this, `alice.name`
and `alice.email` are moved and can no longer be used (but `alice.age` is fine because integers
are `Copy`).

## Impl Blocks — Methods on Structs

TypeScript puts methods inside the class. Rust separates data (`struct`) from behavior (`impl`):

🟦 **TypeScript**
```typescript
class Rectangle {
    constructor(public width: number, public height: number) {}

    area(): number {
        return this.width * this.height;
    }

    static square(size: number): Rectangle {
        return new Rectangle(size, size);
    }
}
```

🦀 **Rust**
```rust
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // Method (takes &self)
    fn area(&self) -> f64 {
        self.width * self.height
    }

    // Associated function (no self — like a static method)
    fn square(size: f64) -> Self {
        Self { width: size, height: size }
    }
}

let r = Rectangle::square(5.0);
println!("Area: {}", r.area());
```

💡 **Key insight**: `&self` is shorthand for `self: &Self`. Methods borrow `self` by default.
Use `&mut self` if the method needs to mutate, or `self` if it consumes the struct.

## Tuple Structs and Newtypes

```rust
struct Meters(f64);
struct Seconds(f64);

let distance = Meters(100.0);
let time = Seconds(9.58);
// distance + time → compile error! Different types.
```

This is like TypeScript's branded types, but enforced at the compiler level rather than
as a convention.

## Collections

### Vec — Dynamic Array

🟦 TypeScript: `Array<T>` / `T[]`
🦀 Rust: `Vec<T>`

```rust
let mut nums: Vec<i32> = Vec::new();
nums.push(1);
nums.push(2);
nums.push(3);

// Or use the vec! macro:
let nums = vec![1, 2, 3];

// Access
let first = nums[0];           // panics if out of bounds
let first = nums.get(0);       // returns Option<&i32>
```

### HashMap — Object / Map

🟦 TypeScript: `Record<string, T>` / `Map<K, V>`
🦀 Rust: `HashMap<K, V>`

```rust
use std::collections::HashMap;

let mut scores: HashMap<String, i32> = HashMap::new();
scores.insert("Alice".to_string(), 100);
scores.insert("Bob".to_string(), 85);

// Access
if let Some(score) = scores.get("Alice") {
    println!("Alice: {score}");
}

// Entry API (like Map.has + set)
scores.entry("Charlie".to_string()).or_insert(0);
```

### HashSet

🟦 TypeScript: `Set<T>`
🦀 Rust: `HashSet<T>`

```rust
use std::collections::HashSet;

let mut tags: HashSet<String> = HashSet::new();
tags.insert("rust".to_string());
tags.insert("wasm".to_string());
tags.contains("rust"); // true
```

### BTreeMap / BTreeSet

Sorted versions of `HashMap` and `HashSet`. Use when you need ordered keys.

### VecDeque

Double-ended queue — efficient push/pop from both ends. TypeScript equivalent:
`Array` used as a deque (but `Array.shift()` is O(n) in JS, while `VecDeque::pop_front()`
is O(1)).

## Option and Result — No More null / undefined

This is arguably the most important section for TypeScript developers.

### Option<T> — replaces `T | null | undefined`

🟦 **TypeScript**
```typescript
function findUser(id: number): User | undefined {
    return users.find(u => u.id === id);
}

const user = findUser(42);
if (user) {
    console.log(user.name);
}
```

🦀 **Rust**
```rust
fn find_user(id: u64) -> Option<User> {
    users.iter().find(|u| u.id == id).cloned()
}

// Pattern matching
match find_user(42) {
    Some(user) => println!("{}", user.name),
    None => println!("not found"),
}

// Or more concisely:
if let Some(user) = find_user(42) {
    println!("{}", user.name);
}
```

### Result<T, E> — replaces try/catch

🟦 **TypeScript**
```typescript
function parseConfig(path: string): Config {
    const data = fs.readFileSync(path, "utf-8"); // might throw
    return JSON.parse(data);                      // might throw
}
```

🦀 **Rust**
```rust
fn parse_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string(path)?;   // ? propagates error
    let config: Config = serde_json::from_str(&data)?;
    Ok(config)
}
```

💡 **Key insight**: The `?` operator is Rust's answer to `try/catch`. If the `Result` is `Ok`,
it unwraps the value. If it's `Err`, it returns the error from the current function immediately.
It's like writing `if (err) return err;` after every operation — but in one character.

🏋️ **Exercise**: Create a `Student` struct with `name: String` and `grades: Vec<f64>`. Add a
method `average(&self) -> Option<f64>` that returns `None` if there are no grades.
