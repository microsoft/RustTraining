<a id="essential-rust-keywords-for-c-developers"></a>
## C# 개발자를 위한 핵심 Rust 키워드

> **이 절에서 배울 내용:** Rust 키워드를 C# 대응 개념과 빠르게 매핑합니다. 가시성 지정자, 소유권 관련 키워드, 제어 흐름, 타입 정의, 패턴 매칭 문법을 한 번에 훑습니다.
>
> **난이도:** 🟢 입문

Rust 키워드와 그 역할을 이해하면 C# 개발자가 Rust 코드를 훨씬 빠르게 읽고 감을 잡을 수 있습니다.

<a id="visibility-and-access-control-keywords"></a>
### 가시성과 접근 제어 키워드

#### C# 접근 지정자
```csharp
public class Example
{
    public int PublicField;           // 어디서나 접근 가능
    private int privateField;        // 이 클래스 내부에서만 접근 가능
    protected int protectedField;    // 이 클래스와 하위 클래스
    internal int internalField;      // 같은 어셈블리 내부
    protected internal int protectedInternalField; // 조합
}
```

#### Rust 가시성 키워드
```rust
// pub - 항목을 공개한다 (C#의 public과 비슷)
pub struct PublicStruct {
    pub public_field: i32,           // 공개 필드
    private_field: i32,              // 기본은 private (키워드 없음)
}

pub mod my_module {
    pub(crate) fn crate_public() {}     // 현재 crate 내부 공개 (internal과 비슷)
    pub(super) fn parent_public() {}    // 부모 모듈까지 공개
    pub(self) fn self_public() {}       // 현재 모듈 내부 공개 (사실상 private)
    
    pub use super::PublicStruct;        // 재노출 (using alias와 비슷)
}

// C#의 protected와 정확히 일치하는 개념은 없다 - 대신 조합을 사용한다
```

<a id="memory-and-ownership-keywords"></a>
### 메모리와 소유권 키워드

#### C# 메모리 관련 키워드
```csharp
// ref - 참조 전달
public void Method(ref int value) { value = 10; }

// out - 출력 파라미터
public bool TryParse(string input, out int result) { /* */ }

// in - 읽기 전용 참조 (C# 7.2+)
public void ReadOnly(in LargeStruct data) { /* Cannot modify data */ }
```

#### Rust 소유권 키워드
```rust
// & - 불변 참조 (C#의 in 파라미터와 비슷)
fn read_only(data: &Vec<i32>) {
    println!("Length: {}", data.len()); // 읽을 수는 있지만 수정은 못 한다
}

// &mut - 가변 참조 (C#의 ref 파라미터와 비슷)
fn modify(data: &mut Vec<i32>) {
    data.push(42); // 수정 가능
}

// move - 클로저에서 강제 move capture
let data = vec![1, 2, 3];
let closure = move || {
    println!("{:?}", data); // data가 클로저 안으로 이동
};
// 이제 여기서는 data에 접근할 수 없다

// Box - 힙 할당 (C#에서 참조 타입에 new를 쓰는 것과 비슷)
let boxed_data = Box::new(42); // 힙에 할당
```

<a id="control-flow-keywords"></a>
### 제어 흐름 키워드

#### C# 제어 흐름
```csharp
// return - 값을 반환하며 함수 종료
public int GetValue() { return 42; }

// yield return - 이터레이터 패턴
public IEnumerable<int> GetNumbers()
{
    yield return 1;
    yield return 2;
}

// break/continue - 반복 제어
foreach (var item in items)
{
    if (item == null) continue;
    if (item.Stop) break;
}
```

#### Rust 제어 흐름 키워드
```rust
// return - 명시적 반환 (보통은 마지막 표현식만 써도 된다)
fn get_value() -> i32 {
    return 42; // 명시적 반환
    // 또는 그냥: 42 (암시적 반환)
}

// break/continue - 값까지 함께 다룰 수 있는 반복 제어
fn find_value() -> Option<i32> {
    loop {
        let value = get_next();
        if value < 0 { continue; }
        if value > 100 { break None; }        // 값을 들고 break
        if value == 42 { break Some(value); } // 성공 값을 들고 break
    }
}

// loop - 무한 반복 (while(true)와 비슷)
loop {
    if condition { break; }
}

// while - 조건 반복
while condition {
    // code
}

// for - 이터레이터 기반 반복
for item in collection {
    // code
}
```

<a id="type-definition-keywords"></a>
### 타입 정의 키워드

#### C# 타입 키워드
```csharp
// class - 참조 타입
public class MyClass { }

// struct - 값 타입
public struct MyStruct { }

// interface - 계약 정의
public interface IMyInterface { }

// enum - 열거형
public enum MyEnum { Value1, Value2 }

// delegate - 함수 포인터
public delegate void MyDelegate(int value);
```

#### Rust 타입 키워드
```rust
// struct - 데이터 구조 (C#의 class/struct를 합친 느낌)
struct MyStruct {
    field: i32,
}

// enum - 대수적 데이터 타입 (C# enum보다 훨씬 강력)
enum MyEnum {
    Variant1,
    Variant2(i32),               // 데이터를 담을 수 있다
    Variant3 { x: i32, y: i32 }, // struct 형태 variant
}

// trait - 인터페이스 정의 (C# interface보다 더 강력)
trait MyTrait {
    fn method(&self);
    
    // 기본 구현 (C# 8+의 default interface method와 비슷)
    fn default_method(&self) {
        println!("Default implementation");
    }
}

// type - 타입 별칭 (C#의 using alias와 비슷)
type UserId = u32;
type Result<T> = std::result::Result<T, MyError>;

// impl - 구현 블록 (C#에는 직접 대응되는 키워드가 없음)
impl MyStruct {
    fn new() -> MyStruct {
        MyStruct { field: 0 }
    }
}

impl MyTrait for MyStruct {
    fn method(&self) {
        println!("Implementation");
    }
}
```

<a id="function-definition-keywords"></a>
### 함수 정의 키워드

#### C# 함수 관련 키워드
```csharp
// static - 클래스 메서드
public static void StaticMethod() { }

// virtual - 재정의 가능
public virtual void VirtualMethod() { }

// override - 기반 메서드 재정의
public override void VirtualMethod() { }

// abstract - 반드시 구현해야 함
public abstract void AbstractMethod();

// async - 비동기 메서드
public async Task<int> AsyncMethod() { return await SomeTask(); }
```

#### Rust 함수 관련 키워드
```rust
// fn - 함수 정의 (C# 메서드와 비슷하지만 독립적으로 존재 가능)
fn regular_function() {
    println!("Hello");
}

// const fn - 컴파일 타임 함수 (C# const를 함수로 확장한 느낌)
const fn compile_time_function() -> i32 {
    42 // 컴파일 타임에 평가 가능
}

// async fn - 비동기 함수 (C# async와 비슷)
async fn async_function() -> i32 {
    some_async_operation().await
}

// unsafe fn - 메모리 안전성을 깨뜨릴 수 있는 함수
unsafe fn unsafe_function() {
    // unsafe operation 수행 가능
}

// extern fn - 외부 함수 인터페이스
extern "C" fn c_compatible_function() {
    // C에서 호출 가능
}
```

<a id="variable-declaration-keywords"></a>
### 변수 선언 키워드

#### C# 변수 키워드
```csharp
// var - 타입 추론
var name = "John"; // string으로 추론

// const - 컴파일 타임 상수
const int MaxSize = 100;

// readonly - 런타임 상수
readonly DateTime createdAt = DateTime.Now;

// static - 클래스 수준 변수
static int instanceCount = 0;
```

#### Rust 변수 키워드
```rust
// let - 변수 바인딩 (C#의 var와 비슷)
let name = "John"; // 기본은 불변

// let mut - 가변 변수 바인딩
let mut count = 0; // 변경 가능
count += 1;

// const - 컴파일 타임 상수 (C# const와 비슷)
const MAX_SIZE: usize = 100;

// static - 전역 변수 (C# static과 비슷)
static INSTANCE_COUNT: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);
```

<a id="pattern-matching-keywords"></a>
### 패턴 매칭 키워드

#### C# 패턴 매칭 (C# 8+)
```csharp
// switch expression
string result = value switch
{
    1 => "One",
    2 => "Two",
    _ => "Other"
};

// is pattern
if (obj is string str)
{
    Console.WriteLine(str.Length);
}
```

#### Rust 패턴 매칭 키워드
```rust
// match - 패턴 매칭 (C# switch보다 훨씬 강력)
let result = match value {
    1 => "One",
    2 => "Two",
    3..=10 => "Between 3 and 10", // 범위 패턴
    _ => "Other", // 와일드카드 (C#의 _와 비슷)
};

// if let - 조건부 패턴 매칭
if let Some(value) = optional {
    println!("Got value: {}", value);
}

// while let - 패턴 매칭 기반 반복
while let Some(item) = iterator.next() {
    println!("Item: {}", item);
}

// 패턴을 쓰는 let - 구조 분해
let (x, y) = point; // tuple 구조 분해
let Some(value) = optional else {
    return; // 패턴이 맞지 않으면 조기 반환
};
```

<a id="memory-safety-keywords"></a>
### 메모리 안전성 키워드

#### C# 메모리 관련 키워드
```csharp
// unsafe - 안전성 검사를 끈다
unsafe
{
    int* ptr = &variable;
    *ptr = 42;
}

// fixed - 관리되는 메모리를 pin 고정
unsafe
{
    fixed (byte* ptr = array)
    {
        // ptr 사용
    }
}
```

#### Rust 안전성 키워드
```rust
// unsafe - borrow checker를 우회한다 (신중히 사용!)
unsafe {
    let ptr = &variable as *const i32;
    let value = *ptr; // raw pointer 역참조
}

// Raw pointer 타입 (C#에 직접 대응되는 개념은 거의 없음)
let ptr: *const i32 = &42;   // 불변 raw pointer
let ptr: *mut i32 = &mut 42; // 가변 raw pointer
```

<a id="common-rust-keywords-not-in-c"></a>
### C#에 없는, Rust에서 자주 보는 키워드

```rust
// where - 제네릭 제약 (C# where보다 더 유연)
fn generic_function<T>()
where
    T: Clone + Send + Sync,
{
    // T는 Clone, Send, Sync를 구현해야 한다
}

// dyn - 동적 trait object (C# object와 비슷하지만 더 타입 안전)
let drawable: Box<dyn Draw> = Box::new(Circle::new());

// Self - 구현 중인 타입 자신을 가리킴 (타입 수준의 this 같은 개념)
impl MyStruct {
    fn new() -> Self { // Self = MyStruct
        Self { field: 0 }
    }
}

// self - 메서드 receiver
impl MyStruct {
    fn method(&self) { }         // 불변 대여
    fn method_mut(&mut self) { } // 가변 대여
    fn consume(self) { }         // 소유권 가져감
}

// crate - 현재 crate 루트를 가리킴
use crate::models::User; // crate 루트부터의 절대 경로

// super - 부모 모듈을 가리킴
use super::utils; // 부모 모듈에서 가져오기
```

<a id="keywords-summary-for-c-developers"></a>
### C# 개발자를 위한 키워드 요약

| 용도 | C# | Rust | 핵심 차이 |
|---------|----|----|----------------|
| 가시성 | `public`, `private`, `internal` | `pub`, 기본 private | `pub(crate)`처럼 더 세분화 가능 |
| 변수 | `var`, `readonly`, `const` | `let`, `let mut`, `const` | 기본이 불변 |
| 함수 | `method()` | `fn` | 독립 함수가 가능 |
| 타입 | `class`, `struct`, `interface` | `struct`, `enum`, `trait` | enum이 대수적 타입 |
| 제네릭 | `<T> where T : IFoo` | `<T> where T: Foo` | 제약 표현이 더 유연 |
| 참조 | `ref`, `out`, `in` | `&`, `&mut` | 컴파일 타임 borrow checking |
| 패턴 | `switch`, `is` | `match`, `if let` | 완전 매칭이 요구됨 |

***
