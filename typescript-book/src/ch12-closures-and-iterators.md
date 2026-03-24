# Closures and Iterators

## Closures

Closures in Rust are anonymous functions that capture their environment — similar to arrow
functions in TypeScript.

🟦 **TypeScript**
```typescript
const add = (a: number, b: number): number => a + b;
const numbers = [3, 1, 4, 1, 5];
const doubled = numbers.map(n => n * 2);
```

🦀 **Rust**
```rust
let add = |a: i32, b: i32| -> i32 { a + b };
let numbers = vec![3, 1, 4, 1, 5];
let doubled: Vec<i32> = numbers.iter().map(|n| n * 2).collect();
```

### Closure Syntax Variations

```rust
let verbose = |x: i32| -> i32 { x + 1 };  // fully annotated
let inferred = |x| x + 1;                   // types inferred from usage
let multiline = |x| {
    let y = x * 2;
    y + 1
};
```

### Capturing — The Key Difference

In TypeScript, closures always capture by reference. In Rust, closures can capture in three
ways, which map to three traits:

| Trait | Capture mode | TypeScript analogy |
|-------|-------------|-------------------|
| `Fn` | Immutable borrow (`&T`) | Reading a const |
| `FnMut` | Mutable borrow (`&mut T`) | Modifying a `let` variable |
| `FnOnce` | Move (takes ownership) | N/A |

```rust
let name = String::from("Alice");

// Fn — borrows name immutably
let greet = || println!("Hello, {name}");
greet();
greet();         // ✅ can call multiple times
println!("{name}"); // ✅ name still accessible

// FnMut — borrows count mutably
let mut count = 0;
let mut increment = || { count += 1; };
increment();
increment();

// FnOnce — moves name into the closure
let consume = move || println!("Consumed: {name}");
consume();
// println!("{name}"); // ❌ name was moved
```

💡 **Key insight**: The compiler infers which trait a closure implements based on how it uses
captured variables. `move` forces ownership transfer regardless.

## Iterators

Rust iterators are lazy chains — like RxJS or lodash chains — that compile down to the same
machine code as hand-written loops.

### The Iterator Trait

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

### Creating Iterators

```rust
let v = vec![1, 2, 3];

v.iter()       // yields &i32 (borrows)
v.iter_mut()   // yields &mut i32 (mutable borrows)
v.into_iter()  // yields i32 (consumes the vec)
```

### Iterator Adapters (Lazy)

These transform an iterator without consuming it:

```rust
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let result: Vec<i32> = numbers.iter()
    .filter(|&&n| n % 2 == 0)    // keep even numbers
    .map(|&n| n * n)              // square them
    .take(3)                      // first 3 results
    .collect();                   // materialize

// result = [4, 16, 36]
```

### TypeScript ↔ Rust Iterator Method Map

| TypeScript (Array) | Rust (Iterator) | Notes |
|-------------------|-----------------|-------|
| `.map(fn)` | `.map(fn)` | Nearly identical |
| `.filter(fn)` | `.filter(fn)` | Closure takes `&&T` for `.iter()` |
| `.reduce(fn, init)` | `.fold(init, fn)` | Argument order swapped |
| `.find(fn)` | `.find(fn)` | Returns `Option<&T>` |
| `.some(fn)` | `.any(fn)` | |
| `.every(fn)` | `.all(fn)` | |
| `.flat()` | `.flatten()` | |
| `.flatMap(fn)` | `.flat_map(fn)` | |
| `.forEach(fn)` | `.for_each(fn)` | Prefer `for` loop instead |
| `.slice(0, n)` | `.take(n)` | Lazy |
| `.slice(n)` | `.skip(n)` | Lazy |
| `[...a, ...b]` | `a.chain(b)` | Lazy concatenation |
| `.entries()` | `.enumerate()` | Yields `(index, value)` |
| `.includes(x)` | `.any(|&v| v == x)` | Or use `.contains()` on slices |
| `Array.from({length: n}, (_, i) => i)` | `(0..n)` | Range |

### Collecting Results

`.collect()` is the terminator that materializes an iterator into a collection:

```rust
let names: Vec<String> = data.iter().map(|d| d.name.clone()).collect();
let set: HashSet<i32> = numbers.iter().copied().collect();
let map: HashMap<&str, i32> = pairs.into_iter().collect();

// Collect Results: if any item is Err, the whole thing is Err
let results: Result<Vec<i32>, _> = strings.iter()
    .map(|s| s.parse::<i32>())
    .collect();
```

### Method Chaining — A Complete Example

🟦 **TypeScript**
```typescript
const topAuthors = posts
    .filter(p => p.published)
    .map(p => p.author)
    .reduce((acc, author) => {
        acc.set(author, (acc.get(author) ?? 0) + 1);
        return acc;
    }, new Map<string, number>());
```

🦀 **Rust**
```rust
let top_authors: HashMap<&str, usize> = posts.iter()
    .filter(|p| p.published)
    .map(|p| p.author.as_str())
    .fold(HashMap::new(), |mut acc, author| {
        *acc.entry(author).or_insert(0) += 1;
        acc
    });
```

### Zero-Cost Abstraction

The iterator chain above compiles to roughly the same machine code as:

```rust
let mut top_authors = HashMap::new();
for post in &posts {
    if post.published {
        *top_authors.entry(post.author.as_str()).or_insert(0) += 1;
    }
}
```

There is no intermediate allocation for `.filter()` or `.map()`. This is what "zero-cost
abstraction" means.

🏋️ **Exercise**: Given a `Vec<String>` of lines from a log file, use iterator chains to:
1. Filter lines containing "ERROR".
2. Extract the timestamp (first 19 characters).
3. Collect into a `Vec<&str>`.
