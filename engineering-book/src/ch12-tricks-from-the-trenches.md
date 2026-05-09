<a id="tricks-from-the-trenches"></a>
# 실전에서 건진 요령 🟡

> **이 장에서 배우는 것:**
> - 한 장에 깔끔하게 묶기 어려운, 실전에서 검증된 패턴
> - CI flake부터 바이너리 비대화까지 이어지는 흔한 함정과 그 해결책
> - 오늘 바로 어떤 Rust 프로젝트에도 적용할 수 있는 quick win 기법
>
> **상호 참조:** 이 책의 모든 장 — 이 요령들은 전 주제에 걸쳐 적용됩니다

이 장은 프로덕션 Rust 코드베이스에서 반복해서 마주치는 엔지니어링 패턴을 모았습니다. 각 요령은 독립적이므로 원하는 순서로 읽어도 됩니다.

---

<a id="1-the-denywarnings-trap"></a>
### 1. `deny(warnings)` 함정

**문제**: 소스 코드에 `#![deny(warnings)]`를 넣어 두면, Clippy가 새 lint를 추가하는 순간 빌드가 깨집니다. 어제까지 컴파일되던 코드가 오늘 갑자기 실패할 수 있습니다.

**해결**: 소스 수준 attribute 대신 CI에서 `CARGO_ENCODED_RUSTFLAGS`를 사용하세요:

```yaml
# CI: 소스를 건드리지 않고 경고를 오류로 취급
env:
  CARGO_ENCODED_RUSTFLAGS: "-Dwarnings"
```

더 세밀한 제어가 필요하면 `[workspace.lints]`를 사용하세요:

```toml
# Cargo.toml
[workspace.lints.rust]
unsafe_code = "deny"

[workspace.lints.clippy]
all = { level = "deny", priority = -1 }
pedantic = { level = "warn", priority = -1 }
```

> 전체 패턴은 [컴파일 시간 도구, 워크스페이스 lint](ch08-compile-time-and-developer-tools.md)에서 자세히 다룹니다.

---

<a id="2-compile-once-test-everywhere"></a>
### 2. 한 번 컴파일하고, 어디서나 테스트하기

**문제**: `cargo test`는 `--lib`, `--doc`, `--test` 사이를 오갈 때마다 서로 다른 profile을 쓰기 때문에 다시 컴파일합니다.

**해결**: unit/integration test는 `cargo nextest`로 돌리고, doc-test는 별도로 실행하세요:

```bash
cargo nextest run --workspace        # 빠름: 병렬 실행, 캐시 활용
cargo test --workspace --doc         # doc-test (nextest는 이것을 실행하지 못함)
```

> `cargo-nextest` 설정은 [컴파일 시간 도구](ch08-compile-time-and-developer-tools.md)를 참고하세요.

---

<a id="3-feature-flag-hygiene"></a>
### 3. Feature Flag 위생 관리

**문제**: 라이브러리 크레이트가 `default = ["std"]`를 갖고 있는데 아무도 `--no-default-features`를 테스트하지 않습니다. 그러다 어느 날 임베디드 사용자가 컴파일조차 되지 않는다고 제보합니다.

**해결**: CI에 `cargo-hack`를 추가하세요:

```yaml
- name: Feature matrix
  run: |
    cargo hack check --each-feature --no-dev-deps
    cargo check --no-default-features
    cargo check --all-features
```

> 전체 패턴은 [`no_std`와 feature 검증](ch09-no-std-and-feature-verification.md)에서 자세히 다룹니다.

---

<a id="4-the-lock-file-debate--commit-or-ignore"></a>
### 4. 잠금 파일 논쟁 — 커밋할까, 무시할까?

**실전 규칙:**

| 크레이트 종류 | `Cargo.lock` 커밋? | 이유 |
|------------|---------------------|-----|
| 바이너리 / 애플리케이션 | **예** | 재현 가능한 빌드 |
| 라이브러리 | **아니오** (`.gitignore`) | 하위 사용자가 버전을 선택하게 둠 |
| 둘 다 있는 워크스페이스 | **예** | 바이너리 쪽 규칙이 우선 |

잠금 파일이 최신 상태인지 확인하는 CI 검사도 추가하세요:

```yaml
- name: Check lock file
  run: cargo update --locked  # Cargo.lock 이 오래되었으면 실패
```

---

<a id="5-debug-builds-with-optimized-dependencies"></a>
### 5. 최적화된 의존성을 사용하는 디버그 빌드

**문제**: 디버그 빌드가 지나치게 느립니다. 특히 `serde`, `regex` 같은 의존성이 최적화되지 않기 때문입니다.

**해결**: dev profile에서는 의존성만 최적화하고, 내 코드는 빠른 재컴파일을 위해 비최적화 상태로 두세요:

```toml
# Cargo.toml
[profile.dev.package."*"]
opt-level = 2  # dev 모드에서 모든 의존성 최적화
```

이 설정은 첫 빌드를 약간 느리게 만들 수 있지만, 개발 중 런타임은 크게 빨라집니다. 특히 데이터베이스를 붙인 서비스나 parser 계열 코드에서 효과가 큽니다.

> 크레이트별 profile override는 [릴리스 프로파일](ch07-release-profiles-and-binary-size.md)을 참고하세요.

---

<a id="6-ci-cache-thrashing"></a>
### 6. CI 캐시 흔들림

**문제**: `Swatinem/rust-cache@v2`가 모든 PR마다 새 캐시를 저장해 스토리지를 불리고 복원 시간도 느려집니다.

**해결**: 캐시는 `main`에서만 저장하고, 복원은 어디서든 하게 하세요:

```yaml
- uses: Swatinem/rust-cache@v2
  with:
    save-if: ${{ github.ref == 'refs/heads/main' }}
```

바이너리가 여러 개인 워크스페이스라면 `shared-key`를 추가하세요:

```yaml
- uses: Swatinem/rust-cache@v2
  with:
    shared-key: "ci-${{ matrix.target }}"
    save-if: ${{ github.ref == 'refs/heads/main' }}
```

> 전체 워크플로는 [CI/CD 파이프라인](ch11-putting-it-all-together-a-production-cic.md)에서 확인할 수 있습니다.

---

<a id="7-rustflags-vs-cargo-encoded-rustflags"></a>
### 7. `RUSTFLAGS` vs `CARGO_ENCODED_RUSTFLAGS`

**문제**: `RUSTFLAGS="-Dwarnings"`는 *모든 것*에 적용됩니다. build script와 proc-macro까지 포함됩니다. 결국 `serde_derive`의 build.rs 경고 하나가 CI를 깨뜨릴 수도 있습니다.

**해결**: 최상위 크레이트에만 적용되는 `CARGO_ENCODED_RUSTFLAGS`를 사용하세요:

```bash
# 나쁨 - 서드파티 build script 경고로도 깨질 수 있음
RUSTFLAGS="-Dwarnings" cargo build

# 좋음 - 내 크레이트에만 영향
CARGO_ENCODED_RUSTFLAGS="-Dwarnings" cargo build

# 이것도 좋음 - 워크스페이스 lint (Cargo.toml)
[workspace.lints.rust]
warnings = "deny"
```

---

<a id="8-reproducible-builds-with-source-date-epoch"></a>
### 8. `SOURCE_DATE_EPOCH`를 이용한 재현 가능한 빌드

**문제**: `build.rs` 안에 `chrono::Utc::now()`를 박아 두면 빌드가 재현 가능하지 않게 됩니다. 빌드할 때마다 바이너리 해시가 달라집니다.

**해결**: `SOURCE_DATE_EPOCH`를 존중하세요:

```rust
// build.rs
let timestamp = std::env::var("SOURCE_DATE_EPOCH")
    .ok()
    .and_then(|s| s.parse::<i64>().ok())
    .unwrap_or_else(|| chrono::Utc::now().timestamp());
println!("cargo:rustc-env=BUILD_TIMESTAMP={timestamp}");
```

> 전체 `build.rs` 패턴은 [빌드 스크립트](ch01-build-scripts-buildrs-in-depth.md)를 참고하세요.

---

<a id="9-the-cargo-tree-deduplication-workflow"></a>
### 9. `cargo tree` 중복 제거 워크플로

**문제**: `cargo tree --duplicates`를 돌려 보니 `syn`은 5개 버전, `tokio-util`은 3개 버전이 잡힙니다. 컴파일 시간이 고통스러워집니다.

**해결**: 체계적으로 중복을 제거하세요:

```bash
# 1단계: 중복 찾기
cargo tree --duplicates

# 2단계: 누가 오래된 버전을 끌어오는지 찾기
cargo tree --invert --package syn@1.0.109

# 3단계: 원인 크레이트 업데이트
cargo update -p serde_derive  # syn 2.x 를 끌어올 수도 있음

# 4단계: 업데이트가 없으면 [patch] 로 고정
# [patch.crates-io]
# old-crate = { git = "...", branch = "syn2-migration" }

# 5단계: 검증
cargo tree --duplicates  # 더 짧아져 있어야 함
```

> `cargo-deny`와 공급망 보안은 [의존성 관리](ch06-dependency-management-and-supply-chain-s.md)에서 다룹니다.

---

<a id="10-pre-push-smoke-test"></a>
### 10. pre-push 스모크 테스트

**문제**: 코드를 push했는데 CI가 10분이나 돌고 나서 포맷팅 문제 하나 때문에 실패합니다.

**해결**: push 전에 빠른 검사를 로컬에서 먼저 돌리세요:

```toml
# Makefile.toml (cargo-make)
[tasks.pre-push]
description = "Local smoke test before pushing"
script = '''
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --lib
'''
```

```bash
cargo make pre-push  # 30초 미만
git push
```

또는 git pre-push hook을 사용할 수도 있습니다:

```bash
#!/bin/sh
# .git/hooks/pre-push
cargo fmt --all -- --check && cargo clippy --workspace -- -D warnings
```

> `Makefile.toml` 패턴은 [CI/CD 파이프라인](ch11-putting-it-all-together-a-production-cic.md)에서 자세히 다룹니다.

---

<a id="exercises"></a>
### 🏋️ 연습문제

<a id="exercise-1-apply-three-tricks"></a>
#### 🟢 연습 1: 세 가지 요령 적용하기

이 장에서 세 가지 요령을 골라 기존 Rust 프로젝트에 적용해 보세요. 무엇이 가장 큰 효과를 냈나요?

<details>
<summary>해답</summary>

보통 효과가 큰 조합은 다음과 같습니다:

1. **`[profile.dev.package."*"] opt-level = 2`** — dev 모드 런타임이 즉시 개선됩니다 (파싱 비중이 큰 코드에서는 2~10배 빨라질 수 있음)

2. **`CARGO_ENCODED_RUSTFLAGS`** — 서드파티 경고 때문에 CI가 거짓 실패하는 일을 없애 줍니다

3. **`cargo-hack --each-feature`** — feature가 3개 이상 있는 프로젝트라면 보통 적어도 하나의 깨진 feature 조합을 찾아냅니다

```bash
# 5번 요령 적용
echo '[profile.dev.package."*"]' >> Cargo.toml
echo 'opt-level = 2' >> Cargo.toml

# CI에 7번 요령 적용
# RUSTFLAGS 를 CARGO_ENCODED_RUSTFLAGS 로 교체

# 3번 요령 적용
cargo install cargo-hack
cargo hack check --each-feature --no-dev-deps
```
</details>

<a id="exercise-2-deduplicate-your-dependency-tree"></a>
#### 🟡 연습 2: 의존성 트리 중복 제거하기

실제 프로젝트에 `cargo tree --duplicates`를 실행해 보세요. 중복을 적어도 하나 제거하고, 전후 컴파일 시간을 측정하세요.

<details>
<summary>해답</summary>

```bash
# 변경 전
time cargo build --release 2>&1 | tail -1
cargo tree --duplicates | wc -l  # 중복 줄 수 세기

# 중복 하나를 찾아 고치기
cargo tree --duplicates
cargo tree --invert --package <duplicate-crate>@<old-version>
cargo update -p <parent-crate>

# 변경 후
time cargo build --release 2>&1 | tail -1
cargo tree --duplicates | wc -l  # 줄어들어 있어야 함

# 보통 결과: 제거한 중복 하나당 컴파일 시간이 5~15% 감소
# (특히 syn, tokio 같은 무거운 크레이트에서 효과가 큼)
```
</details>

<a id="key-takeaways"></a>
### 핵심 정리

- 서드파티 build script를 깨뜨리지 않으려면 `RUSTFLAGS` 대신 `CARGO_ENCODED_RUSTFLAGS`를 사용하세요
- `[profile.dev.package."*"] opt-level = 2`는 개발 경험에 가장 큰 영향을 주는 단일 요령입니다
- 캐시 튜닝 (`main`에서만 `save-if`)은 활발한 저장소에서 CI 캐시 비대화를 막아 줍니다
- `cargo tree --duplicates` + `cargo update`는 공짜에 가까운 컴파일 시간 개선입니다. 매달 한 번씩 해도 좋습니다
- `cargo make pre-push`로 빠른 검사를 로컬에서 돌리면 CI 왕복 시간 낭비를 줄일 수 있습니다

---
