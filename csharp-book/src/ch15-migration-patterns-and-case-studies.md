<a id="common-c-patterns-in-rust"></a>
## Rust에서의 C# 공통 패턴

> **이 장에서 배우는 것:** Repository 패턴, Builder 패턴, 의존성 주입, LINQ 체인, Entity Framework 쿼리, 설정 패턴을 C#에서 관용적인 Rust 코드로 어떻게 옮기는지 배웁니다.
>
> **난이도:** 🟡 중급

```mermaid
graph LR
    subgraph "C# 패턴"
        I["interface IRepo&lt;T&gt;"] --> DI["DI 컨테이너"]
        EX["try / catch"] --> LOG["ILogger"]
        LINQ["LINQ .Where().Select()"] --> LIST["List&lt;T&gt;"]
    end
    subgraph "Rust 대응"
        TR["trait Repo&lt;T&gt;"] --> GEN["Generic&lt;R: Repo&gt;"]
        RES["Result&lt;T, E&gt; + ?"] --> THISERR["thiserror / anyhow"]
        ITER[".iter().filter().map()"] --> VEC["Vec&lt;T&gt;"]
    end
    I -->|"바뀜"| TR
    EX -->|"바뀜"| RES
    LINQ -->|"바뀜"| ITER

    style TR fill:#c8e6c9,color:#000
    style RES fill:#c8e6c9,color:#000
    style ITER fill:#c8e6c9,color:#000
```

<a id="repository-pattern"></a>
### Repository 패턴
```csharp
// C# Repository 패턴
public interface IRepository<T> where T : IEntity
{
    Task<T> GetByIdAsync(int id);
    Task<IEnumerable<T>> GetAllAsync();
    Task<T> AddAsync(T entity);
    Task UpdateAsync(T entity);
    Task DeleteAsync(int id);
}

public class UserRepository : IRepository<User>
{
    private readonly DbContext _context;
    
    public UserRepository(DbContext context)
    {
        _context = context;
    }
    
    public async Task<User> GetByIdAsync(int id)
    {
        return await _context.Users.FindAsync(id);
    }
    
    // ... other implementations
}
```

```rust
// 트레잇과 제네릭을 사용한 Rust Repository 패턴
use async_trait::async_trait;
use std::fmt::Debug;

#[async_trait]
pub trait Repository<T, E> 
where 
    T: Clone + Debug + Send + Sync,
    E: std::error::Error + Send + Sync,
{
    async fn get_by_id(&self, id: u64) -> Result<Option<T>, E>;
    async fn get_all(&self) -> Result<Vec<T>, E>;
    async fn add(&self, entity: T) -> Result<T, E>;
    async fn update(&self, entity: T) -> Result<T, E>;
    async fn delete(&self, id: u64) -> Result<(), E>;
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

#[derive(Debug)]
pub enum RepositoryError {
    NotFound(u64),
    DatabaseError(String),
    ValidationError(String),
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepositoryError::NotFound(id) => write!(f, "Entity with id {} not found", id),
            RepositoryError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            RepositoryError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for RepositoryError {}

pub struct UserRepository {
    // database connection pool, etc.
}

#[async_trait]
impl Repository<User, RepositoryError> for UserRepository {
    async fn get_by_id(&self, id: u64) -> Result<Option<User>, RepositoryError> {
        // 데이터베이스 조회를 흉내 낸다
        if id == 0 {
            return Ok(None);
        }
        
        Ok(Some(User {
            id,
            name: format!("User {}", id),
            email: format!("user{}@example.com", id),
        }))
    }
    
    async fn get_all(&self) -> Result<Vec<User>, RepositoryError> {
        // 구현 위치
        Ok(vec![])
    }
    
    async fn add(&self, entity: User) -> Result<User, RepositoryError> {
        // 검증과 데이터베이스 삽입
        if entity.name.is_empty() {
            return Err(RepositoryError::ValidationError("Name cannot be empty".to_string()));
        }
        Ok(entity)
    }
    
    async fn update(&self, entity: User) -> Result<User, RepositoryError> {
        // 구현 위치
        Ok(entity)
    }
    
    async fn delete(&self, id: u64) -> Result<(), RepositoryError> {
        // 구현 위치
        Ok(())
    }
}
```

<a id="builder-pattern"></a>
### Builder 패턴
```csharp
// C# Builder 패턴 (fluent interface)
public class HttpClientBuilder
{
    private TimeSpan? _timeout;
    private string _baseAddress;
    private Dictionary<string, string> _headers = new();
    
    public HttpClientBuilder WithTimeout(TimeSpan timeout)
    {
        _timeout = timeout;
        return this;
    }
    
    public HttpClientBuilder WithBaseAddress(string baseAddress)
    {
        _baseAddress = baseAddress;
        return this;
    }
    
    public HttpClientBuilder WithHeader(string name, string value)
    {
        _headers[name] = value;
        return this;
    }
    
    public HttpClient Build()
    {
        var client = new HttpClient();
        if (_timeout.HasValue)
            client.Timeout = _timeout.Value;
        if (!string.IsNullOrEmpty(_baseAddress))
            client.BaseAddress = new Uri(_baseAddress);
        foreach (var header in _headers)
            client.DefaultRequestHeaders.Add(header.Key, header.Value);
        return client;
    }
}

// 사용 예
var client = new HttpClientBuilder()
    .WithTimeout(TimeSpan.FromSeconds(30))
    .WithBaseAddress("https://api.example.com")
    .WithHeader("Accept", "application/json")
    .Build();
```

```rust
// Rust Builder 패턴 (consuming builder)
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug)]
pub struct HttpClient {
    timeout: Duration,
    base_address: String,
    headers: HashMap<String, String>,
}

pub struct HttpClientBuilder {
    timeout: Option<Duration>,
    base_address: Option<String>,
    headers: HashMap<String, String>,
}

impl HttpClientBuilder {
    pub fn new() -> Self {
        HttpClientBuilder {
            timeout: None,
            base_address: None,
            headers: HashMap::new(),
        }
    }
    
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
    
    pub fn with_base_address<S: Into<String>>(mut self, base_address: S) -> Self {
        self.base_address = Some(base_address.into());
        self
    }
    
    pub fn with_header<K: Into<String>, V: Into<String>>(mut self, name: K, value: V) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }
    
    pub fn build(self) -> Result<HttpClient, String> {
        let base_address = self.base_address.ok_or("Base address is required")?;
        
        Ok(HttpClient {
            timeout: self.timeout.unwrap_or(Duration::from_secs(30)),
            base_address,
            headers: self.headers,
        })
    }
}

// 사용 예
let client = HttpClientBuilder::new()
    .with_timeout(Duration::from_secs(30))
    .with_base_address("https://api.example.com")
    .with_header("Accept", "application/json")
    .build()?;

// 대안: 일반적인 경우에는 Default 트레잇 사용
impl Default for HttpClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
```

***

<a id="c-to-rust-concept-mapping"></a>
## C#에서 Rust로의 개념 대응

<a id="dependency-injection--constructor-injection--traits"></a>
### 의존성 주입 → 생성자 주입 + 트레잇
```csharp
// DI 컨테이너를 사용하는 C#
services.AddScoped<IUserRepository, UserRepository>();
services.AddScoped<IUserService, UserService>();

public class UserService
{
    private readonly IUserRepository _repository;
    
    public UserService(IUserRepository repository)
    {
        _repository = repository;
    }
}
```

```rust
// Rust: 트레잇을 이용한 생성자 주입
pub trait UserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, Error>;
    async fn save(&self, user: &User) -> Result<(), Error>;
}

pub struct UserService<R> 
where 
    R: UserRepository,
{
    repository: R,
}

impl<R> UserService<R> 
where 
    R: UserRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    
    pub async fn get_user(&self, id: Uuid) -> Result<Option<User>, Error> {
        self.repository.find_by_id(id).await
    }
}

// 사용 예
let repository = PostgresUserRepository::new(pool);
let service = UserService::new(repository);
```

<a id="linq--iterator-chains"></a>
### LINQ → 이터레이터 체인
```csharp
// C# LINQ
var result = users
    .Where(u => u.Age > 18)
    .Select(u => u.Name.ToUpper())
    .OrderBy(name => name)
    .Take(10)
    .ToList();
```

```rust
// Rust: 이터레이터 체인 (제로코스트!)
let result: Vec<String> = users
    .iter()
    .filter(|u| u.age > 18)
    .map(|u| u.name.to_uppercase())
    .collect::<Vec<_>>()
    .into_iter()
    .sorted()
    .take(10)
    .collect();

// 또는 더 LINQ에 가까운 연산을 위해 itertools 크레이트 사용
use itertools::Itertools;

let result: Vec<String> = users
    .iter()
    .filter(|u| u.age > 18)
    .map(|u| u.name.to_uppercase())
    .sorted()
    .take(10)
    .collect();
```

<a id="entity-framework--sqlx--migrations"></a>
### Entity Framework → SQLx + 마이그레이션
```csharp
// C# Entity Framework
public class ApplicationDbContext : DbContext
{
    public DbSet<User> Users { get; set; }
}

var user = await context.Users
    .Where(u => u.Email == email)
    .FirstOrDefaultAsync();
```

```rust
// Rust: 컴파일 타임 검사를 지원하는 SQLx 쿼리
use sqlx::{PgPool, FromRow};

#[derive(FromRow)]
struct User {
    id: Uuid,
    email: String,
    name: String,
}

// 컴파일 타임 검사 쿼리
let user = sqlx::query_as!(
    User,
    "SELECT id, email, name FROM users WHERE email = $1",
    email
)
.fetch_optional(&pool)
.await?;

// 또는 동적 쿼리
let user = sqlx::query_as::<_, User>(
    "SELECT id, email, name FROM users WHERE email = $1"
)
.bind(email)
.fetch_optional(&pool)
.await?;
```

<a id="configuration--config-crates"></a>
### 설정 → Config 계열 크레이트
```csharp
// C# Configuration
public class AppSettings
{
    public string DatabaseUrl { get; set; }
    public int Port { get; set; }
}

var config = builder.Configuration.Get<AppSettings>();
```

```rust
// Rust: serde와 함께 사용하는 Config
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AppSettings {
    database_url: String,
    port: u16,
}

impl AppSettings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(Environment::with_prefix("APP"))
            .build()?;

        s.try_deserialize()
    }
}

// 사용 예
let settings = AppSettings::new()?;
```

---

<a id="case-studies"></a>
## 사례 연구

<a id="case-study-1-cli-tool-migration-csvtool"></a>
### 사례 연구 1: CLI 도구 마이그레이션 (`csvtool`)

**배경**: 한 팀이 큰 CSV 파일을 읽고 변환을 적용한 뒤 결과를 다시 쓰는 C# 콘솔 앱(`CsvProcessor`)을 유지보수하고 있었습니다. 파일 크기가 500MB에 이르면 메모리 사용량이 4GB까지 치솟았고, GC pause 때문에 30초씩 멈추는 현상이 있었습니다.

**마이그레이션 접근 방식**: 2주 동안 모듈 하나씩 Rust로 다시 작성했습니다.

| 단계 | 바뀐 점 | C# → Rust |
|------|-------------|-----------|
| 1 | CSV 파싱 | `CsvHelper` → `csv` crate (streaming `Reader`) |
| 2 | 데이터 모델 | `class Record` → `struct Record` (stack-allocated, `#[derive(Deserialize)]`) |
| 3 | 변환 | LINQ `.Select().Where()` → `.iter().map().filter()` |
| 4 | 파일 I/O | `StreamReader` → `BufReader<File>` with `?` error propagation |
| 5 | CLI 인자 | `System.CommandLine` → `clap` with derive macros |
| 6 | 병렬 처리 | `Parallel.ForEach` → `rayon`의 `.par_iter()` |

**결과**:
- 메모리: 4 GB → 12 MB (전체 파일을 올리지 않고 스트리밍 처리)
- 속도: 500MB 파일 기준 45초 → 3초
- 바이너리 크기: 런타임 의존성 없는 단일 2MB 실행 파일

**핵심 교훈**: 가장 큰 이점은 Rust 그 자체라기보다, Rust의 소유권 모델이 *스트리밍 설계로 강제*했다는 점이었습니다. C#에서는 모든 데이터를 `.ToList()`로 메모리에 올리기 쉬웠지만, Rust에서는 borrow checker가 자연스럽게 `Iterator` 기반 처리 쪽으로 이끌었습니다.

<a id="case-study-2-microservice-replacement-auth-gateway"></a>
### 사례 연구 2: 마이크로서비스 교체 (`auth-gateway`)

**배경**: C# ASP.NET Core로 작성된 인증 게이트웨이가 50개 이상의 백엔드 서비스에 대해 JWT 검증과 rate limiting을 처리하고 있었습니다. 초당 1만 요청 수준에서 p99 지연 시간이 GC 급증과 함께 200ms까지 올라갔습니다.

**마이그레이션 접근 방식**: API 계약은 그대로 유지한 채, `axum` + `tower` 기반 Rust 서비스로 교체했습니다.

```rust
// 이전(C#):  services.AddAuthentication().AddJwtBearer(...)
// 이후(Rust):  tower 미들웨어 레이어

use axum::{Router, middleware};
use tower::ServiceBuilder;

let app = Router::new()
    .route("/api/*path", any(proxy_handler))
    .layer(
        ServiceBuilder::new()
            .layer(middleware::from_fn(validate_jwt))
            .layer(middleware::from_fn(rate_limit))
    );
```

| 지표 | C# (ASP.NET Core) | Rust (`axum`) |
|--------|-------------------|-------------|
| p50 latency | 5ms | 0.8ms |
| p99 latency | 200ms (GC spikes) | 4ms |
| Memory | 300 MB | 8 MB |
| Docker image | 210 MB (.NET runtime) | 12 MB (static binary) |
| Cold start | 2.1s | 0.05s |

**핵심 교훈**:
1. **같은 API 계약을 유지하라** - 클라이언트 변경이 필요 없었고, Rust 서비스는 drop-in replacement가 될 수 있었습니다.
2. **핫패스부터 시작하라** - 병목은 JWT 검증이었습니다. 그 미들웨어 하나만 옮겨도 성능 이득의 80%를 얻을 수 있었습니다.
3. **`tower` 미들웨어를 활용하라** - ASP.NET Core의 미들웨어 파이프라인과 구조가 비슷해 C# 개발자에게 익숙한 아키텍처였습니다.
4. **p99 지연 시간 개선의 핵심은 더 빠른 코드가 아니라 GC pause 제거였다** - steady-state 처리량 자체는 2배 정도 빨라진 수준이었지만, GC가 없어 tail latency가 훨씬 예측 가능해졌습니다.

---

<a id="exercises"></a>
## 연습문제

<details>
<summary><strong>🏋️ 연습문제: C# 서비스를 Rust로 옮기기</strong> (클릭하여 펼치기)</summary>

다음 C# 서비스를 관용적인 Rust 코드로 옮겨 보세요.

```csharp
public interface IUserService
{
    Task<User?> GetByIdAsync(int id);
    Task<List<User>> SearchAsync(string query);
}

public class UserService : IUserService
{
    private readonly IDatabase _db;
    public UserService(IDatabase db) { _db = db; }

    public async Task<User?> GetByIdAsync(int id)
    {
        try { return await _db.QuerySingleAsync<User>(id); }
        catch (NotFoundException) { return null; }
    }

    public async Task<List<User>> SearchAsync(string query)
    {
        return await _db.QueryAsync<User>($"SELECT * WHERE name LIKE '%{query}%'");
    }
}
```

**힌트**: 트레잇을 사용하고, null 대신 `Option<User>`, `try/catch` 대신 `Result`를 사용하세요. 그리고 SQL injection 취약점도 함께 고치세요.

<details>
<summary>🔑 해답</summary>

```rust
use async_trait::async_trait;

#[derive(Debug, Clone)]
struct User { id: i64, name: String }

#[async_trait]
trait Database: Send + Sync {
    async fn get_user(&self, id: i64) -> Result<Option<User>, sqlx::Error>;
    async fn search_users(&self, query: &str) -> Result<Vec<User>, sqlx::Error>;
}

#[async_trait]
trait UserService: Send + Sync {
    async fn get_by_id(&self, id: i64) -> Result<Option<User>, AppError>;
    async fn search(&self, query: &str) -> Result<Vec<User>, AppError>;
}

struct UserServiceImpl<D: Database> {
    db: D,  // Arc가 필요 없다 - Rust의 소유권이 이를 처리한다
}

#[async_trait]
impl<D: Database> UserService for UserServiceImpl<D> {
    async fn get_by_id(&self, id: i64) -> Result<Option<User>, AppError> {
        // null 대신 Option, try/catch 대신 Result
        Ok(self.db.get_user(id).await?)
    }

    async fn search(&self, query: &str) -> Result<Vec<User>, AppError> {
        // 매개변수화 쿼리 - SQL injection 없음!
        // (sqlx는 문자열 보간이 아니라 $1 플레이스홀더를 사용한다)
        self.db.search_users(query).await.map_err(Into::into)
    }
}
```

**C#에서 바뀐 핵심 포인트**:
- `null` → `Option<User>` (컴파일 타임 null 안전성)
- `try/catch` → `Result` + `?` (명시적 에러 전파)
- SQL injection 제거: 문자열 보간 대신 매개변수화 쿼리 사용
- `IDatabase _db` → 제네릭 `D: Database` (정적 디스패치, boxing 없음)

</details>
</details>

***
