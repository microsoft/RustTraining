# WebAssembly and wasm-bindgen

## Why This Matters for TypeScript Developers

WebAssembly (Wasm) is where Rust and TypeScript intersect most directly. You can compile Rust
to Wasm and call it from your TypeScript/JavaScript application — getting native-speed
performance for compute-heavy tasks while keeping your UI in TypeScript.

## Use Cases

- **Image/video processing** — filters, resizing, encoding.
- **Cryptography** — hashing, encryption in the browser.
- **Data transformation** — parsing large CSV/JSON files.
- **Games** — physics engines, pathfinding.
- **Compression** — gzip, brotli, zstd.

## Setup with wasm-pack

```bash
cargo install wasm-pack
cargo new --lib my-wasm-lib
```

```toml
# Cargo.toml
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
```

## Hello Wasm

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let (mut a, mut b) = (0u64, 1u64);
            for _ in 2..=n {
                let temp = a + b;
                a = b;
                b = temp;
            }
            b
        }
    }
}
```

Build:
```bash
wasm-pack build --target web
```

## Calling from TypeScript

```typescript
import init, { greet, fibonacci } from "./pkg/my_wasm_lib.js";

async function main() {
    await init();
    console.log(greet("TypeScript"));  // "Hello, TypeScript!"
    console.log(fibonacci(50));        // 12586269025
}

main();
```

## Passing Complex Types

### Structs

```rust
#[wasm_bindgen]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}
```

```typescript
const p1 = new Point(0, 0);
const p2 = new Point(3, 4);
console.log(p1.distance(p2)); // 5.0
```

### Working with `serde` for JSON

```rust
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub title: String,
}

#[wasm_bindgen]
pub fn process_config(val: JsValue) -> Result<JsValue, JsError> {
    let config: Config = serde_wasm_bindgen::from_value(val)?;
    let result = format!("{}x{}: {}", config.width, config.height, config.title);
    Ok(JsValue::from_str(&result))
}
```

## Performance Tips

1. **Minimize boundary crossings** — each JS↔Wasm call has overhead. Batch work.
2. **Use typed arrays** — pass `&[u8]`, `&[f32]` etc. for bulk data instead of individual values.
3. **Avoid strings for hot paths** — string conversion is expensive at the boundary.
4. **Use `web-sys` and `js-sys`** crates to call browser APIs directly from Rust.

🏋️ **Exercise**: Create a Wasm module with a function that takes a `Vec<f64>` of numbers and
returns statistical measures (mean, median, standard deviation). Call it from a TypeScript file.
