<a id="constructor-patterns"></a>
## 생성자 패턴

> **학습할 내용:** 전통적인 생성자 없이 Rust 구조체를 만드는 방법, `new()` 관례,
> `Default` 트레잇, 팩터리 메서드, 그리고 복잡한 초기화를 위한 빌더 패턴.
>
> **난이도:** 🟢 입문

### C# 생성자 패턴
```csharp
public class Configuration
{
    public string DatabaseUrl { get; set; }
    public int MaxConnections { get; set; }
    public bool EnableLogging { get; set; }
    
    // 기본 생성자
    public Configuration()
    {
        DatabaseUrl = "localhost";
        MaxConnections = 10;
        EnableLogging = false;
    }
    
    // 매개변수 생성자
    public Configuration(string databaseUrl, int maxConnections)
    {
        DatabaseUrl = databaseUrl;
        MaxConnections = maxConnections;
        EnableLogging = false;
    }
    
    // 팩터리 메서드
    public static Configuration ForProduction()
    {
        return new Configuration("prod.db.server", 100)
        {
            EnableLogging = true
        };
    }
}
```

### Rust 생성자 패턴
```rust
#[derive(Debug)]
pub struct Configuration {
    pub database_url: String,
    pub max_connections: u32,
    pub enable_logging: bool,
}

impl Configuration {
    // 기본 생성자
    pub fn new() -> Configuration {
        Configuration {
            database_url: "localhost".to_string(),
            max_connections: 10,
            enable_logging: false,
        }
    }
    
    // 매개변수를 받는 생성자
    pub fn with_database(database_url: String, max_connections: u32) -> Configuration {
        Configuration {
            database_url,
            max_connections,
            enable_logging: false,
        }
    }
    
    // 팩터리 메서드
    pub fn for_production() -> Configuration {
        Configuration {
            database_url: "prod.db.server".to_string(),
            max_connections: 100,
            enable_logging: true,
        }
    }
    
    // 빌더 패턴 메서드
    pub fn enable_logging(mut self) -> Configuration {
        self.enable_logging = true;
        self  // 체이닝을 위해 self 반환
    }
    
    pub fn max_connections(mut self, count: u32) -> Configuration {
        self.max_connections = count;
        self
    }
}

// Default 트레잇 구현
impl Default for Configuration {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    // 다양한 생성 패턴
    let config1 = Configuration::new();
    let config2 = Configuration::with_database("localhost:5432".to_string(), 20);
    let config3 = Configuration::for_production();
    
    // 빌더 패턴
    let config4 = Configuration::new()
        .enable_logging()
        .max_connections(50);
    
    // Default 트레잇 사용
    let config5 = Configuration::default();
    
    println!("{:?}", config4);
}
```

### 빌더 패턴 구현
```rust
// 더 복잡한 빌더 패턴 예제
#[derive(Debug)]
pub struct DatabaseConfig {
    host: String,
    port: u16,
    username: String,
    password: Option<String>,
    ssl_enabled: bool,
    timeout_seconds: u64,
}

pub struct DatabaseConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    username: Option<String>,
    password: Option<String>,
    ssl_enabled: bool,
    timeout_seconds: u64,
}

impl DatabaseConfigBuilder {
    pub fn new() -> Self {
        DatabaseConfigBuilder {
            host: None,
            port: None,
            username: None,
            password: None,
            ssl_enabled: false,
            timeout_seconds: 30,
        }
    }
    
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }
    
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }
    
    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }
    
    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }
    
    pub fn enable_ssl(mut self) -> Self {
        self.ssl_enabled = true;
        self
    }
    
    pub fn timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = seconds;
        self
    }
    
    pub fn build(self) -> Result<DatabaseConfig, String> {
        let host = self.host.ok_or("Host is required")?;
        let port = self.port.ok_or("Port is required")?;
        let username = self.username.ok_or("Username is required")?;
        
        Ok(DatabaseConfig {
            host,
            port,
            username,
            password: self.password,
            ssl_enabled: self.ssl_enabled,
            timeout_seconds: self.timeout_seconds,
        })
    }
}

fn main() {
    let config = DatabaseConfigBuilder::new()
        .host("localhost")
        .port(5432)
        .username("admin")
        .password("secret123")
        .enable_ssl()
        .timeout(60)
        .build()
        .expect("Failed to build config");
    
    println!("{:?}", config);
}
```

---

## 연습문제

<details>
<summary><strong>🏋️ 연습문제: 검증이 있는 빌더</strong> (펼쳐서 보기)</summary>

다음 조건을 만족하는 `EmailBuilder`를 만들어 보세요.
1. `to`와 `subject`는 필수입니다(builder가 그것들 없이 컴파일되지 않게 typestate를 쓰거나, `build()`에서 검증하세요).
2. `body`와 `cc`는 선택 사항입니다(`cc`는 주소들의 `Vec`).
3. `build()`는 `Result<Email, String>`을 반환해야 하며, 비어 있는 `to` 또는 `subject`를 거부해야 합니다.
4. 잘못된 입력이 거부된다는 점을 보여 주는 테스트를 작성하세요.

<details>
<summary>🔑 해답</summary>

```rust
#[derive(Debug)]
struct Email {
    to: String,
    subject: String,
    body: Option<String>,
    cc: Vec<String>,
}

#[derive(Default)]
struct EmailBuilder {
    to: Option<String>,
    subject: Option<String>,
    body: Option<String>,
    cc: Vec<String>,
}

impl EmailBuilder {
    fn new() -> Self { Self::default() }

    fn to(mut self, to: impl Into<String>) -> Self {
        self.to = Some(to.into()); self
    }
    fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into()); self
    }
    fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into()); self
    }
    fn cc(mut self, addr: impl Into<String>) -> Self {
        self.cc.push(addr.into()); self
    }
    fn build(self) -> Result<Email, String> {
        let to = self.to.filter(|s| !s.is_empty())
            .ok_or("'to' is required")?;
        let subject = self.subject.filter(|s| !s.is_empty())
            .ok_or("'subject' is required")?;
        Ok(Email { to, subject, body: self.body, cc: self.cc })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn valid_email() {
        let email = EmailBuilder::new()
            .to("alice@example.com")
            .subject("Hello")
            .build();
        assert!(email.is_ok());
    }
    #[test]
    fn missing_to_fails() {
        let email = EmailBuilder::new().subject("Hello").build();
        assert!(email.is_err());
    }
}
```

</details>
</details>

***