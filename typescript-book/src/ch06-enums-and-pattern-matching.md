# Enums and Pattern Matching

## TypeScript Discriminated Unions → Rust Enums

This chapter will feel familiar. TypeScript's discriminated unions and Rust's enums solve the
same problem — representing a value that can be one of several distinct variants — but Rust
enums are more powerful because the compiler fully understands them.

🟦 **TypeScript**
```typescript
type Shape =
    | { kind: "circle"; radius: number }
    | { kind: "rectangle"; width: number; height: number }
    | { kind: "point" };

function area(shape: Shape): number {
    switch (shape.kind) {
        case "circle":
            return Math.PI * shape.radius ** 2;
        case "rectangle":
            return shape.width * shape.height;
        case "point":
            return 0;
    }
}
```

🦀 **Rust**
```rust
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Point,
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
        Shape::Rectangle { width, height } => width * height,
        Shape::Point => 0.0,
    }
}
```

💡 **Key insight**: Rust enums are *algebraic data types* (tagged unions). Each variant can
hold different data. The `match` is exhaustive — if you add a new variant, every `match` in
your codebase will produce a compile error until you handle it. TypeScript can approximate
this with `never` exhaustiveness checks, but Rust enforces it natively.

## Enum Variants

Rust enums support three kinds of variants:

```rust
enum Message {
    Quit,                            // unit variant (no data)
    Echo(String),                    // tuple variant (unnamed fields)
    Move { x: i32, y: i32 },        // struct variant (named fields)
    Color(u8, u8, u8),              // tuple variant with multiple fields
}
```

## Methods on Enums

Just like structs, enums can have `impl` blocks:

```rust
impl Message {
    fn is_quit(&self) -> bool {
        matches!(self, Message::Quit)
    }

    fn describe(&self) -> String {
        match self {
            Message::Quit => "quit".to_string(),
            Message::Echo(text) => format!("echo: {text}"),
            Message::Move { x, y } => format!("move to ({x}, {y})"),
            Message::Color(r, g, b) => format!("color: ({r}, {g}, {b})"),
        }
    }
}
```

## Option and Result Are Just Enums

This is a crucial realization. They're not special language features — they're regular enums
defined in the standard library:

```rust
// Simplified definitions:
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Because they're enums, everything you learn about pattern matching applies to them.

## Advanced Pattern Matching

### Guards

```rust
match temperature {
    t if t < 0 => println!("freezing"),
    0..=20 => println!("cold"),
    21..=30 => println!("comfortable"),
    _ => println!("hot"),
}
```

### Or-patterns

```rust
match status_code {
    200 | 201 | 204 => println!("success"),
    301 | 302 => println!("redirect"),
    400..=499 => println!("client error"),
    500..=599 => println!("server error"),
    _ => println!("unknown"),
}
```

### Nested Destructuring

```rust
struct Point { x: f64, y: f64 }

enum PlotItem {
    Marker(Point),
    Line(Point, Point),
}

match item {
    PlotItem::Marker(Point { x, y }) => {
        println!("marker at ({x}, {y})");
    }
    PlotItem::Line(Point { x: x1, y: y1 }, Point { x: x2, y: y2 }) => {
        println!("line from ({x1}, {y1}) to ({x2}, {y2})");
    }
}
```

### Binding with @

```rust
match response.status {
    status @ 200..=299 => println!("success: {status}"),
    status @ 400..=499 => println!("client error: {status}"),
    status => println!("other: {status}"),
}
```

## C-like Enums

When you just need named constants (like TypeScript's `enum`):

🟦 **TypeScript**
```typescript
enum Direction {
    North,
    South,
    East,
    West,
}
```

🦀 **Rust**
```rust
#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}
```

These can also have explicit discriminant values:

```rust
#[repr(u8)]
enum HttpMethod {
    Get = 0,
    Post = 1,
    Put = 2,
    Delete = 3,
}
```

## The `matches!` Macro

A concise way to check if a value matches a pattern without a full `match` block:

```rust
let is_vowel = matches!(ch, 'a' | 'e' | 'i' | 'o' | 'u');
```

🏋️ **Exercise**: Define an `HttpResponse` enum with variants `Ok(String)` (body),
`Redirect(String)` (url), `NotFound`, and `Error(u16, String)` (status code, message).
Write a function that returns appropriate log messages for each variant.
