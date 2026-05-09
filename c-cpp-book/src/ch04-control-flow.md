<a id="rust-if-keyword"></a>
# Rust의 if 키워드

> **이 장에서 배우는 것:** Rust의 제어 흐름 구성 요소인 `if`/`else`, `loop`/`while`/`for`, `match`를 살펴보고, 이것이 C/C++과 어떻게 다른지 이해합니다. 핵심은 Rust의 많은 제어 흐름이 값을 반환하는 **표현식**이라는 점입니다.

- Rust에서 `if`는 단순한 문장이 아니라 **표현식**이기도 합니다. 즉 값을 계산해 대입에 사용할 수 있습니다. 물론 일반적인 문장처럼도 동작합니다. [▶ 실행해 보기](https://play.rust-lang.org/)

```rust
fn main() {
    let x = 42;
    if x < 42 {
        println!("Smaller than the secret of life");
    } else if x == 42 {
        println!("Is equal to the secret of life");
    } else {
        println!("Larger than the secret of life");
    }
    let is_secret_of_life = if x == 42 { true } else { false };
    println!("{}", is_secret_of_life);
}
```

<a id="rust-loops-using-while-and-for"></a>
# while과 for를 이용한 반복문
- `while` 키워드는 조건식이 참인 동안 반복할 때 사용합니다.
```rust
fn main() {
    let mut x = 40;
    while x != 42 {
        x += 1;
    }
}
```
- `for` 키워드는 범위나 이터러블을 순회할 때 사용합니다.
```rust
fn main() {
    // 43은 출력되지 않는다. 마지막 값까지 포함하려면 40..=43을 사용
    for x in 40..43 {
        println!("{}", x);
    }
}
```

<a id="rust-loops-using-loop"></a>
# loop를 이용한 반복문
- `loop` 키워드는 `break`를 만날 때까지 도는 무한 반복문을 만듭니다.
```rust
fn main() {
    let mut x = 40;
    // 여기를 'here: loop'로 바꾸면 루프에 라벨을 붙일 수 있다
    loop {
        if x == 42 {
            break; // break x; 를 쓰면 x 값을 반환할 수 있다
        }
        x += 1;
    }
}
```
- `break` 문에는 선택적으로 표현식을 붙일 수 있고, 그 값은 `loop` 표현식의 결과가 됩니다.
- `continue` 키워드는 반복문의 처음으로 돌아갑니다.
- 루프 라벨은 `break`나 `continue`와 함께 쓸 수 있으며, 중첩 반복문을 다룰 때 특히 유용합니다.

<a id="rust-expression-blocks"></a>
# Rust 표현식 블록
- Rust의 표현식 블록은 `{}` 안에 여러 표현식을 묶어 둔 것입니다. 블록의 값은 마지막 표현식의 값이 됩니다.
```rust
fn main() {
    let x = {
        let y = 40;
        y + 2 // 주의: ; 를 붙이면 안 된다
    };
    // Python 스타일 자리표시자 출력
    println!("{x}");
}
```
- Rust 스타일에서는 이 특성을 활용해 함수에서 `return` 키워드를 생략하는 경우가 많습니다.
```rust
fn is_secret_of_life(x: u32) -> bool {
    // if x == 42 { true } else { false } 와 동일
    x == 42 // 주의: ; 를 붙이면 안 된다
}
fn main() {
    println!("{}", is_secret_of_life(42));
}
```
