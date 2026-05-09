<a id="rust-from-and-into-traits"></a>
# Rust From과 Into 트레잇

> **이 장에서 배우는 것:** Rust의 타입 변환 트레잇인 `From<T>`와 `Into<T>`(실패하지 않는 변환), 그리고 `TryFrom`, `TryInto`(실패할 수 있는 변환)를 배웁니다. `From`을 구현하면 `Into`는 자동으로 따라옵니다. 이는 C++의 변환 연산자와 변환 생성자를 대체하는 방식입니다.

- `From`과 `Into`는 타입 변환을 돕는 상보적인 트레잇입니다.
- 보통은 `From` 트레잇을 구현합니다. `String::from()`은 `"&str"`에서 `String`으로 변환하고, 컴파일러는 `&str.into()` 형태도 자동으로 사용할 수 있게 해줍니다.
```rust
struct Point {x: u32, y: u32}
// 튜플로부터 Point를 만든다
impl From<(u32, u32)> for Point {
    fn from(xy : (u32, u32)) -> Self {
        Point {x : xy.0, y: xy.1}       // 튜플의 요소를 사용해 Point 구성
    }
}
fn main() {
    let s = String::from("Rust");
    let x = u32::from(true);
    let p = Point::from((40, 42));
    // let p : Point = (40.42)::into(); // 위와 같은 의미의 다른 형태
    println!("s: {s} x:{x} p.x:{} p.y {}", p.x, p.y);
}
```

<a id="exercise-from-and-into"></a>
# 연습문제: From과 Into
- `Point`를 `TransposePoint`라는 타입으로 변환하는 `From` 트레잇을 구현하세요. `TransposePoint`는 `Point`의 `x`와 `y`를 서로 바꾼 타입입니다.

<details><summary>해답 (클릭하여 펼치기)</summary>

```rust
struct Point { x: u32, y: u32 }
struct TransposePoint { x: u32, y: u32 }

impl From<Point> for TransposePoint {
    fn from(p: Point) -> Self {
        TransposePoint { x: p.y, y: p.x }
    }
}

fn main() {
    let p = Point { x: 10, y: 20 };
    let tp = TransposePoint::from(p);
    println!("Transposed: x={}, y={}", tp.x, tp.y);  // x=20, y=10

    // .into() 사용 - From이 구현되어 있으면 자동으로 동작
    let p2 = Point { x: 3, y: 7 };
    let tp2: TransposePoint = p2.into();
    println!("Transposed: x={}, y={}", tp2.x, tp2.y);  // x=7, y=3
}
// 출력:
// Transposed: x=20, y=10
// Transposed: x=7, y=3
```

</details>

<a id="rust-default-trait"></a>
# Rust Default 트레잇
- `Default`는 타입의 기본값을 구현할 때 사용할 수 있습니다.
    - 타입은 `Default`와 함께 `derive` 매크로를 사용할 수도 있고, 직접 구현을 제공할 수도 있습니다.
```rust
#[derive(Default, Debug)]
struct Point {x: u32, y: u32}
#[derive(Debug)]
struct CustomPoint {x: u32, y: u32}
impl Default for CustomPoint {
    fn default() -> Self {
        CustomPoint {x: 42, y: 42}
    }
}
fn main() {
    let x = Point::default();   // Point{0, 0} 생성
    println!("{x:?}");
    let y = CustomPoint::default();
    println!("{y:?}");
}
```

<a id="rust-default-trait-1"></a>
### Rust Default 트레잇
- `Default` 트레잇은 다음과 같은 여러 용도로 쓸 수 있습니다.
    - 일부 필드만 덮어쓰고 나머지는 기본값으로 초기화하기
    - `unwrap_or_default()` 같은 메서드에서 `Option` 타입의 기본 대안으로 사용하기
```rust
#[derive(Debug)]
struct CustomPoint {x: u32, y: u32}
impl Default for CustomPoint {
    fn default() -> Self {
        CustomPoint {x: 42, y: 42}
    }
}
fn main() {
    let x = CustomPoint::default();
    // y만 덮어쓰고 나머지 필드는 기본값 유지
    let y = CustomPoint {y: 43, ..CustomPoint::default()};
    println!("{x:?} {y:?}");
    let z : Option<CustomPoint> = None;
    // unwrap_or_default()를 unwrap()으로 바꿔보자
    println!("{:?}", z.unwrap_or_default());
}
```

<a id="other-rust-type-conversions"></a>
### 기타 Rust 타입 변환
- Rust는 암시적 타입 변환을 지원하지 않으며, 명시적 변환에는 `as`를 사용할 수 있습니다.
- `as`는 narrowing 등으로 인한 데이터 손실 가능성이 있으므로 꼭 필요할 때만 사용하는 편이 좋습니다. 가능하다면 일반적으로 `into()`나 `from()`을 우선하세요.
```rust
fn main() {
    let f = 42u8;
    // let g : u32 = f;    // 컴파일되지 않는다
    let g = f as u32;      // 가능하지만 선호되지는 않음. narrowing 관련 규칙의 영향을 받음
    let g : u32 = f.into(); // 가장 선호되는 형태; 실패하지 않고 컴파일러가 검사함
    //let k : u8 = f.into();  // 컴파일 실패; narrowing은 데이터 손실을 일으킬 수 있다

    // narrowing 연산을 시도하려면 try_into를 사용해야 한다
    if let Ok(k) = TryInto::<u8>::try_into(g) {
        println!("{k}");
    }
}
```
