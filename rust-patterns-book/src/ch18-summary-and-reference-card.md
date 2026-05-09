<a id="quick-reference-card"></a>

## 빠른 참조 카드

<a id="pattern-decision-guide"></a>

### 패턴 선택 가이드

```text
원시 타입에 타입 안전이 필요?
└── 뉴타입 패턴 (3장)

컴파일 타임에 상태를 강제하고 싶음?
└── 타입 상태 패턴 (3장)

런타임 데이터 없이 "태그"만?
└── PhantomData (4장)

Rc/Arc 순환 참조를 끊어야 함?
└── Weak<T> / sync::Weak<T> (9장)

바쁜 대기 없이 조건 대기?
└── Condvar + Mutex (6장)

"N가지 중 하나" 타입?
├── 닫힌 집합 → Enum
├── 열린 집합, 핫 패스 → 제네릭
├── 열린 집합, 콜드 패스 → dyn Trait
└── 완전 타입 불명 → Any + TypeId (2장)

스레드 간 공유 상태?
├── 단순 카운터/플래그 → 원자적 연산
├── 짧은 임계 구역 → Mutex
├── 읽기 많음 → RwLock
├── 한 번만 지연 초기화 → OnceLock / LazyLock (6장)
└── 복잡한 상태 → 액터 + 채널

계산 병렬화?
├── 컬렉션 처리 → rayon::par_iter
├── 백그라운드 작업 → thread::spawn
└── 지역 데이터 빌림 → thread::scope

async I/O나 동시 네트워킹?
├── 기본 → tokio + async/await (16장)
└── 고급(stream, 미들웨어) → Async Rust Training 참고

에러 처리?
├── 라이브러리 → thiserror (#[derive(Error)])
└── 애플리케이션 → anyhow (Result<T>)

값 이동을 막아야 함?
└── Pin<T> (9장) — Future, 자기 참조 타입에 필요
```

<a id="trait-bounds-cheat-sheet"></a>

### 트레잇 바운드 치트 시트

| 바운드 | 의미 |
|-------|---------|
| `T: Clone` | 복제 가능 |
| `T: Send` | 다른 스레드로 이동 가능 |
| `T: Sync` | `&T`를 스레드 간 공유 가능 |
| `T: 'static` | 비정적 참조 없음 |
| `T: Sized` | 컴파일 타임에 크기 알려짐 (기본) |
| `T: ?Sized` | 크기가 없을 수 있음 (`[T]`, `dyn Trait`) |
| `T: Unpin` | 고정 후 이동해도 안전 |
| `T: Default` | 기본값 있음 |
| `T: Into<U>` | `U`로 변환 가능 |
| `T: AsRef<U>` | `&U`로 빌릴 수 있음 |
| `T: Deref<Target = U>` | 자동으로 `&U`로 역참조 |
| `F: Fn(A) -> B` | 호출 가능, 상태는 불변 빌림 |
| `F: FnMut(A) -> B` | 호출 가능, 상태 변경 가능 |
| `F: FnOnce(A) -> B` | 한 번만 호출, 상태 소비 가능 |

<a id="lifetime-elision-rules"></a>

### 라이프타임 생략 규칙

컴파일러가 세 경우에 라이프타임을 자동 삽입합니다:

```rust
// 규칙 1: 참조 매개변수마다 각각의 라이프타임
// fn foo(x: &str, y: &str)  →  fn foo<'a, 'b>(x: &'a str, y: &'b str)

// 규칙 2: 입력 라이프타임이 정확히 하나면 출력에도 그걸 씀
// fn foo(x: &str) -> &str   →  fn foo<'a>(x: &'a str) -> &'a str

// 규칙 3: &self 또는 &mut self가 있으면 그 라이프타임을 출력에 사용
// fn foo(&self, x: &str) -> &str  →  fn foo<'a>(&'a self, x: &str) -> &'a str
```

**명시적으로 써야 할 때**:
- 참조 입력이 여럿이고 참조를 반환할 때(어떤 입력과 묶일지 추론 불가)
- 참조를 담는 필드: `struct Ref<'a> { data: &'a str }`
- 빌린 참조 없이 쓰려면 `'static` 바운드

<a id="common-derive-traits"></a>

### 자주 쓰는 Derive 트레잇

```rust
#[derive(
    Debug,          // {:?} 포맷
    Clone,          // .clone()
    Copy,           // 암시적 복사(단순 타입만)
    PartialEq, Eq,  // == 비교
    PartialOrd, Ord, // < > 비교 + 정렬
    Hash,           // HashMap/HashSet 키
    Default,        // Type::default()
)]
struct MyType { /* ... */ }
```

<a id="module-visibility-quick-reference"></a>

### 모듈 가시성 빠른 참조

```text
pub           → 어디서나
pub(crate)    → 크레이트 안
pub(super)    → 부모 모듈
pub(in path)  → 특정 경로 안
(없음)        → 현재 모듈 + 자식만
```

<a id="further-reading"></a>

### 더 읽을 거리

| 자료 | 이유 |
|----------|-----|
| [Rust Design Patterns](https://rust-unofficial.github.io/patterns/) | 관용 패턴과 안티패턴 목록 |
| [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) | 공개 API를 다듬는 공식 체크리스트 |
| [Rust Atomics and Locks](https://marabos.nl/atomics/) | Mara Bos의 동시성 프리미티브 심화 |
| [The Rustonomicon](https://doc.rust-lang.org/nomicon/) | Official unsafe Rust 및 어두운 구석 |
| [Error Handling in Rust](https://blog.burntsushi.net/rust-error-handling/) | Andrew Gallant의 에러 처리 가이드 |
| [Jon Gjengset — Crust of Rust series](https://www.youtube.com/playlist?list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa) | 이터레이터, 라이프타임, 채널 등 심화 |
| [Effective Rust](https://www.lurklurk.org/effective-rust/) | 코드 개선 35가지 |

***

*Rust Patterns & Engineering How-Tos 끝*
