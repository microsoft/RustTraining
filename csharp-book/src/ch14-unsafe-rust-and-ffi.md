<a id="unsafe-rust"></a>
## Unsafe Rust 개요

> **이 장에서 배우는 것:** `unsafe`가 허용하는 작업(로 포인터, FFI, 검사되지 않은 캐스트), 안전한 래퍼 패턴, 네이티브 코드를 호출할 때의 C# P/Invoke vs Rust FFI 차이, 그리고 `unsafe` 블록을 작성할 때의 안전성 체크리스트를 배웁니다.
>
> **난이도:** 🔴 고급

Unsafe Rust를 사용하면 borrow checker가 검증할 수 없는 작업을 수행할 수 있습니다. 꼭 필요할 때만, 그리고 가정을 분명히 문서화하면서 사용해야 합니다.

> **심화 학습:** unsafe 코드 위에 안전한 추상화를 쌓는 패턴(arena allocator, lock-free 구조, custom vtable 등)은 [Rust Patterns](../../source-docs/RUST_PATTERNS.md)에서 더 자세히 다룹니다.

<a id="when-you-need-unsafe"></a>
### `unsafe`가 필요한 경우
```rust
// 1. 로 포인터 역참조
let mut value = 42;
let ptr = &mut value as *mut i32;
unsafe {
    *ptr = 100; // unsafe 블록 안에서만 가능하다
}

// 2. unsafe 함수 호출
unsafe fn dangerous() {
    // 호출자가 불변 조건을 지켜야 하는 내부 구현
}

unsafe {
    dangerous(); // 호출자가 책임을 진다
}

// 3. 가변 static 변수 접근
static mut COUNTER: u32 = 0;
unsafe {
    COUNTER += 1; // 스레드 안전하지 않다 - 호출자가 동기화를 보장해야 한다
}

// 4. unsafe 트레잇 구현
unsafe trait UnsafeTrait {
    fn do_something(&self);
}
```

<a id="c-comparison-unsafe-keyword"></a>
### C# 비교: `unsafe` 키워드
```csharp
// C# unsafe - 비슷한 개념이지만 범위가 다르다
unsafe void UnsafeExample()
{
    int value = 42;
    int* ptr = &value;
    *ptr = 100;
    
    // C#의 unsafe는 주로 포인터 연산을 의미한다
    // Rust의 unsafe는 소유권/대여 규칙을 컴파일러가 더 이상 보장하지 않는다는 뜻에 가깝다
}

// C# fixed - 관리 객체를 고정(pin)한다
unsafe void PinnedExample()
{
    byte[] buffer = new byte[100];
    fixed (byte* ptr = buffer)
    {
        // ptr은 이 블록 안에서만 유효하다
    }
}
```

<a id="safe-wrappers"></a>
### 안전한 래퍼
```rust
/// 핵심 패턴: unsafe 코드를 안전한 API로 감싼다
pub struct SafeBuffer {
    data: Vec<u8>,
}

impl SafeBuffer {
    pub fn new(size: usize) -> Self {
        SafeBuffer { data: vec![0; size] }
    }
    
    /// 안전한 API - 범위 검사를 거친 접근
    pub fn get(&self, index: usize) -> Option<u8> {
        self.data.get(index).copied()
    }
    
    /// 빠른 미검사 접근 - 내부적으로 unsafe지만 범위 검사로 안전하게 감싼다
    pub fn get_unchecked_safe(&self, index: usize) -> Option<u8> {
        if index < self.data.len() {
            // SAFETY: 바로 위에서 index가 범위 안에 있음을 확인했다
            Some(unsafe { *self.data.get_unchecked(index) })
        } else {
            None
        }
    }
}
```

***

<a id="interop-with-c-via-ffi"></a>
## FFI를 통한 C# 상호운용

Rust는 C ABI와 호환되는 함수를 노출할 수 있고, C#은 이를 P/Invoke로 호출할 수 있습니다.

```mermaid
graph LR
    subgraph "C# 프로세스"
        CS["C# 코드"] -->|"P/Invoke"| MI["마샬링 계층\nUTF-16 → UTF-8\nstruct layout"]
    end
    MI -->|"C ABI 호출"| FFI["FFI 경계"]
    subgraph "Rust cdylib (.so / .dll)"
        FFI --> RF["extern \"C\" fn\n#[no_mangle]"]
        RF --> Safe["안전한 Rust\n내부 구현"]
    end

    style FFI fill:#fff9c4,color:#000
    style MI fill:#bbdefb,color:#000
    style Safe fill:#c8e6c9,color:#000
```

<a id="rust-library-compiled-as-cdylib"></a>
### Rust 라이브러리 (`cdylib`로 컴파일)
```rust
// src/lib.rs
#[no_mangle]
pub extern "C" fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn process_string(input: *const std::os::raw::c_char) -> i32 {
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

<a id="c-consumer-pinvoke"></a>
### C# 호출 측 (P/Invoke)
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

<a id="ffi-safety-checklist"></a>
### FFI 안전성 체크리스트

C#에 Rust 함수를 노출할 때는 다음 규칙이 가장 흔한 버그를 막아 줍니다.

1. **항상 `extern "C"`를 사용하세요.** 이것이 없으면 Rust는 자체(안정적이지 않은) 호출 규약을 사용합니다. C#의 P/Invoke는 C ABI를 기대합니다.

2. **`#[no_mangle]`를 붙이세요.** Rust 컴파일러가 함수 이름을 name mangling하지 않도록 막아 줍니다. 이것이 없으면 C#이 심볼을 찾지 못합니다.

3. **panic이 FFI 경계를 넘어가게 두지 마세요.** Rust panic이 C# 쪽으로 unwind되면 **정의되지 않은 동작**입니다. FFI 진입점에서 panic을 잡아야 합니다.
    ```rust
    #[no_mangle]
    pub extern "C" fn safe_ffi_function() -> i32 {
        match std::panic::catch_unwind(|| {
            // 실제 로직
            42
        }) {
            Ok(result) => result,
            Err(_) => -1,  // C# 쪽으로 panic을 넘기지 말고 에러 코드를 반환
        }
    }
    ```

4. **불투명 구조체 vs 투명 구조체** - C#이 포인터만 들고 있는 opaque handle이라면 `#[repr(C)]`가 필요 없습니다. 반대로 C#이 `StructLayout`으로 필드를 직접 읽는다면 **반드시** `#[repr(C)]`를 써야 합니다.
    ```rust
    // Opaque - C#은 IntPtr만 들고 있다. #[repr(C)]가 필요 없다.
    pub struct Connection { /* Rust-only fields */ }

    // Transparent - C#이 필드를 직접 마샬링한다. 반드시 #[repr(C)]를 써야 한다.
    #[repr(C)]
    pub struct Point { pub x: f64, pub y: f64 }
    ```

5. **null 포인터를 항상 검사하세요.** 역참조하기 전에 포인터를 검증해야 합니다. C#은 `IntPtr.Zero`를 넘길 수 있습니다.

6. **문자열 인코딩 계약을 명시하세요.** C#은 내부적으로 UTF-16을 사용합니다. `MarshalAs(UnmanagedType.LPUTF8Str)`는 이를 Rust의 `CStr`가 기대하는 UTF-8로 변환합니다.

<a id="end-to-end-example-opaque-handle-with-lifecycle-management"></a>
### 전체 예제: 생명주기 관리가 있는 불투명 핸들(Opaque Handle)

이 패턴은 실무에서 자주 등장합니다. Rust가 객체를 소유하고, C#은 opaque handle만 들고 있으며, 명시적인 create/destroy 함수가 생명주기를 관리합니다.

**Rust 쪽** (`src/lib.rs`):
```rust
use std::ffi::{c_char, CStr};

pub struct ImageProcessor {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

/// 새 프로세서를 생성한다. 크기가 유효하지 않으면 null을 반환한다.
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
    Box::into_raw(Box::new(proc)) // 힙에 할당하고 로 포인터를 반환한다
}

/// 그레이스케일 필터를 적용한다. 성공 시 0, null 포인터면 -1을 반환한다.
#[no_mangle]
pub extern "C" fn processor_grayscale(ptr: *mut ImageProcessor) -> i32 {
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

/// 프로세서를 해제한다. null로 호출해도 안전하다.
#[no_mangle]
pub extern "C" fn processor_free(ptr: *mut ImageProcessor) {
    if !ptr.is_null() {
        // SAFETY: ptr은 processor_new가 Box::into_raw로 만든 포인터다
        unsafe { drop(Box::from_raw(ptr)); }
    }
}
```

**C# 쪽**:
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

// 사용 예 - IDisposable이 Rust 메모리 해제를 보장한다
using var proc = new ImageProcessor(1920, 1080);
proc.Grayscale();
// proc.Dispose()가 자동으로 호출됨 → processor_free() → Rust가 Vec를 drop
```

> **핵심 통찰:** 이것은 C#의 `SafeHandle` 패턴에 대응하는 Rust 방식입니다. Rust의 `Box::into_raw` / `Box::from_raw`가 FFI 경계 너머로 소유권을 넘기고, C#의 `IDisposable` 래퍼가 정리를 보장합니다.

---

<a id="exercises"></a>
## 연습문제

<details>
<summary><strong>🏋️ 연습문제: 로 포인터를 위한 안전한 래퍼</strong> (클릭하여 펼치기)</summary>

C 라이브러리로부터 로 포인터를 받는다고 가정합시다. 이를 감싸는 안전한 Rust 래퍼를 작성하세요.

```rust
// C API를 흉내 낸 예제
extern "C" {
    fn lib_create_buffer(size: usize) -> *mut u8;
    fn lib_free_buffer(ptr: *mut u8);
}
```

요구 사항:
1. 로 포인터를 감싸는 `SafeBuffer` 구조체를 만드세요.
2. `Drop`을 구현해 `lib_free_buffer`를 호출하세요.
3. `as_slice()`를 통해 안전한 `&[u8]` 뷰를 제공하세요.
4. 포인터가 null이면 `SafeBuffer::new()`가 `None`을 반환하도록 하세요.

<details>
<summary>🔑 해답</summary>

```rust,ignore
struct SafeBuffer {
    ptr: *mut u8,
    len: usize,
}

impl SafeBuffer {
    fn new(size: usize) -> Option<Self> {
        let ptr = unsafe { lib_create_buffer(size) };
        if ptr.is_null() {
            None
        } else {
            Some(SafeBuffer { ptr, len: size })
        }
    }

    fn as_slice(&self) -> &[u8] {
        // SAFETY: ptr은 new()에서 null이 아님을 확인했고,
        // len은 할당된 크기이며, 우리는 이 버퍼의 배타적 소유권을 가진다.
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl Drop for SafeBuffer {
    fn drop(&mut self) {
        // SAFETY: ptr은 lib_create_buffer가 할당한 포인터다
        unsafe { lib_free_buffer(self.ptr); }
    }
}

// 사용 예: 모든 unsafe가 SafeBuffer 내부에 캡슐화된다
fn process(buf: &SafeBuffer) {
    let data = buf.as_slice(); // 완전히 안전한 API
    println!("First byte: {}", data[0]);
}
```

**핵심 패턴**: `unsafe`는 작은 모듈 안에 가두고 `// SAFETY:` 주석으로 가정을 적으세요. 외부에는 100% 안전한 공개 API만 노출해야 합니다. Rust 표준 라이브러리도 같은 방식으로 동작합니다. `Vec`, `String`, `HashMap` 모두 내부적으로 unsafe를 사용하지만 외부에는 안전한 인터페이스를 제공합니다.

</details>
</details>

***



