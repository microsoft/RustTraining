<a id="iterator-power-tools-reference"></a>
## 이터레이터 활용 도구 레퍼런스

> **이 장에서 배우는 것:** `filter`/`map`/`collect` 를 넘어서는 고급 이터레이터 조합기들, 즉 `enumerate`, `zip`, `chain`, `flat_map`, `scan`, `windows`, `chunks` 를 배웁니다. C 스타일의 인덱스 기반 `for` 루프를 안전하고 표현력 좋은 Rust 이터레이터로 바꿀 때 핵심이 되는 도구들입니다.

기본적인 `filter`/`map`/`collect` 체인만으로도 많은 문제를 해결할 수 있지만, Rust의 이터레이터 라이브러리는 훨씬 더 풍부합니다. 이 절에서는 특히
인덱스를 직접 관리하거나, 결과를 누적하거나, 데이터를 고정 크기 조각으로 처리하는
C 루프를 Rust로 옮길 때 매일 손이 가게 될 도구들을 정리합니다.

<a id="quick-reference-table"></a>
### 빠른 참조 표

| 메서드 | C 대응 | 설명 | 반환 타입 |
|--------|-------------|-------------|---------|
| `enumerate()` | `for (int i=0; ...)` | 각 원소를 인덱스와 함께 묶음 | `(usize, T)` |
| `zip(other)` | 같은 인덱스를 쓰는 병렬 배열 | 두 이터레이터의 원소를 짝지음 | `(A, B)` |
| `chain(other)` | array1 처리 후 array2 처리 | 두 이터레이터를 이어 붙임 | `T` |
| `flat_map(f)` | 중첩 루프 | 매핑한 뒤 한 단계 평탄화 | `U` |
| `windows(n)` | `for (int i=0; i<len-n+1; i++) &arr[i..i+n]` | 크기 `n` 의 겹치는 슬라이스 | `&[T]` |
| `chunks(n)` | 한 번에 `n` 개씩 처리 | 크기 `n` 의 겹치지 않는 슬라이스 | `&[T]` |
| `fold(init, f)` | `int acc = init; for (...) acc = f(acc, x);` | 단일 값으로 축약 | `Acc` |
| `scan(init, f)` | 출력이 있는 누적 상태 | `fold` 와 비슷하지만 중간 결과를 생성 | `Option<B>` |
| `take(n)` / `skip(n)` | 오프셋에서 시작 / 개수 제한 | 앞의 `n` 개 취하기 / 앞의 `n` 개 건너뛰기 | `T` |
| `take_while(f)` / `skip_while(f)` | `while (pred) {...}` | 조건이 참인 동안 취하기/건너뛰기 | `T` |
| `peekable()` | `arr[i+1]` 식의 미리 보기 | 소비하지 않고 `.peek()` 가능 | `T` |
| `step_by(n)` | `for (i=0; i<len; i+=n)` | n개마다 하나씩 취함 | `T` |
| `unzip()` | 병렬 배열 분리 | pair 컬렉션을 두 컬렉션으로 분리 | `(A, B)` |
| `sum()` / `product()` | 합/곱 누적 | `+` 또는 `*` 로 축약 | `T` |
| `min()` / `max()` | 최솟값/최댓값 찾기 | `Option<T>` 반환 | `Option<T>` |
| `any(f)` / `all(f)` | `bool found = false; for (...) ...` | 단락 평가되는 불리언 탐색 | `bool` |
| `position(f)` | `for (i=0; ...) if (pred) return i;` | 첫 매치의 인덱스 | `Option<usize>` |

### `enumerate` — 인덱스 + 값 (C 스타일 인덱스 루프 대체)

```rust
fn main() {
    let sensors = ["GPU_TEMP", "CPU_TEMP", "FAN_RPM", "PSU_WATT"];

    // C style: for (int i = 0; i < 4; i++) printf("[%d] %s\n", i, sensors[i]);
    for (i, name) in sensors.iter().enumerate() {
        println!("[{i}] {name}");
    }

    // Find the index of a specific sensor
    let gpu_idx = sensors.iter().position(|&s| s == "GPU_TEMP");
    println!("GPU sensor at index: {gpu_idx:?}");  // Some(0)
}
```

### `zip` — 병렬 순회 (병렬 배열 루프 대체)

```rust
fn main() {
    let names = ["accel_diag", "nic_diag", "cpu_diag"];
    let statuses = [true, false, true];
    let durations_ms = [1200, 850, 3400];

    // C: for (int i=0; i<3; i++) printf("%s: %s (%d ms)\n", names[i], ...);
    for ((name, passed), ms) in names.iter().zip(&statuses).zip(&durations_ms) {
        let status = if *passed { "PASS" } else { "FAIL" };
        println!("{name}: {status} ({ms} ms)");
    }
}
```

### `chain` — 이터레이터 이어 붙이기

```rust
fn main() {
    let critical = vec!["ECC error", "Thermal shutdown"];
    let warnings = vec!["Link degraded", "Fan slow"];

    // Process all events in priority order
    let all_events: Vec<_> = critical.iter().chain(warnings.iter()).collect();
    println!("{all_events:?}");
    // ["ECC error", "Thermal shutdown", "Link degraded", "Fan slow"]
}
```

### `flat_map` — 중첩 결과 평탄화

```rust
fn main() {
    let lines = vec!["gpu:42:ok", "nic:99:fail", "cpu:7:ok"];

    // Extract all numeric values from colon-separated lines
    let numbers: Vec<u32> = lines.iter()
        .flat_map(|line| line.split(':'))
        .filter_map(|token| token.parse::<u32>().ok())
        .collect();
    println!("{numbers:?}");  // [42, 99, 7]
}
```

### `windows` 와 `chunks` — 슬라이딩 그룹과 고정 크기 그룹

```rust
fn main() {
    let temps = [65, 68, 72, 71, 75, 80, 78, 76];

    // windows(3): overlapping groups of 3 (like a sliding average)
    // C: for (int i = 0; i <= len-3; i++) avg(arr[i], arr[i+1], arr[i+2]);
    let moving_avg: Vec<f64> = temps.windows(3)
        .map(|w| w.iter().sum::<i32>() as f64 / 3.0)
        .collect();
    println!("Moving avg: {moving_avg:.1?}");

    // chunks(2): non-overlapping groups of 2
    // C: for (int i = 0; i < len; i += 2) process(arr[i], arr[i+1]);
    for pair in temps.chunks(2) {
        println!("Chunk: {pair:?}");
    }

    // chunks_exact(2): same but panics if remainder exists
    // Also: .remainder() gives leftover elements
}
```

### `fold` 와 `scan` — 누적 처리

```rust
fn main() {
    let values = [10, 20, 30, 40, 50];

    // fold: single final result (like C's accumulator loop)
    let sum = values.iter().fold(0, |acc, &x| acc + x);
    println!("Sum: {sum}");  // 150

    // Build a string with fold
    let csv = values.iter()
        .fold(String::new(), |acc, x| {
            if acc.is_empty() { format!("{x}") }
            else { format!("{acc},{x}") }
        });
    println!("CSV: {csv}");  // "10,20,30,40,50"

    // scan: like fold but yields intermediate results
    let running_sum: Vec<i32> = values.iter()
        .scan(0, |state, &x| {
            *state += x;
            Some(*state)
        })
        .collect();
    println!("Running sum: {running_sum:?}");  // [10, 30, 60, 100, 150]
}
```

<a id="exercise-sensor-data-pipeline"></a>
### 연습문제: 센서 데이터 파이프라인

원시 센서 측정값이 한 줄에 하나씩 `"sensor_name:value:unit"` 형식으로 주어졌다고 할 때,
다음을 수행하는 이터레이터 파이프라인을 작성하세요:
1. 각 줄을 `(name, f64, unit)` 으로 파싱한다
2. 임계값보다 낮은 측정값은 걸러낸다
3. `fold` 를 사용해 센서 이름별로 `HashMap` 에 그룹화한다
4. 센서별 평균 측정값을 출력한다

```rust
// Starter code
fn main() {
    let raw_data = vec![
        "gpu_temp:72.5:C",
        "cpu_temp:65.0:C",
        "gpu_temp:74.2:C",
        "fan_rpm:1200.0:RPM",
        "cpu_temp:63.8:C",
        "gpu_temp:80.1:C",
        "fan_rpm:1150.0:RPM",
    ];
    let threshold = 70.0;
    // TODO: Parse, filter values >= threshold, group by name, compute averages
}
```

<details><summary>해답 (클릭하여 펼치기)</summary>

```rust
use std::collections::HashMap;

fn main() {
    let raw_data = vec![
        "gpu_temp:72.5:C",
        "cpu_temp:65.0:C",
        "gpu_temp:74.2:C",
        "fan_rpm:1200.0:RPM",
        "cpu_temp:63.8:C",
        "gpu_temp:80.1:C",
        "fan_rpm:1150.0:RPM",
    ];
    let threshold = 70.0;

    // Parse → filter → group → average
    let grouped = raw_data.iter()
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(3, ':').collect();
            if parts.len() == 3 {
                let value: f64 = parts[1].parse().ok()?;
                Some((parts[0], value, parts[2]))
            } else {
                None
            }
        })
        .filter(|(_, value, _)| *value >= threshold)
        .fold(HashMap::<&str, Vec<f64>>::new(), |mut acc, (name, value, _)| {
            acc.entry(name).or_default().push(value);
            acc
        });

    for (name, values) in &grouped {
        let avg = values.iter().sum::<f64>() / values.len() as f64;
        println!("{name}: avg={avg:.1} ({} readings)", values.len());
    }
}
// Output (order may vary):
// gpu_temp: avg=75.6 (3 readings)
// fan_rpm: avg=1175.0 (2 readings)
```

</details>


<a id="rust-iterators"></a>
# Rust 이터레이터
- ```Iterator``` 트레잇은 사용자 정의 타입에 반복 기능을 구현할 때 사용합니다 (https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)
    - 이 예제에서는 1, 1, 2, ... 로 시작하고 다음 값이 앞의 두 수의 합이 되는 피보나치 수열용 이터레이터를 구현합니다
    - ```Iterator``` 안의 ```associated type``` (```type Item = u32;```) 은 이 이터레이터가 반환하는 출력 타입 (```u32```) 을 정의합니다
    - ```next()``` 메서드는 이터레이터 동작의 핵심 로직을 담습니다. 이 경우 필요한 상태는 모두 ```Fibonacci``` 구조체 안에 있습니다
    - 더 특화된 이터레이터를 위해 ```into_iter()``` 메서드를 제공하는 ```IntoIterator``` 라는 다른 트레잇을 구현할 수도 있습니다
    - https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=ab367dc2611e1b5a0bf98f1185b38f3f


