# Generics Deep Dive

## Monomorphization — Generics at Zero Cost

In TypeScript, generics are erased at compile time — `Array<number>` and `Array<string>` are
the same `Array` at runtime. In Rust, the compiler generates specialized code for each concrete
type used. This is called *monomorphization*.

```rust
fn double<T: std::ops::Mul<Output = T> + Copy>(x: T) -> T {
    x * x
}

// When you call:
double(3_i32);     // compiler generates: fn double_i32(x: i32) -> i32
double(2.5_f64);   // compiler generates: fn double_f64(x: f64) -> f64
```

This means generics in Rust have zero runtime overhead — they're as fast as hand-written
specialized functions.

## Generic Structs

```rust
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }
}

// Conditional implementations
impl<T: Display, U: Display> Pair<T, U> {
    fn display(&self) {
        println!("({}, {})", self.first, self.second);
    }
}
```

## Generic Enums

You've already seen the two most important generic enums:

```rust
enum Option<T> { Some(T), None }
enum Result<T, E> { Ok(T), Err(E) }
```

## Associated Types vs Generic Parameters

🟦 **TypeScript** — generic interface parameter:
```typescript
interface Iterator<T> {
    next(): T | undefined;
}
```

🦀 **Rust** — associated type:
```rust
trait Iterator {
    type Item;              // associated type
    fn next(&mut self) -> Option<Self::Item>;
}
```

Associated types are used when there's exactly one natural type per implementation. Generic
parameters are used when a type can implement the trait multiple times with different types:

```rust
// Associated type: a Vec<i32> has ONE Item type
impl Iterator for MyIter {
    type Item = i32;
    fn next(&mut self) -> Option<i32> { /* ... */ }
}

// Generic parameter: a type can implement From<T> for MANY T's
impl From<String> for MyType { /* ... */ }
impl From<i32> for MyType { /* ... */ }
```

## Phantom Types — Types Without Data

A powerful pattern for encoding state in the type system:

```rust
use std::marker::PhantomData;

struct Draft;
struct Published;

struct Article<State> {
    title: String,
    body: String,
    _state: PhantomData<State>,
}

impl Article<Draft> {
    fn new(title: String) -> Self {
        Article { title, body: String::new(), _state: PhantomData }
    }

    fn publish(self) -> Article<Published> {
        Article { title: self.title, body: self.body, _state: PhantomData }
    }
}

impl Article<Published> {
    fn url(&self) -> String {
        format!("/articles/{}", self.title.to_lowercase().replace(' ', "-"))
    }
}

// draft.url()     // ❌ compile error: method not found
// published.url() // ✅ works
```

This has no TypeScript equivalent — it's compile-time state tracking with zero runtime cost.

🏋️ **Exercise**: Create a generic `Stack<T>` with `push`, `pop`, and `peek` methods. Add a
`display` method that's only available when `T: Display`.
