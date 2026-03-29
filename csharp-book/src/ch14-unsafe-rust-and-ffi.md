## Unsafe Rust

> **What you'll learn:** What `unsafe` permits (raw pointers, FFI, unchecked casts), safe wrapper patterns,
> C# P/Invoke vs Rust FFI for calling native code, and the safety checklist for `unsafe` blocks.
>
> **Difficulty:** 🔴 Advanced

Unsafe Rust allows you to perform operations that the borrow checker cannot verify. Use it sparingly and with clear documentation.

> **Advanced coverage**: For safe abstraction patterns over unsafe code (arena allocators, lock-free structures, custom vtables), see [Rust Patterns](../../rust-patterns-book/src/summary.md).

### When You Need Unsafe
```rust
// 1. Dereferencing raw pointers
let mut value = 42;
let ptr = &mut value as *mut i32;
// SAFETY: ptr points to a valid, live local variable.
unsafe {
    *ptr = 100; // Must be in unsafe block
}

// 2. Calling unsafe functions
unsafe fn dangerous() {
    // Internal implementation that requires caller to maintain invariants
}

// SAFETY: no invariants to uphold for this example function.
unsafe {
    dangerous(); // Caller takes responsibility
}

// 3. Accessing mutable static variables
static mut COUNTER: u32 = 0;
// SAFETY: single-threaded context; no concurrent access to COUNTER.
unsafe {
    COUNTER += 1; // Not thread-safe — caller must ensure synchronization
}

// 4. Implementing unsafe traits
unsafe trait UnsafeTrait {
    fn do_something(&self);
}
```

### C# Comparison: unsafe Keyword
```csharp
// C# unsafe - similar concept, different scope
unsafe void UnsafeExample()
{
    int value = 42;
    int* ptr = &value;
    *ptr = 100;
    
    // C# unsafe is about pointer arithmetic
    // Rust unsafe is about ownership/borrow rule relaxation
}

// C# fixed - pinning managed objects
unsafe void PinnedExample()
{
    byte[] buffer = new byte[100];
    fixed (byte* ptr = buffer)
    {
        // ptr is valid only within this block
    }
}
```

### Safe Wrappers
```rust
/// The key pattern: wrap unsafe code in a safe API
pub struct SafeBuffer {
    data: Vec<u8>,
}

impl SafeBuffer {
    pub fn new(size: usize) -> Self {
        SafeBuffer { data: vec![0; size] }
    }
    
    /// Safe API — bounds-checked access
    pub fn get(&self, index: usize) -> Option<u8> {
        self.data.get(index).copied()
    }
    
    /// Fast unchecked access — unsafe but wrapped safely with bounds check
    pub fn get_unchecked_safe(&self, index: usize) -> Option<u8> {
        if index < self.data.len() {
            // SAFETY: we just checked that index is in bounds
            Some(unsafe { *self.data.get_unchecked(index) })
        } else {
            None
        }
    }
}
```

***

## Interop with C# via FFI

Rust can expose C-compatible functions that C# can call via P/Invoke.

```mermaid
graph LR
    subgraph "C# Process"
        CS["C# Code"] -->|"P/Invoke"| MI["Marshal Layer<br>UTF-16 → UTF-8<br>struct layout"]
    end
    MI -->|"C ABI call"| FFI["FFI Boundary"]
    subgraph "Rust cdylib (.so / .dll)"
        FFI --> RF["extern \"C\" fn<br>#[no_mangle]"]
        RF --> Safe["Safe Rust<br>internals"]
    end

    style FFI fill:#fff9c4,color:#000
    style MI fill:#bbdefb,color:#000
    style Safe fill:#c8e6c9,color:#000
```

### Rust Library (compiled as cdylib)
```rust
// src/lib.rs
#[no_mangle]
pub extern "C" fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn process_string(input: *const std::os::raw::c_char) -> i32 {
    // SAFETY: input is non-null (checked inside) and assumed null-terminated by caller.
    let c_str = unsafe {
        if input.is_null() {
            return -1;
        }
        std::ffi::CStr::from_ptr(input)
    };
    
    match c_str.to_str() {
        Ok(s) => s.len() as i32,
        Err(_) => -1,
    }
}
```

```toml
# Cargo.toml
[lib]
crate-type = ["cdylib"]
```

### C# Consumer (P/Invoke)
```csharp
using System.Runtime.InteropServices;

public static class RustInterop
{
    [DllImport("my_rust_lib", CallingConvention = CallingConvention.Cdecl)]
    public static extern int add_numbers(int a, int b);
    
    [DllImport("my_rust_lib", CallingConvention = CallingConvention.Cdecl)]
    public static extern int process_string(
        [MarshalAs(UnmanagedType.LPUTF8Str)] string input);
}

// Usage
int sum = RustInterop.add_numbers(5, 3);  // 8
int len = RustInterop.process_string("Hello from C#!");  // 15
```

### FFI Safety Checklist

When exposing Rust functions to C#, these rules prevent the most common bugs:

1. **Always use `extern "C"`** — without it, Rust uses its own (unstable) calling convention. C# P/Invoke expects the C ABI.

2. **`#[no_mangle]`** — prevents the Rust compiler from mangling the function name. Without it, C# can't find the symbol.

3. **Never let a panic cross the FFI boundary** — a Rust panic unwinding into C# is **undefined behavior**. Catch panics at FFI entry points:
    ```rust
    #[no_mangle]
    pub extern "C" fn safe_ffi_function() -> i32 {
        match std::panic::catch_unwind(|| {
            // actual logic here
            42
        }) {
            Ok(result) => result,
            Err(_) => -1,  // Return error code instead of panicking into C#
        }
    }
    ```

4. **Opaque vs transparent structs** — if C# only holds a pointer (opaque handle), `#[repr(C)]` is not needed. If C# reads struct fields via `StructLayout`, you **must** use `#[repr(C)]`:
    ```rust
    // Opaque — C# only holds IntPtr. No #[repr(C)] needed.
    pub struct Connection { /* Rust-only fields */ }

    // Transparent — C# marshals fields directly. MUST use #[repr(C)].
    #[repr(C)]
    pub struct Point { pub x: f64, pub y: f64 }
    ```

5. **Null pointer checks** — always validate pointers before dereferencing. C# can pass `IntPtr.Zero`.

6. **String encoding** — C# uses UTF-16 internally. `MarshalAs(UnmanagedType.LPUTF8Str)` converts to UTF-8 for Rust's `CStr`. Document this contract explicitly.

### End-to-End Example: Opaque Handle with Lifecycle Management

This pattern is common in production: Rust owns an object, C# holds an opaque handle, and explicit create/destroy functions manage the lifecycle.

**Rust side** (`src/lib.rs`):
```rust
use std::ffi::{c_char, CStr};

pub struct ImageProcessor {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

/// Create a new processor. Returns null on invalid dimensions.
#[no_mangle]
pub extern "C" fn processor_new(width: u32, height: u32) -> *mut ImageProcessor {
    if width == 0 || height == 0 {
        return std::ptr::null_mut();
    }
    let proc = ImageProcessor {
        width,
        height,
        pixels: vec![0u8; (width * height * 4) as usize],
    };
    Box::into_raw(Box::new(proc)) // Allocate on heap, return raw pointer
}

/// Apply a grayscale filter. Returns 0 on success, -1 on null pointer.
#[no_mangle]
pub extern "C" fn processor_grayscale(ptr: *mut ImageProcessor) -> i32 {
    // SAFETY: ptr was created by Box::into_raw (non-null), still valid.
    let proc = match unsafe { ptr.as_mut() } {
        Some(p) => p,
        None => return -1,
    };
    for chunk in proc.pixels.chunks_exact_mut(4) {
        let gray = (0.299 * chunk[0] as f64
                  + 0.587 * chunk[1] as f64
                  + 0.114 * chunk[2] as f64) as u8;
        chunk[0] = gray;
        chunk[1] = gray;
        chunk[2] = gray;
    }
    0
}

/// Destroy the processor. Safe to call with null.
#[no_mangle]
pub extern "C" fn processor_free(ptr: *mut ImageProcessor) {
    if !ptr.is_null() {
        // SAFETY: ptr was created by processor_new via Box::into_raw
        unsafe { drop(Box::from_raw(ptr)); }
    }
}
```

**C# side**:
```csharp
using System.Runtime.InteropServices;

public sealed class ImageProcessor : IDisposable
{
    [DllImport("image_rust", CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr processor_new(uint width, uint height);

    [DllImport("image_rust", CallingConvention = CallingConvention.Cdecl)]
    private static extern int processor_grayscale(IntPtr ptr);

    [DllImport("image_rust", CallingConvention = CallingConvention.Cdecl)]
    private static extern void processor_free(IntPtr ptr);

    private IntPtr _handle;

    public ImageProcessor(uint width, uint height)
    {
        _handle = processor_new(width, height);
        if (_handle == IntPtr.Zero)
            throw new ArgumentException("Invalid dimensions");
    }

    public void Grayscale()
    {
        if (processor_grayscale(_handle) != 0)
            throw new InvalidOperationException("Processor is null");
    }

    public void Dispose()
    {
        if (_handle != IntPtr.Zero)
        {
            processor_free(_handle);
            _handle = IntPtr.Zero;
        }
    }
}

// Usage — IDisposable ensures Rust memory is freed
using var proc = new ImageProcessor(1920, 1080);
proc.Grayscale();
// proc.Dispose() called automatically → processor_free() → Rust drops the Vec
```

> **Key insight**: This is the Rust equivalent of C#'s `SafeHandle` pattern. Rust's `Box::into_raw` / `Box::from_raw` transfers ownership across the FFI boundary, and the C# `IDisposable` wrapper ensures cleanup.

---

## Exercises

<details>
<summary><strong>🏋️ Exercise: Safe Wrapper for Raw Pointer</strong> (click to expand)</summary>

You receive a raw pointer from a C library. Write a safe Rust wrapper:

```rust
// Simulated C API
extern "C" {
    fn lib_create_buffer(size: usize) -> *mut u8;
    fn lib_free_buffer(ptr: *mut u8);
}
```

Requirements:
1. Create a `SafeBuffer` struct that wraps the raw pointer
2. Implement `Drop` to call `lib_free_buffer`
3. Provide a safe `&[u8]` view via `as_slice()`
4. Ensure `SafeBuffer::new()` returns `None` if the pointer is null

<details>
<summary>🔑 Solution</summary>

```rust,ignore
struct SafeBuffer {
    ptr: *mut u8,
    len: usize,
}

impl SafeBuffer {
    fn new(size: usize) -> Option<Self> {
        // SAFETY: lib_create_buffer returns a valid pointer or null (checked below).
        let ptr = unsafe { lib_create_buffer(size) };
        if ptr.is_null() {
            None
        } else {
            Some(SafeBuffer { ptr, len: size })
        }
    }

    fn as_slice(&self) -> &[u8] {
        // SAFETY: ptr is non-null (checked in new()), len is the
        // allocated size, and we hold exclusive ownership.
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl Drop for SafeBuffer {
    fn drop(&mut self) {
        // SAFETY: ptr was allocated by lib_create_buffer
        unsafe { lib_free_buffer(self.ptr); }
    }
}

// Usage: all unsafe is contained in SafeBuffer
fn process(buf: &SafeBuffer) {
    let data = buf.as_slice(); // completely safe API
    println!("First byte: {}", data[0]);
}
```

**Key pattern**: Encapsulate `unsafe` in a small module with `// SAFETY:` comments. Expose a 100% safe public API. This is how Rust's standard library works — `Vec`, `String`, `HashMap` all contain unsafe internally but present safe interfaces.

</details>
</details>

***



