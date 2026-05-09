<a id="built-in-rust-types"></a>
# Rust 내장 타입

> **이 장에서 배우는 것:** Rust의 기본 타입(`i32`, `u64`, `f64`, `bool`, `char`), 타입 추론, 명시적 타입 표기, 그리고 이것이 C/C++의 기본 타입과 어떻게 대응되는지 살펴봅니다. Rust에는 암시적 형변환이 거의 없으며, 필요한 변환은 명시적으로 수행해야 합니다.

- Rust는 타입 추론을 지원하지만, 필요하면 타입을 명시적으로 적을 수도 있습니다.

|  **설명**  |            **타입**            |          **예시**          |
|:-----------------:|:------------------------------:|:-----------------------------:|
| 부호 있는 정수   | i8, i16, i32, i64, i128, isize | -1, 42, 1_00_000, 1_00_000i64 |
| 부호 없는 정수 | u8, u16, u32, u64, u128, usize | 0, 42, 42u32, 42u64           |
| 부동소수점    | f32, f64                       | 0.0, 0.42                     |
| 유니코드 문자           | char                           | 'a', '$'                      |
| 불리언           | bool                           | true, false                   |

- Rust는 숫자 가독성을 위해 숫자 사이에 `_`를 자유롭게 넣을 수 있습니다.
----
<a id="rust-type-specification-and-assignment"></a>
### Rust 타입 지정과 대입
- Rust는 `let` 키워드로 변수에 값을 바인딩합니다. 변수 타입은 `:` 뒤에 선택적으로 적을 수 있습니다.
```rust
fn main() {
    let x: i32 = 42;
    // 아래 두 대입은 논리적으로 동일하다
    let y: u32 = 42;
    let z = 42u32;
}
```
- 함수 매개변수와 반환값은 항상 타입을 명시해야 합니다. 아래 함수는 `u8`을 받아 `u32`를 반환합니다.
```rust
fn foo(x: u8) -> u32
{
    return x * x;
}
```
- 사용하지 않는 변수는 `_`로 시작시키면 컴파일러 경고를 피할 수 있습니다.
----
<a id="rust-type-specification-and-inference"></a>
# Rust 타입 지정과 추론
- Rust는 문맥을 바탕으로 변수의 타입을 자동 추론할 수 있습니다.
- [▶ Rust Playground에서 실행해 보기](https://play.rust-lang.org/)
```rust
fn secret_of_life_u32(x: u32) {
    println!("The u32 secret_of_life is {}", x);
}

fn secret_of_life_u8(x: u8) {
    println!("The u8 secret_of_life is {}", x);
}

fn main() {
    let a = 42; // let 키워드는 값을 바인딩한다. a의 타입은 u32
    let b = 42; // 문맥에 따라 b의 타입은 u8로 추론된다
    secret_of_life_u32(a);
    secret_of_life_u8(b);
}
```

<a id="rust-variables-and-mutability"></a>
# Rust 변수와 가변성
- Rust 변수는 기본적으로 **불변(immutable)** 입니다. 변수를 변경하려면 `mut` 키워드로 가변임을 명시해야 합니다. 예를 들어 아래 코드는 `let a = 42`를 `let mut a = 42`로 바꾸지 않으면 컴파일되지 않습니다.
```rust
fn main() {
    let a = 42; // 아래 대입을 허용하려면 let mut a = 42로 바꿔야 한다
    a = 43;     // 위를 바꾸지 않으면 컴파일되지 않음
}
```
- Rust는 같은 변수 이름을 다시 사용하는 섀도잉(shadowing)을 허용합니다.
```rust
fn main() {
    let a = 42;
    {
        let a = 43; // OK: 같은 이름의 다른 변수
    }
    // a = 43; // 허용되지 않음
    let a = 43; // OK: 새 변수에 다시 바인딩
}
```
