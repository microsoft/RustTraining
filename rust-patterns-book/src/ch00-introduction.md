# Rust Patterns & Engineering How-Tos

## Speaker Intro

- Principal Firmware Architect in Microsoft SCHIE (Silicon and Cloud Hardware Infrastructure Engineering) team
- Industry veteran with expertise in security, systems programming (firmware, operating systems, hypervisors), CPU and platform architecture, and C++ systems
- Started programming in Rust in 2017 (@AWS EC2), and have been in love with the language ever since

---

A practical guide to intermediate-and-above Rust patterns that arise in real codebases. This is not a language tutorial — it assumes you can write basic Rust and want to level up. Each chapter isolates one concept, explains when and why to use it, and provides compilable examples with inline exercises.

## Who This Is For

- Developers who have finished *The Rust Programming Language* but struggle with "how do I actually design this?"
- C++/C# engineers translating production systems into Rust
- Anyone who has hit a wall with generics, trait bounds, or lifetime errors and wants a systematic toolkit

## Prerequisites

Before starting, you should be comfortable with:
- Ownership, borrowing, and lifetimes (basic level)
- Enums, pattern matching, and `Option`/`Result`
- Structs, methods, and basic traits (`Display`, `Debug`, `Clone`)
- Cargo basics: `cargo build`, `cargo test`, `cargo run`

## How to Use This Book

### Difficulty Legend

Each chapter is tagged with a difficulty level:

| Symbol | Level | Meaning |
|--------|-------|---------|
| 🟢 | Fundamentals | Core concepts every Rust developer needs |
| 🟡 | Intermediate | Patterns used in production codebases |
| 🔴 | Advanced | Deep language mechanics — revisit as needed |

### Pacing Guide

| Chapters | Topic | Suggested Time | Checkpoint |
|----------|-------|----------------|------------|
| **Part I: Type-Level Patterns** | | | |
| 1. Generics 🟢 | Monomorphization, const generics, `const fn` | 1–2 hours | Can explain when `dyn Trait` beats generics |
| 2. Traits 🟡 | Associated types, GATs, blanket impls, vtables | 3–4 hours | Can design a trait with associated types |
| 3. Newtype & Type-State 🟡 | Zero-cost safety, compile-time FSMs | 2–3 hours | Can build a type-state builder pattern |
| 4. PhantomData 🔴 | Lifetime branding, variance, drop check | 2–3 hours | Can explain why `PhantomData<fn(T)>` differs from `PhantomData<T>` |
| **Part II: Concurrency & Runtime** | | | |
| 5. Channels 🟢 | `mpsc`, crossbeam, `select!`, actors | 1–2 hours | Can implement a channel-based worker pool |
| 6. Concurrency 🟡 | Threads, rayon, Mutex, RwLock, atomics | 2–3 hours | Can pick the right sync primitive for a scenario |
| 7. Closures 🟢 | `Fn`/`FnMut`/`FnOnce`, combinators | 1–2 hours | Can write a higher-order function that accepts closures |
| 8. Functional vs. Imperative 🟡 | Combinators, iterator adapters, functional patterns | 2–3 hours | Can explain when functional style beats imperative |
| 9. Smart Pointers 🟡 | Box, Rc, Arc, RefCell, Cow, Pin | 2–3 hours | Can explain when to use each smart pointer |
| **Part III: Systems & Production** | | | |
| 10. Error Handling 🟢 | thiserror, anyhow, `?` operator | 1–2 hours | Can design an error type hierarchy |
| 11. Serialization 🟡 | serde, zero-copy, binary data | 2–3 hours | Can write a custom serde deserializer |
| 12. Unsafe 🔴 | Superpowers, FFI, UB pitfalls, allocators | 2–3 hours | Can wrap unsafe code in a sound safe API |
| 13. Macros 🟡 | `macro_rules!`, proc macros, `syn`/`quote` | 2–3 hours | Can write a declarative macro with `tt` munching |
| 14. Testing 🟢 | Unit/integration/doc tests, proptest, criterion | 1–2 hours | Can set up property-based tests |
| 15. API Design 🟡 | Module layout, ergonomic APIs, feature flags | 2–3 hours | Can apply the "parse, don't validate" pattern |
| 16. Async 🔴 | Futures, Tokio, common pitfalls | 1–2 hours | Can identify async anti-patterns |
| **Appendices** | | | |
| Reference Card | Quick-look trait bounds, lifetimes, patterns | As needed | — |
| Capstone Project | Type-safe task scheduler | 4–6 hours | Submit a working implementation |

**Total estimated time**: 30–45 hours for thorough study with exercises.

### Working Through Exercises

Every chapter ends with a hands-on exercise. For maximum learning:

1. **Try it yourself first** — spend at least 15 minutes before opening the solution
2. **Type the code** — don't copy-paste; typing builds muscle memory
3. **Modify the solution** — add a feature, change a constraint, break something on purpose
4. **Check cross-references** — most exercises combine patterns from multiple chapters

The capstone project (Appendix) ties together patterns from across the book into a single, production-quality system.

## Table of Contents

### Part I: Type-Level Patterns

**[1. Generics — The Full Picture](ch01-generics-the-full-picture.md)** 🟢
Monomorphization, code bloat trade-offs, generics vs enums vs trait objects, const generics, `const fn`.

**[2. Traits In Depth](ch02-traits-in-depth.md)** 🟡
Associated types, GATs, blanket impls, marker traits, vtables, HRTBs, extension traits, enum dispatch.

**[3. The Newtype and Type-State Patterns](ch03-the-newtype-and-type-state-patterns.md)** 🟡
Zero-cost type safety, compile-time state machines, builder patterns, config traits.

**[4. PhantomData — Types That Carry No Data](ch04-phantomdata-types-that-carry-no-data.md)** 🔴
Lifetime branding, unit-of-measure pattern, drop check, variance.

### Part II: Concurrency & Runtime

**[5. Channels and Message Passing](ch05-channels-and-message-passing.md)** 🟢
`std::sync::mpsc`, crossbeam, `select!`, backpressure, actor pattern.

**[6. Concurrency vs Parallelism vs Threads](ch06-concurrency-vs-parallelism-vs-threads.md)** 🟡
OS threads, scoped threads, rayon, Mutex/RwLock/Atomics, Condvar, OnceLock, lock-free patterns.

**[7. Closures and Higher-Order Functions](ch07-closures-and-higher-order-functions.md)** 🟢
`Fn`/`FnMut`/`FnOnce`, closures as parameters/return values, combinators, higher-order APIs.

**[8. Functional vs. Imperative: When Elegance Wins (and When It Doesn't)](ch08-functional-vs-imperative-when-elegance-wins.md)** 🟡
Combinators, iterator adapters, functional patterns.

**[9. Smart Pointers and Interior Mutability](ch09-smart-pointers-and-interior-mutability.md)** 🟡
Box, Rc, Arc, Weak, Cell/RefCell, Cow, Pin, ManuallyDrop.

### Part III: Systems & Production

**[10. Error Handling Patterns](ch10-error-handling-patterns.md)** 🟢
thiserror vs anyhow, `#[from]`, `.context()`, `?` operator, panics.

**[11. Serialization, Zero-Copy, and Binary Data](ch11-serialization-zero-copy-and-binary-data.md)** 🟡
serde fundamentals, enum representations, zero-copy deserialization, `repr(C)`, `bytes::Bytes`.

**[12. Unsafe Rust — Controlled Danger](ch12-unsafe-rust-controlled-danger.md)** 🔴
Five superpowers, sound abstractions, FFI, UB pitfalls, arena/slab allocators.

**[13. Macros — Code That Writes Code](ch13-macros-code-that-writes-code.md)** 🟡
`macro_rules!`, when (not) to use macros, proc macros, derive macros, `syn`/`quote`.

**[14. Testing and Benchmarking Patterns](ch14-testing-and-benchmarking-patterns.md)** 🟢
Unit/integration/doc tests, proptest, criterion, mocking strategies.

**[15. Crate Architecture and API Design](ch15-crate-architecture-and-api-design.md)** 🟡
Module layout, API design checklist, ergonomic parameters, feature flags, workspaces.

**[16. Async/Await Essentials](ch16-asyncawait-essentials.md)** 🔴
Futures, Tokio quick-start, common pitfalls. (For deep async coverage, see our Async Rust Training.)

### Appendices

**[Summary and Reference Card](ch18-summary-and-reference-card.md)**
Pattern decision guide, trait bounds cheat sheet, lifetime elision rules, further reading.

**[Capstone Project: Type-Safe Task Scheduler](ch19-capstone-project.md)**
Integrate generics, traits, typestate, channels, error handling, and testing into a complete system.

***

