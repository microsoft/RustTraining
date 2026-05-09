# Rust에서 타입 주도 정확성

<a id="speaker-intro"></a>
## 발표자 소개

- Microsoft SCHIE(Silicon and Cloud Hardware Infrastructure Engineering) 팀의 Principal Firmware Architect
- 보안, 시스템 프로그래밍(펌웨어, 운영체제, 하이퍼바이저), CPU·플랫폼 아키텍처, C++ 시스템을 아우르는 업계 경력
- 2017년부터 Rust(@AWS EC2)에서 프로그래밍을 시작했으며, 그 이후로 언어에 매료되어 있음

---

Rust의 타입 시스템을 활용해 **컴파일 자체가 불가능한** 버그 클래스를 만드는 실용 가이드입니다. 동반서인 [Rust Patterns](../rust-patterns-book/index.html)가 메커니즘(트레잇, 연관 타입, 타입 상태)을 다룬다면, 이 가이드는 그 메커니즘을 **하드웨어 진단, 암호화, 프로토콜 검증, 임베디드** 같은 실제 도메인에 **적용**하는 방법을 보여 줍니다.

여기서 다루는 패턴은 모두 한 가지 원칙을 따릅니다. **불변식을 런타임 검사에서 타입 시스템으로 옮겨 컴파일러가 강제하도록 한다.**

<a id="how-to-use-this-book"></a>
## 이 책 사용법

<a id="difficulty-legend"></a>
### 난이도 범례

| 기호 | 수준 | 대상 |
|:------:|-------|----------|
| 🟢 | 입문 | 소유권 + 트레잇에 익숙한 개발자 |
| 🟡 | 중급 | 제네릭 + 연관 타입에 익숙한 개발자 |
| 🔴 | 고급 | 타입 상태, 팬텀 타입, 세션 타입까지 다룰 준비가 된 개발자 |

<a id="pacing-guide"></a>
### 학습 속도 가이드

| 목표 | 경로 | 시간 |
|------|------|------|
| **빠른 개요** | ch01, ch13(레퍼런스 카드) | 30분 |
| **IPMI / BMC 개발자** | ch02, ch05, ch07, ch10, ch17 | 2.5시간 |
| **GPU / PCIe 개발자** | ch02, ch06, ch09, ch10, ch15 | 2.5시간 |
| **Redfish 구현자** | ch02, ch05, ch07, ch08, ch17, ch18 | 3시간 |
| **프레임워크 / 인프라** | ch04, ch08, ch11, ch14, ch18 | 2.5시간 |
| **correct-by-construction이 처음** | ch01 → ch10 순서, 이후 ch12 연습문제 | 4시간 |
| **전체 심화** | 모든 장 순차 | 7시간 |

<a id="annotated-table-of-contents"></a>
### 주석 달린 목차

| 장 | 제목 | 난이도 | 핵심 아이디어 |
|----|-------|:----------:|----------|
| 1 | 철학 — 왜 타입이 테스트를 이기는가 | 🟢 | 정확성의 세 가지 수준; Curry–Howard 직관 |
| 2 | 타입이 있는 명령 인터페이스 | 🟡 | 연관 타입으로 요청 → 응답 연결 |
| 3 | 단일 사용 타입 | 🟡 | 암호화를 위한 선형 타입으로서의 move 의미 |
| 4 | Capability Token | 🟡 | 권한의 제로 크기 증명 토큰 |
| 5 | 프로토콜 상태 머신 | 🔴 | IPMI 세션 + PCIe LTSSM을 위한 타입 상태 |
| 6 | 차원 분석 | 🟢 | 뉴타입 래퍼로 단위 혼동 방지 |
| 7 | 검증된 경계 | 🟡 | 경계에서 한 번만 Parse, 타입에 증명을 실어 나르기 |
| 8 | Capability Mixin | 🟡 | 재료 트레잇 + blanket impl |
| 9 | 팬텀 타입 | 🟡 | 레지스터 폭, DMA 방향을 위한 `PhantomData` |
| 10 | 모두 합치기 | 🟡 | 하나의 진단 플랫폼에 7가지 패턴 |
| 11 | 실전에서 건진 열네 가지 요령 | 🟡 | Sentinel→Option, sealed trait, 빌더 등 |
| 12 | 연습문제 | 🟡 | 해답이 있는 여섯 가지 캡스톤 문제 |
| 13 | 레퍼런스 카드 | — | 패턴 목록 + 의사결정 플로차트 |
| 14 | 타입 수준 보장 테스트하기 | 🟡 | trybuild, proptest, cargo-show-asm |
| 15 | Const Fn | 🟠 | 메모리 맵, 레지스터, 비트필드에 대한 컴파일 타임 증명 |
| 16 | Send & Sync | 🟠 | 컴파일 타임 동시성 증명 |
| 17 | Redfish 클라이언트 워크스루 | 🟡 | 여덟 가지 패턴을 조합한 타입 안전 Redfish 클라이언트 |
| 18 | Redfish 서버 워크스루 | 🟡 | 빌더 타입 상태, 소스 토큰, 헬스 롤업, 믹스인 |

<a id="prerequisites"></a>
## 선행 지식

| 개념 | 어디서 배울 수 있는지 |
|---------|-------------------|
| 소유권과 빌림 | [Rust Patterns](../rust-patterns-book/index.html), ch01 |
| 트레잇과 연관 타입 | [Rust Patterns](../rust-patterns-book/index.html), ch02 |
| 뉴타입과 타입 상태 | [Rust Patterns](../rust-patterns-book/index.html), ch03 |
| `PhantomData` | [Rust Patterns](../rust-patterns-book/index.html), ch04 |
| 제네릭과 트레잇 바운드 | [Rust Patterns](../rust-patterns-book/index.html), ch01 |

<a id="the-correct-by-construction-spectrum"></a>
## 구성으로 증명하는 정확성 스펙트럼

```text
← 덜 안전                                                    더 안전 →

런타임 검사      단위 테스트        프로퍼티 테스트      구성으로 증명(Correct by Construction)
─────────────       ──────────        ──────────────      ──────────────────────

if temp > 100 {     #[test]           proptest! {         struct Celsius(f64);
  panic!("too       fn test_temp() {    |t in 0..200| {   // Rpm과 혼동 불가
  hot");              assert!(          assert!(...)       // 타입 수준에서
}                     check(42));     }
                    }                 }
                                                          잘못된 프로그램?
잘못된 프로그램?    잘못된 프로그램?  잘못된 프로그램?    컴파일되지 않음.
프로덕션에서 크래시. CI에서 실패. CI에서 실패 (확률적).
```

이 가이드는 가장 오른쪽 — 타입 시스템이 **표현조차 할 수 없는** 버그가 없는 지점에서 동작합니다.

---

