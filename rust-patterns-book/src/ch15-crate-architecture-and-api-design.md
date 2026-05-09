<a id="crate-architecture-and-api-design"></a>

# 15. 크레이트 아키텍처와 API 설계 🟡

> **이 장에서 배울 내용:**
> - 모듈 배치 관례와 재수출(re-export) 전략
> - 다듬어진 크레이트를 위한 공개 API 설계 체크리스트
> - 인체공학적 매개변수 패턴: `impl Into`, `AsRef`, `Cow`
> - `TryFrom`과 검증된 타입으로 하는 “파싱하고 검증하지 말 것”
> - 기능 플래그, 조건부 컴파일, 워크스페이스 구성

<a id="module-layout-conventions"></a>

## 모듈 배치 관례

```text
my_crate/
├── Cargo.toml
├── src/
│   ├── lib.rs          # 크레이트 루트 — 재수출과 공개 API
│   ├── config.rs       # 기능 모듈
│   ├── parser/         # 하위 모듈이 있는 복잡한 모듈
│   │   ├── mod.rs      # 또는 부모에 parser.rs (Rust 2018+)
│   │   ├── lexer.rs
│   │   └── ast.rs
│   ├── error.rs        # 에러 타입
│   └── utils.rs        # 내부 헬퍼 (pub(crate))
├── tests/
│   └── integration.rs  # 통합 테스트
├── benches/
│   └── perf.rs         # 벤치마크
└── examples/
    └── basic.rs        # cargo run --example basic
```

```rust
// lib.rs — 재수출로 공개 API를 구성합니다
mod config;
mod error;
mod parser;
mod utils;

// 사용자에게 필요한 것만 재수출:
pub use config::Config;
pub use error::Error;
pub use parser::Parser;

// 공개 타입은 크레이트 루트에 — 사용자는 이렇게 씁니다:
// use my_crate::Config;
// use my_crate::config::Config; 가 아님
```

**가시성 한정자**:

| 한정자 | 보이는 범위 |
|----------|-----------|
| `pub` | 모두 |
| `pub(crate)` | 이 크레이트만 |
| `pub(super)` | 부모 모듈 |
| `pub(in path)` | 특정 조상 모듈 |
| (없음) | 현재 모듈과 자식 |

<a id="public-api-design-checklist"></a>

### 공개 API 설계 체크리스트

1. **참조를 받고 소유권을 반환** — `fn process(input: &str) -> String`
2. **매개변수에 `impl Trait` 사용** — 서명을 깔끔하게 하려면 `fn read(r: impl Read)`가 `fn read<R: Read>(r: R)`보다 낫습니다
3. **`Result`를 반환하고 `panic!`은 피하기** — 호출자가 에러 처리 방식을 정하도록
4. **표준 트레잇 구현** — `Debug`, `Display`, `Clone`, `Default`, `From`/`Into`
5. **잘못된 상태를 표현 불가능하게** — 타입 상태와 뉴타입 활용
6. **복잡한 설정은 빌더 패턴** — 필수 필드가 있으면 타입 상태 빌더
7. **사용자 구현을 막을 트레잇은 봉인(seal)** — `pub trait Sealed: private::Sealed {}`
8. **타입·함수에 `#[must_use]`** — 중요한 `Result`, 가드, 값을 조용히 버리는 실수 방지. 반환을 무시하면 거의 항상 버그인 타입에 적용합니다:
   ```rust
   #[must_use = "dropping the guard immediately releases the lock"]
   pub struct LockGuard<'a, T> { /* ... */ }

   #[must_use]
   pub fn validate(input: &str) -> Result<ValidInput, ValidationError> { /* ... */ }
   ```

```rust
// 봉인 트레잇 패턴 — 사용은 가능, 구현은 불가:
mod private {
    pub trait Sealed {}
}

pub trait DatabaseDriver: private::Sealed {
    fn connect(&self, url: &str) -> Connection;
}

// 이 크레이트의 타입만 Sealed를 구현할 수 있음 → DatabaseDriver도 우리만 구현
pub struct PostgresDriver;
impl private::Sealed for PostgresDriver {}
impl DatabaseDriver for PostgresDriver {
    fn connect(&self, url: &str) -> Connection { /* ... */ }
}
```

> **`#[non_exhaustive]`** — 공개 enum·구조체에 붙이면 변형·필드 추가가
> 호환성 깨짐(semver major)이 아닙니다. 하위 크레이트는 match에 와일드카드
> (`_ =>`)를 써야 하고, 구조체 리터럴로는 생성할 수 없습니다:
> ```rust
> #[non_exhaustive]
> pub enum DiagError {
>     Timeout,
>     HardwareFault,
>     // 이후 릴리스에서 변형을 추가해도 semver 위반이 아님
> }
> ```

<a id="ergonomic-parameter-patterns"></a>

### 인체공학적 매개변수 — `impl Into`, `AsRef`, `Cow`

Rust에서 영향이 큰 API 패턴 중 하나는 함수 매개변수에 **가장 일반적인 타입**을 받아
호출부에서 매번 `.to_string()`, `&*s`, `.as_ref()`를 반복하지 않게 하는 것입니다.
“받을 때는 관대하게”의 Rust 버전입니다.

#### `impl Into<T>` — 변환 가능한 것은 무엇이든

```rust
// ❌ 마찰: 호출자가 직접 변환해야 함
fn connect(host: String, port: u16) -> Connection {
    // ...
}
connect("localhost".to_string(), 5432);  // 귀찮은 .to_string()
connect(hostname.clone(), 5432);       // 이미 String이면 불필요한 clone

// ✅ 인체공학: String으로 변환 가능한 것이면 무엇이든
fn connect(host: impl Into<String>, port: u16) -> Connection {
    let host = host.into();  // 함수 안에서 한 번만 변환
    // ...
}
connect("localhost", 5432);     // &str — 마찰 없음
connect(hostname, 5432);        // String — 이동, clone 없음
connect(arc_str, 5432);         // Arc<str> — From이 구현된 경우
```

`From`/`Into` 쌍의 blanket 구현 덕분에 동작합니다. `impl Into<T>`는 “`T`가 될 수 있는 것”을 달라는 뜻입니다.

#### `AsRef<T>` — 참조로 빌리기

`AsRef<T>`는 `Into<T>`의 빌리기 쪽입니다. 소유가 아니라 *읽기만* 할 때 씁니다:

```rust
use std::path::Path;

// ❌ 호출자를 &Path로 강제
fn file_exists(path: &Path) -> bool {
    path.exists()
}
file_exists(Path::new("/tmp/test.txt"));  // 어색함

// ✅ &Path처럼 쓸 수 있는 것이면 무엇이든
fn file_exists(path: impl AsRef<Path>) -> bool {
    path.as_ref().exists()
}
file_exists("/tmp/test.txt");                    // &str ✅
file_exists(String::from("/tmp/test.txt"));      // String ✅
file_exists(Path::new("/tmp/test.txt"));         // &Path ✅
file_exists(PathBuf::from("/tmp/test.txt"));     // PathBuf ✅

// 문자열 계열도 같은 패턴:
fn log_message(msg: impl AsRef<str>) {
    println!("[LOG] {}", msg.as_ref());
}
log_message("hello");                    // &str ✅
log_message(String::from("hello"));      // String ✅
```

#### `Cow<T>` — 쓸 때만 복제

`Cow<'a, T>`(Clone on Write)는 수정이 필요할 때까지 할당을 미룹니다.
`&T`를 빌리거나 `T::Owned`를 가집니다. 대부분의 호출에서 데이터를 바꾸지 않을 때 적합합니다:

```rust
use std::borrow::Cow;

/// 진단 메시지 정규화 — 바꿀 필요가 있을 때만 할당
fn normalize_message(msg: &str) -> Cow<'_, str> {
    if msg.contains('\t') || msg.contains('\r') {
        Cow::Owned(msg.replace('\t', "    ").replace('\r', ""))
    } else {
        Cow::Borrowed(msg)
    }
}

// 대부분의 메시지는 할당 없이 통과:
let clean = normalize_message("All tests passed");          // 빌림 — 비용 없음
let fixed = normalize_message("Error:\tfailed\r\n");        // 소유 — 할당됨

// Cow<str>는 Deref<Target=str>이므로 &str처럼 씁니다:
println!("{}", clean);
println!("{}", fixed.to_uppercase());
```

#### 빠른 참고: 무엇을 쓸까

```text
함수 안에서 데이터 소유가 필요한가?
├── 예 → impl Into<T>
│         "`T`가 될 수 있는 것을 달라"
└── 아니오 → 읽기만 하나?
     ├── 예 → impl AsRef<T> 또는 &T
     │         "`&T`로 빌릴 수 있는 것을 달라"
     └── 가끔 수정?
          └── Cow<'_, T>
              "가능하면 빌리고, 꼭 필요할 때만 복제"
```

| 패턴 | 소유 | 할당 | 쓸 때 |
|---------|-----------|------------|-------------|
| `&str` | 빌림 | 없음 | 단순 문자열 인자 |
| `impl AsRef<str>` | 빌림 | 없음 | String, &str 등 — 읽기만 |
| `impl Into<String>` | 소유 | 변환 시 | &str, String — 저장/소유 |
| `Cow<'_, str>` | 둘 다 | 수정 시만 | 대개 수정 없는 처리 |
| `&[u8]` / `impl AsRef<[u8]>` | 빌림 | 없음 | 바이트 지향 API |

> **`Borrow<T>` vs `AsRef<T>`**: 둘 다 `&T`를 주지만 `Borrow<T>`는 원본과 빌린 형태에서
> `Eq`, `Ord`, `Hash`가 **일치**함을 추가로 보장합니다. 그래서 `HashMap<String, V>::get()`은
> `String: Borrow<Q>`인 `&Q`를 받고 `AsRef`가 아닙니다. 조회 키에는 `Borrow`, 일반적인 “참조 달라”에는 `AsRef`를 쓰세요.

#### API에서 변환 조합하기

```rust
/// 인체공학적 매개변수를 쓴 진단 API 예:
pub struct DiagRunner {
    name: String,
    config_path: PathBuf,
}

impl DiagRunner {
    /// 이름은 문자열 계열, 설정 경로는 경로 계열 아무거나
    pub fn new(
        name: impl Into<String>,
        config_path: impl Into<PathBuf>,
    ) -> Self {
        DiagRunner {
            name: name.into(),
            config_path: config_path.into(),
        }
    }

    /// 읽기 전용 조회에는 AsRef<str>
    pub fn get_result(&self, test_name: impl AsRef<str>) -> Option<&TestResult> {
        self.results.get(test_name.as_ref())
    }
}

// 호출부 마찰 없음:
let runner = DiagRunner::new("GPU Diag", "/etc/diag_tool/config.json");
let runner = DiagRunner::new(format!("Diag-{}", node_id), config_path);
let runner = DiagRunner::new(name_string, path_buf);
```

***

<a id="case-study-designing-a-public-crate-api"></a>

## 사례 연구: 공개 크레이트 API 설계 — 전과 후

문자열에 의존하던 내부 API를 인체공학적이고 타입 안전한 공개 API로 바꾸는 예입니다. 설정 파서 크레이트를 가정합니다.

**전** (문자열 중심, 오용하기 쉬움):

```rust
// ❌ 매개변수가 모두 문자열 — 컴파일 타임 검증 없음
pub fn parse_config(path: &str, format: &str, strict: bool) -> Result<Config, String> {
    // 허용 포맷이 "json"? "JSON"? "Json"?
    // path는 파일 경로인가 URL인가?
    // strict는 정확히 무슨 뜻인가?
    todo!()
}
```

**후** (타입 안전, 자기 설명적):

```rust
use std::path::Path;

/// 지원하는 설정 포맷
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]  // 포맷 추가 시 하위 호환 유지
pub enum Format {
    Json,
    Toml,
    Yaml,
}

/// 파싱 엄격도
#[derive(Debug, Clone, Copy, Default)]
pub enum Strictness {
    /// 알 수 없는 필드 거부(라이브러리 기본)
    #[default]
    Strict,
    /// 알 수 없는 필드 무시(앞으로 호환되는 설정에 유용)
    Lenient,
}

pub fn parse_config(
    path: &Path,          // 타입으로 강제: 파일 시스템 경로
    format: Format,       // enum: 잘못된 포맷 문자열 불가
    strictness: Strictness,  // 이름 있는 대안, raw bool 아님
) -> Result<Config, ConfigError> {
    todo!()
}
```

**개선된 점**:

| 측면 | 전 | 후 |
|--------|--------|-------|
| 포맷 검증 | 런타임 문자열 비교 | 컴파일 타임 enum |
| 경로 타입 | raw `&str` (무엇이든 될 수 있음) | `&Path` (파일 시스템 전용) |
| 엄격도 | 의미 불명 `bool` | 자기 설명 enum |
| 에러 타입 | 불투명 `String` | 구조화된 `ConfigError` |
| 확장성 | 호환 깨지는 변경 | `#[non_exhaustive]` |

> **경험칙**: 문자열에 `match`를 쓰고 있다면 매개변수를 enum으로 바꿔 보라는 신호입니다.
> 문맥 없이 bool이 불명확하면 두 변형 enum을 쓰세요.

***

<a id="parse-dont-validate"></a>

### 파싱하고 검증하지 말 것 — `TryFrom`과 검증된 타입

“Parse, don't validate”는 **데이터를 검사한 뒤에도 raw 형태로 돌려보내지 말고,
유효할 때만 존재할 수 있는 타입으로 파싱하라**는 원칙입니다. Rust에서는 `TryFrom`이 표준 도구입니다.

#### 문제: 검증만 하고 강제는 없음

```rust
// ❌ 검증 후 사용: 검사 후에도 잘못된 값 사용을 막지 못함
fn process_port(port: u16) {
    if port == 0 || port > 65535 {
        panic!("Invalid port");
    }
    start_server(port);  // 그런데 누가 start_server(0)를 직접 부르면?
}

// ❌ 문자열 중심: 이메일이 String일 뿐 — 쓰레기도 통과
fn send_email(to: String, body: String) {
    // `to`가 진짜 이메일인지 알 수 없음
}
```

#### 해결: `TryFrom`으로 검증된 뉴타입

```rust
use std::convert::TryFrom;
use std::fmt;

/// 검증된 TCP 포트(1–65535). `Port`를 가지면 유효하다고 가정할 수 있음
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Port(u16);

impl TryFrom<u16> for Port {
    type Error = PortError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(PortError::Zero)
        } else {
            Ok(Port(value))
        }
    }
}

impl Port {
    pub fn get(&self) -> u16 { self.0 }
}

#[derive(Debug)]
pub enum PortError {
    Zero,
    InvalidFormat,
}

impl fmt::Display for PortError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PortError::Zero => write!(f, "port must be non-zero"),
            PortError::InvalidFormat => write!(f, "invalid port format"),
        }
    }
}

impl std::error::Error for PortError {}

// 타입 시스템이 유효성을 강제:
fn start_server(port: Port) {
    println!("Listening on port {}", port.get());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = Port::try_from(8080)?;   // 경계에서 한 번만 검증
    start_server(port);

    let bad = Port::try_from(0);        // ❌ Err(PortError::Zero)
    Ok(())
}
```

#### 실무 예: 검증된 IPMI 주소

```rust
/// 검증된 IPMI 슬레이브 주소(0x20–0xFE, 짝수만)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IpmiAddr(u8);

#[derive(Debug)]
pub enum IpmiAddrError {
    Odd(u8),
    OutOfRange(u8),
}

impl fmt::Display for IpmiAddrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpmiAddrError::Odd(v) => write!(f, "IPMI address 0x{v:02X} must be even"),
            IpmiAddrError::OutOfRange(v) => {
                write!(f, "IPMI address 0x{v:02X} out of range (0x20..=0xFE)")
            }
        }
    }
}

impl TryFrom<u8> for IpmiAddr {
    type Error = IpmiAddrError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value % 2 != 0 {
            Err(IpmiAddrError::Odd(value))
        } else if value < 0x20 || value > 0xFE {
            Err(IpmiAddrError::OutOfRange(value))
        } else {
            Ok(IpmiAddr(value))
        }
    }
}

impl IpmiAddr {
    pub fn get(&self) -> u8 { self.0 }
}

fn send_ipmi_command(addr: IpmiAddr, cmd: u8, data: &[u8]) -> Result<Vec<u8>, IpmiError> {
    raw_ipmi_send(addr.get(), cmd, data)
}
```

#### `FromStr`로 문자열 파싱

CLI·설정 파일에서 자주 쓰이면 `FromStr`을 구현합니다:

```rust
use std::str::FromStr;

impl FromStr for Port {
    type Err = PortError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n: u16 = s.parse().map_err(|_| PortError::InvalidFormat)?;
        Port::try_from(n)
    }
}

let port: Port = "8080".parse()?;

// clap:
// #[derive(Parser)]
// struct Args {
//     #[arg(short, long)]
//     port: Port,
// }
```

#### 복합 검증의 `TryFrom` 체인

```rust
// 이 예의 스텁 타입 — 실제로는 모듈별 TryFrom이 있음
```

```rust
# struct Hostname(String);
# impl TryFrom<String> for Hostname {
#     type Error = String;
#     fn try_from(s: String) -> Result<Self, String> { Ok(Hostname(s)) }
# }
# struct Timeout(u64);
# impl TryFrom<u64> for Timeout {
#     type Error = String;
#     fn try_from(ms: u64) -> Result<Self, String> {
#         if ms == 0 { Err("timeout must be > 0".into()) } else { Ok(Timeout(ms)) }
#     }
# }
# struct RawConfig { host: String, port: u16, timeout_ms: u64 }
# #[derive(Debug)]
# enum ConfigError {
#     InvalidHost(String),
#     InvalidPort(PortError),
#     InvalidTimeout(String),
# }
# impl From<std::io::Error> for ConfigError {
#     fn from(e: std::io::Error) -> Self { ConfigError::InvalidHost(e.to_string()) }
# }
# impl From<serde_json::Error> for ConfigError {
#     fn from(e: serde_json::Error) -> Self { ConfigError::InvalidHost(e.to_string()) }
# }
/// 모든 필드가 유효할 때만 존재할 수 있는 설정
pub struct ValidConfig {
    pub host: Hostname,
    pub port: Port,
    pub timeout_ms: Timeout,
}

impl TryFrom<RawConfig> for ValidConfig {
    type Error = ConfigError;

    fn try_from(raw: RawConfig) -> Result<Self, Self::Error> {
        Ok(ValidConfig {
            host: Hostname::try_from(raw.host)
                .map_err(ConfigError::InvalidHost)?,
            port: Port::try_from(raw.port)
                .map_err(ConfigError::InvalidPort)?,
            timeout_ms: Timeout::try_from(raw.timeout_ms)
                .map_err(ConfigError::InvalidTimeout)?,
        })
    }
}

fn load_config(path: &str) -> Result<ValidConfig, ConfigError> {
    let raw: RawConfig = serde_json::from_str(&std::fs::read_to_string(path)?)?;
    ValidConfig::try_from(raw)
}
```

#### 요약: 검증 vs 파싱

| 접근 | 데이터 검사? | 컴파일러가 유효성 강제? | 재검증? |
|----------|:---:|:---:|:---:|
| 런타임 검사(if/assert) | ✅ | ❌ | 함수 경계마다 |
| 검증 뉴타입 + `TryFrom` | ✅ | ✅ | 없음 — 타입이 증명 |

규칙: **경계에서 파싱하고, 내부에서는 검증된 타입만 씁니다.**
raw 문자열·정수·바이트 슬라이스는 `TryFrom`/`FromStr`로 파싱된 뒤부터 타입이 보장합니다.

<a id="feature-flags-and-conditional-compilation"></a>

### 기능 플래그와 조건부 컴파일

```toml
```

# Cargo.toml
[features]
default = ["json"]          # 기본 활성화
json = ["dep:serde_json"]   # JSON 지원
xml = ["dep:quick-xml"]     # XML 지원
full = ["json", "xml"]      # 메타 기능: 전부

[dependencies]
serde = "1"
serde_json = { version = "1", optional = true }
quick-xml = { version = "0.31", optional = true }

```rust
#[cfg(feature = "json")]
pub fn to_json<T: serde::Serialize>(value: &T) -> String {
    serde_json::to_string(value).unwrap()
}

#[cfg(feature = "xml")]
pub fn to_xml<T: serde::Serialize>(value: &T) -> String {
    quick_xml::se::to_string(value).unwrap()
}

#[cfg(not(any(feature = "json", feature = "xml")))]
compile_error!("At least one format feature (json, xml) must be enabled");
```

**권장 사항**:
- `default` 기능은 최소로 — 사용자가 옵트인
- 선택 의존성에는 Rust 1.60+ `dep:` 문법으로 암시적 기능 생성 방지
- README와 크레이트 문서에 기능을 설명

<a id="workspace-organization"></a>

### 워크스페이스 구성

큰 프로젝트는 Cargo 워크스페이스로 의존성과 빌드 산출물을 공유합니다:

```toml
```

# 루트 Cargo.toml
[workspace]
members = [
    "core",         # 공유 타입·트레잇
    "parser",       # 파싱 라이브러리
    "server",       # 바이너리 — 메인 앱
    "client",       # 클라이언트 라이브러리
    "cli",          # CLI 바이너리
]

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
tracing = "0.1"

# 각 멤버 Cargo.toml:
# [dependencies]
# serde = { workspace = true }

```rust

**이점**:
```

- 하나의 `Cargo.lock` — 모든 크레이트가 같은 의존성 버전
- `cargo test --workspace`로 전체 테스트
- 공유 빌드 캐시 — 한 크레이트 컴파일이 모두에 도움
- 구성 요소 간 의존 경계가 명확

<a id="cargoconfigtoml-project-level-configuration"></a>

### `.cargo/config.toml`: 프로젝트 단위 설정

워크스페이스 루트나 `$HOME/.cargo/`의 `.cargo/config.toml`로
`Cargo.toml`을 바꾸지 않고 Cargo 동작을 조정합니다:

```toml
```

# .cargo/config.toml

[build]
target = "x86_64-unknown-linux-gnu"

[target.aarch64-unknown-linux-gnu]
runner = "qemu-aarch64-static"
linker = "aarch64-linux-gnu-gcc"

[alias]
xt = "test --workspace --release"
ci = "clippy --workspace -- -D warnings"
cov = "llvm-cov --workspace"

[env]
IPMI_LIB_PATH = "/usr/lib/bmc"

# [registries.internal]
# index = "https://gitlab.internal/crates/index"

```rust

자주 쓰는 설정:

```

| 설정 | 목적 | 예 |
|---------|---------|---------|
| `[build] target` | 기본 컴파일 타깃 | 정적 빌드에 `x86_64-unknown-linux-musl` |
| `[target.X] runner` | 바이너리 실행 방법 | 크로스 빌드에 `"qemu-aarch64-static"` |
| `[target.X] linker` | 사용할 링커 | `"aarch64-linux-gnu-gcc"` |
| `[alias]` | `cargo` 하위 명령 단축 | `xt = "test --workspace"` |
| `[env]` | 빌드 시 환경 변수 | 라이브러리 경로, 기능 토글 |
| `[net] offline` | 네트워크 차단 | 에어갭 빌드에 `true` |

<a id="compile-time-environment-variables"></a>

### 컴파일 타임 환경 변수: `env!()`와 `option_env!()`

버전 문자열, 빌드 메타데이터에 바이너리 안에 환경 변수를 넣을 수 있습니다:

```rust
const VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");

const BUILD_SHA: Option<&str> = option_env!("GIT_SHA");
const BUILD_TIME: Option<&str> = option_env!("BUILD_TIMESTAMP");

fn print_version() {
    println!("{PKG_NAME} v{VERSION}");
    if let Some(sha) = BUILD_SHA {
        println!("  commit: {sha}");
    }
    if let Some(time) = BUILD_TIME {
        println!("  built:  {time}");
    }
}
```

Cargo가 자주 쓰는 변수:

| 변수 | 값 | 용도 |
|----------|-------|----------|
| `CARGO_PKG_VERSION` | `"1.2.3"` | 버전 표시 |
| `CARGO_PKG_NAME` | `"diag_tool"` | 바이너리 식별 |
| `CARGO_PKG_AUTHORS` | `Cargo.toml`에서 | 도움말 등 |
| `CARGO_MANIFEST_DIR` | `Cargo.toml` 절대 경로 | 테스트 데이터 위치 |
| `OUT_DIR` | 빌드 출력 | `build.rs` 생성물 |
| `TARGET` | 타깃 트리플 | `build.rs`에서 플랫폼 분기 |

`build.rs`에서 사용자 정의 env 출력:

```rust
fn main() {
    println!("cargo::rustc-env=GIT_SHA={}", git_sha());
    println!("cargo::rustc-env=BUILD_TIMESTAMP={}", timestamp());
}
```

<a id="cfg_attr-conditional-attributes"></a>

### `cfg_attr`: 조건부 어트리뷰트

조건이 참일 때만 어트리뷰트를 붙입니다. 항목 전체를 넣거나 빼는 `#[cfg()]`보다 세밀합니다:

```rust
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct DiagResult {
    pub fc: u32,
    pub passed: bool,
    pub message: String,
}

#[cfg_attr(test, derive(PartialEq))]
pub struct LargeStruct { /* ... */ }

#[cfg_attr(target_os = "linux", link_name = "ioctl")]
#[cfg_attr(target_os = "freebsd", link_name = "__ioctl")]
extern "C" fn platform_ioctl(fd: i32, request: u64) -> i32;
```

| 패턴 | 동작 |
|---------|-------------|
| `#[cfg(feature = "x")]` | 항목 전체 포함/제외 |
| `#[cfg_attr(feature = "x", derive(Foo))]` | 기능 "x"일 때만 `derive(Foo)` |
| `#[cfg_attr(test, allow(unused))]` | 테스트 빌드에서만 경고 억제 |
| `#[cfg_attr(doc, doc = "...")]` | `cargo doc`에서만 보이는 문서 |

<a id="cargo-deny-and-cargo-audit"></a>

### `cargo deny`와 `cargo audit`: 공급망 보안

```bash
```

cargo install cargo-deny
cargo install cargo-audit

cargo audit

cargo deny check

```rust

워크스페이스 루트에 `deny.toml`로 `cargo deny` 설정:

```

```toml
```

# deny.toml
[advisories]
vulnerability = "deny"
unmaintained = "warn"

[licenses]
allow = ["MIT", "Apache-2.0", "BSD-2-Clause", "BSD-3-Clause"]
deny = ["GPL-3.0"]

[bans]
multiple-versions = "warn"
deny = [

```rust
    { name = "openssl" },
]

[sources]
allow-git = []
```

| 도구 | 목적 | 실행 시점 |
|------|---------|-------------|
| `cargo audit` | 의존성의 알려진 CVE | CI, 릴리스 전 |
| `cargo deny check` | 라이선스, 금지, 권고, 소스 | CI |
| `cargo deny check licenses` | 라이선스만 | 오픈소스 전 |
| `cargo deny check bans` | 특정 크레이트 차단 | 아키텍처 정책 |

<a id="doc-tests-tests-inside-documentation"></a>

### 문서 테스트: 문서 안의 테스트

`///` 안의 코드 블록은 **컴파일되어 테스트로 실행**됩니다:

```rust
/// 문자열에서 진단 fault 코드를 파싱합니다.
///
/// # 예제
///
/// ```
/// use my_crate::parse_fc;
///
/// let fc = parse_fc("FC:12345").unwrap();
/// assert_eq!(fc, 12345);
/// ```
///
/// 잘못된 입력은 에러:
///
/// ```
/// use my_crate::parse_fc;
///
/// assert!(parse_fc("not-a-fc").is_err());
/// ```
pub fn parse_fc(input: &str) -> Result<u32, ParseError> {
    input.strip_prefix("FC:")
        .ok_or(ParseError::MissingPrefix)?
        .parse()
        .map_err(ParseError::InvalidNumber)
}
```

```bash
cargo test --doc
cargo test
```

**모듈 수준 문서**는 파일 맨 위 `//!`:

```rust
//! # Diagnostic Framework
//!
//! 이 크레이트는 진단 실행 엔진을 제공합니다.
//!
//! ## Quick Start
//!
//! ```no_run
//! use diag_framework::Framework;
//!
//! let mut fw = Framework::new("config.json")?;
//! fw.run_all_tests()?;
//! ```
```

<a id="benchmarking-with-criterion-architecture"></a>

### Criterion으로 벤치마킹

> **전체 설명**: [criterion으로 벤치마킹](ch14-testing-and-benchmarking-patterns.md#benchmarking-with-criterion)(14장 테스트와 벤치마킹 패턴)에
> `criterion` 설정, API 예, `cargo bench`와의 비교 표가 있습니다.
> 여기서는 아키텍처 관점 요약만 다룹니다.

공개 API를 벤치마크할 때는 `benches/`에 두고 핫 패스(파서, 직렬화, 검증 경계)에 집중합니다:

```bash
cargo bench
cargo bench -- parse_config
# 결과: target/criterion/ HTML 리포트
```

> **핵심 정리 — 아키텍처와 API 설계**
> - 받을 때는 가장 일반적인 타입(`impl Into`, `impl AsRef`, `Cow`), 돌려줄 때는 가장 구체적으로
> - 파싱하고 검증하지 말 것: `TryFrom`으로 “구성 시점에 유효한” 타입 만들기
> - 공개 enum에는 `#[non_exhaustive]`로 변형 추가 시 호환 유지
> - `#[must_use]`로 중요한 값의 묵시적 폐기 방지

> **참고:** 공개 API의 에러 타입은 [9장 — 에러 처리](ch09-error-handling-patterns.md). 크레이트 공개 API 테스트는 [14장 — 테스트](ch14-testing-and-benchmarking-patterns.md).

---

<a id="exercise-crate-api-refactoring"></a>

### 연습: 크레이트 API 리팩터링 ★★ (~30분)

아래 “문자열 중심” API를 `TryFrom`, 뉴타입, 빌더 패턴을 쓰도록 바꾸세요:

```rust,ignore
// 전: 오용하기 쉬움
fn create_server(host: &str, port: &str, max_conn: &str) -> Server { ... }
```

`Host`, `Port`(1–65535), `MaxConnections`(1–10000)로 검증된 `ServerConfig`를 설계하고, 잘못된 값은 파싱 시점에 거절하게 하세요.

<details>
<summary>🔑 해답</summary>

```rust
#[derive(Debug, Clone)]
struct Host(String);

impl TryFrom<&str> for Host {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, String> {
        if s.is_empty() { return Err("host cannot be empty".into()); }
        if s.contains(' ') { return Err("host cannot contain spaces".into()); }
        Ok(Host(s.to_string()))
    }
}

#[derive(Debug, Clone, Copy)]
struct Port(u16);

impl TryFrom<u16> for Port {
    type Error = String;
    fn try_from(p: u16) -> Result<Self, String> {
        if p == 0 { return Err("port must be >= 1".into()); }
        Ok(Port(p))
    }
}

#[derive(Debug, Clone, Copy)]
struct MaxConnections(u32);

impl TryFrom<u32> for MaxConnections {
    type Error = String;
    fn try_from(n: u32) -> Result<Self, String> {
        if n == 0 || n > 10_000 {
            return Err(format!("max_connections must be 1–10000, got {n}"));
        }
        Ok(MaxConnections(n))
    }
}

#[derive(Debug)]
struct ServerConfig {
    host: Host,
    port: Port,
    max_connections: MaxConnections,
}

impl ServerConfig {
    fn new(host: Host, port: Port, max_connections: MaxConnections) -> Self {
        ServerConfig { host, port, max_connections }
    }
}

fn main() {
    let config = ServerConfig::new(
        Host::try_from("localhost").unwrap(),
        Port::try_from(8080).unwrap(),
        MaxConnections::try_from(100).unwrap(),
    );
    println!("{config:?}");

    assert!(Host::try_from("").is_err());
    assert!(Port::try_from(0).is_err());
    assert!(MaxConnections::try_from(99999).is_err());
}
```

</details>

***

