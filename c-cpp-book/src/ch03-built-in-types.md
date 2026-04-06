# Built-in Rust types

> **What you'll learn:** Rust's fundamental types (`i32`, `u64`, `f64`, `bool`, `char`), type inference, explicit type annotations, and how they compare to C/C++ primitive types. No implicit conversions — Rust requires explicit casts.

- Rust has type inference, but also allows explicit specification of the type 

|  **Description**  |            **Type**            |          **Example**          |
|:-----------------:|:------------------------------:|:-----------------------------:|
| Signed integers   | i8, i16, i32, i64, i128, isize | -1, 42, 1_00_000, 1_00_000i64 |
| Unsigned integers | u8, u16, u32, u64, u128, usize | 0, 42, 42u32, 42u64           |
| Floating point    | f32, f64                       | 0.0, 0.42                     |
| Unicode           | char                           | 'a', '$'                      |
| Boolean           | bool                           | true, false                   |

- Rust permits arbitrary use of ```_``` between numbers for ease of reading
----
### Rust type specification and assignment
- Rust uses the ```let``` keyword to assign values to variables. The type of the variable can be optionally specified after a ```:```
```rust
fn main() {
    let x : i32 = 42;
    // These two assignments are logically equivalent
    let y : u32 = 42;
    let z = 42u32;
}
``` 
- Function parameters and return values (if any) require an explicit type. The following takes a u8 parameter and returns u32
```rust
fn foo(x : u8) -> u32
{
    return x as u32 * x as u32;
}
```
- Unused variables are prefixed with ```_``` to avoid compiler warnings
----
# Rust type specification and inference
- Rust can automatically infer the type of the variable based on the context. 
- [▶ Try it in the Rust Playground](https://play.rust-lang.org/)
```rust
fn secret_of_life_u32(x : u32) {
    println!("The u32 secret_of_life is {}", x);
}

fn secret_of_life_u8(x : u8) {
    println!("The u8 secret_of_life is {}", x);
}

fn main() {
    let a = 42; // The let keyword assigns a value; type of a is u32
    let b = 42; // The let keyword assigns a value; inferred type of b is u8
    secret_of_life_u32(a);
    secret_of_life_u8(b);
}
```

# Rust variables and mutability
- Rust variables are **immutable** by default unless the ```mut``` keyword is used to denote that a variable is mutable. For example, the following code will not compile unless the ```let a = 42``` is changed to ```let mut a = 42```
```rust
fn main() {
    let a = 42; // Must be changed to let mut a = 42 to permit the assignment below 
    a = 43;  // Will not compile unless the above is changed
}
```
- Rust permits the reuse of the variable names (shadowing)
```rust
fn main() {
    let a = 42;
    {
        let a = 43; //OK: Different variable with the same name
    }
    // a = 43; // Not permitted
    let a = 43; // Ok: New variable and assignment
}
```



