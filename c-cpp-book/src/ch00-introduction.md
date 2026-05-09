# C/C++ 개발자를 위한 Rust 부트스트랩 코스

## 과정 개요
- 과정에서 다루는 내용
    - Rust가 필요한 이유 (C와 C++ 양쪽 관점에서)
    - 로컬 설치
    - 타입, 함수, 제어 흐름, 패턴 매칭
    - 모듈, Cargo
    - 트레잇, 제네릭
    - 컬렉션, 에러 처리
    - 클로저, 메모리 관리, 라이프타임, 스마트 포인터
    - 동시성
    - Foreign Function Interface(FFI)를 포함한 Unsafe Rust
    - 펌웨어 팀을 위한 `no_std` 및 임베디드 Rust 핵심
    - 실제 C++ 코드를 Rust로 옮기는 패턴 사례 연구
- 이 과정에서는 `async` Rust를 다루지 않습니다. futures, executor, `Pin`, tokio, 프로덕션 async 패턴은 별도 과정인 [Async Rust Training](../async-book/)을 참고하세요.

---

# 자기주도 학습 가이드

이 자료는 강사 주도형 수업으로도, 개인 학습용 자료로도 사용할 수 있습니다. 혼자 공부한다면 아래 가이드를 따라가면 효율이 좋습니다.

**권장 학습 속도**

| 장 | 주제 | 권장 시간 | 체크포인트 |
|----------|-------|---------------|------------|
| 1–4 | 환경 구성, 타입, 제어 흐름 | 1일 | CLI 온도 변환기를 작성할 수 있다 |
| 5–7 | 자료구조, 소유권 | 1–2일 | `let s2 = s1`이 왜 `s1`을 더 이상 쓸 수 없게 만드는지 설명할 수 있다 |
| 8–9 | 모듈, 에러 처리 | 1일 | `?`를 사용해 에러를 전파하는 다중 파일 프로젝트를 만들 수 있다 |
| 10–12 | 트레잇, 제네릭, 클로저 | 1–2일 | 트레잇 바운드를 가진 제네릭 함수를 작성할 수 있다 |
| 13–14 | 동시성, unsafe/FFI | 1일 | `Arc<Mutex<T>>`로 스레드 안전 카운터를 만들 수 있다 |
| 15–16 | 심화 주제 | 각자 속도에 맞게 | 참고용 자료이므로 필요할 때 읽는다 |
| 17–19 | 모범 사례 및 참고 | 각자 속도에 맞게 | 실제 코드를 작성하면서 옆에 두고 참고한다 |

**연습문제 활용법**
- 모든 장에는 난이도 표시가 있는 실습 문제가 있습니다: 🟢 시작, 🟡 중급, 🔴 도전
- **해답을 펼치기 전에 반드시 먼저 직접 풀어보세요.** borrow checker와 씨름하는 과정 자체가 학습입니다. 컴파일러의 에러 메시지가 곧 선생님입니다.
- 15분 넘게 막히면 해답을 보고 구조를 이해한 뒤, 닫고 처음부터 다시 풀어보세요.
- [Rust Playground](https://play.rust-lang.org/)를 쓰면 로컬 설치 없이도 코드를 실행해볼 수 있습니다.

**막혔을 때는 이렇게 하세요**
- 컴파일러 에러 메시지를 천천히 읽어보세요. Rust 에러는 매우 친절한 편입니다.
- 해당 절을 다시 읽어보세요. 특히 소유권(7장)은 두 번째 읽을 때 감이 오는 경우가 많습니다.
- [Rust 표준 라이브러리 문서](https://doc.rust-lang.org/std/)는 매우 훌륭합니다. 타입이나 메서드 이름을 바로 검색해보세요.
- async 패턴이 필요하면 별도 과정인 [Async Rust Training](../async-book/)을 참고하세요.

---

# 목차

## Part I — 기초

### 1. 소개와 동기
- [발표자 소개와 전체 진행 방식](ch01-introduction-and-motivation.md#speaker-intro-and-general-approach)
- [왜 Rust인가](ch01-introduction-and-motivation.md#the-case-for-rust)
- [Rust는 이 문제들을 어떻게 해결하는가](ch01-introduction-and-motivation.md#how-does-rust-address-these-issues)
- [Rust의 다른 강점과 특징](ch01-introduction-and-motivation.md#other-rust-usps-and-features)
- [빠른 비교: Rust vs C/C++](ch01-introduction-and-motivation.md#quick-reference-rust-vs-cc)
- [왜 C/C++ 개발자에게 Rust가 필요한가](ch01-1-why-c-cpp-developers-need-rust.md)
  - [Rust가 제거하는 문제 전체 목록](ch01-1-why-c-cpp-developers-need-rust.md#what-rust-eliminates--the-complete-list)
  - [C와 C++가 공통으로 안고 있는 문제들](ch01-1-why-c-cpp-developers-need-rust.md#the-problems-shared-by-c-and-c)
  - [C++가 추가로 만드는 문제들](ch01-1-why-c-cpp-developers-need-rust.md#c-adds-more-problems-on-top)

### 2. 시작하기
- [말은 충분하다. 코드부터 보자](ch02-getting-started.md#enough-talk-already-show-me-some-code)
- [Rust 로컬 설치](ch02-getting-started.md#rust-local-installation)
- [Rust 패키지(크레이트)](ch02-getting-started.md#rust-packages-crates)
- [예제: Cargo와 크레이트](ch02-getting-started.md#example-cargo-and-crates)

### 3. 기본 타입과 변수
- [Rust 내장 타입](ch03-built-in-types.md#built-in-rust-types)
- [Rust 타입 지정과 대입](ch03-built-in-types.md#rust-type-specification-and-assignment)
- [Rust 타입 지정과 추론](ch03-built-in-types.md#rust-type-specification-and-inference)
- [Rust 변수와 가변성](ch03-built-in-types.md#rust-variables-and-mutability)

### 4. 제어 흐름
- [Rust의 if 키워드](ch04-control-flow.md#rust-if-keyword)
- [while과 for를 이용한 반복문](ch04-control-flow.md#rust-loops-using-while-and-for)
- [loop를 이용한 반복문](ch04-control-flow.md#rust-loops-using-loop)
- [Rust 표현식 블록](ch04-control-flow.md#rust-expression-blocks)

### 5. 자료구조와 컬렉션
- [Rust 배열 타입](ch05-data-structures.md#rust-array-type)
- [Rust 튜플](ch05-data-structures.md#rust-tuples)
- [Rust 참조](ch05-data-structures.md#rust-references)
- [C++ 참조 vs Rust 참조 — 핵심 차이](ch05-data-structures.md#c-references-vs-rust-references--key-differences)
- [Rust 슬라이스](ch05-data-structures.md#rust-slices)
- [Rust 상수와 static](ch05-data-structures.md#rust-constants-and-statics)
- [Rust 문자열: String vs &str](ch05-data-structures.md#rust-strings-string-vs-str)
- [Rust 구조체](ch05-data-structures.md#rust-structs)
- [Rust Vec\<T\>](ch05-data-structures.md#rust-vec-type)
- [Rust HashMap](ch05-data-structures.md#rust-hashmap-type)
- [연습문제: Vec와 HashMap](ch05-data-structures.md#exercise-vec-and-hashmap)

### 6. 패턴 매칭과 열거형
- [Rust enum 타입](ch06-enums-and-pattern-matching.md#rust-enum-types)
- [Rust match 문](ch06-enums-and-pattern-matching.md#rust-match-statement)
- [연습문제: match와 enum으로 덧셈/뺄셈 구현하기](ch06-enums-and-pattern-matching.md#exercise-implement-add-and-subtract-using-match-and-enum)

### 7. 소유권과 메모리 관리
- [Rust 메모리 관리](ch07-ownership-and-borrowing.md#rust-memory-management)
- [Rust 소유권, 대여, 라이프타임](ch07-ownership-and-borrowing.md#rust-ownership-borrowing-and-lifetimes)
- [Rust 이동 의미론](ch07-ownership-and-borrowing.md#rust-move-semantics)
- [Rust Clone](ch07-ownership-and-borrowing.md#rust-clone)
- [Rust Copy 트레잇](ch07-ownership-and-borrowing.md#rust-copy-trait)
- [Rust Drop 트레잇](ch07-ownership-and-borrowing.md#rust-drop-trait)
- [연습문제: Move, Copy, Drop](ch07-ownership-and-borrowing.md#exercise-move-copy-and-drop)
- [Rust 라이프타임과 대여](ch07-1-lifetimes-and-borrowing-deep-dive.md#rust-lifetime-and-borrowing)
- [Rust 라이프타임 표기](ch07-1-lifetimes-and-borrowing-deep-dive.md#rust-lifetime-annotations)
- [연습문제: 라이프타임이 있는 슬라이스 저장소](ch07-1-lifetimes-and-borrowing-deep-dive.md#exercise-slice-storage-with-lifetimes)
- [라이프타임 생략 규칙 심화](ch07-1-lifetimes-and-borrowing-deep-dive.md#lifetime-elision-rules-deep-dive)
- [Rust Box\<T\>](ch07-2-smart-pointers-and-interior-mutability.md#rust-boxt)
- [내부 가변성: Cell\<T\>와 RefCell\<T\>](ch07-2-smart-pointers-and-interior-mutability.md#interior-mutability-cellt-and-refcellt)
- [공유 소유권: Rc\<T\>](ch07-2-smart-pointers-and-interior-mutability.md#shared-ownership-rct)
- [연습문제: 공유 소유권과 내부 가변성](ch07-2-smart-pointers-and-interior-mutability.md#exercise-shared-ownership-and-interior-mutability)

### 8. 모듈과 크레이트
- [Rust 크레이트와 모듈](ch08-crates-and-modules.md#rust-crates-and-modules)
- [연습문제: 모듈과 함수](ch08-crates-and-modules.md#exercise-modules-and-functions)
- [워크스페이스와 크레이트(패키지)](ch08-crates-and-modules.md#workspaces-and-crates-packages)
- [연습문제: 워크스페이스와 패키지 의존성 사용하기](ch08-crates-and-modules.md#exercise-using-workspaces-and-package-dependencies)
- [crates.io의 커뮤니티 크레이트 사용하기](ch08-crates-and-modules.md#using-community-crates-from-cratesio)
- [크레이트 의존성과 SemVer](ch08-crates-and-modules.md#crates-dependencies-and-semver)
- [연습문제: rand 크레이트 사용하기](ch08-crates-and-modules.md#exercise-using-the-rand-crate)
- [Cargo.toml과 Cargo.lock](ch08-crates-and-modules.md#cargotoml-and-cargolock)
- [Cargo test 기능](ch08-crates-and-modules.md#cargo-test-feature)
- [기타 Cargo 기능](ch08-crates-and-modules.md#other-cargo-features)
- [테스트 패턴](ch08-1-testing-patterns.md)

### 9. 에러 처리
- [enum에서 Option과 Result로 연결하기](ch09-error-handling.md#connecting-enums-to-option-and-result)
- [Rust Option 타입](ch09-error-handling.md#rust-option-type)
- [Rust Result 타입](ch09-error-handling.md#rust-result-type)
- [연습문제: Option을 이용한 log() 함수 구현](ch09-error-handling.md#exercise-log-function-implementation-with-option)
- [Rust 에러 처리](ch09-error-handling.md#rust-error-handling)
- [연습문제: 에러 처리](ch09-error-handling.md#exercise-error-handling)
- [에러 처리 모범 사례](ch09-1-error-handling-best-practices.md)

### 10. 트레잇과 제네릭
- [Rust 트레잇](ch10-traits.md#rust-traits)
- [C++ 연산자 오버로딩 → Rust std::ops 트레잇](ch10-traits.md#c-operator-overloading--rust-stdops-traits)
- [연습문제: Logger 트레잇 구현](ch10-traits.md#exercise-logger-trait-implementation)
- [언제 enum을 쓰고 언제 dyn Trait를 쓸 것인가](ch10-traits.md#when-to-use-enum-vs-dyn-trait)
- [연습문제: 번역하기 전에 먼저 설계하라](ch10-traits.md#exercise-think-before-you-translate)
- [Rust 제네릭](ch10-1-generics.md#rust-generics)
- [연습문제: 제네릭](ch10-1-generics.md#exercise-generics)
- [Rust 트레잇과 제네릭 함께 쓰기](ch10-1-generics.md#combining-rust-traits-and-generics)
- [데이터 타입에서의 Rust 트레잇 제약](ch10-1-generics.md#rust-traits-constraints-in-data-types)
- [연습문제: 트레잇 제약과 제네릭](ch10-1-generics.md#exercise-traits-constraints-and-generics)
- [Rust type-state 패턴과 제네릭](ch10-1-generics.md#rust-type-state-pattern-and-generics)
- [Rust 빌더 패턴](ch10-1-generics.md#rust-builder-pattern)

### 11. 타입 시스템 고급 기능
- [Rust From과 Into 트레잇](ch11-from-and-into-traits.md#rust-from-and-into-traits)
- [연습문제: From과 Into](ch11-from-and-into-traits.md#exercise-from-and-into)
- [Rust Default 트레잇](ch11-from-and-into-traits.md#rust-default-trait)
- [기타 Rust 타입 변환](ch11-from-and-into-traits.md#other-rust-type-conversions)

### 12. 함수형 프로그래밍
- [Rust 클로저](ch12-closures.md#rust-closures)
- [연습문제: 클로저와 캡처](ch12-closures.md#exercise-closures-and-capturing)
- [Rust 이터레이터](ch12-closures.md#rust-iterators)
- [연습문제: Rust 이터레이터](ch12-closures.md#exercise-rust-iterators)
- [이터레이터 활용 도구 레퍼런스](ch12-1-iterator-power-tools.md#iterator-power-tools-reference)

### 13. 동시성
- [Rust 동시성](ch13-concurrency.md#rust-concurrency)
- [Rust가 데이터 레이스를 막는 이유: Send와 Sync](ch13-concurrency.md#why-rust-prevents-data-races-send-and-sync)
- [연습문제: 멀티스레드 단어 수 세기](ch13-concurrency.md#exercise-multi-threaded-word-count)

### 14. Unsafe Rust와 FFI
- [Unsafe Rust](ch14-unsafe-rust-and-ffi.md#unsafe-rust)
- [간단한 FFI 예제](ch14-unsafe-rust-and-ffi.md#simple-ffi-example-rust-library-function-consumed-by-c)
- [복잡한 FFI 예제](ch14-unsafe-rust-and-ffi.md#complex-ffi-example)
- [unsafe 코드의 정확성 보장](ch14-unsafe-rust-and-ffi.md#ensuring-correctness-of-unsafe-code)
- [연습문제: 안전한 FFI 래퍼 작성하기](ch14-unsafe-rust-and-ffi.md#exercise-writing-a-safe-ffi-wrapper)

## Part II — 심화

### 15. no_std — 베어메탈을 위한 Rust
- [no_std란 무엇인가](ch15-no_std-rust-without-the-standard-library.md#what-is-no_std)
- [언제 no_std를 쓰고 언제 std를 쓸 것인가](ch15-no_std-rust-without-the-standard-library.md#when-to-use-no_std-vs-std)
- [연습문제: no_std 링 버퍼](ch15-no_std-rust-without-the-standard-library.md#exercise-no_std-ring-buffer)
- [임베디드 심화](ch15-1-embedded-deep-dive.md)

### 16. 사례 연구: 실제 C++ 코드를 Rust로 옮기기
- [사례 연구 1: 상속 계층 → enum 디스패치](ch16-case-studies.md#case-study-1-inheritance-hierarchy--enum-dispatch)
- [사례 연구 2: shared_ptr 트리 → arena/index 패턴](ch16-case-studies.md#case-study-2-shared_ptr-tree--arenaindex-pattern)
- [사례 연구 3: 프레임워크 통신 → 라이프타임 대여](ch16-1-case-study-lifetime-borrowing.md#case-study-3-framework-communication--lifetime-borrowing)
- [사례 연구 4: God object → 조합 가능한 상태](ch16-1-case-study-lifetime-borrowing.md#case-study-4-god-object--composable-state)
- [사례 연구 5: trait object가 정말 맞는 경우](ch16-1-case-study-lifetime-borrowing.md#case-study-5-trait-objects--when-they-are-right)

## Part III — 모범 사례와 참고 자료

### 17. 모범 사례
- [Rust 모범 사례 요약](ch17-best-practices.md#rust-best-practices-summary)
- [과도한 clone() 피하기](ch17-1-avoiding-excessive-clone.md#avoiding-excessive-clone)
- [검사되지 않은 인덱싱 피하기](ch17-2-avoiding-unchecked-indexing.md#avoiding-unchecked-indexing)
- [중첩된 대입 피라미드 줄이기](ch17-3-collapsing-assignment-pyramids.md#collapsing-assignment-pyramids)
- [캡스톤 연습문제: 진단 이벤트 파이프라인](ch17-3-collapsing-assignment-pyramids.md#capstone-exercise-diagnostic-event-pipeline)
- [로깅과 트레이싱 생태계](ch17-4-logging-and-tracing-ecosystem.md#logging-and-tracing-ecosystem)

### 18. C++ → Rust 의미론 심화
- [캐스팅, 전처리기, 모듈, volatile, static, constexpr, SFINAE 등](ch18-cpp-rust-semantic-deep-dives.md)

### 19. Rust 매크로
- [선언적 매크로 (`macro_rules!`)](ch19-macros.md#declarative-macros-with-macro_rules)
- [표준 라이브러리의 공통 매크로](ch19-macros.md#common-standard-library-macros)
- [derive 매크로](ch19-macros.md#derive-macros)
- [attribute 매크로](ch19-macros.md#attribute-macros)
- [프로시저 매크로](ch19-macros.md#procedural-macros-conceptual-overview)
- [언제 무엇을 써야 하나: 매크로 vs 함수 vs 제네릭](ch19-macros.md#when-to-use-what-macros-vs-functions-vs-generics)
- [연습문제](ch19-macros.md#exercises)
