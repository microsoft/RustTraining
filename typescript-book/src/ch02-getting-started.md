# Getting Started

## Installing Rust

```bash
# Install rustup (manages Rust toolchains)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

This is analogous to installing Node.js via `nvm` — `rustup` manages compiler versions the way
`nvm` manages Node versions.

## Creating a Project

🟦 **TypeScript**
```bash
mkdir my-project && cd my-project
npm init -y
# edit package.json, install typescript, create tsconfig.json…
```

🦀 **Rust**
```bash
cargo new my-project
cd my-project
# That's it. Cargo.toml and src/main.rs are ready.
```

`cargo new` creates:

```
my-project/
├── Cargo.toml    # ≈ package.json
└── src/
    └── main.rs   # entry point
```

### Cargo.toml vs package.json

```toml
[package]
name = "my-project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
```

Compared to `package.json`, there are no `devDependencies` vs `dependencies` split for the
binary itself — `[dev-dependencies]` exists but is only for test/build-time crates.

## Cargo Commands You'll Use Daily

| Command | Purpose | TypeScript equivalent |
|---------|---------|----------------------|
| `cargo build` | Compile (debug) | `tsc` |
| `cargo build --release` | Compile (optimized) | `tsc` + bundler |
| `cargo run` | Build + run | `tsx src/index.ts` |
| `cargo test` | Run all tests | `vitest run` |
| `cargo clippy` | Lint | `eslint .` |
| `cargo fmt` | Format | `prettier --write .` |
| `cargo doc --open` | Generate and view docs | `typedoc` |
| `cargo add serde` | Add a dependency | `npm install serde` |

## Editor Setup

If you use VS Code (most TypeScript developers do):

1. Install the **rust-analyzer** extension (replaces the older Rust extension).
2. Install **CodeLLDB** for debugging.
3. Optional: **Even Better TOML** for `Cargo.toml` syntax highlighting.

`rust-analyzer` provides the same kind of experience you get from TypeScript's language
server — inline type hints, go-to-definition, auto-imports, and real-time error checking.

## Your First Cargo Project

```rust
// src/main.rs
fn main() {
    let language = "Rust";        // type inferred as &str
    let year: u32 = 2015;         // explicit type annotation
    println!("{language} was released in {year}");
}
```

Run it:
```bash
$ cargo run
   Compiling my-project v0.1.0
    Finished dev [unoptimized + debuginfo] target(s)
     Running `target/debug/my-project`
Rust was released in 2015
```

💡 **Key insight**: `let` in Rust is *immutable by default*. To mutate a variable, use
`let mut`. This is the opposite of TypeScript, where `let` is mutable and `const` is
immutable.

```rust
let x = 5;
// x = 6;           // ❌ error: cannot assign twice to immutable variable
let mut y = 5;
y = 6;              // ✅ ok
```

🏋️ **Exercise**: Create a project with `cargo new playground`, add the `chrono` crate with
`cargo add chrono`, and print today's date.
