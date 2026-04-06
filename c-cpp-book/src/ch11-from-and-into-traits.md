# Rust From and Into traits

> **What you'll learn:** Rust's type conversion traits — `From<T>` and `Into<T>` for infallible conversions, `TryFrom` and `TryInto` for fallible ones. Implement `From` and get `Into` for free. Replaces C++ conversion operators and constructors.

- ```From``` and ```Into``` are complementary traits to facilitate type conversion
- Types normally implement the ```From``` trait. The ```String::from()``` converts from "&str" to ```String```, and the compiler can automatically derive ```&str.into```
```rust
struct Point {x: u32, y: u32}
// Construct a Point from a tuple
impl From<(u32, u32)> for Point {
    fn from(xy : (u32, u32)) -> Self {
        Point {x : xy.0, y: xy.1}       // Construct Point using the tuple elements
    }
}
fn main() {
    let s = String::from("Rust");
    let x = u32::from(true);
    let p = Point::from((40, 42));
    // let p : Point = (40.42)::into(); // Alternate form of the above
    println!("s: {s} x:{x} p.x:{} p.y {}", p.x, p.y);   
}
```

# Exercise: From and Into
- Implement a ```From``` trait for ```Point``` to convert into a type called ```TransposePoint```. ```TransposePoint``` swaps the ```x``` and ```y``` elements of ```Point```

<details><summary>Solution (click to expand)</summary>

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

    // Using .into() — works automatically when From is implemented
    let p2 = Point { x: 3, y: 7 };
    let tp2: TransposePoint = p2.into();
    println!("Transposed: x={}, y={}", tp2.x, tp2.y);  // x=7, y=3
}
// Output:
// Transposed: x=20, y=10
// Transposed: x=7, y=3
```

</details>

# Rust Default trait
- ```Default``` can be used to implement default values for a type
    - Types can use the ```Derive``` macro with ```Default``` or provide a custom implementation
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
    let x = Point::default();   // Creates a Point{0, 0}
    println!("{x:?}");
    let y = CustomPoint::default();
    println!("{y:?}");
}
```

### Rust Default trait
- ```Default``` trait has several use cases including
    - Performing a partial copy and using default initialization for rest
    - Default alternative for ```Option``` types in methods like ```unwrap_or_default()```
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
    // Override y, but leave rest of elements as the default
    let y = CustomPoint {y: 43, ..CustomPoint::default()};
    println!("{x:?} {y:?}");
    let z : Option<CustomPoint> = None;
    // Try changing the unwrap_or_default() to unwrap()
    println!("{:?}", z.unwrap_or_default());
}
```

### Other Rust type conversions
- Rust doesn't support implicit type conversions and ```as``` can be used for ```explicit``` conversions
- ```as``` should be sparingly used because it's subject to loss of data by narrowing and so forth. In general, it's preferable to use ```into()``` or ```from()``` where possible
```rust
fn main() {
    let f = 42u8;
    // let g : u32 = f;    // Will not compile
    let g = f as u32;      // Ok, but not preferred. Subject to rules around narrowing
    let g : u32 = f.into(); // Most preferred form; infallible and checked by the compiler
    // let k : u8 = g.into();  // Fails to compile; narrowing can result in loss of data
    
    // Attempting a narrowing operation requires use of try_into
    if let Ok(k) = TryInto::<u8>::try_into(g) {
        println!("{k}");
    }
}
```


