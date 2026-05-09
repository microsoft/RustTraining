## Rust 매크로: 전처리기에서 메타프로그래밍까지

> **이 장에서 배우는 것:** Rust 매크로가 어떻게 동작하는지, 함수나 제네릭 대신 언제 매크로를 써야 하는지, 그리고 매크로가 C/C++ 전처리기를 어떻게 대체하는지 배웁니다. 이 장을 마치면 직접 `macro_rules!` 매크로를 작성할 수 있고, `#[derive(Debug)]`가 내부적으로 무엇을 하는지도 이해하게 됩니다.

매크로는 Rust에서 가장 먼저 마주치는 요소 중 하나입니다(`println!("hello")`를 첫 줄에서 보게 되죠). 하지만 대부분의 강의가 가장 늦게 설명하는 주제이기도 합니다. 이 장에서는 그 공백을 메웁니다.

### 왜 매크로가 존재할까

함수와 제네릭만으로도 Rust의 대부분의 코드 재사용은 해결됩니다. 매크로는 타입 시스템만으로는 닿지 않는 빈틈을 메웁니다:

| 필요한 것 | 함수/제네릭으로 해결? | 매크로? | 이유 |
|------|-------------------|--------|-----|
| 값 계산 | ✅ `fn max<T: Ord>(a: T, b: T) -> T` | — | 타입 시스템으로 해결 가능 |
| 가변 개수의 인자 받기 | ❌ Rust에는 가변 인자 함수가 없음 | ✅ `println!("{} {}", a, b)` | 매크로는 토큰을 원하는 개수만큼 받을 수 있음 |
| 반복적인 `impl` 블록 생성 | ❌ 제네릭만으로는 불가능 | ✅ `macro_rules!` | 매크로가 컴파일 타임에 코드를 생성 |
| 컴파일 타임에 코드 실행 | ❌ `const fn`에는 제약이 많음 | ✅ 프로시저 매크로 | 컴파일 타임에 전체 Rust 코드를 실행 가능 |
| 조건부로 코드 포함 | ❌ | ✅ `#[cfg(...)]` | attribute 매크로가 컴파일 여부를 제어 |

C/C++에서 왔다면, 매크로를 *전처리기를 대체하는 유일하게 올바른 수단*이라고 생각하면 됩니다. 차이는 Rust 매크로가 생 텍스트가 아니라 구문 트리 위에서 동작한다는 점입니다. 그래서 위생적이며(hygienic, 우발적인 이름 충돌이 없음), 타입도 인지합니다.

> **C 개발자를 위한 참고:** Rust 매크로는 `#define`을 완전히 대체합니다. 텍스트 치환형 전처리기는 없습니다. 전처리기와 Rust의 전체 대응표는 [ch18](ch18-cpp-rust-semantic-deep-dives.md)에서 확인하세요.

---

<a id="declarative-macros-with-macro_rules"></a>
## 선언적 매크로와 `macro_rules!`

선언적 매크로("macros by example"이라고도 부름)는 Rust에서 가장 흔한 매크로 형태입니다. 값에 대해 `match`를 쓰듯이, 문법 패턴에 대해 매칭합니다.

### 기본 문법

```rust
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

fn main() {
    say_hello!();  // 다음으로 확장된다: println!("Hello!");
}
```

이름 뒤의 `!`는 이것이 매크로 호출이라는 사실을 사용자와 컴파일러 모두에게 알려줍니다.

### 인자를 받는 패턴 매칭

매크로는 프래그먼트 지정자(fragment specifier)를 사용해 *토큰 트리(token tree)*에 매칭합니다:

```rust
macro_rules! greet {
    // 패턴 1: 인자 없음
    () => {
        println!("Hello, world!");
    };
    // 패턴 2: 표현식 인자 하나
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

fn main() {
    greet!();           // "Hello, world!" 출력
    greet!("Rust");     // "Hello, Rust!" 출력
}
```

#### 프래그먼트 지정자 요약

| 지정자 | 매칭 대상 | 예시 |
|-----------|---------|---------|
| `$x:expr` | 임의의 표현식 | `42`, `a + b`, `foo()` |
| `$x:ty` | 타입 | `i32`, `Vec<String>`, `&str` |
| `$x:ident` | 식별자 | `foo`, `my_var` |
| `$x:pat` | 패턴 | `Some(x)`, `_`, `(a, b)` |
| `$x:stmt` | 문장 | `let x = 5;` |
| `$x:block` | 블록 | `{ println!("hi"); 42 }` |
| `$x:literal` | 리터럴 | `42`, `"hello"`, `true` |
| `$x:tt` | 단일 토큰 트리 | 무엇이든 가능 - 와일드카드 |
| `$x:item` | 아이템(fn, struct, impl 등) | `fn foo() {}` |

### 반복 - 매크로의 핵심 기능

C/C++ 매크로는 반복할 수 없습니다. Rust 매크로는 패턴 반복이 가능합니다:

```rust
macro_rules! make_vec {
    // 쉼표로 구분된 표현식을 0개 이상 매칭
    ( $( $element:expr ),* ) => {
        {
            let mut v = Vec::new();
            $( v.push($element); )*  // 매칭된 각 요소마다 반복
            v
        }
    };
}

fn main() {
    let v = make_vec![1, 2, 3, 4, 5];
    println!("{v:?}");  // [1, 2, 3, 4, 5]
}
```

`$( ... ),*` 문법은 "이 패턴을 쉼표로 구분해 0개 이상 매칭하라"는 뜻입니다. 확장 쪽의 `$( ... )*`는 매칭된 항목마다 본문을 한 번씩 반복합니다.

> **표준 라이브러리의 `vec![]`도 정확히 이런 방식으로 구현됩니다.** 실제 소스는 다음과 같습니다:
> ```rust
> macro_rules! vec {
>     () => { Vec::new() };
>     ($elem:expr; $n:expr) => { vec::from_elem($elem, $n) };
>     ($($x:expr),+ $(,)?) => { <[_]>::into_vec(Box::new([$($x),+])) };
> }
> ```
> 끝의 `$(,)?`는 마지막 쉼표를 선택적으로 허용합니다.

#### 반복 연산자

| 연산자 | 의미 | 예시 |
|----------|---------|---------|
| `$( ... )*` | 0개 이상 | `vec![]`, `vec![1]`, `vec![1, 2, 3]` |
| `$( ... )+` | 1개 이상 | 최소 1개 요소 필요 |
| `$( ... )?` | 0개 또는 1개 | 선택적 요소 |

### 실용 예제: `hashmap!` 생성자

표준 라이브러리에는 `vec![]`는 있지만 `hashmap!{}`는 없습니다. 직접 만들어 봅시다:

```rust
macro_rules! hashmap {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        {
            let mut map = std::collections::HashMap::new();
            $( map.insert($key, $value); )*
            map
        }
    };
}

fn main() {
    let scores = hashmap! {
        "Alice" => 95,
        "Bob" => 87,
        "Carol" => 92,  // $(,)? 덕분에 마지막 쉼표 허용
    };
    println!("{scores:?}");
}
```

### 실용 예제: 진단 체크 매크로

임베디드/진단 코드에서 흔히 쓰는 패턴입니다. 조건을 검사하고 실패하면 에러를 반환합니다:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum DiagError {
    #[error("Check failed: {0}")]
    CheckFailed(String),
}

macro_rules! diag_check {
    ($cond:expr, $msg:expr) => {
        if !($cond) {
            return Err(DiagError::CheckFailed($msg.to_string()));
        }
    };
}

fn run_diagnostics(temp: f64, voltage: f64) -> Result<(), DiagError> {
    diag_check!(temp < 85.0, "GPU too hot");
    diag_check!(voltage > 0.8, "Rail voltage too low");
    diag_check!(voltage < 1.5, "Rail voltage too high");
    println!("All checks passed");
    Ok(())
}
```

> **C/C++ 비교:**
> ```c
> // C 전처리기 - 텍스트 치환, 타입 안전성 없음, hygiene 없음
> #define DIAG_CHECK(cond, msg) \
>     do { if (!(cond)) { log_error(msg); return -1; } } while(0)
> ```
> Rust 버전은 올바른 `Result` 타입을 반환하고, 이중 평가(double evaluation) 위험이 없으며, 컴파일러가 `$cond`가 실제로 `bool` 표현식인지까지 검사해 줍니다.

### 위생성(hygiene): Rust 매크로가 안전한 이유

C/C++ 매크로 버그는 이름 충돌에서 시작되는 경우가 많습니다:

```c
// C: 위험함 - `x`가 호출자 쪽 `x`를 가릴 수 있다
#define SQUARE(x) ((x) * (x))
int x = 5;
int result = SQUARE(x++);  // UB: x가 두 번 증가한다!
```

Rust 매크로는 **위생적(hygienic)** 입니다. 매크로 내부에서 만든 변수는 바깥으로 새지 않습니다:

```rust
macro_rules! make_x {
    () => {
        let x = 42;  // 이 `x`는 매크로 확장 범위 안에만 있다
    };
}

fn main() {
    let x = 10;
    make_x!();
    println!("{x}");  // 10이 출력된다. 42가 아니다 - hygiene이 충돌을 막는다
}
```

매크로 안의 `x`와 호출자 쪽 `x`는 이름이 같아 보여도 컴파일러가 서로 다른 변수로 취급합니다. **이것은 C 전처리기로는 불가능합니다.**

---

<a id="common-standard-library-macros"></a>
## 표준 라이브러리의 공통 매크로

1장부터 계속 써 왔지만, 실제로 무엇을 하는지 정리하면 다음과 같습니다:

| 매크로 | 하는 일 | 대략적인 확장 결과 |
|-------|-------------|------------------------|
| `println!("{}", x)` | 포맷 후 stdout에 출력하고 개행 추가 | `std::io::_print(format_args!(...))` |
| `eprintln!("{}", x)` | stderr에 출력하고 개행 추가 | stderr용으로 동일 |
| `format!("{}", x)` | 포맷 결과를 `String`으로 생성 | 할당 후 `String` 반환 |
| `vec![1, 2, 3]` | 요소가 들어 있는 `Vec` 생성 | `Vec::from([1, 2, 3])` (대략) |
| `todo!()` | 미완성 코드를 표시 | `panic!("not yet implemented")` |
| `unimplemented!()` | 의도적으로 구현하지 않은 코드를 표시 | `panic!("not implemented")` |
| `unreachable!()` | 컴파일러가 도달 불가능하다고 증명하지 못한 코드를 표시 | `panic!("unreachable")` |
| `assert!(cond)` | 조건이 거짓이면 panic | `if !cond { panic!(...) }` |
| `assert_eq!(a, b)` | 값이 다르면 panic | 실패 시 두 값을 함께 보여줌 |
| `dbg!(expr)` | 표현식과 값을 stderr에 출력하고 그 값을 반환 | `eprintln!("[file:line] expr = {:#?}", &expr); expr` |
| `include_str!("file.txt")` | 컴파일 타임에 파일 내용을 `&str`로 포함 | 컴파일 중 파일을 읽음 |
| `include_bytes!("data.bin")` | 컴파일 타임에 파일 내용을 `&[u8]`로 포함 | 컴파일 중 파일을 읽음 |
| `cfg!(condition)` | 컴파일 타임 조건을 `bool` 값으로 평가 | 타깃에 따라 `true` 또는 `false` |
| `env!("VAR")` | 컴파일 타임에 환경 변수 읽기 | 설정되어 있지 않으면 컴파일 실패 |
| `concat!("a", "b")` | 컴파일 타임에 리터럴을 이어 붙임 | `"ab"` |

### `dbg!` - 매일 쓰게 될 디버깅 매크로

```rust
fn factorial(n: u32) -> u32 {
    if dbg!(n <= 1) {     // 출력: [src/main.rs:2] n <= 1 = false
        dbg!(1)           // 출력: [src/main.rs:3] 1 = 1
    } else {
        dbg!(n * factorial(n - 1))  // 중간 값을 출력
    }
}

fn main() {
    dbg!(factorial(4));   // 모든 재귀 호출을 file:line과 함께 출력
}
```

`dbg!`는 감싼 값을 그대로 반환하므로, 프로그램 동작을 바꾸지 않고 거의 어디에나 끼워 넣을 수 있습니다. 출력은 stdout이 아니라 stderr로 나가기 때문에 정상 프로그램 출력과도 섞이지 않습니다. **코드를 커밋하기 전에는 모든 `dbg!` 호출을 제거하세요.**

### 포맷 문자열 문법

`println!`, `format!`, `eprintln!`, `write!`는 모두 같은 포맷팅 엔진을 쓰므로, 여기서 핵심 문법만 빠르게 정리합니다:

```rust
let name = "sensor";
let value = 3.14159;
let count = 42;

println!("{name}");                    // 이름으로 변수 참조(Rust 1.58+)
println!("{}", name);                  // 위치 기반
println!("{value:.2}");                // 소수 둘째 자리까지: "3.14"
println!("{count:>10}");               // 오른쪽 정렬, 너비 10: "        42"
println!("{count:0>10}");              // 앞을 0으로 채움: "0000000042"
println!("{count:#06x}");              // 접두사 포함 16진수: "0x002a"
println!("{count:#010b}");             // 접두사 포함 2진수: "0b00101010"
println!("{value:?}");                 // Debug 포맷
println!("{value:#?}");                // 사람이 읽기 쉬운 Debug 포맷
```

> **C 개발자를 위한 참고:** 이것은 타입 안전한 `printf`라고 생각하면 됩니다. 컴파일러가 `{:.2}`가 문자열이 아니라 부동소수점 값에 적용되는지 검사해 줍니다. `%s`/`%d` 불일치 버그가 없습니다.
>
> **C++ 개발자를 위한 참고:** `std::cout << std::fixed << std::setprecision(2) << value`를 하나의 읽기 쉬운 포맷 문자열로 대체한 것에 가깝습니다.

---

<a id="derive-macros"></a>
## derive 매크로

이 책에서 거의 모든 구조체 위에서 `#[derive(...)]`를 보셨을 겁니다:

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}
```

`#[derive(Debug)]`는 **derive 매크로**입니다. 즉, 트레잇 구현을 자동으로 생성하는 특별한 프로시저 매크로입니다. 대략 다음과 같은 코드를 만들어 냅니다:

```rust
// #[derive(Debug)]가 Point에 대해 생성하는 코드:
impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}
```

`#[derive(Debug)]`가 없다면, 구조체마다 저 `impl` 블록을 손으로 직접 써야 합니다.

### 자주 derive하는 트레잇

| derive | 생성되는 것 | 언제 쓰나 |
|--------|-------------------|-------------|
| `Debug` | `{:?}` 포맷팅 | 거의 항상 - 디버깅 출력을 가능하게 함 |
| `Clone` | `.clone()` 메서드 | 값을 복제해야 할 때 |
| `Copy` | 대입 시 암묵적 복사 | 작고 스택에만 있는 타입(정수, `[f64; 3]` 등) |
| `PartialEq` / `Eq` | `==`와 `!=` 연산자 | 동등 비교가 필요할 때 |
| `PartialOrd` / `Ord` | `<`, `>`, `<=`, `>=` 연산자 | 순서 비교가 필요할 때 |
| `Hash` | `HashMap`/`HashSet` 키용 해시 | 맵 키로 쓰이는 타입 |
| `Default` | `Type::default()` 생성자 | 합리적인 0/빈값이 있는 타입 |
| `serde::Serialize` / `Deserialize` | JSON/TOML 등 직렬화 | API 경계를 넘나드는 데이터 타입 |

### derive 판단 트리

```text
derive를 붙여야 할까?
  │
  ├── 내 타입의 모든 필드가 이 트레잇을 구현하고 있는가?
  │     ├── 예 → #[derive]로 된다
  │     └── 아니오 → 직접 impl을 쓰거나 생략
  │
  └── 이 타입 사용자들이 이런 동작을 자연스럽게 기대할까?
        ├── 예 → derive하자 (Debug, Clone, PartialEq는 거의 늘 무난)
        └── 아니오 → derive하지 말자 (예: 파일 핸들을 가진 타입에 Copy 금지)
```

> **C++ 비교:** `#[derive(Clone)]`는 올바른 복사 생성자를 자동으로 만들어 주는 것과 비슷합니다. `#[derive(PartialEq)]`는 각 필드를 비교하는 `operator==`를 자동 생성하는 것과 비슷한데, 이는 C++20의 `= default` 비교 연산자가 뒤늦게 제공한 기능입니다.

---

<a id="attribute-macros"></a>
## attribute 매크로

attribute 매크로는 자신이 붙은 아이템을 변환합니다. 이미 여러 번 사용했습니다:

```rust
#[test]                    // 함수를 테스트로 표시
fn test_addition() {
    assert_eq!(2 + 2, 4);
}

#[cfg(target_os = "linux")] // 이 함수를 조건부로 포함
fn linux_only() { /* ... */ }

#[derive(Debug)]            // Debug 구현 생성
struct MyType { /* ... */ }

#[allow(dead_code)]         // 컴파일러 경고 억제
fn unused_helper() { /* ... */ }

#[must_use]                 // 반환값을 버리면 경고
fn compute_checksum(data: &[u8]) -> u32 { /* ... */ }
```

자주 쓰는 내장 attribute는 다음과 같습니다:

| Attribute | 목적 |
|-----------|---------|
| `#[test]` | 테스트 함수로 표시 |
| `#[cfg(...)]` | 조건부 컴파일 |
| `#[derive(...)]` | 트레잇 impl 자동 생성 |
| `#[allow(...)]` / `#[deny(...)]` / `#[warn(...)]` | lint 수준 제어 |
| `#[must_use]` | 사용하지 않은 반환값 경고 |
| `#[inline]` / `#[inline(always)]` | 함수 인라인 힌트 |
| `#[repr(C)]` | C 호환 메모리 레이아웃 사용(FFI용) |
| `#[no_mangle]` | 심볼 이름 맹글링 금지(FFI용) |
| `#[deprecated]` | 선택적 메시지와 함께 deprecated 표시 |

> **C/C++ 개발자를 위한 참고:** attribute는 전처리 지시어(`#pragma`), `__attribute__((...))`, 컴파일러별 확장 기능이 섞여 있던 자리를 대체합니다. Rust에서는 이것들이 덧붙인 확장이 아니라 언어 문법 자체의 일부입니다.

---

<a id="procedural-macros-conceptual-overview"></a>
## 프로시저 매크로(개념 개요)

프로시저 매크로("proc macro")는 *별도의 Rust 프로그램*으로 작성되어 컴파일 타임에 실행되고 코드를 생성하는 매크로입니다. `macro_rules!`보다 훨씬 강력하지만, 그만큼 복잡합니다.

종류는 세 가지입니다:

| 종류 | 문법 | 예시 | 하는 일 |
|------|--------|---------|-------------|
| **함수형** | `my_macro!(...)` | `sql!(SELECT * FROM users)` | 커스텀 문법을 파싱해 Rust 코드 생성 |
| **derive** | `#[derive(MyTrait)]` | `#[derive(Serialize)]` | 구조체 정의로부터 트레잇 impl 생성 |
| **attribute** | `#[my_attr]` | `#[tokio::main]`, `#[instrument]` | 붙은 아이템을 변환 |

### 이미 proc macro를 써 봤다

- `thiserror`의 `#[derive(Error)]` - 에러 enum용 `Display`와 `From` impl을 생성합니다
- `serde`의 `#[derive(Serialize, Deserialize)]` - 직렬화 코드를 생성합니다
- `#[tokio::main]` - `async fn main()`을 런타임 설정 + `block_on` 호출로 변환합니다
- `#[test]` - 테스트 하네스가 등록하는 내장 proc macro입니다

### 직접 proc macro를 작성해야 하는 경우

이 과정에서 직접 proc macro를 작성할 일은 아마 많지 않을 겁니다. 다만 다음과 같은 경우에는 유용합니다:
- 컴파일 타임에 구조체 필드나 enum variant를 살펴봐야 할 때(derive 매크로)
- 도메인 특화 언어(DSL)를 만들 때(함수형 매크로)
- 함수 시그니처를 변형해야 할 때(attribute 매크로)

대부분의 경우에는 `macro_rules!`나 일반 함수만으로 충분합니다.

> **C++ 비교:** 프로시저 매크로는 C++에서 코드 생성기, 템플릿 메타프로그래밍, `protoc` 같은 외부 도구가 맡던 역할을 채웁니다. 차이는 proc macro가 Cargo 빌드 파이프라인 안에 들어 있다는 점입니다. 외부 빌드 단계도, CMake custom command도 필요 없습니다.

---

<a id="when-to-use-what-macros-vs-functions-vs-generics"></a>
## 언제 무엇을 써야 하나: 매크로 vs 함수 vs 제네릭

```text
코드를 생성해야 하는가?
  │
  ├── 아니오 → 함수 또는 제네릭 함수를 써라
  │             (더 단순하고, 에러 메시지가 낫고, IDE 지원도 좋다)
  │
  └── 예 ─┬── 인자 개수가 가변적인가?
            │     └── 예 → macro_rules! (예: println!, vec!)
            │
            ├── 많은 타입에 대해 반복적인 impl 블록이 필요한가?
            │     └── 예 → 반복을 사용하는 macro_rules!
            │
            ├── 구조체 필드를 살펴봐야 하는가?
            │     └── 예 → derive 매크로(proc macro)
            │
            ├── 커스텀 문법(DSL)이 필요한가?
            │     └── 예 → 함수형 proc macro
            │
            └── 함수/구조체를 변형해야 하는가?
                  └── 예 → attribute proc macro
```

**일반 원칙:** 함수나 제네릭으로 할 수 있다면 매크로를 쓰지 마세요. 매크로는 에러 메시지가 더 불친절하고, 매크로 본문 안에서는 IDE 자동 완성이 잘 되지 않으며, 디버깅도 더 어렵습니다.

---

<a id="exercises"></a>
## 연습문제

<a id="exercise-1-min-macro"></a>
### 🟢 연습문제 1: `min!` 매크로

다음 조건을 만족하는 `min!` 매크로를 작성하세요:
- `min!(a, b)`는 두 값 중 더 작은 값을 반환
- `min!(a, b, c)`는 세 값 중 가장 작은 값을 반환
- `PartialOrd`를 구현한 어떤 타입에도 동작

**힌트:** `macro_rules!`에 두 개의 match arm이 필요합니다.

<details><summary>해답 (클릭하여 펼치기)</summary>

```rust
macro_rules! min {
    ($a:expr, $b:expr) => {
        if $a < $b { $a } else { $b }
    };
    ($a:expr, $b:expr, $c:expr) => {
        min!(min!($a, $b), $c)
    };
}

fn main() {
    println!("{}", min!(3, 7));        // 3
    println!("{}", min!(9, 2, 5));     // 2
    println!("{}", min!(1.5, 0.3));    // 0.3
}
```

**참고:** 프로덕션 코드에서는 `std::cmp::min`이나 `a.min(b)`를 우선 고려하세요. 이 연습문제의 목적은 여러 arm을 가진 매크로의 동작 방식을 보여 주는 것입니다.

</details>

<a id="exercise-2-hashmap-from-scratch"></a>
### 🟡 연습문제 2: 처음부터 `hashmap!` 만들기

위 예제를 보지 않고, 다음 조건을 만족하는 `hashmap!` 매크로를 작성하세요:
- `key => value` 쌍으로부터 `HashMap`을 생성
- 마지막 쉼표를 지원
- 해시 가능한 어떤 키 타입에도 동작

다음으로 테스트하세요:
```rust
let m = hashmap! {
    "name" => "Alice",
    "role" => "Engineer",
};
assert_eq!(m["name"], "Alice");
assert_eq!(m.len(), 2);
```

<details><summary>해답 (클릭하여 펼치기)</summary>

```rust
use std::collections::HashMap;

macro_rules! hashmap {
    ( $( $key:expr => $val:expr ),* $(,)? ) => {{
        let mut map = HashMap::new();
        $( map.insert($key, $val); )*
        map
    }};
}

fn main() {
    let m = hashmap! {
        "name" => "Alice",
        "role" => "Engineer",
    };
    assert_eq!(m["name"], "Alice");
    assert_eq!(m.len(), 2);
    println!("Tests passed!");
}
```

</details>

<a id="exercise-3-assert_approx_eq-for-floating-point-comparison"></a>
### 🟡 연습문제 3: 부동소수점 비교용 `assert_approx_eq!`

`|a - b| > epsilon`이면 panic하도록 `assert_approx_eq!(a, b, epsilon)` 매크로를 작성하세요. 정확한 동등 비교가 실패하는 부동소수점 계산을 테스트할 때 유용합니다.

다음으로 테스트하세요:
```rust
assert_approx_eq!(0.1 + 0.2, 0.3, 1e-10);        // 성공해야 함
assert_approx_eq!(3.14159, std::f64::consts::PI, 1e-4); // 성공해야 함
// assert_approx_eq!(1.0, 2.0, 0.5);              // panic해야 함
```

<details><summary>해답 (클릭하여 펼치기)</summary>

```rust
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr, $eps:expr) => {
        let (a, b, eps) = ($a as f64, $b as f64, $eps as f64);
        let diff = (a - b).abs();
        if diff > eps {
            panic!(
                "assertion failed: |{} - {}| = {} > {} (epsilon)",
                a, b, diff, eps
            );
        }
    };
}

fn main() {
    assert_approx_eq!(0.1 + 0.2, 0.3, 1e-10);
    assert_approx_eq!(3.14159, std::f64::consts::PI, 1e-4);
    println!("All float comparisons passed!");
}
```

</details>

<a id="exercise-4-impl_display_for_enum"></a>
### 🔴 연습문제 4: `impl_display_for_enum!`

단순한 C 스타일 enum에 대한 `Display` 구현을 생성하는 매크로를 작성하세요. 다음 입력이 주어졌을 때:

```rust
impl_display_for_enum! {
    enum Color {
        Red => "red",
        Green => "green",
        Blue => "blue",
    }
}
```

이 매크로는 `enum Color { Red, Green, Blue }` 정의와, 각 variant를 대응 문자열로 매핑하는 `impl Display for Color`를 모두 생성해야 합니다.

**힌트:** `$( ... ),*` 반복과 여러 프래그먼트 지정자를 함께 써야 합니다.

<details><summary>해답 (클릭하여 펼치기)</summary>

```rust
use std::fmt;

macro_rules! impl_display_for_enum {
    (enum $name:ident { $( $variant:ident => $display:expr ),* $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        enum $name {
            $( $variant ),*
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $( $name::$variant => write!(f, "{}", $display), )*
                }
            }
        }
    };
}

impl_display_for_enum! {
    enum Color {
        Red => "red",
        Green => "green",
        Blue => "blue",
    }
}

fn main() {
    let c = Color::Green;
    println!("Color: {c}");          // "Color: green"
    println!("Debug: {c:?}");        // "Debug: Green"
    assert_eq!(format!("{}", Color::Red), "red");
    println!("All tests passed!");
}
```

</details>
