<a id="rust-best-practices-summary"></a>
# Rust 모범 사례 요약

> **이 장에서 배우는 것:** idiomatic Rust를 쓰기 위한 실전 가이드라인을 정리합니다. 코드 구성, 이름 짓기 규칙, 에러 처리 패턴, 문서화까지 자주 다시 펼쳐 보게 될 빠른 참고 장입니다.

<a id="code-organization"></a>
## 코드 구성
- **함수는 작게 유지하세요**: 테스트와 추론이 쉬워집니다.
- **설명적인 이름을 쓰세요**: `calc()`보다 `calculate_total_price()`가 낫습니다.
- **관련 기능끼리 묶으세요**: 모듈과 파일 분리를 활용하세요.
- **문서를 작성하세요**: 공개 API에는 `///`를 사용하세요.

<a id="error-handling"></a>
## 에러 처리
- **`unwrap()`은 실패할 수 없을 때만 쓰세요**: 패닉하지 않는다고 100% 확신할 때에만 사용합니다.
```rust
// 나쁨: 패닉할 수 있다
let value = some_option.unwrap();

// 좋음: None 케이스를 처리한다
let value = some_option.unwrap_or(default_value);
let value = some_option.unwrap_or_else(|| expensive_computation());
let value = some_option.unwrap_or_default(); // Default 트레잇 사용

// Result<T, E>의 경우
let value = some_result.unwrap_or(fallback_value);
let value = some_result.unwrap_or_else(|err| {
    eprintln!("Error occurred: {err}");
    default_value
});
```
- **설명적인 메시지와 함께 `expect()`를 쓰세요**: `unwrap`이 정당화될 수 있다면, 왜 괜찮은지 메시지로 남기세요.
```rust
let config = std::env::var("CONFIG_PATH")
    .expect("CONFIG_PATH environment variable must be set");
```
- **실패 가능한 연산은 `Result<T, E>`를 반환하세요**: 에러를 어떻게 처리할지는 호출자가 결정하게 하세요.
- **커스텀 에러 타입에는 `thiserror`를 쓰세요**: 수동 구현보다 더 사용하기 편합니다.
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Parse error: {message}")]
    Parse { message: String },
    
    #[error("Value {value} is out of range")]
    OutOfRange { value: i32 },
}
```
- **`?` 연산자로 에러를 연결하세요**: 에러를 호출 스택 위로 자연스럽게 전파합니다.
- **`anyhow`보다 `thiserror`를 우선하세요**: 우리 팀의 관례는 `#[derive(thiserror::Error)]`를 사용해 명시적인 에러 enum을 정의하는 것입니다. 그래야 호출자가 특정 variant에 대해 `match`할 수 있습니다. `anyhow::Error`는 빠른 프로토타이핑에는 편하지만 에러 타입 정보를 지워 버리므로, 호출자가 구체적인 실패를 처리하기 어렵게 만듭니다. 라이브러리와 프로덕션 코드에는 `thiserror`를 쓰고, 에러를 단순 출력만 하면 되는 일회성 스크립트나 최상위 바이너리에서만 `anyhow`를 쓰세요.
- **`unwrap()`이 허용될 수 있는 경우**:
  - **단위 테스트**: `assert_eq!(result.unwrap(), expected)`
  - **프로토타이핑**: 곧 교체할 빠른 실험 코드
  - **실패 불가능한 연산**: 실패하지 않음을 증명할 수 있는 경우
```rust
let numbers = vec![1, 2, 3];
let first = numbers.get(0).unwrap(); // 안전함: 방금 원소를 넣은 vec를 만들었다

// 더 좋음: 이유를 설명하는 expect() 사용
let first = numbers.get(0).expect("numbers vec is non-empty by construction");
```
- **빨리 실패하세요**: 전제 조건을 일찍 검사하고, 에러를 즉시 반환하세요.

<a id="memory-management"></a>
## 메모리 관리
- **clone보다 대여를 우선하세요**: 가능하면 `clone` 대신 `&T`를 사용하세요.
- **`Rc<T>`는 아껴 쓰세요**: 공유 소유권이 정말 필요할 때만 사용합니다.
- **라이프타임 범위를 좁히세요**: 값이 언제 drop되는지 제어하려면 `{}` 스코프를 활용하세요.
- **공개 API에 `RefCell<T>`를 노출하지 마세요**: 내부 가변성은 내부 구현에 가두는 편이 좋습니다.

<a id="performance"></a>
## 성능
- **최적화 전에 먼저 프로파일링하세요**: `cargo bench`와 프로파일링 도구를 사용하세요.
- **루프보다 이터레이터를 우선하세요**: 더 읽기 쉽고, 종종 더 빠릅니다.
- **`String`보다 `&str`를 우선하세요**: 소유권이 필요 없을 때는 대여면 충분합니다.
- **큰 스택 객체에는 `Box<T>`를 고려하세요**: 필요하면 힙으로 옮기세요.

<a id="essential-traits-to-implement"></a>
## 구현을 고려해야 할 핵심 트레잇

<a id="core-traits-every-type-should-consider"></a>
### 모든 타입이 고려해 볼 핵심 트레잇

커스텀 타입을 만들 때는, 그 타입이 Rust의 기본 타입처럼 자연스럽게 느껴지도록 아래 기본 트레잇 구현을 고려해 보세요.

<a id="debug-and-display"></a>
#### `Debug`와 `Display`
```rust
use std::fmt;

#[derive(Debug)]  // 디버깅용 자동 구현
struct Person {
    name: String,
    age: u32,
}

// 사용자에게 보여 주는 출력을 위한 수동 Display 구현
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (age {})", self.name, self.age)
    }
}

// 사용 예:
let person = Person { name: "Alice".to_string(), age: 30 };
println!("{:?}", person);  // Debug: Person { name: "Alice", age: 30 }
println!("{}", person);    // Display: Alice (age 30)
```

<a id="clone-and-copy"></a>
#### `Clone`과 `Copy`
```rust
// Copy: 작고 단순한 타입의 암시적 복제
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

// Clone: 복잡한 타입의 명시적 복제
#[derive(Debug, Clone)]
struct Person {
    name: String,  // String은 Copy를 구현하지 않는다
    age: u32,
}

let p1 = Point { x: 1, y: 2 };
let p2 = p1;  // Copy (암시적)

let person1 = Person { name: "Bob".to_string(), age: 25 };
let person2 = person1.clone();  // Clone (명시적)
```

<a id="partialeq-and-eq"></a>
#### `PartialEq`와 `Eq`
```rust
#[derive(Debug, PartialEq, Eq)]
struct UserId(u64);

#[derive(Debug, PartialEq)]
struct Temperature {
    celsius: f64,  // f64는 Eq를 구현하지 않는다(NaN 때문)
}

let id1 = UserId(123);
let id2 = UserId(123);
assert_eq!(id1, id2);  // PartialEq 덕분에 가능

let temp1 = Temperature { celsius: 20.0 };
let temp2 = Temperature { celsius: 20.0 };
assert_eq!(temp1, temp2);  // PartialEq로 가능
```

<a id="partialord-and-ord"></a>
#### `PartialOrd`와 `Ord`
```rust
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Priority(u8);

let high = Priority(1);
let low = Priority(10);
assert!(high < low);  // 숫자가 작을수록 우선순위가 높음

// 컬렉션에서 사용
let mut priorities = vec![Priority(5), Priority(1), Priority(8)];
priorities.sort();  // Priority가 Ord를 구현했기 때문에 가능
```

<a id="default"></a>
#### `Default`
```rust
#[derive(Debug, Default)]
struct Config {
    debug: bool,           // false (기본값)
    max_connections: u32,  // 0 (기본값)
    timeout: Option<u64>,  // None (기본값)
}

// 커스텀 Default 구현
impl Default for Config {
    fn default() -> Self {
        Config {
            debug: false,
            max_connections: 100,  // 커스텀 기본값
            timeout: Some(30),     // 커스텀 기본값
        }
    }
}

let config = Config::default();
let config = Config { debug: true, ..Default::default() };  // 일부 필드만 덮어쓰기
```

<a id="from-and-into"></a>
#### `From`과 `Into`
```rust
struct UserId(u64);
struct UserName(String);

// From을 구현하면 Into는 자동으로 따라온다
impl From<u64> for UserId {
    fn from(id: u64) -> Self {
        UserId(id)
    }
}

impl From<String> for UserName {
    fn from(name: String) -> Self {
        UserName(name)
    }
}

impl From<&str> for UserName {
    fn from(name: &str) -> Self {
        UserName(name.to_string())
    }
}

// 사용 예:
let user_id: UserId = 123u64.into();         // Into 사용
let user_id = UserId::from(123u64);          // From 사용
let username = UserName::from("alice");      // &str -> UserName
let username: UserName = "bob".into();       // Into 사용
```

<a id="tryfrom-and-tryinto"></a>
#### `TryFrom`과 `TryInto`
```rust
use std::convert::TryFrom;

struct PositiveNumber(u32);

#[derive(Debug)]
struct NegativeNumberError;

impl TryFrom<i32> for PositiveNumber {
    type Error = NegativeNumberError;
    
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(PositiveNumber(value as u32))
        } else {
            Err(NegativeNumberError)
        }
    }
}

// 사용 예:
let positive = PositiveNumber::try_from(42)?;     // Ok(PositiveNumber(42))
let error = PositiveNumber::try_from(-5);         // Err(NegativeNumberError)
```

<a id="serde-for-serialization"></a>
#### 직렬화를 위한 `Serde`
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

// JSON 직렬화/역직렬화 자동 구현
let user = User {
    id: 1,
    name: "Alice".to_string(),
    email: "alice@example.com".to_string(),
};

let json = serde_json::to_string(&user)?;
let deserialized: User = serde_json::from_str(&json)?;
```

<a id="trait-implementation-checklist"></a>
### 트레잇 구현 체크리스트

새 타입을 만들 때는 아래 체크리스트를 떠올려 보세요.

```rust
#[derive(
    Debug,          // [OK] 디버깅을 위해 항상 고려
    Clone,          // [OK] 타입을 복제 가능하게 해야 한다면
    PartialEq,      // [OK] 타입끼리 비교할 수 있어야 한다면
    Eq,             // [OK] 비교가 반사적/추이적이라면
    PartialOrd,     // [OK] 정렬 순서가 있다면
    Ord,            // [OK] 전체 순서가 있다면
    Hash,           // [OK] HashMap 키로 쓸 예정이라면
    Default,        // [OK] 그럴듯한 기본값이 있다면
)]
struct MyType {
    // fields...
}

// 수동 구현을 고려할 만한 트레잇:
impl Display for MyType { /* user-facing representation */ }
impl From<OtherType> for MyType { /* convenient conversion */ }
impl TryFrom<FallibleType> for MyType { /* fallible conversion */ }
```

<a id="when-not-to-implement-traits"></a>
### 트레잇을 구현하지 말아야 하는 경우

- **힙 데이터를 가진 타입에는 `Copy`를 구현하지 마세요**: `String`, `Vec`, `HashMap` 등
- **값이 NaN이 될 수 있으면 `Eq`를 구현하지 마세요**: `f32`/`f64`를 포함한 타입
- **그럴듯한 기본값이 없으면 `Default`를 구현하지 마세요**: 파일 핸들, 네트워크 연결
- **복제가 비싸면 `Clone` 구현을 재고하세요**: 큰 자료구조는 `Rc<T>` 같은 대안을 생각하세요

<a id="summary-trait-benefits"></a>
### 요약: 트레잇이 주는 이점

| Trait | 장점 | 사용할 때 |
|-------|---------|-------------|
| `Debug` | `println!("{:?}", value)` | 거의 항상(드문 예외 제외) |
| `Display` | `println!("{}", value)` | 사용자에게 보여 주는 타입 |
| `Clone` | `value.clone()` | 명시적 복제가 의미 있을 때 |
| `Copy` | 암시적 복제 | 작고 단순한 타입 |
| `PartialEq` | `==`와 `!=` 연산자 | 대부분의 타입 |
| `Eq` | 반사적 동등성 | 수학적으로 타당한 동등성이 있을 때 |
| `PartialOrd` | `<`, `>`, `<=`, `>=` | 자연스러운 순서가 있는 타입 |
| `Ord` | `sort()`, `BinaryHeap` | 전체 순서가 있는 타입 |
| `Hash` | `HashMap` 키 | 맵 키로 쓰는 타입 |
| `Default` | `Default::default()` | 명확한 기본값이 있는 타입 |
| `From/Into` | 편리한 변환 | 자주 쓰는 타입 변환 |
| `TryFrom/TryInto` | 실패 가능한 변환 | 실패할 수 있는 변환 |

----

----
