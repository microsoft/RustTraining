<a id="variables-and-mutability"></a>
## 변수와 가변성

> **이 절에서 배울 내용:** Rust의 변수 선언과 가변성 모델을 C#의 `var`/`const`와 비교하고, 기본 타입 대응 관계, 중요한 `String` vs `&str` 구분, 타입 추론, 그리고 Rust가 C#과 다르게 형변환과 변환을 다루는 방식을 배웁니다.
>
> **난이도:** 🟢 입문

<a id="c-variable-declaration"></a>
### C# 변수 선언
```csharp
// C# - 변수는 기본적으로 가변이다
int count = 0;           // 가변
count = 5;               // ✅ 가능

readonly int maxSize = 100;  // 초기화 이후 불변
// maxSize = 200;        // ❌ 컴파일 에러

const int BUFFER_SIZE = 1024; // 컴파일 타임 상수
```

<a id="rust-variable-declaration"></a>
### Rust 변수 선언
```rust
// Rust - 변수는 기본적으로 불변이다
let count = 0;           // 기본은 불변
// count = 5;            // ❌ 컴파일 에러: immutable 변수에는 다시 대입할 수 없다

let mut count = 0;       // 명시적으로 가변
count = 5;               // ✅ 가능

const BUFFER_SIZE: usize = 1024; // 컴파일 타임 상수
```

<a id="key-mental-shift-for-c-developers"></a>
### C# 개발자가 먼저 가져야 할 사고 전환
```rust
// let은 기본적으로 readonly라고 생각하면 이해가 쉽다
let name = "John";       // C#으로 치면: readonly string name = "John";
let mut age = 30;        // C#으로 치면: int age = 30;

// 변수 섀도잉 (Rust에 특유한 개념)
let spaces = "   ";      // String
let spaces = spaces.len(); // 이제 숫자(usize)
// 이것은 mutation이 아니라 새 변수를 다시 바인딩한 것이다
```

<a id="practical-example-counter"></a>
### 실전 예제: 카운터
```csharp
// C# 버전
public class Counter
{
    private int value = 0;
    
    public void Increment()
    {
        value++;  // 상태 변경
    }
    
    public int GetValue() => value;
}
```

```rust
// Rust 버전
pub struct Counter {
    value: i32,  // 기본은 private
}

impl Counter {
    pub fn new() -> Counter {
        Counter { value: 0 }
    }
    
    pub fn increment(&mut self) {  // 상태 변경에는 &mut가 필요
        self.value += 1;
    }
    
    pub fn get_value(&self) -> i32 {
        self.value
    }
}
```

***

<a id="data-types-comparison"></a>
## 데이터 타입 비교

<a id="primitive-types"></a>
### 기본 타입

| C# 타입 | Rust 타입 | 크기 | 범위 |
|---------|-----------|------|-------|
| `byte` | `u8` | 8비트 | 0 ~ 255 |
| `sbyte` | `i8` | 8비트 | -128 ~ 127 |
| `short` | `i16` | 16비트 | -32,768 ~ 32,767 |
| `ushort` | `u16` | 16비트 | 0 ~ 65,535 |
| `int` | `i32` | 32비트 | -2³¹ ~ 2³¹-1 |
| `uint` | `u32` | 32비트 | 0 ~ 2³²-1 |
| `long` | `i64` | 64비트 | -2⁶³ ~ 2⁶³-1 |
| `ulong` | `u64` | 64비트 | 0 ~ 2⁶⁴-1 |
| `float` | `f32` | 32비트 | IEEE 754 |
| `double` | `f64` | 64비트 | IEEE 754 |
| `bool` | `bool` | 1비트 | true/false |
| `char` | `char` | 32비트 | 유니코드 스칼라 값 |

<a id="size-types-important"></a>
### 크기 타입 (`usize`)은 중요하다
```csharp
// C# - int는 항상 32비트
int arrayIndex = 0;
long fileSize = file.Length;
```

```rust
// Rust - 크기 타입은 포인터 크기(32비트 또는 64비트)에 맞춘다
let array_index: usize = 0;      // C의 size_t와 비슷
let file_size: u64 = file.len(); // 명시적 64비트
```

<a id="type-inference"></a>
### 타입 추론
```csharp
// C# - var 키워드
var name = "John";        // string
var count = 42;           // int
var price = 29.99;        // double
```

```rust
// Rust - 자동 타입 추론
let name = "John";        // &str (string slice)
let count = 42;           // i32 (기본 정수 타입)
let price = 29.99;        // f64 (기본 부동소수점 타입)

// 명시적 타입 표기
let count: u32 = 42;
let price: f32 = 29.99;
```

<a id="arrays-and-collections-overview"></a>
### 배열과 컬렉션 개요
```csharp
// C# - 참조 타입이며 힙에 할당
int[] numbers = new int[5];        // 고정 크기
List<int> list = new List<int>();  // 가변 크기
```

```rust
// Rust - 상황에 따라 선택지가 여러 개다
let numbers: [i32; 5] = [1, 2, 3, 4, 5];  // 스택 배열, 고정 크기
let mut list: Vec<i32> = Vec::new();      // 힙 벡터, 가변 크기
```

***

<a id="string-types-string-vs-str"></a>
## 문자열 타입: String vs &str

이 부분은 C# 개발자가 가장 자주 헷갈리는 개념 중 하나이므로 천천히 나눠서 보겠습니다.

<a id="c-string-handling"></a>
### C# 문자열 다루기
```csharp
// C# - 단순한 문자열 모델
string name = "John";                // 문자열 리터럴
string greeting = "Hello, " + name;  // 문자열 결합
string upper = name.ToUpper();       // 메서드 호출
```

<a id="rust-string-types"></a>
### Rust 문자열 타입
```rust
// Rust - 대표적인 문자열 타입은 두 가지다

// 1. &str (string slice) - C#의 ReadOnlySpan<char>에 가까운 개념
let name: &str = "John";        // 문자열 리터럴 (불변, 대여됨)

// 2. String - 소유하는 문자열
let mut greeting = String::new();      // 빈 문자열
greeting.push_str("Hello, ");          // 덧붙이기
greeting.push_str(name);               // 덧붙이기

// 또는 바로 생성할 수도 있다
let greeting = String::from("Hello, John");
let greeting = "Hello, John".to_string();  // &str -> String 변환
```

<a id="when-to-use-which"></a>
### 언제 무엇을 써야 할까?

| 상황 | 사용 타입 | C#에서 비슷한 개념 |
|----------|-----|---------------|
| 문자열 리터럴 | `&str` | `string` 리터럴 |
| 함수 매개변수 (읽기 전용) | `&str` | `string` 또는 `ReadOnlySpan<char>` |
| 소유하고 수정하는 문자열 | `String` | `StringBuilder` |
| 소유권을 가진 문자열 반환 | `String` | `string` |

<a id="practical-examples"></a>
### 실전 예제
```rust
// 어떤 문자열 타입이든 받을 수 있는 함수
fn greet(name: &str) {  // String과 &str 둘 다 받을 수 있다
    println!("Hello, {}!", name);
}

fn main() {
    let literal = "John";                     // &str
    let owned = String::from("Jane");        // String
    
    greet(literal);                          // 가능
    greet(&owned);                           // 가능 (String을 &str로 빌림)
    greet("Bob");                            // 가능
}

// 소유권 있는 문자열을 반환하는 함수
fn create_greeting(name: &str) -> String {
    format!("Hello, {}!", name)  // format! 매크로는 String을 반환
}
```

<a id="think-of-it-this-way"></a>
### C# 개발자라면 이렇게 이해해도 된다
```rust
// &str은 ReadOnlySpan<char>처럼 문자열 데이터를 가리키는 뷰
// String은 내가 소유하고 수정할 수 있는 문자열 버퍼

let borrowed: &str = "I don't own this data";
let owned: String = String::from("I own this data");

// 둘 사이 변환
let owned_copy: String = borrowed.to_string();  // 복사해서 소유
let borrowed_view: &str = &owned;               // 빌려서 뷰 생성
```

***

<a id="printing-and-string-formatting"></a>
## 출력과 문자열 포매팅

C# 개발자는 `Console.WriteLine`과 문자열 보간(`$""`)에 많이 의존합니다. Rust의 포매팅 시스템도 그에 못지않게 강력하지만, 매크로와 포맷 지정자를 사용한다는 차이가 있습니다.

<a id="basic-output"></a>
### 기본 출력
```csharp
// C# 출력
Console.Write("no newline");
Console.WriteLine("with newline");
Console.Error.WriteLine("to stderr");

// 문자열 보간 (C# 6+)
string name = "Alice";
int age = 30;
Console.WriteLine($"{name} is {age} years old");
```

```rust
// Rust 출력 - 전부 매크로다 (!에 주목)
print!("no newline");                // -> stdout, 개행 없음
println!("with newline");            // -> stdout + 개행
eprint!("to stderr");                // -> stderr, 개행 없음
eprintln!("to stderr with newline"); // -> stderr + 개행

// 문자열 포매팅 (C#의 $"..."와 비슷)
let name = "Alice";
let age = 30;
println!("{name} is {age} years old");     // inline 변수 캡처 (Rust 1.58+)
println!("{} is {} years old", name, age); // 위치 인수

// format!은 출력 대신 String을 반환한다
let msg = format!("{name} is {age} years old");
```

<a id="format-specifiers"></a>
### 포맷 지정자
```csharp
// C# 포맷 지정자
Console.WriteLine($"{price:F2}");         // 고정 소수점: 29.99
Console.WriteLine($"{count:D5}");         // 0 채움 정수: 00042
Console.WriteLine($"{value,10}");         // 오른쪽 정렬, 폭 10
Console.WriteLine($"{value,-10}");        // 왼쪽 정렬, 폭 10
Console.WriteLine($"{hex:X}");            // 16진수: FF
Console.WriteLine($"{ratio:P1}");         // 퍼센트: 85.0%
```

```rust
// Rust 포맷 지정자
println!("{price:.2}");             // 소수 둘째 자리까지: 29.99
println!("{count:05}");             // 0 채움, 폭 5: 00042
println!("{value:>10}");            // 오른쪽 정렬, 폭 10
println!("{value:<10}");            // 왼쪽 정렬, 폭 10
println!("{value:^10}");            // 가운데 정렬, 폭 10
println!("{hex:#X}");               // 접두사 포함 16진수: 0xFF
println!("{hex:08X}");              // 0 채움 16진수: 000000FF
println!("{bits:#010b}");           // 접두사 포함 2진수: 0b00001010
println!("{big}", big = 1_000_000); // 이름 있는 인수
```

<a id="debug-vs-display-printing"></a>
### `Debug` vs `Display` 출력
```rust
// {:?}  - Debug trait (개발자용, derive 가능)
// {:#?} - 보기 좋게 줄바꿈된 Debug
// {}    - Display trait (사용자용, 직접 구현해야 함)

#[derive(Debug)] // Debug 출력 자동 생성
struct Point { x: f64, y: f64 }

let p = Point { x: 1.5, y: 2.7 };

println!("{:?}", p);   // Point { x: 1.5, y: 2.7 } - compact debug
println!("{:#?}", p);  // Point {                    - pretty debug
                        //     x: 1.5,
                        //     y: 2.7,
                        // }
// println!("{}", p);  // ❌ ERROR: Point는 Display를 구현하지 않음

// 사용자 친화적 출력을 위해 Display 구현:
use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
println!("{}", p);    // (1.5, 2.7) - 사용자 친화적 출력
```

```csharp
// C#으로 비유하면:
// {:?}  ~= object.GetType().ToString() 또는 디버거 덤프
// {}    ~= object.ToString()
// C#에서는 ToString()을 override하고, Rust에서는 Display를 구현한다
```

<a id="quick-reference-printing"></a>
### 빠른 비교표

| C# | Rust | 출력 의미 |
|----|------|--------|
| `Console.WriteLine(x)` | `println!("{x}")` | Display 포매팅 |
| `$"{x}"` (보간) | `format!("{x}")` | `String` 반환 |
| `x.ToString()` | `x.to_string()` | `Display` trait 필요 |
| `ToString()` 재정의 | `impl Display` | 사용자 대상 출력 |
| 디버거 보기 | `{:?}` 또는 `dbg!(x)` | 개발자 대상 출력 |
| `String.Format("{0:F2}", x)` | `format!("{x:.2}")` | 포맷된 `String` |
| `Console.Error.WriteLine` | `eprintln!()` | stderr 출력 |

***

<a id="type-casting-and-conversions"></a>
## 형변환과 변환

C#에는 암시적 변환, 명시적 캐스트 `(int)x`, `Convert.To*()`가 있습니다. Rust는 훨씬 더 엄격해서 숫자형 암시적 변환이 없습니다.

<a id="numeric-conversions"></a>
### 숫자 변환
```csharp
// C# - 암시적 변환과 명시적 변환
int small = 42;
long big = small;              // 암시적 확장: 가능
double d = small;              // 암시적 확장: 가능
int truncated = (int)3.14;     // 명시적 축소: 3
byte b = (byte)300;            // 조용히 overflow: 44

// 안전한 변환
if (int.TryParse("42", out int parsed)) { /* ... */ }
```

```rust
// Rust - 모든 숫자 변환은 명시적이다
let small: i32 = 42;
let big: i64 = small as i64;          // 확장도 'as'로 명시
let d: f64 = small as f64;            // 정수 -> 실수도 명시
let truncated: i32 = 3.14_f64 as i32; // 축소: 3 (버림)
let b: u8 = 300_u16 as u8;            // overflow: 44로 wrap (C# unchecked와 비슷)

// TryFrom을 이용한 안전한 변환
use std::convert::TryFrom;
let safe: Result<u8, _> = u8::try_from(300_u16); // Err - 범위 초과
let ok: Result<u8, _>   = u8::try_from(42_u16);  // Ok(42)

// 문자열 파싱 - bool + out 파라미터가 아니라 Result를 반환
let parsed: Result<i32, _> = "42".parse::<i32>();   // Ok(42)
let bad: Result<i32, _>    = "abc".parse::<i32>();  // Err(ParseIntError)

// turbofish 문법과 함께:
let n = "42".parse::<f64>().unwrap(); // 42.0
```

<a id="string-conversions"></a>
### 문자열 변환
```csharp
// C#
int n = 42;
string s = n.ToString();          // "42"
string formatted = $"{n:X}";
int back = int.Parse(s);          // 42 또는 예외 발생
bool ok = int.TryParse(s, out int result);
```

```rust
// Rust - to_string()은 Display 기반, parse()는 FromStr 기반
let n: i32 = 42;
let s: String = n.to_string();            // "42" (Display trait 사용)
let formatted = format!("{n:X}");         // "2A"
let back: i32 = s.parse().unwrap();       // 42 또는 panic
let result: Result<i32, _> = s.parse();   // Ok(42) - 안전한 버전

// &str <-> String 변환 (Rust에서 가장 흔한 변환)
let owned: String = "hello".to_string();    // &str -> String
let owned2: String = String::from("hello"); // &str -> String (동등)
let borrowed: &str = &owned;                // String -> &str (무료, 단순 대여)
```

<a id="reference-conversions-no-inheritance-casting"></a>
### 참조 변환 (상속 캐스팅은 없다!)
```csharp
// C# - upcasting과 downcasting
Animal a = new Dog();              // Upcast (암시적)
Dog d = (Dog)a;                    // Downcast (명시적, 실패 가능)
if (a is Dog dog) { /* ... */ }    // 패턴 매칭으로 안전한 downcast
```

```rust
// Rust - 상속이 없으므로 upcasting/downcasting도 없다
// 다형성에는 trait object를 사용:
let animal: Box<dyn Animal> = Box::new(Dog);

// "Downcasting"이 필요하면 Any trait를 써야 한다 (드문 경우):
use std::any::Any;
if let Some(dog) = animal_any.downcast_ref::<Dog>() {
    // dog 사용
}
// 실전에서는 downcasting 대신 enum을 더 자주 사용한다:
enum Animal {
    Dog(Dog),
    Cat(Cat),
}
match animal {
    Animal::Dog(d) => { /* d 사용 */ }
    Animal::Cat(c) => { /* c 사용 */ }
}
```

<a id="quick-reference-conversions"></a>
### 빠른 비교표

| C# | Rust | 비고 |
|----|------|-------|
| `(int)x` | `x as i32` | 잘라내기/래핑 캐스트 |
| 암시적 확장 변환 | `as`를 반드시 사용 | 숫자형 암시적 변환이 없음 |
| `Convert.ToInt32(x)` | `i32::try_from(x)` | 안전하며 `Result` 반환 |
| `int.Parse(s)` | `s.parse::<i32>().unwrap()` | 실패 시 panic |
| `int.TryParse(s, out n)` | `s.parse::<i32>()` | `Result<i32, _>` 반환 |
| `(Dog)animal` | 제공되지 않음 | enum 또는 `Any` 사용 |
| `as Dog` / `is Dog` | `downcast_ref::<Dog>()` | `Any` 기반, 가능하면 enum 선호 |

***

<a id="comments-and-documentation"></a>
## 주석과 문서화

<a id="regular-comments"></a>
### 일반 주석
```csharp
// C# 주석
// 한 줄 주석
/* 여러 줄
   주석 */

/// <summary>
/// XML 문서 주석
/// </summary>
/// <param name="name">사용자 이름</param>
/// <returns>인사말 문자열</returns>
public string Greet(string name)
{
    return $"Hello, {name}!";
}
```

```rust
// Rust 주석
// 한 줄 주석
/* 여러 줄
   주석 */

/// 문서 주석 (C#의 ///와 비슷)
/// 이 함수는 이름을 받아 인사말을 반환합니다.
///
/// # Arguments
///
/// * `name` - 문자열 슬라이스 형태의 사용자 이름
///
/// # Returns
///
/// 인사말을 담은 `String`
///
/// # Examples
///
/// ```
/// let greeting = greet("Alice");
/// assert_eq!(greeting, "Hello, Alice!");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

<a id="documentation-generation"></a>
### 문서 생성
```bash
# 문서 생성 (C#의 XML 문서와 비슷한 역할)
cargo doc --open

# 문서 테스트 실행
cargo test --doc
```

---

<a id="exercises"></a>
## 연습문제

<details>
<summary><strong>🏋️ 연습문제: 타입 안전한 온도 변환</strong> (클릭해서 펼치기)</summary>

다음 조건을 만족하는 Rust 프로그램을 작성해 보세요.
1. 섭씨 절대영도(`-273.15`)를 나타내는 `const`를 선언한다.
2. 수행된 변환 횟수를 세는 `static` 카운터를 선언한다. (`AtomicU32` 사용)
3. 절대영도보다 낮은 온도는 `f64::NAN`을 반환해 거부하는 `celsius_to_fahrenheit(c: f64) -> f64` 함수를 작성한다.
4. 문자열 `"98.6"`을 `f64`로 파싱한 뒤 변환하면서 shadowing을 보여준다.

<details>
<summary>🔑 해답</summary>

```rust
use std::sync::atomic::{AtomicU32, Ordering};

const ABSOLUTE_ZERO_C: f64 = -273.15;
static CONVERSION_COUNT: AtomicU32 = AtomicU32::new(0);

fn celsius_to_fahrenheit(c: f64) -> f64 {
    if c < ABSOLUTE_ZERO_C {
        return f64::NAN;
    }
    CONVERSION_COUNT.fetch_add(1, Ordering::Relaxed);
    c * 9.0 / 5.0 + 32.0
}

fn main() {
    let temp = "98.6";                     // &str
    let temp: f64 = temp.parse().unwrap(); // f64로 shadowing
    let temp = celsius_to_fahrenheit(temp); // 화씨 값으로 shadowing
    println!("{temp:.1}°F");
    println!("Conversions: {}", CONVERSION_COUNT.load(Ordering::Relaxed));
}
```

</details>
</details>

***