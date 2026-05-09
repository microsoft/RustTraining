<a id="rust-engineering-practices-beyond-cargo-build"></a>
# Rust 엔지니어링 실무 — `cargo build`를 넘어서

<a id="speaker-intro"></a>
## 발표자 소개

- Microsoft SCHIE(Silicon and Cloud Hardware Infrastructure Engineering) 팀의 Principal Firmware Architect
- 보안, 시스템 프로그래밍(펌웨어, 운영체제, 하이퍼바이저), CPU 및 플랫폼 아키텍처, C++ 시스템 분야에서 오랜 업계 경험
- 2017년(@AWS EC2)부터 Rust를 사용하기 시작했고, 그 이후로 꾸준히 이 언어를 애정해 왔음

---

> 대부분의 팀이 너무 늦게서야 발견하는 Rust 툴체인 기능을 실무 관점에서 안내합니다:
> build script, cross-compilation, benchmarking, code coverage,
> 그리고 Miri와 Valgrind를 활용한 안전성 검증까지 다룹니다. 각 장은
> 실제 하드웨어 진단 코드베이스,
> 즉 대규모 멀티 크레이트 워크스페이스에서 가져온 구체적인 예제를 사용하므로
> 모든 기법을 프로덕션 코드에 바로 대응시켜 볼 수 있습니다.

<a id="how-to-use-this-book"></a>
## 이 책 활용법

이 책은 **자기주도 학습**과 **팀 워크숍** 모두를 염두에 두고 구성되었습니다. 각 장은 대체로 독립적이므로 순서대로 읽어도 되고, 당장 필요한 주제로 바로 건너뛰어도 됩니다.

<a id="difficulty-legend"></a>
### 난이도 안내

| 기호 | 수준 | 의미 |
|:----:|------|------|
| 🟢 | 입문 | 패턴이 명확해 첫날부터 바로 써먹을 수 있는 도구 |
| 🟡 | 중급 | 툴체인 내부나 플랫폼 개념에 대한 이해가 필요 |
| 🔴 | 고급 | 깊은 툴체인 지식, nightly 기능, 또는 여러 도구를 엮는 구성 필요 |

<a id="pacing-guide"></a>
### 학습 속도 가이드

| 파트 | 장 | 예상 시간 | 핵심 결과 |
|------|----|:---------:|-----------|
| **I — 빌드와 배포** | ch01–02 | 3–4시간 | 빌드 메타데이터, 크로스 컴파일, 정적 바이너리 |
| **II — 측정과 검증** | ch03–05 | 4–5시간 | 통계 기반 벤치마킹, 커버리지 게이트, Miri/sanitizer |
| **III — 강화와 최적화** | ch06–10 | 6–8시간 | 공급망 보안, 릴리스 프로파일, 컴파일 시간 도구, `no_std`, Windows |
| **IV — 통합** | ch11–13 | 3–4시간 | 프로덕션 CI/CD 파이프라인, 현장 요령, 빠른 참고 자료 |
| | | **16–21시간** | **프로덕션 엔지니어링 파이프라인 전체 흐름** |

<a id="working-through-exercises"></a>
### 연습문제 학습 방법

각 장에는 난이도 표시가 있는 **🏋️ 연습문제**가 들어 있습니다. 해답은 펼칠 수 있는 `<details>` 블록 안에 제공됩니다. 먼저 직접 풀어본 뒤, 그 다음에 답을 확인하세요.

- 🟢 연습문제는 대개 10~15분 안에 끝낼 수 있습니다
- 🟡 연습문제는 20~40분 정도 걸리며 로컬에서 도구를 실행해야 할 수 있습니다
- 🔴 연습문제는 의미 있는 환경 구성과 실험이 필요하며 1시간 이상 걸릴 수 있습니다

<a id="prerequisites"></a>
## 선행 지식

| 개념 | 학습 위치 |
|------|-----------|
| Cargo 워크스페이스 구조 | [Rust Book 14.3장](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) |
| Feature flag | [Cargo Reference — Features](https://doc.rust-lang.org/cargo/reference/features.html) |
| `#[cfg(test)]`와 기본 테스트 | Rust Patterns 12장 |
| `unsafe` 블록과 FFI 기초 | Rust Patterns 10장 |

<a id="chapter-dependency-map"></a>
## 장 의존성 지도

```text
                 ┌──────────┐
                 │ ch00     │
                 │  Intro   │
                 └────┬─────┘
        ┌─────┬───┬──┴──┬──────┬──────┐
        ▼     ▼   ▼     ▼      ▼      ▼
      ch01  ch03 ch04  ch05   ch06   ch09
      Build Bench Cov  Miri   Deps   no_std
        │     │    │    │      │      │
        │     └────┴────┘      │      ▼
        │          │           │    ch10
        ▼          ▼           ▼   Windows
       ch02      ch07        ch07    │
       Cross    RelProf     RelProf  │
        │          │           │     │
        │          ▼           │     │
        │        ch08          │     │
        │      CompTime        │     │
        └──────────┴───────────┴─────┘
                   │
                   ▼
                 ch11
               CI/CD Pipeline
                   │
                   ▼
                ch12 ─── ch13
              Tricks    Quick Ref
```

**아무 순서로나 읽어도 되는 장**: ch01, ch03, ch04, ch05, ch06, ch09는 서로 독립적입니다.
**선행 장 이후에 읽으면 좋은 장**: ch02는 ch01이 필요하고, ch07–ch08은 ch03–ch06을 먼저 읽으면 도움이 되며, ch10은 ch09를 알고 읽으면 좋습니다.
**마지막에 읽으면 좋은 장**: ch11은 전체를 하나로 묶고, ch12는 실전 요령을 모아 두었으며, ch13은 참고 카드 역할을 합니다.

<a id="annotated-table-of-contents"></a>
## 주석 달린 목차

<a id="part-i-build-and-ship"></a>
### Part I — 빌드와 배포

| # | 장 | 난이도 | 설명 |
|---|----|:------:|------|
| 1 | [빌드 스크립트 — `build.rs` 심화](ch01-build-scripts-buildrs-in-depth.md) | 🟢 | 컴파일 타임 상수, C 코드 컴파일, protobuf 코드 생성, 시스템 라이브러리 링크, 안티패턴 |
| 2 | [크로스 컴파일 — 하나의 소스, 여러 타깃](ch02-cross-compilation-one-source-many-target.md) | 🟡 | 타깃 트리플, musl 정적 바이너리, ARM 크로스 컴파일, `cross`, `cargo-zigbuild`, GitHub Actions |

<a id="part-ii-measure-and-verify"></a>
### Part II — 측정과 검증

| # | 장 | 난이도 | 설명 |
|---|----|:------:|------|
| 3 | [벤치마킹 — 중요한 것을 측정하기](ch03-benchmarking-measuring-what-matters.md) | 🟡 | Criterion.rs, Divan, `perf` flamegraph, PGO, CI 연속 벤치마킹 |
| 4 | [코드 커버리지 — 테스트가 놓친 것을 보기](ch04-code-coverage-seeing-what-tests-miss.md) | 🟢 | `cargo-llvm-cov`, `cargo-tarpaulin`, `grcov`, Codecov/Coveralls CI 연동 |
| 5 | [Miri, Valgrind, Sanitizer — unsafe 코드 검증하기](ch05-miri-valgrind-and-sanitizers-verifying-u.md) | 🔴 | MIR 인터프리터, Valgrind memcheck/Helgrind, ASan/MSan/TSan, cargo-fuzz, loom |

<a id="part-iii-harden-and-optimize"></a>
### Part III — 강화와 최적화

| # | 장 | 난이도 | 설명 |
|---|----|:------:|------|
| 6 | [의존성 관리와 공급망 보안](ch06-dependency-management-and-supply-chain-s.md) | 🟢 | `cargo-audit`, `cargo-deny`, `cargo-vet`, `cargo-outdated`, `cargo-semver-checks` |
| 7 | [릴리스 프로파일과 바이너리 크기](ch07-release-profiles-and-binary-size.md) | 🟡 | 릴리스 프로파일 구성, LTO 트레이드오프, `cargo-bloat`, `cargo-udeps` |
| 8 | [컴파일 시간과 개발자 도구](ch08-compile-time-and-developer-tools.md) | 🟡 | `sccache`, `mold`, `cargo-nextest`, `cargo-expand`, `cargo-geiger`, 워크스페이스 lint, MSRV |
| 9 | [`no_std`와 feature 검증](ch09-no-std-and-feature-verification.md) | 🔴 | `cargo-hack`, `core`/`alloc`/`std` 계층, 커스텀 panic handler, `no_std` 코드 테스트 |
| 10 | [Windows와 조건부 컴파일](ch10-windows-and-conditional-compilation.md) | 🟡 | `#[cfg]` 패턴, `windows-sys`/`windows` 크레이트, `cargo-xwin`, 플랫폼 추상화 |

<a id="part-iv-integrate"></a>
### Part IV — 통합

| # | 장 | 난이도 | 설명 |
|---|----|:------:|------|
| 11 | [모두 합치기 — 프로덕션 CI/CD 파이프라인](ch11-putting-it-all-together-a-production-cic.md) | 🟡 | GitHub Actions 워크플로, `cargo-make`, pre-commit hook, `cargo-dist`, 캡스톤 |
| 12 | [실전에서 건진 요령](ch12-tricks-from-the-trenches.md) | 🟡 | `deny(warnings)` 함정, 캐시 튜닝, 중복 의존성 제거, `RUSTFLAGS` 등 검증된 패턴 10가지 |
| 13 | [빠른 레퍼런스 카드](ch13-quick-reference-card.md) | — | 자주 쓰는 명령, 60개 이상의 의사결정 표 항목, 추가 읽을거리 링크 |

