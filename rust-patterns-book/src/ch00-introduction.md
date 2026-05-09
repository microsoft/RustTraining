# Rust 패턴과 엔지니어링 실무 가이드

<a id="speaker-intro"></a>
## 발표자 소개

- Microsoft SCHIE(Silicon and Cloud Hardware Infrastructure Engineering) 팀의 Principal Firmware Architect
- 보안, 시스템 프로그래밍(펌웨어, 운영체제, 하이퍼바이저), CPU·플랫폼 아키텍처, C++ 시스템 분야의 업계 베테랑
- 2017년(@AWS EC2)부터 Rust를 사용했으며, 그 이후로 언어에 매료되어 있음

---

실제 코드베이스에서 자주 등장하는 중급 이상 Rust 패턴을 다루는 실용서입니다. 언어 입문서가 아니라, 기본적인 Rust는 작성할 수 있고 한 단계 올리고 싶은 독자를 대상으로 합니다. 각 장은 개념 하나에 집중해, 언제·왜 쓰는지 설명하고 컴파일 가능한 예제와 인라인 연습문제를 제공합니다.

<a id="who-this-is-for"></a>
## 이런 분께 권합니다

- *The Rust Programming Language*는 읽었지만 “실제로 어떻게 설계하지?”에서 막히는 개발자
- 프로덕션 시스템을 Rust로 옮기는 C++/C# 엔지니어
- 제네릭, 트레잇 바운드, 라이프타임 에러에서 막혀 체계적인 도구상자가 필요한 분

<a id="prerequisites"></a>
## 선행 지식

시작하기 전에 다음에 익숙해 두는 것이 좋습니다.

- 소유권, 빌림, 라이프타임(기본 수준)
- 열거형, 패턴 매칭, `Option`/`Result`
- 구조체, 메서드, 기본 트레잇(`Display`, `Debug`, `Clone`)
- Cargo 기본: `cargo build`, `cargo test`, `cargo run`

<a id="how-to-use-this-book"></a>
## 이 책 활용법

<a id="difficulty-legend"></a>
### 난이도 표기

각 장에는 난이도 태그가 붙어 있습니다.

| 기호 | 수준 | 의미 |
|--------|-------|-------------|
| 🟢 | 기초 | 모든 Rust 개발자가 알아야 할 핵심 개념 |
| 🟡 | 중급 | 프로덕션 코드베이스에서 쓰이는 패턴 |
| 🔴 | 고급 | 언어 메커니즘을 깊게 다룸 — 필요할 때 다시 읽기 |

<a id="pacing-guide"></a>
### 학습 속도 가이드

| 장 | 주제 | 권장 시간 | 체크포인트 |
|----------|-------|----------------|------------|
| **Part I: 타입 수준 패턴** | | | |
| 1. 제네릭 🟢 | 단형성화, const 제네릭, `const fn` | 1–2시간 | `dyn Trait`가 제네릭보다 나은 경우를 설명할 수 있다 |
| 2. 트레잇 🟡 | 연관 타입, GAT, blanket 구현, vtable | 3–4시간 | 연관 타입을 가진 트레잇을 설계할 수 있다 |
| 3. 뉴타입·타입 상태 🟡 | 제로 코스트 안전성, 컴파일 타임 FSM, 빌더 | 2–3시간 | 타입 상태 빌더 패턴을 만들 수 있다 |
| 4. PhantomData 🔴 | 라이프타임 브랜딩, 분산, drop check | 2–3시간 | `PhantomData<fn(T)>`와 `PhantomData<T>`가 왜 다른지 설명할 수 있다 |
| **Part II: 동시성·런타임** | | | |
| 5. 채널 🟢 | `mpsc`, crossbeam, `select!`, 액터 | 1–2시간 | 채널 기반 워커 풀을 구현할 수 있다 |
| 6. 동시성 🟡 | 스레드, rayon, Mutex, RwLock, 원자 연산 | 2–3시간 | 상황에 맞는 동기화 원시 타입을 고를 수 있다 |
| 7. 클로저 🟢 | `Fn`/`FnMut`/`FnOnce`, 조합자 | 1–2시간 | 클로저를 인자로 받는 고차 함수를 작성할 수 있다 |
| 8. 스마트 포인터 🟡 | Box, Rc, Arc, RefCell, Cow, Pin | 2–3시간 | 각 스마트 포인터의 사용 시점을 설명할 수 있다 |
| **Part III: 시스템·프로덕션** | | | |
| 9. 에러 처리 🟢 | thiserror, anyhow, `?` 연산자 | 1–2시간 | 에러 타입 계층을 설계할 수 있다 |
| 10. 직렬화 🟡 | serde, 제로 카피, 바이너리 데이터 | 2–3시간 | 커스텀 serde 역직렬화를 작성할 수 있다 |
| 11. Unsafe 🔴 | 초능력, FFI, UB 함정, 할당자 | 2–3시간 | unsafe를 건전한 안전 API로 감쌀 수 있다 |
| 12. 매크로 🟡 | `macro_rules!`, 절차 매크로, `syn`/`quote` | 2–3시간 | `tt` 먼칭이 있는 선언적 매크로를 작성할 수 있다 |
| 13. 테스트 🟢 | 단위/통합/문서 테스트, proptest, criterion | 1–2시간 | 속성 기반 테스트를 설정할 수 있다 |
| 14. API 설계 🟡 | 모듈 구성, 인체공학적 API, feature 플래그 | 2–3시간 | “파싱하면 검증이 된다(parse, don't validate)” 패턴을 적용할 수 있다 |
| 15. Async 🔴 | Future, Tokio, 흔한 함정 | 1–2시간 | async 안티패턴을 짚을 수 있다 |
| **부록** | | | |
| 레퍼런스 카드 | 트레잇 바운드·라이프타임·패턴 한눈에 | 필요 시 | — |
| 캡스톤 | 타입 안전 작업 스케줄러 | 4–6시간 | 동작하는 구현 제출 |

**전체 예상 시간**: 연습을 포함해 꼼꼼히 공부하면 약 30–45시간입니다.

<a id="working-through-exercises"></a>
### 연습문제 풀이 팁

각 장 끝에는 실습 문제가 있습니다. 학습 효과를 높이려면:

1. **먼저 직접 풀기** — 해답을 보기 전에 최소 15분은 시도하기
2. **직접 타이핑** — 복붙하지 않기; 타이핑이 근육 기억을 만든다
3. **해답을 변형하기** — 기능 추가, 제약 변경, 의도적으로 깨뜨려 보기
4. **교차 참조 확인** — 대부분의 문제는 여러 장의 패턴을 조합한다

부록의 캡스톤 프로젝트는 책 전체의 패턴을 하나의 프로덕션 수준 시스템으로 묶습니다.

<a id="table-of-contents"></a>
## 목차

<a id="part-i-type-level-patterns"></a>
### Part I: 타입 수준 패턴

**[1. 제네릭 — 전체 그림](ch01-generics-the-full-picture.md)** 🟢  
단형성화, 바이너리 팽창 트레이드오프, 제네릭 vs 열거형 vs 트레잇 객체, const 제네릭, `const fn`.

**[2. 트레잇 심화](ch02-traits-in-depth.md)** 🟡  
연관 타입, GAT, blanket 구현, 마커 트레잇, vtable, HRTB, 확장 트레잇, 열거형 디스패치.

**[3. 뉴타입과 타입 상태 패턴](ch03-the-newtype-and-type-state-patterns.md)** 🟡  
제로 코스트 타입 안전성, 컴파일 타임 상태 머신, 빌더 패턴, Config 트레잇.

**[4. PhantomData — 데이터를 담지 않는 타입](ch04-phantomdata-types-that-carry-no-data.md)** 🔴  
라이프타임 브랜딩, 단위(unit-of-measure) 패턴, drop check, 분산(variance).

<a id="part-ii-concurrency-runtime"></a>
### Part II: 동시성·런타임

**[5. 채널과 메시지 전달](ch05-channels-and-message-passing.md)** 🟢  
`std::sync::mpsc`, crossbeam, `select!`, 백프레셔, 액터 패턴.

**[6. 동시성 vs 병렬성 vs 스레드](ch06-concurrency-vs-parallelism-vs-threads.md)** 🟡  
OS 스레드, 스코프 스레드, rayon, Mutex/RwLock/원자 연산, Condvar, OnceLock, 락프리 패턴.

**[7. 클로저와 고차 함수](ch07-closures-and-higher-order-functions.md)** 🟢  
`Fn`/`FnMut`/`FnOnce`, 매개변수·반환값으로서의 클로저, 조합자, 고차 API.

**[8. 스마트 포인터와 내부 가변성](ch08-smart-pointers-and-interior-mutability.md)** 🟡  
Box, Rc, Arc, Weak, Cell/RefCell, Cow, Pin, ManuallyDrop.

<a id="part-iii-systems-production"></a>
### Part III: 시스템·프로덕션

**[9. 에러 처리 패턴](ch09-error-handling-patterns.md)** 🟢  
thiserror vs anyhow, `#[from]`, `.context()`, `?` 연산자, 패닉.

**[10. 직렬화, 제로 카피, 바이너리 데이터](ch10-serialization-zero-copy-and-binary-data.md)** 🟡  
serde 기초, 열거형 표현, 제로 카피 역직렬화, `repr(C)`, `bytes::Bytes`.

**[11. Unsafe Rust — 통제된 위험](ch11-unsafe-rust-controlled-danger.md)** 🔴  
다섯 가지 초능력, 건전한 추상화, FFI, UB 함정, 아레나/슬랩 할당자.

**[12. 매크로 — 코드를 작성하는 코드](ch12-macros-code-that-writes-code.md)** 🟡  
`macro_rules!`, 매크로를 (안) 쓰는 경우, 절차 매크로, derive 매크로, `syn`/`quote`.

**[13. 테스트와 벤치마킹 패턴](ch13-testing-and-benchmarking-patterns.md)** 🟢  
단위/통합/문서 테스트, proptest, criterion, 모킹 전략.

**[14. 크레이트 아키텍처와 API 설계](ch14-crate-architecture-and-api-design.md)** 🟡  
모듈 배치, API 설계 체크리스트, 인체공학적 매개변수, feature 플래그, 워크스페이스.

**[15. Async/Await 핵심](ch15-asyncawait-essentials.md)** 🔴  
Future, Tokio 빠른 시작, 흔한 함정. (async 심화는 별도 Async Rust Training 참고.)

<a id="appendices"></a>
### 부록

**[요약 및 레퍼런스 카드](ch17-summary-and-reference-card.md)**  
패턴 선택 가이드, 트레잇 바운드 치트 시트, 라이프타임 생략 규칙, 추가 읽을거리.

**[캡스톤 프로젝트: 타입 안전 작업 스케줄러](ch18-capstone-project.md)**  
제네릭, 트레잇, 타입 상태, 채널, 에러 처리, 테스트를 하나의 완결된 시스템으로 통합합니다.

***
