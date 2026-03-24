# Traits and Generics

## Traits — Rust's Interfaces

Traits are Rust's mechanism for shared behavior. They're similar to TypeScript interfaces, but
with important differences: traits can provide default implementations, and they use *nominal*
typing (types must explicitly implement a trait) rather than *structural* typing.

🟦 **TypeScript** — structural (duck) typing:
```typescript
interface Printable {
    display(): string;
}

// Any object with a display() method satisfies Printable — no declaration needed.
const item = { display: () => "hello" };
function print(p: Printable) { console.log(p.display()); }
print(item); // ✅ works because the shape matches
```

🦀 **Rust** — nominal typing:
```rust
trait Printable {
    fn display(&self) -> String;
}

struct Item;

impl Printable for Item {
    fn display(&self) -> String {
        "hello".to_string()
    }
}

fn print(p: &impl Printable) {
    println!("{}", p.display());
}
```

💡 **Key insight**: In TypeScript, compatibility is checked by shape. In Rust, a type must
explicitly `impl` a trait. This is more verbose but eliminates accidental matches and enables
the compiler to monomorphize generics.

## Default Implementations

```rust
trait Summary {
    fn title(&self) -> &str;

    // Default implementation — types can override or keep it
    fn summary(&self) -> String {
        format!("{} (read more...)", self.title())
    }
}

struct Article { title: String, body: String }

impl Summary for Article {
    fn title(&self) -> &str { &self.title }
    // summary() uses the default implementation
}
```

## Common Standard Library Traits

| Trait | TypeScript equivalent | Purpose |
|-------|---------------------|---------|
| `Display` | `toString()` | Human-readable formatting |
| `Debug` | `console.log` output | Developer-facing formatting |
| `Clone` | Spread / structuredClone | Deep copy |
| `Copy` | (primitive value semantics) | Implicit bitwise copy |
| `PartialEq` / `Eq` | `===` | Equality comparison |
| `PartialOrd` / `Ord` | `<`, `>`, compareFn | Ordering |
| `Default` | Default values | Provide a default value |
| `From` / `Into` | Type coercion / conversion | Type conversion |
| `Iterator` | `Symbol.iterator` | Iteration protocol |

### Deriving Traits

```rust
#[derive(Debug, Clone, PartialEq, Default)]
struct Config {
    host: String,
    port: u16,
    debug: bool,
}
```

`#[derive(...)]` auto-implements traits when all fields support them. This is like TypeScript
automatically getting serialization — but explicit.

## Trait Bounds — Constrained Generics

🟦 **TypeScript** — generic constraints:
```typescript
function longest<T extends { length: number }>(a: T, b: T): T {
    return a.length >= b.length ? a : b;
}
```

🦀 **Rust** — trait bounds:
```rust
fn longest<T: AsRef<str>>(a: T, b: T) -> T {
    if a.as_ref().len() >= b.as_ref().len() { a } else { b }
}
```

### Multiple Bounds

```rust
// Syntax 1: inline
fn process<T: Clone + Debug + Display>(item: T) { /* ... */ }

// Syntax 2: where clause (cleaner for complex bounds)
fn process<T>(item: T)
where
    T: Clone + Debug + Display,
{
    /* ... */
}
```

## `impl Trait` — Simplified Syntax

```rust
// In function arguments (accepts any type implementing the trait):
fn notify(item: &impl Summary) {
    println!("Breaking: {}", item.summary());
}

// In return position (returns some concrete type implementing the trait):
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}
```

## Dynamic Dispatch — Trait Objects

Sometimes you need to store different types implementing the same trait in a single collection.
This is like TypeScript's polymorphism:

🟦 **TypeScript**
```typescript
interface Shape { area(): number; }
const shapes: Shape[] = [new Circle(5), new Rectangle(3, 4)];
```

🦀 **Rust**
```rust
trait Shape { fn area(&self) -> f64; }

let shapes: Vec<Box<dyn Shape>> = vec![
    Box::new(Circle { radius: 5.0 }),
    Box::new(Rectangle { width: 3.0, height: 4.0 }),
];

for shape in &shapes {
    println!("{}", shape.area());
}
```

`dyn Shape` is a *trait object* — it uses a vtable for runtime dispatch (like virtual methods
in C++). `Box<dyn Shape>` is a heap-allocated trait object.

| | Static dispatch (`impl Trait` / generics) | Dynamic dispatch (`dyn Trait`) |
|--|---|---|
| Performance | Monomorphized, inlined | vtable lookup |
| Binary size | Larger (code duplicated per type) | Smaller |
| Flexibility | Type known at compile time | Type erased at runtime |

🏋️ **Exercise**: Define a `Renderable` trait with a `render(&self) -> String` method.
Implement it for `Paragraph`, `Heading`, and `Image` structs. Create a `Vec<Box<dyn Renderable>>`
and render all items.
