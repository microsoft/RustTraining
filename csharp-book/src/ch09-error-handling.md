<a id="exceptions-vs-resultt-e"></a>
## 예외와 `Result<T, E>`

> **학습할 내용:** Rust가 예외 대신 `Result<T, E>`와 `Option<T>`을 사용하는 이유,
> `?` 연산자로 에러를 간결하게 전파하는 방법, 그리고 명시적인 에러 처리가
> C#의 `try`/`catch` 코드에서 흔한 숨겨진 제어 흐름을 어떻게 없애는지 배웁니다.
>
> **난이도:** 🟡 중급
>
> **함께 읽기:** `thiserror`, `anyhow`를 활용한 프로덕션 에러 패턴은
> [크레이트 수준 에러 타입](ch09-1-crate-level-error-types-and-result-alias.md)에서,
> 에러 처리 생태계는 [필수 크레이트](ch15-1-essential-crates-for-c-developers.md)에서 더 다룹니다.

### C#의 예외 기반 에러 처리
```csharp
// C# - 예외 기반 에러 처리
public class UserService
{
    public User GetUser(int userId)
    {
        if (userId <= 0)
        {
            throw new ArgumentException("User ID must be positive");
        }
        
        var user = database.FindUser(userId);
        if (user == null)
        {
            throw new UserNotFoundException($"User {userId} not found");
        }
        
        return user;
    }
    
    public async Task<string> GetUserEmailAsync(int userId)
    {
        try
        {
            var user = GetUser(userId);
            return user.Email ?? throw new InvalidOperationException("User has no email");
        }
        catch (UserNotFoundException ex)
        {
            logger.Warning("User not found: {UserId}", userId);
            return "noreply@company.com";
        }
        catch (Exception ex)
        {
            logger.Error(ex, "Unexpected error getting user email");
            throw; // Re-throw
        }
    }
}
```

### Rust의 Result 기반 에러 처리
```rust
use std::fmt;

#[derive(Debug)]
pub enum UserError {
    InvalidId(i32),
    NotFound(i32),
    NoEmail,
    DatabaseError(String),
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserError::InvalidId(id) => write!(f, "Invalid user ID: {}", id),
            UserError::NotFound(id) => write!(f, "User {} not found", id),
            UserError::NoEmail => write!(f, "User has no email address"),
            UserError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl std::error::Error for UserError {}

pub struct UserService {
    // database connection, etc.
}

impl UserService {
    pub fn get_user(&self, user_id: i32) -> Result<User, UserError> {
        if user_id <= 0 {
            return Err(UserError::InvalidId(user_id));
        }
        
        // 데이터베이스 조회를 흉내 냄
        self.database_find_user(user_id)
            .ok_or(UserError::NotFound(user_id))
    }
    
    pub fn get_user_email(&self, user_id: i32) -> Result<String, UserError> {
        let user = self.get_user(user_id)?; // ? 연산자가 에러를 전파
        
        user.email
            .ok_or(UserError::NoEmail)
    }
    
    pub fn get_user_email_or_default(&self, user_id: i32) -> String {
        match self.get_user_email(user_id) {
            Ok(email) => email,
            Err(UserError::NotFound(_)) => {
                log::warn!("User not found: {}", user_id);
                "noreply@company.com".to_string()
            }
            Err(err) => {
                log::error!("Error getting user email: {}", err);
                "error@company.com".to_string()
            }
        }
    }
}
```

```mermaid
graph TD
    subgraph "C# 예외 모델"
        CS_CALL["메서드 호출"]
        CS_SUCCESS["성공 경로"]
        CS_EXCEPTION["throw Exception"]
        CS_STACK["스택 언와인딩<br/>(런타임 비용)"]
        CS_CATCH["try/catch 블록"]
        CS_HIDDEN["[단점] 숨겨진 제어 흐름<br/>[단점] 성능 비용<br/>[단점] 무시하기 쉬움"]
        
        CS_CALL --> CS_SUCCESS
        CS_CALL --> CS_EXCEPTION
        CS_EXCEPTION --> CS_STACK
        CS_STACK --> CS_CATCH
        CS_EXCEPTION --> CS_HIDDEN
    end
    
    subgraph "Rust Result 모델"
        RUST_CALL["함수 호출"]
        RUST_OK["Ok(value)"]
        RUST_ERR["Err(error)"]
        RUST_MATCH["match 결과 처리"]
        RUST_QUESTION["? 연산자<br/>(조기 반환)"]
        RUST_EXPLICIT["[장점] 명시적인 에러 처리<br/>[장점] 런타임 비용 없음<br/>[장점] 에러를 무시할 수 없음"]
        
        RUST_CALL --> RUST_OK
        RUST_CALL --> RUST_ERR
        RUST_OK --> RUST_MATCH
        RUST_ERR --> RUST_MATCH
        RUST_ERR --> RUST_QUESTION
        RUST_MATCH --> RUST_EXPLICIT
        RUST_QUESTION --> RUST_EXPLICIT
    end
    
    style CS_HIDDEN fill:#ffcdd2,color:#000
    style RUST_EXPLICIT fill:#c8e6c9,color:#000
    style CS_STACK fill:#fff3e0,color:#000
    style RUST_QUESTION fill:#c8e6c9,color:#000
```

***

<a id="the--operator-propagating-errors-concisely"></a>
### `?` 연산자: 간결한 에러 전파
```csharp
// C# - 예외 전파(암묵적)
public async Task<string> ProcessFileAsync(string path)
{
    var content = await File.ReadAllTextAsync(path);  // Throws on error
    var processed = ProcessContent(content);          // Throws on error
    return processed;
}
```

```rust
// Rust - ?를 이용한 에러 전파
fn process_file(path: &str) -> Result<String, ConfigError> {
    let content = read_config(path)?;  // Err면 ?가 에러를 전파
    let processed = process_content(&content)?;  // Err면 ?가 에러를 전파
    Ok(processed)  // 성공 값을 Ok로 감쌈
}

fn process_content(content: &str) -> Result<String, ConfigError> {
    if content.is_empty() {
        Err(ConfigError::InvalidFormat)
    } else {
        Ok(content.to_uppercase())
    }
}
```

### null 가능 값에 대한 `Option<T>`
```csharp
// C# - nullable 참조 타입
public string? FindUserName(int userId)
{
    var user = database.FindUser(userId);
    return user?.Name;  // Returns null if user not found
}

public void ProcessUser(int userId)
{
    string? name = FindUserName(userId);
    if (name != null)
    {
        Console.WriteLine($"User: {name}");
    }
    else
    {
        Console.WriteLine("User not found");
    }
}
```

```rust
// Rust - 선택적 값을 위한 Option<T>
fn find_user_name(user_id: u32) -> Option<String> {
    // 데이터베이스 조회를 흉내 냄
    if user_id == 1 {
        Some("Alice".to_string())
    } else {
        None
    }
}

fn process_user(user_id: u32) {
    match find_user_name(user_id) {
        Some(name) => println!("User: {}", name),
        None => println!("User not found"),
    }
    
    // 또는 if let 사용(패턴 매칭 축약형)
    if let Some(name) = find_user_name(user_id) {
        println!("User: {}", name);
    } else {
        println!("User not found");
    }
}
```

### `Option`과 `Result` 함께 사용하기
```rust
fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b != 0.0 {
        Some(a / b)
    } else {
        None
    }
}

fn parse_and_divide(a_str: &str, b_str: &str) -> Result<Option<f64>, ParseFloatError> {
    let a: f64 = a_str.parse()?;  // 잘못된 입력이면 파싱 에러 반환
    let b: f64 = b_str.parse()?;  // 잘못된 입력이면 파싱 에러 반환
    Ok(safe_divide(a, b))         // Ok(Some(result)) 또는 Ok(None) 반환
}

use std::num::ParseFloatError;

fn main() {
    match parse_and_divide("10.0", "2.0") {
        Ok(Some(result)) => println!("Result: {}", result),
        Ok(None) => println!("Division by zero"),
        Err(error) => println!("Parse error: {}", error),
    }
}
```

***


<details>
<summary><strong>🏋️ 연습문제: 크레이트 수준 에러 타입 만들기</strong> (펼쳐서 보기)</summary>

**도전 과제:** I/O 에러, JSON 파싱 에러, 검증 에러 때문에 실패할 수 있는 파일 처리 애플리케이션용 `AppError` enum을 만들어 보세요. 자동 `?` 전파를 위해 `From` 변환도 구현하세요.

```rust
// 시작 코드
use std::io;

// TODO: 다음 variant를 가진 AppError를 정의하세요:
//   Io(io::Error), Json(serde_json::Error), Validation(String)
// TODO: Display와 Error 트레잇을 구현하세요
// TODO: From<io::Error>와 From<serde_json::Error>를 구현하세요
// TODO: 타입 별칭을 정의하세요: type Result<T> = std::result::Result<T, AppError>;

fn load_config(path: &str) -> Result<Config> {
    let content = std::fs::read_to_string(path)?;  // io::Error -> AppError
    let config: Config = serde_json::from_str(&content)?;  // serde error -> AppError
    if config.name.is_empty() {
        return Err(AppError::Validation("name cannot be empty".into()));
    }
    Ok(config)
}
```

<details>
<summary>🔑 해설</summary>

```rust
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Validation: {0}")]
    Validation(String),
}

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(serde::Deserialize)]
struct Config {
    name: String,
    port: u16,
}

fn load_config(path: &str) -> Result<Config> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&content)?;
    if config.name.is_empty() {
        return Err(AppError::Validation("name cannot be empty".into()));
    }
    Ok(config)
}
```

**핵심 요점:**
- `thiserror`는 attribute만으로 `Display`와 `Error` 구현을 생성해 줍니다.
- `#[from]`은 `From<T>` 구현을 생성해 자동 `?` 변환을 가능하게 합니다.
- `Result<T>` 별칭을 두면 크레이트 전반의 보일러플레이트를 줄일 수 있습니다.
- C# 예외와 달리 Rust에서는 모든 함수 시그니처에 에러 타입이 드러납니다.

</details>
</details>


