# Lifetimes for TypeScript Developers

## Why Lifetimes Exist

In TypeScript, the garbage collector knows when to free memory by counting references at
runtime. In Rust, the compiler must prove at *compile time* that every reference is valid.
Lifetimes are the annotations that help the compiler do this.

## The Problem

```rust
fn longest(a: &str, b: &str) -> &str {
    if a.len() >= b.len() { a } else { b }
}
```

This won't compile. The compiler asks: *"The return type is a reference — but a reference to
what? Does the returned reference live as long as `a` or `b`?"* It needs help.

## Lifetime Annotations

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}
```

`'a` is a *lifetime parameter*. It says: "the returned reference lives at least as long as
the shorter of the two input lifetimes." The compiler uses this to verify callers don't use the
result after either input is dropped.

💡 **Key insight**: Lifetime annotations don't change how long values live. They describe
relationships between references so the compiler can verify correctness. They're like
TypeScript type annotations — they don't change runtime behavior, just help the type checker.

## Lifetime Elision Rules

Most of the time, you don't need to write lifetime annotations. The compiler applies three
elision rules automatically:

1. Each reference parameter gets its own lifetime: `fn foo(x: &str, y: &str)` becomes
   `fn foo<'a, 'b>(x: &'a str, y: &'b str)`.
2. If there's exactly one input lifetime, it's applied to all output references:
   `fn foo(x: &str) -> &str` becomes `fn foo<'a>(x: &'a str) -> &'a str`.
3. If one of the parameters is `&self` or `&mut self`, its lifetime is applied to all
   output references.

These rules cover ~95% of cases. You only write explicit lifetimes when the compiler can't
figure it out — typically when you have multiple reference inputs and a reference output.

## Lifetimes in Structs

If a struct holds a reference, it needs a lifetime annotation:

```rust
struct Excerpt<'a> {
    text: &'a str,
}

let novel = String::from("Call me Ishmael. Some years ago...");
let first_sentence = novel.split('.').next().unwrap();
let excerpt = Excerpt { text: first_sentence };
// excerpt cannot outlive novel, because it borrows from it
```

🟦 **TypeScript analogy**: Imagine if TypeScript enforced that an object holding a reference
to another object's data could never outlive that object — no dangling references, guaranteed
at compile time.

## The `'static` Lifetime

`'static` means "lives for the entire duration of the program." String literals have this
lifetime:

```rust
let s: &'static str = "I live forever";
```

⚠️ **Pitfall**: Seeing `'static` in error messages usually means the compiler wants you to own
the data rather than borrow it. The fix is often changing `&str` to `String`.

## Common Patterns

### Returning Owned Data Avoids Lifetime Issues

When in doubt, return an owned type:

```rust
// Simple: no lifetime needed
fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}
```

### Storing References vs Owned Data

```rust
// This struct borrows — needs a lifetime
struct BorrowedConfig<'a> {
    name: &'a str,
}

// This struct owns — no lifetime needed
struct OwnedConfig {
    name: String,
}
```

For TypeScript developers, start with owned data (`String`, `Vec<T>`) everywhere. Optimize to
borrowed references later when profiling shows it matters.

🏋️ **Exercise**: Write a struct `Highlight<'a>` that holds a `&'a str` reference to a portion
of text and a `color: &'a str`. Write a function that creates a `Highlight` from a text string.
