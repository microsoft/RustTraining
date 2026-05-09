<a id="avoiding-excessive-clone"></a>
## 과도한 `clone()` 피하기

> **이 장에서 배우는 것:** 왜 `.clone()`이 Rust에서 코드 냄새가 될 수 있는지, 불필요한 복사를 없애기 위해 소유권을 어떻게 재구성하는지, 그리고 어떤 패턴이 소유권 설계 문제를 드러내는 신호인지 살펴봅니다.

- C++에서 오면 `.clone()`이 "그냥 복사하면 되지" 같은 안전한 기본값처럼 느껴집니다. 하지만 과도한 clone은 소유권 문제를 가리고 성능을 해칩니다.
- **경험칙**: borrow checker를 만족시키기 위해 clone하고 있다면, 대개는 소유권 구조를 다시 짜야 합니다.

<a id="when-clone-is-wrong"></a>
### `clone()`이 잘못된 경우

```rust
// 나쁨: 단지 읽기만 하는 함수에 넘기려고 String을 clone한다
fn log_message(msg: String) {  // 불필요하게 소유권을 가져간다
    println!("[LOG] {}", msg);
}
let message = String::from("GPU test passed");
log_message(message.clone());  // 낭비: String 전체를 새로 할당한다
log_message(message);           // 원본도 소비된다 — 앞선 clone은 무의미했다
```

```rust
// 좋음: 대여를 받는다 — 할당이 전혀 없다
fn log_message(msg: &str) {    // 소유하지 않고 빌려 쓴다
    println!("[LOG] {}", msg);
}
let message = String::from("GPU test passed");
log_message(&message);          // clone 없음, 할당 없음
log_message(&message);          // 다시 호출 가능 — message가 소비되지 않음
```

<a id="real-example-returning-str-instead-of-cloning"></a>
### 실제 예: clone 대신 `&str` 반환하기
```rust
// 예시: healthcheck.rs — 빌린 뷰를 반환한다, 할당 없음
pub fn serial_or_unknown(&self) -> &str {
    self.serial.as_deref().unwrap_or(UNKNOWN_VALUE)
}

pub fn model_or_unknown(&self) -> &str {
    self.model.as_deref().unwrap_or(UNKNOWN_VALUE)
}
```
C++에서 대응되는 코드는 `const std::string&`나 `std::string_view`를 반환할 것입니다. 하지만 C++에서는 둘 다 라이프타임 검사를 받지 않습니다. Rust에서는 borrow checker가 반환된 `&str`가 `self`보다 오래 살 수 없음을 보장합니다.

<a id="real-example-static-string-slices--no-heap-at-all"></a>
### 실제 예: static 문자열 슬라이스 — 힙 자체가 없음
```rust
// 예시: healthcheck.rs — 컴파일 시점 문자열 테이블
const HBM_SCREEN_RECIPES: &[&str] = &[
    "hbm_ds_ntd", "hbm_ds_ntd_gfx", "hbm_dt_ntd", "hbm_dt_ntd_gfx",
    "hbm_burnin_8h", "hbm_burnin_24h",
];
```
C++에서는 보통 이런 코드를 `std::vector<std::string>`로 작성합니다(첫 사용 시 힙 할당). Rust의 `&'static [&'static str]`는 읽기 전용 메모리에 놓이므로 런타임 비용이 0입니다.

<a id="when-clone-is-appropriate"></a>
### `clone()`이 적절한 경우

| **상황** | **왜 clone이 괜찮은가** | **예시** |
|--------------|--------------------|-----------|
| 스레드용 `Arc::clone()` | 참조 카운트만 증가시킴(약 1ns), 데이터를 복사하지 않음 | `let flag = stop_flag.clone();` |
| 데이터를 생성한 스레드로 옮길 때 | 스레드가 자기 사본을 가져야 함 | `let ctx = ctx.clone(); thread::spawn(move \|\| { ... })` |
| `&self` 필드에서 값을 꺼낼 때 | 빌린 값에서 move할 수 없음 | `String`을 반환할 때 `self.name.clone()` |
| `Option`에 감싼 작은 `Copy` 타입 | `.clone()`보다 `.copied()`가 더 명확함 | `Option<&u32>` → `Option<u32>`에 `opt.get(0).copied()` |

<a id="real-example-arcclone-for-thread-sharing"></a>
### 실제 예: 스레드 공유를 위한 `Arc::clone`
```rust
// 예시: workload.rs — Arc::clone은 싸다(참조 카운트 증가만 수행)
let stop_flag = Arc::new(AtomicBool::new(false));
let stop_flag_clone = stop_flag.clone();   // 약 1ns, 데이터 복사 없음
let ctx_clone = ctx.clone();               // 스레드로 move하기 위해 context를 clone

let sensor_handle = thread::spawn(move || {
    // ...stop_flag_clone과 ctx_clone 사용
});
```

<a id="checklist-should-i-clone"></a>
### 체크리스트: 지금 clone해야 할까?
1. **`String` / `T` 대신 `&str` / `&T`를 받을 수 있는가?** → clone하지 말고 대여하세요.
2. **소유자가 둘 필요 없도록 구조를 바꿀 수 있는가?** → 참조로 전달하거나 스코프를 활용하세요.
3. **이게 `Arc::clone()`인가?** → 괜찮습니다. O(1)입니다.
4. **데이터를 스레드/클로저로 move하고 있는가?** → clone이 필요합니다.
5. **뜨거운 루프 안에서 clone하고 있는가?** → 프로파일링하고, 대여나 `Cow<T>`를 고려하세요.

----

<a id="cowa-t-clone-on-write--borrow-when-you-can-clone-when-you-must"></a>
## `Cow<'a, T>`: Clone-on-Write — 가능하면 대여하고, 꼭 필요할 때만 clone하기

`Cow`(Clone on Write)는 **빌린 참조** 또는 **소유한 값** 중 하나를 담는 enum입니다. Rust에서 "가능하면 할당을 피하고, 수정이 필요할 때만 할당하라"를 표현하는 도구입니다. C++에는 정확히 같은 개념이 없고, 가장 가까운 비유는 어떤 함수가 때로는 `const std::string&`를, 때로는 `std::string`을 반환하는 경우입니다.

<a id="why-cow-exists"></a>
### 왜 `Cow`가 필요한가

```rust
// Cow가 없으면 둘 중 하나를 골라야 한다: 항상 빌리거나, 항상 clone하거나
fn normalize(s: &str) -> String {          // 항상 할당한다!
    if s.contains(' ') {
        s.replace(' ', "_")               // 새 String (할당 필요)
    } else {
        s.to_string()                     // 불필요한 할당!
    }
}

// Cow를 쓰면 바뀌지 않을 때는 빌리고, 수정될 때만 할당한다
use std::borrow::Cow;

fn normalize(s: &str) -> Cow<'_, str> {
    if s.contains(' ') {
        Cow::Owned(s.replace(' ', "_"))    // 할당함(수정 필요)
    } else {
        Cow::Borrowed(s)                   // 할당 없음(그대로 통과)
    }
}
```

<a id="how-cow-works"></a>
### `Cow`는 어떻게 동작하는가

```rust
use std::borrow::Cow;

// Cow<'a, str>는 개념적으로 대략 다음과 같다:
// enum Cow<'a, str> {
//     Borrowed(&'a str),     // 비용 없는 참조
//     Owned(String),          // 힙에 할당된 소유 값
// }

fn greet(name: &str) -> Cow<'_, str> {
    if name.is_empty() {
        Cow::Borrowed("stranger")         // static 문자열 — 할당 없음
    } else if name.starts_with(' ') {
        Cow::Owned(name.trim().to_string()) // 수정되므로 할당 필요
    } else {
        Cow::Borrowed(name)               // 그대로 통과 — 할당 없음
    }
}

fn main() {
    let g1 = greet("Alice");     // Cow::Borrowed("Alice")
    let g2 = greet("");          // Cow::Borrowed("stranger")
    let g3 = greet(" Bob ");     // Cow::Owned("Bob")
    
    // Cow<str>는 Deref<Target = str>를 구현하므로 &str처럼 사용할 수 있다
    println!("Hello, {g1}!");    // 가능함 — Cow가 자동으로 &str로 deref된다
    println!("Hello, {g2}!");
    println!("Hello, {g3}!");
}
```

<a id="real-world-use-case-config-value-normalization"></a>
### 실전 사용 예: 설정 값 정규화

```rust
use std::borrow::Cow;

/// SKU 이름을 정규화한다: 공백 제거 후 소문자화.
/// 이미 정규화되어 있으면 Cow::Borrowed를 반환한다(할당 없음).
fn normalize_sku(sku: &str) -> Cow<'_, str> {
    let trimmed = sku.trim();
    if trimmed == sku && sku.chars().all(|c| c.is_lowercase() || !c.is_alphabetic()) {
        Cow::Borrowed(sku)   // 이미 정규화됨 — 할당 없음
    } else {
        Cow::Owned(trimmed.to_lowercase())  // 수정 필요 — 할당
    }
}

fn main() {
    let s1 = normalize_sku("server-x1");   // Borrowed — zero alloc
    let s2 = normalize_sku("  Server-X1 "); // Owned — must allocate
    println!("{s1}, {s2}"); // "server-x1, server-x1"
}
```

<a id="when-to-use-cow"></a>
### `Cow`를 언제 써야 하는가

| **상황** | **`Cow`를 쓸까?** |
|--------------|---------------|
| 함수가 대부분의 경우 입력을 그대로 반환함 | ✅ 예 — 불필요한 clone을 피할 수 있음 |
| 문자열 파싱/정규화(trim, lowercase, replace) | ✅ 예 — 입력이 이미 유효한 경우가 많음 |
| 항상 수정해서 모든 경로가 할당함 | ❌ 아니오 — 그냥 `String`을 반환하세요 |
| 단순 패스스루(절대 수정하지 않음) | ❌ 아니오 — 그냥 `&str`를 반환하세요 |
| 구조체에 오래 저장할 데이터 | ❌ 아니오 — 소유 타입인 `String`을 사용하세요 |

> **C++ 비교**: `Cow<str>`는 `std::variant<std::string_view, std::string>`를 반환하는 함수와 비슷합니다.
> 다만 Rust에서는 자동 deref가 되고, 값에 접근하기 위한 보일러플레이트도 없습니다.

----

<a id="weakt-breaking-reference-cycles--rusts-weak_ptr"></a>
## `Weak<T>`: 참조 사이클 끊기 — Rust의 `weak_ptr`

`Weak<T>`는 C++의 `std::weak_ptr<T>`에 해당하는 Rust 타입입니다. `Rc<T>` 또는 `Arc<T>` 값을 비소유 참조로 가리킵니다. `Weak` 참조가 남아 있어도 실제 값은 해제될 수 있고, 이때 `upgrade()`를 호출하면 `None`이 반환됩니다.

<a id="why-weak-exists"></a>
### 왜 `Weak`가 필요한가

`Rc<T>`와 `Arc<T>`는 두 값이 서로를 가리키면 참조 사이클을 만듭니다. 그러면 어느 쪽도 refcount 0이 되지 않아서 drop되지 않습니다(메모리 누수). `Weak`는 이 사이클을 끊어 줍니다.

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: String,
    parent: RefCell<Weak<Node>>,      // Weak — 부모를 drop하지 못하게 막지 않음
    children: RefCell<Vec<Rc<Node>>>,  // Strong — 부모가 자식을 소유
}

impl Node {
    fn new(value: &str) -> Rc<Node> {
        Rc::new(Node {
            value: value.to_string(),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        })
    }

    fn add_child(parent: &Rc<Node>, child: &Rc<Node>) {
        // 자식은 부모에 대한 약한 참조를 가진다(사이클 없음)
        *child.parent.borrow_mut() = Rc::downgrade(parent);
        // 부모는 자식에 대한 강한 참조를 가진다
        parent.children.borrow_mut().push(Rc::clone(child));
    }
}

fn main() {
    let root = Node::new("root");
    let child = Node::new("child");
    Node::add_child(&root, &child);

    // upgrade()로 자식에서 부모 접근
    if let Some(parent) = child.parent.borrow().upgrade() {
        println!("Child's parent: {}", parent.value); // "root"
    }
    
    println!("Root strong count: {}", Rc::strong_count(&root));  // 1
    println!("Root weak count: {}", Rc::weak_count(&root));      // 1
}
```

<a id="c-comparison"></a>
### C++ 비교

```cpp
// C++ — shared_ptr 사이클을 끊기 위한 weak_ptr
struct Node {
    std::string value;
    std::weak_ptr<Node> parent;                  // Weak — 비소유
    std::vector<std::shared_ptr<Node>> children;  // Strong — 자식을 소유

    static auto create(const std::string& v) {
        return std::make_shared<Node>(Node{v, {}, {}});
    }
};

auto root = Node::create("root");
auto child = Node::create("child");
child->parent = root;          // weak_ptr 대입
root->children.push_back(child);

if (auto p = child->parent.lock()) {   // lock() → shared_ptr 또는 null
    std::cout << "Parent: " << p->value << std::endl;
}
```

| C++ | Rust | 설명 |
|-----|------|-------|
| `shared_ptr<T>` | `Rc<T>` (단일 스레드) / `Arc<T>` (멀티스레드) | 의미가 같다 |
| `weak_ptr<T>` | `Rc::downgrade()` / `Arc::downgrade()`로 얻는 `Weak<T>` | 의미가 같다 |
| `weak_ptr::lock()` → `shared_ptr` 또는 null | `Weak::upgrade()` → `Option<Rc<T>>` | 이미 drop되었으면 `None` |
| `shared_ptr::use_count()` | `Rc::strong_count()` | 의미가 같다 |

<a id="when-to-use-weak"></a>
### `Weak`를 언제 써야 하는가

| **상황** | **패턴** |
|--------------|-----------|
| 부모 ↔ 자식 트리 관계 | 부모는 `Rc<Child>`, 자식은 `Weak<Parent>` |
| Observer 패턴 / 이벤트 리스너 | 이벤트 소스는 `Weak<Observer>`, observer는 `Rc<Source>` |
| 해제를 막지 않는 캐시 | `HashMap<Key, Weak<Value>>` — 항목이 자연스럽게 stale해짐 |
| 그래프 구조에서 사이클 끊기 | 교차 링크는 `Weak`, 트리 간선은 `Rc`/`Arc` |

> **새 코드에서는 트리 구조에 `Rc/Weak`보다 arena 패턴(사례 연구 2)을 우선하세요.**
> `Vec<T>` + 인덱스가 더 단순하고 더 빠르며, 참조 카운팅 오버헤드도 없습니다.
> 동적 라이프타임을 가진 공유 소유권이 정말 필요할 때만 `Rc/Weak`를 쓰세요.

----

<a id="copy-vs-clone-partialeq-vs-eq--when-to-derive-what"></a>
## `Copy` vs `Clone`, `PartialEq` vs `Eq` — 무엇을 derive해야 할까?

- **`Copy` ≈ C++의 trivially copyable 타입(커스텀 copy ctor/dtor 없음)** 입니다. `int`, `enum`, 단순 POD 구조체 같은 타입은 컴파일러가 비트 단위 `memcpy`를 자동으로 생성합니다. Rust의 `Copy`도 같은 개념으로, `let b = a;` 대입 시 암시적 비트 복사가 일어나고 두 변수 모두 계속 유효합니다.
- **`Clone` ≈ C++의 copy constructor / `operator=` 깊은 복사** 입니다. 예를 들어 `std::vector` 멤버를 깊게 복사하는 커스텀 copy constructor가 있는 C++ 클래스라면, Rust에서 대응되는 개념은 `Clone` 구현입니다. 다만 Rust에서는 반드시 `.clone()`을 명시적으로 호출해야 합니다. 비싼 복사가 `=` 뒤에 숨지 않습니다.
- **핵심 차이**: C++에서는 trivial copy와 deep copy가 모두 같은 `=` 문법으로 암시적으로 일어납니다. Rust는 선택을 강제합니다. `Copy` 타입은 조용히 복사되고(싸다), `Copy`가 아닌 타입은 기본적으로 **move**되며, 비싼 복제가 필요하면 `.clone()`으로 명시적으로 요청해야 합니다.
- 비슷하게, C++의 `operator==`는 `a == a`가 항상 성립하는 타입(정수 등)과 그렇지 않은 타입(NaN을 가진 `float`)을 구분하지 않습니다. Rust는 이를 `PartialEq`와 `Eq`로 타입 시스템에 반영합니다.

<a id="copy-vs-clone"></a>
### `Copy` vs `Clone`

| | **Copy** | **Clone** |
|---|---------|----------|
| **동작 방식** | 비트 단위 memcpy(암시적) | 커스텀 로직(명시적 `.clone()`) |
| **언제 일어나는가** | 대입 시: `let b = a;` | `.clone()`을 호출할 때만 |
| **이후 상태** | `a`와 `b` 모두 유효 | `a`와 `b` 모두 유효 |
| **둘 다 없으면** | `let b = a;`가 `a`를 **move**함(`a`는 사라짐) | `let b = a;`가 `a`를 **move**함(`a`는 사라짐) |
| **허용 대상** | 힙 데이터를 갖지 않는 타입 | 모든 타입 |
| **C++ 비유** | trivially copyable / POD 타입(커스텀 copy ctor 없음) | 커스텀 copy constructor(깊은 복사) |

<a id="real-example-copy--simple-enums"></a>
### 실제 예: `Copy` — 단순 enum
```rust
// fan_diag/src/sensor.rs에서 발췌 — 모든 variant가 unit이고 1바이트에 들어간다
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum FanStatus {
    #[default]
    Normal,
    Low,
    High,
    Missing,
    Failed,
    Unknown,
}

let status = FanStatus::Normal;
let copy = status;   // 암시적 복사 — status는 여전히 유효
println!("{:?} {:?}", status, copy);  // 둘 다 사용 가능
```

<a id="real-example-copy--enum-with-integer-payloads"></a>
### 실제 예: `Copy` — 정수 payload를 가진 enum
```rust
// 예시: healthcheck.rs — u32 payload는 Copy이므로 enum 전체도 Copy가 될 수 있다
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthcheckStatus {
    Pass,
    ProgramError(u32),
    DmesgError(u32),
    RasError(u32),
    OtherError(u32),
    Unknown,
}
```

<a id="real-example-clone-only--struct-with-heap-data"></a>
### 실제 예: `Clone`만 가능 — 힙 데이터를 가진 구조체
```rust
// 예시: components.rs — String 때문에 Copy가 불가능하다
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FruData {
    pub technology: DeviceTechnology,
    pub physical_location: String,      // ← String: 힙 할당 타입이므로 Copy 불가
    pub expected: bool,
    pub removable: bool,
}
// let a = fru_data;   → MOVE (fru_data는 사라짐)
// let a = fru_data.clone();  → CLONE (fru_data는 계속 유효, 새 힙 할당 발생)
```

<a id="the-rule-can-it-be-copy"></a>
### 규칙: 이 타입은 `Copy`가 될 수 있는가?
```text
이 타입이 String, Vec, Box, HashMap,
Rc, Arc 또는 다른 힙 소유 타입을 포함하는가?
    YES → Clone만 가능 (Copy는 불가)
    NO  → Copy를 derive할 수 있음 (작다면 보통 하는 편이 좋음)
```

<a id="partialeq-vs-eq"></a>
### `PartialEq` vs `Eq`

| | **PartialEq** | **Eq** |
|---|--------------|-------|
| **무엇을 제공하나** | `==`와 `!=` 연산자 | 마커: "동등성이 반사적이다" |
| **반사적인가? (`a == a`)** | 보장되지 않음 | **보장됨** |
| **왜 중요한가** | `f32::NAN != f32::NAN` | `HashMap` 키는 **`Eq`가 필요함** |
| **언제 derive하나** | 거의 항상 | `f32`/`f64` 필드가 없을 때 |
| **C++ 비유** | `operator==` | 직접 대응 개념 없음(C++는 검사하지 않음) |

<a id="real-example-eq--used-as-hashmap-key"></a>
### 실제 예: `Eq` — `HashMap` 키로 사용
```rust
// hms_trap/src/cpu_handler.rs에서 발췌 — Hash에는 Eq가 필요하다
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CpuFaultType {
    InvalidFaultType,
    CpuCperFatalErr,
    CpuLpddr5UceErr,
    CpuC2CUceFatalErr,
    // ...
}
// 사용 예: HashMap<CpuFaultType, FaultHandler>
// HashMap 키는 Eq + Hash가 필요하다 — PartialEq만으로는 컴파일되지 않는다
```

<a id="real-example-no-eq-possible--type-contains-f32"></a>
### 실제 예: `Eq` 불가 — 타입에 `f32`가 포함됨
```rust
// 예시: types.rs — f32 때문에 Eq 불가
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TemperatureSensors {
    pub warning_threshold: Option<f32>,   // ← f32는 NaN ≠ NaN
    pub critical_threshold: Option<f32>,  // ← Eq를 derive할 수 없음
    pub sensor_names: Vec<String>,
}
// HashMap 키로 쓸 수 없다. Eq를 derive할 수 없다.
// 이유: f32::NAN == f32::NAN 이 false라서 반사성을 깨뜨린다.
```

<a id="partialord-vs-ord"></a>
### `PartialOrd` vs `Ord`

| | **PartialOrd** | **Ord** |
|---|---------------|--------|
| **무엇을 제공하나** | `<`, `>`, `<=`, `>=` | `.sort()`, `BTreeMap` 키 |
| **전체 순서인가?** | 아니오(비교 불가능한 쌍이 있을 수 있음) | **예**(모든 쌍이 비교 가능) |
| **`f32`/`f64`는?** | PartialOrd만 가능(NaN이 순서를 깨뜨림) | Ord derive 불가 |

<a id="real-example-ord--severity-ranking"></a>
### 실제 예: `Ord` — 심각도 순위
```rust
// hms_trap/src/fault.rs에서 발췌 — variant 순서가 심각도를 정의한다
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FaultSeverity {
    Info,      // lowest  (discriminant 0)
    Warning,   //         (discriminant 1)
    Error,     //         (discriminant 2)
    Critical,  // highest (discriminant 3)
}
// FaultSeverity::Info < FaultSeverity::Critical → true
// 가능해지는 코드: if severity >= FaultSeverity::Error { escalate(); }
```

<a id="real-example-ord--diagnostic-levels-for-comparison"></a>
### 실제 예: `Ord` — 비교 가능한 진단 레벨
```rust
// 예시: orchestration.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum GpuDiagLevel {
    #[default]
    Quick,     // lowest
    Standard,
    Extended,
    Full,      // highest
}
// 가능해지는 코드: if requested_level >= GpuDiagLevel::Extended { run_extended_tests(); }
```

<a id="derive-decision-tree"></a>
### derive 결정 트리

```text
                        새로 만든 타입
                            │
                 String/Vec/Box를 포함하는가?
                      /              \
                    YES                NO
                     │                  │
               Clone만 가능       Clone + Copy
                     │                  │
               f32/f64 포함?       f32/f64 포함?
                /          \         /          \
              YES           NO     YES           NO
               │             │      │             │
         PartialEq만     PartialEq  PartialEq  PartialEq
                         + Eq       만         + Eq
                          │                      │
                    정렬이 필요한가?        정렬이 필요한가?
                      /       \               /       \
                    YES        NO            YES        NO
                     │          │              │          │
               PartialOrd    끝          PartialOrd    끝
               + Ord                     + Ord
                     │                        │
               맵 키로                  맵 키로
               쓸 것인가?              쓸 것인가?
                  │                        │
                + Hash                   + Hash
```

<a id="quick-reference-common-derive-combos-from-production-rust-code"></a>
### 빠른 참고: 프로덕션 Rust 코드에서 자주 보는 derive 조합

| **타입 범주** | **전형적인 derive** | **예시** |
|-------------------|--------------------|------------|
| 단순 상태 enum | `Copy, Clone, PartialEq, Eq, Default` | `FanStatus` |
| `HashMap` 키로 쓰는 enum | `Copy, Clone, PartialEq, Eq, Hash` | `CpuFaultType`, `SelComponent` |
| 정렬 가능한 심각도 enum | `Copy, Clone, PartialEq, Eq, PartialOrd, Ord` | `FaultSeverity`, `GpuDiagLevel` |
| 문자열을 담는 데이터 구조체 | `Clone, Debug, Serialize, Deserialize` | `FruData`, `OverallSummary` |
| 직렬화 가능한 설정 구조체 | `Clone, Debug, Default, Serialize, Deserialize` | `DiagConfig` |

----
