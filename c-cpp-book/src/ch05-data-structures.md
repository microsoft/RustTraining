<a id="rust-array-type"></a>
### Rust 배열 타입

> **이 장에서 배우는 것:** Rust의 핵심 자료구조인 배열, 튜플, 슬라이스, 문자열, 구조체, `Vec`, `HashMap`을 살펴봅니다. 밀도 있는 장이므로 특히 `String`과 `&str`의 차이, 그리고 구조체 사용법을 확실히 이해하는 데 집중하세요. 참조와 대여는 7장에서 더 깊게 다룹니다.

- 배열은 같은 타입의 원소를 고정된 개수만큼 담습니다.
    - 다른 Rust 타입과 마찬가지로 배열도 기본적으로는 불변이며, 변경하려면 `mut`를 사용해야 합니다.
    - 배열은 `[]`로 인덱싱하며 경계 검사가 수행됩니다. `len()` 메서드로 배열 길이를 얻을 수 있습니다.
```rust
    fn get_index(y: usize) -> usize {
        y + 1
    }
    
    fn main() {
        // 길이 3의 배열을 만들고 모든 값을 42로 초기화
        let a: [u8; 3] = [42; 3];
        // 대안 문법
        // let a = [42u8, 42u8, 42u8];
        for x in a {
            println!("{x}");
        }
        let y = get_index(a.len());
        // 아래 줄의 주석을 해제하면 패닉이 발생
        // println!("{}", a[y]);
    }
```

----
### Rust 배열 타입 계속
- 배열은 중첩할 수도 있습니다.
    - Rust에는 여러 내장 출력 포매터가 있습니다. 아래의 `:?`는 `debug` 포매터입니다. `:#?`는 `pretty print` 용도입니다. 이런 포매터는 타입별로 커스터마이즈할 수 있습니다.
```rust
    fn main() {
        let a = [
            [40, 0], // 중첩 배열 정의
            [41, 0],
            [42, 1],
        ];
        for x in a {
            println!("{x:?}");
        }
    }
```
----
<a id="rust-tuples"></a>
### Rust 튜플
- 튜플은 고정된 크기를 가지며, 서로 다른 타입의 값을 하나의 복합 타입으로 묶을 수 있습니다.
    - 각 원소는 위치 기반 인덱스(`.0`, `.1`, `.2`, ...)로 접근합니다. 빈 튜플 `()`는 unit 값이라 부르며, `void` 반환과 비슷한 역할을 합니다.
    - Rust는 튜플 구조 분해를 지원하므로 각 원소를 개별 변수에 쉽게 바인딩할 수 있습니다.
```rust
fn get_tuple() -> (u32, bool) {
    (42, true)
}

fn main() {
   let t: (u8, bool) = (42, true);
   let u: (u32, bool) = (43, false);
   println!("{}, {}", t.0, t.1);
   println!("{}, {}", u.0, u.1);
   let (num, flag) = get_tuple(); // 튜플 구조 분해
   println!("{num}, {flag}");
}
```

<a id="rust-references"></a>
### Rust 참조
- Rust의 참조는 대략적으로 C의 포인터와 비슷하지만 중요한 차이가 있습니다.
    - 어떤 시점에도 하나의 변수에 대해 읽기 전용(불변) 참조는 여러 개 가질 수 있습니다. 참조는 원래 변수의 스코프보다 오래 살 수 없습니다. 이 개념이 바로 **라이프타임**이며 뒤에서 자세히 다룹니다.
    - 쓰기 가능한(가변) 참조는 오직 하나만 허용되며, 다른 참조와 겹칠 수 없습니다.
```rust
fn main() {
    let mut a = 42;
    {
        let b = &a;
        let c = b;
        println!("{} {}", *b, *c); // 컴파일러가 자동 역참조를 해준다
        // b와 c가 아직 스코프 안에 있으므로 불법
        // let d = &mut a;
    }
    let d = &mut a; // OK: b와 c는 이미 스코프 밖
    *d = 43;
}
```

----
<a id="rust-slices"></a>
# Rust 슬라이스
- Rust 참조를 사용하면 배열의 부분 범위를 가리킬 수 있습니다.
    - 컴파일 타임에 길이가 고정되는 배열과 달리, 슬라이스는 임의 길이를 가질 수 있습니다. 내부적으로 슬라이스는 길이 정보와 시작 위치 포인터를 함께 가지는 "fat pointer"로 구현됩니다.
```rust
fn main() {
    let a = [40, 41, 42, 43];
    let b = &a[1..a.len()]; // 원본의 두 번째 원소부터 시작하는 슬라이스
    let c = &a[1..];        // 위와 동일
    let d = &a[..];         // &a[0..] 또는 &a[0..a.len()]와 동일
    println!("{b:?} {c:?} {d:?}");
}
```
----
<a id="rust-constants-and-statics"></a>
# Rust 상수와 static
- `const` 키워드는 상수 값을 정의할 때 사용합니다. 상수 값은 **컴파일 타임에** 계산되어 프로그램 안에 인라인됩니다.
- `static` 키워드는 C/C++의 전역 변수에 해당하는 값을 정의할 때 사용합니다. static 변수는 주소를 가지는 메모리 위치에 한 번 생성되며, 프로그램이 끝날 때까지 살아 있습니다.
```rust
const SECRET_OF_LIFE: u32 = 42;
static GLOBAL_VARIABLE: u32 = 2;
fn main() {
    println!("The secret of life is {}", SECRET_OF_LIFE);
    println!("Value of global variable is {GLOBAL_VARIABLE}")
}
```

----
<a id="rust-strings-string-vs-str"></a>
# Rust 문자열: String vs &str

- Rust에는 역할이 다른 문자열 타입이 **두 가지** 있습니다.
    - `String` - 소유권을 가지며, 힙에 할당되고, 길이가 늘어날 수 있습니다. C에서 `malloc`으로 만든 버퍼나 C++의 `std::string`과 비슷합니다.
    - `&str` - 대여된 가벼운 참조입니다. 길이 정보를 가진 C의 `const char*` 또는 C++의 `std::string_view`와 비슷하지만, Rust의 `&str`는 **라이프타임 검사를 받기 때문에 절대 댕글링되지 않습니다.**
    - C의 널 종료 문자열과 달리, Rust 문자열은 길이를 추적하고 항상 올바른 UTF-8입니다.

> **C++ 개발자를 위한 비교:** `String`은 대략 `std::string`, `&str`는 대략 `std::string_view`에 대응합니다. 하지만 `std::string_view`와 달리 `&str`는 borrow checker가 전체 라이프타임 동안 유효함을 보장합니다.

## String vs &str: 소유 vs 대여

> **프로덕션 패턴:** 실제 serde 코드에서 문자열 처리가 어떻게 보이는지는 [JSON handling: nlohmann::json → serde](ch17-2-avoiding-unchecked-indexing.md#json-handling-nlohmannjson--serde)를 참고하세요.

| **관점** | **C `char*`** | **C++ `std::string`** | **Rust `String`** | **Rust `&str`** |
|------------|--------------|----------------------|-------------------|----------------|
| **메모리** | 수동 (`malloc`/`free`) | 힙 할당, 버퍼 소유 | 힙 할당, 자동 해제 | 대여된 참조 (라이프타임 검사) |
| **가변성** | 포인터를 통해 항상 변경 가능 | 가변 | `mut`일 때 가변 | 항상 불변 |
| **크기 정보** | 없음 (`'\0'` 의존) | 길이와 capacity 추적 | 길이와 capacity 추적 | 길이 추적 (fat pointer) |
| **인코딩** | 미정 (보통 ASCII) | 미정 (보통 ASCII) | 항상 유효한 UTF-8 | 항상 유효한 UTF-8 |
| **널 종료** | 필요 | 필요 (`c_str()`) | 사용하지 않음 | 사용하지 않음 |

```rust
fn main() {
    // &str - 문자열 슬라이스 (대여됨, 불변, 보통 문자열 리터럴)
    let greeting: &str = "Hello";  // 읽기 전용 메모리를 가리킴

    // String - 소유권을 가지며, 힙에 있고, 길이를 늘릴 수 있음
    let mut owned = String::from(greeting);  // 데이터를 힙으로 복사
    owned.push_str(", World!");              // 문자열 확장
    owned.push('!');                         // 문자 하나 추가

    // String과 &str 사이 변환
    let slice: &str = &owned;               // String -> &str (할당 없음, 그냥 대여)
    let owned2: String = slice.to_string(); // &str -> String (할당 발생)
    let owned3: String = String::from(slice); // 위와 동일

    // String 이어 붙이기 (주의: +는 왼쪽 피연산자를 소비함)
    let hello = String::from("Hello");
    let world = String::from(", World!");
    let combined = hello + &world;  // hello는 move, world는 borrow
    // println!("{hello}");  // 컴파일되지 않음: hello는 이미 이동됨

    // move 문제를 피하려면 format! 사용
    let a = String::from("Hello");
    let b = String::from("World");
    let combined = format!("{a}, {b}!");  // a와 b 모두 소비되지 않음

    println!("{combined}");
}
```

## 왜 문자열은 `[]`로 인덱싱할 수 없을까
```rust
fn main() {
    let s = String::from("hello");
    // let c = s[0];  // 컴파일되지 않음! Rust 문자열은 UTF-8이지 바이트 배열이 아님

    // 안전한 대안들:
    let first_char = s.chars().next(); // Option<char>: Some('h')
    let as_bytes = s.as_bytes();       // &[u8]: UTF-8 바이트
    let substring = &s[0..1];          // &str: "h" (유효한 UTF-8 경계여야 함)

    println!("First char: {:?}", first_char);
    println!("Bytes: {:?}", &as_bytes[..5]);
}
```

## 연습문제: 문자열 조작

🟢 **Starter**
- 문자열에서 공백 기준 단어 수를 세는 `fn count_words(text: &str) -> usize` 함수를 작성하세요.
- 가장 긴 단어를 반환하는 `fn longest_word(text: &str) -> &str` 함수를 작성하세요. 힌트: 반환 타입이 왜 `String`이 아니라 `&str`인지, 라이프타임 관점에서 생각해보세요.

<details><summary>해답 (클릭하여 펼치기)</summary>

```rust
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn longest_word(text: &str) -> &str {
    text.split_whitespace()
        .max_by_key(|word| word.len())
        .unwrap_or("")
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog";
    println!("Word count: {}", count_words(text));      // 9
    println!("Longest word: {}", longest_word(text));   // "jumps"
}
```

</details>

<a id="rust-structs"></a>
# Rust 구조체
- `struct` 키워드는 사용자 정의 구조체 타입을 선언합니다.
    - `struct` 필드는 이름이 있을 수도 있고, 이름 없는 튜플 구조체일 수도 있습니다.
- Rust에는 C++ 같은 "데이터 상속" 개념이 없습니다.
```rust
fn main() {
    struct MyStruct {
        num: u32,
        is_secret_of_life: bool,
    }
    let x = MyStruct {
        num: 42,
        is_secret_of_life: true,
    };
    let y = MyStruct {
        num: x.num,
        is_secret_of_life: x.is_secret_of_life,
    };
    let z = MyStruct { num: x.num, ..x }; // ..는 나머지 필드를 채운다는 뜻
    println!("{} {} {}", x.num, y.is_secret_of_life, z.num);
}
```

## Rust 튜플 구조체
- Rust의 튜플 구조체는 튜플과 비슷하지만 별도의 타입을 정의한다는 점이 다릅니다. 각 필드에는 이름이 없습니다.
    - 튜플처럼 `.0`, `.1`, `.2`로 접근합니다. 튜플 구조체의 흔한 사용 사례는 기본 타입을 감싸 새 타입을 만드는 것입니다. **같은 기본 타입끼리 섞이는 실수를 막는 데 유용합니다.**
```rust
struct WeightInGrams(u32);
struct WeightInMilligrams(u32);
fn to_weight_in_grams(kilograms: u32) -> WeightInGrams {
    WeightInGrams(kilograms * 1000)
}

fn to_weight_in_milligrams(w: WeightInGrams) -> WeightInMilligrams {
    WeightInMilligrams(w.0 * 1000)
}

fn main() {
    let x = to_weight_in_grams(42);
    let y = to_weight_in_milligrams(x);
    // let z: WeightInGrams = x;  // 컴파일되지 않음: x는 이미 이동됨
    // let a: WeightInGrams = y;  // 컴파일되지 않음: 타입 불일치
}
```

**참고**: `#[derive(...)]` 속성은 구조체와 enum에 자주 쓰이는 트레잇 구현을 자동 생성합니다. 이후 장에서도 계속 보게 될 것입니다.
```rust
#[derive(Debug, Clone, PartialEq)]
struct Point { x: i32, y: i32 }

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{:?}", p); // Debug: #[derive(Debug)] 덕분에 가능
    let p2 = p.clone();  // Clone: #[derive(Clone)] 덕분에 가능
    assert_eq!(p, p2);   // PartialEq: #[derive(PartialEq)] 덕분에 가능
}
```
뒤에서 트레잇 시스템을 깊게 다루겠지만, `#[derive(Debug)]`는 너무 유용해서 거의 모든 `struct`와 `enum`에 붙이는 습관을 들여도 좋습니다.

<a id="rust-vec-type"></a>
# Rust Vec 타입
- `Vec<T>`는 동적으로 크기가 변하는 힙 버퍼를 구현합니다. C에서 수동으로 관리하는 `malloc`/`realloc` 배열이나, C++의 `std::vector`와 비슷합니다.
    - 고정 길이 배열과 달리 `Vec`은 런타임에 늘어나고 줄어들 수 있습니다.
    - `Vec`은 자신의 데이터를 소유하며, 메모리 할당과 해제를 자동으로 관리합니다.
- 자주 쓰는 연산: `push()`, `pop()`, `insert()`, `remove()`, `len()`, `capacity()`
```rust
fn main() {
    let mut v = Vec::new(); // 빈 벡터, 사용 문맥으로 타입 추론
    v.push(42);             // 끝에 원소 추가 - Vec<i32>
    v.push(43);
    
    // 안전한 순회 (권장)
    for x in &v {           // 벡터를 소비하지 않고 대여
        println!("{x}");
    }
    
    // 초기화 단축 문법
    let mut v2 = vec![1, 2, 3, 4, 5];
    let v3 = vec![0; 10];   // 0 열 개
    
    // 안전한 접근 방식 (인덱싱보다 권장)
    match v2.get(0) {
        Some(first) => println!("First: {first}"),
        None => println!("Empty vector"),
    }
    
    // 유용한 메서드들
    println!("Length: {}, Capacity: {}", v2.len(), v2.capacity());
    if let Some(last) = v2.pop() {
        println!("Popped: {last}");
    }
    
    // 위험: 직접 인덱싱 (패닉 가능)
    // println!("{}", v2[100]);  // 런타임에 패닉
}
```
> **프로덕션 패턴:** 실제 Rust 코드에서 `.get()`을 어떻게 쓰는지는 [Avoiding unchecked indexing](ch17-2-avoiding-unchecked-indexing.md#avoiding-unchecked-indexing)를 참고하세요.

<a id="rust-hashmap-type"></a>
# Rust HashMap 타입
- `HashMap`은 일반적인 `key -> value` 조회를 제공합니다. 즉 딕셔너리나 맵입니다.
```rust
fn main() {
    use std::collections::HashMap; // Vec와 달리 명시적 import 필요
    let mut map = HashMap::new();  // 빈 HashMap 생성
    map.insert(40, false);         // 타입은 int -> bool로 추론
    map.insert(41, false);
    map.insert(42, true);
    for (key, value) in map {
        println!("{key} {value}");
    }
    let map = HashMap::from([(40, false), (41, false), (42, true)]);
    if let Some(x) = map.get(&43) {
        println!("43 was mapped to {x:?}");
    } else {
        println!("No mapping was found for 43");
    }
    let x = map.get(&43).or(Some(&false)); // 키가 없을 때 기본값
    println!("{x:?}");
}
```

<a id="exercise-vec-and-hashmap"></a>
# 연습문제: Vec와 HashMap

🟢 **Starter**
- 몇 개의 항목이 들어 있는 `HashMap<u32, bool>`을 만드세요. 값은 `true`와 `false`가 섞이도록 하세요. 그리고 모든 원소를 순회하면서 키는 하나의 `Vec`에, 값은 다른 `Vec`에 넣어보세요.

<details><summary>해답 (클릭하여 펼치기)</summary>

```rust
use std::collections::HashMap;

fn main() {
    let map = HashMap::from([(1, true), (2, false), (3, true), (4, false)]);
    let mut keys = Vec::new();
    let mut values = Vec::new();
    for (k, v) in &map {
        keys.push(*k);
        values.push(*v);
    }
    println!("Keys:   {keys:?}");
    println!("Values: {values:?}");

    // 대안: unzip()을 사용하는 iterator 스타일
    let (keys2, values2): (Vec<u32>, Vec<bool>) = map.into_iter().unzip();
    println!("Keys (unzip):   {keys2:?}");
    println!("Values (unzip): {values2:?}");
}
```

</details>

---

<a id="c-references-vs-rust-references--key-differences"></a>
## 심화: C++ 참조 vs Rust 참조 - 핵심 차이

> **C++ 개발자를 위한 내용:** 많은 C++ 개발자는 Rust의 `&T`가 C++의 `T&`와 비슷하게 동작할 것이라 생각합니다. 겉보기엔 비슷하지만, 실제로는 중요한 차이가 많고 여기서 혼란이 자주 생깁니다. C 개발자는 이 절을 건너뛰고 [소유권과 대여](ch07-ownership-and-borrowing.md)로 가도 됩니다.

#### 1. Rvalue Reference와 Universal Reference가 없다

C++에서 `&&`는 문맥에 따라 두 의미를 가집니다.

```cpp
// C++: &&는 문맥에 따라 다른 의미
int&& rref = 42;            // Rvalue reference - 임시값에 바인딩
void process(Widget&& w);   // Rvalue reference - 호출자가 std::move 해야 함

// Universal (forwarding) reference - 템플릿 추론 문맥:
template<typename T>
void forward(T&& arg) {     // rvalue ref가 아니다! T& 또는 T&&로 추론됨
    inner(std::forward<T>(arg));  // Perfect forwarding
}
```

**Rust에는 이런 개념이 없습니다.** `&&`는 그저 논리 AND 연산자입니다.

```rust
// Rust: &&는 단순한 불리언 AND
let a = true && false; // false

// Rust에는 rvalue reference, universal reference, perfect forwarding이 없다.
// 대신:
//   - Copy가 아닌 타입은 기본이 move (std::move 필요 없음)
//   - generics + trait bounds가 universal reference 역할을 대체
//   - "임시값 바인딩" 구분이 따로 없음 - 값은 그냥 값

fn process(w: Widget) { }          // 값으로 받음 = 소유권 이전
fn process_ref(w: &Widget) { }     // 불변 대여
fn process_mut(w: &mut Widget) { } // 가변 대여 (배타적)
```

| C++ 개념 | Rust 대응 | 설명 |
|-------------|-----------------|-------|
| `T&` (lvalue ref) | `&T` 또는 `&mut T` | Rust는 공유 대여와 배타 대여를 분리한다 |
| `T&&` (rvalue ref) | 그냥 `T` | 값으로 받는다는 것은 소유권을 받는다는 뜻 |
| 템플릿 문맥의 `T&&` | `impl Trait` 또는 `<T: Trait>` | 제네릭이 forwarding을 대체 |
| `std::move(x)` | `x`를 그대로 사용 | move가 기본 동작 |
| `std::forward<T>(x)` | 별도 대응 없음 | universal reference 자체가 없다 |

#### 2. Move는 비트 단위 이동이다 - Move Constructor가 없다

C++에서 move는 *사용자 정의 연산*입니다. 즉 move constructor / move assignment가 호출됩니다. Rust에서 move는 항상 값의 **비트 단위 복사(memcpy)** 이며, 원본은 무효화됩니다.

```rust
// Rust move = 바이트를 복사하고 원본을 invalid 처리
let s1 = String::from("hello");
let s2 = s1; // s1의 바이트를 s2의 스택 슬롯으로 복사
             // s1은 이제 무효 - 컴파일러가 강제
// println!("{s1}"); // ❌ 컴파일 에러: value used after move
```

```cpp
// C++ move = move constructor 호출 (사용자 정의 가능)
std::string s1 = "hello";
std::string s2 = std::move(s1); // string의 move ctor 호출
// s1은 이제 "유효하지만 정의되지 않은 상태"의 좀비 객체
std::cout << s1; // 컴파일된다! 무엇이 출력될지는 구현 의존적
```

**결과적으로**
- Rust에는 Rule of Five가 없다
- 이동된 뒤의 "좀비 상태"가 없다. 컴파일러가 접근 자체를 막는다
- move에 `noexcept`를 고민할 필요도 없다. 비트 복사는 예외를 던지지 않는다

#### 3. Auto-Deref: 컴파일러가 간접 참조 계층을 꿰뚫어 본다

Rust는 `Deref` 트레잇을 통해 여러 겹의 포인터/래퍼를 자동 역참조합니다. C++에는 직접 대응하는 개념이 없습니다.

```rust
use std::sync::{Arc, Mutex};

// 중첩 래핑: Arc<Mutex<Vec<String>>>
let data = Arc::new(Mutex::new(vec!["hello".to_string()]));

// C++이라면 각 레이어마다 명시적 잠금과 역참조가 필요할 수 있다.
// Rust는 Arc → Mutex → MutexGuard → Vec 로 auto-deref 한다.
let guard = data.lock().unwrap(); // Arc가 Mutex로 auto-deref
let first: &str = &guard[0];      // MutexGuard→Vec, Vec[0], &String→&str
println!("First: {first}");

// 메서드 호출도 auto-deref
let boxed_string = Box::new(String::from("hello"));
println!("Length: {}", boxed_string.len());  // Box→String, then String::len()
```

**Deref coercion**은 함수 인자에도 적용됩니다. 컴파일러가 타입을 맞추기 위해 자동으로 역참조를 삽입합니다.

```rust
fn greet(name: &str) {
    println!("Hello, {name}");
}

fn main() {
    let owned = String::from("Alice");
    let boxed = Box::new(String::from("Bob"));
    let arced = std::sync::Arc::new(String::from("Carol"));

    greet(&owned);  // &String → &str
    greet(&boxed);  // &Box<String> → &String → &str
    greet(&arced);  // &Arc<String> → &String → &str
    greet("Dave");  // 이미 &str
}
```

**Deref 체인**: `x.method()`를 호출하면 Rust는 먼저 `T`, 그다음 `&T`, `&mut T`를 확인합니다. 일치하는 메서드가 없으면 `Deref`를 통해 대상 타입으로 역참조하고 같은 과정을 반복합니다. 그래서 `Box<Vec<T>>`도 마치 `Vec<T>`처럼 자연스럽게 쓸 수 있습니다. 함수 인자에서의 Deref coercion은 이와 관련 있지만 별도의 메커니즘입니다.

#### 4. Null 참조도 Optional 참조도 없다

```cpp
// C++: 참조는 null이 아니어야 하지만, 포인터는 null일 수 있고 둘 사이 경계가 흐리다
Widget& ref = *ptr;  // ptr이 null이면 UB
Widget* opt = nullptr;  // 포인터를 이용한 "선택적 참조"
```

```rust
// Rust: 참조는 항상 유효하다 - borrow checker가 보장
// 안전한 코드에서는 null 또는 댕글링 참조를 만들 수 없다
let r: &i32 = &42; // 항상 유효

// "선택적 참조"는 명시적이다
let opt: Option<&Widget> = None;
if let Some(w) = opt {
    w.do_something();
}
```

#### 5. 참조는 다시 꽂아 바꿔 끼우는(reseat) 별도 문법이 아니다

```cpp
// C++: 참조는 alias이므로 다시 바인딩할 수 없다
int a = 1, b = 2;
int& r = a;
r = b;  // r이 b를 가리키게 되는 것이 아니라, a에 b 값을 대입하는 것
// a는 이제 2, r은 여전히 a를 참조
```

```rust
// Rust: let 바인딩은 shadowing 가능, 참조는 일반 값처럼 다뤄진다
let a = 1;
let b = 2;
let r = &a;
// r = &b;   // ❌ 불변 변수에 재대입 불가
let r = &b;  // ✅ shadowing으로 새 바인딩 생성

// mut라면:
let mut r = &a;
r = &b;      // ✅ 이제 r은 b를 가리킨다
```

> **마음속 모델:** C++에서 참조는 한 객체에 영구적으로 연결된 alias입니다. Rust에서 참조는 라이프타임 보장을 가진 값이며, 일반 변수 바인딩 규칙을 따릅니다. 기본은 불변이고, `mut`일 때만 재바인딩할 수 있습니다.
