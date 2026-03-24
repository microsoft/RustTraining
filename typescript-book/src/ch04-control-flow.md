# Control Flow

## Expressions vs Statements

The single biggest conceptual shift: in Rust, almost everything is an **expression** that
returns a value. In TypeScript, `if` is a statement; in Rust, it's an expression.

🟦 **TypeScript**
```typescript
// if is a statement — you need a ternary or variable
const label = x > 0 ? "positive" : "non-positive";
```

🦀 **Rust**
```rust
// if is an expression — no ternary operator needed
let label = if x > 0 { "positive" } else { "non-positive" };
```

💡 **Key insight**: The last expression in a block (without a semicolon) is the block's return
value. Adding a semicolon turns it into a statement that returns `()`.

```rust
let val = {
    let a = 1;
    let b = 2;
    a + b       // ← no semicolon → this is the block's value (3)
};
```

## `if` / `else if` / `else`

🟦 **TypeScript**
```typescript
if (temp > 30) {
    console.log("hot");
} else if (temp > 20) {
    console.log("warm");
} else {
    console.log("cold");
}
```

🦀 **Rust**
```rust
if temp > 30 {
    println!("hot");
} else if temp > 20 {
    println!("warm");
} else {
    println!("cold");
}
```

No parentheses around the condition (they're allowed but `clippy` will warn). Braces are
always required — no single-line `if` without braces.

## Loops

### `loop` — infinite loop (no TypeScript equivalent)

```rust
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;   // break with a value!
    }
};
// result == 20
```

### `while`

Nearly identical to TypeScript:

```rust
let mut n = 0;
while n < 5 {
    println!("{n}");
    n += 1;
}
```

### `for` — Range-based iteration

🟦 **TypeScript**
```typescript
for (let i = 0; i < 5; i++) { … }
for (const item of items) { … }
```

🦀 **Rust**
```rust
for i in 0..5 { … }           // 0, 1, 2, 3, 4
for i in 0..=5 { … }          // 0, 1, 2, 3, 4, 5 (inclusive)
for item in &items { … }      // borrow each item
for item in items { … }       // consume (move) the collection
for (i, item) in items.iter().enumerate() { … }  // index + value
```

⚠️ **Common pitfall**: `for item in items` *moves* the vector. After the loop, `items` is no
longer usable. Use `for item in &items` to borrow instead.

## `match` — TypeScript's `switch` on Steroids

`match` is Rust's most powerful control flow construct. It's like TypeScript's `switch`, but:
- It must be *exhaustive* — every possible value must be covered.
- It can destructure values.
- It returns a value (it's an expression).

🟦 **TypeScript**
```typescript
switch (status) {
    case "ok":
        return 200;
    case "not_found":
        return 404;
    default:
        return 500;
}
```

🦀 **Rust**
```rust
match status {
    "ok" => 200,
    "not_found" => 404,
    _ => 500,             // _ is the wildcard / default
}
```

### Pattern Matching with Destructuring

```rust
let point = (3, -5);
match point {
    (0, 0) => println!("origin"),
    (x, 0) => println!("on x-axis at {x}"),
    (0, y) => println!("on y-axis at {y}"),
    (x, y) if x > 0 && y > 0 => println!("quadrant I"),
    (x, y) => println!("at ({x}, {y})"),
}
```

### `if let` — Match a Single Pattern

🟦 **TypeScript** (with discriminated unions)
```typescript
if (shape.kind === "circle") {
    console.log(shape.radius);
}
```

🦀 **Rust**
```rust
if let Shape::Circle { radius } = shape {
    println!("{radius}");
}
```

### `let else` — The Inverse

```rust
let Some(count) = maybe_count else {
    return Err("no count provided");
};
// count is now available here, unwrapped
```

This is similar to TypeScript's early-return pattern with narrowing:
```typescript
if (maybeCount === undefined) {
    throw new Error("no count provided");
}
// maybeCount is now narrowed to number
```

## `while let`

Useful for consuming an iterator or repeatedly matching:

```rust
let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    println!("{top}");
}
// prints 3, 2, 1
```

🏋️ **Exercise**: Write a `match` expression that takes a letter grade (`'A'`, `'B'`, `'C'`,
`'D'`, `'F'`) and returns the corresponding GPA value as an `f64`. Handle invalid grades with
a default arm.
