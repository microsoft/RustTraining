<a id="rust-generics"></a>
# Rust 제네릭

> **이 장에서 배우는 것:** 제네릭 타입 매개변수, 모노모피제이션(제로 코스트 제네릭), 트레잇 바운드, 그리고 Rust 제네릭이 C++ 템플릿과 어떻게 다른지 배웁니다. 더 나은 에러 메시지를 주고 SFINAE도 없습니다.

- 제네릭을 사용하면 같은 알고리즘이나 자료구조를 여러 데이터 타입에 재사용할 수 있습니다.
    - 제네릭 매개변수는 `<>` 안의 식별자로 나타납니다. 예: `<T>`. 매개변수 이름은 어떤 합법적인 식별자든 가능하지만, 보통 간결함을 위해 짧게 씁니다.
    - 컴파일러는 컴파일 타임에 모노모피제이션을 수행합니다. 즉, 실제로 사용된 `T`의 각 변형마다 새로운 타입/코드를 생성합니다.
```rust
// 타입 <T>의 left와 right로 이루어진 <T> 튜플을 반환한다
fn pick<T>(x: u32, left: T, right: T) -> (T, T) {
   if x == 42 {
    (left, right)
   } else {
    (right, left)
   }
}
fn main() {
    let a = pick(42, true, false);
    let b = pick(42, "hello", "world");
    println!("{a:?}, {b:?}");
}
```

<a id="rust-generics-1"></a>
# Rust 제네릭
- 제네릭은 데이터 타입과 관련 메서드에도 적용할 수 있습니다. 특정 `<T>`에 대해 구현을 특수화하는 것도 가능합니다(예: `f32` vs `u32`).
```rust
#[derive(Debug)] // 이것은 뒤에서 설명한다
struct Point<T> {
    x : T,
    y : T,
}
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point {x, y}
    }
    fn set_x(&mut self, x: T) {
         self.x = x;
    }
    fn set_y(&mut self, y: T) {
         self.y = y;
    }
}
impl Point<f32> {
    fn is_secret(&self) -> bool {
        self.x == 42.0
    }
}
fn main() {
    let mut p = Point::new(2, 4); // i32
    let q = Point::new(2.0, 4.0); // f32
    p.set_x(42);
    p.set_y(43);
    println!("{p:?} {q:?} {}", q.is_secret());
}
```

<a id="exercise-generics"></a>
# 연습문제: 제네릭

🟢 **Starter**
- `Point` 타입을 수정해 `x`와 `y`가 서로 다른 두 타입(`T`, `U`)을 쓰도록 바꿔보세요.

<details><summary>해답 (클릭하여 펼치기)</summary>

```rust
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Self {
        Point { x, y }
    }
}

fn main() {
    let p1 = Point::new(42, 3.14);        // Point<i32, f64>
    let p2 = Point::new("hello", true);   // Point<&str, bool>
    let p3 = Point::new(1u8, 1000u64);    // Point<u8, u64>
    println!("{p1:?}");
    println!("{p2:?}");
    println!("{p3:?}");
}
// 출력:
// Point { x: 42, y: 3.14 }
// Point { x: "hello", y: true }
// Point { x: 1, y: 1000 }
```

</details>

<a id="combining-rust-traits-and-generics"></a>
### Rust 트레잇과 제네릭 함께 쓰기
- 트레잇은 제네릭 타입에 제약(constraint)을 걸 때 사용할 수 있습니다.
- 제약은 제네릭 타입 매개변수 뒤에 `:`를 붙이거나 `where`를 사용해 지정할 수 있습니다. 아래는 `ComputeArea` 트레잇을 구현한 어떤 타입 `T`든 받아들이는 제네릭 함수 `get_area`를 정의한 예입니다.
```rust
    trait ComputeArea {
        fn area(&self) -> u64;
    }
    fn get_area<T: ComputeArea>(t: &T) -> u64 {
        t.area()
    }
```
- [▶ Rust Playground에서 실행해보기](https://play.rust-lang.org/)

<a id="combining-rust-traits-and-generics-1"></a>
### Rust 트레잇과 제네릭 함께 쓰기
- 여러 개의 트레잇 제약을 동시에 둘 수도 있습니다.
```rust
trait Fish {}
trait Mammal {}
struct Shark;
struct Whale;
impl Fish for Shark {}
impl Fish for Whale {}
impl Mammal for Whale {}
fn only_fish_and_mammals<T: Fish + Mammal>(_t: &T) {}
fn main() {
    let w = Whale {};
    only_fish_and_mammals(&w);
    let _s = Shark {};
    // 컴파일되지 않는다
    only_fish_and_mammals(&_s);
}
```

<a id="rust-traits-constraints-in-data-types"></a>
### 데이터 타입에서의 Rust 트레잇 제약
- 트레잇 제약은 데이터 타입에서도 제네릭과 함께 사용할 수 있습니다.
- 아래 예제에서는 `PrintDescription` 트레잇과, 그 트레잇으로 제약된 멤버를 가지는 제네릭 `struct` `Shape`를 정의합니다.
```rust
trait PrintDescription {
    fn print_description(&self);
}
struct Shape<S: PrintDescription> {
    shape: S,
}
// PrintDescription을 구현한 어떤 타입에 대해서도 동작하는 제네릭 Shape 구현
impl<S: PrintDescription> Shape<S> {
    fn print(&self) {
        self.shape.print_description();
    }
}
```
- [▶ Rust Playground에서 실행해보기](https://play.rust-lang.org/)

<a id="exercise-traits-constraints-and-generics"></a>
# 연습문제: 트레잇 제약과 제네릭

🟡 **Intermediate**
- `CipherText`를 구현한 제네릭 멤버 `cipher`를 가지는 `struct`를 구현하세요.
```rust
trait CipherText {
    fn encrypt(&self);
}
// TO DO
//struct Cipher<>

```
- 다음으로 `struct`의 `impl`에 `encrypt`라는 메서드를 구현해, 내부의 `cipher`에 대해 `encrypt`를 호출하게 하세요.
```rust
// TO DO
impl for Cipher<> {}
```
- 다음으로 `CipherOne`, `CipherTwo`라는 두 `struct`에 `CipherText`를 구현하세요(`println()`만으로도 충분합니다). `CipherOne`과 `CipherTwo`를 만들고, `Cipher`를 통해 호출해보세요.

<details><summary>해답 (클릭하여 펼치기)</summary>

```rust
trait CipherText {
    fn encrypt(&self);
}

struct Cipher<T: CipherText> {
    cipher: T,
}

impl<T: CipherText> Cipher<T> {
    fn encrypt(&self) {
        self.cipher.encrypt();
    }
}

struct CipherOne;
struct CipherTwo;

impl CipherText for CipherOne {
    fn encrypt(&self) {
        println!("CipherOne encryption applied");
    }
}

impl CipherText for CipherTwo {
    fn encrypt(&self) {
        println!("CipherTwo encryption applied");
    }
}

fn main() {
    let c1 = Cipher { cipher: CipherOne };
    let c2 = Cipher { cipher: CipherTwo };
    c1.encrypt();
    c2.encrypt();
}
// 출력:
// CipherOne encryption applied
// CipherTwo encryption applied
```

</details>

<a id="rust-type-state-pattern-and-generics"></a>
### Rust type-state 패턴과 제네릭
- Rust 타입을 사용하면 상태 기계 전이를 *컴파일 타임에* 강제할 수 있습니다.
    - 예를 들어 `Drone`에 `Idle`과 `Flying` 두 상태가 있다고 합시다. `Idle` 상태에서는 `takeoff()`만 허용하고, `Flying` 상태에서는 `land()`만 허용할 수 있습니다.

- 한 가지 접근은 아래처럼 상태 기계를 모델링하는 것입니다.
```rust
enum DroneState {
    Idle,
    Flying
}
struct Drone {x: u64, y: u64, z: u64, state: DroneState}  // x, y, z는 좌표
```
- 하지만 이 방식은 상태 기계 의미를 강제하기 위해 많은 런타임 검사가 필요합니다. 왜 그런지 [▶ 직접 실행해보기](https://play.rust-lang.org/).

<a id="rust-type-state-pattern-generics"></a>
### Rust type-state 패턴과 제네릭
- 제네릭을 사용하면 상태 기계를 *컴파일 타임에* 강제할 수 있습니다. 이를 위해 `PhantomData<T>`라는 특별한 제네릭을 사용합니다.
- `PhantomData<T>`는 `zero-sized` 마커 데이터 타입입니다. 여기서는 `Idle`과 `Flying` 상태를 표현하는 데 쓰이지만, 런타임 크기는 `zero`입니다.
- `takeoff`와 `land` 메서드가 `self`를 매개변수로 받는다는 점에 주목하세요. 이를 `consuming`이라고 부릅니다(`&self`를 써서 빌리는 것과 대비). 즉, `Drone<Idle>`에서 `takeoff()`를 호출하면 오직 `Drone<Flying>`만 돌려받을 수 있고, 그 반대도 마찬가지입니다.
```rust
struct Drone<T> {x: u64, y: u64, z: u64, state: PhantomData<T> }
impl Drone<Idle> {
    fn takeoff(self) -> Drone<Flying> {...}
}
impl Drone<Flying> {
    fn land(self) -> Drone<Idle> { ...}
}
```
    - [▶ Rust Playground에서 실행해보기](https://play.rust-lang.org/)

<a id="rust-type-state-pattern-generics-1"></a>
### Rust type-state 패턴과 제네릭
- 핵심 요약:
    - 상태는 `struct`(zero-size)로 표현할 수 있습니다.
    - 상태 `T`를 `PhantomData<T>`(zero-size)와 결합할 수 있습니다.
    - 상태 기계의 특정 단계에만 메서드를 구현하는 일은 이제 `impl State<T>`를 쓰면 됩니다.
    - 한 상태에서 다른 상태로 전이할 때는 `self`를 소비하는 메서드를 사용하세요.
    - 이렇게 하면 `zero cost` 추상화를 얻습니다. 컴파일러가 상태 기계를 컴파일 타임에 강제하므로, 올바른 상태가 아니면 메서드를 호출하는 것이 불가능합니다.

<a id="rust-builder-pattern"></a>
### Rust 빌더 패턴
- `self`를 소비하는 방식은 빌더 패턴에서도 유용합니다.
- 수십 개의 핀을 가진 GPIO 설정을 생각해봅시다. 각 핀은 high 또는 low로 설정할 수 있고(기본값은 low), 이런 설정을 단계적으로 조합하고 싶을 수 있습니다.
```rust
#[derive(default)]
enum PinState {
    #[default]
    Low,
    High,
}
#[derive(default)]
struct GPIOConfig {
    pin0: PinState,
    pin1: PinState
    ...
}
```
- 빌더 패턴을 사용하면 체이닝 방식으로 GPIO 설정을 구성할 수 있습니다. [▶ 직접 실행해보기](https://play.rust-lang.org/)
