<a id="unsafe-rust"></a>
### Unsafe Rust 개요

> **이 장에서 배우는 것:** `unsafe`를 언제, 어떻게 써야 하는지, 로 포인터 역참조, Rust에서 C를 호출하고 C에서 Rust를 호출하는 FFI(Foreign Function Interface), 문자열 상호운용을 위한 `CString`/`CStr`, 그리고 unsafe 코드를 감싸는 안전한 래퍼를 작성하는 방법을 배웁니다.

- `unsafe`는 Rust 컴파일러가 평소에는 허용하지 않는 기능에 접근할 수 있게 해 줍니다.
    - 로 포인터 역참조
    - *mutable* 정적 변수 접근
    - https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
- 큰 힘에는 큰 책임이 따릅니다.
    - `unsafe`는 컴파일러에게 "컴파일러가 평소에 보장하는 불변 조건은 이제 프로그래머인 내가 책임지겠다"라고 선언하는 것입니다.
    - 가변 참조와 불변 참조가 별칭되지 않음, 댕글링 포인터가 없음, 잘못된 참조가 없음 등을 직접 보장해야 합니다.
    - `unsafe` 사용 범위는 가능한 한 가장 작은 스코프로 제한해야 합니다.
    - `unsafe`를 쓰는 모든 코드는 어떤 가정을 두는지 설명하는 `safety` 주석을 가져야 합니다.

<a id="unsafe-rust-examples"></a>
### Unsafe Rust 예제
```rust
unsafe fn harmless() {}
fn main() {
    // Safety: 해가 없는 unsafe 함수를 호출한다
    unsafe {
        harmless();
    }
    let a = 42u32;
    let p = &a as *const u32;
    // Safety: p는 아직 스코프 안에 있는 유효한 변수 a를 가리킨다
    unsafe {
        println!("{}", *p);
    }
    // Safety: 안전하지 않다; 설명용 예제일 뿐이다
    let dangerous_buffer = 0xb8000 as *mut u32;
    unsafe {
        println!("About to go kaboom!!!");
        *dangerous_buffer = 0; // 대부분의 최신 시스템에서는 SEGV가 발생한다
    }
}
```

<a id="simple-ffi-example-rust-library-function-consumed-by-c"></a>
### 간단한 FFI 예제 (C가 소비하는 Rust 라이브러리 함수)

<a id="ffi-strings-cstring-and-cstr"></a>
## FFI 문자열: `CString`과 `CStr`

FFI는 *Foreign Function Interface*의 약자로, Rust가 다른 언어(예: C)로 작성된 함수를 호출하고 반대로 다른 언어가 Rust 함수를 호출할 수 있게 하는 메커니즘입니다.

C 코드와 연결할 때 Rust의 `String`과 `&str` 타입은(C 문자열과 달리 UTF-8이며 null terminator가 없음) C 문자열(null 종료 바이트 배열)과 바로 호환되지 않습니다. 이를 위해 Rust는 `std::ffi`에 `CString`(소유형)과 `CStr`(대여형)을 제공합니다.

| 타입 | 비슷한 Rust 타입 | 사용하는 상황 |
|------|------------------|---------------|
| `CString` | `String`(소유) | Rust 데이터로부터 C 문자열을 만들 때 |
| `&CStr` | `&str`(대여) | 외부 코드로부터 C 문자열을 받을 때 |

```rust
use std::ffi::{CString, CStr};
use std::os::raw::c_char;

fn demo_ffi_strings() {
    // C 호환 문자열 생성(null terminator 추가)
    let c_string = CString::new("Hello from Rust").expect("CString::new failed");
    let ptr: *const c_char = c_string.as_ptr();

    // C 문자열을 다시 Rust로 변환(포인터를 신뢰하므로 unsafe)
    // Safety: ptr은 유효하며 null로 종료되어 있다(바로 위에서 생성했다)
    let back_to_rust: &CStr = unsafe { CStr::from_ptr(ptr) };
    let rust_str: &str = back_to_rust.to_str().expect("Invalid UTF-8");
    println!("{}", rust_str);
}
```

> **주의:** `CString::new()`는 입력에 중간 null 바이트(`\0`)가 있으면 에러를 반환합니다. 항상 `Result`를 처리하세요. 아래 FFI 예제들에서 `CStr`를 계속 보게 될 것입니다.

- `FFI` 함수는 컴파일러가 이름을 맹글링하지 않도록 `#[no_mangle]`를 붙여야 합니다.
- 이 크레이트는 정적 라이브러리로 컴파일합니다.
    ```
    #[no_mangle] 
    pub extern "C" fn add(left: u64, right: u64) -> u64 {
        left + right
    }
    ```
- 아래 C 코드를 컴파일한 뒤, 우리가 만든 정적 라이브러리와 링크합니다.
    ```
    #include <stdio.h>
    #include <stdint.h>
    extern uint64_t add(uint64_t, uint64_t);
    int main() {
        printf("Add returned %llu\n", add(21, 21));
    }
    ``` 

<a id="complex-ffi-example"></a>
### 복잡한 FFI 예제
- 아래 예제에서는 Rust 로깅 인터페이스를 만들고 이를 [PYTHON]과 `C`에 노출합니다.
    - 같은 인터페이스를 Rust와 C에서 네이티브하게 어떻게 사용할 수 있는지 봅니다.
    - `C`용 헤더 파일을 생성하는 `cbindgen` 같은 도구를 살펴봅니다.
    - `unsafe` 래퍼가 안전한 Rust 코드로 가는 다리 역할을 하는 방식도 확인합니다.

<a id="logger-helper-functions"></a>
## 로거 헬퍼 함수
```rust
fn create_or_open_log_file(log_file: &str, overwrite: bool) -> Result<File, String> {
    if overwrite {
        File::create(log_file).map_err(|e| e.to_string())
    } else {
        OpenOptions::new()
            .write(true)
            .append(true)
            .open(log_file)
            .map_err(|e| e.to_string())
    }
}

fn log_to_file(file_handle: &mut File, message: &str) -> Result<(), String> {
    file_handle
        .write_all(message.as_bytes())
        .map_err(|e| e.to_string())
}
```

<a id="logger-struct"></a>
## Logger 구조체
```rust
struct SimpleLogger {
    log_level: LogLevel,
    file_handle: File,
}

impl SimpleLogger {
    fn new(log_file: &str, overwrite: bool, log_level: LogLevel) -> Result<Self, String> {
        let file_handle = create_or_open_log_file(log_file, overwrite)?;
        Ok(Self {
            file_handle,
            log_level,
        })
    }

    fn log_message(&mut self, log_level: LogLevel, message: &str) -> Result<(), String> {
        if log_level as u32 <= self.log_level as u32 {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let message = format!("Simple: {timestamp} {log_level} {message}\n");
            log_to_file(&mut self.file_handle, &message)
        } else {
            Ok(())
        }
    }
}
```

<a id="testing"></a>
## 테스트
- Rust로 기능을 테스트하는 일은 매우 쉽습니다.
    - 테스트 함수에는 `#[test]`를 붙이며, 컴파일된 바이너리에는 포함되지 않습니다.
    - 테스트용 목(mock) 메서드를 만드는 것도 쉽습니다.
```rust
#[test]
fn testfunc() -> Result<(), String> {
    let mut logger = SimpleLogger::new("test.log", false, LogLevel::INFO)?;
    logger.log_message(LogLevel::TRACELEVEL1, "Hello world")?;
    logger.log_message(LogLevel::CRITICAL, "Critical message")?;
    Ok(()) // 컴파일러가 여기서 logger를 자동으로 drop한다
}
```
```bash
cargo test
```

<a id="c-rust-ffi"></a>
## (C)-Rust FFI
- `cbindgen`은 Rust에서 export한 함수를 위한 헤더 파일을 생성하는 데 매우 유용한 도구입니다.
    - cargo로 설치할 수 있습니다.
```bash
cargo install cbindgen
cbindgen 
```
- 함수와 구조체는 `#[no_mangle]` 및 `#[repr(C)]`를 통해 외부에 노출할 수 있습니다.
    - 여기서는 실제 구현체를 가리키는 `**`를 받고, 성공 시 0, 실패 시 0이 아닌 값을 반환하는 흔한 인터페이스 패턴을 가정합니다.
    - **불투명 구조체 vs 투명 구조체**: `SimpleLogger`는 *opaque pointer*(`*mut SimpleLogger`)로 전달되므로 C 쪽은 내부 필드에 접근하지 않습니다. 따라서 `#[repr(C)]`가 **필요하지 않습니다**. C 코드가 구조체 필드를 직접 읽고 써야 할 때만 `#[repr(C)]`를 사용하세요.

```rust
// Opaque — C는 포인터만 들고 있고 필드를 들여다보지 않는다. #[repr(C)]가 필요 없다.
struct SimpleLogger { /* Rust-only fields */ }

// Transparent — C가 필드를 직접 읽고 쓴다. 반드시 #[repr(C)]를 써야 한다.
#[repr(C)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
```
```c
typedef struct SimpleLogger SimpleLogger;
uint32_t create_simple_logger(const char *file_name, struct SimpleLogger **out_logger);
uint32_t log_entry(struct SimpleLogger *logger, const char *message);
uint32_t drop_logger(struct SimpleLogger *logger);
```

- 이런 FFI 경계에서는 매우 많은 sanity check가 필요합니다.
- Rust가 자동으로 메모리를 해제하지 않도록 일부 메모리는 명시적으로 leak해야 합니다.
```rust
#[no_mangle] 
pub extern "C" fn create_simple_logger(file_name: *const std::os::raw::c_char, out_logger: *mut *mut SimpleLogger) -> u32 {
    use std::ffi::CStr;
    // 포인터가 NULL이 아닌지 확인
    if file_name.is_null() || out_logger.is_null() {
        return 1;
    }
    // Safety: 계약상 전달된 포인터는 NULL이 아니면 null-terminated 문자열이다
    let file_name = unsafe {
        CStr::from_ptr(file_name)
    };
    let file_name = file_name.to_str();
    // file_name에 깨진 문자가 없는지 확인
    if file_name.is_err() {
        return 1;
    }
    let file_name = file_name.unwrap();
    // 예제에서는 기본값을 가정한다; 실제 코드에서는 인자로 받을 수 있다
    let new_logger = SimpleLogger::new(file_name, false, LogLevel::CRITICAL);
    // 로거를 정상적으로 생성했는지 확인
    if new_logger.is_err() {
        return 1;
    }
    let new_logger = Box::new(new_logger.unwrap());
    // Box가 스코프를 벗어날 때 drop되지 않도록 막는다
    let logger_ptr: *mut SimpleLogger = Box::leak(new_logger);
    // Safety: out_logger는 non-null이며 logger_ptr은 유효하다
    unsafe {
        *out_logger = logger_ptr;
    }
    return 0;
}
```

- `log_entry()`에도 비슷한 에러 검사가 들어갑니다.
```rust
#[no_mangle]
pub extern "C" fn log_entry(logger: *mut SimpleLogger, message: *const std::os::raw::c_char) -> u32 {
    use std::ffi::CStr;
    if message.is_null() || logger.is_null() {
        return 1;
    }
    // Safety: message는 non-null이다
    let message = unsafe {
        CStr::from_ptr(message)
    };
    let message = message.to_str();
    // message에 깨진 문자가 없는지 확인
    if message.is_err() {
        return 1;
    }
    // Safety: logger는 create_simple_logger()가 이전에 생성한 유효한 포인터다
    unsafe {
        (*logger).log_message(LogLevel::CRITICAL, message.unwrap()).is_err() as u32
    }
}

#[no_mangle]
pub extern "C" fn drop_logger(logger: *mut SimpleLogger) -> u32 {
    if logger.is_null() {
        return 1;
    }
    // Safety: logger는 create_simple_logger()가 이전에 생성한 유효한 포인터다
    unsafe {
        // Box<SimpleLogger>를 재구성하면, 스코프 종료 시 자동으로 drop된다
        let _ = Box::from_raw(logger);
    }
    0
}
```

- 이 (C)-FFI는 Rust로 테스트할 수도 있고, 직접 C 프로그램을 작성해서 테스트할 수도 있습니다.
```rust
#[test]
fn test_c_logger() {
    // c".."는 NULL 종료 문자열을 만든다
    let file_name = c"test.log".as_ptr() as *const std::os::raw::c_char;
    let mut c_logger: *mut SimpleLogger = std::ptr::null_mut();
    assert_eq!(create_simple_logger(file_name, &mut c_logger), 0);
    // c"..." 문자열을 만드는 수동 방식
    let message = b"message from C\0".as_ptr() as *const std::os::raw::c_char;
    assert_eq!(log_entry(c_logger, message), 0);
    drop_logger(c_logger);
}
```
```c
#include "logger.h"
...
int main() {
    SimpleLogger *logger = NULL;
    if (create_simple_logger("test.log", &logger) == 0) {
        log_entry(logger, "Hello from C");
        drop_logger(logger); /*핸들을 닫는 등 정리가 필요하다.*/
    } 
    ...
}
```

<a id="ensuring-correctness-of-unsafe-code"></a>
## unsafe 코드의 정확성 보장
- 요약하면 `unsafe`를 쓸 때는 의도적이고 신중한 사고가 필요합니다.
    - 코드가 두는 안전 가정을 항상 문서화하고, 가능하면 전문가와 함께 검토하세요.
    - `cbindgen`, Miri, Valgrind 같은 도구를 활용해 정확성을 검증하세요.
    - **패닉이 FFI 경계를 넘어 unwind되게 두면 안 됩니다.** 이것은 UB입니다. FFI 진입점에서 `std::panic::catch_unwind`를 사용하거나, 프로파일에서 `panic = "abort"`를 설정하세요.
    - 구조체가 FFI를 통해 공유된다면 C 호환 메모리 레이아웃을 보장하기 위해 `#[repr(C)]`를 붙이세요.
    - https://doc.rust-lang.org/nomicon/intro.html ("Rustonomicon", 즉 unsafe Rust의 암흑 마법서)를 참고하세요.
    - 사내 전문가의 도움을 받는 것도 좋습니다.

<a id="verification-tools-miri-vs-valgrind"></a>
### 검증 도구: Miri vs Valgrind

C++ 개발자에게 Valgrind와 sanitizer는 익숙합니다. Rust에는 그런 도구들에 더해, Rust 고유의 UB를 훨씬 더 정밀하게 잡아내는 Miri가 있습니다.

| 항목 | **Miri** | **Valgrind** | **C++ sanitizer (ASan/MSan/UBSan)** |
|------|----------|--------------|-------------------------------------|
| **무엇을 잡는가** | Rust 특화 UB: stacked borrows, 잘못된 `enum` discriminant, 초기화되지 않은 읽기, aliasing 위반 | 메모리 누수, use-after-free, 잘못된 읽기/쓰기, 초기화되지 않은 메모리 | 버퍼 오버플로, use-after-free, 데이터 레이스, UB |
| **동작 방식** | MIR(Rust 중간 IR)을 해석함. 네이티브 실행은 하지 않음 | 컴파일된 바이너리를 런타임에 계측 | 컴파일 시점 계측 |
| **FFI 지원** | ❌ FFI 경계를 넘을 수 없음(C 호출은 건너뜀) | ✅ FFI를 포함한 모든 컴파일된 바이너리에 사용 가능 | ✅ C 코드도 sanitizer로 컴파일했다면 사용 가능 |
| **속도** | 네이티브보다 약 100배 느림 | 약 10~50배 느림 | 약 2~5배 느림 |
| **언제 쓰는가** | 순수 Rust `unsafe` 코드, 자료구조 불변식 점검 | FFI 코드, 전체 바이너리 통합 테스트 | FFI의 C/C++ 쪽, 성능에 민감한 테스트 |
| **aliasing 버그 탐지** | ✅ Stacked Borrows 모델 사용 | ❌ | 부분적으로 가능(TSan은 데이터 레이스 중심) |

**권장 사항:** 둘 다 쓰세요. 순수 Rust unsafe에는 Miri, FFI 통합에는 Valgrind가 좋습니다.

- **Miri**: Valgrind가 볼 수 없는 Rust 특화 UB(aliasing 위반, 잘못된 enum 값, stacked borrows)를 잡습니다.
    ```
    rustup +nightly component add miri
    cargo +nightly miri test                    # 모든 테스트를 Miri 아래에서 실행
    cargo +nightly miri test -- test_name       # 특정 테스트만 실행
    ```
    > ⚠️ Miri는 nightly가 필요하며 FFI 호출을 실행할 수 없습니다. unsafe Rust 로직을 테스트 가능한 단위로 분리하세요.

- **Valgrind**: 이미 익숙한 그 도구이며, FFI를 포함한 컴파일된 바이너리 전체에 적용할 수 있습니다.
    ```
    sudo apt install valgrind
    cargo install cargo-valgrind
    cargo valgrind test                         # 모든 테스트를 Valgrind 아래에서 실행
    ```
    > FFI 코드에서 흔한 `Box::leak` / `Box::from_raw` 패턴의 누수를 잡는 데 유용합니다.

- **cargo-careful**: 일반 테스트와 Miri의 중간쯤 되는 추가 런타임 검사를 켜고 테스트를 실행합니다.
    ```
    cargo install cargo-careful
    cargo +nightly careful test
    ```

<a id="unsafe-rust-summary"></a>
## Unsafe Rust 요약
- `cbindgen`은 Rust에서 C로 가는 FFI에 매우 좋은 도구입니다.
    - 반대 방향 FFI 인터페이스에는 `bindgen`을 사용하세요. 문서가 매우 잘 되어 있습니다.
- **unsafe 코드가 맞다고, 혹은 safe Rust에서 써도 괜찮다고 가정하지 마세요. 실수하기 정말 쉽고, 겉보기에는 잘 동작하는 코드도 미묘한 이유로 틀릴 수 있습니다.**
    - 도구를 사용해 정확성을 검증하세요.
    - 여전히 확신이 서지 않으면 전문가에게 도움을 구하세요.
- `unsafe` 코드에는 어떤 가정 위에 서 있는지, 왜 올바른지 명시적으로 설명하는 주석이 반드시 있어야 합니다.
    - `unsafe` 코드를 호출하는 쪽도 그에 대응하는 `Safety` 주석을 두고, 필요한 제약을 지켜야 합니다.

<a id="exercise-writing-a-safe-ffi-wrapper"></a>
# 연습문제: 안전한 FFI 래퍼 작성하기

🔴 **도전** — unsafe 블록, 로 포인터, 안전한 API 설계를 함께 이해해야 합니다.

- `unsafe`한 FFI 스타일 함수를 감싸는 안전한 Rust 래퍼를 작성하세요. 이 연습문제는 호출자가 제공한 버퍼에 서식화된 문자열을 써 넣는 C 함수를 흉내 냅니다.
- **1단계**: 인사말을 로 `*mut u8` 버퍼에 쓰는 `unsafe_greet` 함수를 구현하세요.
- **2단계**: `Vec<u8>`를 할당하고, unsafe 함수를 호출한 뒤, `String`을 반환하는 안전한 래퍼 `safe_greet`를 작성하세요.
- **3단계**: 모든 unsafe 블록에 올바른 `// Safety:` 주석을 추가하세요.

**시작 코드:**
```rust
use std::fmt::Write as _;

/// C 함수를 흉내 낸다: buffer에 "Hello, <name>!"을 기록한다.
/// null terminator를 제외한 실제 기록 바이트 수를 반환한다.
/// # Safety
/// - `buf`는 최소 `buf_len` 바이트의 쓰기 가능한 메모리를 가리켜야 한다
/// - `name`은 null-terminated C 문자열을 가리키는 유효한 포인터여야 한다
unsafe fn unsafe_greet(buf: *mut u8, buf_len: usize, name: *const u8) -> isize {
    // TODO: greeting을 만들고, buf에 바이트를 복사한 뒤, 길이를 반환하라
    // 힌트: std::ffi::CStr::from_ptr를 쓰거나 바이트를 직접 순회하라
    todo!()
}

/// 안전한 래퍼 — public API에는 unsafe가 없다
fn safe_greet(name: &str) -> Result<String, String> {
    // TODO: Vec<u8> 버퍼를 할당하고, null-terminated 이름을 만든 다음,
    // Safety 주석과 함께 unsafe 블록 안에서 unsafe_greet를 호출하고,
    // 결과를 다시 String으로 변환하라
    todo!()
}

fn main() {
    match safe_greet("Rustacean") {
        Ok(msg) => println!("{msg}"),
        Err(e) => eprintln!("Error: {e}"),
    }
    // 예상 출력: Hello, Rustacean!
}
```

<details><summary>해답 (클릭하여 펼치기)</summary>

```rust
use std::ffi::CStr;

/// C 함수를 흉내 낸다: buffer에 "Hello, <name>!"을 기록한다.
/// 버퍼가 너무 작으면 -1을 반환한다.
/// # Safety
/// - `buf`는 최소 `buf_len` 바이트의 쓰기 가능한 메모리를 가리켜야 한다
/// - `name`은 null-terminated C 문자열을 가리키는 유효한 포인터여야 한다
unsafe fn unsafe_greet(buf: *mut u8, buf_len: usize, name: *const u8) -> isize {
    // Safety: 호출자가 name이 유효한 null-terminated 문자열이라고 보장한다
    let name_cstr = unsafe { CStr::from_ptr(name as *const std::os::raw::c_char) };
    let name_str = match name_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };
    let greeting = format!("Hello, {}!", name_str);
    if greeting.len() > buf_len {
        return -1;
    }
    // Safety: buf는 최소 buf_len 바이트의 쓰기 가능한 메모리를 가리킨다(호출자 보장)
    unsafe {
        std::ptr::copy_nonoverlapping(greeting.as_ptr(), buf, greeting.len());
    }
    greeting.len() as isize
}

/// 안전한 래퍼 — public API에는 unsafe가 없다
fn safe_greet(name: &str) -> Result<String, String> {
    let mut buffer = vec![0u8; 256];
    // C API에 넘길 null-terminated 이름을 만든다
    let name_with_null: Vec<u8> = name.bytes().chain(std::iter::once(0)).collect();

    // Safety: buffer는 256바이트의 쓰기 가능한 공간을 가지며, name_with_null은 null-terminated다
    let bytes_written = unsafe {
        unsafe_greet(buffer.as_mut_ptr(), buffer.len(), name_with_null.as_ptr())
    };

    if bytes_written < 0 {
        return Err("Buffer too small or invalid name".to_string());
    }

    String::from_utf8(buffer[..bytes_written as usize].to_vec())
        .map_err(|e| format!("Invalid UTF-8: {e}"))
}

fn main() {
    match safe_greet("Rustacean") {
        Ok(msg) => println!("{msg}"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
// Output:
// Hello, Rustacean!
```

</details>

----
