# Ownership and Borrowing

This is the chapter. If you absorb one concept from this entire book, let it be ownership.
It's the idea that has no equivalent in TypeScript, and it's the reason Rust can guarantee
memory safety without a garbage collector.

## The Problem Ownership Solves

In TypeScript, the V8 garbage collector tracks every object and frees it when nothing
references it. This is convenient but has costs: GC pauses, unpredictable latency, and
higher memory usage. Rust replaces the garbage collector with three compile-time rules.

## The Three Rules

1. **Every value has exactly one owner** (a variable binding).
2. **When the owner goes out of scope, the value is dropped** (freed).
3. **At any given time, you can have *either* one mutable reference *or* any number of
   immutable references** — but not both.

That's it. These three rules, enforced at compile time, eliminate use-after-free, double-free,
data races, and dangling pointers.

## Move Semantics

🟦 **TypeScript** — assignment copies the reference, both variables point to the same object:
```typescript
const a = { name: "Alice" };
const b = a;       // b and a both reference the same object
console.log(a);    // ✅ works fine
```

🦀 **Rust** — assignment *moves* the value; the original is invalidated:
```rust
let a = String::from("Alice");
let b = a;          // value moves from a to b
// println!("{a}"); // ❌ compile error: value used after move
println!("{b}");    // ✅ b owns the string now
```

💡 **Key insight**: A move is not a copy and not a reference — it's a transfer of ownership.
After `let b = a;`, the variable `a` no longer exists conceptually. The compiler enforces this.

### Copy Types

Small, stack-allocated types implement the `Copy` trait and are *copied* instead of moved:

```rust
let x: i32 = 42;
let y = x;         // x is copied (integers are Copy)
println!("{x}");   // ✅ still valid
```

Types that are `Copy`: all integers, floats, `bool`, `char`, tuples of `Copy` types, arrays
of `Copy` types. Types that are *not* `Copy`: `String`, `Vec<T>`, `HashMap<K, V>`, any type
that manages heap memory.

### Clone — Explicit Deep Copy

When you genuinely need a copy of a non-`Copy` type:

```rust
let a = String::from("hello");
let b = a.clone();     // explicit deep copy
println!("{a}");       // ✅ a is still valid
println!("{b}");       // ✅ b is an independent copy
```

## Borrowing — References

Most of the time, you don't want to transfer ownership — you just want to *look at* a value
or temporarily modify it. This is borrowing.

### Immutable References (`&T`)

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
    // s goes out of scope here, but since it's a reference,
    // the value it refers to is NOT dropped
}

let name = String::from("Alice");
let len = calculate_length(&name);  // borrow name
println!("{name} is {len} bytes");  // ✅ name still valid
```

🟦 **TypeScript analogy**: This is like passing a `Readonly<T>` reference — you can read but
not modify. Except Rust enforces it at compile time, not just as a type hint.

### Mutable References (`&mut T`)

```rust
fn add_greeting(s: &mut String) {
    s.push_str(", hello!");
}

let mut name = String::from("Alice");
add_greeting(&mut name);
println!("{name}");  // "Alice, hello!"
```

### The Borrow Rules Visualized

```
    ┌──────────────────────────────────┐
    │ At any point in time, you have:  │
    │                                  │
    │   EITHER  many &T  (readers)     │
    │   OR      one &mut T (writer)    │
    │                                  │
    │   NEVER both at the same time    │
    └──────────────────────────────────┘
```

```rust
let mut data = vec![1, 2, 3];

let r1 = &data;       // ✅ first immutable borrow
let r2 = &data;       // ✅ second immutable borrow
println!("{r1:?} {r2:?}");

let r3 = &mut data;   // ✅ mutable borrow (r1 and r2 are no longer used)
r3.push(4);
```

⚠️ **Common pitfall**: The borrow checker uses *non-lexical lifetimes* (NLL). A borrow lasts
until its last use, not until the end of the scope. So this works:

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];
println!("{first}");   // last use of `first`
v.push(4);             // ✅ mutable borrow starts after immutable one ends
```

But this doesn't:
```rust
let mut v = vec![1, 2, 3];
let first = &v[0];
v.push(4);             // ❌ mutable borrow while immutable borrow is active
println!("{first}");   // first is used AFTER the push
```

## Ownership in Functions

```rust
fn take_ownership(s: String) {
    println!("{s}");
} // s is dropped here

fn borrow(s: &String) {
    println!("{s}");
} // s goes out of scope but the value is NOT dropped

let name = String::from("Alice");

borrow(&name);          // borrow — name still valid
take_ownership(name);   // move — name is consumed
// println!("{name}");  // ❌ name was moved
```

## Returning Ownership

```rust
fn create_greeting(name: &str) -> String {
    format!("Hello, {name}!")  // a new String is created and returned (moved to caller)
}

let greeting = create_greeting("Alice");  // caller now owns this String
```

## Thinking Like the Borrow Checker

When you get a borrow checker error, ask yourself:
1. **Who owns this value?** Trace back to the `let` binding.
2. **Is someone else still reading it?** Check for active `&T` references.
3. **Am I trying to mutate while someone reads?** That's the most common conflict.
4. **Can I restructure to use the borrow, then release it, then mutate?**

🏋️ **Exercise**: Write a function `longest(a: &str, b: &str) -> &str` that returns the longer
string. You'll discover you need a *lifetime annotation* — that's the topic of the next section.
