# C# 개발자를 위한 Rust 완전 학습 가이드

C# 경험이 있는 개발자를 위해 Rust를 배우는 과정을 체계적으로 정리한 가이드입니다. 기본 문법부터 고급 패턴까지 다루며, 두 언어 사이의 개념적 전환과 실무적인 차이를 중심에 둡니다.

## 과정 개요
- **Rust가 필요한 이유** — 성능, 안전성, 정확성 관점에서 C# 개발자에게 Rust가 왜 중요한지
- **시작하기** — 설치, 툴링, 첫 프로그램 작성
- **기초 구성 요소** — 타입, 변수, 제어 흐름
- **자료구조** — 배열, 튜플, 구조체, 컬렉션
- **패턴 매칭과 enum** — 대수적 데이터 타입과 완전 매칭
- **소유권과 대여** — Rust의 메모리 관리 모델
- **모듈과 크레이트** — 코드 구성과 의존성 관리
- **에러 처리** — `Result` 기반 에러 전파
- **트레잇과 제네릭** — Rust의 타입 시스템
- **클로저와 이터레이터** — 함수형 프로그래밍 패턴
- **동시성** — 타입 시스템이 보장하는 안전한 동시성과 async/await 심화
- **Unsafe Rust와 FFI** — 안전한 Rust를 넘어가야 할 때와 그 방법
- **마이그레이션 패턴** — 실제 C# -> Rust 전환 패턴과 점진적 도입
- **모범 사례** — C# 개발자를 위한 Rust다운 코딩 습관

---

# 자기주도 학습 가이드

이 자료는 강사 주도형 수업으로도, 개인 학습용 자료로도 사용할 수 있습니다. 혼자 학습한다면 아래 가이드를 따르면 효율이 좋습니다.

**권장 학습 속도**

| 장 | 주제 | 권장 시간 | 체크포인트 |
|----------|-------|---------------|------------|
| 1–4 | 환경 구성, 타입, 제어 흐름 | 1일 | Rust로 CLI 온도 변환기를 작성할 수 있다 |
| 5–6 | 자료구조, enum, 패턴 매칭 | 1–2일 | 데이터를 담는 enum을 정의하고 `match`로 완전하게 분기할 수 있다 |
| 7 | 소유권과 대여 | 1–2일 | `let s2 = s1`이 왜 `s1`을 무효화하는지 설명할 수 있다 |
| 8–9 | 모듈, 에러 처리 | 1일 | `?`를 사용해 에러를 전파하는 다중 파일 프로젝트를 만들 수 있다 |
| 10–12 | 트레잇, 제네릭, 클로저, 이터레이터 | 1–2일 | LINQ 체인을 Rust 이터레이터로 옮길 수 있다 |
| 13 | 동시성과 async | 1일 | `Arc<Mutex<T>>`로 스레드 안전 카운터를 작성할 수 있다 |
| 14 | Unsafe Rust, FFI, 테스트 | 1일 | C#에서 P/Invoke로 Rust 함수를 호출할 수 있다 |
| 15–16 | 마이그레이션, 모범 사례, 툴링 | 각자 속도에 맞게 | 실제 코드를 작성하면서 참고 자료로 활용한다 |
| 17 | 캡스톤 프로젝트 | 1–2일 | 날씨 데이터를 가져오는 CLI 도구를 완성한다 |

**연습문제 활용법**
- 각 장에는 접을 수 있는 `<details>` 블록 안에 실습 문제와 해답이 포함되어 있습니다.
- **해답을 펼치기 전에 반드시 먼저 직접 풀어보세요.** borrow checker와 씨름하는 과정 자체가 학습이며, 컴파일러의 에러 메시지가 곧 선생님입니다.
- 15분 이상 막히면 해답을 펼쳐 구조를 이해한 뒤, 닫고 처음부터 다시 풀어보세요.
- [Rust Playground](https://play.rust-lang.org/)를 쓰면 로컬 설치 없이도 코드를 실행할 수 있습니다.

**난이도 표시**
- 🟢 **입문** — C# 개념을 직접 Rust로 옮겨보는 단계
- 🟡 **중급** — 소유권이나 트레잇에 대한 이해가 필요함
- 🔴 **고급** — 라이프타임, async 내부 동작, unsafe 코드가 등장함

**막혔을 때는 이렇게 하세요**
- 컴파일러 에러 메시지를 천천히 읽어보세요. Rust 에러는 유난히 도움이 많이 됩니다.
- 관련 절을 다시 읽어보세요. 특히 소유권(7장)은 두 번째 읽을 때 감이 오는 경우가 많습니다.
- [Rust 표준 라이브러리 문서](https://doc.rust-lang.org/std/)는 매우 훌륭합니다. 타입이나 메서드 이름을 바로 검색해보세요.
- 더 깊은 async 패턴이 필요하면 보조 과정인 [Async Rust Training](../async-book/)을 참고하세요.

---

# 목차

## Part I — 기초

### 1. 소개와 동기 🟢
- [C# 개발자에게 Rust가 필요한 이유](ch01-introduction-and-motivation.md#the-case-for-rust-for-c-developers)
- [Rust가 해결하는 C#의 대표적인 문제점](ch01-introduction-and-motivation.md#common-c-pain-points-that-rust-addresses)
- [언제 C#보다 Rust를 선택해야 하는가](ch01-introduction-and-motivation.md#when-to-choose-rust-over-c)
- [언어 철학 비교](ch01-introduction-and-motivation.md#language-philosophy-comparison)
- [빠른 비교: Rust vs C#](ch01-introduction-and-motivation.md#quick-reference-rust-vs-c)

### 2. 시작하기 🟢
- [설치와 환경 구성](ch02-getting-started.md#installation-and-setup)
- [첫 Rust 프로그램](ch02-getting-started.md#your-first-rust-program)
- [Cargo vs NuGet/MSBuild](ch02-getting-started.md#cargo-vs-nugetmsbuild)
- [입력 읽기와 CLI 인수](ch02-getting-started.md#reading-input-and-cli-arguments)
- [핵심 Rust 키워드 *(선택 참고 자료 - 필요할 때 펼쳐보기)*](ch02-1-essential-keywords-reference.md#essential-rust-keywords-for-c-developers)

### 3. 내장 타입과 변수 🟢
- [변수와 가변성](ch03-built-in-types-and-variables.md#variables-and-mutability)
- [기본 타입 비교](ch03-built-in-types-and-variables.md#primitive-types)
- [문자열 타입: String vs &str](ch03-built-in-types-and-variables.md#string-types-string-vs-str)
- [출력과 문자열 포매팅](ch03-built-in-types-and-variables.md#printing-and-string-formatting)
- [형변환과 변환](ch03-built-in-types-and-variables.md#type-casting-and-conversions)
- [진짜 불변성과 레코드 환상](ch03-1-true-immutability-vs-record-illusions.md#true-immutability-vs-record-illusions)

### 4. 제어 흐름 🟢
- [함수 vs 메서드](ch04-control-flow.md#functions-vs-methods)
- [표현식 vs 문장 (중요!)](ch04-control-flow.md#expression-vs-statement-important)
- [조건문](ch04-control-flow.md#conditional-statements)
- [반복문과 순회](ch04-control-flow.md#loops)

### 5. 자료구조와 컬렉션 🟢
- [튜플과 구조 분해](ch05-data-structures-and-collections.md#tuples-and-destructuring)
- [배열과 슬라이스](ch05-data-structures-and-collections.md#arrays-and-slices)
- [구조체 vs 클래스](ch05-data-structures-and-collections.md#structs-vs-classes)
- [생성자 패턴](ch05-1-constructor-patterns.md#constructor-patterns)
- [`Vec<T>` vs `List<T>`](ch05-2-collections-vec-hashmap-and-iterators.md#vect-vs-listt)
- [HashMap vs Dictionary](ch05-2-collections-vec-hashmap-and-iterators.md#hashmap-vs-dictionary)

### 6. 열거형과 패턴 매칭 🟡
- [대수적 데이터 타입 vs C# 유니온](ch06-enums-and-pattern-matching.md#algebraic-data-types-vs-c-unions)
- [완전한 패턴 매칭](ch06-1-exhaustive-matching-and-null-safety.md#exhaustive-pattern-matching-compiler-guarantees-vs-runtime-errors)
- [null 안전성을 위한 `Option<T>`](ch06-1-exhaustive-matching-and-null-safety.md#null-safety-nullablet-vs-optiont)
- [가드와 고급 패턴](ch06-enums-and-pattern-matching.md#guards-and-advanced-patterns)

### 7. 소유권과 대여 🟡
- [소유권 이해하기](ch07-ownership-and-borrowing.md#understanding-ownership)
- [이동 시맨틱 vs 참조 시맨틱](ch07-ownership-and-borrowing.md#move-semantics)
- [대여와 참조](ch07-ownership-and-borrowing.md#borrowing-basics)
- [메모리 안전성 심화](ch07-1-memory-safety-deep-dive.md#references-vs-pointers)
- [라이프타임 심화](ch07-2-lifetimes-deep-dive.md#lifetimes-telling-the-compiler-how-long-references-live) 🔴
- [스마트 포인터, Drop, Deref](ch07-3-smart-pointers-beyond-single-ownership.md#smart-pointers-when-single-ownership-isnt-enough) 🔴

### 8. 크레이트와 모듈 🟢
- [Rust 모듈 vs C# 네임스페이스](ch08-crates-and-modules.md#rust-modules-vs-c-namespaces)
- [크레이트 vs .NET 어셈블리](ch08-crates-and-modules.md#crates-vs-net-assemblies)
- [패키지 관리: Cargo vs NuGet](ch08-1-package-management-cargo-vs-nuget.md#package-management-cargo-vs-nuget)

### 9. 에러 처리 🟡
- [예외 vs `Result<T, E>`](ch09-error-handling.md#exceptions-vs-resultt-e)
- [`?` 연산자](ch09-error-handling.md#the--operator-propagating-errors-concisely)
- [커스텀 에러 타입](ch06-1-exhaustive-matching-and-null-safety.md#custom-error-types)
- [크레이트 수준 에러 타입과 Result 별칭](ch09-1-crate-level-error-types-and-result-alias.md#crate-level-error-types-and-result-aliases)
- [에러 복구 패턴](ch09-1-crate-level-error-types-and-result-alias.md#error-recovery-patterns)

### 10. 트레잇과 제네릭 🟡
- [트레잇 vs 인터페이스](ch10-traits-and-generics.md#traits---rusts-interfaces)
- [상속 vs 조합](ch10-2-inheritance-vs-composition.md#inheritance-vs-composition)
- [제네릭 제약: `where` vs 트레잇 바운드](ch10-1-generic-constraints.md#generic-constraints-where-vs-trait-bounds)
- [자주 쓰는 표준 라이브러리 트레잇](ch10-traits-and-generics.md#common-standard-library-traits)

### 11. From과 Into 트레잇 🟡
- [Rust의 타입 변환](ch11-from-and-into-traits.md#type-conversions-in-rust)
- [커스텀 타입에 From 구현하기](ch11-from-and-into-traits.md#rust-from-and-into)

### 12. 클로저와 이터레이터 🟡
- [Rust 클로저](ch12-closures-and-iterators.md#rust-closures)
- [LINQ vs Rust 이터레이터](ch12-closures-and-iterators.md#linq-vs-rust-iterators)
- [매크로 입문](ch12-1-macros-primer.md#macros-code-that-writes-code)

---

## Part II — 동시성과 시스템

### 13. 동시성 🔴
- [스레드 안전성: 관례 vs 타입 시스템 보장](ch13-concurrency.md#thread-safety-convention-vs-type-system-guarantees)
- [async/await: C# Task vs Rust Future](ch13-1-asyncawait-deep-dive.md#async-programming-c-task-vs-rust-future)
- [취소 패턴](ch13-1-asyncawait-deep-dive.md#cancellation-cancellationtoken-vs-drop--select)
- [Pin과 `tokio::spawn`](ch13-1-asyncawait-deep-dive.md#pin-why-rust-async-has-a-concept-c-doesnt)

### 14. Unsafe Rust, FFI, 테스트 🟡
- [언제 왜 unsafe가 필요한가](ch14-unsafe-rust-and-ffi.md#when-you-need-unsafe)
- [FFI를 통한 C# 연동](ch14-unsafe-rust-and-ffi.md#interop-with-c-via-ffi)
- [Rust 테스트 vs C# 테스트](ch14-1-testing.md#testing-in-rust-vs-c)
- [프로퍼티 테스트와 모킹](ch14-1-testing.md#property-testing-proving-correctness-at-scale)

---

## Part III — 마이그레이션과 모범 사례

### 15. 마이그레이션 패턴과 사례 연구 🟡
- [Rust에서의 흔한 C# 패턴](ch15-migration-patterns-and-case-studies.md#common-c-patterns-in-rust)
- [C# 개발자를 위한 필수 크레이트](ch15-1-essential-crates-for-c-developers.md#essential-crates-for-c-developers)
- [점진적 도입 전략](ch15-2-incremental-adoption-strategy.md#incremental-adoption-strategy)

### 16. 모범 사례와 참고 자료 🟡
- [C# 개발자를 위한 Rust다운 작성법](ch16-best-practices.md#best-practices-for-c-developers)
- [성능 비교: 관리형 vs 네이티브](ch16-1-performance-comparison-and-migration.md#performance-comparison-managed-vs-native)
- [흔한 함정과 해결책](ch16-2-learning-path-and-resources.md#common-pitfalls-for-c-developers)
- [학습 경로와 자료](ch16-2-learning-path-and-resources.md#learning-path-and-next-steps)
- [Rust 툴링 생태계](ch16-3-rust-tooling-ecosystem.md#essential-rust-tooling-for-c-developers)

---

## 캡스톤

### 17. 캡스톤 프로젝트 🟡
- [CLI 날씨 도구 만들기](ch17-capstone-project.md#capstone-project-build-a-cli-weather-tool) — 구조체, 트레잇, 에러 처리, async, 모듈, serde, 테스트를 묶어 실제로 동작하는 애플리케이션을 완성합니다.


