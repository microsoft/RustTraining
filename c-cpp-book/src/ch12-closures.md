## Rust closures

> **What you'll learn:** Closures as anonymous functions, the three capture traits (`Fn`, `FnMut`, `FnOnce`), `move` closures, and how Rust closures compare to C++ lambdas — with automatic capture analysis instead of manual `[&]`/`[=]` specifications.

- Closures are anonymous functions that can capture their environment
    - C++ equivalent: lambdas (`[&](int x) { return x + 1; }`)
    - Key difference: Rust closures have **three** capture traits (`Fn`, `FnMut`, `FnOnce`) that the compiler selects automatically
    - C++ capture modes (`[=]`, `[&]`, `[this]`) are manual and error-prone (dangling `[&]`!)
    - Rust's borrow checker prevents dangling captures at compile time
- Closures can be identified by the `||` symbol. The parameters for the types are enclosed within the `||` and can use type inference
- Closures are frequently used in conjunction with iterators (next topic)
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


# Exercise: Closures and capturing

🟡 **Intermediate**

- Create a closure that captures a `String` from the enclosing scope and appends to it (hint: use `move`)
- Create a vector of closures: `Vec<Box<dyn Fn(i32) -> i32>>` containing closures that add 1, multiply by 2, and square the input. Iterate over the vector and apply each closure to the number 5

<details><summary>Solution (click to expand)</summary>

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

# Rust iterators
- Iterators are one of the most powerful features of Rust. They enable very elegant methods for performing operations on collections, including filtering (```filter()```), transformation (```map()```), filter and map (```filter_map()```), searching (```find()```) and much more
- In the example below, the ```|&x| *x >= 42``` is a closure that performs the same comparison. The ```|x| println!("{x}")``` is another closure
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

# Rust iterators
- A key feature of iterators is that most of them are ```lazy```, i.e., they do not do anything until they are evaluated. For example, ```a.iter().filter(|&x| *x >= 42);``` wouldn't have done *anything* without the ```for_each```. The Rust compiler emits an explicit warning when it detects such a situation
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

# Rust iterators
- The ```collect()``` method can be used to gather the results into a separate collection
    - In the below the ```_``` in ```Vec<_>``` is the equivalent of a wildcard character for the type returned by the ```map```. For example, we can even return a ```String``` from ```map``` 
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

# Exercise: Rust iterators

🟢 **Starter**
- Create an integer array composed of odd and even elements. Iterate over the array and split it into two different vectors with even and odd elements in each
- Can this be done in a single pass (hint: use ```partition()```)?

<details><summary>Solution (click to expand)</summary>

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

> **Production patterns**: See [Collapsing assignment pyramids with closures](ch17-3-collapsing-assignment-pyramids.md#collapsing-assignment-pyramids-with-closures) for real iterator chains (`.map().collect()`, `.filter().collect()`, `.find_map()`) from production Rust code.

### Iterator power tools: the methods that replace C++ loops

The following iterator adapters are used *extensively* in production Rust code. C++ has
`<algorithm>` and C++20 ranges, but Rust's iterator chains are more composable
and more commonly used.

#### `enumerate` — index + value (replaces `for (int i = 0; ...)`)

```rust
let sensors = vec!["temp0", "temp1", "temp2"];
for (idx, name) in sensors.iter().enumerate() {
    println!("Sensor {idx}: {name}");
}
// Sensor 0: temp0
// Sensor 1: temp1
// Sensor 2: temp2
```

C++ equivalent: `for (size_t i = 0; i < sensors.size(); ++i) { auto& name = sensors[i]; ... }`

#### `zip` — pair elements from two iterators (replaces parallel index loops)

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

C++ equivalent: `for (size_t i = 0; i < std::min(names.size(), temps.size()); ++i) { ... }`

#### `flat_map` — map + flatten nested collections

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

C++ equivalent: nested `for` loop pushing into a single vector.

#### `chain` — concatenate two iterators

```rust
let critical_gpus = vec!["gpu0", "gpu3"];
let warning_gpus = vec!["gpu1", "gpu5"];

// Process all flagged GPUs, critical first
for gpu in critical_gpus.iter().chain(warning_gpus.iter()) {
    println!("Flagged: {gpu}");
}
```

#### `windows` and `chunks` — sliding/fixed-size views over slices

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

C++ equivalent: manual index arithmetic with `i` and `i+1`/`i+2`.

#### `fold` — accumulate into a single value (replaces `std::accumulate`)

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

#### `scan` — stateful transform (running total, delta detection)

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

#### Quick reference: C++ loop → Rust iterator

| **C++ Pattern** | **Rust Iterator** | **Example** |
|----------------|------------------|------------|
| `for (int i = 0; i < v.size(); i++)` | `.enumerate()` | `v.iter().enumerate()` |
| Parallel iteration with index | `.zip()` | `a.iter().zip(b.iter())` |
| Nested loop → flat result | `.flat_map()` | `vecs.iter().flat_map(\|v\| v.iter())` |
| Concatenate two containers | `.chain()` | `a.iter().chain(b.iter())` |
| Sliding window `v[i..i+n]` | `.windows(n)` | `v.windows(3)` |
| Process in fixed-size groups | `.chunks(n)` | `v.chunks(4)` |
| `std::accumulate` / manual accumulator | `.fold()` | `.fold(init, \|acc, x\| ...)` |
| Running total / delta tracking | `.scan()` | `.scan(state, \|s, x\| ...)` |
| `while (it != end && count < n) { ++it; ++count; }` | `.take(n)` | `.iter().take(5)` |
| `while (it != end && !pred(*it)) { ++it; }` | `.skip_while()` | `.skip_while(\|x\| x < &threshold)` |
| `std::any_of` | `.any()` | `.iter().any(\|x\| x > &limit)` |
| `std::all_of` | `.all()` | `.iter().all(\|x\| x.is_valid())` |
| `std::none_of` | `!.any()` | `!iter.any(\|x\| x.failed())` |
| `std::count_if` | `.filter().count()` | `.filter(\|x\| x > &0).count()` |
| `std::min_element` / `std::max_element` | `.min()` / `.max()` | `.iter().max()` → `Option<&T>` |
| `std::unique` | `.dedup()` (on sorted) | `v.dedup()` (in-place on Vec) |

### Exercise: Iterator chains

Given sensor data as `Vec<(String, f64)>` (name, temperature), write a **single
iterator chain** that:
1. Filters sensors with temp > 80.0
2. Sorts them by temperature (descending)
3. Formats each as `"{name}: {temp}°C [ALARM]"`
4. Collects into `Vec<String>`

Hint: you'll need `.collect()` before `.sort_by()`, since sorting requires a `Vec`.

<details><summary>Solution (click to expand)</summary>

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

# Rust iterators
- The ```Iterator``` trait is used to implement iteration over user-defined types (https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)
    - In the example, we'll implement an iterator for the Fibonacci sequence, which starts with 1, 1, 2, ... and the successor is the sum of the previous two numbers
    - The ```associated type``` in the ```Iterator``` (```type Item = u32;```) defines the output type from our iterator (```u32```)
    - The ```next()``` method simply contains the logic for implementing our iterator. In this case, all state information is available in the ```Fibonacci``` structure
    - We could have implemented another trait called ```IntoIterator``` to implement the ```into_iter()``` method for more specialized iterators
    - [▶ Try it in the Rust Playground](https://play.rust-lang.org/)


