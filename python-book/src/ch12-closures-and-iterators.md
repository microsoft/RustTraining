<a id="rust-closures-vs-python-lambdas"></a>
## Rust 클로저 vs Python 람다

> **이 장에서 배울 것:** 한 줄짜리 람다를 넘어서는 여러 줄 클로저, `Fn`/`FnMut`/`FnOnce` 캡처 의미론,
> 리스트 컴프리헨션과 이터레이터 체인의 대응, `map`/`filter`/`fold`, 그리고 `macro_rules!` 기초.
>
> **난이도:** 🟡 중급

### Python 클로저와 람다
```python
# Python — 람다는 한 표현식만 가질 수 있는 익명 함수다
double = lambda x: x * 2
result = double(5)  # 10

# 완전한 클로저는 바깥 스코프의 변수를 캡처한다:
def make_adder(n):
    def adder(x):
        return x + n    # 바깥 스코프의 `n`을 캡처
    return adder

add_5 = make_adder(5)
print(add_5(10))  # 15

# 고차 함수:
numbers = [1, 2, 3, 4, 5]
doubled = list(map(lambda x: x * 2, numbers))
evens = list(filter(lambda x: x % 2 == 0, numbers))
```

### Rust 클로저
```rust
// Rust — 클로저는 |args| body 문법을 사용한다
let double = |x: i32| x * 2;
let result = double(5);  // 10

// 클로저는 바깥 스코프의 변수를 캡처할 수 있다:
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n    // `move`는 `n`의 소유권을 클로저로 옮긴다
}

let add_5 = make_adder(5);
println!("{}", add_5(10));  // 15

// 이터레이터와 함께 쓰는 고차 함수:
let numbers = vec![1, 2, 3, 4, 5];
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).copied().collect();
```

### 클로저 문법 비교
```text
Python:                              Rust:
─────────                            ─────
lambda x: x * 2                      |x| x * 2
lambda x, y: x + y                   |x, y| x + y
lambda: 42                           || 42

# 여러 줄
def f(x):                            |x| {
    y = x * 2                            let y = x * 2;
    return y + 1                         y + 1
                                      }
```

### 클로저 캡처 — Rust가 다른 점
```python
# Python — 클로저는 참조로 캡처한다 (late binding 주의!)
funcs = [lambda: i for i in range(3)]
print([f() for f in funcs])  # [2, 2, 2] — 놀랍게도 모두 같은 `i`를 본다

# 기본 인자 트릭으로 수정:
funcs = [lambda i=i: i for i in range(3)]
print([f() for f in funcs])  # [0, 1, 2]
```

```rust
// Rust — 클로저는 기대한 대로 캡처한다 (late-binding 함정 없음)
let funcs: Vec<Box<dyn Fn() -> i32>> = (0..3)
    .map(|i| Box::new(move || i) as Box<dyn Fn() -> i32>)
    .collect();

let results: Vec<i32> = funcs.iter().map(|f| f()).collect();
println!("{:?}", results);  // [0, 1, 2] — 올바른 결과!

// `move`는 각 클로저마다 `i`의 복사본을 캡처한다 — late binding 놀람이 없다.
```

### 세 가지 클로저 트레잇
```rust
// Rust 클로저는 아래 트레잇 중 하나 이상을 구현한다:

// Fn — 여러 번 호출 가능, 캡처한 값을 변경하지 않음 (가장 흔함)
fn apply(f: impl Fn(i32) -> i32, x: i32) -> i32 { f(x) }

// FnMut — 여러 번 호출 가능, 캡처한 값을 변경할 수도 있음
fn apply_mut(mut f: impl FnMut(i32) -> i32, x: i32) -> i32 { f(x) }

// FnOnce — 한 번만 호출 가능 (캡처를 소비함)
fn apply_once(f: impl FnOnce() -> String) -> String { f() }

// Python에는 직접 대응되는 개념이 없다 — 클로저가 늘 Fn처럼 보인다.
// Rust에서는 컴파일러가 어떤 트레잇을 쓸지 자동으로 결정한다.
```

***

<a id="iterators-vs-generators"></a>
## 이터레이터 vs 제너레이터

### Python 제너레이터
```python
# Python — yield를 사용하는 제너레이터
def fibonacci():
    a, b = 0, 1
    while True:
        yield a
        a, b = b, a + b

# 지연 평가 — 값은 필요할 때 계산된다
fib = fibonacci()
first_10 = [next(fib) for _ in range(10)]

# 제너레이터 표현식 — 지연 리스트 컴프리헨션과 비슷
squares = (x ** 2 for x in range(1000000))  # 메모리 할당 없음
first_5 = [next(squares) for _ in range(5)]
```

### Rust 이터레이터
```rust
// Rust — Iterator 트레잇 (개념은 비슷하지만 문법은 다름)
struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.a;
        self.a = self.b;
        self.b = current + self.b;
        Some(current)
    }
}

// 지연 평가 — Python 제너레이터처럼 필요할 때 계산된다
let first_10: Vec<u64> = Fibonacci::new().take(10).collect();

// 이터레이터 체인 — 제너레이터 표현식과 비슷
let squares: Vec<u64> = (0..1_000_000u64).map(|x| x * x).take(5).collect();
```

***

## 컴프리헨션 vs 이터레이터 체인

이 절에서는 Python의 컴프리헨션 문법이 Rust의 이터레이터 체인으로 어떻게 대응되는지 살펴봅니다.

### 리스트 컴프리헨션 → map/filter/collect
```python
# Python 컴프리헨션:
squares = [x ** 2 for x in range(10)]
evens = [x for x in range(20) if x % 2 == 0]
names = [user.name for user in users if user.active]
pairs = [(x, y) for x in range(3) for y in range(3)]
flat = [item for sublist in nested for item in sublist]
```

```mermaid
flowchart LR
    A["입력\n[1,2,3,4,5]"] -->|.iter\(\)| B["이터레이터"]
    B -->|.filter\(\|x\| x%2==0\)| C["[2, 4]"]
    C -->|.map\(\|x\| x*x\)| D["[4, 16]"]
    D -->|.collect\(\)| E["Vec&lt;i32&gt;\n[4, 16]"]
    style A fill:#ffeeba
    style E fill:#d4edda
```

> **핵심 포인트**: Rust 이터레이터는 지연 평가됩니다. `.collect()`가 호출되기 전에는 아무 일도 일어나지 않습니다. Python의 제너레이터도 비슷하지만, 리스트 컴프리헨션은 즉시 평가됩니다.

```rust
// Rust 이터레이터 체인:
let squares: Vec<i32> = (0..10).map(|x| x * x).collect();
let evens: Vec<i32> = (0..20).filter(|x| x % 2 == 0).collect();
let names: Vec<&str> = users.iter()
    .filter(|u| u.active)
    .map(|u| u.name.as_str())
    .collect();
let pairs: Vec<(i32, i32)> = (0..3)
    .flat_map(|x| (0..3).map(move |y| (x, y)))
    .collect();
let flat: Vec<i32> = nested.iter()
    .flat_map(|sublist| sublist.iter().copied())
    .collect();
```

### dict 컴프리헨션 → HashMap으로 collect
```python
# Python
word_lengths = {word: len(word) for word in words}
inverted = {v: k for k, v in mapping.items()}
```

```rust
// Rust
let word_lengths: HashMap<&str, usize> = words.iter()
    .map(|w| (*w, w.len()))
    .collect();
let inverted: HashMap<&V, &K> = mapping.iter()
    .map(|(k, v)| (v, k))
    .collect();
```

### set 컴프리헨션 → HashSet으로 collect
```python
# Python
unique_lengths = {len(word) for word in words}
```

```rust
// Rust
let unique_lengths: HashSet<usize> = words.iter()
    .map(|w| w.len())
    .collect();
```

### 자주 쓰는 이터레이터 메서드

| Python | Rust | 비고 |
|--------|------|------|
| `map(f, iter)` | `.map(f)` | 각 요소를 변환 |
| `filter(f, iter)` | `.filter(f)` | 조건에 맞는 요소만 유지 |
| `sum(iter)` | `.sum()` | 모든 요소 합산 |
| `min(iter)` / `max(iter)` | `.min()` / `.max()` | `Option` 반환 |
| `any(f(x) for x in iter)` | `.any(f)` | 하나라도 참이면 `true` |
| `all(f(x) for x in iter)` | `.all(f)` | 모두 참이면 `true` |
| `enumerate(iter)` | `.enumerate()` | 인덱스 + 값 |
| `zip(a, b)` | `a.zip(b)` | 요소를 쌍으로 묶음 |
| `len(list)` | `.count()` (소비함!) 또는 `.len()` | 요소 개수 |
| `list(reversed(x))` | `.rev()` | 역순 순회 |
| `itertools.chain(a, b)` | `a.chain(b)` | 이터레이터 이어 붙이기 |
| `next(iter)` | `.next()` | 다음 요소 가져오기 |
| `next(iter, default)` | `.next().unwrap_or(default)` | 기본값 포함 |
| `list(iter)` | `.collect::<Vec<_>>()` | 컬렉션으로 구체화 |
| `sorted(iter)` | collect 후 `.sort()` | 지연 정렬 이터레이터는 없음 |
| `functools.reduce(f, iter)` | `.fold(init, f)` 또는 `.reduce(f)` | 누적 계산 |

### 핵심 차이
```text
Python 이터레이터:                  Rust 이터레이터:
─────────────────                  ──────────────
- 기본적으로 지연 평가 (제너레이터) - 기본적으로 지연 평가 (모든 이터레이터 체인)
- yield로 제너레이터 생성          - impl Iterator { fn next() }
- StopIteration으로 종료            - None으로 종료
- 한 번만 소비 가능                 - 한 번만 소비 가능
- 타입 안전성이 약함                - 완전한 타입 안전성
- 인터프리터라 약간 느림            - 제로 코스트 (컴파일 시 최적화)
```

***


<!-- ch12a: Macros -->
<a id="why-macros-exist-in-rust"></a>
## Rust에 매크로가 존재하는 이유

Python에는 매크로 시스템이 없습니다. 대신 데코레이터, 메타클래스, 런타임
인트로스펙션으로 메타프로그래밍을 합니다. Rust는 컴파일 타임 코드 생성을 위해 매크로를 사용합니다.

### Python 메타프로그래밍 vs Rust 매크로
```python
# Python — 메타프로그래밍에 데코레이터와 메타클래스를 사용
from dataclasses import dataclass
from functools import wraps

@dataclass              # import 시점에 __init__, __repr__, __eq__ 생성
class Point:
    x: float
    y: float

# 커스텀 데코레이터
def log_calls(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        print(f"Calling {func.__name__}")
        return func(*args, **kwargs)
    return wrapper

@log_calls
def process(data):
    return data.upper()
```

```rust
// Rust — 코드 생성을 위한 derive 매크로와 선언적 매크로
#[derive(Debug, Clone, PartialEq)]  // 컴파일 타임에 Debug, Clone, PartialEq 구현 생성
struct Point {
    x: f64,
    y: f64,
}

// 선언적 매크로 (템플릿과 비슷)
macro_rules! log_call {
    ($func_name:expr, $body:expr) => {
        println!("Calling {}", $func_name);
        $body
    };
}

fn process(data: &str) -> String {
    log_call!("process", data.to_uppercase())
}
```

### 자주 쓰는 내장 매크로
```rust
// 이 매크로들은 Rust 곳곳에서 자주 사용된다:

println!("Hello, {}!", name);           // 포매팅해서 출력
format!("Value: {}", x);               // 포매팅된 String 생성
vec![1, 2, 3];                          // Vec 생성
assert_eq!(2 + 2, 4);                  // 테스트 단언
assert!(value > 0, "must be positive"); // 불리언 단언
dbg!(expression);                       // 디버그 출력: 식과 값을 함께 출력
todo!();                                // 자리 표시자 — 컴파일되지만 실행되면 panic
unimplemented!();                       // 아직 구현 안 된 코드 표시
panic!("something went wrong");         // 메시지와 함께 중단 (raise RuntimeError와 비슷)

// 왜 함수가 아니라 매크로일까?
// - println!은 가변 인자를 받아야 한다 (Rust 함수로는 불가)
// - vec!는 어떤 타입과 길이에도 맞는 코드를 생성한다
// - assert_eq!는 비교한 원본 소스 코드를 알고 있다
// - dbg!는 파일 이름과 줄 번호를 알고 있다
```

## macro_rules!로 간단한 매크로 작성하기
```rust
// Python dict()에 대응하는 예
// Python: d = dict(a=1, b=2)
// Rust:   let d = hashmap!{ "a" => 1, "b" => 2 };

macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut map = std::collections::HashMap::new();
            $(map.insert($key, $value);)*
            map
        }
    };
}

let scores = hashmap! {
    "Alice" => 100,
    "Bob" => 85,
    "Charlie" => 90,
};
```

## derive 매크로 — 트레잇 자동 구현
```rust
// #[derive(...)]는 Python의 @dataclass 데코레이터에 가장 가까운 Rust 기능이다

// Python:
// @dataclass(frozen=True, order=True)
// class Student:
//     name: str
//     grade: int

// Rust:
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Student {
    name: String,
    grade: i32,
}

// 자주 쓰는 derive 매크로:
// Debug         → {:?} 포매팅 (__repr__와 비슷)
// Clone         → .clone() 깊은 복사
// Copy          → 암묵적 복사 (단순 타입만 가능)
// PartialEq, Eq → == 비교 (__eq__와 비슷)
// PartialOrd, Ord → <, >, 정렬 (__lt__ 등과 비슷)
// Hash          → HashMap 키로 사용 가능 (__hash__와 비슷)
// Default       → MyType::default() (인자 없는 __init__와 비슷)

// 크레이트가 제공하는 derive 매크로:
// Serialize, Deserialize (serde) → JSON/YAML/TOML 직렬화
//                                  (Python의 json.dumps/loads와 비슷하지만 타입 안전)
```

### Python 데코레이터 vs Rust Derive

| Python 데코레이터 | Rust Derive | 용도 |
|------------------|-------------|------|
| `@dataclass` | `#[derive(Debug, Clone, PartialEq)]` | 데이터 클래스 |
| `@dataclass(frozen=True)` | 기본적으로 불변 | 불변성 |
| `@dataclass(order=True)` | `#[derive(Ord, PartialOrd)]` | 비교/정렬 |
| `@total_ordering` | `#[derive(PartialOrd, Ord)]` | 완전한 순서 |
| JSON `json.dumps(obj.__dict__)` | `#[derive(Serialize)]` | 직렬화 |
| JSON `MyClass(**json.loads(s))` | `#[derive(Deserialize)]` | 역직렬화 |

---

<a id="exercises"></a>
## 연습문제

<details>
<summary><strong>🏋️ 연습문제: Derive와 커스텀 Debug</strong> (펼쳐서 보기)</summary>

**도전 과제**: 필드가 `name: String`, `email: String`, `password_hash: String`인 `User` 구조체를 만드세요. `Clone`과 `PartialEq`는 derive하고, `Debug`는 직접 구현해서 이름과 이메일은 출력하되 비밀번호는 `"***"`로 가리도록 하세요.

<details>
<summary>🔑 해답</summary>

```rust
use std::fmt;

#[derive(Clone, PartialEq)]
struct User {
    name: String,
    email: String,
    password_hash: String,
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("User")
            .field("name", &self.name)
            .field("email", &self.email)
            .field("password_hash", &"***")
            .finish()
    }
}

fn main() {
    let user = User {
        name: "Alice".into(),
        email: "alice@example.com".into(),
        password_hash: "a1b2c3d4e5f6".into(),
    };
    println!("{user:?}");
    // Output: User { name: "Alice", email: "alice@example.com", password_hash: "***" }
}
```

**핵심 포인트**: Python의 `__repr__`와 달리, Rust는 `Debug`를 손쉽게 derive할 수 있으면서도 민감한 필드가 있으면 직접 재정의할 수 있습니다. `print(user)`로 비밀 정보가 새기 쉬운 Python보다 더 안전한 패턴입니다.

</details>
</details>

***

