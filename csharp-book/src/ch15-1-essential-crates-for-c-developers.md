<a id="essential-crates-for-c-developers"></a>
## C# 개발자를 위한 필수 크레이트

> **이 장에서 배우는 것:** 흔히 쓰는 .NET 라이브러리에 대응하는 Rust 크레이트들, 예를 들어 serde(JSON.NET), reqwest(HttpClient), tokio(Task/async), sqlx(Entity Framework)를 살펴보고, `System.Text.Json`과 비교하면서 serde의 속성(attribute) 시스템도 깊이 있게 이해합니다.
>
> **난이도:** 🟡 중급

<a id="core-functionality-equivalents"></a>
### 핵심 기능 대응표

```rust
// C# 개발자를 위한 Cargo.toml 의존성 예시
[dependencies]
# 직렬화 (Newtonsoft.Json 또는 System.Text.Json과 유사)
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# HTTP 클라이언트 (HttpClient와 유사)
reqwest = { version = "0.11", features = ["json"] }

# Async 런타임 (Task.Run, async/await와 유사)
tokio = { version = "1.0", features = ["full"] }

# 에러 처리 (사용자 정의 예외와 유사)
thiserror = "1.0"
anyhow = "1.0"

# 로깅 (ILogger, Serilog와 유사)
log = "0.4"
env_logger = "0.10"

# 날짜/시간 (DateTime과 유사)
chrono = { version = "0.4", features = ["serde"] }

# UUID (System.Guid와 유사)
uuid = { version = "1.0", features = ["v4", "serde"] }

# 컬렉션 (List<T>, Dictionary<K,V>와 유사)
# 기본 컬렉션은 std에 포함되어 있지만, 고급 컬렉션이 필요하면:
indexmap = "2.0"  # 순서가 유지되는 HashMap

# 설정 (IConfiguration과 유사)
config = "0.13"

# 데이터베이스 (Entity Framework와 유사)
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono"] }

# 테스트 (xUnit, NUnit과 유사)
# 기본 테스트 기능은 std에 포함되어 있지만, 추가 기능이 필요하면:
rstest = "0.18"  # 매개변수화 테스트

# 목 객체 (Moq와 유사)
mockall = "0.11"

# 병렬 처리 (Parallel.ForEach와 유사)
rayon = "1.7"
```

<a id="example-usage-patterns"></a>
### 사용 패턴 예시

```rust
use serde::{Deserialize, Serialize};
use reqwest;
use tokio;
use thiserror::Error;
use chrono::{DateTime, Utc};
use uuid::Uuid;

// 데이터 모델 (속성(attribute)이 붙은 C# POCO와 유사)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
}

// 사용자 정의 에러 타입 (사용자 정의 예외와 유사)
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("Serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("User not found: {id}")]
    UserNotFound { id: Uuid },
    
    #[error("Validation failed: {message}")]
    Validation { message: String },
}

// 서비스 클래스에 대응하는 구조
pub struct UserService {
    client: reqwest::Client,
    base_url: String,
}

impl UserService {
    pub fn new(base_url: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");
            
        UserService { client, base_url }
    }
    
    // 비동기 메서드 (C#의 async Task<User>와 유사)
    pub async fn get_user(&self, id: Uuid) -> Result<User, ApiError> {
        let url = format!("{}/users/{}", self.base_url, id);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
        
        if response.status() == 404 {
            return Err(ApiError::UserNotFound { id });
        }
        
        let user = response.json::<User>().await?;
        Ok(user)
    }
    
    // 사용자 생성 (C#의 async Task<User>와 유사)
    pub async fn create_user(&self, name: String, email: String) -> Result<User, ApiError> {
        if name.trim().is_empty() {
            return Err(ApiError::Validation {
                message: "Name cannot be empty".to_string(),
            });
        }
        
        let new_user = User {
            id: Uuid::new_v4(),
            name,
            email,
            created_at: Utc::now(),
        };
        
        let response = self.client
            .post(&format!("{}/users", self.base_url))
            .json(&new_user)
            .send()
            .await?;
        
        let created_user = response.json::<User>().await?;
        Ok(created_user)
    }
}

// 사용 예 (C#의 Main 메서드와 유사)
#[tokio::main]
async fn main() -> Result<(), ApiError> {
    // 로깅 초기화 (ILogger 설정과 유사)
    env_logger::init();
    
    let service = UserService::new("https://api.example.com".to_string());
    
    // 사용자 생성
    let user = service.create_user(
        "John Doe".to_string(),
        "john@example.com".to_string(),
    ).await?;
    
    println!("Created user: {:?}", user);
    
    // 사용자 조회
    let retrieved_user = service.get_user(user.id).await?;
    println!("Retrieved user: {:?}", retrieved_user);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]  // C#의 [Test] 또는 [Fact]와 비슷하다
    async fn test_user_creation() {
        let service = UserService::new("http://localhost:8080".to_string());
        
        let result = service.create_user(
            "Test User".to_string(),
            "test@example.com".to_string(),
        ).await;
        
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.name, "Test User");
        assert_eq!(user.email, "test@example.com");
    }
    
    #[test]
    fn test_validation() {
        // 동기 테스트
        let error = ApiError::Validation {
            message: "Invalid input".to_string(),
        };
        
        assert_eq!(error.to_string(), "Validation failed: Invalid input");
    }
}
```

***


<!-- ch15.1a: Serde Deep Dive for C# Developers -->
<a id="serde-deep-dive-json-serialization-for-c-developers"></a>
## Serde 심화: C# 개발자를 위한 JSON 직렬화

C# 개발자는 `System.Text.Json` 또는 `Newtonsoft.Json`에 크게 의존합니다. Rust에서는 **serde**(serialize/deserialize)가 사실상 표준 프레임워크이며, 그 속성(attribute) 시스템을 이해하면 대부분의 데이터 처리 시나리오를 다룰 수 있습니다.

<a id="basic-derive-the-starting-point"></a>
### 기본 `derive`: 출발점
```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}

let user = User { name: "Alice".into(), age: 30, email: "alice@co.com".into() };
let json = serde_json::to_string_pretty(&user)?;
let parsed: User = serde_json::from_str(&json)?;
```

```csharp
// C#에서의 대응 예
public class User
{
    public string Name { get; set; }
    public int Age { get; set; }
    public string Email { get; set; }
}
var json = JsonSerializer.Serialize(user, new JsonSerializerOptions { WriteIndented = true });
var parsed = JsonSerializer.Deserialize<User>(json);
```

<a id="field-level-attributes-like-jsonproperty"></a>
### 필드 단위 속성(attribute) (`[JsonProperty]`와 유사)

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    // JSON 출력에서 필드 이름 바꾸기 ([JsonPropertyName("user_id")]와 유사)
    #[serde(rename = "user_id")]
    id: u64,

    // 직렬화와 역직렬화에 서로 다른 이름 사용
    #[serde(rename(serialize = "userName", deserialize = "user_name"))]
    name: String,

    // 이 필드를 완전히 건너뛴다 ([JsonIgnore]와 유사)
    #[serde(skip)]
    internal_cache: Option<String>,

    // 직렬화할 때만 제외
    #[serde(skip_serializing)]
    password_hash: String,

    // JSON에 없으면 기본값 사용 (기본 생성자 값과 유사)
    #[serde(default)]
    is_active: bool,

    // 사용자 정의 기본값
    #[serde(default = "default_role")]
    role: String,

    // 중첩된 struct를 부모에 펼친다 ([JsonExtensionData]와 유사)
    #[serde(flatten)]
    metadata: Metadata,

    // 값이 None이면 직렬화하지 않는다 (null 필드 생략)
    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<String>,
}

fn default_role() -> String { "viewer".into() }

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    created_at: String,
    version: u32,
}
```

```csharp
// C#에서의 대응 속성(attribute)
public class ApiResponse
{
    [JsonPropertyName("user_id")]
    public ulong Id { get; set; }

    [JsonIgnore]
    public string? InternalCache { get; set; }

    [JsonExtensionData]
    public Dictionary<string, JsonElement>? Metadata { get; set; }
}
```

<a id="enum-representations-critical-difference-from-c"></a>
### enum 표현 방식: C#과의 중요한 차이

Rust의 serde는 enum에 대해 **네 가지 서로 다른 JSON 표현 방식**을 지원합니다. C# enum은 정수 또는 문자열로만 직렬화되는 경우가 많기 때문에, 이 부분은 직접적인 대응 개념이 없습니다.

```rust
use serde::{Deserialize, Serialize};

// 1. externally tagged (기본값) - 가장 일반적
#[derive(Serialize, Deserialize)]
enum Message {
    Text(String),
    Image { url: String, width: u32 },
    Ping,
}
// Text variant:  {"Text": "hello"}
// Image variant: {"Image": {"url": "...", "width": 100}}
// Ping variant:  "Ping"

// 2. internally tagged - 다른 언어의 discriminated union과 유사
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Event {
    Created { id: u64, name: String },
    Deleted { id: u64 },
    Updated { id: u64, fields: Vec<String> },
}
// {"type": "Created", "id": 1, "name": "Alice"}
// {"type": "Deleted", "id": 1}

// 3. adjacently tagged - 태그와 본문을 분리된 필드로 저장
#[derive(Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
enum ApiResult {
    Success(UserData),
    Error(String),
}
// {"t": "Success", "c": {"name": "Alice"}}
// {"t": "Error", "c": "not found"}

// 4. untagged - serde가 각 variant를 순서대로 시도
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum FlexibleValue {
    Integer(i64),
    Float(f64),
    Text(String),
    Bool(bool),
}
// 42, 3.14, "hello", true - serde가 variant를 자동 판별
```

<a id="custom-serialization-like-jsonconverter"></a>
### 사용자 정의 직렬화 (`JsonConverter`와 유사)
```rust
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// 특정 필드에 대한 사용자 정의 직렬화
#[derive(Serialize, Deserialize)]
struct Config {
    #[serde(serialize_with = "serialize_duration", deserialize_with = "deserialize_duration")]
    timeout: std::time::Duration,
}

fn serialize_duration<S: Serializer>(dur: &std::time::Duration, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_u64(dur.as_millis() as u64)
}

fn deserialize_duration<'de, D: Deserializer<'de>>(d: D) -> Result<std::time::Duration, D::Error> {
    let ms = u64::deserialize(d)?;
    Ok(std::time::Duration::from_millis(ms))
}
// JSON: {"timeout": 5000}  ↔  Config { timeout: Duration::from_millis(5000) }
```

<a id="container-level-attributes"></a>
### 컨테이너 단위 속성(attribute)

```rust
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]  // 모든 필드가 JSON에서 camelCase가 된다
struct UserProfile {
    first_name: String,      // → "firstName"
    last_name: String,       // → "lastName"
    email_address: String,   // → "emailAddress"
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]  // 추가 필드가 있는 JSON을 거부한다 (엄격한 파싱)
struct StrictConfig {
    port: u16,
    host: String,
}
// serde_json::from_str::<StrictConfig>(r#"{"port":8080,"host":"localhost","extra":true}"#)
// → Error: unknown field `extra`
```

<a id="quick-reference-serde-attributes"></a>
### 빠른 참고: Serde 속성

| 속성 | 수준 | C# 대응 | 용도 |
|-----------|-------|---------------|---------|
| `#[serde(rename = "...")]` | 필드 | `[JsonPropertyName]` | JSON 이름 바꾸기 |
| `#[serde(skip)]` | 필드 | `[JsonIgnore]` | 완전히 생략 |
| `#[serde(default)]` | 필드 | 기본값 | 값이 없으면 `Default::default()` 사용 |
| `#[serde(flatten)]` | 필드 | `[JsonExtensionData]` | 중첩 struct를 부모에 병합 |
| `#[serde(skip_serializing_if = "...")]` | 필드 | `JsonIgnoreCondition` | 조건부 생략 |
| `#[serde(rename_all = "camelCase")]` | 컨테이너 | `JsonSerializerOptions.PropertyNamingPolicy` | 이름 규칙 지정 |
| `#[serde(deny_unknown_fields)]` | 컨테이너 | - | 엄격한 역직렬화 |
| `#[serde(tag = "type")]` | enum | 구분자 패턴 | 내부 태깅 |
| `#[serde(untagged)]` | enum | - | variant를 순서대로 시도 |
| `#[serde(with = "...")]` | 필드 | `[JsonConverter]` | 사용자 정의 ser/de |

<a id="beyond-json-serde-works-everywhere"></a>
### JSON을 넘어: serde는 어디서나 쓸 수 있다
```rust
// 같은 derive를 모든 포맷에 그대로 쓸 수 있다 - 크레이트만 바꾸면 된다
let user = User { name: "Alice".into(), age: 30, email: "a@b.com".into() };

let json  = serde_json::to_string(&user)?;        // JSON
let toml  = toml::to_string(&user)?;               // TOML (설정 파일)
let yaml  = serde_yaml::to_string(&user)?;          // YAML
let cbor  = serde_cbor::to_vec(&user)?;             // CBOR (바이너리, compact)
let msgpk = rmp_serde::to_vec(&user)?;              // MessagePack (바이너리)

// #[derive(Serialize, Deserialize)] 한 번이면 모든 포맷을 지원할 수 있다
```

***
