<a id="rust-closures"></a>
## Rust 클로저

> **이 장에서 배우는 것:** 익명 함수로서의 클로저, 세 가지 캡처 트레잇 (`Fn`, `FnMut`, `FnOnce`), `move` 클로저, 그리고 Rust 클로저가 C++ 람다와 어떻게 비교되는지 배웁니다. Rust는 수동 `[&]`/`[=]` 지정 대신 캡처 방식을 자동으로 분석합니다.

- 클로저는 자신의 환경을 캡처할 수 있는 익명 함수입니다
    - C++ 대응 개념: 람다 (`[&](int x) { return x + 1; }`)
    - 핵심 차이: Rust 클로저에는 **세 가지** 캡처 트레잇 (`Fn`, `FnMut`, `FnOnce`)이 있으며, 컴파일러가 이를 자동으로 선택합니다
    - C++ 캡처 모드 (`[=]`, `[&]`, `[this]`)는 수동 지정이라 실수하기 쉽습니다 (dangling `[&]`!)
    - Rust의 borrow checker는 dangling capture를 컴파일 시점에 막아줍니다
- 클로저는 `||` 기호로 알아볼 수 있습니다. 매개변수와 타입은 `||` 안에 쓰며, 타입 추론도 사용할 수 있습니다
- 클로저는 이터레이터와 함께 매우 자주 사용됩니다 (다음 주제)
```rust
fn add_one(x: u32) -> u32 {
    x + 1
}
fn main() {
    let add_one_v1 = |x : u32| {x + 1}; // Explicitly specified type
    let add_one_v2 = |x| {x + 1};   // Type is inferred from call site
    let add_one_v3 = |x| x+1;   // Permitted for single line functions
    println!("{} {} {} {}", add_one(42), add_one_v1(42), add_one_v2(42), add_one_v3(42) );
}
```


<a id="exercise-closures-and-capturing"></a>
# 연습문제: 클로저와 캡처

🟡 **Intermediate**

- 바깥 스코프의 `String`을 캡처해서 뒤에 문자열을 덧붙이는 클로저를 작성하세요 (힌트: `move` 사용)
- 1을 더하는 클로저, 2를 곱하는 클로저, 입력값을 제곱하는 클로저를 담는 `Vec<Box<dyn Fn(i32) -> i32>>`를 만드세요. 그 벡터를 순회하면서 각 클로저를 숫자 5에 적용해 보세요

<details><summary>해답 (클릭하여 펼치기)</summary>

```rust
fn main() {
    // Part 1: Closure that captures and appends to a String
    let mut greeting = String::from("Hello");
    let mut append = |suffix: &str| {
        greeting.push_str(suffix);
    };
    append(", world");
    append("!");
    println!("{greeting}");  // "Hello, world!"

    // Part 2: Vector of closures
    let operations: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x + 1),      // add 1
        Box::new(|x| x * 2),      // multiply by 2
        Box::new(|x| x * x),      // square
    ];

    let input = 5;
    for (i, op) in operations.iter().enumerate() {
        println!("Operation {i} on {input}: {}", op(input));
    }
}
// Output:
// Hello, world!
// Operation 0 on 5: 6
// Operation 1 on 5: 10
// Operation 2 on 5: 25
```

</details>

<a id="rust-iterators"></a>
# Rust 이터레이터
- 이터레이터는 Rust의 가장 강력한 기능 중 하나입니다. 컬렉션에 대한 연산을 매우 우아하게 표현할 수 있으며, 필터링 (```filter()```), 변환 (```map()```), filter and map (```filter_and_map()```), 검색 (```find()```) 등 다양한 작업에 활용됩니다
- 아래 예제의 ```|&x| *x >= 42``` 는 같은 비교를 수행하는 클로저입니다. ```|x| println!("{x}")``` 도 또 다른 클로저입니다
```rust
fn main() {
    let a = [0, 1, 2, 3, 42, 43];
    for x in &a {
        if *x >= 42 {
            println!("{x}");
        }
    }
    // Same as above
    a.iter().filter(|&x| *x >= 42).for_each(|x| println!("{x}"))
}
```

# Rust 이터레이터
- 이터레이터의 핵심 특징 중 하나는 대부분이 ```lazy```하다는 점입니다. 즉, 실제로 평가되기 전까지는 아무 일도 하지 않습니다. 예를 들어 ```a.iter().filter(|&x| *x >= 42);``` 는 뒤의 ```for_each``` 가 없으면 *아무것도* 하지 않습니다. Rust 컴파일러는 이런 상황을 발견하면 명시적인 경고를 출력합니다
```rust
fn main() {
    let a = [0, 1, 2, 3, 42, 43];
    // Add one to each element and print it
    let _ = a.iter().map(|x|x + 1).for_each(|x|println!("{x}"));
    let found = a.iter().find(|&x|*x == 42);
    println!("{found:?}");
    // Count elements
    let count = a.iter().count();
    println!("{count}");
}
```

# Rust 이터레이터
- ```collect()``` 메서드는 결과를 별도의 컬렉션으로 모을 때 사용할 수 있습니다
    - 아래 예제에서 ```Vec<_>``` 의 ```_``` 는 ```map``` 이 반환하는 타입에 대한 와일드카드와 같습니다. 예를 들어 ```map``` 에서 ```String``` 을 반환하는 것도 가능합니다
```rust
fn main() {
    let a = [0, 1, 2, 3, 42, 43];
    let squared_a : Vec<_> = a.iter().map(|x|x*x).collect();
    for x in &squared_a {
        println!("{x}");
    }
    let squared_a_strings : Vec<_> = a.iter().map(|x|(x*x).to_string()).collect();
    // These are actually string representations
    for x in &squared_a_strings {
        println!("{x}");
    }
}
```

<a id="exercise-rust-iterators"></a>
# 연습문제: Rust 이터레이터

🟢 **Starter**
- 홀수와 짝수가 섞인 정수 배열을 만드세요. 배열을 순회하면서 짝수만 담은 벡터와 홀수만 담은 벡터, 이렇게 두 개로 나누어 보세요
- 이 작업을 한 번의 순회로 할 수 있을까요? (힌트: ```partition()``` 사용)

<details><summary>해답 (클릭하여 펼치기)</summary>

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Approach 1: Manual iteration
    let mut evens = Vec::new();
    let mut odds = Vec::new();
    for n in numbers {
        if n % 2 == 0 {
            evens.push(n);
        } else {
            odds.push(n);
        }
    }
    println!("Evens: {evens:?}");
    println!("Odds:  {odds:?}");

    // Approach 2: Single pass with partition()
    let (evens, odds): (Vec<i32>, Vec<i32>) = numbers
        .into_iter()
        .partition(|n| n % 2 == 0);
    println!("Evens (partition): {evens:?}");
    println!("Odds  (partition): {odds:?}");
}
// Output:
// Evens: [2, 4, 6, 8, 10]
// Odds:  [1, 3, 5, 7, 9]
// Evens (partition): [2, 4, 6, 8, 10]
// Odds  (partition): [1, 3, 5, 7, 9]
```

</details>

> **프로덕션 패턴:** 실제 Rust 프로덕션 코드에서 쓰이는 이터레이터 체인 (`.map().collect()`, `.filter().collect()`, `.find_map()`) 예시는 [클로저로 중첩된 대입 피라미드 줄이기](ch17-3-collapsing-assignment-pyramids.md#collapsing-assignment-pyramids-with-closures)를 참고하세요.

<a id="iterator-power-tools-the-methods-that-replace-c-loops"></a>
### 이터레이터 활용 도구: C++ 루프를 대체하는 메서드들

다음 이터레이터 어댑터들은 실제 Rust 코드에서 *매우 자주* 사용됩니다. C++에도
`<algorithm>` 과 C++20 ranges가 있지만, Rust의 이터레이터 체인은 더 조합하기 쉽고
실무에서도 더 흔하게 쓰입니다.

#### `enumerate` — 인덱스 + 값 (`for (int i = 0; ...)` 대체)

```rust
let sensors = vec!["temp0", "temp1", "temp2"];
for (idx, name) in sensors.iter().enumerate() {
    println!("Sensor {idx}: {name}");
}
// Sensor 0: temp0
// Sensor 1: temp1
// Sensor 2: temp2
```

C++ 대응: `for (size_t i = 0; i < sensors.size(); ++i) { auto& name = sensors[i]; ... }`

#### `zip` — 두 이터레이터의 원소를 짝지어 묶기 (병렬 인덱스 루프 대체)

```rust
let names = ["gpu0", "gpu1", "gpu2"];
let temps = [72.5, 68.0, 75.3];

let report: Vec<String> = names.iter()
    .zip(temps.iter())
    .map(|(name, temp)| format!("{name}: {temp}°C"))
    .collect();
println!("{report:?}");
// ["gpu0: 72.5°C", "gpu1: 68.0°C", "gpu2: 75.3°C"]

// Stops at the shorter iterator — no out-of-bounds risk
```

C++ 대응: `for (size_t i = 0; i < std::min(names.size(), temps.size()); ++i) { ... }`

#### `flat_map` — 변환 후 중첩 컬렉션 평탄화

```rust
// Each GPU has multiple PCIe BDFs; collect all BDFs across all GPUs
let gpu_bdfs = vec![
    vec!["0000:01:00.0", "0000:02:00.0"],
    vec!["0000:41:00.0"],
    vec!["0000:81:00.0", "0000:82:00.0"],
];

let all_bdfs: Vec<&str> = gpu_bdfs.iter()
    .flat_map(|bdfs| bdfs.iter().copied())
    .collect();
println!("{all_bdfs:?}");
// ["0000:01:00.0", "0000:02:00.0", "0000:41:00.0", "0000:81:00.0", "0000:82:00.0"]
```

C++ 대응: 중첩 `for` 루프를 돌며 하나의 벡터에 `push` 하는 패턴.

#### `chain` — 두 이터레이터 이어 붙이기

```rust
let critical_gpus = vec!["gpu0", "gpu3"];
let warning_gpus = vec!["gpu1", "gpu5"];

// Process all flagged GPUs, critical first
for gpu in critical_gpus.iter().chain(warning_gpus.iter()) {
    println!("Flagged: {gpu}");
}
```

#### `windows` 와 `chunks` — 슬라이딩/고정 크기 슬라이스 뷰

```rust
let temps = [70, 72, 75, 73, 71, 68, 65];

// windows(3): sliding window of size 3 — detect trends
let rising = temps.windows(3)
    .any(|w| w[0] < w[1] && w[1] < w[2]);
println!("Rising trend detected: {rising}"); // true (70 < 72 < 75)

// chunks(2): fixed-size groups — process in pairs
for pair in temps.chunks(2) {
    println!("Pair: {pair:?}");
}
// Pair: [70, 72]
// Pair: [75, 73]
// Pair: [71, 68]
// Pair: [65]       ← last chunk can be smaller
```

C++ 대응: `i` 와 `i+1`/`i+2` 를 직접 계산하는 수동 인덱스 연산.

#### `fold` — 단일 값으로 누적하기 (`std::accumulate` 대체)

```rust
let errors = vec![
    ("gpu0", 3u32),
    ("gpu1", 0),
    ("gpu2", 7),
    ("gpu3", 1),
];

// Count total errors and build summary in one pass
let (total, summary) = errors.iter().fold(
    (0u32, String::new()),
    |(count, mut s), (name, errs)| {
        if *errs > 0 {
            s.push_str(&format!("{name}:{errs} "));
        }
        (count + errs, s)
    },
);
println!("Total errors: {total}, details: {summary}");
// Total errors: 11, details: gpu0:3 gpu2:7 gpu3:1
```

#### `scan` — 상태를 가진 변환 (누적 합, 변화량 감지)

```rust
let readings = [100, 105, 103, 110, 108];

// Compute deltas between consecutive readings
let deltas: Vec<i32> = readings.iter()
    .scan(None::<i32>, |prev, &val| {
        let delta = prev.map(|p| val - p);
        *prev = Some(val);
        Some(delta)
    })
    .flatten()  // Remove the initial None
    .collect();
println!("Deltas: {deltas:?}"); // [5, -2, 7, -2]
```

#### 빠른 참조: C++ 루프 → Rust 이터레이터

| **C++ 패턴** | **Rust 이터레이터** | **예시** |
|----------------|------------------|------------|
| `for (int i = 0; i < v.size(); i++)` | `.enumerate()` | `v.iter().enumerate()` |
| 인덱스를 이용한 병렬 순회 | `.zip()` | `a.iter().zip(b.iter())` |
| 중첩 루프 → 평탄한 결과 | `.flat_map()` | `vecs.iter().flat_map(\|v\| v.iter())` |
| 두 컨테이너 이어 붙이기 | `.chain()` | `a.iter().chain(b.iter())` |
| 슬라이딩 윈도우 `v[i..i+n]` | `.windows(n)` | `v.windows(3)` |
| 고정 크기 그룹 단위 처리 | `.chunks(n)` | `v.chunks(4)` |
| `std::accumulate` / 수동 누산기 | `.fold()` | `.fold(init, \|acc, x\| ...)` |
| 누적 합 / 변화량 추적 | `.scan()` | `.scan(state, \|s, x\| ...)` |
| `while (it != end && count < n) { ++it; ++count; }` | `.take(n)` | `.iter().take(5)` |
| `while (it != end && !pred(*it)) { ++it; }` | `.skip_while()` | `.skip_while(\|x\| x < &threshold)` |
| `std::any_of` | `.any()` | `.iter().any(\|x\| x > &limit)` |
| `std::all_of` | `.all()` | `.iter().all(\|x\| x.is_valid())` |
| `std::none_of` | `!.any()` | `!iter.any(\|x\| x.failed())` |
| `std::count_if` | `.filter().count()` | `.filter(\|x\| x > &0).count()` |
| `std::min_element` / `std::max_element` | `.min()` / `.max()` | `.iter().max()` → `Option<&T>` |
| `std::unique` | `.dedup()` (정렬된 경우) | `v.dedup()` (Vec에서 제자리 처리) |

<a id="exercise-iterator-chains"></a>
### 연습문제: 이터레이터 체인

센서 데이터가 `Vec<(String, f64)>` (이름, 온도) 형태로 주어졌을 때, 다음을 수행하는 **하나의
이터레이터 체인**을 작성하세요:
1. 온도가 80.0보다 큰 센서만 필터링한다
2. 온도 기준 내림차순으로 정렬한다
3. 각 항목을 `"{name}: {temp}°C [ALARM]"` 형식으로 만든다
4. 결과를 `Vec<String>` 으로 수집한다

힌트: 정렬은 `Vec` 가 필요하므로 `.sort_by()` 전에 `.collect()` 가 필요합니다.

<details><summary>해답 (클릭하여 펼치기)</summary>

```rust
fn alarm_report(sensors: &[(String, f64)]) -> Vec<String> {
    let mut hot: Vec<_> = sensors.iter()
        .filter(|(_, temp)| *temp > 80.0)
        .collect();
    hot.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    hot.iter()
        .map(|(name, temp)| format!("{name}: {temp}°C [ALARM]"))
        .collect()
}

fn main() {
    let sensors = vec![
        ("gpu0".to_string(), 72.5),
        ("gpu1".to_string(), 85.3),
        ("gpu2".to_string(), 91.0),
        ("gpu3".to_string(), 78.0),
        ("gpu4".to_string(), 88.7),
    ];
    for line in alarm_report(&sensors) {
        println!("{line}");
    }
}
// Output:
// gpu2: 91°C [ALARM]
// gpu4: 88.7°C [ALARM]
// gpu1: 85.3°C [ALARM]
```

</details>

----

# Rust 이터레이터
- ```Iterator``` 트레잇은 사용자 정의 타입에 반복 기능을 구현할 때 사용합니다 (https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)
    - 이 예제에서는 1, 1, 2, ... 로 시작하고 다음 값이 앞의 두 수의 합이 되는 피보나치 수열용 이터레이터를 구현합니다
    - ```Iterator``` 안의 ```associated type``` (```type Item = u32;```) 은 이 이터레이터가 반환하는 출력 타입 (```u32```) 을 정의합니다
    - ```next()``` 메서드는 이터레이터의 동작 로직을 담습니다. 이 경우 필요한 상태는 모두 ```Fibonacci``` 구조체 안에 있습니다
    - 더 특화된 이터레이터를 위해 ```into_iter()``` 메서드를 제공하는 ```IntoIterator``` 라는 다른 트레잇을 구현할 수도 있습니다
    - [▶ Rust Playground에서 실행해 보기](https://play.rust-lang.org/)


