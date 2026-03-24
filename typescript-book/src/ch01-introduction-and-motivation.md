# Introduction and Motivation

## Why Rust for TypeScript Developers?

As a TypeScript developer, you chose TypeScript over JavaScript for a reason: you value type
safety, better tooling, and catching bugs early. Rust takes that philosophy to its logical
extreme — every category of bug that TypeScript's type system can't catch (null pointer
dereferences at runtime, data races, use-after-free) is caught at compile time in Rust.

### Where TypeScript Falls Short

TypeScript improves on JavaScript, but it still inherits fundamental limitations:

- **Runtime overhead** — V8 (or Deno/Bun) adds GC pauses, JIT warm-up, and memory overhead.
- **Type erasure** — Types vanish at runtime. A `User` type offers zero runtime guarantees.
- **`any` escape hatch** — One `any` can silently break an entire type chain.
- **Concurrency model** — Single-threaded event loop. True parallelism requires worker threads
  with message passing and serialization overhead.
- **No memory control** — You cannot control allocations, layout, or lifetimes.

### What Rust Offers

| Concern | TypeScript | Rust |
|---------|-----------|------|
| Null safety | Optional (`strictNullChecks`) | Enforced (`Option<T>`) |
| Error handling | Thrown exceptions (unchecked) | `Result<T, E>` (checked) |
| Memory management | Garbage collector | Ownership system (zero-cost) |
| Concurrency | Single-threaded + workers | Fearless concurrency (threads, async) |
| Performance | JIT-compiled, GC pauses | Compiled to native, no runtime |
| Type guarantees | Erased at runtime | Present at compile time, monomorphized |
| Package manager | npm / yarn / pnpm | cargo (built-in) |

### When to Reach for Rust

Rust is not a replacement for TypeScript in every scenario. Use Rust when you need:

- **Performance-critical services** — HTTP servers, data pipelines, real-time systems.
- **WebAssembly modules** — Ship compiled Rust to the browser alongside your TypeScript app.
- **CLI tools** — Fast startup, single binary, no runtime dependency.
- **Embedded / systems** — Where a GC and runtime are not available.
- **Correctness guarantees** — When "it compiled, therefore it works" matters.

Keep using TypeScript for rapid UI prototyping, full-stack web apps where developer velocity
is paramount, and anywhere the Node.js ecosystem gives you a critical advantage.

## Toolchain Comparison

If you're used to the TypeScript toolchain, here's how Rust's tools map:

| TypeScript | Rust | Purpose |
|-----------|------|---------|
| `npm` / `yarn` / `pnpm` | `cargo` | Package management and task runner |
| `package.json` | `Cargo.toml` | Project manifest |
| `node_modules/` | `~/.cargo/registry/` | Dependency cache |
| `tsc` | `rustc` | Compiler |
| `tsconfig.json` | `Cargo.toml` + `rustfmt.toml` | Compiler and formatter config |
| `eslint` | `clippy` | Linter |
| `prettier` | `rustfmt` | Formatter |
| `jest` / `vitest` | `cargo test` | Test runner (built-in) |
| `tsx` / `ts-node` | `cargo run` | Run a project |
| `npmjs.com` | `crates.io` | Package registry |
| `tsdoc` / `typedoc` | `cargo doc` / `rustdoc` | Documentation generator |

## Hello, Rust!

Let's compare a minimal program:

🟦 **TypeScript**
```typescript
function greet(name: string): string {
  return `Hello, ${name}!`;
}

console.log(greet("TypeScript"));
```

🦀 **Rust**
```rust
fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

fn main() {
    println!("{}", greet("Rust"));
}
```

💡 **Key differences to notice**:
- `fn` instead of `function`.
- Return type comes after `->`, not before with `:`.
- No `return` keyword needed — the last expression is the return value (no semicolon).
- `&str` is a *borrowed* string slice; `String` is an *owned* heap string. We'll cover this
  distinction in depth in Chapter 7.
- `main()` is the entry point — there's no top-level execution like Node.js.
- `println!` is a *macro* (note the `!`), not a function.

🏋️ **Exercise**: Create a new project with `cargo new hello-ts` and modify `src/main.rs` to
print a greeting. Run it with `cargo run`.
