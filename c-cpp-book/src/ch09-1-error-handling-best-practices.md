<a id="rust-option-and-result-key-takeaways"></a>
# Rust Option과 Result 핵심 요약

> **이 장에서 배우는 것:** 관용적인 에러 처리 패턴, `unwrap()`의 안전한 대안, 에러 전파를 위한 `?` 연산자, 커스텀 에러 타입, 그리고 실제 코드에서 `anyhow`와 `thiserror`를 언제 써야 하는지를 배웁니다.

- `Option`과 `Result`는 관용적인 Rust의 핵심 요소입니다.
- **`unwrap()`의 안전한 대안**:
```rust
// Option<T>의 안전한 대안
let value = opt.unwrap_or(default);               // 대체 값을 제공
let value = opt.unwrap_or_else(|| compute());     // 필요할 때만 대체 값을 계산
let value = opt.unwrap_or_default();              // Default 트레잇 구현 사용
let value = opt.expect("descriptive message");    // panic이 허용될 때만 사용

// Result<T, E>의 안전한 대안
let value = result.unwrap_or(fallback);           // 에러를 무시하고 대체 값 사용
let value = result.unwrap_or_else(|e| handle(e)); // 에러를 처리하고 대체 값 반환
let value = result.unwrap_or_default();           // Default 트레잇 사용
```
- **명시적 제어를 위한 패턴 매칭**:
```rust
match some_option {
    Some(value) => println!("Got: {}", value),
    None => println!("No value found"),
}

match some_result {
    Ok(value) => process(value),
    Err(error) => log_error(error),
}
```
- **에러 전파에는 `?` 연산자 사용**: 실패 시 즉시 빠져나가고 에러를 호출자에게 올려 보냅니다.
```rust
fn process_file(path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(path)?; // 에러를 자동으로 반환
    Ok(content.to_uppercase())
}
```
- **변환 메서드들**:
    - `map()`: 성공 값 `Ok(T)`를 `Ok(U)`로, 또는 `Some(T)`를 `Some(U)`로 변환합니다.
    - `map_err()`: 에러 타입 `Err(E)`를 `Err(F)`로 변환합니다.
    - `and_then()`: 실패할 수 있는 연산을 연쇄적으로 연결합니다.
- **자기 API에서의 사용**: 예외나 에러 코드보다 `Result<T, E>`를 우선하세요.
- **참고 자료**: [Option 문서](https://doc.rust-lang.org/std/option/enum.Option.html) | [Result 문서](https://doc.rust-lang.org/std/result/enum.Result.html)

<a id="rust-common-pitfalls-and-debugging-tips"></a>
# Rust에서 흔한 함정과 디버깅 팁
- **대여 관련 문제**: 초보자가 가장 자주 겪는 실수입니다.
    - `"cannot borrow as mutable"` -> 한 시점에 가변 참조는 하나만 허용됩니다.
    - `"borrowed value does not live long enough"` -> 참조가 가리키는 데이터보다 오래 살아남으려 합니다.
    - **해결 방법**: 스코프 `{}`를 사용해 참조 수명을 줄이거나, 필요하면 데이터를 clone하세요.
- **빠진 트레잇 구현**: `"method not found"` 에러
    - **해결 방법**: 자주 쓰는 트레잇에는 `#[derive(Debug, Clone, PartialEq)]`를 추가하세요.
    - `cargo run`보다 `cargo check`가 더 좋은 에러 메시지를 줄 때가 많습니다.
- **디버그 모드 정수 오버플로**: Rust는 오버플로 시 panic합니다.
    - **해결 방법**: 의도를 명시하기 위해 `wrapping_add()`, `saturating_add()`, `checked_add()`를 사용하세요.
- **`String`과 `&str` 혼동**: 용도가 다른 타입입니다.
    - 빌린 문자열 슬라이스에는 `&str`, 소유하는 문자열에는 `String`을 사용하세요.
    - **해결 방법**: `.to_string()`이나 `String::from()`으로 `&str`를 `String`으로 바꾸세요.
- **borrow checker와 싸우기**: 이기려 하지 마세요.
    - **해결 방법**: 소유권 규칙을 거스르지 말고, 그 규칙에 맞게 코드를 재구성하세요.
    - 복잡한 공유 시나리오에서는 `Rc<RefCell<T>>`를 고려할 수 있지만, 남용하지 마세요.

<a id="error-handling-examples-good-vs-bad"></a>
## 에러 처리 예제: 나쁜 예와 좋은 예
```rust
// [ERROR] 나쁜 예: 예상치 못하게 panic할 수 있다
fn bad_config_reader() -> String {
    let config = std::env::var("CONFIG_FILE").unwrap(); // 설정되지 않았다면 panic!
    std::fs::read_to_string(config).unwrap()            // 파일이 없으면 panic!
}

// [OK] 좋은 예: 에러를 우아하게 처리한다
fn good_config_reader() -> Result<String, ConfigError> {
    let config_path = std::env::var("CONFIG_FILE")
        .unwrap_or_else(|_| "default.conf".to_string()); // 기본값으로 대체

    let content = std::fs::read_to_string(config_path)
        .map_err(ConfigError::FileRead)?;                // 에러를 변환하고 전파

    Ok(content)
}

// [OK] 더 좋은 예: 적절한 에러 타입을 사용한다
use thiserror::Error;

#[derive(Error, Debug)]
enum ConfigError {
    #[error("Failed to read config file: {0}")]
    FileRead(#[from] std::io::Error),

    #[error("Invalid configuration: {message}")]
    Invalid { message: String },
}
```

여기서 무슨 일이 일어나는지 분해해서 봅시다. `ConfigError`에는 variant가 **딱 두 개** 있습니다. 하나는 I/O 에러용, 다른 하나는 검증 에러용입니다. 대부분의 모듈은 이 정도에서 시작하는 것이 적절합니다.

| `ConfigError` variant | 담는 값 | 만들어지는 위치 |
|----------------------|---------|------------------|
| `FileRead(io::Error)` | 원래의 I/O 에러 | `#[from]`이 `?`를 통해 자동 변환 |
| `Invalid { message }` | 사람이 읽을 수 있는 설명 | 직접 작성한 검증 코드 |

이제 `Result<T, ConfigError>`를 반환하는 함수를 작성할 수 있습니다.

```rust
fn read_config(path: &str) -> Result<String, ConfigError> {
    let content = std::fs::read_to_string(path)?;  // io::Error → ConfigError::FileRead
    if content.is_empty() {
        return Err(ConfigError::Invalid {
            message: "config file is empty".to_string(),
        });
    }
    Ok(content)
}
```

> **🟢 자기주도 학습 체크포인트:** 계속 진행하기 전에 아래 질문에 답할 수 있는지 확인해보세요.
> 1. `read_to_string` 호출에 붙은 `?`는 왜 동작할까요? (`#[from]`이 `impl From<io::Error> for ConfigError`를 생성해주기 때문입니다.)
> 2. 세 번째 variant `MissingKey(String)`를 추가하면 어떤 코드가 바뀔까요? (variant만 추가하면 되고, 기존 코드는 그대로 컴파일됩니다.)

<a id="crate-level-error-types-and-result-aliases"></a>
## 크레이트 수준 에러 타입과 Result 별칭

프로젝트가 단일 파일을 넘어 커지면, 여러 모듈 수준 에러를 하나의 **크레이트 수준 에러 타입**으로 합치게 됩니다. 이것이 프로덕션 Rust의 표준 패턴입니다. 위의 `ConfigError`에서부터 차근차근 확장해봅시다.

실제 Rust 프로젝트에서는 각 크레이트(또는 중요한 모듈)가 자체 `Error` enum과 `Result` 타입 별칭을 정의합니다. 이것이 관용적인 방식이며, C++에서 라이브러리별 예외 계층을 만들고 `using Result = std::expected<T, Error>`를 두는 것과 비슷합니다.

<a id="the-pattern"></a>
### 이 패턴의 기본 형태

```rust
// src/error.rs  (또는 lib.rs 상단)
use thiserror::Error;

/// 이 크레이트가 만들어낼 수 있는 모든 에러
#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),          // From을 통해 자동 변환

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),     // From을 통해 자동 변환

    #[error("Invalid sensor id: {0}")]
    InvalidSensor(u32),                  // 도메인 전용 variant

    #[error("Timeout after {ms} ms")]
    Timeout { ms: u64 },
}

/// 크레이트 전역 Result 별칭 - 크레이트 전반에서 타이핑을 줄여준다
pub type Result<T> = core::result::Result<T, Error>;
```

<a id="how-it-simplifies-every-function"></a>
### 모든 함수를 어떻게 단순하게 만드는가

별칭이 없다면 이렇게 작성해야 합니다.

```rust
// 장황함 - 에러 타입을 매번 반복해야 한다
fn read_sensor(id: u32) -> Result<f64, crate::Error> { ... }
fn parse_config(path: &str) -> Result<Config, crate::Error> { ... }
```

별칭이 있으면:

```rust
// 깔끔함 - `Result<T>`만 쓰면 된다
use crate::{Error, Result};

fn read_sensor(id: u32) -> Result<f64> {
    if id > 128 {
        return Err(Error::InvalidSensor(id));
    }
    let raw = std::fs::read_to_string(format!("/dev/sensor/{id}"))?; // io::Error → Error::Io
    let value: f64 = raw.trim().parse()
        .map_err(|_| Error::InvalidSensor(id))?;
    Ok(value)
}
```

`Io`에 붙은 `#[from]` 속성은 아래 `impl`을 자동으로 생성해줍니다.

```rust
// thiserror의 #[from]이 자동 생성
impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        Error::Io(source)
    }
}
```

이것이 `?`가 동작하는 이유입니다. 어떤 함수가 `std::io::Error`를 반환하고, 내 함수가 `Result<T>`(내 별칭)를 반환할 때, 컴파일러가 `From::from()`을 호출해 에러를 자동으로 변환합니다.

<a id="composing-module-level-errors"></a>
### 모듈 수준 에러 조합하기

큰 크레이트는 모듈별로 에러를 나눈 다음, 크레이트 루트에서 이를 조합합니다.

```rust
// src/config/error.rs
#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("Missing key: {0}")]
    MissingKey(String),
    #[error("Invalid value for '{key}': {reason}")]
    InvalidValue { key: String, reason: String },
}

// src/error.rs  (크레이트 수준)
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]               // 내부 에러의 Display를 그대로 위임
    Config(#[from] crate::config::ConfigError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
pub type Result<T> = core::result::Result<T, Error>;
```

호출자는 여전히 구체적인 config 에러에 대해 패턴 매칭할 수 있습니다.

```rust
match result {
    Err(Error::Config(ConfigError::MissingKey(k))) => eprintln!("Add '{k}' to config"),
    Err(e) => eprintln!("Other error: {e}"),
    Ok(v) => use_value(v),
}
```

<a id="c-comparison"></a>
### C++와 비교

| 개념 | C++ | Rust |
|------|-----|------|
| 에러 계층 | `class AppError : public std::runtime_error` | `#[derive(thiserror::Error)] enum Error { ... }` |
| 에러 반환 | `std::expected<T, Error>` 또는 `throw` | `fn foo() -> Result<T>` |
| 에러 변환 | 수동 `try/catch` + 재던지기 | `#[from]` + `?` - 보일러플레이트 거의 없음 |
| Result 별칭 | `template<class T> using Result = std::expected<T, Error>;` | `pub type Result<T> = core::result::Result<T, Error>;` |
| 에러 메시지 | `what()` 오버라이드 | `#[error("...")]` - `Display` 구현으로 컴파일됨 |
