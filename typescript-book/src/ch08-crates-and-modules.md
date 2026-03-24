# Crates and Modules

## Module System Overview

| TypeScript | Rust | Purpose |
|-----------|------|---------|
| File = module | `mod` declaration | Namespace boundary |
| `export` | `pub` | Make items visible |
| `import { X } from` | `use path::X` | Bring items into scope |
| `package.json` | `Cargo.toml` | Project manifest |
| npm package | Crate | Unit of compilation and distribution |

## Defining Modules

🟦 **TypeScript** — each file is a module, exports opt in:
```typescript
// utils.ts
export function add(a: number, b: number): number {
    return a + b;
}

// main.ts
import { add } from "./utils";
```

🦀 **Rust** — modules are declared explicitly:

```rust
// src/main.rs
mod utils;

fn main() {
    println!("{}", utils::add(2, 3));
}

// src/utils.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

💡 **Key insight**: In Rust, `mod utils;` tells the compiler to look for `src/utils.rs` or
`src/utils/mod.rs`. Nothing is exported by default — you must add `pub` to every function,
struct, and field you want visible.

## Visibility

| Rust keyword | TypeScript equivalent | Meaning |
|-------------|----------------------|---------|
| (nothing) | (nothing — not exported) | Private to current module |
| `pub` | `export` | Public to everyone |
| `pub(crate)` | (no equivalent) | Public within the crate only |
| `pub(super)` | (no equivalent) | Public to parent module |

```rust
pub struct User {
    pub name: String,       // public
    pub(crate) email: String,   // visible within the crate
    password_hash: String,  // private (no pub)
}
```

⚠️ **Pitfall**: In TypeScript, if you export a class, all its properties are accessible. In
Rust, a `pub struct` with private fields cannot be constructed outside its module — you need a
constructor function.

```rust
impl User {
    pub fn new(name: String, email: String, password: &str) -> Self {
        Self {
            name,
            email,
            password_hash: hash(password),
        }
    }
}
```

## Nested Modules

```rust
// src/lib.rs
pub mod api {
    pub mod handlers {
        pub fn health_check() -> &'static str { "ok" }
    }
    pub mod middleware {
        pub fn log_request() { /* ... */ }
    }
}

// Usage:
use crate::api::handlers::health_check;
```

## File Layout

For a module `api` with submodules:

```
src/
├── main.rs           // mod api;
└── api/
    ├── mod.rs        // pub mod handlers; pub mod middleware;
    ├── handlers.rs
    └── middleware.rs
```

Or with the newer convention (Rust 2018+):

```
src/
├── main.rs           // mod api;
├── api.rs            // pub mod handlers; pub mod middleware;
└── api/
    ├── handlers.rs
    └── middleware.rs
```

## `use` Declarations

```rust
// Bring a single item into scope
use std::collections::HashMap;

// Bring multiple items
use std::collections::{HashMap, HashSet, BTreeMap};

// Rename on import (like TypeScript's `import { X as Y }`)
use std::collections::HashMap as Map;

// Glob import (generally discouraged, like `import *`)
use std::collections::*;

// Re-export (like TypeScript's `export { X } from "./module"`)
pub use self::handlers::health_check;
```

## Crates — Library vs Binary

A *crate* is Rust's compilation unit. There are two kinds:

- **Binary crate** — has a `main()` function, produces an executable. Entry: `src/main.rs`.
- **Library crate** — no `main()`, produces a `.rlib`. Entry: `src/lib.rs`.

A single Cargo project can have both `src/main.rs` and `src/lib.rs`.

## Adding Dependencies

```toml
# Cargo.toml
[dependencies]
serde = { version = "1", features = ["derive"] }
reqwest = { version = "0.12", features = ["json"] }
tokio = { version = "1", features = ["full"] }
```

Or via the command line:
```bash
cargo add serde --features derive
cargo add tokio --features full
```

## Workspaces — Monorepos

Similar to npm workspaces or Turborepo:

```toml
# Cargo.toml (root)
[workspace]
members = [
    "api-server",
    "shared-types",
    "cli-tool",
]
```

Each member is a separate crate with its own `Cargo.toml` but they share a single
`target/` directory and `Cargo.lock`.

🏋️ **Exercise**: Create a library crate with a `math` module containing `add`, `subtract`,
and `multiply` functions. Write a binary crate that depends on the library and uses all three.
