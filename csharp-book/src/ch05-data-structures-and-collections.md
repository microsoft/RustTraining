<a id="tuples-and-destructuring"></a>
## 튜플과 구조 분해

> **학습할 내용:** Rust 튜플과 C# `ValueTuple`의 차이, 배열과 슬라이스, 구조체와 클래스,
> 제로 비용 타입 안전성으로 도메인을 모델링하는 newtype 패턴, 그리고 구조 분해 문법.
>
> **난이도:** 🟢 입문

C#에는 `ValueTuple`이 있습니다(C# 7부터). Rust의 튜플은 비슷해 보이지만, 언어에 훨씬 더 깊게 통합되어 있습니다.

### C# 튜플
```csharp
// C# ValueTuple (C# 7+)
var point = (10, 20);                         // (int, int)
var named = (X: 10, Y: 20);                   // 이름 있는 요소
Console.WriteLine($"{named.X}, {named.Y}");

// 반환 타입으로 쓰는 튜플
public (int Quotient, int Remainder) Divide(int a, int b)
{
    return (a / b, a % b);
}

var (q, r) = Divide(10, 3);    // 구조 분해
Console.WriteLine($"{q} remainder {r}");

// 버리기
var (_, remainder) = Divide(10, 3);  // 몫은 무시
```

### Rust 튜플
```rust
// Rust 튜플 — 기본적으로 불변이며 이름 있는 요소를 지원하지 않음
let point = (10, 20);                // (i32, i32)
let point3d: (f64, f64, f64) = (1.0, 2.0, 3.0);

// 인덱스로 접근(0부터 시작)
println!("x={}, y={}", point.0, point.1);

// 반환 타입으로 쓰는 튜플
fn divide(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}

let (q, r) = divide(10, 3);       // 구조 분해
println!("{q} remainder {r}");

// _ 로 값 버리기
let (_, remainder) = divide(10, 3);

// 단위 타입 () — "비어 있는 튜플"(C#의 void와 비슷함)
fn greet() {          // 반환 타입은 암묵적으로 ()
    println!("hi");
}
```

### 핵심 차이

| 항목 | C# `ValueTuple` | Rust 튜플 |
|---------|-----------------|------------|
| 이름 있는 요소 | `(int X, int Y)` | 지원하지 않음 — 구조체 사용 |
| 최대 길이 | 약 8개(그 이상은 중첩) | 제한 없음(실용적으로는 약 12개) |
| 비교 | 자동 지원 | 12개 이하 요소의 튜플은 자동 지원 |
| 딕셔너리 키로 사용 | 가능 | 가능(요소들이 `Hash`를 구현하면) |
| 함수 반환값으로 사용 | 흔함 | 흔함 |
| 요소 변경 가능성 | 항상 변경 가능 | `let mut`일 때만 가능 |

### 튜플 구조체(Newtype)
```rust
// 일반 튜플이 충분히 설명적이지 않다면 튜플 구조체를 사용합니다.
struct Meters(f64);     // 단일 필드 "newtype" 래퍼
struct Celsius(f64);
struct Fahrenheit(f64);

// 컴파일러는 이들을 서로 다른 타입으로 취급합니다.
let distance = Meters(100.0);
let temp = Celsius(36.6);
// distance == temp;  // ❌ 오류: Meters와 Celsius는 비교할 수 없음

// newtype 패턴은 단위 혼동 버그를 컴파일 타임에 막아 줍니다!
// C#에서 같은 안전성을 얻으려면 완전한 class/struct를 만들어야 합니다.
```

```csharp
// C#에서 같은 개념을 표현하려면 보일러플레이트가 더 많습니다.
public readonly record struct Meters(double Value);
public readonly record struct Celsius(double Value);
// 서로 바꿔 쓸 수는 없지만, record는 Rust의 제로 비용 newtype보다 부담이 큽니다.
```

### Newtype 패턴 심화: 제로 비용으로 도메인 모델링하기

newtype는 단위 혼동을 막는 수준을 훨씬 넘어섭니다. Rust에서 newtype는 **비즈니스 규칙을 타입 시스템에 인코딩하는 핵심 도구**이며, C#에서 흔한 "guard clause"나 "validation class" 패턴을 대체합니다.

#### C# 검증 방식: 런타임 가드
```csharp
// C# — 검증은 매번 런타임에 수행됩니다.
public class UserService
{
    public User CreateUser(string email, int age)
    {
        if (string.IsNullOrWhiteSpace(email) || !email.Contains('@'))
            throw new ArgumentException("Invalid email");
        if (age < 0 || age > 150)
            throw new ArgumentException("Invalid age");

        return new User { Email = email, Age = age };
    }

    public void SendEmail(string email)
    {
        // 다시 검증해야 할까, 아니면 호출자를 믿어야 할까?
        if (!email.Contains('@')) throw new ArgumentException("Invalid email");
        // ...
    }
}
```

#### Rust Newtype 방식: 컴파일 타임 증명
```rust
/// 검증된 이메일 주소 — 타입 자체가 유효성의 증거다.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    /// Email을 만드는 유일한 방법 — 생성 시 한 번만 검증한다.
    pub fn new(raw: &str) -> Result<Self, &'static str> {
        if raw.contains('@') && raw.len() > 3 {
            Ok(Email(raw.to_lowercase()))
        } else {
            Err("invalid email format")
        }
    }

    /// 내부 값에 안전하게 접근
    pub fn as_str(&self) -> &str { &self.0 }
}

/// 검증된 나이 — 잘못된 값을 만들 수 없다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Age(u8);

impl Age {
    pub fn new(raw: u8) -> Result<Self, &'static str> {
        if raw <= 150 { Ok(Age(raw)) } else { Err("age out of range") }
    }
    pub fn value(&self) -> u8 { self.0 }
}

// 이제 함수는 "검증 완료된 타입"을 받습니다 — 재검증이 필요 없습니다!
fn create_user(email: Email, age: Age) -> User {
    // email은 반드시 유효합니다 — 이것이 타입 불변식입니다.
    User { email, age }
}

fn send_email(to: &Email) {
    // 검증이 필요 없습니다 — Email 타입이 유효성을 증명합니다.
    println!("Sending to: {}", to.as_str());
}
```

#### C# 개발자가 자주 쓰는 Newtype 활용 예

| C# 패턴 | Rust Newtype | 막아 주는 문제 |
|------------|-------------|------------------|
| UserId, Email 등에 `string` 사용 | `struct UserId(Uuid)` | 잘못된 문자열을 잘못된 매개변수에 넘기는 실수 |
| Port, Count, Index에 `int` 사용 | `struct Port(u16)` | Port와 Count를 서로 바꿔 쓰는 실수 |
| 곳곳에 guard clause 추가 | 생성자에서 한 번만 검증 | 중복 검증, 검증 누락 |
| USD, EUR에 `decimal` 사용 | `struct Usd(Decimal)` | USD와 EUR를 실수로 더하는 문제 |
| 의미가 다른 값에 `TimeSpan` 사용 | `struct Timeout(Duration)` | 연결 타임아웃을 요청 타임아웃 자리에 넘기는 실수 |

```rust
// 제로 비용: newtype는 내부 타입과 같은 어셈블리로 컴파일됩니다.
// 이 Rust 코드는:
struct UserId(u64);
fn lookup(id: UserId) -> Option<User> { /* ... */ }

// 다음과 동일한 기계어를 생성합니다:
fn lookup(id: u64) -> Option<User> { /* ... */ }
// 하지만 컴파일 타임 타입 안전성은 그대로 유지됩니다!
```

***

<a id="arrays-and-slices"></a>
## 배열과 슬라이스

배열, 슬라이스, 벡터의 차이를 이해하는 것은 매우 중요합니다.

### C# 배열
```csharp
// C# 배열
int[] numbers = new int[5];         // 고정 크기, 힙 할당
int[] initialized = { 1, 2, 3, 4, 5 }; // 배열 리터럴

// 접근
numbers[0] = 10;
int first = numbers[0];

// 길이
int length = numbers.Length;

// 매개변수로 전달되는 배열(참조 타입)
void ProcessArray(int[] array)
{
    array[0] = 99;  // 원본 수정
}
```

### Rust 배열, 슬라이스, 벡터
```rust
// 1. 배열 - 고정 크기, 스택 할당
let numbers: [i32; 5] = [1, 2, 3, 4, 5];  // 타입: [i32; 5]
let zeros = [0; 10];                       // 0이 10개

// 접근
let first = numbers[0];
// numbers[0] = 10;  // ❌ 오류: 배열은 기본적으로 불변

let mut mut_array = [1, 2, 3, 4, 5];
mut_array[0] = 10;  // ✅ mut이면 가능

// 2. 슬라이스 - 배열이나 벡터를 바라보는 뷰
let slice: &[i32] = &numbers[1..4];  // 요소 1, 2, 3
let all_slice: &[i32] = &numbers;    // 배열 전체를 슬라이스로

// 3. 벡터 - 동적 크기, 힙 할당(앞에서 다룸)
let mut vec = vec![1, 2, 3, 4, 5];
vec.push(6);  // 크기를 늘릴 수 있음
```

### 함수 매개변수로서의 슬라이스
```csharp
// C# - 배열에서만 동작하는 메서드
public void ProcessNumbers(int[] numbers)
{
    for (int i = 0; i < numbers.Length; i++)
    {
        Console.WriteLine(numbers[i]);
    }
}

// 배열에서만 사용 가능
ProcessNumbers(new int[] { 1, 2, 3 });
```

```rust
// Rust - 어떤 시퀀스와도 동작하는 함수
fn process_numbers(numbers: &[i32]) {  // 슬라이스 매개변수
    for (i, num) in numbers.iter().enumerate() {
        println!("Index {}: {}", i, num);
    }
}

fn main() {
    let array = [1, 2, 3, 4, 5];
    let vec = vec![1, 2, 3, 4, 5];
    
    // 같은 함수가 둘 다 처리합니다!
    process_numbers(&array);      // 배열을 슬라이스로
    process_numbers(&vec);        // 벡터를 슬라이스로
    process_numbers(&vec[1..4]);  // 부분 슬라이스
}
```

### 문자열 슬라이스(`&str`) 다시 보기
```rust
// String과 &str의 관계
fn string_slice_example() {
    let owned = String::from("Hello, World!");
    let slice: &str = &owned[0..5];      // "Hello"
    let slice2: &str = &owned[7..];      // "World!"
    
    println!("{}", slice);   // "Hello"
    println!("{}", slice2);  // "World!"
    
    // 어떤 문자열 타입이든 받을 수 있는 함수
    print_string("String literal");      // &str
    print_string(&owned);               // String을 &str로
    print_string(slice);                // &str 슬라이스
}

fn print_string(s: &str) {
    println!("{}", s);
}
```

***

<a id="structs-vs-classes"></a>
## 구조체와 클래스

Rust의 구조체는 C#의 클래스와 비슷하지만, 소유권과 메서드 측면에서 중요한 차이가 있습니다.

```mermaid
graph TD
    subgraph "C# 클래스 (힙)"
        CObj["객체 헤더\n+ vtable 포인터"] --> CFields["Name: string 참조\nAge: int\nHobbies: List 참조"]
        CFields --> CHeap1["\"Alice\" 힙 버퍼"]
        CFields --> CHeap2["List&lt;string&gt; 힙 버퍼"]
    end
    subgraph "Rust 구조체 (스택)"
        RFields["name: String\n  ptr | len | cap\nage: i32\nhobbies: Vec\n  ptr | len | cap"]
        RFields --> RHeap1["\"Alice\" 힙 버퍼"]
        RFields --> RHeap2["Vec 힙 버퍼"]
    end

    style CObj fill:#bbdefb,color:#000
    style RFields fill:#c8e6c9,color:#000
```

> **핵심 통찰:** C# 클래스는 항상 참조 뒤에 놓인 힙 객체입니다. Rust 구조체는 기본적으로 스택에 놓이고, `String` 내용처럼 동적 크기 데이터만 힙에 갑니다. 덕분에 작고 자주 생성되는 객체에서 GC 오버헤드를 줄일 수 있습니다.

### C# 클래스 정의
```csharp
// 프로퍼티와 메서드를 가진 C# 클래스
public class Person
{
    public string Name { get; set; }
    public int Age { get; set; }
    public List<string> Hobbies { get; set; }
    
    public Person(string name, int age)
    {
        Name = name;
        Age = age;
        Hobbies = new List<string>();
    }
    
    public void AddHobby(string hobby)
    {
        Hobbies.Add(hobby);
    }
    
    public string GetInfo()
    {
        return $"{Name} is {Age} years old";
    }
}
```

### Rust 구조체 정의
```rust
// 연관 함수와 메서드를 가진 Rust 구조체
#[derive(Debug)]  // Debug 트레잇 자동 구현
pub struct Person {
    pub name: String,    // 공개 필드
    pub age: u32,        // 공개 필드
    hobbies: Vec<String>, // 비공개 필드(pub 없음)
}

impl Person {
    // 연관 함수(static 메서드와 비슷함)
    pub fn new(name: String, age: u32) -> Person {
        Person {
            name,
            age,
            hobbies: Vec::new(),
        }
    }
    
    // 메서드(&self, &mut self, self 중 하나를 받음)
    pub fn add_hobby(&mut self, hobby: String) {
        self.hobbies.push(hobby);
    }
    
    // 불변 대여하는 메서드
    pub fn get_info(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
    
    // 비공개 필드용 getter
    pub fn hobbies(&self) -> &Vec<String> {
        &self.hobbies
    }
}
```

### 인스턴스 생성과 사용
```csharp
// C# 객체 생성과 사용
var person = new Person("Alice", 30);
person.AddHobby("Reading");
person.AddHobby("Swimming");

Console.WriteLine(person.GetInfo());
Console.WriteLine($"Hobbies: {string.Join(", ", person.Hobbies)}");

// 프로퍼티 직접 수정
person.Age = 31;
```

```rust
// Rust 구조체 생성과 사용
let mut person = Person::new("Alice".to_string(), 30);
person.add_hobby("Reading".to_string());
person.add_hobby("Swimming".to_string());

println!("{}", person.get_info());
println!("Hobbies: {:?}", person.hobbies());

// 공개 필드 직접 수정
person.age = 31;

// 구조체 전체를 Debug 출력
println!("{:?}", person);
```

### 구조체 초기화 패턴
```csharp
// C# 객체 초기화
var person = new Person("Bob", 25)
{
    Hobbies = new List<string> { "Gaming", "Coding" }
};

// 익명 타입
var anonymous = new { Name = "Charlie", Age = 35 };
```

```rust
// Rust 구조체 초기화
let person = Person {
    name: "Bob".to_string(),
    age: 25,
    hobbies: vec!["Gaming".to_string(), "Coding".to_string()],
};

// 구조체 업데이트 문법(object spread와 비슷함)
let older_person = Person {
    age: 26,
    ..person  // 나머지 필드는 person에서 가져옴(person은 move됨!)
};

// 튜플 구조체(익명 타입과 비슷한 느낌)
#[derive(Debug)]
struct Point(i32, i32);

let point = Point(10, 20);
println!("Point: ({}, {})", point.0, point.1);
```

***

## 메서드와 연관 함수

메서드와 연관 함수의 차이를 이해하는 것은 중요합니다.

### C# 메서드 종류
```csharp
public class Calculator
{
    private int memory = 0;
    
    // 인스턴스 메서드
    public int Add(int a, int b)
    {
        return a + b;
    }
    
    // 상태를 사용하는 인스턴스 메서드
    public void StoreInMemory(int value)
    {
        memory = value;
    }
    
    // 정적 메서드
    public static int Multiply(int a, int b)
    {
        return a * b;
    }
    
    // 정적 팩터리 메서드
    public static Calculator CreateWithMemory(int initialMemory)
    {
        var calc = new Calculator();
        calc.memory = initialMemory;
        return calc;
    }
}
```

### Rust 메서드 종류
```rust
#[derive(Debug)]
pub struct Calculator {
    memory: i32,
}

impl Calculator {
    // 연관 함수(static 메서드와 비슷함) - self 매개변수가 없음
    pub fn new() -> Calculator {
        Calculator { memory: 0 }
    }
    
    // 매개변수를 받는 연관 함수
    pub fn with_memory(initial_memory: i32) -> Calculator {
        Calculator { memory: initial_memory }
    }
    
    // 불변 대여하는 메서드(&self)
    pub fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
    
    // 가변 대여하는 메서드(&mut self)
    pub fn store_in_memory(&mut self, value: i32) {
        self.memory = value;
    }
    
    // 소유권을 가져가는 메서드(self)
    pub fn into_memory(self) -> i32 {
        self.memory  // Calculator가 소비됨
    }
    
    // getter 메서드
    pub fn memory(&self) -> i32 {
        self.memory
    }
}

fn main() {
    // 연관 함수는 :: 로 호출
    let mut calc = Calculator::new();
    let calc2 = Calculator::with_memory(42);
    
    // 메서드는 . 으로 호출
    let result = calc.add(5, 3);
    calc.store_in_memory(result);
    
    println!("Memory: {}", calc.memory());
    
    // 소비하는 메서드
    let memory_value = calc.into_memory();  // calc는 더 이상 사용할 수 없음
    println!("Final memory: {}", memory_value);
}
```

### 메서드 리시버 타입 설명
```rust
impl Person {
    // &self - 불변 대여(가장 흔함)
    // 데이터를 읽기만 하면 될 때 사용
    pub fn get_name(&self) -> &str {
        &self.name
    }
    
    // &mut self - 가변 대여
    // 데이터를 수정해야 할 때 사용
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    
    // self - 소유권 가져오기(덜 흔함)
    // 구조체를 소비하고 싶을 때 사용
    pub fn consume(self) -> String {
        self.name  // Person이 move되어 더 이상 접근할 수 없음
    }
}

fn method_examples() {
    let mut person = Person::new("Alice".to_string(), 30);
    
    // 불변 대여
    let name = person.get_name();  // person은 계속 사용할 수 있음
    println!("Name: {}", name);
    
    // 가변 대여
    person.set_name("Alice Smith".to_string());  // person은 계속 사용할 수 있음
    
    // 소유권 가져오기
    let final_name = person.consume();  // person은 더 이상 사용할 수 없음
    println!("Final name: {}", final_name);
}
```

---

## 연습문제

<details>
<summary><strong>🏋️ 연습문제: 슬라이스 윈도 평균</strong> (펼쳐서 보기)</summary>

**도전 과제:** `f64` 슬라이스와 윈도 크기를 받아, 구간별 평균을 담은 `Vec<f64>`를 반환하는 함수를 작성해 보세요. 예를 들어 `[1.0, 2.0, 3.0, 4.0, 5.0]`에 윈도 크기 3을 주면 `[2.0, 3.0, 4.0]`를 반환해야 합니다.

```rust
fn rolling_average(data: &[f64], window: usize) -> Vec<f64> {
    // 여기에 구현하세요
    todo!()
}

fn main() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let avgs = rolling_average(&data, 3);
    println!("{avgs:?}"); // [2.0, 3.0, 4.0]
}
```

<details>
<summary>🔑 해답</summary>

```rust
fn rolling_average(data: &[f64], window: usize) -> Vec<f64> {
    data.windows(window)
        .map(|w| w.iter().sum::<f64>() / w.len() as f64)
        .collect()
}

fn main() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let avgs = rolling_average(&data, 3);
    assert_eq!(avgs, vec![2.0, 3.0, 4.0]);
    println!("{avgs:?}");
}
```

**핵심 포인트:** 슬라이스에는 `.windows()`, `.chunks()`, `.split()`처럼 강력한 내장 메서드가 있어 수동 인덱스 계산을 대체할 수 있습니다. C#에서는 `Enumerable.Range`나 LINQ의 `.Skip().Take()`를 떠올리면 됩니다.

</details>
</details>

<details>
<summary><strong>🏋️ 연습문제: 미니 주소록</strong> (펼쳐서 보기)</summary>

구조체, 열거형, 메서드를 사용해 작은 주소록을 만들어 보세요.

1. `PhoneType { Mobile, Home, Work }` 열거형을 정의하세요.
2. `name: String`과 `phones: Vec<(PhoneType, String)>`를 가진 `Contact` 구조체를 정의하세요.
3. `Contact::new(name: impl Into<String>) -> Self`를 구현하세요.
4. `Contact::add_phone(&mut self, kind: PhoneType, number: impl Into<String>)`를 구현하세요.
5. 휴대전화 번호만 반환하는 `Contact::mobile_numbers(&self) -> Vec<&str>`를 구현하세요.
6. `main`에서 연락처를 만들고 전화번호 두 개를 추가한 뒤, 휴대전화 번호를 출력하세요.

<details>
<summary>🔑 해답</summary>

```rust
#[derive(Debug, PartialEq)]
enum PhoneType { Mobile, Home, Work }

#[derive(Debug)]
struct Contact {
    name: String,
    phones: Vec<(PhoneType, String)>,
}

impl Contact {
    fn new(name: impl Into<String>) -> Self {
        Contact { name: name.into(), phones: Vec::new() }
    }

    fn add_phone(&mut self, kind: PhoneType, number: impl Into<String>) {
        self.phones.push((kind, number.into()));
    }

    fn mobile_numbers(&self) -> Vec<&str> {
        self.phones
            .iter()
            .filter(|(kind, _)| *kind == PhoneType::Mobile)
            .map(|(_, num)| num.as_str())
            .collect()
    }
}

fn main() {
    let mut alice = Contact::new("Alice");
    alice.add_phone(PhoneType::Mobile, "+1-555-0100");
    alice.add_phone(PhoneType::Work, "+1-555-0200");
    alice.add_phone(PhoneType::Mobile, "+1-555-0101");

    println!("{}'s mobile numbers: {:?}", alice.name, alice.mobile_numbers());
}
```

</details>
</details>

***