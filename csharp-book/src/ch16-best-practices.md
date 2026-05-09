<a id="best-practices-for-c-developers"></a>
## C# 개발자를 위한 모범 사례

> **이 장에서 배울 내용:** 다섯 가지 핵심 사고방식 전환(GC→소유권, 예외→`Result`, 상속→조합),
> 관용적인 프로젝트 구성, 에러 처리 전략, 테스트 패턴, 그리고 C# 개발자가 Rust에서 가장 자주
> 저지르는 실수들을 정리합니다.
>
> **난이도:** 🟡 중급

### 1. **사고방식 전환**
- **GC에서 소유권으로**: 누가 데이터를 소유하고 언제 해제되는지 생각하세요
- **예외에서 `Result`로**: 에러 처리를 명시적이고 눈에 보이게 만드세요
- **상속에서 조합으로**: 트레잇으로 동작을 조합하세요
- **`null`에서 `Option`으로**: 값의 부재를 타입 시스템에 명시하세요

### 2. **코드 구성**
```rust
// C# 솔루션처럼 프로젝트를 구성하기
src/
├── main.rs          // Program.cs에 해당
├── lib.rs           // 라이브러리 진입점
├── models/          // C#의 Models/ 폴더와 유사
│   ├── mod.rs
│   ├── user.rs
│   └── product.rs
├── services/        // Services/ 폴더와 유사
│   ├── mod.rs
│   ├── user_service.rs
│   └── product_service.rs
├── controllers/     // Controllers/ 와 유사 (웹 앱이라면)
├── repositories/    // Repositories/ 와 유사
└── utils/           // Utilities/ 와 유사
```

### 3. **에러 처리 전략**
```rust
// 애플리케이션 전체에서 공통으로 쓸 Result 타입 만들기
pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("Validation error: {message}")]
    Validation { message: String },
    
    #[error("Business logic error: {message}")]
    Business { message: String },
}

// 애플리케이션 전반에서 일관되게 사용
pub async fn create_user(data: CreateUserRequest) -> AppResult<User> {
    validate_user_data(&data)?;  // AppError::Validation 반환
    let user = repository.create_user(data).await?;  // AppError::Database 반환
    Ok(user)
}
```

### 4. **테스트 패턴**
```rust
// C# 단위 테스트처럼 구조화하기
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;  // C# [Theory] 같은 매개변수화 테스트용
    
    #[test]
    fn test_basic_functionality() {
        // Arrange
        let input = "test data";
        
        // Act
        let result = process_data(input);
        
        // Assert
        assert_eq!(result, "expected output");
    }
    
    #[rstest]
    #[case(1, 2, 3)]
    #[case(5, 5, 10)]
    #[case(0, 0, 0)]
    fn test_addition(#[case] a: i32, #[case] b: i32, #[case] expected: i32) {
        assert_eq!(add(a, b), expected);
    }
    
    #[tokio::test]  // async 테스트용
    async fn test_async_functionality() {
        let result = async_function().await;
        assert!(result.is_ok());
    }
}
```

### 5. **피해야 할 흔한 실수**
```rust
// [ERROR] 상속을 구현하려고 하지 마세요
// 대신:
// struct Manager : Employee  // Rust에는 이런 문법이 없습니다

// [OK] 트레잇 기반 조합을 사용하세요
trait Employee {
    fn get_salary(&self) -> u32;
}

trait Manager: Employee {
    fn get_team_size(&self) -> usize;
}

// [ERROR] 어디서나 unwrap()을 쓰지 마세요 (예외를 무시하는 것과 비슷합니다)
let value = might_fail().unwrap();  // panic 가능!

// [OK] 에러를 올바르게 처리하세요
let value = match might_fail() {
    Ok(v) => v,
    Err(e) => {
        log::error!("Operation failed: {}", e);
        return Err(e.into());
    }
};

// [ERROR] 모든 것을 clone하지 마세요 (불필요한 객체 복사와 비슷합니다)
let data = expensive_data.clone();  // 비용이 큽니다!

// [OK] 가능하면 빌리세요
let data = &expensive_data;  // 단순한 참조

// [ERROR] RefCell을 만능처럼 쓰지 마세요 (모든 것을 mutable로 만드는 것과 비슷합니다)
struct Data {
    value: RefCell<i32>,  // 내부 가변성 - 꼭 필요할 때만
}

// [OK] 소유 데이터나 빌린 데이터를 우선하세요
struct Data {
    value: i32,  // 단순하고 명확함
}
```

이 가이드는 C# 개발자가 자신이 이미 가진 지식을 Rust에 어떻게 옮길 수 있는지 폭넓게 보여주며, 두 언어의 닮은 점과 접근 방식의 근본적인 차이도 함께 강조합니다. 핵심은 Rust의 제약(예: 소유권)이 약간의 초기 복잡도를 대가로, C#에서 발생할 수 있는 버그의 큰 범주를 통째로 막기 위해 설계되었다는 점을 이해하는 것입니다.

---

### 6. **과도한 `clone()` 피하기** 🟡

C# 개발자는 GC가 비용을 처리해주기 때문에 데이터를 반사적으로 clone하는 경향이 있습니다. Rust에서는 모든 `.clone()`이 명시적인 할당입니다. 대부분은 대여로 없앨 수 있습니다.

```rust
// [ERROR] C# 습관: 문자열을 넘길 때마다 clone
fn greet(name: String) {
    println!("Hello, {name}");
}

let user_name = String::from("Alice");
greet(user_name.clone());  // 불필요한 할당
greet(user_name.clone());  // 또 한 번

// [OK] 대신 빌리기 — 할당 0회
fn greet(name: &str) {
    println!("Hello, {name}");
}

let user_name = String::from("Alice");
greet(&user_name);  // 빌림
greet(&user_name);  // 다시 빌림 — 추가 비용 없음
```

**`clone`이 적절한 경우:**
- 데이터를 스레드나 `'static` 클로저로 옮겨야 할 때 (`Arc::clone`은 카운터만 늘리므로 저렴합니다)
- 캐싱: 정말로 독립적인 복사본이 필요할 때
- 프로토타이핑: 먼저 동작하게 만든 뒤 나중에 clone을 줄일 때

**판단 체크리스트:**
1. 대신 `&T`나 `&str`를 넘길 수 있나요? → 그 방법을 쓰세요
2. 호출 대상이 소유권을 꼭 필요로 하나요? → clone하지 말고 move 하세요
3. 스레드 간에 공유되나요? → `Arc<T>`를 쓰세요 (`clone`은 참조 카운트만 증가)
4. 위에 모두 해당하지 않나요? → 그때는 `clone()`이 정당합니다

---

### 7. **프로덕션 코드에서 `unwrap()` 피하기** 🟡

예외를 무시하는 습관이 있는 C# 개발자는 Rust에서도 `.unwrap()`을 곳곳에 쓰곤 합니다. 둘 다 똑같이 위험합니다.

```rust
// [ERROR] "나중에 고치지 뭐" 함정
let config = std::fs::read_to_string("config.toml").unwrap();
let port: u16 = config_value.parse().unwrap();
let conn = db_pool.get().await.unwrap();

// [OK] 애플리케이션 코드에서는 ?로 전파
let config = std::fs::read_to_string("config.toml")?;
let port: u16 = config_value.parse()?;
let conn = db_pool.get().await?;

// [OK] 실패가 정말 버그일 때만 expect() 사용
let home = std::env::var("HOME")
    .expect("HOME environment variable must be set");  // invariant를 문서화함
```

**실전 규칙:**
| 메서드 | 사용 시점 |
|--------|-----------|
| `?` | 애플리케이션/라이브러리 코드에서 호출자에게 전파할 때 |
| `expect("reason")` | 시작 시점 검증, 반드시 성립해야 하는 invariant |
| `unwrap()` | 테스트에서만, 또는 `is_some()`/`is_ok()` 확인 직후 |
| `unwrap_or(default)` | 합리적인 기본값이 있을 때 |
| `unwrap_or_else(|| ...)` | 기본값 계산 비용이 클 때 |

---

### 8. **borrow checker와 싸우지 않기 (그리고 멈추는 법)** 🟡

모든 C# 개발자는 한 번쯤 borrow checker가 "멀쩡해 보이는" 코드를 거부하는 시기를 겪습니다. 해결책은 대개 우회가 아니라 구조 변경입니다.

```rust
// [ERROR] 순회하면서 동시에 수정하려고 함 (C# foreach + 수정 패턴)
let mut items = vec![1, 2, 3, 4, 5];
for item in &items {
    if *item > 3 {
        items.push(*item * 2);  // ERROR: items를 mutable로 빌릴 수 없음
    }
}

// [OK] 먼저 모은 뒤, 나중에 수정
let extras: Vec<i32> = items.iter()
    .filter(|&&x| x > 3)
    .map(|&x| x * 2)
    .collect();
items.extend(extras);
```

```rust
// [ERROR] 지역 변수에 대한 참조를 반환하려고 함 (C#은 GC 덕분에 참조 반환이 비교적 자유로움)
fn get_greeting() -> &str {
    let s = String::from("hello");
    &s  // ERROR: 함수 끝에서 s가 drop됨
}

// [OK] 소유 데이터를 반환
fn get_greeting() -> String {
    String::from("hello")  // 호출자가 소유
}
```

**borrow checker 충돌을 푸는 대표 패턴:**

| C# 습관 | Rust 해법 |
|---------|-----------|
| 구조체 안에 참조 저장 | 소유 데이터를 쓰거나 lifetime 매개변수 추가 |
| 공유 상태를 자유롭게 수정 | `Arc<Mutex<T>>`를 쓰거나 공유 자체를 줄이도록 재구성 |
| 지역 변수 참조 반환 | 소유 값을 반환 |
| 순회 중 컬렉션 수정 | 변경 사항을 모아두었다가 나중에 적용 |
| 여러 개의 mutable 참조 | 구조체를 독립적인 부분으로 분리 |

---

### 9. **중첩된 대입 피라미드 줄이기** 🟢

C# 개발자는 `if (x != null) { if (x.Value > 0) { ... } }` 같은 중첩 코드를 자주 씁니다. Rust의 `match`, `if let`, `?`는 이를 평평하게 펴줍니다.

```rust
// [ERROR] C#식 중첩 null 체크 스타일
fn process(input: Option<String>) -> Option<usize> {
    match input {
        Some(s) => {
            if !s.is_empty() {
                match s.parse::<usize>() {
                    Ok(n) => {
                        if n > 0 {
                            Some(n * 2)
                        } else {
                            None
                        }
                    }
                    Err(_) => None,
                }
            } else {
                None
            }
        }
        None => None,
    }
}

// [OK] 조합자로 평평하게 만들기
fn process(input: Option<String>) -> Option<usize> {
    input
        .filter(|s| !s.is_empty())
        .and_then(|s| s.parse::<usize>().ok())
        .filter(|&n| n > 0)
        .map(|n| n * 2)
}
```

**C# 개발자가 꼭 알아둘 핵심 조합자:**

| 조합자 | 역할 | C# 대응 개념 |
|--------|------|---------------|
| `map` | 내부 값을 변환 | `Select` / null 조건부 `?.` |
| `and_then` | `Option`/`Result`를 반환하는 연산을 연쇄 | `SelectMany` / `?.Method()` |
| `filter` | 조건을 만족할 때만 값 유지 | `Where` |
| `unwrap_or` | 기본값 제공 | `?? defaultValue` |
| `ok()` | `Result`를 `Option`으로 변환 (에러는 버림) | — |
| `transpose` | `Option<Result>`를 `Result<Option>`으로 뒤집기 | — |



