# Python 개발자를 위한 Rust 완전 학습 가이드

Python 경험이 있는 개발자를 위해 Rust를 배울 때 필요한 내용을 폭넓게 담은 가이드입니다. 기본 문법부터 고급 패턴까지 다루며, 동적 타이핑과 가비지 컬렉션에 익숙한 환경에서 정적 타입과 컴파일 타임 메모리 안전성을 갖춘 시스템 언어로 넘어갈 때 필요한 사고방식의 전환에 초점을 맞춥니다.

<a id="how-to-use-this-book"></a>
## 이 책을 활용하는 방법

**자기주도 학습 순서**: 먼저 Part I(1~6장)을 차례대로 학습하세요. 이 장들은 이미 익숙한 Python 개념과 가장 직접적으로 연결됩니다. Part II(7~12장)에서는 ownership, traits처럼 Rust 고유의 핵심 아이디어를 다룹니다. Part III(13~16장)에서는 고급 주제와 마이그레이션 전략을 살펴봅니다.

**권장 학습 속도:**

| 장 | 주제 | 권장 시간 | 체크포인트 |
|----------|-------|---------------|------------|
| 1–4 | 환경 구성, 타입, 제어 흐름 | 1일 | Rust로 CLI 온도 변환기를 작성할 수 있다 |
| 5–6 | 자료구조, enum, 패턴 매칭 | 1–2일 | 데이터를 담는 enum을 정의하고 `match`로 빠짐없이 처리할 수 있다 |
| 7 | ownership과 borrowing | 1–2일 | `let s2 = s1`이 *왜* `s1`을 더 이상 쓸 수 없게 만드는지 설명할 수 있다 |
| 8–9 | 모듈, 에러 처리 | 1일 | `?`로 에러를 전파하는 다중 파일 프로젝트를 만들 수 있다 |
| 10–12 | traits, generics, closures, iterators | 1–2일 | list comprehension을 iterator 체인으로 옮길 수 있다 |
| 13 | 동시성 | 1일 | `Arc<Mutex<T>>`로 스레드 안전 카운터를 작성할 수 있다 |
| 14 | unsafe, PyO3, 테스트 | 1일 | PyO3를 통해 Python에서 Rust 함수를 호출할 수 있다 |
| 15–16 | 마이그레이션, 모범 사례 | 각자 속도에 맞게 | 실제 코드를 작성할 때 곁에 두는 참고 자료로 활용한다 |
| 17 | 캡스톤 프로젝트 | 2–3일 | 모든 내용을 연결한 완전한 CLI 앱을 만든다 |

**연습문제 활용법:**
- 각 장에는 해답이 접혀 있는 `<details>` 블록 형태의 실습 문제가 포함되어 있습니다.
- **해답을 펼치기 전에 반드시 먼저 직접 풀어보세요.** borrow checker와 씨름하는 과정도 학습의 일부이며, 컴파일러의 에러 메시지가 곧 선생님입니다.
- 15분 넘게 막히면 해답을 펼쳐 구조를 이해한 뒤, 다시 닫고 처음부터 풀어보세요.
- [Rust Playground](https://play.rust-lang.org/)를 이용하면 로컬 설치 없이도 코드를 실행해볼 수 있습니다.

**난이도 표시:**
- 🟢 **입문** — Python 개념을 Rust로 직접 옮겨 보는 수준
- 🟡 **중급** — ownership이나 trait 이해가 필요한 수준
- 🔴 **고급** — 라이프타임, async 내부 구조, unsafe code를 다루는 수준

**막혔을 때는 이렇게 하세요:**
- 컴파일러 에러 메시지를 차근차근 읽어보세요. Rust 에러는 매우 친절한 편입니다.
- 해당 절을 다시 읽어보세요. ownership(7장) 같은 개념은 두 번째 읽을 때 감이 오는 경우가 많습니다.
- [Rust 표준 라이브러리 문서](https://doc.rust-lang.org/std/)는 훌륭합니다. 타입이나 메서드 이름을 바로 검색해보세요.
- async 패턴을 더 깊이 배우고 싶다면 companion [Async Rust Training](../async-book/)을 참고하세요.

---

<a id="table-of-contents"></a>
## 목차

<a id="part-i-foundations"></a>
### Part I — 기초

#### 1. 소개와 동기 🟢
- [Python 개발자에게 Rust가 필요한 이유](ch01-introduction-and-motivation.md#the-case-for-rust-for-python-developers)
- [Rust가 해결하는 Python의 대표적인 문제들](ch01-introduction-and-motivation.md#common-python-pain-points-that-rust-addresses)
- [언제 Python 대신 Rust를 선택할 것인가](ch01-introduction-and-motivation.md#when-to-choose-rust-over-python)

#### 2. 시작하기 🟢
- [설치와 설정](ch02-getting-started.md#installation-and-setup)
- [첫 Rust 프로그램](ch02-getting-started.md#your-first-rust-program)
- [Cargo와 pip/Poetry 비교](ch02-getting-started.md#cargo-vs-pippoetry)

#### 3. 내장 타입과 변수 🟢
- [변수와 가변성](ch03-built-in-types-and-variables.md#variables-and-mutability)
- [기본 타입 비교](ch03-built-in-types-and-variables.md#primitive-types-comparison)
- [문자열 타입: String vs &str](ch03-built-in-types-and-variables.md#string-types-string-vs-str)

#### 4. 제어 흐름 🟢
- [조건문](ch04-control-flow.md#conditional-statements)
- [반복문과 순회](ch04-control-flow.md#loops-and-iteration)
- [표현식 블록](ch04-control-flow.md#expression-blocks)
- [함수와 타입 시그니처](ch04-control-flow.md#functions-and-type-signatures)

#### 5. 자료구조와 컬렉션 🟢
- [튜플, 배열, 슬라이스](ch05-data-structures-and-collections.md#tuples-and-destructuring)
- [구조체 vs 클래스](ch05-data-structures-and-collections.md#structs-vs-classes)
- [Vec vs list, HashMap vs dict](ch05-data-structures-and-collections.md#vec-vs-list)

#### 6. 열거형과 패턴 매칭 🟡
- [대수적 데이터 타입 vs 유니언 타입](ch06-enums-and-pattern-matching.md#algebraic-data-types-vs-union-types)
- [누락 없는 패턴 매칭](ch06-enums-and-pattern-matching.md#exhaustive-pattern-matching)
- [Option으로 None 안전성 확보](ch06-enums-and-pattern-matching.md#option-for-none-safety)

<a id="part-ii-core-concepts"></a>
### Part II — 핵심 개념

#### 7. 소유권과 대여 🟡
- [소유권 이해하기](ch07-ownership-and-borrowing.md#understanding-ownership)
- [move semantics vs 참조 카운팅](ch07-ownership-and-borrowing.md#move-semantics-vs-reference-counting)
- [borrowing과 라이프타임](ch07-ownership-and-borrowing.md#borrowing-and-lifetimes)
- [스마트 포인터](ch07-ownership-and-borrowing.md#smart-pointers)

#### 8. 크레이트와 모듈 🟢
- [Rust 모듈 vs Python 패키지](ch08-crates-and-modules.md#rust-modules-vs-python-packages)
- [크레이트 vs PyPI 패키지](ch08-crates-and-modules.md#crates-vs-pypi-packages)

#### 9. 에러 처리 🟡
- [예외 vs Result](ch09-error-handling.md#exceptions-vs-result)
- [`?` 연산자](ch09-error-handling.md#the--operator)
- [`thiserror`로 커스텀 에러 타입 만들기](ch09-error-handling.md#custom-error-types-with-thiserror)

#### 10. 트레잇과 제네릭 🟡
- [trait vs duck typing](ch10-traits-and-generics.md#traits-vs-duck-typing)
- [Protocol(PEP 544) vs trait](ch10-traits-and-generics.md#protocols-pep-544-vs-traits)
- [제네릭 제약](ch10-traits-and-generics.md#generic-constraints)

#### 11. From과 Into 트레잇 🟡
- [Rust의 타입 변환](ch11-from-and-into-traits.md#type-conversions-in-rust)
- [From, Into, TryFrom](ch11-from-and-into-traits.md#rust-frominto)
- [문자열 변환 패턴](ch11-from-and-into-traits.md#string-conversions)

#### 12. 클로저와 이터레이터 🟡
- [클로저 vs 람다](ch12-closures-and-iterators.md#rust-closures-vs-python-lambdas)
- [이터레이터 vs 제너레이터](ch12-closures-and-iterators.md#iterators-vs-generators)
- [매크로: 코드를 작성하는 코드](ch12-closures-and-iterators.md#why-macros-exist-in-rust)

<a id="part-iii-advanced-topics-migration"></a>
### Part III — 고급 주제와 마이그레이션

#### 13. 동시성 🔴
- [GIL 없는 진짜 병렬성](ch13-concurrency.md#no-gil-true-parallelism)
- [스레드 안전성: 타입 시스템이 보장하는 안전](ch13-concurrency.md#thread-safety-type-system-guarantees)
- [async/await 비교](ch13-concurrency.md#asyncawait-comparison)

#### 14. Unsafe Rust, FFI, 테스트 🔴
- [언제, 왜 unsafe를 쓰는가](ch14-unsafe-rust-and-ffi.md#when-and-why-to-use-unsafe)
- [PyO3: Python용 Rust 확장](ch14-unsafe-rust-and-ffi.md#pyo3-rust-extensions-for-python)
- [단위 테스트 vs pytest](ch14-unsafe-rust-and-ffi.md#unit-tests-vs-pytest)

#### 15. 마이그레이션 패턴 🟡
- [Rust로 옮기는 대표적인 Python 패턴](ch15-migration-patterns.md#common-python-patterns-in-rust)
- [Python 개발자를 위한 필수 크레이트](ch08-crates-and-modules.md#essential-crates-for-python-developers)
- [점진적 도입 전략](ch15-migration-patterns.md#incremental-adoption-strategy)

#### 16. 모범 사례 🟡
- [Python 개발자를 위한 Rust다운 코드](ch16-best-practices.md#idiomatic-rust-for-python-developers)
- [자주 겪는 함정과 해결책](ch16-best-practices.md#common-pitfalls-and-solutions)
- [Python→Rust 로제타 스톤](ch16-best-practices.md#rosetta-stone-python-to-rust)
- [학습 경로와 참고 자료](ch16-best-practices.md#learning-path-and-resources)

---

<a id="part-iv-capstone"></a>
### Part IV — 캡스톤

#### 17. 캡스톤 프로젝트: CLI 작업 관리자 🔴
- [프로젝트 소개: `rustdo`](ch17-capstone-project.md#the-project-rustdo)
- [데이터 모델, 저장소, 명령, 비즈니스 로직](ch17-capstone-project.md#step-1-define-the-data-model-ch-3-6-10-11)
- [테스트와 확장 과제](ch17-capstone-project.md#step-7-tests-ch-14)

***
