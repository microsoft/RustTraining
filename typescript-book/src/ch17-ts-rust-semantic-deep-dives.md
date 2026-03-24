# TypeScript → Rust Semantic Deep Dives

## Concepts That Look Similar but Behave Differently

This chapter catalogs the subtle semantic differences between TypeScript and Rust that trip
up experienced developers.

## 1. Equality

🟦 **TypeScript**: `===` compares by reference for objects, by value for primitives.
🦀 **Rust**: `==` always uses the `PartialEq` trait — defaults to value comparison for derived
types.

```rust
let a = String::from("hello");
let b = String::from("hello");
assert_eq!(a, b);  // true — compares contents, not pointer identity

// For reference identity, compare pointers:
let x = &a;
let y = &a;
assert!(std::ptr::eq(x, y));  // true — same address
```

## 2. Truthiness

🟦 **TypeScript**: `""`, `0`, `null`, `undefined`, `NaN`, `false` are all falsy.
🦀 **Rust**: Only `bool` can be used in conditions. No implicit conversions.

```rust
let s = String::new();
// if s { }            // ❌ compile error
if !s.is_empty() { }  // ✅
```

## 3. Destructuring with Ownership

🟦 **TypeScript**: Destructuring always copies the reference.
🦀 **Rust**: Destructuring can *move* fields out of a struct.

```rust
struct Pair { name: String, value: i32 }

let pair = Pair { name: "x".into(), value: 42 };
let Pair { name, value } = pair;
// `pair` is now partially moved — name was moved, value was copied
// println!("{}", pair.name);  // ❌ moved
// println!("{}", pair.value); // ❌ partial move prevents this too
```

## 4. Method Resolution

🟦 **TypeScript**: Method lookup walks the prototype chain at runtime.
🦀 **Rust**: Method lookup uses auto-ref and auto-deref at compile time.

```rust
let s = String::from("hello");
s.len();        // compiler auto-borrows: (&s).len()
(&s).len();     // explicit borrow — same thing
(&&s).len();    // auto-deref through multiple references
```

## 5. Closures and Ownership

🟦 **TypeScript**: Closures always capture by reference. Variables are shared.
🦀 **Rust**: Closures capture by the least restrictive mode needed (borrow → mut borrow → move).

```rust
let mut name = String::from("Alice");

// This closure borrows `name` immutably
let greet = || println!("Hello, {name}");

// This closure borrows `name` mutably
let mut update = || name.push_str(" Smith");

// This closure moves `name` into itself
let consume = move || println!("Consumed: {name}");
```

## 6. Iteration and Ownership

🟦 **TypeScript**: `for...of` never consumes the iterable.
🦀 **Rust**: `for x in collection` consumes it by default.

```rust
let v = vec![1, 2, 3];

for x in &v { }     // borrows — v is still usable
for x in &mut v { } // mutably borrows
for x in v { }      // MOVES — v is consumed, no longer usable
```

## 7. String Concatenation

🟦 **TypeScript**: `"a" + "b"` just works.
🦀 **Rust**: String operations are explicit about ownership.

```rust
let a = String::from("hello");
let b = String::from(" world");

// These all work but differently:
let c = a + &b;           // a is MOVED, b is borrowed. a is now invalid.
let c = format!("{a}{b}"); // neither moved — uses references
let c = [a.as_str(), b.as_str()].concat(); // borrows both
```

## 8. Default Values

🟦 **TypeScript**: `const x = opts.value ?? "default";`
🦀 **Rust**: `let x = opts.value.unwrap_or("default".to_string());`

```rust
// More options:
let x = opts.value.unwrap_or_default();          // uses Default trait
let x = opts.value.unwrap_or_else(|| expensive()); // lazy default
let x = opts.value.map_or("fallback", |v| v);    // map + default
```

## 9. Spread / Rest

🟦 **TypeScript**: `const [first, ...rest] = arr;`
🦀 **Rust**: Pattern matching with slices:

```rust
let v = vec![1, 2, 3, 4, 5];
match v.as_slice() {
    [first, rest @ ..] => println!("first: {first}, rest: {rest:?}"),
    [] => println!("empty"),
}
```

## 10. Typeof / Type Checking at Runtime

🟦 **TypeScript**: `typeof x === "string"`, `x instanceof MyClass`
🦀 **Rust**: No runtime type information. Use enums or trait objects.

```rust
// Instead of runtime type checks, use enums:
enum Value {
    Str(String),
    Num(f64),
    Bool(bool),
}

fn process(v: &Value) {
    match v {
        Value::Str(s) => println!("string: {s}"),
        Value::Num(n) => println!("number: {n}"),
        Value::Bool(b) => println!("bool: {b}"),
    }
}
```

🏋️ **Exercise**: For each of the 10 deep dives above, write a small Rust program that
demonstrates the behavior. Try to predict the output before running it.
