# Migration Patterns

## Strategies for Introducing Rust into a TypeScript Codebase

You don't need to rewrite everything. Here are battle-tested strategies for incremental
adoption.

## Strategy 1: Wasm Module for Hot Paths

Keep your TypeScript application and replace performance-critical functions with Rust compiled
to WebAssembly. This is the lowest-risk approach.

**Example**: A data processing pipeline.

```
┌─────────────────────────────────────┐
│        TypeScript Application       │
│                                     │
│   UI ──► Parser ──► Transformer ──► │
│          (Rust/    (Rust/Wasm)      │
│           Wasm)                     │
└─────────────────────────────────────┘
```

## Strategy 2: Rust CLI Tool

Replace Node.js scripts and build tools with Rust CLIs. These are standalone binaries with
instant startup and no runtime dependency.

```bash
# Before: Node.js script (requires node, npm install)
node scripts/process-data.js --input data.csv

# After: Rust binary (single file, instant start)
./process-data --input data.csv
```

## Strategy 3: Rust Microservice

Run Rust as a separate HTTP service alongside your TypeScript backend:

```
┌──────────────┐     HTTP/gRPC     ┌──────────────┐
│  TypeScript   │ ◄──────────────► │    Rust       │
│  API Gateway  │                  │  Compute Svc  │
└──────────────┘                   └──────────────┘
```

## Strategy 4: N-API Native Module

For Node.js applications, compile Rust as a native addon using `napi-rs`:

```rust
use napi_derive::napi;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}
```

```typescript
import { sum } from "./index.node";
console.log(sum(2, 3)); // 5
```

## TypeScript → Rust Translation Patterns

### Classes → Structs + Impl + Traits

🟦 **TypeScript**
```typescript
abstract class Animal {
    constructor(public name: string) {}
    abstract speak(): string;
    greet(): string { return `I'm ${this.name}`; }
}

class Dog extends Animal {
    speak() { return "Woof!"; }
}
```

🦀 **Rust**
```rust
trait Animal {
    fn name(&self) -> &str;
    fn speak(&self) -> String;
    fn greet(&self) -> String {
        format!("I'm {}", self.name())
    }
}

struct Dog { name: String }

impl Animal for Dog {
    fn name(&self) -> &str { &self.name }
    fn speak(&self) -> String { "Woof!".to_string() }
}
```

### Inheritance → Composition + Traits

Rust has no inheritance. Use composition:

```rust
struct Logger { prefix: String }
struct Database { connection: String }

struct App {
    logger: Logger,
    db: Database,
}
```

### Optional Fields → `Option<T>`

```typescript
interface Config {
    host: string;
    port?: number;
    debug?: boolean;
}
```

```rust
struct Config {
    host: String,
    port: Option<u16>,
    debug: Option<bool>,
}
```

### Builder Pattern (replaces optional constructor arguments)

```rust
struct ServerConfig {
    host: String,
    port: u16,
    workers: usize,
}

struct ServerConfigBuilder {
    host: String,
    port: u16,
    workers: usize,
}

impl ServerConfigBuilder {
    fn new(host: impl Into<String>) -> Self {
        Self { host: host.into(), port: 8080, workers: 4 }
    }

    fn port(mut self, port: u16) -> Self { self.port = port; self }
    fn workers(mut self, n: usize) -> Self { self.workers = n; self }

    fn build(self) -> ServerConfig {
        ServerConfig {
            host: self.host,
            port: self.port,
            workers: self.workers,
        }
    }
}

let config = ServerConfigBuilder::new("localhost")
    .port(3000)
    .workers(8)
    .build();
```

### Callbacks → Closures or Channels

```typescript
function processAsync(data: string, callback: (result: string) => void) {
    setTimeout(() => callback(data.toUpperCase()), 100);
}
```

```rust
fn process_async(data: &str, callback: impl FnOnce(String)) {
    let result = data.to_uppercase();
    callback(result);
}
```

🏋️ **Exercise**: Take a TypeScript module from one of your projects and translate it to Rust.
Start with a simple utility module with pure functions.
