<a id="rust-crates-and-modules"></a>
# Rust 크레이트와 모듈

> **이 장에서 배우는 것:** Rust가 코드를 어떻게 모듈과 크레이트로 구성하는지 배웁니다. 기본 비공개(privacy by default), `pub` 가시성, 워크스페이스, `crates.io` 생태계를 다룹니다. C/C++의 헤더 파일, `#include`, CMake 기반 의존성 관리를 대체하는 개념입니다.

- 모듈은 크레이트 내부에서 코드를 조직하는 기본 단위입니다.
    - 각 소스 파일(`.rs`)은 하나의 모듈이며, `mod` 키워드로 중첩 모듈을 만들 수 있습니다.
    - (하위) 모듈 안의 모든 타입은 기본적으로 **비공개** 입니다. 외부에서 쓰려면 명시적으로 `pub`로 선언해야 합니다. 필요하면 `pub(crate)`처럼 범위를 더 좁힐 수도 있습니다.
    - 어떤 타입이 공개되어 있어도, 다른 모듈에서 자동으로 보이는 것은 아닙니다. 사용하려면 `use`로 가져와야 합니다. 하위 모듈은 `use super::`로 부모 범위를 참조할 수 있습니다.
    - 소스 파일(`.rs`)은 `main.rs`(실행 파일)나 `lib.rs`에 명시적으로 연결하지 않으면 자동으로 크레이트에 포함되지 않습니다.

<a id="exercise-modules-and-functions"></a>
# 연습문제: 모듈과 함수
- [hello world](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=522d86dbb8c4af71ff2ec081fb76aee7) 예제를 조금 수정해서 다른 함수를 호출해 보겠습니다.
    - 앞에서 설명했듯, 함수는 `fn` 키워드로 정의합니다. `->`는 함수가 값을 반환함을 나타내며, 여기서는 타입이 `u32`입니다.
    - 함수는 모듈 스코프를 따릅니다. 즉, 서로 다른 모듈에 같은 이름의 함수가 있어도 충돌하지 않습니다.
        - 이 모듈 스코프 규칙은 모든 타입에도 적용됩니다. 예를 들어 `mod a { struct foo; }`의 `a::foo`와 `mod b { struct foo; }`의 `b::foo`는 서로 다른 타입입니다.

**시작 코드** - 함수를 완성하세요.
```rust
mod math {
    // TODO: pub fn add(a: u32, b: u32) -> u32 구현
}

fn greet(name: &str) -> String {
    // TODO: "Hello, <name>! The secret number is <math::add(21,21)>" 반환
    todo!()
}

fn main() {
    println!("{}", greet("Rustacean"));
}
```

<details><summary>해답 (클릭하여 펼치기)</summary>

```rust
mod math {
    pub fn add(a: u32, b: u32) -> u32 {
        a + b
    }
}

fn greet(name: &str) -> String {
    format!("Hello, {}! The secret number is {}", name, math::add(21, 21))
}

fn main() {
    println!("{}", greet("Rustacean"));
}
// Output: Hello, Rustacean! The secret number is 42
```

</details>

<a id="workspaces-and-crates-packages"></a>
## 워크스페이스와 크레이트(패키지)

- 규모가 있는 Rust 프로젝트라면 보통 워크스페이스를 사용해 여러 크레이트를 조직합니다.
    - 워크스페이스는 타깃 바이너리를 빌드하기 위해 함께 쓰이는 로컬 크레이트들의 집합입니다. 워크스페이스 루트의 `Cargo.toml`에는 포함될 패키지(크레이트) 목록이 들어갑니다.

```toml
[workspace]
resolver = "2"
members = ["package1", "package2"]
```

```text
workspace_root/
|-- Cargo.toml      # 워크스페이스 설정
|-- package1/
|   |-- Cargo.toml  # 패키지 1 설정
|   `-- src/
|       `-- lib.rs  # 패키지 1 소스
|-- package2/
|   |-- Cargo.toml  # 패키지 2 설정
|   `-- src/
|       `-- main.rs # 패키지 2 소스
```

---
<a id="exercise-using-workspaces-and-package-dependencies"></a>
## 연습문제: 워크스페이스와 패키지 의존성 사용하기
- 간단한 패키지를 만들고, 이를 `hello world` 프로그램에서 사용해 보겠습니다.
- 워크스페이스 디렉터리 생성
```bash
mkdir workspace
cd workspace
```
- `Cargo.toml` 파일을 만들고 다음 내용을 넣습니다. 비어 있는 워크스페이스가 생성됩니다.
```toml
[workspace]
resolver = "2"
members = []
```
- 패키지 추가 (`cargo new --lib`는 실행 파일이 아니라 라이브러리를 만듭니다)
```bash
cargo new hello
cargo new --lib hellolib
```

## 연습문제: 워크스페이스와 패키지 의존성 사용하기
- 생성된 `hello`와 `hellolib`의 `Cargo.toml`을 살펴보세요. 둘 다 상위 `Cargo.toml`에 연결됩니다.
- `hellolib`에 `lib.rs`가 있다는 것은 라이브러리 패키지라는 뜻입니다. 참고: https://doc.rust-lang.org/cargo/reference/cargo-targets.html
- `hello`의 `Cargo.toml`에 `hellolib` 의존성 추가
```toml
[dependencies]
hellolib = { path = "../hellolib" }
```
- `hellolib`의 `add()` 사용
```rust
fn main() {
    println!("Hello, world! {}", hellolib::add(21, 21));
}
```

<details><summary>해답 (클릭하여 펼치기)</summary>

전체 워크스페이스 설정:

```bash
# 터미널 명령
mkdir workspace && cd workspace

# workspace Cargo.toml 생성
cat > Cargo.toml << 'EOF'
[workspace]
resolver = "2"
members = ["hello", "hellolib"]
EOF

cargo new hello
cargo new --lib hellolib
```

```toml
# hello/Cargo.toml — 의존성 추가
[dependencies]
hellolib = { path = "../hellolib" }
```

```rust
// hellolib/src/lib.rs — cargo new --lib가 만든 add()
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
```

```rust,ignore
// hello/src/main.rs
fn main() {
    println!("Hello, world! {}", hellolib::add(21, 21));
}
// Output: Hello, world! 42
```

</details>

<a id="using-community-crates-from-cratesio"></a>
# crates.io의 커뮤니티 크레이트 사용하기
- Rust에는 활발한 커뮤니티 크레이트 생태계가 있습니다. https://crates.io/ 를 참고하세요.
    - Rust 철학은 표준 라이브러리를 비교적 작게 유지하고, 많은 기능을 커뮤니티 크레이트로 분리하는 것입니다.
    - 커뮤니티 크레이트를 사용할지 말지에 대한 절대 규칙은 없지만, 일반적으로는 버전 번호로 드러나는 성숙도와 유지보수 상태를 확인하는 것이 좋습니다. 확신이 없으면 내부 검토 절차를 거치세요.
- `crates.io`에 게시된 모든 크레이트는 메이저/마이너 버전을 가집니다.
    - 크레이트는 https://doc.rust-lang.org/cargo/reference/semver.html 에 정의된 `SemVer` 가이드를 따르는 것이 기대됩니다.
    - 아주 짧게 말하면, 같은 마이너 버전 범위에서는 깨지는 변경이 없어야 합니다.

<a id="crates-dependencies-and-semver"></a>
# 크레이트 의존성과 SemVer
- 크레이트는 특정 정확한 버전, 특정 범위의 마이너/메이저 버전, 혹은 "상관없음" 식으로 의존성을 선언할 수 있습니다. 아래는 `rand` 크레이트를 선언하는 예시입니다.
- 최소 `0.10.0` 이상이고, `0.11.0` 미만이면 모두 허용
```toml
[dependencies]
rand = { version = "0.10.0" }
```
- 오직 `0.10.0`만 허용
```toml
[dependencies]
rand = { version = "=0.10.0" }
```
- 신경 쓰지 않음. `cargo`가 최신 버전을 선택
```toml
[dependencies]
rand = { version = "*" }
```
- 참고: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
----
<a id="exercise-using-the-rand-crate"></a>
# 연습문제: rand 크레이트 사용하기
- `helloworld` 예제를 수정해 랜덤 숫자를 출력해 보세요.
- `cargo add rand`로 의존성을 추가하세요.
- API 참고: `https://docs.rs/rand/latest/rand/`

**시작 코드** - `cargo add rand` 실행 후 `main.rs`에 아래를 추가하세요.
```rust,ignore
use rand::RngExt;

fn main() {
    let mut rng = rand::rng();
    // TODO: 1..=100 범위의 랜덤 u32 생성 후 출력
    // TODO: 랜덤 bool 생성 후 출력
    // TODO: 랜덤 f64 생성 후 출력
}
```

<details><summary>해답 (클릭하여 펼치기)</summary>

```rust
use rand::RngExt;

fn main() {
    let mut rng = rand::rng();
    let n: u32 = rng.random_range(1..=100);
    println!("Random number (1-100): {n}");

    // 랜덤 불리언
    let b: bool = rng.random();
    println!("Random bool: {b}");

    // 0.0 ~ 1.0 사이 랜덤 실수
    let f: f64 = rng.random();
    println!("Random float: {f:.4}");
}
```

</details>

<a id="cargotoml-and-cargolock"></a>
# Cargo.toml과 Cargo.lock
- 앞서 언급했듯, Cargo.lock은 Cargo.toml을 바탕으로 자동 생성됩니다.
    - Cargo.lock의 핵심 목적은 재현 가능한 빌드를 보장하는 것입니다. 예를 들어 `Cargo.toml`에 `0.10.0`을 적었다면, cargo는 실제로는 `0.11.0` 미만의 다른 패치 버전을 선택할 수 있습니다.
    - Cargo.lock에는 그 빌드에서 실제로 선택된 `rand`의 **구체적인 버전** 이 기록됩니다.
    - 재현 가능한 빌드를 위해 `Cargo.lock`을 git 저장소에 포함하는 것을 권장합니다.

<a id="cargo-test-feature"></a>
## Cargo test 기능
- Rust의 단위 테스트는 보통 같은 소스 파일 안에 존재하며, 관례상 별도 모듈에 묶습니다.
    - 테스트 코드는 실제 바이너리에는 포함되지 않습니다. 이것은 `cfg`(configuration) 기능으로 가능합니다. 이 기능은 플랫폼별 코드(`Linux` vs `Windows`)를 만들 때도 유용합니다.
    - 테스트는 `cargo test`로 실행할 수 있습니다. 참고: https://doc.rust-lang.org/reference/conditional-compilation.html

```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
// 테스트 시에만 포함
#[cfg(test)]
mod tests {
    use super::*; // 부모 스코프의 타입을 모두 볼 수 있게 함
    #[test]
    fn it_works() {
        let result = add(2, 2); // 또는 super::add(2, 2);
        assert_eq!(result, 4);
    }
}
```

<a id="other-cargo-features"></a>
# 기타 Cargo 기능
- `cargo`에는 다른 유용한 기능도 많습니다.
    - `cargo clippy`는 Rust 코드를 린트하는 훌륭한 도구입니다. 일반적으로 경고는 고치고, 정말 필요한 경우에만 드물게 억제하는 것이 좋습니다.
    - `cargo format`은 `rustfmt`를 실행해 코드를 포맷합니다. 이 도구를 사용하면 저장소에 들어가는 코드 형식이 통일되고, 스타일 논쟁도 사라집니다.
    - `cargo doc`은 `///` 스타일 주석에서 문서를 생성합니다. `crates.io`의 문서도 같은 방식으로 만들어집니다.

### 빌드 프로필: 최적화 제어

C에서는 `gcc`/`clang`에 `-O0`, `-O2`, `-Os`, `-flto` 같은 플래그를 넘깁니다. Rust에서는 `Cargo.toml`에서 빌드 프로필을 설정합니다.

```toml
# Cargo.toml — 빌드 프로필 설정

[profile.dev]
opt-level = 0          # 최적화 없음 (빠른 컴파일, -O0와 유사)
debug = true           # 전체 디버그 심볼 ( -g 와 유사)

[profile.release]
opt-level = 3          # 최대 최적화 ( -O3 와 유사)
lto = "fat"            # 링크 타임 최적화 ( -flto 와 유사)
strip = true           # 심볼 제거 (strip 명령과 유사)
codegen-units = 1      # 단일 codegen unit — 느리지만 최적화에 유리
panic = "abort"        # unwind table 제거 (바이너리 축소)
```

| C/GCC 플래그 | Cargo.toml 키 | 값 |
|------------|---------------|--------|
| `-O0` / `-O2` / `-O3` | `opt-level` | `0`, `1`, `2`, `3`, `"s"`, `"z"` |
| `-flto` | `lto` | `false`, `"thin"`, `"fat"` |
| `-g` / no `-g` | `debug` | `true`, `false`, `"line-tables-only"` |
| `strip` 명령 | `strip` | `"none"`, `"debuginfo"`, `"symbols"`, `true`/`false` |
| — | `codegen-units` | `1` = 최고의 최적화, 가장 느린 컴파일 |

```bash
cargo build              # [profile.dev] 사용
cargo build --release    # [profile.release] 사용
```

### 빌드 스크립트 (`build.rs`): C 라이브러리 링크

C에서는 Makefile이나 CMake로 라이브러리를 링크하고 코드 생성을 수행합니다. Rust에서는 크레이트 루트의 `build.rs` 파일로 처리합니다.

```rust
// build.rs — 크레이트 컴파일 전에 실행

fn main() {
    // 시스템 C 라이브러리 링크 (gcc의 -lbmc_ipmi와 비슷)
    println!("cargo::rustc-link-lib=bmc_ipmi");

    // 라이브러리 탐색 경로 지정 ( -L/usr/lib/bmc 와 비슷)
    println!("cargo::rustc-link-search=/usr/lib/bmc");

    // C 헤더가 바뀌면 다시 실행
    println!("cargo::rerun-if-changed=wrapper.h");
}
```

Rust 크레이트에서 C 소스를 직접 컴파일할 수도 있습니다.

```toml
# Cargo.toml
[build-dependencies]
cc = "1"  # C 컴파일러 연동
```

```rust
// build.rs
fn main() {
    cc::Build::new()
        .file("src/c_helpers/ipmi_raw.c")
        .include("/usr/include/bmc")
        .compile("ipmi_raw");   // libipmi_raw.a 생성 후 자동 링크
    println!("cargo::rerun-if-changed=src/c_helpers/ipmi_raw.c");
}
```

| C / Make / CMake | Rust `build.rs` |
|-----------------|-----------------|
| `-lfoo` | `println!("cargo::rustc-link-lib=foo")` |
| `-L/path` | `println!("cargo::rustc-link-search=/path")` |
| C 소스 컴파일 | `cc::Build::new().file("foo.c").compile("foo")` |
| 코드 생성 | `$OUT_DIR`에 파일 생성 후 `include!()` |

### 크로스 컴파일

C에서는 별도 툴체인(`arm-linux-gnueabihf-gcc`)을 설치하고 Make/CMake를 조정해야 합니다. Rust에서는:

```bash
# 크로스 컴파일 타깃 설치
rustup target add aarch64-unknown-linux-gnu

# 크로스 컴파일
cargo build --target aarch64-unknown-linux-gnu --release
```

`.cargo/config.toml`에 링커 지정:

```toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

| C 크로스 컴파일 | Rust 대응 |
|-----------------|-----------------|
| `apt install gcc-aarch64-linux-gnu` | `rustup target add aarch64-unknown-linux-gnu` + 링커 설치 |
| `CC=aarch64-linux-gnu-gcc make` | `.cargo/config.toml`의 `[target.X] linker = "..."` |
| `#ifdef __aarch64__` | `#[cfg(target_arch = "aarch64")]` |
| 별도 Makefile 타깃 | `cargo build --target ...` |

### Feature Flags: 조건부 컴파일

C는 `#ifdef`와 `-DFOO`를 씁니다. Rust는 `Cargo.toml`에 정의한 feature flag를 사용합니다.

```toml
# Cargo.toml
[features]
default = ["json"]         # 기본 활성화
json = ["dep:serde_json"]  # 선택적 의존성
verbose = []               # 의존성 없는 플래그
gpu = ["dep:cuda-sys"]     # 선택적 GPU 지원
```

```rust
// feature에 따라 포함되는 코드:
#[cfg(feature = "json")]
pub fn parse_config(data: &str) -> Result<Config, Error> {
    serde_json::from_str(data).map_err(Error::from)
}

#[cfg(feature = "verbose")]
macro_rules! verbose {
    ($($arg:tt)*) => { eprintln!("[VERBOSE] {}", format!($($arg)*)); }
}
#[cfg(not(feature = "verbose"))]
macro_rules! verbose {
    ($($arg:tt)*) => {};
}
```

| C 전처리기 | Rust feature flag |
|---------------|-------------------|
| `gcc -DDEBUG` | `cargo build --features verbose` |
| `#ifdef DEBUG` | `#[cfg(feature = "verbose")]` |
| `#define MAX 100` | `const MAX: u32 = 100;` |
| `#ifdef __linux__` | `#[cfg(target_os = "linux")]` |

### 통합 테스트 vs 단위 테스트

단위 테스트는 코드 옆에 `#[cfg(test)]`와 함께 둡니다. **통합 테스트**는 `tests/` 디렉터리에 두며, 크레이트의 **공개 API만** 테스트합니다.

```rust
// tests/smoke_test.rs — #[cfg(test)] 필요 없음
use my_crate::parse_config;

#[test]
fn parse_valid_config() {
    let config = parse_config("test_data/valid.json").unwrap();
    assert_eq!(config.max_retries, 5);
}
```

| 항목 | 단위 테스트 (`#[cfg(test)]`) | 통합 테스트 (`tests/`) |
|--------|----------------------------|------------------------------|
| 위치 | 코드와 같은 파일 | 별도의 `tests/` 디렉터리 |
| 접근 가능 범위 | private + public | **공개 API만** |
| 실행 명령 | `cargo test` | `cargo test --test smoke_test` |


### 테스트 패턴과 전략

C 펌웨어 팀은 보통 CUnit, CMocka, 또는 커스텀 프레임워크로 많은 보일러플레이트를 작성합니다. Rust의 내장 테스트 하네스는 훨씬 강력합니다. 여기서는 실무에서 자주 쓰는 패턴을 다룹니다.

#### `#[should_panic]` - 예상 실패 테스트

```rust
// 특정 조건에서 패닉이 발생해야 하는지 테스트
#[test]
#[should_panic(expected = "index out of bounds")]
fn test_bounds_check() {
    let v = vec![1, 2, 3];
    let _ = v[10];
}

#[test]
#[should_panic(expected = "temperature exceeds safe limit")]
fn test_thermal_shutdown() {
    fn check_temperature(celsius: f64) {
        if celsius > 105.0 {
            panic!("temperature exceeds safe limit: {celsius}°C");
        }
    }
    check_temperature(110.0);
}
```

#### `#[ignore]` - 느리거나 하드웨어 의존적인 테스트

```rust
// 특별한 조건이 필요한 테스트 표시
#[test]
#[ignore = "requires GPU hardware"]
fn test_gpu_ecc_scrub() {
    // GPU가 있는 환경에서만 실행
    // 실행: cargo test -- --ignored
    // 전체 포함: cargo test -- --include-ignored
}
```

#### `Result`를 반환하는 테스트 (`unwrap` 연쇄 대신)

```rust
// unwrap()을 여러 번 쓰는 대신:
#[test]
fn test_config_parsing() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{"hostname": "node-01", "port": 8080}"#;
    let config: ServerConfig = serde_json::from_str(json)?;
    assert_eq!(config.hostname, "node-01");
    assert_eq!(config.port, 8080);
    Ok(())
}
```

#### 빌더 함수 기반 테스트 픽스처

C는 `setUp()`/`tearDown()`을 많이 씁니다. Rust는 헬퍼 함수와 `Drop`으로 해결합니다.

```rust
struct TestFixture {
    temp_dir: std::path::PathBuf,
    config: Config,
}

impl TestFixture {
    fn new() -> Self {
        let temp_dir = std::env::temp_dir().join(format!("test_{}", std::process::id()));
        std::fs::create_dir_all(&temp_dir).unwrap();
        let config = Config {
            log_dir: temp_dir.clone(),
            max_retries: 3,
            ..Default::default()
        };
        Self { temp_dir, config }
    }
}

impl Drop for TestFixture {
    fn drop(&mut self) {
        // 자동 정리 - C의 tearDown()과 비슷하지만 잊을 수 없음
        let _ = std::fs::remove_dir_all(&self.temp_dir);
    }
}

#[test]
fn test_with_fixture() {
    let fixture = TestFixture::new();
    assert!(fixture.temp_dir.exists());
    // fixture는 여기서 자동 drop → cleanup 수행
}
```

#### 하드웨어 인터페이스 모킹을 위한 trait 사용

C에서 하드웨어 모킹은 전처리기 트릭이나 함수 포인터 교체로 해결하는 경우가 많습니다. Rust에서는 trait 덕분에 자연스럽습니다.

```rust
// 프로덕션용 IPMI 통신 trait
trait IpmiTransport {
    fn send_command(&self, cmd: u8, data: &[u8]) -> Result<Vec<u8>, String>;
}

// 실제 구현
struct RealIpmi { /* BMC connection details */ }
impl IpmiTransport for RealIpmi {
    fn send_command(&self, cmd: u8, data: &[u8]) -> Result<Vec<u8>, String> {
        todo!("Real IPMI call")
    }
}

// 테스트용 mock 구현
struct MockIpmi {
    responses: std::collections::HashMap<u8, Vec<u8>>,
}
impl IpmiTransport for MockIpmi {
    fn send_command(&self, cmd: u8, _data: &[u8]) -> Result<Vec<u8>, String> {
        self.responses.get(&cmd)
            .cloned()
            .ok_or_else(|| format!("No mock response for cmd 0x{cmd:02x}"))
    }
}

// 실제 구현과 mock 둘 다에서 동작하는 제네릭 함수
fn read_sensor_temperature(transport: &dyn IpmiTransport) -> Result<f64, String> {
    let response = transport.send_command(0x2D, &[])?;
    if response.len() < 2 {
        return Err("Response too short".into());
    }
    Ok(response[0] as f64 + (response[1] as f64 / 256.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperature_reading() {
        let mut mock = MockIpmi { responses: std::collections::HashMap::new() };
        mock.responses.insert(0x2D, vec![72, 128]); // 72.5°C

        let temp = read_sensor_temperature(&mock).unwrap();
        assert!((temp - 72.5).abs() < 0.01);
    }

    #[test]
    fn test_short_response() {
        let mock = MockIpmi { responses: std::collections::HashMap::new() };
        assert!(read_sensor_temperature(&mock).is_err());
    }
}
```

#### `proptest`를 이용한 property-based testing

특정 값만 테스트하는 대신, 항상 성립해야 하는 **성질(property)** 을 테스트할 수 있습니다.

```rust
// Cargo.toml: [dev-dependencies] proptest = "1"
use proptest::prelude::*;

fn parse_sensor_id(s: &str) -> Option<u32> {
    s.strip_prefix("sensor_")?.parse().ok()
}

fn format_sensor_id(id: u32) -> String {
    format!("sensor_{id}")
}

proptest! {
    #[test]
    fn roundtrip_sensor_id(id in 0u32..10000) {
        // 성질: format 후 parse하면 원래 값이 나와야 함
        let formatted = format_sensor_id(id);
        let parsed = parse_sensor_id(&formatted);
        prop_assert_eq!(parsed, Some(id));
    }

    #[test]
    fn parse_rejects_garbage(s in "[^s].*") {
        // 성질: s로 시작하지 않는 문자열은 parse되면 안 됨
        let result = parse_sensor_id(&s);
        prop_assert!(result.is_none());
    }
}
```

#### C vs Rust 테스트 비교

| C 테스트 | Rust 대응 |
|-----------|----------------|
| `CUnit`, `CMocka`, 커스텀 프레임워크 | 내장 `#[test]` + `cargo test` |
| `setUp()` / `tearDown()` | 빌더 함수 + `Drop` 트레잇 |
| `#ifdef TEST` 기반 mock 함수 | trait 기반 의존성 주입 |
| `assert(x == y)` | `assert_eq!(x, y)` + 자동 diff 출력 |
| 별도 테스트 실행 파일 | 같은 바이너리, `#[cfg(test)]`로 조건부 포함 |
| `valgrind --leak-check=full ./test` | `cargo test` + 필요시 `cargo miri test` |
| 코드 커버리지: `gcov` / `lcov` | `cargo tarpaulin` 또는 `cargo llvm-cov` |
| 테스트 등록 수동 관리 | `#[test]` 함수 자동 탐지 |
