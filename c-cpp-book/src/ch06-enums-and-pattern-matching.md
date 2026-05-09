<a id="rust-enum-types"></a>
# Rust enum 타입

> **이 장에서 배우는 것:** Rust의 enum을 discriminated union, 즉 "제대로 된 tagged union"으로 이해하고, `match`를 통한 완전한 패턴 매칭, 그리고 enum이 어떻게 C++ 클래스 계층이나 C의 tagged union을 더 안전하게 대체하는지 배웁니다.

- enum 타입은 discriminated union입니다. 즉 여러 가능한 타입 중 하나를 담는 합 타입(sum type)이며, 현재 어떤 variant인지 나타내는 태그를 함께 가집니다.
    - C 개발자 관점: Rust enum은 데이터를 담을 수 있습니다. 즉 "제대로 된 tagged union"이며, 어떤 variant가 활성 상태인지 컴파일러가 추적합니다.
    - C++ 개발자 관점: Rust enum은 `std::variant`와 비슷하지만, 완전한 패턴 매칭이 있고 `std::get` 예외도 없으며 `std::visit` 보일러플레이트도 없습니다.
    - enum의 크기는 가장 큰 variant의 크기에 맞춰집니다. 각 variant는 서로 상속 관계가 없으며 완전히 다른 타입을 담을 수 있습니다.
    - enum은 Rust에서 가장 강력한 기능 중 하나이며, C++의 클래스 계층 전체를 대체하기도 합니다.
```rust
fn main() {
    enum Numbers {
        Zero,
        SmallNumber(u8),
        BiggerNumber(u32),
        EvenBiggerNumber(u64),
    }
    let a = Numbers::Zero;
    let b = Numbers::SmallNumber(42);
    let c: Numbers = a; // OK -- a의 타입은 Numbers
    let d: Numbers = b; // OK -- b의 타입도 Numbers
}
```
----
<a id="rust-match-statement"></a>
# Rust match 문
- Rust의 `match`는 C의 `switch`를 훨씬 강력하게 만든 것이라 볼 수 있습니다.
    - `match`는 단순한 값뿐 아니라 `struct`, `enum`에도 패턴 매칭할 수 있습니다.
    - `match`는 반드시 완전해야 합니다. 즉 해당 `type`의 가능한 모든 경우를 다뤄야 합니다. `_`는 "그 외 모든 경우"를 의미하는 와일드카드입니다.
    - `match`는 값을 반환할 수 있지만, 모든 arm(`=>`)은 같은 타입의 값을 반환해야 합니다.

```rust
fn main() {
    let x = 42;
    // 이 경우 _는 명시된 값 외의 모든 수를 포괄한다
    let is_secret_of_life = match x {
        42 => true,  // bool 반환
        _ => false,  // bool 반환
        // 아래는 반환 타입이 bool이 아니므로 컴파일되지 않음
        // _ => 0
    };
    println!("{is_secret_of_life}");
}
```

# Rust match 문
- `match`는 범위, 불리언 조건, `if` 가드도 지원합니다.
```rust
fn main() {
    let x = 42;
    match x {
        // =41 이므로 끝값 포함 범위
        0..=41 => println!("Less than the secret of life"),
        42 => println!("Secret of life"),
        _ => println!("More than the secret of life"),
    }
    let y = 100;
    match y {
        100 if x == 43 => println!("y is 100% not secret of life"),
        100 if x == 42 => println!("y is 100% secret of life"),
        _ => (), // 아무것도 하지 않음
    }
}
```

# Rust match 문
- `match`와 `enum`은 자주 함께 사용됩니다.
    - `match`는 variant 안의 값을 변수로 바인딩할 수 있습니다. 값이 중요하지 않다면 `_`를 사용하면 됩니다.
    - `matches!` 매크로를 쓰면 특정 variant인지 불리언으로 검사할 수 있습니다.
```rust
fn main() {
    enum Numbers {
        Zero,
        SmallNumber(u8),
        BiggerNumber(u32),
        EvenBiggerNumber(u64),
    }
    let b = Numbers::SmallNumber(42);
    match b {
        Numbers::Zero => println!("Zero"),
        Numbers::SmallNumber(value) => println!("Small number {value}"),
        Numbers::BiggerNumber(_) | Numbers::EvenBiggerNumber(_) => {
            println!("Some BiggerNumber or EvenBiggerNumber")
        }
    }
    
    // 특정 variant인지 불리언 테스트
    if matches!(b, Numbers::Zero | Numbers::SmallNumber(_)) {
        println!("Matched Zero or small number");
    }
}
```

# Rust match 문
- `match`는 구조 분해와 슬라이스 패턴도 지원합니다.
```rust
fn main() {
    struct Foo {
        x: (u32, bool),
        y: u32
    }
    let f = Foo {x: (42, true), y: 100};
    match f {
        // x의 값을 tuple 변수로 바인딩
        Foo { y: 100, x: tuple } => println!("Matched x: {tuple:?}"),
        _ => ()
    }
    let a = [40, 41, 42];
    match a {
        // 슬라이스의 마지막 원소가 42여야 한다. @로 패턴 결과를 바인딩
        [rest @ .., 42] => println!("{rest:?}"),
        // 슬라이스의 첫 원소가 42여야 한다
        [42, rest @ ..] => println!("{rest:?}"),
        _ => (),
    }
}
```

<a id="exercise-implement-add-and-subtract-using-match-and-enum"></a>
# 연습문제: match와 enum으로 덧셈/뺄셈 구현하기

🟢 **Starter**

- 부호 없는 64비트 정수에 대한 산술 연산 함수를 작성하세요.
- **1단계**: 연산용 enum 정의
```rust
enum Operation {
    Add(u64, u64),
    Subtract(u64, u64),
}
```
- **2단계**: 결과용 enum 정의
```rust
enum CalcResult {
    Ok(u64),                    // 성공 결과
    Invalid(String),            // 잘못된 연산에 대한 에러 메시지
}
```
- **3단계**: `calculate(op: Operation) -> CalcResult` 구현
    - Add는 `Ok(sum)` 반환
    - Subtract는 첫 번째 값이 두 번째 값보다 크거나 같으면 `Ok(difference)`, 아니면 `Invalid("Underflow")`
- **힌트**: 함수 안에서 패턴 매칭을 사용하세요.
```rust
match op {
    Operation::Add(a, b) => { /* your code */ },
    Operation::Subtract(a, b) => { /* your code */ },
}
```

<details><summary>해답 (클릭하여 펼치기)</summary>

```rust
enum Operation {
    Add(u64, u64),
    Subtract(u64, u64),
}

enum CalcResult {
    Ok(u64),
    Invalid(String),
}

fn calculate(op: Operation) -> CalcResult {
    match op {
        Operation::Add(a, b) => CalcResult::Ok(a + b),
        Operation::Subtract(a, b) => {
            if a >= b {
                CalcResult::Ok(a - b)
            } else {
                CalcResult::Invalid("Underflow".to_string())
            }
        }
    }
}

fn main() {
    match calculate(Operation::Add(10, 20)) {
        CalcResult::Ok(result) => println!("10 + 20 = {result}"),
        CalcResult::Invalid(msg) => println!("Error: {msg}"),
    }
    match calculate(Operation::Subtract(5, 10)) {
        CalcResult::Ok(result) => println!("5 - 10 = {result}"),
        CalcResult::Invalid(msg) => println!("Error: {msg}"),
    }
}
// Output:
// 10 + 20 = 30
// Error: Underflow
```

</details>

# Rust 연관 메서드
- `impl`은 `struct`, `enum` 같은 타입에 연관 메서드를 정의할 수 있습니다.
    - 메서드는 선택적으로 `self`를 받을 수 있습니다. `self`는 개념적으로 C에서 구조체 포인터를 첫 번째 인자로 넘기는 것, 혹은 C++의 `this`와 비슷합니다.
    - `self` 참조는 불변(`&self`), 가변(`&mut self`), 소유권 이전(`self`) 중 하나일 수 있습니다.
    - `Self` 키워드는 현재 타입을 가리키는 축약 표기입니다.
```rust
struct Point { x: u32, y: u32 }
impl Point {
    fn new(x: u32, y: u32) -> Self {
        Point { x, y }
    }
    fn increment_x(&mut self) {
        self.x += 1;
    }
}
fn main() {
    let mut p = Point::new(10, 20);
    p.increment_x();
}
```

# 연습문제: Point add와 transform

🟡 **Intermediate** - 메서드 시그니처에서 move와 borrow 차이를 이해해야 합니다.
- `Point`에 다음 연관 메서드를 구현하세요.
    - `add()`는 다른 `Point`를 받아 x와 y를 제자리에서 증가시킵니다. 힌트: `&mut self`
    - `transform()`은 기존 `Point`를 소비합니다. 힌트: `self`를 사용하고, x와 y를 제곱한 새 `Point`를 반환하세요.

<details><summary>해답 (클릭하여 펼치기)</summary>

```rust
struct Point { x: u32, y: u32 }

impl Point {
    fn new(x: u32, y: u32) -> Self {
        Point { x, y }
    }
    fn add(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
    }
    fn transform(self) -> Point {
        Point { x: self.x * self.x, y: self.y * self.y }
    }
}

fn main() {
    let mut p1 = Point::new(2, 3);
    let p2 = Point::new(10, 20);
    p1.add(&p2);
    println!("After add: x={}, y={}", p1.x, p1.y);       // x=12, y=23
    let p3 = p1.transform();
    println!("After transform: x={}, y={}", p3.x, p3.y); // x=144, y=529
    // p1은 더 이상 접근할 수 없음 - transform()이 소비했기 때문
}
```

</details>

----
