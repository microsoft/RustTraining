### Rust array type

> **What you'll learn:** Rust's core data structures — arrays, tuples, slices, strings, structs, `Vec`, and `HashMap`. This is a dense chapter; focus on understanding `String` vs `&str` and how structs work. You'll revisit references and borrowing in depth in chapter 7.

- Arrays contain a fixed number of elements of the same type
    - Like all other Rust types, arrays are immutable by default (unless mut is used)
    - Arrays are indexed using [] and are bounds checked. The len() method can be used to obtain the length of the array
```rust
    fn get_index(y : usize) -> usize {
        y+1        
    }
    
    fn main() {
        // Initializes an array of 3 elements and sets all to 42
        let a : [u8; 3] = [42; 3];
        // Alternative syntax
        // let a = [42u8, 42u8, 42u8];
        for x in a {
            println!("{x}");
        }
        let y = get_index(a.len());
        // Commenting out the below will cause a panic
        //println!("{}", a[y]);
    }
```

----
### Rust array type continued
- Arrays can be nested
    - Rust has several built-in formatters for printing. In the below, the ```:?``` is the ```debug``` print formatter. The ```:#?``` formatter can be used for ```pretty print```. These formatters can be customized per type (more on this later) 
```rust
    fn main() {
        let a = [
            [40, 0], // Define a nested array
            [41, 0],
            [42, 1],
        ];
        for x in a {
            println!("{x:?}");
        }
    }
```
----
### Rust tuples
- Tuples have a fixed size and can group arbitrary types into a single compound type
    - The constituent types can be indexed by their relative location (.0, .1, .2, ...). An empty tuple, i.e., () is called the unit value and is the equivalent of a void return value
    - Rust supports tuple destructuring to make it easy to bind variables to individual elements
```rust
fn get_tuple() -> (u32, bool) {
    (42, true)        
}

fn main() {
   let t : (u8, bool) = (42, true);
   let u : (u32, bool) = (43, false);
   println!("{}, {}", t.0, t.1);
   println!("{}, {}", u.0, u.1);
   let (num, flag) = get_tuple(); // Tuple destructuring
   println!("{num}, {flag}");
}
```

### Rust references
- References in Rust are roughly equivalent to pointers in C with some key differences
    - It is legal to have any number of read-only (immutable) references to a variable at any point of time. A reference cannot outlive the variable scope (this is a key concept called **lifetime**; discussed in detail later)
    - Only a single writable (mutable) reference to a mutable variable is permitted and it must not overlap with any other reference.
```rust
fn main() {
    let mut a = 42;
    {
        let b = &a;
        let c = b;
        println!("{} {}", *b, *c); // The compiler automatically dereferences *c
        
        let d = &mut a;
        
        /* 
         * Uncommenting the line below would be cause the 
         * program to not compile, because `b` is used 
         * while the mutable reference `d` is live in the current scope
         * 
         * You cannot have a mutable and immutable reference in use in the same scope
         * at the same time!
         */
        // println!("{}", *b);
    }
    let d = &mut a; // Ok: b and c are not in scope
    *d = 43;
}
```

----
# Rust slices
- Rust references can be used to create subsets of arrays
    - Unlike arrays, which have a static fixed length determined at compile time, slices can be of arbitrary size. Internally, slices are implemented as a "fat-pointer" that contains the length of the slice and a pointer to the starting element in the original array
```rust
fn main() {
    let a = [40, 41, 42, 43];
    let b = &a[1..a.len()]; // A slice starting with the second element in the original
    let c = &a[1..]; // Same as the above
    let d = &a[..]; // Same as &a[0..] or &a[0..a.len()]
    println!("{b:?} {c:?} {d:?}");
}
```
----
# Rust constants and statics
- The ```const``` keyword can be used to define a constant value. Constant values are evaluated at **compile time** and are inlined into the program
- The ```static``` keyword is used to define the equivalent of global variables in languages like C/C++ Static variables have an addressable memory location and are created once and last the entire lifetime of the program
```rust
const SECRET_OF_LIFE: u32 = 42;
static GLOBAL_VARIABLE : u32 = 2;
fn main() {
    println!("The secret of life is {}", SECRET_OF_LIFE);
    println!("Value of global variable is {GLOBAL_VARIABLE}")
}
```

----
# Rust strings: String vs &str

- Rust has **two** string types that serve different purposes
    - `String` — owned, heap-allocated, growable (like C's `malloc`'d buffer, or C++'s `std::string`)
    - `&str` — borrowed, lightweight reference (like C's `const char*` with length, or C++'s `std::string_view` — but `&str` is **lifetime-checked** so it can never dangle)
    - Unlike C's null-terminated strings, Rust strings track their length and are guaranteed valid UTF-8

> **For C++ developers:** `String` ≈ `std::string`, `&str` ≈ `std::string_view`. Unlike `std::string_view`, a `&str` is guaranteed valid for its entire lifetime by the borrow checker.

## String vs &str: Owned vs Borrowed

> **Production patterns**: See [JSON handling: nlohmann::json → serde](ch17-2-avoiding-unchecked-indexing.md#json-handling-nlohmannjson--serde) for how string handling works with serde in production code.

| **Aspect** | **C `char*`** | **C++ `std::string`** | **Rust `String`** | **Rust `&str`** |
|------------|--------------|----------------------|-------------------|----------------|
| **Memory** | Manual (`malloc`/`free`) | Heap-allocated, owns buffer | Heap-allocated, auto-freed | Borrowed reference (lifetime-checked) |
| **Mutability** | Always mutable via pointer | Mutable | Mutable with `mut` | Always immutable |
| **Size info** | None (relies on `'\0'`) | Tracks length and capacity | Tracks length and capacity | Tracks length (fat pointer) |
| **Encoding** | Unspecified (usually ASCII) | Unspecified (usually ASCII) | Guaranteed valid UTF-8 | Guaranteed valid UTF-8 |
| **Null terminator** | Required | Required (`c_str()`) | Not used | Not used |

```rust
fn main() {
    // &str - string slice (borrowed, immutable, usually a string literal)
    let greeting: &str = "Hello";  // Points to read-only memory

    // String - owned, heap-allocated, growable
    let mut owned = String::from(greeting);  // Copies data to heap
    owned.push_str(", World!");        // Grow the string
    owned.push('!');                   // Append a single character

    // Converting between String and &str
    let slice: &str = &owned;          // String -> &str (free, just a borrow)
    let owned2: String = slice.to_string();  // &str -> String (allocates)
    let owned3: String = String::from(slice); // Same as above

    // String concatenation (note: + consumes the left operand)
    let hello = String::from("Hello");
    let world = String::from(", World!");
    let combined = hello + &world;  // hello is moved (consumed), world is borrowed
    // println!("{hello}");  // Won't compile: hello was moved

    // Use format! to avoid move issues
    let a = String::from("Hello");
    let b = String::from("World");
    let combined = format!("{a}, {b}!");  // Neither a nor b is consumed

    println!("{combined}");
}
```

## Why You Cannot Index Strings with `[]`
```rust
fn main() {
    let s = String::from("hello");
    // let c = s[0];  // Won't compile! Rust strings are UTF-8, not byte arrays

    // Safe alternatives:
    let first_char = s.chars().next();           // Option<char>: Some('h')
    let as_bytes = s.as_bytes();                 // &[u8]: raw UTF-8 bytes
    let substring = &s[0..1];                    // &str: "h" (byte range, must be valid UTF-8 boundary)

    println!("First char: {:?}", first_char);
    println!("Bytes: {:?}", &as_bytes[..5]);
}
```

## Exercise: String manipulation

🟢 **Starter**
- Write a function `fn count_words(text: &str) -> usize` that counts the number of whitespace-separated words in a string
- Write a function `fn longest_word(text: &str) -> &str` that returns the longest word (hint: you'll need to think about lifetimes -- why does the return type need to be `&str` and not `String`?)

<details><summary>Solution (click to expand)</summary>

```rust
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn longest_word(text: &str) -> &str {
    text.split_whitespace()
        .max_by_key(|word| word.len())
        .unwrap_or("")
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog";
    println!("Word count: {}", count_words(text));       // 9
    println!("Longest word: {}", longest_word(text));     // "jumps"
}
```

</details>

# Rust structs
- The ```struct``` keyword declares a user-defined struct type
    - ```struct``` members can either be named, or anonymous (tuple structs)
- Unlike languages like C++, there's no notion of "data inheritance" in Rust
```rust
fn main() {
    struct MyStruct {
        num: u32,
        is_secret_of_life: bool,
    }
    let x = MyStruct {
        num: 42,
        is_secret_of_life: true,
    };
    let y = MyStruct {
        num: x.num,
        is_secret_of_life: x.is_secret_of_life,
    };
    let z = MyStruct { num: x.num, ..x }; // The .. means copy remaining
    println!("{} {} {}", x.num, y.is_secret_of_life, z.num);
}
```

# Rust tuple structs
- Rust tuple structs are similar to tuples and individual fields don't have names
    - Like tuples, individual elements are accessed using .0, .1, .2, .... A common use case for tuple structs is to wrap primitive types to create custom types. **This can be useful to avoid mixing differing values of the same type**
```rust
struct WeightInGrams(u32);
struct WeightInMilligrams(u32);
fn to_weight_in_grams(kilograms: u32) -> WeightInGrams {
    WeightInGrams(kilograms * 1000)
}

fn to_weight_in_milligrams(w : WeightInGrams) -> WeightInMilligrams  {
    WeightInMilligrams(w.0 * 1000)
}

fn main() {
    let x = to_weight_in_grams(42);
    let y = to_weight_in_milligrams(x);
    // let z : WeightInGrams = x;  // Won't compile: x was moved into to_weight_in_milligrams()
    // let a : WeightInGrams = y;   // Won't compile: type mismatch (WeightInMilligrams vs WeightInGrams)
}
```


**Note**: The `#[derive(...)]` attribute automatically generates common trait implementations for structs and enums. You'll see this used throughout the course:
```rust
#[derive(Debug, Clone, PartialEq)]
struct Point { x: i32, y: i32 }

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{:?}", p);           // Debug: works because of #[derive(Debug)]
    let p2 = p.clone();           // Clone: works because of #[derive(Clone)]
    assert_eq!(p, p2);            // PartialEq: works because of #[derive(PartialEq)]
}
```
We'll cover the trait system in depth later, but `#[derive(Debug)]` is so useful that you should add it to nearly every `struct` and `enum` you create.

# Rust Vec type
- The ```Vec<T>``` type implements a dynamic heap allocated buffer (similar to manually managed `malloc`/`realloc` arrays in C, or C++'s `std::vector`)
    - Unlike arrays with fixed size, `Vec` can grow and shrink at runtime
    - `Vec` owns its data and automatically manages memory allocation/deallocation
- Common operations: `push()`, `pop()`, `insert()`, `remove()`, `len()`, `capacity()`
```rust
fn main() {
    let mut v = Vec::new();    // Empty vector, type inferred from usage
    v.push(42);                // Add element to end - Vec<i32>
    v.push(43);                
    
    // Safe iteration (preferred)
    for x in &v {              // Borrow elements, don't consume vector
        println!("{x}");
    }
    
    // Initialization shortcuts
    let mut v2 = vec![1, 2, 3, 4, 5];           // Macro for initialization
    let v3 = vec![0; 10];                       // 10 zeros
    
    // Safe access methods (preferred over indexing)
    match v2.get(0) {
        Some(first) => println!("First: {first}"),
        None => println!("Empty vector"),
    }
    
    // Useful methods
    println!("Length: {}, Capacity: {}", v2.len(), v2.capacity());
    if let Some(last) = v2.pop() {             // Remove and return last element
        println!("Popped: {last}");
    }
    
    // Dangerous: direct indexing (can panic!)
    // println!("{}", v2[100]);  // Would panic at runtime
}
```
> **Production patterns**: See [Avoiding unchecked indexing](ch17-2-avoiding-unchecked-indexing.md#avoiding-unchecked-indexing) for safe `.get()` patterns from production Rust code.

# Rust HashMap type
- ```HashMap``` implements generic ```key``` -> ```value``` lookups (a.k.a. ```dictionary``` or ```map```)
```rust
fn main() {
    use std::collections::HashMap;  // Need explicit import, unlike Vec
    let mut map = HashMap::new();       // Allocate an empty HashMap
    map.insert(40, false);  // Type is inferred as int -> bool
    map.insert(41, false);
    map.insert(42, true);
    for (key, value) in map {
        println!("{key} {value}");
    }
    let map = HashMap::from([(40, false), (41, false), (42, true)]);
    if let Some(x) = map.get(&43) {
        println!("43 was mapped to {x:?}");
    } else {
        println!("No mapping was found for 43");
    }
    let x = map.get(&43).or(Some(&false));  // Default value if key isn't found
    println!("{x:?}"); 
}
```

# Exercise: Vec and HashMap

🟢 **Starter**
- Create a ```HashMap<u32, bool>``` with a few entries (make sure that some values are ```true``` and others are ```false```). Loop over all elements in the hashmap and put the keys into one ```Vec``` and the values into another

<details><summary>Solution (click to expand)</summary>

```rust
use std::collections::HashMap;

fn main() {
    let map = HashMap::from([(1, true), (2, false), (3, true), (4, false)]);
    let mut keys = Vec::new();
    let mut values = Vec::new();
    for (k, v) in &map {
        keys.push(*k);
        values.push(*v);
    }
    println!("Keys:   {keys:?}");
    println!("Values: {values:?}");

    // Alternative: use iterators with unzip()
    let (keys2, values2): (Vec<u32>, Vec<bool>) = map.into_iter().unzip();
    println!("Keys (unzip):   {keys2:?}");
    println!("Values (unzip): {values2:?}");
}
```

</details>

---

## Deep Dive: C++ References vs Rust References

> **For C++ developers:** C++ programmers often assume Rust `&T` works like C++ `T&`. While superficially similar, there are fundamental differences that cause confusion. C developers can skip this section — Rust references are covered in [Ownership and Borrowing](ch07-ownership-and-borrowing.md).

#### 1. No Rvalue References or Universal References

In C++, `&&` has two meanings depending on context:

```cpp
// C++: && means different things:
int&& rref = 42;           // Rvalue reference — binds to temporaries
void process(Widget&& w);   // Rvalue reference — caller must std::move

// Universal (forwarding) reference — deduced template context:
template<typename T>
void forward(T&& arg) {     // NOT an rvalue ref! Deduced as T& or T&&
    inner(std::forward<T>(arg));  // Perfect forwarding
}
```

**In Rust: none of this exists.** `&&` is simply the logical AND operator.

```rust
// Rust: && is just boolean AND
let a = true && false; // false

// Rust has NO rvalue references, no universal references, no perfect forwarding.
// Instead:
//   - Move is the default for non-Copy types (no std::move needed)
//   - Generics + trait bounds replace universal references
//   - No temporary-binding distinction — values are values

fn process(w: Widget) { }      // Takes ownership (like C++ value param + implicit move)
fn process_ref(w: &Widget) { } // Borrows immutably (like C++ const T&)
fn process_mut(w: &mut Widget) { } // Borrows mutably (like C++ T&, but exclusive)
```

| C++ Concept | Rust Equivalent | Notes |
|-------------|-----------------|-------|
| `T&` (lvalue ref) | `&T` or `&mut T` | Rust splits into shared vs exclusive |
| `T&&` (rvalue ref) | Just `T` | Take by value = take ownership |
| `T&&` in template (universal ref) | `impl Trait` or `<T: Trait>` | Generics replace forwarding |
| `std::move(x)` | `x` (just use it) | Move is the default |
| `std::forward<T>(x)` | No equivalent needed | No universal references to forward |

#### 2. Moves Are Bitwise — No Move Constructors

In C++, moving is a *user-defined operation* (move constructor / move assignment). In Rust, moving is always a **bitwise memcpy** of the value, and the source is invalidated:

```rust
// Rust move = memcpy the bytes, mark source as invalid
let s1 = String::from("hello");
let s2 = s1; // Bytes of s1 are copied to s2's stack slot
              // s1 is now invalid — compiler enforces this
// println!("{s1}"); // ❌ Compile error: value used after move
```

```cpp
// C++ move = call the move constructor (user-defined!)
std::string s1 = "hello";
std::string s2 = std::move(s1); // Calls string's move ctor
// s1 is now a "valid but unspecified state" zombie
std::cout << s1; // Compiles! Prints... something (empty string, usually)
```

**Consequences**:
- Rust has no Rule of Five (no copy ctor, move ctor, copy=, move=, destructor to define)
- No moved-from "zombie" state — the compiler simply prevents access
- No `noexcept` considerations for moves — bitwise copy can't throw

#### 3. Auto-Deref: The Compiler Sees Through Indirection

Rust automatically dereferences through multiple layers of pointers/wrappers via the `Deref` trait. This has no C++ equivalent:

```rust
use std::sync::{Arc, Mutex};

// Nested wrapping: Arc<Mutex<Vec<String>>>
let data = Arc::new(Mutex::new(vec!["hello".to_string()]));

// In C++, you'd need explicit unlocking and manual dereferencing at each layer.
// In Rust, the compiler auto-derefs through Arc → Mutex → MutexGuard → Vec:
let guard = data.lock().unwrap(); // Arc auto-derefs to Mutex
let first: &str = &guard[0];      // MutexGuard→Vec (Deref), Vec[0] (Index),
                                   // &String→&str (Deref coercion)
println!("First: {first}");

// Method calls also auto-deref:
let boxed_string = Box::new(String::from("hello"));
println!("Length: {}", boxed_string.len());  // Box→String, then String::len()
// No need for (*boxed_string).len() or boxed_string->len()
```

**Deref coercion** also applies to function arguments — the compiler inserts dereferences to make types match:

```rust
fn greet(name: &str) {
    println!("Hello, {name}");
}

fn main() {
    let owned = String::from("Alice");
    let boxed = Box::new(String::from("Bob"));
    let arced = std::sync::Arc::new(String::from("Carol"));

    greet(&owned);  // &String → &str  (1 deref coercion)
    greet(&boxed);  // &Box<String> → &String → &str  (2 deref coercions)
    greet(&arced);  // &Arc<String> → &String → &str  (2 deref coercions)
    greet("Dave");  // &str already — no coercion needed
}
// In C++ you'd need .c_str() or explicit conversions for each case.
```

**The Deref chain**: When you call `x.method()`, Rust's method resolution
tries the receiver type `T`, then `&T`, then `&mut T`. If no match, it
dereferences via the `Deref` trait and repeats with the target type.
This continues through multiple layers — which is why `Box<Vec<T>>`
"just works" like a `Vec<T>`. Deref *coercion* (for function arguments)
is a separate but related mechanism that automatically converts `&Box<String>`
to `&str` by chaining `Deref` impls.

#### 4. No Null References, No Optional References

```cpp
// C++: references can't be null, but pointers can, and the distinction is blurry
Widget& ref = *ptr;  // If ptr is null → UB
Widget* opt = nullptr;  // "optional" reference via pointer
```

```rust
// Rust: references are ALWAYS valid — guaranteed by the borrow checker
// No way to create a null or dangling reference in safe code
let r: &i32 = &42; // Always valid

// "Optional reference" is explicit:
let opt: Option<&Widget> = None; // Clear intent, no null pointer
if let Some(w) = opt {
    w.do_something(); // Only reachable when present
}
```

#### 5. References Cannot Be Reseated

```cpp
// C++: a reference is an alias — it can't be rebound
int a = 1, b = 2;
int& r = a;
r = b;  // This ASSIGNS b's value to a — it does NOT rebind r!
// a is now 2, r still refers to a
```

```rust
// Rust: let bindings can shadow, but references follow different rules
let a = 1;
let b = 2;
let r = &a;
// r = &b;   // ❌ Cannot assign to immutable variable
let r = &b;  // ✅ But you can SHADOW r with a new binding
             // The old binding is gone, not reseated

// With mut:
let mut r = &a;
r = &b;      // ✅ r now points to b — this IS rebinding (not assignment through)
```

> **Mental model**: In C++, a reference is a permanent alias for one object.
> In Rust, a reference is a value (a pointer with lifetime guarantees) that
> follows normal variable binding rules — immutable by default, rebindable
> only if declared `mut`.
