# Rust for TypeScript Programmers: Complete Training Guide

A comprehensive guide to learning Rust for developers with TypeScript experience. This guide
covers everything from basic syntax to advanced patterns, focusing on the conceptual shifts
required when moving from a dynamically-typed runtime with a garbage collector and an optional
type system to a statically-typed systems language with compile-time memory safety.

## How to Use This Book

**Self-study format**: Work through Part I (ch 1–6) first — these map closely to TypeScript
concepts you already know and will feel familiar. Part II (ch 7–12) introduces the ideas that
make Rust *different*: ownership, borrowing, lifetimes, and traits. These are where TypeScript
developers typically need the most time, so take it slow. Part III (ch 13–17) covers advanced
topics, migration strategies, and the async runtime model. Part IV wraps up with a capstone
project that ties everything together.

**Workshop format (3 days)**:

| Day | Chapters | Theme |
|-----|----------|-------|
| 1   | 1 – 6    | Foundations: types, data, control flow |
| 2   | 7 – 12   | Core: ownership, traits, generics, iterators |
| 3   | 13 – 18  | Advanced: async, Wasm, migration, capstone |

**Quick reference**: Each chapter starts with a "TypeScript ↔ Rust" comparison table so you can
scan for equivalents fast. Side-by-side code blocks are used throughout to build on your
existing TypeScript knowledge.

## What You Already Know (and How It Helps)

Coming from TypeScript, you already have a strong foundation:

- **Static types** — You understand type annotations, generics, union types, and structural
  typing. Rust's type system will feel both familiar and stricter.
- **Algebraic data types** — TypeScript's discriminated unions (`type Shape = Circle | Square`)
  are conceptually close to Rust enums.
- **Generics** — You're used to `Array<T>`, `Promise<T>`, and generic functions. Rust generics
  work similarly but are monomorphized at compile time.
- **Async/await** — TypeScript's `async`/`await` over Promises maps to Rust's `async`/`await`
  over Futures, though the execution model is very different.
- **Module systems** — TypeScript's ES module imports/exports have clear parallels to Rust's
  `mod`, `use`, and `pub`.
- **Toolchain** — If you're comfortable with `npm`, `tsc`, `eslint`, and `prettier`, you'll
  find `cargo`, `rustc`, `clippy`, and `rustfmt` refreshingly similar in purpose.

## What Will Be New

These are the concepts without direct TypeScript equivalents:

- **Ownership and borrowing** — No garbage collector. The compiler enforces memory safety rules
  at compile time.
- **Lifetimes** — Explicit annotations that tell the compiler how long references live.
- **Move semantics** — Assigning a value can *move* it, making the original binding invalid.
- **No null, no undefined** — `Option<T>` and `Result<T, E>` replace nullable types.
- **No classes** — Structs + traits replace class-based OOP.
- **No runtime reflection** — No `typeof` at runtime, no `Object.keys()` on arbitrary types.
- **Manual string handling** — Multiple string types (`String`, `&str`, `OsString`, …) instead
  of one universal `string`.
- **No exceptions** — Errors are values, not thrown. `?` replaces `try/catch` in most cases.

## Conventions

Throughout this book:

- 🟦 **TypeScript** code blocks show the familiar pattern.
- 🦀 **Rust** code blocks show the idiomatic Rust equivalent.
- 💡 **Key insight** callouts explain the conceptual shift.
- ⚠️ **Common pitfall** callouts warn about traps TypeScript developers commonly fall into.
- 🏋️ **Exercise** callouts provide hands-on practice.

## Prerequisites

- Comfortable writing TypeScript (basic generics, async/await, union types).
- A working Rust installation: [rustup.rs](https://rustup.rs).
- An editor with `rust-analyzer` (VS Code recommended — you likely already have it).

Let's get started!
