# C# → Rust 학습: 통합 장 계획

## 원본 문서

| 문서 | 파일 | 줄 수 |
|-----|------|-------|
| **부트스트랩(B)** | `RustBootstrapForCSharp.md` | 5,363 |
| **고급(A)** | `RustTrainingForCSharp.md` | 3,021 |
| **원본 합계** | | **8,384** |
| **통합 추정** | (중복 제거 후) | **~5,800** |

## Mermaid 다이어그램 목록 (총 13개 — 모두 고급(A) 문서)

| # | 고급 문서 행 | 주제 | 대상 장 |
|---|----------|---------|----------------|
| M1 | L84 | 개발 모델 비교 | ch01 |
| M2 | L173 | 메모리 관리: GC vs RAII | ch01 |
| M3 | L282 | C# null 처리의 변천 | ch06.1 |
| M4 | L410 | C# 판별 공용체(우회 방법) | ch06 |
| M5 | L536 | C# 패턴 매칭의 한계 | ch06.1 |
| M6 | L829 | C# 레코드 — 얕은 불변성 | ch03.1 |
| M7 | L998 | 런타임 안전 vs 컴파일 타임 안전 | ch07.1 |
| M8 | L1153 | C# 상속 계층 | ch10.2 |
| M9 | L1153 | C# 예외 모델 | ch09 |
| M10 | L1290 | C# LINQ 특성 | ch12 |
| M11 | L1463 | C# 제네릭 제약 | ch10.1 |
| M12 | L2156 | C# 스레드 안전 과제 | ch13 |
| M13 | L2850 | 마이그레이션 전략 의사결정 트리 | ch16 |

---

## 장 구조

### 제0장: 서문
<!-- ch00: Introduction -->

**파일:** `ch00-introduction.md`
**추정 줄 수:** ~30
**내용:** 책 개요, 이 가이드 사용법, 전제 조건(C# 경험 가정).
**출처:** 신규 내용(C/C++ 책 ch00 패턴을 참고).

---

### 제1장: 소개와 동기
<!-- ch01: Introduction and Motivation -->

**파일:** `ch01-introduction-and-motivation.md`
**추정 줄 수:** ~380
**Mermaid 다이어그램:** M1, M2

| 하위 섹션 마커 | 출처 | 줄 수 | 비고 |
|---|---|---|---|
| <!-- ch01.1: Quick Reference --> | B L93–110 | 18 | 빠른 참조 표 — **C# 문서에만 있음, 그대로 유지** |
| <!-- ch01.2: Language Philosophy --> | A L70–125 | 56 | C# vs Rust 철학; **M1** 포함 |
| <!-- ch01.3: GC vs RAII --> | A L126–214 | 89 | GC vs 소유권 개요; **M2** 포함 |
| <!-- ch01.4: The Case for Rust --> | B L111–221 | 111 | 성능, 메모리 안전 논거 |
| <!-- ch01.5: C# Pain Points --> | B L222–348 | 80 | ~80줄로 다듬기(null, 예외, GC 관련 문제 — ch01.2–01.3에서 이미 다룬 A 철학·GC와 겹치는 부분 제거) |
| <!-- ch01.6: When to Choose --> | B L349–400 | 52 | Rust vs C# 선택, 실제 영향 |

**중복 정리:** 부트스트랩 문서의 「고통 지점(Pain Points)」 §1(Null)과 §3(GC)은 고급(A) 문서의 철학·GC 대 RAII와 부분 겹침. 고급 문서 버전(Mermaid 포함)을 유지하고, 부트스트랩 쪽 고통 지점은 중복을 피해 다듬음. §2(숨은 예외)는 고유 — 전부 유지.

---

### 제2장: 시작하기
<!-- ch02: Getting Started -->

**파일:** `ch02-getting-started.md`
**추정 줄 수:** ~170

| 하위 섹션 마커 | 출처 | 줄 수 | 비고 |
|---|---|---|---|
| <!-- ch02.1: Installation --> | B L401–434 | 34 | rustup, 도구 비교 표 |
| <!-- ch02.2: First Program --> | B L435–486 | 52 | Hello World C# vs Rust 비교 |
| <!-- ch02.3: Cargo vs NuGet --> | B L487–564 | 78 | 프로젝트 설정, 명령, workspace vs solution |

#### 하위 장: ch02.1 — C# 개발자를 위한 필수 Rust 키워드
<!-- ch02.1: Keywords Reference -->

**파일:** `ch02-1-keywords-reference.md`
**추정 줄 수:** ~400
**출처:** B L842–1244 (403줄)
**비고:** 이 포괄적인 키워드 대응 표는 **C# 문서에만 있음**. 가시성, 메모리, 제어 흐름, 타입 정의, 함수, 변수, 패턴 매칭, 안전 키워드를 C# 대응에서 모두 매핑. 그대로 유지. ~400줄 규모로 별도 하위 장이 타당함.

---

### 제3장: 기본 제공 타입
<!-- ch03: Built-in Types -->

**파일:** `ch03-built-in-types.md`
**추정 줄 수:** ~280

| 하위 섹션 마커 | 출처 | 줄 수 | 비고 |
|---|---|---|---|
| <!-- ch03.1: Variables and Mutability --> | B L565–641 | 77 | let vs var, mut, const, shadowing |
| <!-- ch03.2: Primitive Types --> | B L642–707 | 66 | 타입 비교 표, 크기 타입, 추론 |
| <!-- ch03.3: String Types --> | B L708–782 | 75 | String vs &str, 실용 예 |
| <!-- ch03.4: Comments and Docs --> | B L783–841 | 59 | 주석, 문서 주석, rustdoc |

#### 하위 장: ch03.1 — 진짜 불변성 심화
<!-- ch03.1: True Immutability -->

**파일:** `ch03-1-true-immutability.md`
**추정 줄 수:** ~136
**출처:** A L577–712 (136줄)
**Mermaid 다이어그램:** M6
**비고:** C# 레코드의 "불변 연극" vs Rust의 진짜 불변성. **M6**(레코드 — 얕은 불변성 다이어그램) 포함. **C# 문서에만 있는 내용** — `record`가 진짜 불변은 아닌 이유를 C# 개발자가 알아야 함.

---

### 제4장: 제어 흐름
<!-- ch04: Control Flow -->

**파일:** `ch04-control-flow.md`
**추정 줄 수:** ~280

| 하위 섹션 마커 | 출처 | 줄 수 | 비고 |
|---|---|---|---|
| <!-- ch04.1: Functions vs Methods --> | B L1638–1745 | 108 | 선언, 식 vs 문, 매개변수/반환 |
| <!-- ch04.2: Conditionals --> | B L1748–1792 | 45 | if/else, if-let, 삼항 대응 |
| <!-- ch04.3: Loops --> | B L1793–1886 | 93 | loop, while, for, 루프 제어(break/continue 레이블) |
| <!-- ch04.4: Pattern Matching Preview --> | B L1887–1978 | 35 | 짧은 소개만(92줄에서 ~35줄로 다듬음); 전체는 ch06. 앞으로 참조: "자세한 내용은 6장 참고." |

**비고:** 패턴 매칭 소개 전체(B L1887–1978, 92줄)는 ch06과 많이 겹침. 기본 `match` 문법 미리보기만(~35줄) 추출하고 ch06을 앞으로 참조.

---

### 제5장: 자료 구조
<!-- ch05: Data Structures -->

**파일:** `ch05-data-structures.md`
**추정 줄 수:** ~380

| 하위 섹션 마커 | 출처 | 줄 수 | 비고 |
|---|---|---|---|
| <!-- ch05.1: Arrays and Slices --> | B L2445–2548 | 104 | C# 배열 vs Rust 배열, 슬라이스, 문자열 슬라이스 |
| <!-- ch05.2: Structs vs Classes --> | B L2673–2807 | 135 | 구조체 정의, 인스턴스 생성, 초기화 패턴 |
| <!-- ch05.3: Methods and Associated Functions --> | B L2808–2941 | 134 | impl 블록, &self/&mut self/self, 메서드 수신자 타입 |

#### 하위 장: ch05.1 — 생성자 패턴
<!-- ch05.1: Constructor Patterns -->

**파일:** `ch05-1-constructor-patterns.md`
**추정 줄 수:** ~210
**출처:** B L3084–3291 (208줄)
**비고:** C# 생성자 vs Rust `new()` 관례, `Default` 트레잇, 빌더 패턴 구현. 큰 독립 섹션이라 하위 장이 적합.

#### 하위 장: ch05.2 — 컬렉션: Vec, HashMap, 반복
<!-- ch05.2: Collections -->

**파일:** `ch05-2-collections.md`
**추정 줄 수:** ~390

| 하위 섹션 마커 | 출처 | 줄 수 | 비고 |
|---|---|---|---|
| <!-- ch05.2.1: Vec vs List --> | B L2163–2307 | 145 | 생성, 초기화, 일반 연산, 안전 접근 |
| <!-- ch05.2.2: HashMap vs Dictionary --> | B L2308–2444 | 137 | 연산, entry API, 키/값과 소유권 |
| <!-- ch05.2.3: Working with Collections --> | B L2549–2672 | 110 | 반복 패턴, IntoIterator/Iter, 결과 수집(다듬음 — LINQ 스타일 이터레이터는 ch12로 이동) |

**중복 비고:** 「컬렉션 다루기(Working with Collections)」(B L2549–2672)에 ch12(클로저/LINQ)와 겹치는 이터레이터 체인이 일부 있음. 기본 반복 패턴은 여기 유지, 고급 체인·LINQ 비교는 ch12로.

---

### 제6장: 열거형과 패턴 매칭
<!-- ch06: Enums and Pattern Matching -->

**파일:** `ch06-enums-and-pattern-matching.md`
**추정 줄 수:** ~320
**Mermaid 다이어그램:** M4

| 하위 섹션 마커 | 출처 | 줄 수 | 비고 |
|---|---|---|---|
| <!-- ch06.1: C# Enum Limitations --> | B L3296–3342 | 47 | C# enum이 제한적인 이유 |
| <!-- ch06.2: Rust Enum Power --> | B L3343–3378 | 36 | 데이터를 가진 enum 변형 |
| <!-- ch06.3: Algebraic Data Types --> | A L319–451 | 100 | ADT vs C# union; **M4** 포함. 133줄에서 ~100줄로 다듬음(위 기본 enum과 겹침 제거) |
| <!-- ch06.4: Pattern Matching --> | B L3379–3461 | 83 | match 식, 구조 분해 |
| <!-- ch06.5: Guards and Advanced --> | B L3462–3502 | 41 | match 가드, 중첩 패턴 |

#### 하위 장: ch06.1 — 완전 매칭과 null 안전
<!-- ch06.1: Exhaustive Matching and Null Safety -->

**파일:** `ch06-1-exhaustive-matching-and-null-safety.md`
**추정 줄 수:** ~300
**Mermaid 다이어그램:** M3, M5

| 하위 섹션 마커 | 출처 | 줄 수 | 비고 |
|---|---|---|---|
| <!-- ch06.1.1: Exhaustive Matching --> | A L452–576 | 125 | 컴파일러 보장 vs 런타임 오류; **M5** 포함 |
| <!-- ch06.1.2: Null Safety: Option --> | A L215–318 | 80 | Nullable<T> vs Option<T>; **M3** 포함. 104줄에서 ~80줄로 다듬음(B의 Option 절과 겹침 제거) |
| <!-- ch06.1.3: Option and Result --> | B L3503–3615 | 113 | Option<T>와 Result<T,E> 실용 사용 |

**중복 정리:** 두 문서 모두 Option<T>를 다룸. 고급(A L215–318)에 Mermaid와 null 처리 변천 서술이 더 깊음 — 개념 도입으로 사용. 부트스트랩(B L3503–3615)에 실습 코드 예가 많음 — 실무 파트로 유지. 겹치는 예는 하나로 정리.

---

### 제7장: 소유권과 대여
<!-- ch07: Ownership and Borrowing -->

**파일:** `ch07-ownership-and-borrowing.md`
**추정 줄 수:** ~330

| 하위 섹션 마커 | 출처 | 줄 수 | 비고 |
|---|---|---|---|
| <!-- ch07.1: C# Memory Model --> | B L1249–1267 | 19 | C# 참조 타입, GC 복습 |
| <!-- ch07.2: Ownership Rules --> | B L1268–1316 | 49 | 세 규칙, C# 개발자를 위한 Move, Copy vs Move |
| <!-- ch07.3: Practical Examples --> | B L1317–1348 | 32 | 값 교환 예 |
| <!-- ch07.4: Borrowing --> | B L1349–1472 | 124 | 공유/가변 참조, 대여 규칙, ref 안전 비교 |
| <!-- ch07.5: Move Semantics --> | B L1540–1637 | 98 | 값/참조 타입 vs move 의미, move 피하기 |

#### 하위 장: ch07.1 — 참조, 포인터, 메모리 안전
<!-- ch07.1: Memory Safety Deep Dive -->

**파일:** `ch07-1-references-pointers-and-memory-safety.md`
**추정 줄 수:** ~220
**Mermaid 다이어그램:** M7

| 하위 섹션 마커 | 출처 | 줄 수 | 비고 |
|---|---|---|---|
| <!-- ch07.1.1: References vs Pointers --> | B L1473–1539 | 67 | C# unsafe 포인터 vs Rust 안전 참조, 수명 기초 |
| <!-- ch07.1.2: Memory Safety --> | A L713–870 | 158 | 런타임 검사 vs 컴파일 타임 증명; **M7** 포함. 소유권이 버그 범주 전체를 막는 이유에 대한 가장 깊은 설명 — **C# 독자에게 특유의 깊이** |

---

### 제8장: 크레이트와 모듈
<!-- ch08: Crates and Modules -->

**파일:** `ch08-crates-and-modules.md`
**추정 줄 수:** ~340

| 하위 섹션 마커 | 출처 | 줄 수 | 비고 |
|---|---|---|---|
| <!-- ch08.1: Modules vs Namespaces --> | B L3674–3882 | 209 | C# namespace → Rust 모듈 매핑, 계층, 가시성, 파일 구성 |
| <!-- ch08.2: Crates vs Assemblies --> | B L3883–4009 | 127 | 어셈블리 모델 vs 크레이트 모델, 크레이트 종류, workspace vs solution |

#### 하위 장: ch08.1 — 패키지 관리 심화
<!-- ch08.1: Package Management -->

**파일:** `ch08-1-package-management.md`
**추정 줄 수:** ~235

| 하위 섹션 마커 | 출처 | 줄 수 | 비고 |
|---|---|---|---|
| <!-- ch08.1.1: Dependencies --> | B L4010–4055 | 46 | Cargo.toml vs .csproj, 의존성 종류 |
| <!-- ch08.1.2: Version Management --> | B L4056–4089 | 34 | 시맨틱 버전, Cargo.lock |
| <!-- ch08.1.3: Package Sources --> | B L4090–4132 | 43 | crates.io vs NuGet, 대체 레지스트리 |
| <!-- ch08.1.4: Features --> | B L4133–4182 | 50 | feature 플래그 vs #if DEBUG 조건부 컴파일 |
| <!-- ch08.1.5: External Crates --> | B L4183–4244 | 62 | 인기 크레이트 목록, HTTP 클라이언트 마이그레이션 예 |

---

### 제9장: 에러 처리
<!-- ch09: Error Handling -->

**파일:** `ch09-error-handling.md`
**추정 줄 수:** ~350
**Mermaid 다이어그램:** M9

| 하위 섹션 마커 | 출처 | 줄 수 | 비고 |
|---|---|---|---|
| <!-- ch09.1: C# Exception Model --> | A L1046–1089 | 44 | 예외 기반 처리, 문제점; **M9** 맥락 일부 |
| <!-- ch09.2: Exceptions vs Result --> | A L1090–1194 | 105 | Result 기반 에러 처리(고급 문서 버전 — 더 깊고 Mermaid **M9** 포함) |
| <!-- ch09.3: The ? Operator --> | B L2057–2084 | 28 | `?` 연산자를 "C#의 await과 비슷하다"로 설명 |
| <!-- ch09.4: Custom Error Types --> | B L3616–3673 | 58 | thiserror 기반 사용자 정의 에러(enum 장에서 이동) |
| <!-- ch09.5: Error Handling Deep Dive --> | B L4558–4715 | 120 | 에러 처리 패턴 종합(158줄에서 다듬음 — 위 A의 Result와 겹침 제거) |

**중복 정리:** 에러 처리를 다루는 출처가 세 곳:
1. **B L1979–2162** 「에러 처리 기초」(원문 *Error Handling Basics*, 184줄) — 입문
2. **B L4558–4715** 「에러 처리 심화」(원문 *Error Handling Deep Dive*, 158줄) — 고급 패턴
3. **A L1046–1194** 「예외와 Result」(원문 *Exceptions vs Result*, 149줄) — Mermaid가 있는 개념 비교

**전략:** 개념 틀은 A 버전 사용(M9 다이어그램과 C# 비교가 더 깊음). 실무 패턴은 B 심화 절. B 기초 절은 제외(A + B 심화 조합과 중복). `?` 설명은 B 기초에서만 잘 풀리므로 그대로 유지.

#### 하위 장: ch09.1 — 에러 처리 모범 사례
<!-- ch09.1: Error Handling Best Practices -->

**파일:** `ch09-1-error-handling-best-practices.md`
**추정 줄 수:** ~80
**출처:** B L4612–4715에서 추출(본 ch09에서 다루지 않은 실무 패턴) + A L2916–2938(모범 사례 절의 에러 처리 전략).
**비고:** `anyhow` vs `thiserror` 선택, 에러 변환 패턴, 에러 컨텍스트 체인. C/C++ 책의 ch09 + ch09.1 패턴을 따름.

---

### 제10장: 트레잇과 제네릭
<!-- ch10: Traits and Generics -->

**파일:** `ch10-traits.md`
**추정 줄 수:** ~380

| 하위 섹션 마커 | 출처 | 줄 수 | 비고 |
|---|---|---|---|
| <!-- ch10.1: Traits vs Interfaces --> | B L4245–4383 | 139 | 정의, 구현, C# 인터페이스 비교 |
| <!-- ch10.2: Implementing Behavior --> | B L2942–3083 | 100 | 구조체에 트레잇 구현, 여러 impl(142줄에서 다듬음 — ch10.1과 겹침 제거) |
| <!-- ch10.3: Trait Objects --> | B L4385–4443 | 59 | 동적 디스패치, dyn Trait, Box<dyn Trait> |
| <!-- ch10.4: Derived Traits --> | B L4444–4491 | 48 | #[derive], 파생 가능한 일반 트레잇 |
| <!-- ch10.5: Std Library Traits --> | B L4492–4557 | 40 | Display, Debug, Clone, Iterator(다듬음 — From/Into는 ch11로) |

#### 하위 장: ch10.1 — 제네릭과 제약
<!-- ch10.1: Generics -->

**파일:** `ch10-1-generics.md`
**추정 줄 수:** ~170
**출처:** A L1338–1505 (168줄)
**Mermaid 다이어그램:** M11
**비고:** C# `where T : class` vs Rust 트레잇 바운드, 단형화, 연관 타입. **M11**(제네릭 제약 다이어그램) 포함. 고급(A) 문서 설명이 부트스트랩(B)보다 훨씬 깊음.

#### 하위 장: ch10.2 — 상속 vs 합성
<!-- ch10.2: Inheritance vs Composition -->

**파일:** `ch10-2-inheritance-vs-composition.md`
**추정 줄 수:** ~175
**출처:** A L871–1045 (175줄)
**Mermaid 다이어그램:** M8
**비고:** C# 상속 계층 vs Rust 합성 모델. **M8**(상속 계층 다이어그램) 포함. 클래스 계층을 버려야 하는 **C# 개발자에게 특히 유용**. 트레잇 객체로 다형성, newtype, 위임.

---

### 제11장: From과 Into 트레잇
<!-- ch11: From and Into Traits -->

**파일:** `ch11-from-and-into-traits.md`
**추정 줄 수:** ~120

| 하위 섹션 마커 | 출처 | 줄 수 | 비고 |
|---|---|---|---|
| <!-- ch11.1: From/Into Basics --> | B L4492–4530 | 40 | From<T> 구현, 자동 Into<T>(표준 라이브러리 트레잇 절에서 추출) |
| <!-- ch11.2: Conversion Patterns --> | 신규 | 40 | C# implicit/explicit 연산자 vs From/Into, TryFrom/TryInto |
| <!-- ch11.3: Error Conversions --> | B L4617–4650 | 30 | 에러 타입 변환을 위한 From<E>(에러 처리 심화에서 추출) |
| <!-- ch11.4: Practical Examples --> | 신규 | 10 | 문자열 변환, 숫자 타입 변환 |

**비고:** 두 원본 모두 From/Into 전용 장이 없음. 부트스트랩 표준 라이브러리 트레잇(From/Into 예)과 에러 처리(에러 변환용 From)에서 조립. C# implicit/explicit 캐스트 매핑을 위한 연결 문단은 신규 필요. 장은 작음(~120줄)이나 C/C++ 책 구조와 맞춤.

---

### 제12장: 클로저와 이터레이터
<!-- ch12: Closures and Iterators -->

**파일:** `ch12-closures-and-iterators.md`
**추정 줄 수:** ~300
**Mermaid 다이어그램:** M10

| 하위 섹션 마커 | 출처 | 줄 수 | 비고 |
|---|---|---|---|
| <!-- ch12.1: Closures --> | 신규 | 60 | C# 람다 vs Rust 클로저, Fn/FnMut/FnOnce, 캡처 의미(람다에 익숙한 C# 독자 — 소유권 차이에 집중) |
| <!-- ch12.2: LINQ vs Iterators --> | A L1195–1337 | 143 | LINQ→이터레이터 전면 매핑; **M10** 포함. **C# 개발자에게 특히 가치 있음** |
| <!-- ch12.3: Advanced Iteration --> | B L2595–2672 | 78 | Iterator/IntoIterator/Iter 구분, 결과 수집(ch05 컬렉션에서 이동한 고급 이터레이션) |

**비고:** C/C++ 책은 ch12가 「클로저」. C# 개발자에게 클로저 자체는 익숙하므로 초점은 (1) Rust 클로저 차이(소유권 캡처), (2) LINQ→이터레이터 매핑(핵심). 고급 문서의 LINQ 절이 뛰어나고 고유함.

---

### 제13장: 동시성
<!-- ch13: Concurrency -->

**파일:** `ch13-concurrency.md`
**추정 줄 수:** ~260
**Mermaid 다이어그램:** M12

| 하위 섹션 마커 | 출처 | 줄 수 | 비고 |
|---|---|---|---|
| <!-- ch13.1: Thread Safety --> | A L1947–2155 | 209 | 관례 vs 타입 시스템 보장, Send/Sync, Arc/Mutex, 채널; **M12** 포함 |
| <!-- ch13.2: Async Comparison --> | A L2156–2204 | 49 | Rust async/await vs C# async/await, tokio 런타임 |

**비고:** 전부 고급(A) 문서. 스레드 안전 절이 포괄적이며 C# 스레드 안전 과제를 보여주는 M12 Mermaid 포함. 그다음 async 비교가 자연스럽게 이어짐. 부트스트랩 문서에는 동시성이 없어 불필요.

---

### 제14장: Unsafe Rust와 FFI
<!-- ch14: Unsafe Rust and FFI -->

**파일:** `ch14-unsafe-rust-and-ffi.md`
**추정 줄 수:** ~120

| 하위 섹션 마커 | 출처 | 줄 수 | 비고 |
|---|---|---|---|
| <!-- ch14.1: Unsafe Blocks --> | 신규 | 50 | C# `unsafe` 키워드 vs Rust `unsafe` 블록, unsafe가 허용하는 것, 안전 불변식 |
| <!-- ch14.2: FFI Basics --> | 신규 | 40 | C# P/Invoke + COM Interop vs Rust FFI(`extern "C"`), bindgen |
| <!-- ch14.3: When to Use Unsafe --> | 신규 | 30 | 가이드라인, 안전 API로 감싼 unsafe 추상화 |

**비고:** 두 원본 모두 unsafe/FFI 전용 절이 없음(고급 문서 목차에는 있으나 미작성). 신규 내용 필요. C# 개발자에게 핵심 매핑: `unsafe {}`, P/Invoke → `extern "C"`, COM Interop → FFI 바인딩. C#→Rust 전환에서 덜 쓰이므로 간결히.

---

### 제15장: 사례 연구와 실무 마이그레이션
<!-- ch15: Case Studies -->

**파일:** `ch15-case-studies.md`
**추정 줄 수:** ~400

| 하위 섹션 마커 | 출처 | 줄 수 | 비고 |
|---|---|---|---|
| <!-- ch15.1: Config Management --> | B L4720–4854 | 135 | C# IConfiguration → Rust config 크레이트 마이그레이션 |
| <!-- ch15.2: Data Processing --> | B L4855–5039 | 185 | LINQ 파이프라인 → Rust 이터레이터 파이프라인 |
| <!-- ch15.3: HTTP Client --> | B L5040–5218 | 80 | HttpClient → reqwest 마이그레이션(179줄에서 다듬음 — ch15.2 필수 크레이트·UserService 예와 겹침 제거) |

#### 하위 장: ch15.1 — 흔한 패턴과 필수 크레이트
<!-- ch15.1: Common Patterns and Essential Crates -->

**파일:** `ch15-1-common-patterns-and-essential-crates.md`
**추정 줄 수:** ~400

| 하위 섹션 마커 | 출처 | 줄 수 | 비고 |
|---|---|---|---|
| <!-- ch15.1.1: Repository Pattern --> | A L1506–1625 | 120 | C# repository → Rust 트레잇 기반 repository |
| <!-- ch15.1.2: Builder Pattern --> | A L1626–1743 | 118 | C# 빌더 → 소비 self 빌더 |
| <!-- ch15.1.3: Essential Crates --> | A L1744–1946 | 160 | **C# 문서에만 있음.** Cargo.toml 템플릿으로 C# 라이브러리마다 Rust 대응(serde↔Json, reqwest↔HttpClient, tokio↔Task, thiserror↔Exception, sqlx↔EF 등) + 전체 UserService 예. 203줄에서 ~160줄로 다듬음(ch15 HTTP 클라이언트와 겹침 제거) |

#### 하위 장: ch15.2 — 도입 전략과 개념 매핑
<!-- ch15.2: Adoption Strategy -->

**파일:** `ch15-2-adoption-strategy.md`
**추정 줄 수:** ~390

| 하위 섹션 마커 | 출처 | 줄 수 | 비고 |
|---|---|---|---|
| <!-- ch15.2.1: Concept Mapping --> | A L2428–2595 | 168 | **고유하고 가치 큼.** DI → 트레잇 주입, LINQ → 이터레이터 체인, EF → SQLx, IConfiguration → config 크레이트. 각각 C#/Rust 코드 나란히 |
| <!-- ch15.2.2: Incremental Adoption --> | A L2205–2427 | 120 | 1/2/3단계 도입 전략(223줄에서 다듬음 — 필수 크레이트·개념 매핑과 겹침 제거) |
| <!-- ch15.2.3: Team Timeline --> | A L2596–2708 | 100 | 1/2/3개월+ 일정과 구체 마일스톤(113줄에서 다듬음 — 도입 단계와 겹침 제거) |

---

### 제16장: 모범 사례
<!-- ch16: Best Practices -->

**파일:** `ch16-best-practices.md`
**추정 줄 수:** ~340
**Mermaid 다이어그램:** M13

| 하위 섹션 마커 | 출처 | 줄 수 | 비고 |
|---|---|---|---|
| <!-- ch16.1: Mindset Shifts --> | A L2886–2891 | 6 | 핵심 사고방식 전환 |
| <!-- ch16.2: Code Organization --> | A L2892–2915 | 24 | 프로젝트 구조 권장 |
| <!-- ch16.3: Testing Patterns --> | A L2939–2974 | 36 | #[test], #[cfg(test)], 통합 테스트 |
| <!-- ch16.4: Common Mistakes --> | A L2975–3021 | 47 | 상속 시도, unwrap 남용, 과도한 clone, RefCell 과다 |
| <!-- ch16.5: Performance Comparison --> | A L2709–2883 | 130 | 관리 vs 네이티브 성능 특성, 벤치마크, CPU 작업부하, 판단 기준; **M13**(마이그레이션 전략 의사결정 트리) 포함. 175줄에서 ~130줄로 다듬음(ch01 「언제 선택할지」와 겹침 제거) |
| <!-- ch16.6: Common Pitfalls --> | B L5288–5363 | 76 | 소유권 혼동, borrow checker와의 싸움, null 기대 |

#### 하위 장: ch16.1 — 학습 경로와 자료
<!-- ch16.1: Learning Path -->

**파일:** `ch16-1-learning-path.md`
**추정 줄 수:** ~100
**출처:** B L5219–5287 (69줄) + B L5269–5287에서 선별(자료)
**비고:** 주·월 단위 학습 계획. 도서, 온라인 자료, 연습 프로젝트. 145줄에서 ~100줄로 다듬음(일정 내용은 ch15.2 팀 일정과 겹침).

---

## `SUMMARY.md` 초안 (mdBook 요약 목록 형식)

```markdown
# 요약

[서문](ch00-introduction.md)

---

- [1. 소개와 동기](ch01-introduction-and-motivation.md)
- [2. 시작하기](ch02-getting-started.md)
    - [키워드 참조](ch02-1-keywords-reference.md)
- [3. 기본 제공 타입](ch03-built-in-types.md)
    - [진짜 불변성 심화](ch03-1-true-immutability.md)
- [4. 제어 흐름](ch04-control-flow.md)
- [5. 자료 구조](ch05-data-structures.md)
    - [생성자 패턴](ch05-1-constructor-patterns.md)
    - [컬렉션: Vec, HashMap, 반복](ch05-2-collections.md)
- [6. 열거형과 패턴 매칭](ch06-enums-and-pattern-matching.md)
    - [완전 매칭과 null 안전](ch06-1-exhaustive-matching-and-null-safety.md)
- [7. 소유권과 대여](ch07-ownership-and-borrowing.md)
    - [참조, 포인터, 메모리 안전](ch07-1-references-pointers-and-memory-safety.md)
- [8. 크레이트와 모듈](ch08-crates-and-modules.md)
    - [패키지 관리 심화](ch08-1-package-management.md)
- [9. 에러 처리](ch09-error-handling.md)
    - [에러 처리 모범 사례](ch09-1-error-handling-best-practices.md)
- [10. 트레잇과 제네릭](ch10-traits.md)
    - [제네릭](ch10-1-generics.md)
    - [상속 vs 합성](ch10-2-inheritance-vs-composition.md)
- [11. From과 Into 트레잇](ch11-from-and-into-traits.md)
- [12. 클로저와 이터레이터](ch12-closures-and-iterators.md)
- [13. 동시성](ch13-concurrency.md)
- [14. Unsafe Rust와 FFI](ch14-unsafe-rust-and-ffi.md)
- [15. 사례 연구](ch15-case-studies.md)
    - [흔한 패턴과 필수 크레이트](ch15-1-common-patterns-and-essential-crates.md)
    - [도입 전략과 개념 매핑](ch15-2-adoption-strategy.md)
- [16. 모범 사례](ch16-best-practices.md)
    - [학습 경로와 자료](ch16-1-learning-path.md)
```

---

## 중복 정리 요약

| 겹치는 주제 | 부트스트랩(B) 출처 | 고급(A) 출처 | 처리 |
|---|---|---|---|
| **Option/null 안전** | B L2085–2133, B L3503–3615 | A L215–318 (M3) | 개념 도입은 A(Mermaid). 실습은 B L3503–3615. B L2085–2133 제거(중복). → ch06.1 |
| **에러 처리** | B L1979–2162(기초), B L4558–4715(심화) | A L1046–1194 (M9) | 개념 틀은 A(Mermaid). 패턴은 B 심화. B 기초 제거(중복). → ch09 |
| **패턴 매칭** | B L1887–1978(소개), B L3379–3502(전체) | A L452–576 (M5) | ch04에 B 소개 ~35줄만. 전체는 B L3379+에서 ch06. 완전 매칭은 A. → ch04, ch06 |
| **트레잇/인터페이스** | B L4245–4557(전체), B L2942–3083(impl) | A L871–1045(상속, M8) | B로 트레잇 기계(ch10 본문). A로 상속-합성 철학(ch10.2). B impl은 ch10 본문에 병합. |
| **GC vs 소유권** | B L222–270(고통 지점) | A L126–214 (M2) | A 사용(Mermaid). B 고통 지점은 중복 피해 다듬음. → ch01 |
| **철학/동기** | B L111–400(논거 + 고통 지점) | A L70–125 (M1) | 철학 깊이는 A(Mermaid). 실무 동기는 B. → ch01 |
| **컬렉션/반복** | B L2549–2672(컬렉션 다루기) | A L1195–1337(LINQ, M10) | 기본 반복은 ch05.2. LINQ 비교는 ch12 A. B의 고급 반복은 ch12로 이동. |

---

## 장별 추정 줄 수

| 장 | 본문 | 하위 장 | 합계 |
|---------|------|-------------|-------|
| ch00 서문 | 30 | — | 30 |
| ch01 소개·동기 | 380 | — | 380 |
| ch02 시작하기 | 170 | ch02.1 키워드 (400) | 570 |
| ch03 기본 제공 타입 | 280 | ch03.1 불변성 (136) | 416 |
| ch04 제어 흐름 | 280 | — | 280 |
| ch05 자료 구조 | 380 | ch05.1 생성자 (210) + ch05.2 컬렉션 (390) | 980 |
| ch06 열거형·패턴 매칭 | 320 | ch06.1 완전 매칭·null (300) | 620 |
| ch07 소유권 | 330 | ch07.1 메모리 안전 (220) | 550 |
| ch08 크레이트·모듈 | 340 | ch08.1 패키지 관리 (235) | 575 |
| ch09 에러 처리 | 350 | ch09.1 모범 사례 (80) | 430 |
| ch10 트레잇·제네릭 | 380 | ch10.1 제네릭 (170) + ch10.2 상속 (175) | 725 |
| ch11 From/Into | 120 | — | 120 |
| ch12 클로저·이터레이터 | 300 | — | 300 |
| ch13 동시성 | 260 | — | 260 |
| ch14 Unsafe·FFI | 120 | — | 120 |
| ch15 사례 연구 | 400 | ch15.1 패턴·크레이트 (400) + ch15.2 도입 (390) | 1,190 |
| ch16 모범 사례 | 340 | ch16.1 학습 경로 (100) | 440 |
| **합계** | | | **~7,986** |

**원본 합계 대비 감소:** 8,384 → 중복 제거 후 고유 내용 ~5,800 + 신규 ~120(ch11 연결, ch14 신규) ≈ **병합 결과 약 5,920줄**, 16개 장 + 14개 하위 장에 분산.

---

## 보존한 C# 특화 콘텐츠

| 콘텐츠 | 출처 | 장 | 중요성 |
|---------|--------|---------|----------------|
| 빠른 참조 표 | B L93–110 | ch01 | 한눈에 C#→Rust 매핑 |
| 키워드 참조(400줄) | B L842–1244 | ch02.1 | C# 키워드 → Rust 포괄 매핑 |
| 진짜 불변 vs 레코드 | A L577–712 | ch03.1 | C# `record`는 진짜 불변이 아님 |
| Mermaid 다이어그램 13개 | A 여러 곳 | 여러 장 | 개념 시각 비교 |
| LINQ vs 이터레이터 | A L1195–1337 | ch12 | LINQ 메서드마다 Rust 대응 |
| DI → 트레잇 주입 | A L2430–2478 | ch15.2 | IServiceCollection → 제네릭 생성자 |
| EF → SQLx 매핑 | A L2514–2555 | ch15.2 | DbContext → sqlx::query_as! |
| IConfiguration → config | A L2556–2595 | ch15.2 | appsettings.json → config 크레이트 |
| 필수 크레이트 매핑 | A L1744–1946 | ch15.1 | C# 라이브러리마다 Rust 크레이트 대응 |
| 리포지토리 패턴 | A L1506–1625 | ch15.1 | IRepository → 트레잇 + async_trait |
| 빌더 패턴 | A L1626–1743 | ch15.1 | C# 빌더 → 소비 self 빌더 |
| 스레드 안전 보장 | A L1947–2204 | ch13 | 관례 → 타입 시스템 강제 |
| 마이그레이션 의사결정 트리 | A L2850–2883 | ch16 | 도입 결정용 Mermaid 흐름도 |
| 성능 벤치마크 | A L2709–2830 | ch16 | 관리 vs 네이티브 성능 데이터 |
| 팀 도입 일정 | A L2596–2708 | ch15.2 | 월별 배포 계획 |
