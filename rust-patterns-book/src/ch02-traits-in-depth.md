# 2. Traits In Depth 🟡

> **What you'll learn:**
> - Associated types vs generic parameters — and when to use each
> - GATs, blanket impls, marker traits, and dyn compatibility rules
> - How vtables and fat pointers work under the hood
> - Extension traits, enum dispatch, and typed command patterns

## Associated Types vs Generic Parameters

Both let a trait work with different types, but they serve different purposes:

```rust
// --- ASSOCIATED TYPE: One implementation per type ---
trait Iterator {
    type Item; // Each iterator produces exactly ONE kind of item

    fn next(&mut self) -> Option<Self::Item>;
}

// A custom iterator that always yields i32 — there's no choice
struct Counter { max: i32, current: i32 }

impl Iterator for Counter {
    type Item = i32; // Exactly one Item type per implementation
    fn next(&mut self) -> Option<i32> {
        if self.current < self.max {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}

// --- GENERIC PARAMETER: Multiple implementations per type ---
trait Convert<T> {
    fn convert(&self) -> T;
}

// A single type can implement Convert for MANY target types:
impl Convert<f64> for i32 {
    fn convert(&self) -> f64 { *self as f64 }
}
impl Convert<String> for i32 {
    fn convert(&self) -> String { self.to_string() }
}
```

**When to use which**:

| Use | When |
|-----|------|
| **Associated type** | There's exactly ONE natural output/result per implementing type. `Iterator::Item`, `Deref::Target`, `Add::Output` |
| **Generic parameter** | A type can meaningfully implement the trait for MANY different types. `From<T>`, `AsRef<T>`, `PartialEq<Rhs>` |

**Intuition**: If it makes sense to ask "what is the `Item` of this iterator?", use associated type. If it makes sense to ask "can this convert to `f64`? to `String`? to `bool`?", use a generic parameter.

```rust
// Real-world example: std::ops::Add
trait Add<Rhs = Self> {
    type Output; // Associated type — addition has ONE result type
    fn add(self, rhs: Rhs) -> Self::Output;
}

// Rhs is a generic parameter — you can add different types to Meters:
struct Meters(f64);
struct Centimeters(f64);

impl Add<Meters> for Meters {
    type Output = Meters;
    fn add(self, rhs: Meters) -> Meters { Meters(self.0 + rhs.0) }
}
impl Add<Centimeters> for Meters {
    type Output = Meters;
    fn add(self, rhs: Centimeters) -> Meters { Meters(self.0 + rhs.0 / 100.0) }
}
```

### Generic Associated Types (GATs)

Since Rust 1.65, associated types can have generic parameters of their own.
This enables **lending iterators** — iterators that return references tied to
the iterator rather than to the underlying collection:

```rust
// Without GATs — impossible to express a lending iterator:
// trait LendingIterator {
//     type Item<'a>;  // ← This was rejected before 1.65
// }

// With GATs (Rust 1.65+):
trait LendingIterator {
    type Item<'a> where Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>>;
}

// Example: an iterator that yields overlapping windows
struct WindowIter<'data> {
    data: &'data [u8],
    pos: usize,
    window_size: usize,
}

impl<'data> LendingIterator for WindowIter<'data> {
    type Item<'a> = &'a [u8] where Self: 'a;

    fn next(&mut self) -> Option<&[u8]> {
        if self.pos + self.window_size <= self.data.len() {
            let window = &self.data[self.pos..self.pos + self.window_size];
            self.pos += 1;
            Some(window)
        } else {
            None
        }
    }
}
```

> **When you need GATs**: Lending iterators, streaming parsers, or any trait
> where the associated type's lifetime depends on the `&self` borrow.
> For most code, plain associated types are sufficient.

### Supertraits and Trait Hierarchies

Traits can require other traits as prerequisites, forming hierarchies:

```mermaid
graph BT
    Display["Display"]
    Debug["Debug"]
    Error["Error"]
    Clone["Clone"]
    Copy["Copy"]
    PartialEq["PartialEq"]
    Eq["Eq"]
    PartialOrd["PartialOrd"]
    Ord["Ord"]

    Error --> Display
    Error --> Debug
    Copy --> Clone
    Eq --> PartialEq
    Ord --> Eq
    Ord --> PartialOrd
    PartialOrd --> PartialEq

    style Display fill:#e8f4f8,stroke:#2980b9,color:#000
    style Debug fill:#e8f4f8,stroke:#2980b9,color:#000
    style Error fill:#fdebd0,stroke:#e67e22,color:#000
    style Clone fill:#d4efdf,stroke:#27ae60,color:#000
    style Copy fill:#d4efdf,stroke:#27ae60,color:#000
    style PartialEq fill:#fef9e7,stroke:#f1c40f,color:#000
    style Eq fill:#fef9e7,stroke:#f1c40f,color:#000
    style PartialOrd fill:#fef9e7,stroke:#f1c40f,color:#000
    style Ord fill:#fef9e7,stroke:#f1c40f,color:#000
```

> Arrows point from subtrait to supertrait: implementing `Error` requires `Display` + `Debug`.

A trait can require that implementors also implement other traits:

```rust
use std::fmt;

// Display is a supertrait of Error
trait Error: fmt::Display + fmt::Debug {
    fn source(&self) -> Option<&(dyn Error + 'static)> { None }
}
// Any type implementing Error MUST also implement Display and Debug

// Build your own hierarchies:
trait Identifiable {
    fn id(&self) -> u64;
}

trait Timestamped {
    fn created_at(&self) -> chrono::DateTime<chrono::Utc>;
}

// Entity requires both:
trait Entity: Identifiable + Timestamped {
    fn is_active(&self) -> bool;
}

// Implementing Entity forces you to implement all three:
struct User { id: u64, name: String, created: chrono::DateTime<chrono::Utc> }

impl Identifiable for User {
    fn id(&self) -> u64 { self.id }
}
impl Timestamped for User {
    fn created_at(&self) -> chrono::DateTime<chrono::Utc> { self.created }
}
impl Entity for User {
    fn is_active(&self) -> bool { true }
}
```

### Blanket Implementations

Implement a trait for ALL types that satisfy some bound:

```rust
// std does this: any type that implements Display automatically gets ToString
impl<T: fmt::Display> ToString for T {
    fn to_string(&self) -> String {
        format!("{self}")
    }
}
// Now i32, &str, your custom types — anything with Display — gets to_string() for free.

// Your own blanket impl:
trait Loggable {
    fn log(&self);
}

// Every Debug type is automatically Loggable:
impl<T: std::fmt::Debug> Loggable for T {
    fn log(&self) {
        eprintln!("[LOG] {self:?}");
    }
}

// Now ANY Debug type has .log():
// 42.log();              // [LOG] 42
// "hello".log();         // [LOG] "hello"
// vec![1, 2, 3].log();   // [LOG] [1, 2, 3]
```

> **Caution**: Blanket impls are powerful but irreversible — you can't add a
> more specific impl for a type that's already covered by a blanket impl
> (orphan rules + coherence). Design them carefully.

### Marker Traits

Traits with no methods — they mark a type as having some property:

```rust
// Standard library marker traits:
// Send    — safe to transfer between threads
// Sync    — safe to share (&T) between threads
// Unpin   — safe to move after pinning
// Sized   — has a known size at compile time
// Copy    — can be duplicated with memcpy

// Your own marker trait:
/// Marker: this sensor has been factory-calibrated
trait Calibrated {}

struct RawSensor { reading: f64 }
struct CalibratedSensor { reading: f64 }

impl Calibrated for CalibratedSensor {}

// Only calibrated sensors can be used in production:
fn record_measurement<S: Calibrated>(sensor: &S) {
    // ...
}
// record_measurement(&RawSensor { reading: 0.0 }); // ❌ Compile error
// record_measurement(&CalibratedSensor { reading: 0.0 }); // ✅
```

This connects directly to the **type-state pattern** in Chapter 3.

### Dyn Compatibility (formerly "Object Safety")

Not every trait can be used as `dyn Trait`. A trait is **dyn compatible** only if:

1. **`Sized` must not be a supertrait** — i.e. the trait must not require `Self: Sized`
2. **No generic type parameters** on methods — lifetime parameters *are* allowed,
   since they're erased before codegen and need no extra vtable slot
3. **No use of `Self`** anywhere in a method signature except in the type of the
   receiver — this covers parameters as well as return types
4. **Every associated function must be dispatchable or opted out.** A dispatchable
   method needs a receiver of type `&self`, `&mut self`, `self: Box<Self>`,
   `self: Rc<Self>`, `self: Arc<Self>`, or `self: Pin<P>` where `P` is one of
   those. Anything else — including a bare `fn create() -> Self` — must carry
   `where Self: Sized` to be excluded from the vtable
5. **All supertraits must themselves be dyn compatible** — the property is inherited
6. **No associated constants**, and **no associated types with generics** (GATs).
   Plain associated types are fine, but must be pinned down at the use site:
   `dyn Iterator<Item = u32>`, never a bare `dyn Iterator`
7. **No opaque return types** on dispatchable methods — neither `async fn` (which
   hides a `Future` type) nor return-position `impl Trait`

```rust
// ✅ Dyn compatible — can be used as dyn Drawable
trait Drawable {
    fn draw(&self);
    fn bounding_box(&self) -> (f64, f64, f64, f64);
}

let shapes: Vec<Box<dyn Drawable>> = vec![/* ... */]; // ✅ Works

// ❌ NOT dyn compatible — mentions Self outside the receiver
trait Cloneable {
    fn clone_self(&self) -> Self;
    //                      ^^^^ "...because method `clone_self` references
    //                            the `Self` type in its return type"
}
// let items: Vec<Box<dyn Cloneable>> = ...; // ❌ Compile error

// ❌ Same rule, argument position — this is why PartialEq isn't dyn compatible
trait Comparable {
    fn equals(&self, other: &Self) -> bool;
    //                       ^^^^ "...references the `Self` type in this parameter"
}

// ⚠️ Wrapping in Box does NOT help — Box<Self> still names Self
trait Spawner {
    fn spawn(&self) -> Box<Self>; // ❌ Still not dyn compatible
}

// ✅ Return Box<dyn Trait> instead — that's a concrete type, not Self.
// This is the standard "clone through a trait object" idiom:
trait CloneableDyn {
    fn clone_box(&self) -> Box<dyn CloneableDyn>;
}

// ❌ NOT dyn compatible — generic method
trait Converter {
    fn convert<T>(&self) -> T;
    //        ^^^ The vtable can't contain infinite monomorphizations
}

// ✅ Dyn compatible — a generic LIFETIME is fine, only type params are barred
trait Tokenizer {
    fn first_token<'a>(&self, input: &'a str) -> &'a str;
    //            ^^^^ One vtable slot is enough: lifetimes are erased
}

// ❌ NOT dyn compatible — associated function with no receiver
trait Factory {
    fn create() -> Self;
    // "...because associated function `create` has no `self` parameter"
}

// ✅ Same function, opted out of the vtable — the trait is dyn compatible again
trait FactoryFixed {
    fn describe(&self) -> String;      // dispatchable
    fn create() -> Self where Self: Sized; // excluded from the vtable
}

// ❌ NOT dyn compatible — inherited from a supertrait that isn't.
//    Nothing is wrong with Derived itself; the compiler points at Base::make.
trait Base {
    fn make() -> Self;
}
trait Derived: Base {
    fn show(&self);
}
// let d: &dyn Derived = ...; // ❌ Compile error, blamed on `Base::make`

// ❌ NOT dyn compatible — an associated const has no vtable representation
trait Sensor {
    const MAX: f64;
    //    ^^^ "...because it contains associated const `MAX`"
    fn read(&self) -> f64;
}
// Workaround: make it a method — `fn max_reading(&self) -> f64`. Note there is
// NO `where Self: Sized` escape hatch here; `const MAX: f64 where Self: Sized;`
// isn't even accepted syntax on stable (generic const items are unstable).

// ❌ NOT dyn compatible — a GAT is a family of types, not one type
trait LendingIterator {
    type Item<'a> where Self: 'a;
    //   ^^^^ "...because it contains generic associated type `Item`"
    fn next(&mut self) -> Option<Self::Item<'_>>;
}
// This is the same LendingIterator from the GATs section above: the price of
// a lending iterator is that `dyn LendingIterator` can never exist.

// ❌ NOT dyn compatible — opaque return type (RPITIT)
trait Container {
    fn items(&self) -> impl Iterator<Item = u32>;
    //                 ^^^^ "...references an `impl Trait` type in its return type"
}

// ❌ NOT dyn compatible — `async fn` is the same problem with sugar on top
trait DataStore {
    async fn get(&self, key: &str) -> Option<String>;
    // "...because method `get` is `async`"
}
// Workaround for both: erase the type yourself and return a concrete boxed
// trait object — `Box<dyn Iterator<Item = u32> + '_>` and
// `Pin<Box<dyn Future<Output = Option<String>> + '_>>` are ordinary types,
// so they get ordinary vtable slots.

// ⚠️ `self` by value is NOT a dispatchable receiver — it implies
//    `where Self: Sized`. The trait stays dyn compatible, but the method
//    simply isn't in the vtable and can't be called through `dyn Trait`:
trait Consume {
    fn describe(&self) -> String; // in the vtable
    fn consume(self) -> String;   // implicitly `where Self: Sized`
}
// let t: &dyn Consume = &token;  // ✅ the trait object is fine
// boxed.consume();               // ❌ error[E0161]: cannot move a value
//                                //    of type `dyn Consume`
```

**Workarounds**:

```rust
// Add `where Self: Sized` to exclude a method from the vtable:
trait MyTrait {
    fn regular_method(&self); // Included in vtable

    fn generic_method<T>(&self) -> T
    where
        Self: Sized; // Excluded from vtable — can't be called via dyn MyTrait
}

// Now dyn MyTrait is valid, but generic_method can only be called
// when the concrete type is known.
```

> **Rule of thumb**: If you plan to use `dyn Trait`, keep methods simple — no
> generic type parameters, no `Self` outside the receiver, and no `Sized`
> **supertrait**. Note the asymmetry: `trait Widget: Sized` is fatal, while
> `where Self: Sized` on an individual method is the sanctioned opt-out shown
> above. When in doubt, try `let _: Box<dyn YourTrait>;` and let the compiler
> tell you.

> **Why `AsyncFn` isn't dyn compatible**: the Reference lists `AsyncFn`,
> `AsyncFnMut` and `AsyncFnOnce` as a separate rule, but the compiler shows it's
> really a corollary of the GAT rule above — it blames
> `AsyncFnMut::CallRefFuture<'a>`, a generic associated type in the std
> definition. Plain `Fn`/`FnMut`/`FnOnce` have no such member and are dyn
> compatible, which is why `Box<dyn Fn(u32) -> u32>` works fine.

> **See also**: the RPITIT section below uses `-> impl Trait` in a trait
> definition — convenient, but it costs the trait its dyn compatibility.
> [Async Book — Ch 10](../async-book/ch10-async-traits.html) covers the
> `async fn` half and the `Pin<Box<dyn Future>>` workaround in depth.

### Trait Objects Under the Hood — vtables and Fat Pointers

A `&dyn Trait` (or `Box<dyn Trait>`) is a **fat pointer** — two machine words:

```text
┌──────────────────────────────────────────────────┐
│  &dyn Drawable (on 64-bit: 16 bytes total)       │
├──────────────┬───────────────────────────────────┤
│  data_ptr    │  vtable_ptr                       │
│  (8 bytes)   │  (8 bytes)                        │
│  ↓           │  ↓                                │
│  ┌─────────┐ │  ┌──────────────────────────────┐ │
│  │ Circle  │ │  │ vtable for <Circle as        │ │
│  │ {       │ │  │           Drawable>          │ │
│  │  r: 5.0 │ │  │                              │ │
│  │ }       │ │  │  drop_in_place: 0x7f...a0    │ │
│  └─────────┘ │  │  size:           8           │ │
│              │  │  align:          8           │ │
│              │  │  draw:          0x7f...b4    │ │
│              │  │  bounding_box:  0x7f...c8    │ │
│              │  └──────────────────────────────┘ │
└──────────────┴───────────────────────────────────┘
```

**How a vtable call works** (e.g., `shape.draw()`):

1. Load `vtable_ptr` from the fat pointer (second word)
2. Index into the vtable to find the `draw` function pointer
3. Call it, passing `data_ptr` as the `self` argument

This is similar to C++ virtual dispatch in cost (one pointer indirection
per call), but Rust stores the vtable pointer in the fat pointer rather
than inside the object — so a plain `Circle` on the stack carries no
vtable pointer at all.

```rust
trait Drawable {
    fn draw(&self);
    fn area(&self) -> f64;
}

struct Circle { radius: f64 }

impl Drawable for Circle {
    fn draw(&self) { println!("Drawing circle r={}", self.radius); }
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
}

struct Square { side: f64 }

impl Drawable for Square {
    fn draw(&self) { println!("Drawing square s={}", self.side); }
    fn area(&self) -> f64 { self.side * self.side }
}

fn main() {
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Square { side: 3.0 }),
    ];

    // Each element is a fat pointer: (data_ptr, vtable_ptr)
    // The vtable for Circle and Square are DIFFERENT
    for shape in &shapes {
        shape.draw();  // vtable dispatch → Circle::draw or Square::draw
        println!("  area = {:.2}", shape.area());
    }

    // Size comparison:
    println!("size_of::<&Circle>()        = {}", size_of::<&Circle>());
    // → 8 bytes (one pointer — the compiler knows the type)
    println!("size_of::<&dyn Drawable>()  = {}", size_of::<&dyn Drawable>());
    // → 16 bytes (data_ptr + vtable_ptr)
}
```

**Performance cost model**:

| Aspect | Static dispatch (`impl Trait` / generics) | Dynamic dispatch (`dyn Trait`) |
|--------|------------------------------------------|-------------------------------|
| Call overhead | Zero — inlined by LLVM | One pointer indirection per call |
| Inlining | ✅ Compiler can inline | ❌ Opaque function pointer |
| Binary size | Larger (one copy per type) | Smaller (one shared function) |
| Pointer size | Thin (1 word) | Fat (2 words) |
| Heterogeneous collections | ❌ | ✅ `Vec<Box<dyn Trait>>` |

> **When vtable cost matters**: In tight loops calling a trait method millions
> of times, the indirection and inability to inline can be significant (2-10×
> slower). For cold paths, configuration, or plugin architectures, the
> flexibility of `dyn Trait` is worth the small cost.

### Higher-Ranked Trait Bounds (HRTBs)

Sometimes you need a function that works with references of *any* lifetime, not a specific one. This is where `for<'a>` syntax appears:

```rust
// Problem: this function needs a closure that can process
// references with ANY lifetime, not just one specific lifetime.

// ❌ This is too restrictive — 'a is fixed by the caller:
// fn apply<'a, F: Fn(&'a str) -> &'a str>(f: F, data: &'a str) -> &'a str

// ✅ HRTB: F must work for ALL possible lifetimes:
fn apply<F>(f: F, data: &str) -> &str
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    f(data)
}

fn main() {
    let result = apply(|s| s.trim(), "  hello  ");
    println!("{result}"); // "hello"
}
```

**When you encounter HRTBs**:
- `Fn(&T) -> &U` traits — the compiler infers `for<'a>` automatically in most cases
- Custom trait implementations that must work across different borrows
- Deserialization with `serde`: `for<'de> Deserialize<'de>`

```rust,ignore
// serde's DeserializeOwned is defined as:
// trait DeserializeOwned: for<'de> Deserialize<'de> {}
// Meaning: "can be deserialized from data with ANY lifetime"
// (i.e., the result doesn't borrow from the input)

use serde::de::DeserializeOwned;

fn parse_json<T: DeserializeOwned>(input: &str) -> T {
    serde_json::from_str(input).unwrap()
}
```

> **Practical advice**: You'll rarely write `for<'a>` yourself. It mostly appears
> in trait bounds on closure parameters, where the compiler handles it implicitly.
> But recognizing it in error messages ("expected a `for<'a> Fn(&'a ...)` bound")
> helps you understand what the compiler is asking for.

### `impl Trait` — Argument Position vs Return Position

`impl Trait` appears in two positions with **different semantics**:

```rust
// --- Argument-Position impl Trait (APIT) ---
// "Caller chooses the type" — syntactic sugar for a generic parameter
fn print_all(items: impl Iterator<Item = i32>) {
    for item in items { println!("{item}"); }
}
// Equivalent to:
fn print_all_verbose<I: Iterator<Item = i32>>(items: I) {
    for item in items { println!("{item}"); }
}
// Caller decides: print_all(vec![1,2,3].into_iter())
//                 print_all(0..10)

// --- Return-Position impl Trait (RPIT) ---
// "Callee chooses the type" — the function picks one concrete type
fn evens(limit: i32) -> impl Iterator<Item = i32> {
    (0..limit).filter(|x| x % 2 == 0)
    // The concrete type is Filter<Range<i32>, Closure>
    // but the caller only sees "some Iterator<Item = i32>"
}
```

**Key difference**:

| | APIT (`fn foo(x: impl T)`) | RPIT (`fn foo() -> impl T`) |
|---|---|---|
| Who picks the type? | Caller | Callee (function body) |
| Monomorphized? | Yes — one copy per type | Yes — one concrete type |
| Turbofish? | No (`foo::<X>()` not allowed) | N/A |
| Equivalent to | `fn foo<X: T>(x: X)` | Existential type |

#### RPIT in Trait Definitions (RPITIT)

Since Rust 1.75, you can use `-> impl Trait` directly in trait definitions:

```rust
trait Container {
    fn items(&self) -> impl Iterator<Item = &str>;
    //                 ^^^^ Each implementor returns its own concrete type
}

struct CsvRow {
    fields: Vec<String>,
}

impl Container for CsvRow {
    fn items(&self) -> impl Iterator<Item = &str> {
        self.fields.iter().map(String::as_str)
    }
}

struct FixedFields;

impl Container for FixedFields {
    fn items(&self) -> impl Iterator<Item = &str> {
        ["host", "port", "timeout"].into_iter()
    }
}
```

> **Before Rust 1.75**, you had to use `Box<dyn Iterator>` or an associated
> type to achieve this in traits. RPITIT removes the allocation.
>
> **But it costs dyn compatibility**: `-> impl Trait` is an opaque return type,
> so `dyn Container` is rejected (see [Dyn Compatibility](#dyn-compatibility-formerly-object-safety),
> rule 7). If you need the trait object, keep returning
> `Box<dyn Iterator<Item = &str> + '_>` and pay the allocation.

#### `impl Trait` vs `dyn Trait` — Decision Guide

```text
Do you know the concrete type at compile time?
├── YES → Use impl Trait or generics (zero cost, inlinable)
└── NO  → Do you need a heterogeneous collection?
     ├── YES → Use dyn Trait (Box<dyn T>, &dyn T)
     └── NO  → Do you need the SAME trait object across an API boundary?
          ├── YES → Use dyn Trait
          └── NO  → Use generics / impl Trait
```

| Feature | `impl Trait` | `dyn Trait` |
|---------|-------------|------------|
| Dispatch | Static (monomorphized) | Dynamic (vtable) |
| Performance | Best — inlinable | One indirection per call |
| Heterogeneous collections | ❌ | ✅ |
| Binary size per type | One copy each | Shared code |
| Trait must be dyn compatible? | No | Yes |
| Works in trait definitions | ✅ (Rust 1.75+) | Always |

***

## Type Erasure with `Any` and `TypeId`

Sometimes you need to store values of *unknown* types and downcast them later — a pattern
familiar from `void*` in C or `object` in C#. Rust provides this through `std::any::Any`:

```rust
use std::any::Any;

// Store heterogeneous values:
fn log_value(value: &dyn Any) {
    if let Some(s) = value.downcast_ref::<String>() {
        println!("String: {s}");
    } else if let Some(n) = value.downcast_ref::<i32>() {
        println!("i32: {n}");
    } else {
        // TypeId lets you inspect the type at runtime:
        println!("Unknown type: {:?}", value.type_id());
    }
}

// Useful for plugin systems, event buses, or ECS-style architectures:
struct AnyMap(std::collections::HashMap<std::any::TypeId, Box<dyn Any + Send>>);

impl AnyMap {
    fn new() -> Self { AnyMap(std::collections::HashMap::new()) }

    fn insert<T: Any + Send + 'static>(&mut self, value: T) {
        self.0.insert(std::any::TypeId::of::<T>(), Box::new(value));
    }

    fn get<T: Any + Send + 'static>(&self) -> Option<&T> {
        self.0.get(&std::any::TypeId::of::<T>())?
            .downcast_ref()
    }
}

fn main() {
    let mut map = AnyMap::new();
    map.insert(42_i32);
    map.insert(String::from("hello"));

    assert_eq!(map.get::<i32>(), Some(&42));
    assert_eq!(map.get::<String>().map(|s| s.as_str()), Some("hello"));
    assert_eq!(map.get::<f64>(), None); // Never inserted
}
```

> **When to use `Any`**: Plugin/extension systems, type-indexed maps (`typemap`),
> error downcasting (`anyhow::Error::downcast_ref`). Prefer generics or trait
> objects when the set of types is known at compile time — `Any` is a last resort
> that trades compile-time safety for flexibility.

***

## Extension Traits — Adding Methods to Types You Don't Own

Rust's orphan rule prevents you from implementing a foreign trait on a foreign type.
Extension traits are the standard workaround: define a **new trait** in your crate whose
methods have a blanket implementation for any type that meets a bound. The caller imports
the trait and the new methods appear on existing types.

This pattern is pervasive in the Rust ecosystem: `itertools::Itertools`, `futures::StreamExt`,
`tokio::io::AsyncReadExt`, `tower::ServiceExt`.

### The Problem

```rust
// We want to add a .mean() method to all iterators that yield f64.
// But Iterator is defined in std and f64 is a primitive — orphan rule prevents:
//
// impl<I: Iterator<Item = f64>> I {   // ❌ Cannot add inherent methods to a foreign type
//     fn mean(self) -> f64 { ... }
// }
```

### The Solution: An Extension Trait

```rust
/// Extension methods for iterators over numeric values.
pub trait IteratorExt: Iterator {
    /// Computes the arithmetic mean. Returns `None` for empty iterators.
    fn mean(self) -> Option<f64>
    where
        Self: Sized,
        Self::Item: Into<f64>;
}

// Blanket implementation — automatically applies to ALL iterators
impl<I: Iterator> IteratorExt for I {
    fn mean(self) -> Option<f64>
    where
        Self: Sized,
        Self::Item: Into<f64>,
    {
        let mut sum: f64 = 0.0;
        let mut count: u64 = 0;
        for item in self {
            sum += item.into();
            count += 1;
        }
        if count == 0 { None } else { Some(sum / count as f64) }
    }
}

// Usage — just import the trait:
use crate::IteratorExt;  // One import and the method appears on all iterators

fn analyze_temperatures(readings: &[f64]) -> Option<f64> {
    readings.iter().copied().mean()  // .mean() is now available!
}

fn analyze_sensor_data(data: &[i32]) -> Option<f64> {
    data.iter().copied().mean()  // Works on i32 too (i32: Into<f64>)
}
```

### Real-World Example: Diagnostic Result Extensions

```rust
use std::collections::HashMap;

struct DiagResult {
    component: String,
    passed: bool,
    message: String,
}

/// Extension trait for Vec<DiagResult> — adds domain-specific analysis methods.
pub trait DiagResultsExt {
    fn passed_count(&self) -> usize;
    fn failed_count(&self) -> usize;
    fn overall_pass(&self) -> bool;
    fn failures_by_component(&self) -> HashMap<String, Vec<&DiagResult>>;
}

impl DiagResultsExt for Vec<DiagResult> {
    fn passed_count(&self) -> usize {
        self.iter().filter(|r| r.passed).count()
    }

    fn failed_count(&self) -> usize {
        self.iter().filter(|r| !r.passed).count()
    }

    fn overall_pass(&self) -> bool {
        self.iter().all(|r| r.passed)
    }

    fn failures_by_component(&self) -> HashMap<String, Vec<&DiagResult>> {
        let mut map = HashMap::new();
        for r in self.iter().filter(|r| !r.passed) {
            map.entry(r.component.clone()).or_default().push(r);
        }
        map
    }
}

// Now any Vec<DiagResult> has these methods:
fn report(results: Vec<DiagResult>) {
    if !results.overall_pass() {
        let failures = results.failures_by_component();
        for (component, fails) in &failures {
            eprintln!("{component}: {} failures", fails.len());
        }
    }
}
```

### Naming Convention

The Rust ecosystem uses a consistent `Ext` suffix:

| Crate | Extension Trait | Extends |
|-------|----------------|---------|
| `itertools` | `Itertools` | `Iterator` |
| `futures` | `StreamExt`, `FutureExt` | `Stream`, `Future` |
| `tokio` | `AsyncReadExt`, `AsyncWriteExt` | `AsyncRead`, `AsyncWrite` |
| `tower` | `ServiceExt` | `Service` |
| `bytes` | `BufMut` (partial) | `&mut [u8]` |
| Your crate | `DiagResultsExt` | `Vec<DiagResult>` |

### When to Use

| Situation | Use Extension Trait? |
|-----------|:---:|
| Adding convenience methods to a foreign type | ✅ |
| Grouping domain-specific logic on generic collections | ✅ |
| The method needs access to private fields | ❌ (use a wrapper/newtype) |
| The method logically belongs on a new type you control | ❌ (just add it to your type) |
| You want the method available without any import | ❌ (inherent methods only) |

***

## Enum Dispatch — Static Polymorphism Without `dyn`

When you have a **closed set** of types implementing a trait, you can replace `dyn Trait`
with an enum whose variants hold the concrete types. This eliminates the vtable indirection
and heap allocation while preserving the same caller-facing interface.

### The Problem with `dyn Trait`

```rust
trait Sensor {
    fn read(&self) -> f64;
    fn name(&self) -> &str;
}

struct Gps { lat: f64, lon: f64 }
struct Thermometer { temp_c: f64 }
struct Accelerometer { g_force: f64 }

impl Sensor for Gps {
    fn read(&self) -> f64 { self.lat }
    fn name(&self) -> &str { "GPS" }
}
impl Sensor for Thermometer {
    fn read(&self) -> f64 { self.temp_c }
    fn name(&self) -> &str { "Thermometer" }
}
impl Sensor for Accelerometer {
    fn read(&self) -> f64 { self.g_force }
    fn name(&self) -> &str { "Accelerometer" }
}

// Heterogeneous collection with dyn — works, but has costs:
fn read_all_dyn(sensors: &[Box<dyn Sensor>]) -> Vec<f64> {
    sensors.iter().map(|s| s.read()).collect()
    // Each .read() goes through a vtable indirection
    // Each Box allocates on the heap
}
```

### The Enum Dispatch Solution

```rust
// Replace the trait object with an enum:
enum AnySensor {
    Gps(Gps),
    Thermometer(Thermometer),
    Accelerometer(Accelerometer),
}

impl AnySensor {
    fn read(&self) -> f64 {
        match self {
            AnySensor::Gps(s) => s.read(),
            AnySensor::Thermometer(s) => s.read(),
            AnySensor::Accelerometer(s) => s.read(),
        }
    }

    fn name(&self) -> &str {
        match self {
            AnySensor::Gps(s) => s.name(),
            AnySensor::Thermometer(s) => s.name(),
            AnySensor::Accelerometer(s) => s.name(),
        }
    }
}

// Now: no heap allocation, no vtable, stored inline
fn read_all(sensors: &[AnySensor]) -> Vec<f64> {
    sensors.iter().map(|s| s.read()).collect()
    // Each .read() is a match branch — compiler can inline everything
}

fn main() {
    let sensors = vec![
        AnySensor::Gps(Gps { lat: 47.6, lon: -122.3 }),
        AnySensor::Thermometer(Thermometer { temp_c: 72.5 }),
        AnySensor::Accelerometer(Accelerometer { g_force: 1.02 }),
    ];

    for sensor in &sensors {
        println!("{}: {:.2}", sensor.name(), sensor.read());
    }
}
```

### Implement the Trait on the Enum

For interoperability, you can implement the original trait on the enum itself:

```rust
impl Sensor for AnySensor {
    fn read(&self) -> f64 {
        match self {
            AnySensor::Gps(s) => s.read(),
            AnySensor::Thermometer(s) => s.read(),
            AnySensor::Accelerometer(s) => s.read(),
        }
    }

    fn name(&self) -> &str {
        match self {
            AnySensor::Gps(s) => s.name(),
            AnySensor::Thermometer(s) => s.name(),
            AnySensor::Accelerometer(s) => s.name(),
        }
    }
}

// Now AnySensor works anywhere a Sensor is expected via generics:
fn report<S: Sensor>(s: &S) {
    println!("{}: {:.2}", s.name(), s.read());
}
```

### Reducing Boilerplate with a Macro

The match-arm delegation is repetitive. A macro eliminates it:

```rust
macro_rules! dispatch_sensor {
    ($self:expr, $method:ident $(, $arg:expr)*) => {
        match $self {
            AnySensor::Gps(s) => s.$method($($arg),*),
            AnySensor::Thermometer(s) => s.$method($($arg),*),
            AnySensor::Accelerometer(s) => s.$method($($arg),*),
        }
    };
}

impl Sensor for AnySensor {
    fn read(&self) -> f64     { dispatch_sensor!(self, read) }
    fn name(&self) -> &str    { dispatch_sensor!(self, name) }
}
```

For larger projects, the `enum_dispatch` crate automates this entirely:

```rust
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
trait Sensor {
    fn read(&self) -> f64;
    fn name(&self) -> &str;
}

#[enum_dispatch(Sensor)]
enum AnySensor {
    Gps,
    Thermometer,
    Accelerometer,
}
// All delegation code is generated automatically.
```

### `dyn Trait` vs Enum Dispatch — Decision Guide

```text
Is the set of types closed (known at compile time)?
├── YES → Prefer enum dispatch (faster, no heap allocation)
│         ├── Few variants (< ~20)?     → Manual enum
│         └── Many variants or growing? → enum_dispatch crate
└── NO  → Must use dyn Trait (plugins, user-provided types)
```

| Property | `dyn Trait` | Enum Dispatch |
|----------|:-----------:|:-------------:|
| Dispatch cost | Vtable indirection (~2ns) | Branch prediction (~0.3ns) |
| Heap allocation | Usually (Box) | None (inline) |
| Cache-friendly | No (pointer chasing) | Yes (contiguous) |
| Open to new types | ✅ (anyone can impl) | ❌ (closed set) |
| Code size | Shared | One copy per variant |
| Trait must be dyn compatible | Yes | No |
| Adding a variant | No code changes | Update enum + match arms |

### When to Use Enum Dispatch

| Scenario | Recommendation |
|----------|---------------|
| Diagnostic test types (CPU, GPU, NIC, Memory, ...) | ✅ Enum dispatch — closed set, known at compile time |
| Bus protocols (SPI, I2C, UART, ...) | ✅ Enum dispatch or Config trait |
| Plugin system (user loads .so at runtime) | ❌ Use `dyn Trait` |
| 2-3 variants | ✅ Manual enum dispatch |
| 10+ variants with many methods | ✅ `enum_dispatch` crate |
| Performance-critical inner loop | ✅ Enum dispatch (eliminates vtable) |

***

## Capability Mixins — Associated Types as Zero-Cost Composition

Ruby developers compose behaviour with **mixins** — `include SomeModule` injects methods
into a class.  Rust traits with **associated types + default methods + blanket impls**
produce the same result, except:

* Everything resolves at **compile time** — no method-missing surprises
* Each associated type is a **knob** that changes what the default methods produce
* The compiler **monomorphises** each combination — zero vtable overhead

### The Problem: Cross-Cutting Bus Dependencies

Hardware diagnostic routines share common operations — read an IPMI sensor, toggle a
GPIO rail, sample a temperature over SPI — but different diagnostics need different
combinations.  Inheritance hierarchies don't exist in Rust.  Passing every bus handle
as a function argument creates unwieldy signatures.  We need a way to **mix in** bus
capabilities à la carte.

### Step 1 — Define "Ingredient" Traits

Each ingredient provides one hardware capability via an associated type:

```rust
use std::io;

// ── Bus abstractions (traits the hardware team provides) ──────────
pub trait SpiBus {
    fn spi_transfer(&self, tx: &[u8], rx: &mut [u8]) -> io::Result<()>;
}

pub trait I2cBus {
    fn i2c_read(&self, addr: u8, reg: u8, buf: &mut [u8]) -> io::Result<()>;
    fn i2c_write(&self, addr: u8, reg: u8, data: &[u8]) -> io::Result<()>;
}

pub trait GpioPin {
    fn set_high(&self) -> io::Result<()>;
    fn set_low(&self) -> io::Result<()>;
    fn read_level(&self) -> io::Result<bool>;
}

pub trait IpmiBmc {
    fn raw_command(&self, net_fn: u8, cmd: u8, data: &[u8]) -> io::Result<Vec<u8>>;
    fn read_sensor(&self, sensor_id: u8) -> io::Result<f64>;
}

// ── Ingredient traits — one per bus, carries an associated type ───
pub trait HasSpi {
    type Spi: SpiBus;
    fn spi(&self) -> &Self::Spi;
}

pub trait HasI2c {
    type I2c: I2cBus;
    fn i2c(&self) -> &Self::I2c;
}

pub trait HasGpio {
    type Gpio: GpioPin;
    fn gpio(&self) -> &Self::Gpio;
}

pub trait HasIpmi {
    type Ipmi: IpmiBmc;
    fn ipmi(&self) -> &Self::Ipmi;
}
```

Each ingredient is tiny, generic, and testable in isolation.

### Step 2 — Define "Mixin" Traits

A mixin trait declares its required ingredients as supertraits, then provides all
its methods via **defaults** — implementors get them for free:

```rust
/// Mixin: fan diagnostics — needs I2C (tachometer) + GPIO (PWM enable)
pub trait FanDiagMixin: HasI2c + HasGpio {
    /// Read fan RPM from the tachometer IC over I2C.
    fn read_fan_rpm(&self, fan_id: u8) -> io::Result<u32> {
        let mut buf = [0u8; 2];
        self.i2c().i2c_read(0x48 + fan_id, 0x00, &mut buf)?;
        Ok(u16::from_be_bytes(buf) as u32 * 60) // tach counts → RPM
    }

    /// Enable or disable the fan PWM output via GPIO.
    fn set_fan_pwm(&self, enable: bool) -> io::Result<()> {
        if enable { self.gpio().set_high() }
        else      { self.gpio().set_low() }
    }

    /// Full fan health check — read RPM + verify within threshold.
    fn check_fan_health(&self, fan_id: u8, min_rpm: u32) -> io::Result<bool> {
        let rpm = self.read_fan_rpm(fan_id)?;
        Ok(rpm >= min_rpm)
    }
}

/// Mixin: temperature monitoring — needs SPI (thermocouple ADC) + IPMI (BMC sensors)
pub trait TempMonitorMixin: HasSpi + HasIpmi {
    /// Read a thermocouple via the SPI ADC (e.g. MAX31855).
    fn read_thermocouple(&self) -> io::Result<f64> {
        let mut rx = [0u8; 4];
        self.spi().spi_transfer(&[0x00; 4], &mut rx)?;
        let raw = i32::from_be_bytes(rx) >> 18; // 14-bit signed
        Ok(raw as f64 * 0.25)
    }

    /// Read a BMC-managed temperature sensor via IPMI.
    fn read_bmc_temp(&self, sensor_id: u8) -> io::Result<f64> {
        self.ipmi().read_sensor(sensor_id)
    }

    /// Cross-validate: thermocouple vs BMC must agree within delta.
    fn validate_temps(&self, sensor_id: u8, max_delta: f64) -> io::Result<bool> {
        let tc = self.read_thermocouple()?;
        let bmc = self.read_bmc_temp(sensor_id)?;
        Ok((tc - bmc).abs() <= max_delta)
    }
}

/// Mixin: power sequencing — needs GPIO (rail enable) + IPMI (event logging)
pub trait PowerSeqMixin: HasGpio + HasIpmi {
    /// Assert the power-good GPIO and verify via IPMI sensor.
    fn enable_power_rail(&self, sensor_id: u8) -> io::Result<bool> {
        self.gpio().set_high()?;
        std::thread::sleep(std::time::Duration::from_millis(50));
        let voltage = self.ipmi().read_sensor(sensor_id)?;
        Ok(voltage > 0.8) // above 80% nominal = good
    }

    /// De-assert power and log shutdown via IPMI OEM command.
    fn disable_power_rail(&self) -> io::Result<()> {
        self.gpio().set_low()?;
        // Log OEM "power rail disabled" event to BMC
        self.ipmi().raw_command(0x2E, 0x01, &[0x00, 0x01])?;
        Ok(())
    }
}
```

### Step 3 — Blanket Impls Make It Truly "Mixin"

The magic line — provide the ingredients, get the methods:

```rust
impl<T: HasI2c + HasGpio>  FanDiagMixin    for T {}
impl<T: HasSpi  + HasIpmi>  TempMonitorMixin for T {}
impl<T: HasGpio + HasIpmi>  PowerSeqMixin   for T {}
```

Any struct that implements the right ingredient traits **automatically** gains every
mixin method — no boilerplate, no forwarding, no inheritance.

### Step 4 — Wire Up Production

```rust
// ── Concrete bus implementations (Linux platform) ────────────────
struct LinuxSpi  { dev: String }
struct LinuxI2c  { dev: String }
struct SysfsGpio { pin: u32 }
struct IpmiTool  { timeout_secs: u32 }

impl SpiBus for LinuxSpi {
    fn spi_transfer(&self, _tx: &[u8], _rx: &mut [u8]) -> io::Result<()> {
        // spidev ioctl — omitted for brevity
        Ok(())
    }
}
impl I2cBus for LinuxI2c {
    fn i2c_read(&self, _addr: u8, _reg: u8, _buf: &mut [u8]) -> io::Result<()> {
        // i2c-dev ioctl — omitted for brevity
        Ok(())
    }
    fn i2c_write(&self, _addr: u8, _reg: u8, _data: &[u8]) -> io::Result<()> { Ok(()) }
}
impl GpioPin for SysfsGpio {
    fn set_high(&self) -> io::Result<()>  { /* /sys/class/gpio */ Ok(()) }
    fn set_low(&self) -> io::Result<()>   { Ok(()) }
    fn read_level(&self) -> io::Result<bool> { Ok(true) }
}
impl IpmiBmc for IpmiTool {
    fn raw_command(&self, _nf: u8, _cmd: u8, _data: &[u8]) -> io::Result<Vec<u8>> {
        // shells out to ipmitool — omitted for brevity
        Ok(vec![])
    }
    fn read_sensor(&self, _id: u8) -> io::Result<f64> { Ok(25.0) }
}

// ── Production platform — all four buses ─────────────────────────
struct DiagPlatform {
    spi:  LinuxSpi,
    i2c:  LinuxI2c,
    gpio: SysfsGpio,
    ipmi: IpmiTool,
}

impl HasSpi  for DiagPlatform { type Spi  = LinuxSpi;  fn spi(&self)  -> &LinuxSpi  { &self.spi  } }
impl HasI2c  for DiagPlatform { type I2c  = LinuxI2c;  fn i2c(&self)  -> &LinuxI2c  { &self.i2c  } }
impl HasGpio for DiagPlatform { type Gpio = SysfsGpio; fn gpio(&self) -> &SysfsGpio { &self.gpio } }
impl HasIpmi for DiagPlatform { type Ipmi = IpmiTool;  fn ipmi(&self) -> &IpmiTool  { &self.ipmi } }

// DiagPlatform now has ALL mixin methods:
fn production_diagnostics(platform: &DiagPlatform) -> io::Result<()> {
    let rpm = platform.read_fan_rpm(0)?;       // from FanDiagMixin
    let tc  = platform.read_thermocouple()?;   // from TempMonitorMixin
    let ok  = platform.enable_power_rail(42)?;  // from PowerSeqMixin
    println!("Fan: {rpm} RPM, Temp: {tc}°C, Power: {ok}");
    Ok(())
}
```

### Step 5 — Test With Mocks (No Hardware Required)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    struct MockSpi  { temp: Cell<f64> }
    struct MockI2c  { rpm: Cell<u32> }
    struct MockGpio { level: Cell<bool> }
    struct MockIpmi { sensor_val: Cell<f64> }

    impl SpiBus for MockSpi {
        fn spi_transfer(&self, _tx: &[u8], rx: &mut [u8]) -> io::Result<()> {
            // Encode mock temp as MAX31855 format
            let raw = ((self.temp.get() / 0.25) as i32) << 18;
            rx.copy_from_slice(&raw.to_be_bytes());
            Ok(())
        }
    }
    impl I2cBus for MockI2c {
        fn i2c_read(&self, _addr: u8, _reg: u8, buf: &mut [u8]) -> io::Result<()> {
            let tach = (self.rpm.get() / 60) as u16;
            buf.copy_from_slice(&tach.to_be_bytes());
            Ok(())
        }
        fn i2c_write(&self, _: u8, _: u8, _: &[u8]) -> io::Result<()> { Ok(()) }
    }
    impl GpioPin for MockGpio {
        fn set_high(&self)  -> io::Result<()>   { self.level.set(true);  Ok(()) }
        fn set_low(&self)   -> io::Result<()>   { self.level.set(false); Ok(()) }
        fn read_level(&self) -> io::Result<bool> { Ok(self.level.get()) }
    }
    impl IpmiBmc for MockIpmi {
        fn raw_command(&self, _: u8, _: u8, _: &[u8]) -> io::Result<Vec<u8>> { Ok(vec![]) }
        fn read_sensor(&self, _: u8) -> io::Result<f64> { Ok(self.sensor_val.get()) }
    }

    // ── Partial platform: only fan-related buses ─────────────────
    struct FanTestRig {
        i2c:  MockI2c,
        gpio: MockGpio,
    }
    impl HasI2c  for FanTestRig { type I2c  = MockI2c;  fn i2c(&self)  -> &MockI2c  { &self.i2c  } }
    impl HasGpio for FanTestRig { type Gpio = MockGpio; fn gpio(&self) -> &MockGpio { &self.gpio } }
    // FanTestRig gets FanDiagMixin but NOT TempMonitorMixin or PowerSeqMixin

    #[test]
    fn fan_health_check_passes_above_threshold() {
        let rig = FanTestRig {
            i2c:  MockI2c  { rpm: Cell::new(6000) },
            gpio: MockGpio { level: Cell::new(false) },
        };
        assert!(rig.check_fan_health(0, 4000).unwrap());
    }

    #[test]
    fn fan_health_check_fails_below_threshold() {
        let rig = FanTestRig {
            i2c:  MockI2c  { rpm: Cell::new(2000) },
            gpio: MockGpio { level: Cell::new(false) },
        };
        assert!(!rig.check_fan_health(0, 4000).unwrap());
    }
}
```

Notice that `FanTestRig` only implements `HasI2c + HasGpio` — it gets `FanDiagMixin`
automatically, but the compiler **refuses** `rig.read_thermocouple()` because `HasSpi`
is not satisfied.  This is mixin scoping enforced at compile time.

### Conditional Methods — Beyond What Ruby Can Do

Add `where` bounds to individual default methods.  The method only **exists** when
the associated type satisfies the extra bound:

```rust
/// Marker trait for DMA-capable SPI controllers
pub trait DmaCapable: SpiBus {
    fn dma_transfer(&self, tx: &[u8], rx: &mut [u8]) -> io::Result<()>;
}

/// Marker trait for interrupt-capable GPIO pins
pub trait InterruptCapable: GpioPin {
    fn wait_for_edge(&self, timeout_ms: u32) -> io::Result<bool>;
}

pub trait AdvancedDiagMixin: HasSpi + HasGpio {
    // Always available
    fn basic_probe(&self) -> io::Result<bool> {
        let mut rx = [0u8; 1];
        self.spi().spi_transfer(&[0xFF], &mut rx)?;
        Ok(rx[0] != 0x00)
    }

    // Only exists when the SPI controller supports DMA
    fn bulk_sensor_read(&self, buf: &mut [u8]) -> io::Result<()>
    where
        Self::Spi: DmaCapable,
    {
        self.spi().dma_transfer(&vec![0x00; buf.len()], buf)
    }

    // Only exists when the GPIO pin supports interrupts
    fn wait_for_fault_signal(&self, timeout_ms: u32) -> io::Result<bool>
    where
        Self::Gpio: InterruptCapable,
    {
        self.gpio().wait_for_edge(timeout_ms)
    }
}

impl<T: HasSpi + HasGpio> AdvancedDiagMixin for T {}
```

If your platform's SPI doesn't support DMA, calling `bulk_sensor_read()` is a
**compile error**, not a runtime crash.  Ruby's `respond_to?` check is the closest
equivalent — but it happens at deploy time, not compile time.

### Composability: Stacking Mixins

Multiple mixins can share the same ingredient — no diamond problem:

```text
┌─────────────┐    ┌───────────┐    ┌──────────────┐
│ FanDiagMixin│    │TempMonitor│    │ PowerSeqMixin│
│  (I2C+GPIO) │    │ (SPI+IPMI)│    │  (GPIO+IPMI) │
└──────┬──────┘    └─────┬─────┘    └──────┬───────┘
       │                 │                 │
       │   ┌─────────────┴─────────────┐   │
       └──►│      DiagPlatform         │◄──┘
           │ HasSpi+HasI2c+HasGpio     │
           │        +HasIpmi           │
           └───────────────────────────┘
```

`DiagPlatform` implements `HasGpio` **once**, and both `FanDiagMixin` and
`PowerSeqMixin` use the same `self.gpio()`.  In Ruby, this would be two modules
both calling `self.gpio_pin` — but if they expected different pin numbers, you'd
discover the conflict at runtime.  In Rust, you can disambiguate at the type level.

### Comparison: Ruby Mixins vs Rust Capability Mixins

| Dimension | Ruby Mixins | Rust Capability Mixins |
|-----------|-------------|------------------------|
| Dispatch | Runtime (method table lookup) | Compile-time (monomorphised) |
| Safe composition | MRO linearisation hides conflicts | Compiler rejects ambiguity |
| Conditional methods | `respond_to?` at runtime | `where` bounds at compile time |
| Overhead | Method dispatch + GC | Zero-cost (inlined) |
| Testability | Stub/mock via metaprogramming | Generic over mock types |
| Adding new buses | `include` at runtime | Add ingredient trait, recompile |
| Runtime flexibility | `extend`, `prepend`, open classes | None (fully static) |

### When to Use Capability Mixins

| Scenario | Use Mixins? |
|----------|:-----------:|
| Multiple diagnostics share bus-reading logic | ✅ |
| Test harness needs different bus subsets | ✅ (partial ingredient structs) |
| Methods only valid for certain bus capabilities (DMA, IRQ) | ✅ (conditional `where` bounds) |
| You need runtime module loading (plugins) | ❌ (use `dyn Trait` or enum dispatch) |
| Single struct with one bus — no sharing needed | ❌ (keep it simple) |
| Cross-crate ingredients with coherence issues | ⚠️ (use newtype wrappers) |

> **Key Takeaways — Capability Mixins**
>
> 1. **Ingredient trait** = associated type + accessor method (e.g., `HasSpi`)
> 2. **Mixin trait** = supertrait bounds on ingredients + default method bodies
> 3. **Blanket impl** = `impl<T: HasX + HasY> Mixin for T {}` — auto-injects methods
> 4. **Conditional methods** = `where Self::Spi: DmaCapable` on individual defaults
> 5. **Partial platforms** = test structs that only impl the needed ingredients
> 6. **No runtime cost** — the compiler generates specialised code for each platform type

***

## Typed Commands — GADT-Style Return Type Safety

In Haskell, **Generalised Algebraic Data Types (GADTs)** let each constructor of a
data type refine the type parameter — so `Expr Int` and `Expr Bool` are enforced by
the type checker.  Rust has no direct GADT syntax, but **traits with associated types**
achieve the same guarantee: the command type **determines** the response type, and
mixing them up is a compile error.

This pattern is particularly powerful for hardware diagnostics, where IPMI commands,
register reads, and sensor queries each return different physical quantities that
should never be confused.

### The Problem: The Untyped `Vec<u8>` Swamp

Most C/C++ IPMI stacks — and naïve Rust ports — use raw bytes everywhere:

```rust
use std::io;

struct BmcConnectionUntyped { timeout_secs: u32 }

impl BmcConnectionUntyped {
    fn raw_command(&self, net_fn: u8, cmd: u8, data: &[u8]) -> io::Result<Vec<u8>> {
        // ... shells out to ipmitool ...
        Ok(vec![0x00, 0x19, 0x00]) // stub
    }
}

fn diagnose_thermal_untyped(bmc: &BmcConnectionUntyped) -> io::Result<()> {
    // Read CPU temperature — sensor ID 0x20
    let raw = bmc.raw_command(0x04, 0x2D, &[0x20])?;
    let cpu_temp = raw[0] as f64;  // 🤞 hope byte 0 is the reading

    // Read fan speed — sensor ID 0x30
    let raw = bmc.raw_command(0x04, 0x2D, &[0x30])?;
    let fan_rpm = raw[0] as u32;  // 🐛 BUG: fan speed is 2 bytes LE

    // Read inlet voltage — sensor ID 0x40
    let raw = bmc.raw_command(0x04, 0x2D, &[0x40])?;
    let voltage = raw[0] as f64;  // 🐛 BUG: need to divide by 1000

    // 🐛 Comparing °C to RPM — compiles, but nonsensical
    if cpu_temp > fan_rpm as f64 {
        println!("uh oh");
    }

    // 🐛 Passing Volts as temperature — compiles fine
    log_temp_untyped(voltage);
    log_volts_untyped(cpu_temp);

    Ok(())
}

fn log_temp_untyped(t: f64)  { println!("Temp: {t}°C"); }
fn log_volts_untyped(v: f64) { println!("Voltage: {v}V"); }
```

**Every reading is `f64`** — the compiler has no idea that one is a temperature, another
is RPM, another is voltage.  Four distinct bugs compile without warning:

| # | Bug | Consequence | Discovered |
|---|-----|-------------|------------|
| 1 | Fan RPM parsed as 1 byte instead of 2 | Reads 25 RPM instead of 6400 | Production, 3 AM fan-failure flood |
| 2 | Voltage not divided by 1000 | 12000V instead of 12.0V | Threshold check flags every PSU |
| 3 | Comparing °C to RPM | Meaningless boolean | Possibly never |
| 4 | Voltage passed to `log_temp_untyped()` | Silent data corruption in logs | 6 months later, reading history |

### The Solution: Typed Commands via Associated Types

#### Step 1 — Domain newtypes

```rust
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Celsius(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Rpm(u32);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Volts(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Watts(f64);
```

#### Step 2 — The command trait (the GADT equivalent)

The associated type `Response` is the key — it binds each command to its return type:

```rust
trait IpmiCmd {
    /// The GADT "index" — determines what execute() returns.
    type Response;

    fn net_fn(&self) -> u8;
    fn cmd_byte(&self) -> u8;
    fn payload(&self) -> Vec<u8>;

    /// Parsing is encapsulated HERE — each command knows its own byte layout.
    fn parse_response(&self, raw: &[u8]) -> io::Result<Self::Response>;
}
```

#### Step 3 — One struct per command, parsing written once

```rust
struct ReadTemp { sensor_id: u8 }
impl IpmiCmd for ReadTemp {
    type Response = Celsius;  // ← "this command returns a temperature"
    fn net_fn(&self) -> u8 { 0x04 }
    fn cmd_byte(&self) -> u8 { 0x2D }
    fn payload(&self) -> Vec<u8> { vec![self.sensor_id] }
    fn parse_response(&self, raw: &[u8]) -> io::Result<Celsius> {
        // Signed byte per IPMI SDR — written once, tested once
        Ok(Celsius(raw[0] as i8 as f64))
    }
}

struct ReadFanSpeed { fan_id: u8 }
impl IpmiCmd for ReadFanSpeed {
    type Response = Rpm;     // ← "this command returns RPM"
    fn net_fn(&self) -> u8 { 0x04 }
    fn cmd_byte(&self) -> u8 { 0x2D }
    fn payload(&self) -> Vec<u8> { vec![self.fan_id] }
    fn parse_response(&self, raw: &[u8]) -> io::Result<Rpm> {
        // 2-byte LE — the correct layout, encoded once
        Ok(Rpm(u16::from_le_bytes([raw[0], raw[1]]) as u32))
    }
}

struct ReadVoltage { rail: u8 }
impl IpmiCmd for ReadVoltage {
    type Response = Volts;   // ← "this command returns voltage"
    fn net_fn(&self) -> u8 { 0x04 }
    fn cmd_byte(&self) -> u8 { 0x2D }
    fn payload(&self) -> Vec<u8> { vec![self.rail] }
    fn parse_response(&self, raw: &[u8]) -> io::Result<Volts> {
        // Millivolts → Volts, always correct
        Ok(Volts(u16::from_le_bytes([raw[0], raw[1]]) as f64 / 1000.0))
    }
}

struct ReadFru { fru_id: u8 }
impl IpmiCmd for ReadFru {
    type Response = String;
    fn net_fn(&self) -> u8 { 0x0A }
    fn cmd_byte(&self) -> u8 { 0x11 }
    fn payload(&self) -> Vec<u8> { vec![self.fru_id, 0x00, 0x00, 0xFF] }
    fn parse_response(&self, raw: &[u8]) -> io::Result<String> {
        Ok(String::from_utf8_lossy(raw).to_string())
    }
}
```

#### Step 4 — The executor (zero `dyn`, monomorphised)

```rust
struct BmcConnection { timeout_secs: u32 }

impl BmcConnection {
    /// Generic over any command — compiler generates one version per command type.
    fn execute<C: IpmiCmd>(&self, cmd: &C) -> io::Result<C::Response> {
        let raw = self.raw_send(cmd.net_fn(), cmd.cmd_byte(), &cmd.payload())?;
        cmd.parse_response(&raw)
    }

    fn raw_send(&self, _nf: u8, _cmd: u8, _data: &[u8]) -> io::Result<Vec<u8>> {
        Ok(vec![0x19, 0x00]) // stub — real impl calls ipmitool
    }
}
```

#### Step 5 — Caller code: all four bugs become compile errors

```rust
fn diagnose_thermal(bmc: &BmcConnection) -> io::Result<()> {
    let cpu_temp: Celsius = bmc.execute(&ReadTemp { sensor_id: 0x20 })?;
    let fan_rpm:  Rpm     = bmc.execute(&ReadFanSpeed { fan_id: 0x30 })?;
    let voltage:  Volts   = bmc.execute(&ReadVoltage { rail: 0x40 })?;

    // Bug #1 — IMPOSSIBLE: parsing lives in ReadFanSpeed::parse_response
    // Bug #2 — IMPOSSIBLE: scaling lives in ReadVoltage::parse_response

    // Bug #3 — COMPILE ERROR:
    // if cpu_temp > fan_rpm { }
    //    ^^^^^^^^   ^^^^^^^
    //    Celsius    Rpm      → "mismatched types" ❌

    // Bug #4 — COMPILE ERROR:
    // log_temperature(voltage);
    //                 ^^^^^^^  Volts, expected Celsius ❌

    // Only correct comparisons compile:
    if cpu_temp > Celsius(85.0) {
        println!("CPU overheating: {:?}", cpu_temp);
    }
    if fan_rpm < Rpm(4000) {
        println!("Fan too slow: {:?}", fan_rpm);
    }

    Ok(())
}

fn log_temperature(t: Celsius) { println!("Temp: {:?}", t); }
fn log_voltage(v: Volts)       { println!("Voltage: {:?}", v); }
```

### Macro DSL for Diagnostic Scripts

For large diagnostic routines that run many commands in sequence, a macro gives
concise declarative syntax while preserving full type safety:

```rust
/// Execute a series of typed IPMI commands, returning a tuple of results.
/// Each element of the tuple has the command's own Response type.
macro_rules! diag_script {
    ($bmc:expr; $($cmd:expr),+ $(,)?) => {{
        ( $( $bmc.execute(&$cmd)?, )+ )
    }};
}

fn full_pre_flight(bmc: &BmcConnection) -> io::Result<()> {
    // Expands to: (Celsius, Rpm, Volts, String) — every type tracked
    let (temp, rpm, volts, board_pn) = diag_script!(bmc;
        ReadTemp     { sensor_id: 0x20 },
        ReadFanSpeed { fan_id:    0x30 },
        ReadVoltage  { rail:      0x40 },
        ReadFru      { fru_id:    0x00 },
    );

    println!("Board: {:?}", board_pn);
    println!("CPU: {:?}, Fan: {:?}, 12V: {:?}", temp, rpm, volts);

    // Type-safe threshold checks:
    assert!(temp  < Celsius(95.0), "CPU too hot");
    assert!(rpm   > Rpm(3000),     "Fan too slow");
    assert!(volts > Volts(11.4),   "12V rail sagging");

    Ok(())
}
```

The macro is just syntactic sugar — the tuple type `(Celsius, Rpm, Volts, String)` is
fully inferred by the compiler.  Swap two commands and the destructuring breaks at
compile time, not at runtime.

### Enum Dispatch for Heterogeneous Command Lists

When you need a `Vec` of mixed commands (e.g., a configurable script loaded from JSON),
use enum dispatch to stay `dyn`-free:

```rust
enum AnyReading {
    Temp(Celsius),
    Rpm(Rpm),
    Volt(Volts),
    Text(String),
}

enum AnyCmd {
    Temp(ReadTemp),
    Fan(ReadFanSpeed),
    Voltage(ReadVoltage),
    Fru(ReadFru),
}

impl AnyCmd {
    fn execute(&self, bmc: &BmcConnection) -> io::Result<AnyReading> {
        match self {
            AnyCmd::Temp(c)    => Ok(AnyReading::Temp(bmc.execute(c)?)),
            AnyCmd::Fan(c)     => Ok(AnyReading::Rpm(bmc.execute(c)?)),
            AnyCmd::Voltage(c) => Ok(AnyReading::Volt(bmc.execute(c)?)),
            AnyCmd::Fru(c)     => Ok(AnyReading::Text(bmc.execute(c)?)),
        }
    }
}

/// Dynamic diagnostic script — commands loaded at runtime
fn run_script(bmc: &BmcConnection, script: &[AnyCmd]) -> io::Result<Vec<AnyReading>> {
    script.iter().map(|cmd| cmd.execute(bmc)).collect()
}
```

You lose per-element type tracking (everything is `AnyReading`), but you gain
runtime flexibility — and the parsing is still encapsulated in each `IpmiCmd` impl.

### Testing Typed Commands

```rust
#[cfg(test)]
mod tests {
    use super::*;

    struct StubBmc {
        responses: std::collections::HashMap<u8, Vec<u8>>,
    }

    impl StubBmc {
        fn execute<C: IpmiCmd>(&self, cmd: &C) -> io::Result<C::Response> {
            let key = cmd.payload()[0]; // sensor ID as key
            let raw = self.responses.get(&key)
                .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "no stub"))?;
            cmd.parse_response(raw)
        }
    }

    #[test]
    fn read_temp_parses_signed_byte() {
        let bmc = StubBmc {
            responses: [( 0x20, vec![0xE7] )].into() // -25 as i8 = 0xE7
        };
        let temp = bmc.execute(&ReadTemp { sensor_id: 0x20 }).unwrap();
        assert_eq!(temp, Celsius(-25.0));
    }

    #[test]
    fn read_fan_parses_two_byte_le() {
        let bmc = StubBmc {
            responses: [( 0x30, vec![0x00, 0x19] )].into() // 0x1900 = 6400
        };
        let rpm = bmc.execute(&ReadFanSpeed { fan_id: 0x30 }).unwrap();
        assert_eq!(rpm, Rpm(6400));
    }

    #[test]
    fn read_voltage_scales_millivolts() {
        let bmc = StubBmc {
            responses: [( 0x40, vec![0xE8, 0x2E] )].into() // 0x2EE8 = 12008 mV
        };
        let v = bmc.execute(&ReadVoltage { rail: 0x40 }).unwrap();
        assert!((v.0 - 12.008).abs() < 0.001);
    }
}
```

Each command's parsing is tested independently.  If `ReadFanSpeed` changes from 2-byte
LE to 4-byte BE in a new IPMI spec revision, you update **one** `parse_response` and
the test catches regressions.

### How This Maps to Haskell GADTs

```text
Haskell GADT                         Rust Equivalent
────────────────                     ───────────────────────
data Cmd a where                     trait IpmiCmd {
  ReadTemp :: SensorId -> Cmd Temp       type Response;
  ReadFan  :: FanId    -> Cmd Rpm        ...
                                     }

eval :: Cmd a -> IO a                fn execute<C: IpmiCmd>(&self, cmd: &C)
                                         -> io::Result<C::Response>

Type refinement in case branches     Monomorphisation: compiler generates
                                     execute::<ReadTemp>() → returns Celsius
                                     execute::<ReadFanSpeed>() → returns Rpm
```

Both guarantee: **the command determines the return type**.  Rust achieves it through
generic monomorphisation instead of type-level case analysis — same safety, zero
runtime cost.

### Before vs After Summary

| Dimension | Untyped (`Vec<u8>`) | Typed Commands |
|-----------|:---:|:---:|
| Lines per sensor | ~3 (duplicated at every call site) | ~15 (written and tested once) |
| Parsing errors possible | At every call site | In one `parse_response` impl |
| Unit confusion bugs | Unlimited | Zero (compile error) |
| Adding a new sensor | Touch N files, copy-paste parsing | Add 1 struct + 1 impl |
| Runtime cost | — | Identical (monomorphised) |
| IDE autocomplete | `f64` everywhere | `Celsius`, `Rpm`, `Volts` — self-documenting |
| Code review burden | Must verify every raw byte parse | Verify one `parse_response` per sensor |
| Macro DSL | N/A | `diag_script!(bmc; ReadTemp{..}, ReadFan{..})` → `(Celsius, Rpm)` |
| Dynamic scripts | Manual dispatch | `AnyCmd` enum — still `dyn`-free |

### When to Use Typed Commands

| Scenario | Recommendation |
|----------|:--------------:|
| IPMI sensor reads with distinct physical units | ✅ Typed commands |
| Register map with different-width fields | ✅ Typed commands |
| Network protocol messages (request → response) | ✅ Typed commands |
| Single command type with one return format | ❌ Overkill — just return the type directly |
| Prototyping / exploring an unknown device | ❌ Raw bytes first, type later |
| Plugin system where commands aren't known at compile time | ⚠️ Use `AnyCmd` enum dispatch |

> **Key Takeaways — Traits**
> - Associated types = one impl per type; generic parameters = many impls per type
> - GATs unlock lending iterators and async-in-traits patterns
> - Use enum dispatch for closed sets (fast); `dyn Trait` for open sets (flexible)
> - `Any` + `TypeId` is the escape hatch when compile-time types are unknown

> **See also:** [Ch 1 — Generics](ch01-generics-the-full-picture.md) for monomorphization and when generics cause code bloat. [Ch 3 — Newtype & Type-State](ch03-the-newtype-and-type-state-patterns.md) for using traits with the config trait pattern.

---

### Exercise: Repository with Associated Types ★★★ (~40 min)

Design a `Repository` trait with associated `Error`, `Id`, and `Item` types. Implement it for an in-memory store and demonstrate compile-time type safety.

<details>
<summary>🔑 Solution</summary>

```rust
use std::collections::HashMap;

trait Repository {
    type Item;
    type Id;
    type Error;

    fn get(&self, id: &Self::Id) -> Result<Option<&Self::Item>, Self::Error>;
    fn insert(&mut self, item: Self::Item) -> Result<Self::Id, Self::Error>;
    fn delete(&mut self, id: &Self::Id) -> Result<bool, Self::Error>;
}

#[derive(Debug, Clone)]
struct User {
    name: String,
    email: String,
}

struct InMemoryUserRepo {
    data: HashMap<u64, User>,
    next_id: u64,
}

impl InMemoryUserRepo {
    fn new() -> Self {
        InMemoryUserRepo { data: HashMap::new(), next_id: 1 }
    }
}

impl Repository for InMemoryUserRepo {
    type Item = User;
    type Id = u64;
    type Error = std::convert::Infallible;

    fn get(&self, id: &u64) -> Result<Option<&User>, Self::Error> {
        Ok(self.data.get(id))
    }

    fn insert(&mut self, item: User) -> Result<u64, Self::Error> {
        let id = self.next_id;
        self.next_id += 1;
        self.data.insert(id, item);
        Ok(id)
    }

    fn delete(&mut self, id: &u64) -> Result<bool, Self::Error> {
        Ok(self.data.remove(id).is_some())
    }
}

fn create_and_fetch<R: Repository>(repo: &mut R, item: R::Item) -> Result<(), R::Error>
where
    R::Item: std::fmt::Debug,
    R::Id: std::fmt::Debug,
{
    let id = repo.insert(item)?;
    println!("Inserted with id: {id:?}");
    let retrieved = repo.get(&id)?;
    println!("Retrieved: {retrieved:?}");
    Ok(())
}

fn main() {
    let mut repo = InMemoryUserRepo::new();
    create_and_fetch(&mut repo, User {
        name: "Alice".into(),
        email: "alice@example.com".into(),
    }).unwrap();
}
```

</details>

***
