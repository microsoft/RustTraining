<a id="reference-card"></a>
# 레퍼런스 카드

> **빠른 참고:** 14개 이상의 correct-by-construction 패턴 전체를 선택 흐름도, 패턴 카탈로그, 조합 규칙, 크레이트 매핑, Curry-Howard 치트시트와 함께 정리했습니다.
>
> **교차 참조:** 모든 장 — 이 책 전체를 위한 조회표입니다.

<a id="quick-reference-correct-by-construction-patterns"></a>
## 빠른 참고: Correct-by-Construction 패턴

<a id="pattern-selection-guide"></a>
### 패턴 선택 가이드

```text
놓쳤을 때 버그가 치명적인가?
├── 예 → 타입으로 인코딩할 수 있는가?
│         ├── 예 → CORRECT-BY-CONSTRUCTION 사용
│         └── 아니오 → 런타임 검사 + 광범위한 테스트
└── 아니오 → 런타임 검사로 충분
```

<a id="pattern-catalogue"></a>
### 패턴 카탈로그

| # | 패턴 | 핵심 트레잇/타입 | 방지하는 것 | 런타임 비용 | 장 |
|---|------|-----------------|------------|:----------:|-----|
| 1 | Typed Commands | `trait IpmiCmd { type Response; }` | 잘못된 응답 타입 | 없음 | ch02 |
| 2 | Single-Use Types | `struct Nonce` (Clone/Copy 아님) | 논스/키 재사용 | 없음 | ch03 |
| 3 | Capability Tokens | `struct AdminToken { _private: () }` | 무단 접근 | 없음 | ch04 |
| 4 | Type-State | `Session<Active>` | 프로토콜 위반 | 없음 | ch05 |
| 5 | Dimensional Types | `struct Celsius(f64)` | 단위 혼동 | 없음 | ch06 |
| 6 | Validated Boundaries | `struct ValidFru` (TryFrom 경유) | 검증 없는 데이터 사용 | 한 번 파싱 | ch07 |
| 7 | Capability Mixins | `trait FanDiagMixin: HasSpi + HasI2c` | 버스 접근 누락 | 없음 | ch08 |
| 8 | Phantom Types | `Register<Width16>` | 폭/방향 불일치 | 없음 | ch09 |
| 9 | Sentinel → Option | `Option<u8>` (`0xFF` 아님) | 센티널을 값으로 쓰는 버그 | 없음 | ch11 |
| 10 | Sealed Traits | `trait Cmd: private::Sealed` | 외부에서의 부적절한 impl | 없음 | ch11 |
| 11 | Non-Exhaustive Enums | `#[non_exhaustive] enum Sku` | 조용한 match 폴백 | 없음 | ch11 |
| 12 | Typestate Builder | `DerBuilder<Set, Missing>` | 불완전한 생성 | 없음 | ch11 |
| 13 | FromStr Validation | `impl FromStr for DiagLevel` | 검증 없는 문자열 입력 | 한 번 파싱 | ch11 |
| 14 | Const-Generic Size | `RegisterBank<const N: usize>` | 버퍼 크기 불일치 | 없음 | ch11 |
| 15 | Safe `unsafe` Wrapper | `MmioRegion::read_u32()` | 검증 없는 MMIO/FFI | 없음 | ch11 |
| 16 | Async Type-State | `AsyncSession<Active>` | 비동기 프로토콜 위반 | 없음 | ch11 |
| 17 | Const Assertions | `SdrSensorId<const N: u8>` | 잘못된 컴파일 타임 ID | 없음 | ch11 |
| 18 | Session Types | `Chan<SendRequest>` | 채널 연산 순서 위반 | 없음 | ch11 |
| 19 | Pin Self-Referential | `Pin<Box<StreamParser>>` | 구조체 내부 댕글링 포인터 | 없음 | ch11 |
| 20 | RAII / Drop | `impl Drop for Session` | 모든 종료 경로에서 리소스 누수 | 없음 | ch11 |
| 21 | Error Type Hierarchy | `#[derive(Error)] enum DiagError` | 에러 삼키기 | 없음 | ch11 |
| 22 | `#[must_use]` | `#[must_use] struct Token` | 조용히 드롭되는 값 | 없음 | ch11 |

<a id="composition-rules"></a>
### 조합 규칙

```text
Capability Token + Type-State = 승인된 상태 전이
Typed Command + Dimensional Type = 물리 단위가 붙은 응답
Validated Boundary + Phantom Type = 검증된 설정 위의 타입화된 레지스터 접근
Capability Mixin + Typed Command = 버스를 아는 타입화된 연산
Single-Use Type + Type-State = 전이 시 소비하는 프로토콜
Sealed Trait + Typed Command = 닫힌, 건전한 명령 집합
Sentinel → Option + Validated Boundary = 깨끗한 parse-once 파이프라인
Typestate Builder + Capability Token = 완전한 생성에 대한 증명
FromStr + #[non_exhaustive] = 진화 가능하고 fail-fast한 enum 파싱
Const-Generic Size + Validated Boundary = 크기가 정해지고 검증된 프로토콜 버퍼
Safe unsafe Wrapper + Phantom Type = 타입화되고 안전한 MMIO 접근
Async Type-State + Capability Token = 승인된 비동기 전이
Session Types + Typed Command = 완전히 타입화된 요청–응답 채널
Pin + Type-State = 이동할 수 없는 자기 참조 상태 머신
RAII (Drop) + Type-State = 상태에 따른 정리 보장
Error Hierarchy + Validated Boundary = 완전한 처리가 가능한 타입화된 파싱 에러
#[must_use] + Single-Use Type = 무시하기 어렵고 재사용하기 어려운 토큰
```

<a id="anti-patterns-to-avoid"></a>
### 피해야 할 안티 패턴

| 안티 패턴 | 왜 문제인가 | 올바른 대안 |
|----------|------------|------------|
| `fn read_sensor() -> f64` | 단위 없음 — °C, °F, RPM일 수 있음 | `fn read_sensor() -> Celsius` |
| `fn encrypt(nonce: &[u8; 12])` | 논스 재사용 가능(빌림) | `fn encrypt(nonce: Nonce)` (이동) |
| `fn admin_op(is_admin: bool)` | 호출자가 `true`로 거짓말 가능 | `fn admin_op(_: &AdminToken)` |
| `fn send(session: &Session)` | 상태 보장 없음 | `fn send(session: &Session<Active>)` |
| `fn process(data: &[u8])` | 검증되지 않음 | `fn process(data: &ValidFru)` |
| 일시적 키에 `Clone` | 단일 사용 보장 무력화 | Clone 파생 금지 |
| `let vendor_id: u16 = 0xFFFF` | 센티널을 내부에 유지 | `let vendor_id: Option<u16> = None` |
| 폴백이 있는 `fn route(level: &str)` | 오타가 조용히 기본값 | `let level: DiagLevel = s.parse()?` |
| 필드 없이 `Builder::new().finish()` | 불완전한 객체 생성 | 타입 상태 빌더: `Set`일 때만 `finish()` |
| 고정 크기 HW 버퍼에 `let buf: Vec<u8>` | 크기는 런타임에만 검사 | `RegisterBank<4096>` (const 제네릭) |
| 곳곳에 원시 `unsafe { ptr::read(...) }` | UB 위험, 감사 불가 | `MmioRegion::read_u32()` 안전 래퍼 |
| `async fn transition(&mut self)` | 가변 빌림만으로는 상태 강제 불가 | `async fn transition(self) -> NextState` |
| 수동 호출 `fn cleanup()` | 조기 반환/패닉 시 누락 | `impl Drop` — 컴파일러가 호출 삽입 |
| `fn op() -> Result<T, String>` | 불투명한 에러, variant 매칭 불가 | `fn op() -> Result<T, DiagError>` enum |

<a id="mapping-to-a-diagnostics-codebase"></a>
### 진단 코드베이스에 매핑하기

| 모듈 | 적용 가능한 패턴 |
|---------------------|----------------------|
| `protocol_lib` | Typed commands, type-state 세션 |
| `thermal_diag` | Capability mixins, dimensional types |
| `accel_diag` | Validated boundaries, phantom 레지스터 |
| `network_diag` | Type-state(링크 트레이닝), capability tokens |
| `pci_topology` | Phantom types(레지스터 폭), 검증된 설정, sentinel → Option |
| `event_handler` | Single-use 감사 토큰, capability tokens, FromStr(Component) |
| `event_log` | Validated boundaries(SEL 레코드 파싱) |
| `compute_diag` | Dimensional types(온도, 주파수) |
| `memory_diag` | Validated boundaries(SPD 데이터), dimensional types |
| `switch_diag` | Type-state(포트 열거), phantom types |
| `config_loader` | FromStr(DiagLevel, FaultStatus, DiagAction) |
| `log_analyzer` | Validated boundaries(CompiledPatterns) |
| `diag_framework` | Typestate 빌더(DerBuilder), session types(오케스트레이터↔워커) |
| `topology_lib` | Const-generic 레지스터 뱅크, 안전한 MMIO 래퍼 |

<a id="curry-howard-cheat-sheet"></a>
### Curry-Howard 치트시트

| 논리 개념 | Rust 대응 | 예 |
|----------|----------|-----|
| 명제 | 타입 | `AdminToken` |
| 증명 | 그 타입의 값 | `let tok = authenticate()?;` |
| 함의 (A → B) | 함수 `fn(A) -> B` | `fn activate(AdminToken) -> Session<Active>` |
| 논리곱 (A ∧ B) | 튜플 `(A, B)` 또는 다중 인자 | `fn op(a: &AdminToken, b: &LinkTrained)` |
| 논리합 (A ∨ B) | `enum { A(A), B(B) }` 또는 `Result<A, B>` | `Result<Session<Active>, Error>` |
| 참 | `()` (unit 타입) | 항상 생성 가능 |
| 거짓 | `!` (never 타입) 또는 `enum Void {}` | 절대 생성 불가 |

---

