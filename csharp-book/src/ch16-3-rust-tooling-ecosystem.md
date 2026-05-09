<a id="essential-rust-tooling-for-c-developers"></a>
## C# 개발자를 위한 필수 Rust 툴링

> **이 장에서 배울 내용:** Rust 개발 도구를 C#의 대응 도구에 연결해 봅니다. Clippy(Roslyn analyzer),
> rustfmt(`dotnet format`), cargo doc(XML 문서), cargo watch(`dotnet watch`), VS Code 확장까지 다룹니다.
>
> **난이도:** 🟢 기초

### 도구 비교

| C# 도구 | Rust 대응 도구 | 설치 | 용도 |
|---------|----------------|------|------|
| Roslyn analyzer | **Clippy** | `rustup component add clippy` | 린트 + 스타일 제안 |
| `dotnet format` | **rustfmt** | `rustup component add rustfmt` | 자동 포매팅 |
| XML 문서 주석 | **`cargo doc`** | 기본 제공 | HTML 문서 생성 |
| OmniSharp / Roslyn | **rust-analyzer** | VS Code 확장 | IDE 지원 |
| `dotnet watch` | **cargo-watch** | `cargo install cargo-watch` | 저장 시 자동 재빌드 |
| — | **cargo-expand** | `cargo install cargo-expand` | 매크로 확장 결과 보기 |
| `dotnet audit` | **cargo-audit** | `cargo install cargo-audit` | 보안 취약점 스캔 |

### Clippy: 자동 코드 리뷰어
```bash
# 프로젝트에 Clippy 실행
cargo clippy

# 경고를 에러로 취급 (CI/CD)
cargo clippy -- -D warnings

# 제안 사항 자동 반영
cargo clippy --fix
```

```rust
// Clippy는 수백 가지 안티패턴을 잡아냅니다:

// Clippy 전:
if x == true { }           // warning: bool과 equality 비교
let _ = vec.len() == 0;    // warning: 대신 .is_empty() 사용
for i in 0..vec.len() { }  // warning: .iter().enumerate() 사용

// Clippy 제안 반영 후:
if x { }
let _ = vec.is_empty();
for (i, item) in vec.iter().enumerate() { }
```

### rustfmt: 일관된 포매팅
```bash
# 모든 파일 포맷팅
cargo fmt

# 변경 없이 포맷 상태만 검사 (CI/CD)
cargo fmt -- --check
```

```toml
# rustfmt.toml — 포매팅 규칙 커스터마이즈 (.editorconfig와 유사)
max_width = 100
tab_spaces = 4
use_field_init_shorthand = true
```

### cargo doc: 문서 생성
```bash
# 문서 생성 후 열기 (의존성 포함)
cargo doc --open

# 문서 테스트 실행
cargo test --doc
```

```rust
/// Calculate the area of a circle.
///
/// # Arguments
/// * `radius` - The radius of the circle (must be non-negative)
///
/// # Examples
/// ```
/// let area = my_crate::circle_area(5.0);
/// assert!((area - 78.54).abs() < 0.01);
/// ```
///
/// # Panics
/// Panics if `radius` is negative.
pub fn circle_area(radius: f64) -> f64 {
    assert!(radius >= 0.0, "radius must be non-negative");
    std::f64::consts::PI * radius * radius
}
// /// ``` 블록 안의 코드는 `cargo test` 때 컴파일되고 실행됩니다!
```

### cargo watch: 자동 재빌드
```bash
# 파일 변경 시 다시 실행 (`dotnet watch`와 유사)
cargo watch -x check          # 타입 체크만 (가장 빠름)
cargo watch -x test           # 저장 시 테스트 실행
cargo watch -x 'run -- args'  # 저장 시 프로그램 실행
cargo watch -x clippy         # 저장 시 린트 실행
```

### cargo expand: 매크로가 펼쳐진 결과 보기
```bash
# derive 매크로가 확장된 결과 보기
cargo expand --lib            # lib.rs 확장
cargo expand module_name      # 특정 모듈 확장
```

### 추천 VS Code 확장

| 확장 | 용도 |
|------|------|
| **rust-analyzer** | 코드 완성, 인라인 에러, 리팩터링 |
| **CodeLLDB** | 디버거 (Visual Studio 디버거와 유사) |
| **Even Better TOML** | `Cargo.toml` 문법 강조 |
| **crates** | `Cargo.toml`에서 최신 크레이트 버전 표시 |
| **Error Lens** | 인라인 에러/경고 표시 |

***

이 가이드에서 언급한 심화 주제를 더 깊게 보려면, 함께 제공되는 아래 문서를 참고하세요.

- **[Rust Patterns](../../source-docs/RUST_PATTERNS.md)** — Pin projection, 커스텀 allocator, arena 패턴, lock-free 자료구조, 고급 unsafe 패턴
- **[Async Rust Training](../../source-docs/ASYNC_RUST_TRAINING.md)** — tokio, async cancellation safety, stream 처리, 프로덕션 async 아키텍처 심화
- **[Rust Training for C++ Developers](./RUST_TRAINING_FOR_CPP.md)** — 팀에 C++ 경험자도 있다면 유용합니다. move semantics 대응, RAII 차이, template vs generics를 다룹니다
- **[Rust Training for C Developers](./RUST_TRAINING_FOR_C.md)** — 상호운용 시나리오에 유용합니다. FFI 패턴, 임베디드 Rust 디버깅, `no_std` 프로그래밍을 다룹니다