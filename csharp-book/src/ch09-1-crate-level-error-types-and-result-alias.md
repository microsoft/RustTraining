<a id="crate-level-error-types-and-result-aliases"></a>
## 크레이트 수준 에러 타입과 Result 별칭

> **학습할 내용:** `thiserror`로 크레이트별 에러 enum을 정의하는 프로덕션 패턴,
> `Result<T>` 타입 별칭을 만드는 방법, 그리고 `thiserror`(라이브러리)와
> `anyhow`(애플리케이션) 중 언제 무엇을 고를지 배웁니다.
>
> **난이도:** 🟡 중급

프로덕션 Rust에서 매우 중요한 패턴은 크레이트 단위의 에러 enum과 `Result` 타입 별칭을 정의해 보일러플레이트를 줄이는 것입니다.

### 기본 패턴
```rust
// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Validation error: {message}")]
    Validation { message: String },

    #[error("Not found: {entity} with id {id}")]
    NotFound { entity: String, id: String },
}

/// 크레이트 전역 Result 별칭 - 모든 함수가 이것을 반환
pub type Result<T> = std::result::Result<T, AppError>;
```

### 크레이트 전반에서 사용하기
```rust
use crate::error::{AppError, Result};

pub async fn get_user(id: Uuid) -> Result<User> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_optional(&pool)
        .await?;  // #[from]을 통해 sqlx::Error -> AppError::Database

    user.ok_or_else(|| AppError::NotFound {
        entity: "User".into(),
        id: id.to_string(),
    })
}

pub async fn create_user(req: CreateUserRequest) -> Result<User> {
    if req.name.trim().is_empty() {
        return Err(AppError::Validation {
            message: "Name cannot be empty".into(),
        });
    }
    // ...
}
```

### C# 비교
```csharp
// C#에서 비슷한 패턴
public class AppException : Exception
{
    public string ErrorCode { get; }
    public AppException(string code, string message) : base(message)
    {
        ErrorCode = code;
    }
}

// 하지만 C#에서는 호출자가 어떤 예외를 예상해야 하는지 알기 어렵다!
// Rust에서는 함수 시그니처에 에러 타입이 드러난다.
```

### 왜 중요한가
- **`thiserror`**는 `Display`와 `Error` 구현을 자동으로 생성합니다.
- **`#[from]`**은 `?` 연산자가 라이브러리 에러를 자동 변환하도록 해 줍니다.
- `Result<T>` 별칭을 두면 모든 함수 시그니처를 `fn foo() -> Result<Bar>`처럼 깔끔하게 유지할 수 있습니다.
- **C# 예외와 달리**, Rust에서는 호출자가 타입만 보고 가능한 에러 variant를 확인할 수 있습니다.


### `thiserror` vs `anyhow`: 언제 무엇을 쓸까

Rust 에러 처리는 이 두 크레이트가 사실상 표준입니다. 둘 중 무엇을 고를지는 가장 먼저 하게 되는 결정입니다.

| | `thiserror` | `anyhow` |
|---|---|---|
| **목적** | **라이브러리**를 위한 구조화된 에러 타입 정의 | **애플리케이션**을 위한 빠른 에러 처리 |
| **출력** | 직접 제어하는 커스텀 enum | 불투명한 `anyhow::Error` 래퍼 |
| **호출자가 보는 것** | 타입에 모든 에러 variant가 드러남 | `anyhow::Error`만 보임 - 구체 정보는 감춰짐 |
| **적합한 대상** | 라이브러리 크레이트, API, 소비자가 있는 코드 | 바이너리, 스크립트, 프로토타입, CLI 도구 |
| **다운캐스팅** | variant에 직접 `match` | `error.downcast_ref::<MyError>()` |

```rust
// thiserror - 라이브러리용(호출자가 에러 variant를 구분해야 함)
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("File not found: {path}")]
    NotFound { path: String },

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub fn read_config(path: &str) -> Result<String, StorageError> {
    std::fs::read_to_string(path).map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => StorageError::NotFound { path: path.into() },
        std::io::ErrorKind::PermissionDenied => StorageError::PermissionDenied(path.into()),
        _ => StorageError::Io(e),
    })
}
```

```rust
// anyhow - 애플리케이션용(에러를 전파하면 되고 타입을 따로 정의하지 않아도 됨)
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let config = std::fs::read_to_string("config.toml")
        .context("Failed to read config file")?;

    let port: u16 = config.parse()
        .context("Failed to parse port number")?;

    println!("Listening on port {port}");
    Ok(())
}
// anyhow::Result<T> = Result<T, anyhow::Error>
// .context()는 모든 에러에 사람이 읽기 쉬운 문맥을 덧붙인다
```

```csharp
// C# 비교:
// thiserror ~= 특정 속성을 가진 커스텀 예외 클래스를 정의하는 것
// anyhow ~= Exception을 잡아서 메시지와 함께 다시 감싸는 것:
//   throw new InvalidOperationException("Failed to read config", ex);
```

**가이드라인:** 여러분의 코드가 **라이브러리**라면(`다른 코드가 호출함`) `thiserror`를 쓰세요. **애플리케이션**이라면(`최종 바이너리`) `anyhow`가 더 잘 맞습니다. 실제 프로젝트에서는 둘을 함께 쓰는 경우도 많습니다. 공개 API가 있는 라이브러리 크레이트에는 `thiserror`, `main()` 바이너리에는 `anyhow`를 두는 식입니다.

<a id="error-recovery-patterns"></a>
### 에러 복구 패턴

C# 개발자는 특정 예외를 복구하는 `try/catch` 블록에 익숙합니다. Rust에서는 같은 목적을 위해 `Result` 조합기를 사용합니다.

```rust
use std::fs;

// 패턴 1: 대체값으로 복구
let config = fs::read_to_string("config.toml")
    .unwrap_or_else(|_| String::from("port = 8080"));  // 파일이 없으면 기본값 사용

// 패턴 2: 특정 에러만 복구하고, 나머지는 전파
fn read_or_create(path: &str) -> Result<String, std::io::Error> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let default = String::from("# new file");
            fs::write(path, &default)?;
            Ok(default)
        }
        Err(e) => Err(e),  // 권한 에러 등은 그대로 전파
    }
}

// 패턴 3: 전파하기 전에 문맥 추가
use anyhow::Context;

fn load_config() -> anyhow::Result<Config> {
    let text = fs::read_to_string("config.toml")
        .context("Failed to read config.toml")?;
    let config: Config = toml::from_str(&text)
        .context("Failed to parse config.toml")?;
    Ok(config)
}

// 패턴 4: 에러를 도메인 타입으로 매핑
fn parse_port(s: &str) -> Result<u16, AppError> {
    s.parse::<u16>()
        .map_err(|_| AppError::Validation {
            message: format!("Invalid port: {s}"),
        })
}
```

```csharp
// C#에서 비슷한 형태:
try { config = File.ReadAllText("config.toml"); }
catch (FileNotFoundException) { config = "port = 8080"; }  // 패턴 1

try { /* ... */ }
catch (FileNotFoundException) { /* create file */ }        // 패턴 2
catch { throw; }                                            // 나머지는 다시 던짐
```

**언제 복구하고 언제 전파할까:**
- **복구**는 합리적인 기본값이나 재시도 전략이 있을 때 합니다.
- **`?`로 전파**는 무엇을 할지 *호출자*가 결정해야 할 때 합니다.
- **문맥 추가**(`.context()`)는 모듈 경계에서 에러 추적 정보를 쌓고 싶을 때 합니다.

---

## 연습문제

<details>
<summary><strong>🏋️ 연습문제: 크레이트 에러 타입 설계하기</strong> (펼쳐서 보기)</summary>

사용자 등록 서비스를 만든다고 가정해 봅시다. `thiserror`를 사용해 에러 타입을 설계해 보세요.

1. `DuplicateEmail(String)`, `WeakPassword(String)`, `DatabaseError(#[from] sqlx::Error)`, `RateLimited { retry_after_secs: u64 }` variant를 가진 `RegistrationError`를 정의하세요.
2. `type Result<T> = std::result::Result<T, RegistrationError>;` 별칭을 만드세요.
3. `?` 전파와 명시적 에러 생성을 보여 주는 `register_user(email: &str, password: &str) -> Result<()>`를 작성하세요.

<details>
<summary>🔑 해설</summary>

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegistrationError {
    #[error("Email already registered: {0}")]
    DuplicateEmail(String),

    #[error("Password too weak: {0}")]
    WeakPassword(String),

    #[error("Database error")]
    Database(#[from] sqlx::Error),

    #[error("Rate limited - retry after {retry_after_secs}s")]
    RateLimited { retry_after_secs: u64 },
}

pub type Result<T> = std::result::Result<T, RegistrationError>;

pub fn register_user(email: &str, password: &str) -> Result<()> {
    if password.len() < 8 {
        return Err(RegistrationError::WeakPassword(
            "must be at least 8 characters".into(),
        ));
    }

    // 이 ?는 sqlx::Error를 RegistrationError::Database로 자동 변환한다
    // db.check_email_unique(email).await?;

    // 도메인 로직은 이렇게 명시적으로 구성한다
    if email.contains("+spam") {
        return Err(RegistrationError::DuplicateEmail(email.to_string()));
    }

    Ok(())
}
```

**핵심 패턴:** `#[from]`은 라이브러리 에러에 대해 `?`를 가능하게 하고, 도메인 로직은 `Err(...)`로 명시적으로 구성합니다. Result 별칭은 모든 시그니처를 깔끔하게 유지해 줍니다.

</details>
</details>

***


