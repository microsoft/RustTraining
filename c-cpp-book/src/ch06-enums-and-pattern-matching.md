# Rust enum types

> **What you'll learn:** Rust enums as discriminated unions (tagged unions done right), `match` for exhaustive pattern matching, and how enums replace C++ class hierarchies and C tagged unions with compiler-enforced safety.

- Enum types are discriminated unions, i.e., they are a sum type of several possible different types with a tag that identifies the specific variant
    - For C developers: enums in Rust can carry data (tagged unions done right — the compiler tracks which variant is active)
    - For C++ developers: Rust enums are like `std::variant` but with exhaustive pattern matching, no `std::get` exceptions, and no `std::visit` boilerplate
    - The size of the `enum` is that of the largest possible type. The individual variants are not related to one another and can have completely different types
    - `enum` types are one of the most powerful features of the language — they replace entire class hierarchies in C++ (more on this in the Case Studies)
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
    let c : Numbers = a; // Ok -- the type of a is Numbers
    let d : Numbers = b; // Ok -- the type of b is Numbers
}
```
----
# Rust match statement
- The Rust ```match``` is the equivalent of the C "switch" on steroids
    - ```match``` can be used for pattern matching on simple data types, ```struct```, ```enum```
    - The ```match``` statement must be exhaustive, i.e., they must cover all possible cases for a given ```type```. The ```_``` can be used a wildcard for the "all else" case
    - ```match``` can yield a value, but all arms (```=>```) must return a value of the same type

```rust
fn main() {
    let x = 42;
    // In this case, the _ covers all numbers except the ones explicitly listed
    let is_secret_of_life = match x {
        42 => true, // return type is boolean value
        _ => false, // return type boolean value
        // This won't compile because return type isn't boolean
        // _ => 0  
    };
    println!("{is_secret_of_life}");
}
```

# Rust match statement
- ```match``` supports ranges, boolean filters, and ```if``` guard statements
```rust
fn main() {
    let x = 42;
    match x {
        // Note that the =41 ensures the inclusive range
        0..=41 => println!("Less than the secret of life"),
        42 => println!("Secret of life"),
        _ => println!("More than the secret of life"),
    }
    let y = 100;
    match y {
        100 if x == 43 => println!("y is 100% not secret of life"),
        100 if x == 42 => println!("y is 100% secret of life"),
        _ => (),    // Do nothing
    }
}
```

# Rust match statement
- ```match``` and ```enums``` are often combined together
    - The match statement can "bind" the contained value to a variable. Use ```_``` if the value is a don't care
    - The ```matches!``` macro can be used to match to specific variant
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
        Numbers::BiggerNumber(_) | Numbers::EvenBiggerNumber(_) => println!("Some BiggerNumber or EvenBiggerNumber"),
    }
    
    // Boolean test for specific variants
    if matches!(b, Numbers::Zero | Numbers::SmallNumber(_)) {
        println!("Matched Zero or small number");
    }
}
```

# Rust match statement
- ```match``` can also perform matches using destructuring and slices
```rust
fn main() {
    struct Foo {
        x: (u32, bool),
        y: u32
    }
    let f = Foo {x: (42, true), y: 100};
    match f {
        // Capture the value of x into a variable called tuple
        Foo{y: 100, x : tuple} => println!("Matched x: {tuple:?}"),
        _ => ()
    }
    let a = [40, 41, 42];
    match a {
        // Last element of slice must be 42. @ is used to bind the match
        [rest @ .., 42] => println!("{rest:?}"),
        // First element of the slice must be 42. @ is used to bind the match
        [42, rest @ ..] => println!("{rest:?}"),
        _ => (),
    }
}
```

# Exercise: Implement add and subtract using match and enum

🟢 **Starter**

- Write a function that implements arithmetic operations on unsigned 64-bit numbers
- **Step 1**: Define an enum for operations:
```rust
enum Operation {
    Add(u64, u64),
    Subtract(u64, u64),
}
```
- **Step 2**: Define a result enum:
```rust
enum CalcResult {
    Ok(u64),                    // Successful result
    Invalid(String),            // Error message for invalid operations
}
```
- **Step 3**: Implement `calculate(op: Operation) -> CalcResult`
    - For Add: return Ok(sum)
    - For Subtract: return Ok(difference) if first >= second, otherwise Invalid("Underflow")
- **Hint**: Use pattern matching in your function:
```rust
match op {
    Operation::Add(a, b) => { /* your code */ },
    Operation::Subtract(a, b) => { /* your code */ },
}
```

<details><summary>Solution (click to expand)</summary>

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

# Rust associated methods
- ```impl``` can define methods associated for types like ```struct```, ```enum```, etc
    - The methods may optionally take ```self``` as a parameter. ```self``` is conceptually similar to passing a pointer to the struct as the first parameter in C, or ```this``` in C++
    - The reference to ```self``` can be immutable (default: ```&self```), mutable (```&mut self```), or ```self``` (transferring ownership)
    - The ```Self``` keyword can be used a shortcut to imply the type
```rust
struct Point {x: u32, y: u32}
impl Point {
    fn new(x: u32, y: u32) -> Self {
        Point {x, y}
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

# Exercise: Point add and transform

🟡 **Intermediate** — requires understanding move vs borrow from method signatures
- Implement the following associated methods for ```Point```
    - ```add()``` will take another ```Point``` and will increment the x and y values in place (hint: use ```&mut self```)
    - ```transform()``` will consume an existing ```Point``` (hint: use ```self```) and return a new ```Point``` by squaring the x and y

<details><summary>Solution (click to expand)</summary>

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
    println!("After add: x={}, y={}", p1.x, p1.y);           // x=12, y=23
    let p3 = p1.transform();
    println!("After transform: x={}, y={}", p3.x, p3.y);     // x=144, y=529
    // p1 is no longer accessible — transform() consumed it
}
```

</details>

----

