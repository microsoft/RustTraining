## Iterator Power Tools Reference

> **What you'll learn:** Advanced iterator combinators beyond `filter`/`map`/`collect` — `enumerate`, `zip`, `chain`, `flat_map`, `scan`, `windows`, and `chunks`. Essential for replacing C-style indexed `for` loops with safe, expressive Rust iterators.

The basic `filter`/`map`/`collect` chain covers many cases, but Rust's iterator library
is far richer. This section covers the tools you'll reach for daily — especially when
translating C loops that manually track indices, accumulate results, or process
data in fixed-size chunks.

### Quick Reference Table

| Method | C Equivalent | What it does | Returns |
|--------|-------------|-------------|---------|
| `enumerate()` | `for (int i=0; ...)` | Pairs each element with its index | `(usize, T)` |
| `zip(other)` | Parallel arrays with same index | Pairs elements from two iterators | `(A, B)` |
| `chain(other)` | Process array1 then array2 | Concatenates two iterators | `T` |
| `flat_map(f)` | Nested loops | Maps then flattens one level | `U` |
| `windows(n)` | `for (int i=0; i<len-n+1; i++) &arr[i..i+n]` | Overlapping slices of size `n` | `&[T]` |
| `chunks(n)` | Process `n` elements at a time | Non-overlapping slices of size `n` | `&[T]` |
| `fold(init, f)` | `int acc = init; for (...) acc = f(acc, x);` | Reduce to single value | `Acc` |
| `scan(init, f)` | Running accumulator with output | Like `fold` but yields intermediate results | `Option<B>` |
| `take(n)` / `skip(n)` | Start loop at offset / limit | First `n` / skip first `n` elements | `T` |
| `take_while(f)` / `skip_while(f)` | `while (pred) {...}` | Take/skip while predicate holds | `T` |
| `peekable()` | Lookahead with `arr[i+1]` | Allows `.peek()` without consuming | `T` |
| `step_by(n)` | `for (i=0; i<len; i+=n)` | Take every nth element | `T` |
| `unzip()` | Split parallel arrays | Collect pairs into two collections | `(A, B)` |
| `sum()` / `product()` | Accumulate sum/product | Reduce with `+` or `*` | `T` |
| `min()` / `max()` | Find extremes | Return `Option<T>` | `Option<T>` |
| `any(f)` / `all(f)` | `bool found = false; for (...) ...` | Short-circuit boolean search | `bool` |
| `position(f)` | `for (i=0; ...) if (pred) return i;` | Index of first match | `Option<usize>` |

### `enumerate` — Index + Value (replaces C index loops)

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

### `zip` — Parallel Iteration (replaces parallel array loops)

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

### `chain` — Concatenate Iterators

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

### `flat_map` — Flatten Nested Results

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

### `windows` and `chunks` — Sliding and Fixed-Size Groups

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

### `fold` and `scan` — Accumulation

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

### Exercise: Sensor Data Pipeline

Given raw sensor readings (one per line, format `"sensor_name:value:unit"`), write an
iterator pipeline that:
1. Parses each line into `(name, f64, unit)`
2. Filters out readings below a threshold
3. Groups by sensor name using `fold` into a `HashMap`
4. Prints the average reading per sensor

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

<details><summary>Solution (click to expand)</summary>

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


# Rust iterators
- The ```Iterator``` trait is used to implement iteration over user-defined types (https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)
    - In the example, we'll implement an iterator for the Fibonacci sequence, which starts with 1, 1, 2, ... and the successor is the sum of the previous two numbers
    - The ```associated type``` in the ```Iterator``` (```type Item = u32;```) defines the output type from our iterator (```u32```)
    - The ```next()``` method simply contains the logic for implementing our iterator. In this case, all state information is available in the ```Fibonacci``` structure
    - We could have implemented another trait called ```IntoIterator``` to implement the ```into_iter()``` method for more specialized iterators
    - https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=ab367dc2611e1b5a0bf98f1185b38f3f


