## `Vec<T>` vs `List<T>`

> **What you'll learn:** `Vec<T>` vs `List<T>`, `HashMap` vs `Dictionary`, safe access patterns
> (why Rust returns `Option` instead of throwing), and the ownership implications of collections.
>
> **Difficulty:** 🟢 Beginner

`Vec<T>` is Rust's equivalent to C#'s `List<T>`, but with ownership semantics.

### C# `List<T>`
```csharp
// C# List<T> - Reference type, heap allocated
var numbers = new List<int>();
numbers.Add(1);
numbers.Add(2);
numbers.Add(3);

// Pass to method - reference is copied
ProcessList(numbers);
Console.WriteLine(numbers.Count);  // Still accessible

void ProcessList(List<int> list)
{
    list.Add(4);  // Modifies original list
    Console.WriteLine($"Count in method: {list.Count}");
}
```

### Rust `Vec<T>`
```rust
// Rust Vec<T> - Owned type, heap allocated
let mut numbers = Vec::new();
numbers.push(1);
numbers.push(2);
numbers.push(3);

// Method that takes ownership
process_vec(numbers);
// println!("{:?}", numbers);  // ❌ Error: numbers was moved

// Method that borrows
let mut numbers = vec![1, 2, 3];  // vec! macro for convenience
process_vec_borrowed(&mut numbers);
println!("{:?}", numbers);  // ✅ Still accessible

fn process_vec(mut vec: Vec<i32>) {  // Takes ownership
    vec.push(4);
    println!("Count in method: {}", vec.len());
    // vec is dropped here
}

fn process_vec_borrowed(vec: &mut Vec<i32>) {  // Borrows mutably
    vec.push(4);
    println!("Count in method: {}", vec.len());
}
```

### Creating and Initializing Vectors
```csharp
// C# List initialization
var numbers = new List<int> { 1, 2, 3, 4, 5 };
var empty = new List<int>();
var sized = new List<int>(10);  // Initial capacity

// From other collections
var fromArray = new List<int>(new[] { 1, 2, 3 });
```

```rust
// Rust Vec initialization
let numbers = vec![1, 2, 3, 4, 5];  // vec! macro
let empty: Vec<i32> = Vec::new();   // Type annotation needed for empty
let sized = Vec::with_capacity(10); // Pre-allocate capacity

// From iterator
let from_range: Vec<i32> = (1..=5).collect();
let from_array = vec![1, 2, 3];
```

### Common Operations Comparison
```csharp
// C# List operations
var list = new List<int> { 1, 2, 3 };

list.Add(4);                    // Add element
list.Insert(0, 0);              // Insert at index
list.Remove(2);                 // Remove first occurrence
list.RemoveAt(1);               // Remove at index
list.Clear();                   // Remove all

int first = list[0];            // Index access
int count = list.Count;         // Get count
bool contains = list.Contains(3); // Check if contains
```

```rust
// Rust Vec operations
let mut vec = vec![1, 2, 3];

vec.push(4);                    // Add element
vec.insert(0, 0);               // Insert at index
vec.retain(|&x| x != 2);        // Remove elements (functional style)
vec.remove(1);                  // Remove at index
vec.clear();                    // Remove all

let first = vec[0];             // Index access (panics if out of bounds)
let safe_first = vec.get(0);    // Safe access, returns Option<&T>
let count = vec.len();          // Get count
let contains = vec.contains(&3); // Check if contains
```

### Safe Access Patterns
```csharp
// C# - Exception-based bounds checking
public int SafeAccess(List<int> list, int index)
{
    try
    {
        return list[index];
    }
    catch (ArgumentOutOfRangeException)
    {
        return -1;  // Default value
    }
}
```

```rust
// Rust - Option-based safe access
fn safe_access(vec: &[i32], index: usize) -> Option<i32> {
    vec.get(index).copied()  // Returns Option<i32>
}

fn main() {
    let vec = vec![1, 2, 3];
    
    // Safe access patterns
    match vec.get(10) {
        Some(value) => println!("Value: {}", value),
        None => println!("Index out of bounds"),
    }
    
    // Or with unwrap_or
    let value = vec.get(10).copied().unwrap_or(-1);
    println!("Value: {}", value);
}
```

***

## HashMap vs Dictionary

HashMap is Rust's equivalent to C#'s `Dictionary<K,V>`.

### C# Dictionary
```csharp
// C# Dictionary<TKey, TValue>
var scores = new Dictionary<string, int>
{
    ["Alice"] = 100,
    ["Bob"] = 85,
    ["Charlie"] = 92
};

// Add/Update
scores["Dave"] = 78;
scores["Alice"] = 105;  // Update existing

// Safe access
if (scores.TryGetValue("Eve", out int score))
{
    Console.WriteLine($"Eve's score: {score}");
}
else
{
    Console.WriteLine("Eve not found");
}

// Iteration
foreach (var kvp in scores)
{
    Console.WriteLine($"{kvp.Key}: {kvp.Value}");
}
```

### Rust HashMap
```rust
use std::collections::HashMap;

// Create and initialize HashMap
let mut scores = HashMap::new();
scores.insert("Alice".to_string(), 100);
scores.insert("Bob".to_string(), 85);
scores.insert("Charlie".to_string(), 92);

// Or use from iterator
let scores: HashMap<String, i32> = [
    ("Alice".to_string(), 100),
    ("Bob".to_string(), 85),
    ("Charlie".to_string(), 92),
].into_iter().collect();

// Add/Update
let mut scores = scores;  // Make mutable
scores.insert("Dave".to_string(), 78);
scores.insert("Alice".to_string(), 105);  // Update existing

// Safe access
match scores.get("Eve") {
    Some(score) => println!("Eve's score: {}", score),
    None => println!("Eve not found"),
}

// Iteration
for (name, score) in &scores {
    println!("{}: {}", name, score);
}
```

### HashMap Operations
```csharp
// C# Dictionary operations
var dict = new Dictionary<string, int>();

dict["key"] = 42;                    // Insert/update
bool exists = dict.ContainsKey("key"); // Check existence
bool removed = dict.Remove("key");    // Remove
dict.Clear();                        // Clear all

// Get with default
int value = dict.GetValueOrDefault("missing", 0);
```

```rust
use std::collections::HashMap;

// Rust HashMap operations
let mut map = HashMap::new();

map.insert("key".to_string(), 42);   // Insert/update
let exists = map.contains_key("key"); // Check existence
let removed = map.remove("key");      // Remove, returns Option<V>
map.clear();                         // Clear all

// Entry API for advanced operations
let mut map = HashMap::new();
map.entry("key".to_string()).or_insert(42);  // Insert if not exists
map.entry("key".to_string()).and_modify(|v| *v += 1); // Modify if exists

// Get with default
let value = map.get("missing").copied().unwrap_or(0);
```

### Ownership with HashMap Keys and Values
```rust
// Understanding ownership with HashMap
fn ownership_example() {
    let mut map = HashMap::new();
    
    // String keys and values are moved into the map
    let key = String::from("name");
    let value = String::from("Alice");
    
    map.insert(key, value);
    // println!("{}", key);   // ❌ Error: key was moved
    // println!("{}", value); // ❌ Error: value was moved
    
    // Access via references
    if let Some(name) = map.get("name") {
        println!("Name: {}", name);  // Borrowing the value
    }
}

// Using &str keys (no ownership transfer)
fn string_slice_keys() {
    let mut map = HashMap::new();
    
    map.insert("name", "Alice");     // &str keys and values
    map.insert("age", "30");
    
    // No ownership issues with string literals
    println!("Name exists: {}", map.contains_key("name"));
}
```

***

## Working with Collections

### Iteration Patterns
```csharp
// C# iteration patterns
var numbers = new List<int> { 1, 2, 3, 4, 5 };

// For loop with index
for (int i = 0; i < numbers.Count; i++)
{
    Console.WriteLine($"Index {i}: {numbers[i]}");
}

// Foreach loop
foreach (int num in numbers)
{
    Console.WriteLine(num);
}

// LINQ methods
var doubled = numbers.Select(x => x * 2).ToList();
var evens = numbers.Where(x => x % 2 == 0).ToList();
```

```rust
// Rust iteration patterns
let numbers = vec![1, 2, 3, 4, 5];

// For loop with index
for (i, num) in numbers.iter().enumerate() {
    println!("Index {}: {}", i, num);
}

// For loop over values
for num in &numbers {  // Borrow each element
    println!("{}", num);
}

// Iterator methods (like LINQ)
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
let evens: Vec<i32> = numbers.iter().filter(|&x| x % 2 == 0).cloned().collect();

// Or more efficiently, consuming iterator
let doubled: Vec<i32> = numbers.into_iter().map(|x| x * 2).collect();
```

### Iterator vs IntoIterator vs Iter
```rust
// Understanding different iteration methods
fn iteration_methods() {
    let vec = vec![1, 2, 3, 4, 5];
    
    // 1. iter() - borrows elements (&T)
    for item in vec.iter() {
        println!("{}", item);  // item is &i32
    }
    // vec is still usable here
    
    // 2. into_iter() - takes ownership (T)
    for item in vec.into_iter() {
        println!("{}", item);  // item is i32
    }
    // vec is no longer usable here
    
    let mut vec = vec![1, 2, 3, 4, 5];
    
    // 3. iter_mut() - mutable borrows (&mut T)
    for item in vec.iter_mut() {
        *item *= 2;  // item is &mut i32
    }
    println!("{:?}", vec);  // [2, 4, 6, 8, 10]
}
```

### Collecting Results
```csharp
// C# - Processing collections with potential errors
public List<int> ParseNumbers(List<string> inputs)
{
    var results = new List<int>();
    foreach (string input in inputs)
    {
        if (int.TryParse(input, out int result))
        {
            results.Add(result);
        }
        // Silently skip invalid inputs
    }
    return results;
}
```

```rust
// Rust - Explicit error handling with collect
fn parse_numbers(inputs: Vec<String>) -> Result<Vec<i32>, std::num::ParseIntError> {
    inputs.into_iter()
        .map(|s| s.parse::<i32>())  // Returns Result<i32, ParseIntError>
        .collect()                  // Collects into Result<Vec<i32>, ParseIntError>
}

// Alternative: Filter out errors
fn parse_numbers_filter(inputs: Vec<String>) -> Vec<i32> {
    inputs.into_iter()
        .filter_map(|s| s.parse::<i32>().ok())  // Keep only Ok values
        .collect()
}

fn main() {
    let inputs = vec!["1".to_string(), "2".to_string(), "invalid".to_string(), "4".to_string()];
    
    // Version that fails on first error
    match parse_numbers(inputs.clone()) {
        Ok(numbers) => println!("All parsed: {:?}", numbers),
        Err(error) => println!("Parse error: {}", error),
    }
    
    // Version that skips errors
    let numbers = parse_numbers_filter(inputs);
    println!("Successfully parsed: {:?}", numbers);  // [1, 2, 4]
}
```

---

## Exercises

<details>
<summary><strong>🏋️ Exercise: LINQ to Iterators</strong> (click to expand)</summary>

Translate this C# LINQ query to idiomatic Rust iterators:

```csharp
var result = students
    .Where(s => s.Grade >= 90)
    .OrderByDescending(s => s.Grade)
    .Select(s => $"{s.Name}: {s.Grade}")
    .Take(3)
    .ToList();
```

Use this struct:
```rust
struct Student { name: String, grade: u32 }
```

Return a `Vec<String>` of the top 3 students with grade ≥ 90, formatted as `"Name: Grade"`.

<details>
<summary>🔑 Solution</summary>

```rust
#[derive(Debug)]
struct Student { name: String, grade: u32 }

fn top_students(students: &mut [Student]) -> Vec<String> {
    students.sort_by(|a, b| b.grade.cmp(&a.grade)); // sort descending
    students.iter()
        .filter(|s| s.grade >= 90)
        .take(3)
        .map(|s| format!("{}: {}", s.name, s.grade))
        .collect()
}

fn main() {
    let mut students = vec![
        Student { name: "Alice".into(), grade: 95 },
        Student { name: "Bob".into(), grade: 88 },
        Student { name: "Carol".into(), grade: 92 },
        Student { name: "Dave".into(), grade: 97 },
        Student { name: "Eve".into(), grade: 91 },
    ];
    let result = top_students(&mut students);
    assert_eq!(result, vec!["Dave: 97", "Alice: 95", "Carol: 92"]);
    println!("{result:?}");
}
```

**Key difference from C#**: Rust iterators are lazy (like LINQ), but `.sort_by()` is eager and in-place — there's no lazy `OrderBy`. You sort first, then chain lazy operations.

</details>
</details>

***


