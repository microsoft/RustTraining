<a id="async-rust-from-futures-to-production"></a>
<a id="introduction"></a>
# Async Rust: Future부터 프로덕션까지

<a id="speaker-intro"></a>
## 발표자 소개

- Microsoft SCHIE(Silicon and Cloud Hardware Infrastructure Engineering) 팀의 수석 펌웨어 아키텍트
- 보안, 시스템 프로그래밍(펌웨어, 운영체제, 하이퍼바이저), CPU 및 플랫폼 아키텍처, C++ 시스템 분야를 다뤄 온 업계 베테랑
- 2017년(@AWS EC2)부터 Rust를 사용하기 시작했고, 그 이후로 계속 이 언어를 깊이 좋아하게 됨

---

Rust의 비동기 프로그래밍을 깊이 있게 다루는 가이드입니다. 많은 async 튜토리얼이 `tokio::main`부터 시작하고 내부 동작은 대충 넘어가는 것과 달리, 이 가이드는 `Future` 트레잇, polling, 상태 머신 같은 기초 원리부터 이해를 쌓은 뒤, 실제 패턴, runtime 선택, 프로덕션에서 만나는 함정까지 단계적으로 다룹니다.

<a id="who-this-is-for"></a>
## 이 책이 적합한 독자

- 동기 Rust 코드는 작성할 수 있지만 async가 헷갈리는 Rust 개발자
- `async/await`는 알지만 Rust의 모델은 익숙하지 않은 C#, Go, Python, JavaScript 개발자
- `Future is not Send`, `Pin<Box<dyn Future>>`, "왜 내 프로그램은 멈추지?" 같은 문제를 겪어본 사람

<a id="prerequisites"></a>
## 선수 지식

다음 주제에 익숙해야 합니다:

- 소유권, 대여, 라이프타임
- 트레잇과 제네릭(`impl Trait` 포함)
- `Result<T, E>`와 `?` 연산자 사용
- 기본적인 멀티스레딩(`std::thread::spawn`, `Arc`, `Mutex`)

Async Rust 경험은 없어도 됩니다.

<a id="how-to-use-this-book"></a>
## 이 책을 활용하는 방법

**처음에는 순서대로 읽으세요.** Part I–III는 앞선 내용을 바탕으로 이어집니다. 각 장에는 다음 요소가 들어 있습니다:

| 기호 | 의미 |
|--------|---------|
| 🟢 | 입문 — 기초 개념 |
| 🟡 | 중급 — 앞선 장의 내용이 필요함 |
| 🔴 | 고급 — 깊은 내부 동작 또는 프로덕션 패턴 |

각 장에는 다음이 포함됩니다:

- 맨 위의 **"이 장에서 배울 내용"** 블록
- 시각적으로 이해하기 쉬운 **Mermaid 다이어그램**
- 펼쳐서 볼 수 있는 **본문 중간 연습문제**
- 핵심 아이디어를 요약하는 **핵심 정리**
- 관련 장으로 이어지는 **상호 참조**

<a id="pacing-guide"></a>
## 권장 학습 속도

| 장 | 주제 | 권장 시간 | 체크포인트 |
|----------|-------|----------------|------------|
| 1–5 | Async가 작동하는 방식 | 6–8시간 | `Future`, `Poll`, `Pin`, 그리고 Rust에 내장 runtime이 없는 이유를 설명할 수 있다 |
| 6–10 | 생태계 | 6–8시간 | future를 직접 만들고, runtime을 선택하고, tokio API를 사용할 수 있다 |
| 11–13 | 프로덕션 Async | 6–8시간 | stream, 적절한 에러 처리, graceful shutdown을 포함한 프로덕션 수준의 async 코드를 작성할 수 있다 |
| 캡스톤 | 채팅 서버 | 4–6시간 | 모든 개념을 통합한 실제 async 애플리케이션을 만들 수 있다 |

**총 예상 시간: 22–30시간**

<a id="working-through-exercises"></a>
## 연습문제 활용법

모든 본문 장에는 중간 연습문제가 있습니다. 캡스톤(16장)은 앞서 배운 내용을 하나의 프로젝트로 통합합니다. 학습 효과를 높이려면:

1. **해답을 펼치기 전에 먼저 직접 풀어보세요** — 막혀 보는 과정에서 학습이 일어납니다
2. **코드를 직접 타이핑하세요. 복붙하지 마세요** — Rust 문법은 손으로 익히는 감각도 중요합니다
3. **모든 예제를 직접 실행해 보세요** — `cargo new async-exercises`로 시작해 하나씩 확인하세요

<a id="table-of-contents"></a>
## 목차

<a id="part-i-how-async-works"></a>
### Part I: Async가 작동하는 방식

- [1. Rust에서 Async가 다른 이유](ch01-why-async-is-different-in-rust.md) 🟢 — 핵심 차이: Rust에는 내장 runtime이 없다
- [2. Future 트레잇](ch02-the-future-trait.md) 🟡 — `poll()`, `Waker`, 그리고 전체 동작을 가능하게 하는 계약
- [3. Poll은 어떻게 동작하는가](ch03-how-poll-works.md) 🟡 — polling 상태 머신과 최소 executor
- [4. Pin과 Unpin](ch04-pin-and-unpin.md) 🔴 — 자기 참조 구조체에 pinning이 필요한 이유
- [5. 상태 머신의 정체](ch05-the-state-machine-reveal.md) 🟢 — `async fn`에서 컴파일러가 실제로 생성하는 것

<a id="part-ii-the-ecosystem"></a>
### Part II: 생태계

- [6. Future를 직접 만들기](ch06-building-futures-by-hand.md) 🟡 — TimerFuture, Join, Select를 처음부터 구현하기
- [7. Executor와 Runtime](ch07-executors-and-runtimes.md) 🟡 — tokio, smol, async-std, embassy 중 무엇을 선택할지
- [8. Tokio 심화](ch08-tokio-deep-dive.md) 🟡 — runtime 종류, spawn, 채널, 동기화 프리미티브
- [9. Tokio가 맞지 않는 경우](ch09-when-tokio-isnt-the-right-fit.md) 🟡 — LocalSet, FuturesUnordered, runtime-agnostic 설계
- [10. Async 트레잇](ch10-async-traits.md) 🟡 — RPITIT, dyn dispatch, trait_variant, async closure

<a id="part-iii-production-async"></a>
### Part III: 프로덕션 Async

- [11. Streams와 AsyncIterator](ch11-streams-and-asynciterator.md) 🟡 — 비동기 반복, AsyncRead/Write, stream combinator
- [12. 흔한 함정](ch12-common-pitfalls.md) 🔴 — 프로덕션에서 실제로 터지는 9가지 버그와 회피법
- [13. 프로덕션 패턴](ch13-production-patterns.md) 🔴 — graceful shutdown, backpressure, Tower 미들웨어

<a id="appendices"></a>
### 부록

- [요약 및 레퍼런스 카드](ch15-summary-and-reference-card.md) — 빠르게 찾아볼 수 있는 표와 의사결정 트리
- [캡스톤 프로젝트: Async 채팅 서버](ch16-capstone-project.md) — 완전한 async 애플리케이션 만들기

***


