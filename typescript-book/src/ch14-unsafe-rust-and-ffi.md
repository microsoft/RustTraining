# Unsafe Rust and FFI

## What Is Unsafe?

Rust's safety guarantees come from the borrow checker, type system, and lifetime analysis. The
`unsafe` keyword lets you opt out of specific checks when you need to do things the compiler
can't verify. Think of it as TypeScript's `as any` — but scoped, auditable, and necessary only
in rare cases.

## What Unsafe Allows

Inside an `unsafe` block, you can:
1. Dereference raw pointers.
2. Call `unsafe` functions.
3. Access mutable static variables.
4. Implement `unsafe` traits.
5. Access fields of `union` types.

Everything else (borrow checking, type checking, etc.) is still enforced.

```rust
let mut x = 42;
let ptr = &mut x as *mut i32;  // raw pointer — creating is safe

unsafe {
    *ptr = 100;  // dereferencing is unsafe
}
```

## When You'll Actually Need Unsafe

As a TypeScript developer coming to Rust, you'll rarely write `unsafe` code yourself. You'll
encounter it in:

1. **FFI** — calling C libraries or being called from C/Wasm.
2. **Performance-critical hot paths** — avoiding bounds checks in proven-safe contexts.
3. **Low-level data structures** — implementing things like custom allocators.

## FFI — Calling C from Rust

```rust
extern "C" {
    fn abs(input: i32) -> i32;
    fn strlen(s: *const u8) -> usize;
}

fn main() {
    unsafe {
        println!("abs(-5) = {}", abs(-5));
    }
}
```

## FFI — Calling Rust from C / Node.js

You can create a shared library and call it from Node.js via `node-ffi` or N-API:

```rust
// lib.rs
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

```toml
# Cargo.toml
[lib]
crate-type = ["cdylib"]
```

## Safe Abstractions over Unsafe Code

The Rust idiom is to write a small `unsafe` core and wrap it in a safe API:

```rust
pub struct SafeBuffer {
    data: *mut u8,
    len: usize,
}

impl SafeBuffer {
    pub fn new(size: usize) -> Self {
        let data = unsafe {
            std::alloc::alloc(std::alloc::Layout::array::<u8>(size).unwrap())
        };
        SafeBuffer { data, len: size }
    }

    pub fn get(&self, index: usize) -> Option<u8> {
        if index < self.len {
            Some(unsafe { *self.data.add(index) })
        } else {
            None
        }
    }
}

impl Drop for SafeBuffer {
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(
                self.data,
                std::alloc::Layout::array::<u8>(self.len).unwrap(),
            );
        }
    }
}
```

Users of `SafeBuffer` never need `unsafe` — the API ensures correctness.

🏋️ **Exercise**: Write a safe wrapper around `libc::getenv` that returns `Option<String>`.
