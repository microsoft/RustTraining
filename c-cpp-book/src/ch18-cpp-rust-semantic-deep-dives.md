<a id="cpp-rust-semantic-deep-dives"></a>
## C++ → Rust 의미론 심층 비교

> **이 장에서 배우는 것:** Rust에 1:1로 대응되지 않는 C++ 개념들을 자세히 매핑합니다. 네 가지 명명된 캐스트, SFINAE와 트레잇 바운드의 차이, CRTP와 연관 타입, 그리고 번역 과정에서 자주 부딪히는 마찰 지점을 다룹니다.

아래 섹션들은 Rust에서 명확한 1:1 대응이 없는 C++ 개념들을 매핑합니다. 이런 차이들은 번역 작업 중 C++ 프로그래머가 자주 걸려 넘어지는 지점입니다.

<a id="casting-hierarchy-four-cpp-casts-rust-equivalents"></a>
### 캐스팅 계층: 네 가지 C++ 캐스트 → Rust 대응

C++에는 이름이 붙은 캐스트가 네 가지 있습니다. Rust는 이를 서로 다른, 더 명시적인 메커니즘으로 대체합니다:

```cpp
// C++ 캐스팅 계층
int i = static_cast<int>(3.14);            // 1. 숫자 변환 / 업캐스트
Derived* d = dynamic_cast<Derived*>(base); // 2. 런타임 다운캐스팅
int* p = const_cast<int*>(cp);             // 3. const 제거
auto* raw = reinterpret_cast<char*>(&obj); // 4. 비트 수준 재해석
```

| C++ 캐스트 | Rust 대응 | 안전성 | 비고 |
|----------|----------------|--------|-------|
| `static_cast` (숫자 변환) | `as` 키워드 | 안전하지만 잘리거나 래핑될 수 있음 | `let i = 3.14_f64 as i32;` — 3으로 잘림 |
| `static_cast` (숫자 변환, 검사됨) | `From`/`Into` | 안전, 컴파일 타임 검증 | `let i: i32 = 42_u8.into();` — 확장 변환만 가능 |
| `static_cast` (숫자 변환, 실패 가능) | `TryFrom`/`TryInto` | 안전, `Result` 반환 | `let i: u8 = 300_u16.try_into()?;` — `Err` 반환 |
| `dynamic_cast` (다운캐스트) | enum에 대한 `match` / `Any::downcast_ref` | 안전 | enum은 패턴 매칭, 트레잇 객체는 `Any` 사용 |
| `const_cast` | 대응 없음 |  | Rust에는 안전 코드에서 `&` → `&mut`로 캐스팅하는 방법이 없습니다. 내부 가변성에는 `Cell`/`RefCell`을 사용합니다 |
| `reinterpret_cast` | `std::mem::transmute` | **`unsafe`** | 비트 패턴을 재해석합니다. 거의 항상 잘못된 선택이므로 `from_le_bytes()` 등을 우선하세요 |

```rust
// Rust 대응:

// 1. 숫자 캐스트 — `as`보다 From/Into를 우선
let widened: u32 = 42_u8.into();             // 실패 불가능한 확장 변환 — 항상 이쪽을 우선
let truncated = 300_u16 as u8;               // ⚠ 44로 래핑됨! 조용한 데이터 손실
let checked: Result<u8, _> = 300_u16.try_into(); // Err — 안전한 실패 가능 변환

// 2. 다운캐스트: enum(권장) 또는 Any(타입 소거가 필요할 때)
use std::any::Any;

fn handle_any(val: &dyn Any) {
    if let Some(s) = val.downcast_ref::<String>() {
        println!("Got string: {s}");
    } else if let Some(n) = val.downcast_ref::<i32>() {
        println!("Got int: {n}");
    }
}

// 3. "const_cast" → 내부 가변성 (`unsafe` 불필요)
use std::cell::Cell;
struct Sensor {
    read_count: Cell<u32>,  // `&self`를 통해 변경
}
impl Sensor {
    fn read(&self) -> f64 {
        self.read_count.set(self.read_count.get() + 1); // `&mut self`가 아니라 `&self`
        42.0
    }
}

// 4. reinterpret_cast → transmute (거의 필요 없음)
// 안전한 대안을 우선:
let bytes: [u8; 4] = 0x12345678_u32.to_ne_bytes();  // ✅ 안전
let val = u32::from_ne_bytes(bytes);                // ✅ 안전
// unsafe { std::mem::transmute::<u32, [u8; 4]>(val) } // ❌ 피하세요
```

> **가이드라인**: idiomatic Rust에서 `as`는 드물어야 합니다(`From`/`Into`는 확장 변환, `TryFrom`/`TryInto`는 축소 변환에 사용). `transmute`는 예외적인 경우에만 써야 하고, `const_cast`는 내부 가변성 타입이 그 역할을 대신하기 때문에 대응물이 없습니다.

---

<a id="preprocessor-cfg-feature-flags-and-macro-rules"></a>
### 전처리기 → `cfg`, 피처 플래그, `macro_rules!`

C++는 조건부 컴파일, 상수, 코드 생성을 위해 전처리기에 크게 의존합니다. Rust는 이 모든 것을 일급 언어 기능으로 대체합니다.

<a id="define-constants-const-or-const-fn"></a>
#### `#define` 상수 → `const` 또는 `const fn`

```cpp
// C++
#define MAX_RETRIES 5
#define BUFFER_SIZE (1024 * 64)
#define SQUARE(x) ((x) * (x))  // 매크로 — 텍스트 치환, 타입 안전성 없음
```

```rust
// Rust — 타입 안전, 스코프 보유, 텍스트 치환 없음
const MAX_RETRIES: u32 = 5;
const BUFFER_SIZE: usize = 1024 * 64;
const fn square(x: u32) -> u32 { x * x }  // 컴파일 타임 평가

// const 컨텍스트에서도 사용 가능:
const AREA: u32 = square(12);  // 컴파일 타임에 계산됨
static BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
```

<a id="ifdef-if-cfg-and-cfg"></a>
#### `#ifdef` / `#if` → `#[cfg()]`와 `cfg!()`

```cpp
// C++
#ifdef DEBUG
    log_verbose("Step 1 complete");
#endif

#if defined(LINUX) && !defined(ARM)
    use_x86_path();
#else
    use_generic_path();
#endif
```

```rust
// Rust — 속성 기반 조건부 컴파일
#[cfg(debug_assertions)]
fn log_verbose(msg: &str) { eprintln!("[VERBOSE] {msg}"); }

#[cfg(not(debug_assertions))]
fn log_verbose(_msg: &str) { /* 릴리즈에서는 컴파일에서 제거됨 */ }

// 조건 결합:
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn use_x86_path() { /* ... */ }

#[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
fn use_generic_path() { /* ... */ }

// 런타임 검사(조건 자체는 여전히 컴파일 타임이지만 식 안에서 사용 가능):
if cfg!(target_os = "windows") {
    println!("Running on Windows");
}
```

<a id="feature-flags-in-cargo-toml"></a>
#### `Cargo.toml`의 피처 플래그

```toml
# Cargo.toml — #ifdef FEATURE_FOO 대체
[features]
default = ["json"]
json = ["dep:serde_json"]       # 선택적 의존성
verbose-logging = []            # 추가 의존성 없는 플래그
gpu-support = ["dep:cuda-sys"]  # 선택적 GPU 지원
```

```rust
// 피처 플래그에 따른 조건부 코드:
#[cfg(feature = "json")]
pub fn parse_config(data: &str) -> Result<Config, Error> {
    serde_json::from_str(data).map_err(Error::from)
}

#[cfg(feature = "verbose-logging")]
macro_rules! verbose {
    ($($arg:tt)*) => { eprintln!("[VERBOSE] {}", format!($($arg)*)); }
}
#[cfg(not(feature = "verbose-logging"))]
macro_rules! verbose {
    ($($arg:tt)*) => { }; // 아무것도 생성하지 않음
}
```

<a id="define-macro-x-macro-rules"></a>
#### `#define MACRO(x)` → `macro_rules!`

```cpp
// C++ — 텍스트 치환 기반이며, 악명 높을 만큼 실수가 잦음
#define DIAG_CHECK(cond, msg) \
    do { if (!(cond)) { log_error(msg); return false; } } while(0)
```

```rust
// Rust — 위생적(hygienic)이고, 타입 검사되며, 구문 트리 위에서 동작
macro_rules! diag_check {
    ($cond:expr, $msg:expr) => {
        if !($cond) {
            log_error($msg);
            return Err(DiagError::CheckFailed($msg.to_string()));
        }
    };
}

fn run_test() -> Result<(), DiagError> {
    diag_check!(temperature < 85.0, "GPU too hot");
    diag_check!(voltage > 0.8, "Rail voltage too low");
    Ok(())
}
```

| C++ 전처리기 | Rust 대응 | 장점 |
|-----------------|----------------|-----------|
| `#define PI 3.14` | `const PI: f64 = 3.14;` | 타입이 있고, 스코프가 있으며, 디버거에서 보임 |
| `#define MAX(a,b) ((a)>(b)?(a):(b))` | `macro_rules!` 또는 제네릭 `fn max<T: Ord>` | 이중 평가 버그 없음 |
| `#ifdef DEBUG` | `#[cfg(debug_assertions)]` | 컴파일러가 검사하며, 오타 위험 없음 |
| `#ifdef FEATURE_X` | `#[cfg(feature = "x")]` | Cargo가 feature를 관리하고 의존성과 연동됨 |
| `#include "header.h"` | `mod module;` + `use module::Item;` | include guard가 필요 없고, 순환 include도 없음 |
| `#pragma once` | 필요 없음 | 각 `.rs` 파일은 하나의 모듈이며 정확히 한 번만 포함됨 |

---

<a id="header-files-and-include-modules-and-use"></a>
### 헤더 파일과 `#include` → 모듈과 `use`

C++에서 컴파일 모델은 텍스트 포함(textual inclusion)을 중심으로 돌아갑니다:

```cpp
// widget.h — Widget를 사용하는 모든 번역 단위가 이 파일을 include
#pragma once
#include <string>
#include <vector>

class Widget {
public:
    Widget(std::string name);
    void activate();
private:
    std::string name_;
    std::vector<int> data_;
};
```

```cpp
// widget.cpp — 정의는 별도 파일에 존재
#include "widget.h"
Widget::Widget(std::string name) : name_(std::move(name)) {}
void Widget::activate() { /* ... */ }
```

Rust에는 **헤더 파일도, forward declaration도, include guard도 없습니다**:

```rust
// src/widget.rs — 선언과 정의가 한 파일에 함께 있음
pub struct Widget {
    name: String,         // 기본값은 private
    data: Vec<i32>,
}

impl Widget {
    pub fn new(name: String) -> Self {
        Widget { name, data: Vec::new() }
    }
    pub fn activate(&self) { /* ... */ }
}
```

```rust
// src/main.rs — 모듈 경로로 가져오기
mod widget;  // 컴파일러에게 src/widget.rs를 포함하라고 알림
use widget::Widget;

fn main() {
    let w = Widget::new("sensor".to_string());
    w.activate();
}
```

| C++ | Rust | 더 나은 이유 |
|-----|------|-----------------|
| `#include "foo.h"` | 부모에서 `mod foo;` + `use foo::Item;` | 텍스트 포함이 없어 ODR 위반이 없음 |
| `#pragma once` / include guards | 필요 없음 | 각 `.rs` 파일은 하나의 모듈이며 한 번만 컴파일됨 |
| Forward declarations | 필요 없음 | 컴파일러가 crate 전체를 보므로 선언 순서가 중요하지 않음 |
| `class Foo;` (불완전 타입) | 필요 없음 | 선언/정의를 분리하지 않음 |
| 각 클래스마다 `.h` + `.cpp` | 단일 `.rs` 파일 | 선언/정의 불일치 버그가 없음 |
| `using namespace std;` | `use std::collections::HashMap;` | 항상 명시적이라 전역 네임스페이스 오염이 없음 |
| 중첩된 `namespace a::b` | 중첩 `mod a { mod b { } }` 또는 `a/b.rs` | 파일 시스템이 모듈 트리를 그대로 반영함 |

---

<a id="friend-and-access-control-module-visibility"></a>
### `friend`와 접근 제어 → 모듈 가시성

C++는 `friend`를 사용해 특정 클래스나 함수에 private 멤버 접근 권한을 부여합니다. Rust에는 `friend` 키워드가 없습니다. 대신 **프라이버시는 모듈 스코프**입니다:

```cpp
// C++
class Engine {
    friend class Car;   // Car는 private 멤버에 접근 가능
    int rpm_;
    void set_rpm(int r) { rpm_ = r; }
public:
    int rpm() const { return rpm_; }
};
```

```rust
// Rust — 같은 모듈 안의 아이템은 모든 필드에 접근할 수 있으므로 `friend`가 필요 없음
mod vehicle {
    pub struct Engine {
        rpm: u32,  // struct 기준이 아니라 모듈 기준으로 private
    }

    impl Engine {
        pub fn new() -> Self { Engine { rpm: 0 } }
        pub fn rpm(&self) -> u32 { self.rpm }
    }

    pub struct Car {
        engine: Engine,
    }

    impl Car {
        pub fn new() -> Self { Car { engine: Engine::new() } }
        pub fn accelerate(&mut self) {
            self.engine.rpm = 3000; // ✅ 같은 모듈 — 필드 직접 접근 가능
        }
        pub fn rpm(&self) -> u32 {
            self.engine.rpm  // ✅ 같은 모듈 — private 필드 읽기 가능
        }
    }
}

fn main() {
    let mut car = vehicle::Car::new();
    car.accelerate();
    // car.engine.rpm = 9000;  // ❌ 컴파일 에러: `engine`는 private
    println!("RPM: {}", car.rpm()); // ✅ Car의 public 메서드 사용
}
```

| C++ 접근 제어 | Rust 대응 | 범위 |
|-----------|----------------|-------|
| `private` | (기본값, 키워드 없음) | 같은 모듈 내부에서만 접근 가능 |
| `protected` | 직접 대응 없음 | 부모 모듈 접근에는 `pub(super)` 사용 |
| `public` | `pub` | 어디서나 접근 가능 |
| `friend class Foo` | `Foo`를 같은 모듈에 둠 | 모듈 수준 프라이버시가 friend를 대체 |
| — | `pub(crate)` | crate 내부에서는 보이지만 외부 의존성에서는 안 보임 |
| — | `pub(super)` | 부모 모듈에서만 보임 |
| — | `pub(in crate::path)` | 특정 모듈 서브트리 안에서만 보임 |

> **핵심 통찰**: C++의 프라이버시는 클래스 단위이고, Rust의 프라이버시는 모듈 단위입니다. 따라서 어떤 타입들을 같은 모듈에 둘지로 접근 제어를 설계합니다. 같은 모듈에 함께 둔 타입들은 서로의 private 필드에 완전히 접근할 수 있습니다.

---

<a id="volatile-atomics-and-read-volatile-write-volatile"></a>
### `volatile` → atomics와 `read_volatile`/`write_volatile`

C++에서 `volatile`은 읽기/쓰기를 최적화로 제거하지 말라고 컴파일러에 지시합니다. 보통 메모리 매핑 하드웨어 레지스터에 사용됩니다. **Rust에는 `volatile` 키워드가 없습니다.**

```cpp
// C++: 하드웨어 레지스터를 위한 volatile
volatile uint32_t* const GPIO_REG = reinterpret_cast<volatile uint32_t*>(0x4002'0000);
*GPIO_REG = 0x01;              // 쓰기가 최적화로 제거되지 않음
uint32_t val = *GPIO_REG;     // 읽기가 최적화로 제거되지 않음
```

```rust
// Rust: 명시적인 volatile 연산 — `unsafe` 코드에서만 가능
use std::ptr;

const GPIO_REG: *mut u32 = 0x4002_0000 as *mut u32;

unsafe {
    ptr::write_volatile(GPIO_REG, 0x01);    // 쓰기가 최적화로 제거되지 않음
    let val = ptr::read_volatile(GPIO_REG); // 읽기가 최적화로 제거되지 않음
}
```

**동시성에서 공유되는 상태**라는 또 다른 흔한 `volatile` 사용처에서는, Rust는 atomics를 사용합니다:

```cpp
// C++: `volatile`만으로는 스레드 안전성이 보장되지 않음(흔한 실수!)
volatile bool stop_flag = false;  // ❌ 데이터 레이스 — C++11+에서는 UB

// 올바른 C++:
std::atomic<bool> stop_flag{false};
```

```rust
// Rust: 스레드 사이에서 변경 가능한 상태를 공유하는 유일한 방법은 atomics
use std::sync::atomic::{AtomicBool, Ordering};

static STOP_FLAG: AtomicBool = AtomicBool::new(false);

// 다른 스레드에서:
STOP_FLAG.store(true, Ordering::Release);

// 확인:
if STOP_FLAG.load(Ordering::Acquire) {
    println!("Stopping");
}
```

| C++ 사용 방식 | Rust 대응 | 비고 |
|-----------|----------------|-------|
| 하드웨어 레지스터용 `volatile` | `ptr::read_volatile` / `ptr::write_volatile` | `unsafe`가 필요하지만 MMIO에는 정확한 선택 |
| 스레드 신호 전달용 `volatile` | `AtomicBool` / `AtomicU32` 등 | 이 용도에서는 C++의 `volatile`도 잘못된 선택 |
| `std::atomic<T>` | `std::sync::atomic::AtomicT` | 같은 의미론, 같은 메모리 순서 |
| `std::atomic<T>::load(memory_order_acquire)` | `AtomicT::load(Ordering::Acquire)` | 1:1 매핑 |

---

<a id="static-variables-static-const-lazylock-oncelock"></a>
### `static` 변수 → `static`, `const`, `LazyLock`, `OnceLock`

<a id="basic-static-and-const"></a>
#### 기본 `static`과 `const`

```cpp
// C++
const int MAX_RETRIES = 5;                    // 컴파일 타임 상수
static std::string CONFIG_PATH = "/etc/app";  // 정적 초기화 — 순서가 정의되지 않음!
```

```rust
// Rust
const MAX_RETRIES: u32 = 5;                   // 컴파일 타임 상수, 인라인됨
static CONFIG_PATH: &str = "/etc/app";        // `'static` 라이프타임, 고정 주소
```

<a id="the-static-initialization-order-fiasco"></a>
#### 정적 초기화 순서 문제

C++에는 잘 알려진 문제가 있습니다. 서로 다른 번역 단위에 있는 전역 생성자들이 **정해지지 않은 순서**로 실행됩니다. Rust는 이 문제를 아예 피합니다. `static` 값은 컴파일 타임 상수여야 하므로 생성자가 없습니다.

런타임에 초기화되는 전역값이 필요하다면 `LazyLock`(Rust 1.80+)이나 `OnceLock`을 사용하세요:

```rust
use std::sync::LazyLock;

// C++의 `static std::regex`에 해당 — 첫 접근 시 초기화되며, 스레드 안전
static CONFIG_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"^[a-z]+_diag$").expect("invalid regex")
});

fn is_valid_diag(name: &str) -> bool {
    CONFIG_REGEX.is_match(name)  // 첫 호출에서 초기화, 이후 호출은 빠름
}
```

```rust
use std::sync::OnceLock;

// OnceLock: 한 번만 초기화되며, 런타임 데이터를 이용해 설정 가능
static DB_CONN: OnceLock<String> = OnceLock::new();

fn init_db(connection_string: &str) {
    DB_CONN.set(connection_string.to_string())
        .expect("DB_CONN already initialized");
}

fn get_db() -> &'static str {
    DB_CONN.get().expect("DB not initialized")
}
```

| C++ | Rust | 비고 |
|-----|------|-------|
| `const int X = 5;` | `const X: i32 = 5;` | 둘 다 컴파일 타임. Rust는 타입 표기가 필요 |
| `constexpr int X = 5;` | `const X: i32 = 5;` | Rust의 `const`는 항상 constexpr |
| `static int count = 0;` (파일 스코프) | `static COUNT: AtomicI32 = AtomicI32::new(0);` | 변경 가능한 static은 `unsafe`나 atomics가 필요 |
| `static std::string s = "hi";` | `static S: &str = "hi";` 또는 `LazyLock<String>` | 단순한 경우 런타임 생성자 불필요 |
| `static MyObj obj;` (복잡한 초기화) | `static OBJ: LazyLock<MyObj> = LazyLock::new(\|\| { ... });` | 스레드 안전, 지연 초기화, 초기화 순서 문제 없음 |
| `thread_local` | `thread_local! { static X: Cell<u32> = Cell::new(0); }` | 의미론 동일 |

---

<a id="constexpr-const-fn"></a>
### `constexpr` → `const fn`

C++의 `constexpr`는 함수와 변수를 컴파일 타임 평가 대상으로 지정합니다. Rust는 같은 목적에 `const fn`과 `const`를 사용합니다:

```cpp
// C++
constexpr int factorial(int n) {
    return n <= 1 ? 1 : n * factorial(n - 1);
}
constexpr int val = factorial(5);  // 컴파일 타임에 계산됨 → 120
```

```rust
// Rust
const fn factorial(n: u32) -> u32 {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}
const VAL: u32 = factorial(5);  // 컴파일 타임에 계산됨 → 120

// 배열 크기와 match 패턴에도 사용 가능:
const LOOKUP: [u32; 5] = [factorial(1), factorial(2), factorial(3),
                           factorial(4), factorial(5)];
```

| C++ | Rust | 비고 |
|-----|------|-------|
| `constexpr int f()` | `const fn f() -> i32` | 의도는 동일 — 컴파일 타임 평가 가능 |
| `constexpr` 변수 | `const` 변수 | Rust의 `const`는 항상 컴파일 타임 |
| `consteval` (C++20) | 대응 없음 | `const fn`은 런타임에도 실행 가능 |
| `if constexpr` (C++17) | 직접 대응 없음 (`cfg!`나 제네릭 사용) | 일부 경우는 트레잇 specialization이 대신함 |
| `constinit` (C++20) | const 초기화자가 있는 `static` | Rust의 `static`은 기본적으로 const 초기화여야 함 |

> **`const fn`의 현재 제한 사항** (Rust 1.82 기준으로 안정화된 내용):
> - 트레잇 메서드는 사용할 수 없음(const 컨텍스트에서 `Vec`에 `.len()` 호출 불가)
> - 힙 할당 불가(`Box::new`, `Vec::new`는 const 아님)
> - ~~부동소수점 연산 불가~~ — **Rust 1.82에서 안정화**
> - `for` 루프를 사용할 수 없음(재귀나 수동 인덱스를 둔 `while` 사용)

---

<a id="sfinae-and-enable-if-trait-bounds-and-where-clauses"></a>
### SFINAE와 `enable_if` → 트레잇 바운드와 `where` 절

C++에서 SFINAE(Substitution Failure Is Not An Error)는 조건부 제네릭 프로그래밍의 핵심 메커니즘입니다. 강력하지만 읽기 어렵기로 악명 높습니다. Rust는 이를 **트레잇 바운드**로 완전히 대체합니다:

```cpp
// C++: SFINAE 기반 조건부 함수(C++20 이전)
template<typename T,
         std::enable_if_t<std::is_integral_v<T>, int> = 0>
T double_it(T val) { return val * 2; }

template<typename T,
         std::enable_if_t<std::is_floating_point_v<T>, int> = 0>
T double_it(T val) { return val * 2.0; }

// C++20 concepts — 더 깔끔하지만 여전히 장황함
template<std::integral T>
T double_it(T val) { return val * 2; }
```

```rust
// Rust: 트레잇 바운드 — 읽기 쉽고, 조합 가능하며, 에러 메시지도 뛰어남
use std::ops::Mul;

fn double_it<T: Mul<Output = T> + From<u8>>(val: T) -> T {
    val * T::from(2)
}

// 복잡한 제약에는 where 절 사용:
fn process<T>(val: T) -> String
where
    T: std::fmt::Display + Clone + Send,
{
    format!("Processing: {}", val)
}

// 별도의 impl로 조건부 동작 구현(SFINAE 오버로드 세트를 대체)
trait Describable {
    fn describe(&self) -> String;
}

impl Describable for u32 {
    fn describe(&self) -> String { format!("integer: {self}") }
}

impl Describable for f64 {
    fn describe(&self) -> String { format!("float: {self:.2}") }
}
```

| C++ 템플릿 메타프로그래밍 | Rust 대응 | 가독성 |
|-----------------------------|----------------|-------------|
| `std::enable_if_t<cond>` | `where T: Trait` | 🟢 영어처럼 읽힘 |
| `std::is_integral_v<T>` | 숫자 트레잇이나 구체 타입에 대한 바운드 | 🟢 `_v` / `_t` 접미사 없음 |
| SFINAE 오버로드 세트 | 별도의 `impl Trait for ConcreteType` 블록 | 🟢 각 impl이 독립적으로 읽힘 |
| `if constexpr (std::is_same_v<T, int>)` | 트레잇 impl 기반 specialization | 🟢 컴파일 타임 디스패치 |
| C++20 `concept` | `trait` | 🟢 의도가 거의 동일 |
| `requires` 절 | `where` 절 | 🟢 위치와 문법이 비슷 |
| 템플릿 깊숙한 곳에서 컴파일 실패 | 호출 지점에서 트레잇 불일치로 컴파일 실패 | 🟢 200줄짜리 오류 연쇄가 없음 |

> **핵심 통찰**: C++20 concept는 Rust 트레잇에 가장 가까운 개념입니다. C++20 concept에 익숙하다면, Rust 트레잇을 "1.0부터 일급 언어 기능으로 제공된 concept"라고 생각하면 됩니다. 단순한 duck typing이 아니라, 일관된 구현 모델(트레잇 impl)까지 포함합니다.

---

<a id="std-function-function-pointers-impl-fn-and-box-dyn-fn"></a>
### `std::function` → 함수 포인터, `impl Fn`, `Box<dyn Fn>`

C++의 `std::function<R(Args...)>`는 타입 소거된 호출 가능 객체입니다. Rust에는 서로 다른 트레이드오프를 가진 세 가지 선택지가 있습니다:

```cpp
// C++: 만능 해법(힙 할당, 타입 소거)
#include <functional>
std::function<int(int)> make_adder(int n) {
    return [n](int x) { return x + n; };
}
```

```rust
// Rust 옵션 1: fn 포인터 — 단순하고, 캡처가 없으며, 할당이 없음
fn add_one(x: i32) -> i32 { x + 1 }
let f: fn(i32) -> i32 = add_one;
println!("{}", f(5)); // 6

// Rust 옵션 2: impl Fn — 모노모피제이션, 제로 오버헤드, 캡처 가능
fn apply(val: i32, f: impl Fn(i32) -> i32) -> i32 { f(val) }
let n = 10;
let result = apply(5, |x| x + n);  // 클로저가 `n`을 캡처

// Rust 옵션 3: Box<dyn Fn> — 타입 소거, 힙 할당(`std::function`과 유사)
fn make_adder(n: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + n)
}
let adder = make_adder(10);
println!("{}", adder(5));  // 15

// 서로 다른 callable 저장(`vector<function<int(int)>>`와 유사):
let callbacks: Vec<Box<dyn Fn(i32) -> i32>> = vec![
    Box::new(|x| x + 1),
    Box::new(|x| x * 2),
    Box::new(make_adder(100)),
];
for cb in &callbacks {
    println!("{}", cb(5));  // 6, 10, 105
}
```

| 사용할 때 | C++ 대응 | Rust 선택 |
|------------|---------------|-------------|
| 최상위 함수, 캡처 없음 | 함수 포인터 | `fn(Args) -> Ret` |
| callable을 받는 제네릭 함수 | 템플릿 매개변수 | `impl Fn(Args) -> Ret` (정적 디스패치) |
| 제네릭의 트레잇 바운드 | `template<typename F>` | `F: Fn(Args) -> Ret` |
| 저장되는 callable, 타입 소거 필요 | `std::function<R(Args)>` | `Box<dyn Fn(Args) -> Ret>` |
| 상태를 변경하는 콜백 | 가변 람다를 담은 `std::function` | `Box<dyn FnMut(Args) -> Ret>` |
| 한 번만 호출되는 콜백(소모됨) | 이동된 `std::function` | `Box<dyn FnOnce(Args) -> Ret>` |

> **성능 참고**: `impl Fn`은 제로 오버헤드입니다(모노모피제이션되어 C++ 템플릿과 비슷함). `Box<dyn Fn>`은 `std::function`과 같은 수준의 오버헤드(vtable + 힙 할당)가 있습니다. 서로 다른 callable을 저장해야 할 때가 아니라면 `impl Fn`을 우선하세요.

---

<a id="container-mapping-cpp-stl-rust-std-collections"></a>
### 컨테이너 매핑: C++ STL → Rust `std::collections`

| C++ STL 컨테이너 | Rust 대응 | 비고 |
|------------------|----------------|-------|
| `std::vector<T>` | `Vec<T>` | API가 거의 동일. Rust는 기본적으로 경계 검사 수행 |
| `std::array<T, N>` | `[T; N]` | 스택에 할당되는 고정 크기 배열 |
| `std::deque<T>` | `std::collections::VecDeque<T>` | 링 버퍼. 양끝 push/pop이 효율적 |
| `std::list<T>` | `std::collections::LinkedList<T>` | Rust에서는 거의 쓰지 않음 — 대부분 `Vec`가 더 빠름 |
| `std::forward_list<T>` | 대응 없음 | `Vec`나 `VecDeque` 사용 |
| `std::unordered_map<K, V>` | `std::collections::HashMap<K, V>` | 기본 해시로 `SipHash` 사용(DoS 저항) |
| `std::map<K, V>` | `std::collections::BTreeMap<K, V>` | B-tree 기반, 키 정렬됨, `K: Ord` 필요 |
| `std::unordered_set<T>` | `std::collections::HashSet<T>` | `T: Hash + Eq` 필요 |
| `std::set<T>` | `std::collections::BTreeSet<T>` | 정렬 집합, `T: Ord` 필요 |
| `std::priority_queue<T>` | `std::collections::BinaryHeap<T>` | 기본이 max-heap(C++와 동일) |
| `std::stack<T>` | `.push()` / `.pop()`을 사용하는 `Vec<T>` | 별도 stack 타입 불필요 |
| `std::queue<T>` | `.push_back()` / `.pop_front()`를 사용하는 `VecDeque<T>` | 별도 queue 타입 불필요 |
| `std::string` | `String` | UTF-8이 보장되며, null 종료 문자열이 아님 |
| `std::string_view` | `&str` | 빌린 UTF-8 슬라이스 |
| `std::span<T>` (C++20) | `&[T]` / `&mut [T]` | Rust 슬라이스는 1.0부터 일급 타입 |
| `std::tuple<A, B, C>` | `(A, B, C)` | 문법 차원에서 지원되며 구조 분해 가능 |
| `std::pair<A, B>` | `(A, B)` | 그냥 2원소 튜플 |
| `std::bitset<N>` | std 대응 없음 | `bitvec` crate 또는 `[u8; N/8]` 사용 |

**핵심 차이점**:
- Rust의 `HashMap`/`HashSet`은 `K: Hash + Eq`를 요구합니다. C++처럼 해시 불가능한 키를 썼을 때 STL 깊숙한 곳에서 템플릿 오류가 터지는 대신, Rust는 컴파일러가 이를 타입 수준에서 강제합니다.
- `Vec` 인덱싱(`v[i]`)은 기본적으로 범위를 벗어나면 panic합니다. `Option<&T>`를 반환하는 `.get(i)`나, 경계 검사 자체를 없애는 iterator를 사용하세요.
- `std::multimap`이나 `std::multiset`은 없습니다. `HashMap<K, Vec<V>>` 또는 `BTreeMap<K, Vec<V>>`를 사용하세요.

---

<a id="exception-safety-panic-safety"></a>
### 예외 안전성 → 패닉 안전성

C++는 예외 안전성을 세 단계(Abrahams guarantees)로 나눕니다:

| C++ 수준 | 의미 | Rust 대응 |
|----------|---------|----------------|
| **No-throw** | 함수가 절대 throw하지 않음 | 함수가 panic하지 않음(`Result`를 반환) |
| **Strong** (commit-or-rollback) | throw되더라도 상태는 바뀌지 않음 | 소유권 모델 덕분에 자연스럽게 달성됨 — `?`로 조기 반환되면 부분적으로 만들어진 값은 drop됨 |
| **Basic** | throw되더라도 불변식은 유지됨 | Rust의 기본값 — `Drop`이 실행되고, 누수 없음 |

<a id="how-rusts-ownership-model-helps"></a>
#### Rust의 소유권 모델이 주는 이점

```rust
// Strong guarantee가 사실상 공짜 — file.write()가 실패해도 config는 바뀌지 않음
fn update_config(config: &mut Config, path: &str) -> Result<(), Error> {
    let new_data = fetch_from_network()?; // Err → 조기 반환, config는 그대로
    let validated = validate(new_data)?;  // Err → 조기 반환, config는 그대로
    *config = validated;                  // 성공했을 때만 실행(commit)
    Ok(())
}
```

C++에서 strong guarantee를 달성하려면 수동 rollback이나 copy-and-swap 관용구가 필요합니다. Rust에서는 대부분의 코드에서 `?` 전파만으로 strong guarantee를 기본적으로 얻게 됩니다.

<a id="catch-unwind-rusts-equivalent-of-catch"></a>
#### `catch_unwind` — Rust에서 `catch(...)`에 해당하는 것

```rust
use std::panic;

// panic 잡기(C++의 catch(...)와 유사) — 정말 필요한 경우는 드묾
let result = panic::catch_unwind(|| {
    // panic할 수 있는 코드
    let v = vec![1, 2, 3];
    v[10]  // Panic! (인덱스 범위 초과)
});

match result {
    Ok(val) => println!("Got: {val}"),
    Err(_) => eprintln!("Caught a panic — cleaned up"),
}
```

<a id="unwindsafe-marking-types-as-panic-safe"></a>
#### `UnwindSafe` — 타입이 패닉에 안전함을 표시하기

```rust
use std::panic::UnwindSafe;

// &mut 뒤에 있는 타입은 기본적으로 UnwindSafe가 아님 — panic이
// 중간 수정 상태를 남겼을 수 있기 때문
fn safe_execute<F: FnOnce() + UnwindSafe>(f: F) {
    let _ = std::panic::catch_unwind(f);
}

// 코드를 검토해 안전함을 확인했다면 AssertUnwindSafe로 덮어쓸 수 있음:
use std::panic::AssertUnwindSafe;
let mut data = vec![1, 2, 3];
let _ = std::panic::catch_unwind(AssertUnwindSafe(|| {
    data.push(4);
}));
```

| C++ 예외 패턴 | Rust 대응 |
|-----------------------|-----------------|
| `throw MyException()` | `return Err(MyError::...)`(권장) 또는 `panic!("...")` |
| `try { } catch (const E& e)` | `match result { Ok(v) => ..., Err(e) => ... }` 또는 `?` |
| `catch (...)` | `std::panic::catch_unwind(...)` |
| `noexcept` | `-> Result<T, E>`(에러는 예외가 아니라 값) |
| 스택 언와인딩 중 RAII cleanup | panic 언와인딩 중에도 `Drop::drop()` 실행 |
| `std::uncaught_exceptions()` | `std::thread::panicking()` |
| `-fno-exceptions` 컴파일 플래그 | `Cargo.toml`의 `[profile]`에서 `panic = "abort"` |

> **핵심 요약**: Rust에서는 대부분의 코드가 예외 대신 `Result<T, E>`를 사용하므로, 에러 경로가 명시적이고 조합 가능합니다. `panic!`은 일상적인 오류가 아니라 버그(`assert!` 실패 같은 경우)를 위한 것입니다. 그래서 "예외 안전성"은 대체로 큰 이슈가 아닙니다. 정리(cleanup)는 소유권 시스템이 자동으로 처리합니다.

---

<a id="cpp-to-rust-migration-patterns"></a>
## C++에서 Rust로의 마이그레이션 패턴

<a id="quick-reference-cpp-rust-idiom-map"></a>
### 빠른 참조: C++ → Rust 관용구 매핑

| **C++ 패턴** | **Rust 관용구** | **비고** |
|----------------|---------------|----------|
| `class Derived : public Base` | `enum Variant { A {...}, B {...} }` | 닫힌 집합에는 enum을 우선 |
| `virtual void method() = 0` | `trait MyTrait { fn method(&self); }` | 열려 있거나 확장 가능한 인터페이스에 사용 |
| `dynamic_cast<Derived*>(ptr)` | `match value { Variant::A(data) => ..., }` | exhaustiveness가 보장되며 런타임 실패 없음 |
| `vector<unique_ptr<Base>>` | `Vec<Box<dyn Trait>>` | 진짜 다형성이 필요할 때만 |
| `shared_ptr<T>` | `Rc<T>` 또는 `Arc<T>` | 먼저 `Box<T>`나 소유 값으로 해결 가능한지 확인 |
| `enable_shared_from_this<T>` | arena 패턴(`Vec<T>` + 인덱스) | 참조 사이클을 아예 없앰 |
| 모든 클래스에 `Base* m_pFramework` 저장 | `fn execute(&mut self, ctx: &mut Context)` | 포인터를 저장하지 말고 context를 전달 |
| `try { } catch (...) { }` | `match result { Ok(v) => ..., Err(e) => ... }` | 또는 `?`로 전파 |
| `std::optional<T>` | `Option<T>` | `match`를 강제하므로 `None`을 잊을 수 없음 |
| `const std::string&` 매개변수 | `&str` 매개변수 | `String`과 `&str` 모두 받을 수 있음 |
| `enum class Foo { A, B, C }` | `enum Foo { A, B, C }` | Rust enum은 데이터도 함께 담을 수 있음 |
| `auto x = std::move(obj)` | `let x = obj;` | move가 기본값이므로 `std::move` 불필요 |
| CMake + make + lint | `cargo build / test / clippy / fmt` | 하나의 도구로 일관되게 처리 |

<a id="migration-strategy"></a>
### 마이그레이션 전략

1. **데이터 타입부터 시작하세요**: 먼저 struct와 enum을 옮기면 소유권을 어떻게 설계할지 강제로 고민하게 됩니다.
2. **팩토리를 enum으로 바꾸세요**: 팩토리가 서로 다른 파생 타입을 만든다면, 대개 `enum` + `match`가 더 적합합니다.
3. **god object를 합성된 struct로 바꾸세요**: 관련 필드를 관심사별로 묶어 더 작은 struct로 나누세요.
4. **포인터를 borrow로 바꾸세요**: 저장된 `Base*` 포인터를 라이프타임이 있는 `&'a T` borrow로 전환하세요.
5. **`Box<dyn Trait>`는 아껴 쓰세요**: 플러그인 시스템과 테스트 mocking 같은 경우에만 사용하세요.
6. **컴파일러의 안내를 따르세요**: Rust의 오류 메시지는 매우 좋습니다. 천천히 읽어 보면 설계 방향까지 알려줍니다.
