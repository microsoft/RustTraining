# Rust Bootstrap for C# Developers

A structured introduction to Rust for developers with C# experience. This guide follows a proven pedagogical approach, building concepts step by step to help you understand not just *how* Rust works, but *why* it was designed this way.

## Course Overview
- **The case for Rust** - Why Rust matters for C# developers
- **Getting started** - Installation, tooling, and your first program
- **Basic building blocks** - Types, variables, control flow
- **Data structures** - Arrays, tuples, structs
- **Pattern matching and enums** - Essential Rust concepts
- **Modules and crates** - Code organization and dependencies (vs .NET assemblies)
- **Traits and generics** - Advanced type system
- **Error handling** - Rust's approach to safety
- **Memory management** - Ownership, borrowing, and lifetimes
- **Practical migration** - Real-world examples

## Table of Contents

### 1. Introduction and Motivation
- [Quick Reference: Rust vs C#](#quick-reference-rust-vs-c)
- [The Case for Rust for C# Developers](#the-case-for-rust-for-c-developers)
- [Common C# Pain Points That Rust Addresses](#common-c-pain-points-that-rust-addresses)
- [When to Choose Rust Over C#](#when-to-choose-rust-over-c)

### 2. Getting Started
- [Installation and Setup](#installation-and-setup)
- [Your First Rust Program](#your-first-rust-program)
- [Cargo vs NuGet/MSBuild](#cargo-vs-nugetmsbuild)
- [IDE Setup for C# Developers](#ide-setup-for-c-developers)

### 3. Basic Types and Variables
- [Built-in Types Comparison](#built-in-types-comparison)
- [Variables and Mutability](#variables-and-mutability)
- [String Types: String vs &str](#string-types-string-vs-str)
- [Comments and Documentation](#comments-and-documentation)

### 4. Control Flow
- [Conditional Statements](#conditional-statements)
- [Loops and Iteration](#loops-and-iteration)
- [Expression Blocks](#expression-blocks)
- [Functions vs Methods](#functions-vs-methods)

### 5. Data Structures
- [Arrays and Slices](#arrays-and-slices)
- [Tuples](#tuples)
- [Structs vs Classes](#structs-vs-classes)
- [References and Borrowing Basics](#references-and-borrowing-basics)

### 6. Pattern Matching and Enums
- [Enums vs C# Enums](#enums-vs-c-enums)
- [Match Expressions](#match-expressions)
- [Option<T> for Null Safety](#optiont-for-null-safety)
- [Result<T, E> for Error Handling](#resultt-e-for-error-handling)

### 7. Modules and Crates
- [Rust Modules vs C# Namespaces](#rust-modules-vs-c-namespaces)
- [Crates vs .NET Assemblies](#crates-vs-net-assemblies)
- [Package Management: Cargo vs NuGet](#package-management-cargo-vs-nuget)
- [Visibility and Access Control](#visibility-and-access-control)

### 8. Traits and Generics
- [Traits vs Interfaces](#traits-vs-interfaces)
- [Generic Types and Functions](#generic-types-and-functions)
- [Trait Bounds and Constraints](#trait-bounds-and-constraints)
- [Common Standard Library Traits](#common-standard-library-traits)

### 9. Collections and Error Handling
- [Vec<T> vs List<T>](#vect-vs-listt)
- [HashMap vs Dictionary](#hashmap-vs-dictionary)
- [Iterator Patterns](#iterator-patterns)
- [Comprehensive Error Handling](#comprehensive-error-handling)

### 10. Memory Management
- [Understanding Ownership](#understanding-ownership)
- [Move Semantics vs Reference Semantics](#move-semantics-vs-reference-semantics)
- [Borrowing and Lifetimes](#borrowing-and-lifetimes)
- [Smart Pointers](#smart-pointers)

### 11. Practical Migration Examples
- [Configuration Management](#configuration-management)
- [Data Processing Pipelines](#data-processing-pipelines)
- [HTTP Clients and APIs](#http-clients-and-apis)
- [File I/O and Serialization](#file-io-and-serialization)

### 12. Next Steps and Best Practices
- [Testing in Rust vs C#](#testing-in-rust-vs-c)
- [Common Pitfalls for C# Developers](#common-pitfalls-for-c-developers)
- [Learning Path and Resources](#learning-path-and-resources)
- [Moving to Advanced Topics](#moving-to-advanced-topics)

***

## Quick Reference: Rust vs C#

| **Concept** | **C#** | **Rust** | **Key Difference** |
|-------------|--------|----------|-------------------|
| Memory management | Garbage collector | Ownership system | Zero-cost, deterministic cleanup |
| Null references | `null` everywhere | `Option<T>` | Compile-time null safety |
| Error handling | Exceptions | `Result<T, E>` | Explicit, no hidden control flow |
| Mutability | Mutable by default | Immutable by default | Opt-in to mutation |
| Type system | Reference/value types | Ownership types | Move semantics, borrowing |
| Assemblies | GAC, app domains (.NET Framework); side-by-side (.NET 5+) | Crates | Static linking, no runtime |
| Namespaces | `using System.IO` | `use std::fs` | Module system |
| Interfaces | `interface IFoo` | `trait Foo` | Default implementations |
| Generics | `List<T>` (optional constraints via `where`) | `Vec<T>` (trait bounds like `T: Clone`) | Zero-cost abstractions |
| Threading | locks, async/await | Ownership + Send/Sync | Data race prevention |
| Performance | JIT compilation | AOT compilation | Predictable, no GC pauses |

***

## The Case for Rust for C# Developers

### Performance Without the Runtime Tax
```csharp
// C# - Great productivity, runtime overhead
public class DataProcessor
{
    private List<int> data = new List<int>();
    
    public void ProcessLargeDataset()
    {
        // Allocations trigger GC
        for (int i = 0; i < 10_000_000; i++)
        {
            data.Add(i * 2); // GC pressure
        }
        // Unpredictable GC pauses during processing
    }
}
// Runtime: Variable (50-200ms due to GC)
// Memory: ~80MB (including GC overhead)
// Predictability: Low (GC pauses)
```

```rust
// Rust - Same expressiveness, zero runtime overhead
struct DataProcessor {
    data: Vec<i32>,
}

impl DataProcessor {
    fn process_large_dataset(&mut self) {
        // Zero-cost abstractions
        for i in 0..10_000_000 {
            self.data.push(i * 2); // No GC pressure
        }
        // Deterministic performance
    }
}
// Runtime: Consistent (~30ms)
// Memory: ~40MB (exact allocation)
// Predictability: High (no GC)
```

### Memory Safety Without Runtime Checks
```csharp
// C# - Runtime safety with overhead
public class UnsafeOperations
{
    public string ProcessArray(int[] array)
    {
        // Runtime bounds checking
        if (array.Length > 0)
        {
            return array[0].ToString(); // NullReferenceException possible
        }
        return null; // Null propagation
    }
    
    public void ProcessConcurrently()
    {
        var list = new List<int>();
        
        // Data races possible, requires careful locking
        Parallel.For(0, 1000, i =>
        {
            lock (list) // Runtime overhead
            {
                list.Add(i);
            }
        });
    }
}
```

```rust
// Rust - Compile-time safety with zero runtime cost
struct SafeOperations;

impl SafeOperations {
    // Compile-time null safety, no runtime checks
    fn process_array(array: &[i32]) -> Option<String> {
        array.first().map(|x| x.to_string())
        // No null references possible
        // Bounds checking optimized away when provably safe
    }
    
    fn process_concurrently() {
        use std::sync::Mutex;
        use std::thread;
        
        let data = Mutex::new(Vec::new());
        
        // Data races prevented at compile time
        let handles: Vec<_> = (0..1000).map(|i| {
            let data = &data;
            thread::spawn(move || {
                data.lock().unwrap().push(i);
                // No lock overhead when single-threaded
            })
        }).collect();
        
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
```

***

## Common C# Pain Points That Rust Addresses

### 1. The Billion Dollar Mistake: Null References
```csharp
// C# - Null reference exceptions are runtime bombs
public class UserService
{
    public string GetUserDisplayName(User user)
    {
        // Any of these could throw NullReferenceException
        return user.Profile.DisplayName.ToUpper();
        //     ^^^^^ ^^^^^^^ ^^^^^^^^^^^ ^^^^^^^
        //     Could be null at runtime
    }
    
    // Nullable reference types (C# 8+) help, but nulls can still slip through
    public string GetDisplayName(User? user)
    {
        return user?.Profile?.DisplayName?.ToUpper() ?? "Unknown";
        // This specific line is null-safe thanks to ?. and ??,
        // but NRTs are advisory — the compiler can be overridden with `!`
    }
}
```

```rust
// Rust - Null safety guaranteed at compile time
struct UserService;

impl UserService {
    fn get_user_display_name(user: &User) -> Option<String> {
        user.profile.as_ref()?
            .display_name.as_ref()
            .map(|name| name.to_uppercase())
        // Compiler forces you to handle None case
        // Impossible to have null pointer exceptions
    }
    
    fn get_display_name_safe(user: Option<&User>) -> String {
        user.and_then(|u| u.profile.as_ref())
            .and_then(|p| p.display_name.as_ref())
            .map(|name| name.to_uppercase())
            .unwrap_or_else(|| "Unknown".to_string())
        // Explicit handling, no surprises
    }
}
```

### 2. Hidden Exceptions and Control Flow
```csharp
// C# - Exceptions can be thrown from anywhere
public async Task<UserData> GetUserDataAsync(int userId)
{
    // Each of these might throw different exceptions
    var user = await userRepository.GetAsync(userId);        // SqlException
    var permissions = await permissionService.GetAsync(user); // HttpRequestException  
    var preferences = await preferenceService.GetAsync(user); // TimeoutException
    
    return new UserData(user, permissions, preferences);
    // Caller has no idea what exceptions to expect
}
```

```rust
// Rust - All errors explicit in function signatures
#[derive(Debug)]
enum UserDataError {
    DatabaseError(String),
    NetworkError(String),
    Timeout,
    UserNotFound(i32),
}

async fn get_user_data(user_id: i32) -> Result<UserData, UserDataError> {
    // All errors explicit and handled
    let user = user_repository.get(user_id).await
        .map_err(UserDataError::DatabaseError)?;
    
    let permissions = permission_service.get(&user).await
        .map_err(UserDataError::NetworkError)?;
    
    let preferences = preference_service.get(&user).await
        .map_err(|_| UserDataError::Timeout)?;
    
    Ok(UserData::new(user, permissions, preferences))
    // Caller knows exactly what errors are possible
}
```

### 3. Unpredictable Performance Due to GC
```csharp
// C# - GC can pause at any time
public class HighFrequencyTrader
{
    private List<Trade> trades = new List<Trade>();
    
    public void ProcessMarketData(MarketTick tick)
    {
        // Allocations can trigger GC at worst possible moment
        var analysis = new MarketAnalysis(tick);
        trades.Add(new Trade(analysis.Signal, tick.Price));
        
        // GC might pause here during critical market moment
        // Pause duration: 1-100ms depending on heap size
    }
}
```

```rust
// Rust - Predictable, deterministic performance
struct HighFrequencyTrader {
    trades: Vec<Trade>,
}

impl HighFrequencyTrader {
    fn process_market_data(&mut self, tick: MarketTick) {
        // Zero allocations, predictable performance
        let analysis = MarketAnalysis::from(tick);
        self.trades.push(Trade::new(analysis.signal(), tick.price));
        
        // No GC pauses, consistent sub-microsecond latency
        // Performance guaranteed by type system
    }
}
```

***

## When to Choose Rust Over C#

### ✅ Choose Rust When:
- **Performance is critical**: Real-time systems, high-frequency trading, game engines
- **Memory usage matters**: Embedded systems, cloud costs, mobile applications
- **Predictability required**: Medical devices, automotive, financial systems
- **Security is paramount**: Cryptography, network security, system-level code
- **Long-running services**: Where GC pauses cause issues
- **Resource-constrained environments**: IoT, edge computing
- **System programming**: CLI tools, databases, web servers, operating systems

### ✅ Stay with C# When:
- **Rapid application development**: Business applications, CRUD applications
- **Large existing codebase**: When migration cost is prohibitive
- **Team expertise**: When Rust learning curve doesn't justify benefits
- **Enterprise integrations**: Heavy .NET Framework/Windows dependencies
- **GUI applications**: WPF, WinUI, Blazor ecosystems
- **Time to market**: When development speed trumps performance

### 🔄 Consider Both (Hybrid Approach):
- **Performance-critical components in Rust**: Called from C# via P/Invoke
- **Business logic in C#**: Familiar, productive development
- **Gradual migration**: Start with new services in Rust

***

## Real-World Impact: Why Companies Choose Rust

### Dropbox: Storage Infrastructure
- **Before (Python)**: High CPU usage, memory overhead
- **After (Rust)**: 10x performance improvement, 50% memory reduction
- **Result**: Millions saved in infrastructure costs

### Discord: Voice/Video Backend  
- **Before (Go)**: GC pauses causing audio drops
- **After (Rust)**: Consistent low-latency performance
- **Result**: Better user experience, reduced server costs

### Microsoft: Windows Components
- **Rust in Windows**: File system, networking stack components
- **Benefit**: Memory safety without performance cost
- **Impact**: Fewer security vulnerabilities, same performance

### Why This Matters for C# Developers:
1. **Complementary skills**: Rust and C# solve different problems
2. **Career growth**: Systems programming expertise increasingly valuable
3. **Performance understanding**: Learn zero-cost abstractions
4. **Safety mindset**: Apply ownership thinking to any language
5. **Cloud costs**: Performance directly impacts infrastructure spend

***

## Installation and Setup

### Installing Rust
```bash
# Install Rust (works on Windows, macOS, Linux)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# On Windows, you can also download from: https://rustup.rs/
```

### Rust Tools vs C# Tools
| C# Tool | Rust Equivalent | Purpose |
|---------|----------------|---------|
| `dotnet new` | `cargo new` | Create new project |
| `dotnet build` | `cargo build` | Compile project |
| `dotnet run` | `cargo run` | Run project |
| `dotnet test` | `cargo test` | Run tests |
| NuGet | Crates.io | Package repository |
| MSBuild | Cargo | Build system |
| Visual Studio | VS Code + rust-analyzer | IDE |

### IDE Setup
1. **VS Code** (Recommended for beginners)
   - Install "rust-analyzer" extension
   - Install "CodeLLDB" for debugging

2. **Visual Studio** (Windows)
   - Install Rust support extension

3. **JetBrains RustRover** (Full IDE)
   - Similar to Rider for C#

***

## Your First Rust Program

### C# Hello World
```csharp
// Program.cs
using System;

namespace HelloWorld
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Hello, World!");
        }
    }
}
```

### Rust Hello World
```rust
// main.rs
fn main() {
    println!("Hello, World!");
}
```

### Key Differences for C# Developers
1. **No classes required** - Functions can exist at the top level
2. **No namespaces** - Uses module system instead
3. **`println!` is a macro** - Notice the `!` 
4. **Semicolons matter** - Omitting the trailing semicolon turns a statement into a return expression
5. **No explicit return type** - `main` returns `()` (unit type)

### Creating Your First Project
```bash
# Create new project (like 'dotnet new console')
cargo new hello_rust
cd hello_rust

# Project structure created:
# hello_rust/
# ├── Cargo.toml      (like .csproj file)
# └── src/
#     └── main.rs     (like Program.cs)

# Run the project (like 'dotnet run')
cargo run
```

***

## Cargo vs NuGet/MSBuild

### Project Configuration

**C# (.csproj)**
```xml
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net8.0</TargetFramework>
  </PropertyGroup>
  
  <PackageReference Include="Newtonsoft.Json" Version="13.0.3" />
  <PackageReference Include="Serilog" Version="3.0.1" />
</Project>
```

**Rust (Cargo.toml)**
```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2021"

[dependencies]
serde_json = "1.0"    # Like Newtonsoft.Json
log = "0.4"           # Like Serilog
```

### Common Cargo Commands
```bash
# Create new project
cargo new my_project
cargo new my_project --lib  # Create library project

# Build and run
cargo build          # Like 'dotnet build'
cargo run            # Like 'dotnet run'
cargo test           # Like 'dotnet test'

# Package management
cargo add serde      # Add dependency (like 'dotnet add package')
cargo update         # Update dependencies

# Release build
cargo build --release  # Optimized build
cargo run --release    # Run optimized version

# Documentation
cargo doc --open     # Generate and open docs
```

### Workspace vs Solution

**C# Solution (.sln)**
```
MySolution/
├── MySolution.sln
├── WebApi/
│   └── WebApi.csproj
├── Business/
│   └── Business.csproj
└── Tests/
    └── Tests.csproj
```

**Rust Workspace (Cargo.toml)**
```toml
[workspace]
members = [
    "web_api",
    "business", 
    "tests"
]
```

***

## Variables and Mutability

### C# Variable Declaration
```csharp
// C# - Variables are mutable by default
int count = 0;           // Mutable
count = 5;               // ✅ Works

readonly int maxSize = 100;  // Immutable after initialization
// maxSize = 200;        // ❌ Compile error

const int BUFFER_SIZE = 1024; // Compile-time constant
```

### Rust Variable Declaration
```rust
// Rust - Variables are immutable by default
let count = 0;           // Immutable by default
// count = 5;            // ❌ Compile error: cannot assign twice to immutable variable

let mut count = 0;       // Explicitly mutable
count = 5;               // ✅ Works

const BUFFER_SIZE: usize = 1024; // Compile-time constant
```

### Key Mental Shift for C# Developers
```rust
// Think of 'let' as 'readonly' by default
let name = "John";       // Like: readonly string name = "John";
let mut age = 30;        // Like: int age = 30;

// Variable shadowing (unique to Rust)
let spaces = "   ";      // String
let spaces = spaces.len(); // Now it's a number (usize)
// This is different from mutation - we're creating a new variable
```

### Practical Example: Counter
```csharp
// C# version
public class Counter
{
    private int value = 0;
    
    public void Increment()
    {
        value++;  // Mutation
    }
    
    public int GetValue() => value;
}
```

```rust
// Rust version
pub struct Counter {
    value: i32,  // Private by default
}

impl Counter {
    pub fn new() -> Counter {
        Counter { value: 0 }
    }
    
    pub fn increment(&mut self) {  // &mut needed for mutation
        self.value += 1;
    }
    
    pub fn get_value(&self) -> i32 {
        self.value
    }
}
```

***

## Data Types Comparison

### Primitive Types

| C# Type | Rust Type | Size | Range |
|---------|-----------|------|-------|
| `byte` | `u8` | 8 bits | 0 to 255 |
| `sbyte` | `i8` | 8 bits | -128 to 127 |
| `short` | `i16` | 16 bits | -32,768 to 32,767 |
| `ushort` | `u16` | 16 bits | 0 to 65,535 |
| `int` | `i32` | 32 bits | -2³¹ to 2³¹-1 |
| `uint` | `u32` | 32 bits | 0 to 2³²-1 |
| `long` | `i64` | 64 bits | -2⁶³ to 2⁶³-1 |
| `ulong` | `u64` | 64 bits | 0 to 2⁶⁴-1 |
| `float` | `f32` | 32 bits | IEEE 754 |
| `double` | `f64` | 64 bits | IEEE 754 |
| `bool` | `bool` | 1 bit | true/false |
| `char` | `char` | 32 bits | Unicode scalar |

### Size Types (Important!)
```csharp
// C# - int is always 32-bit
int arrayIndex = 0;
long fileSize = file.Length;
```

```rust
// Rust - size types match pointer size (32-bit or 64-bit)
let array_index: usize = 0;    // Like size_t in C
let file_size: u64 = file.len(); // Explicit 64-bit
```

### Type Inference
```csharp
// C# - var keyword
var name = "John";        // string
var count = 42;           // int
var price = 29.99;        // double
```

```rust
// Rust - automatic type inference
let name = "John";        // &str (string slice)
let count = 42;           // i32 (default integer)
let price = 29.99;        // f64 (default float)

// Explicit type annotations
let count: u32 = 42;
let price: f32 = 29.99;
```

### Arrays and Collections Overview
```csharp
// C# - reference types, heap allocated
int[] numbers = new int[5];        // Fixed size
List<int> list = new List<int>();  // Dynamic size
```

```rust
// Rust - multiple options
let numbers: [i32; 5] = [1, 2, 3, 4, 5];  // Stack array, fixed size
let mut list: Vec<i32> = Vec::new();       // Heap vector, dynamic size
```

***

## String Types: String vs &str

This is one of the most confusing concepts for C# developers, so let's break it down carefully.

### C# String Handling
```csharp
// C# - Simple string model
string name = "John";           // String literal
string greeting = "Hello, " + name;  // String concatenation
string upper = name.ToUpper();  // Method call
```

### Rust String Types
```rust
// Rust - Two main string types

// 1. &str (string slice) - like ReadOnlySpan<char> in C#
let name: &str = "John";        // String literal (immutable, borrowed)

// 2. String - like StringBuilder or mutable string
let mut greeting = String::new();       // Empty string
greeting.push_str("Hello, ");          // Append
greeting.push_str(name);               // Append

// Or create directly
let greeting = String::from("Hello, John");
let greeting = "Hello, John".to_string();  // Convert &str to String
```

### When to Use Which?

| Scenario | Use | C# Equivalent |
|----------|-----|---------------|
| String literals | `&str` | `string` literal |
| Function parameters (read-only) | `&str` | `string` or `ReadOnlySpan<char>` |
| Owned, mutable strings | `String` | `StringBuilder` |
| Return owned strings | `String` | `string` |

### Practical Examples
```rust
// Function that accepts any string type
fn greet(name: &str) {  // Accepts both String and &str
    println!("Hello, {}!", name);
}

fn main() {
    let literal = "John";                    // &str
    let owned = String::from("Jane");        // String
    
    greet(literal);                          // Works
    greet(&owned);                           // Works (borrow String as &str)
    greet("Bob");                            // Works
}

// Function that returns owned string
fn create_greeting(name: &str) -> String {
    format!("Hello, {}!", name)  // format! macro returns String
}
```

### C# Developers: Think of it This Way
```rust
// &str is like ReadOnlySpan<char> - a view into string data
// String is like a char[] that you own and can modify

let borrowed: &str = "I don't own this data";
let owned: String = String::from("I own this data");

// Convert between them
let owned_copy: String = borrowed.to_string();  // Copy to owned
let borrowed_view: &str = &owned;               // Borrow from owned
```

***

## Comments and Documentation

### Regular Comments
```csharp
// C# comments
// Single line comment
/* Multi-line
   comment */

/// <summary>
/// XML documentation comment
/// </summary>
/// <param name="name">The user's name</param>
/// <returns>A greeting string</returns>
public string Greet(string name)
{
    return $"Hello, {name}!";
}
```

```rust
// Rust comments
// Single line comment
/* Multi-line
   comment */

/// Documentation comment (like C# ///)
/// This function greets a user by name.
/// 
/// # Arguments
/// 
/// * `name` - The user's name as a string slice
/// 
/// # Returns
/// 
/// A `String` containing the greeting
/// 
/// # Examples
/// 
/// ```
/// let greeting = greet("Alice");
/// assert_eq!(greeting, "Hello, Alice!");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

### Documentation Generation
```bash
# Generate documentation (like XML docs in C#)
cargo doc --open

# Run documentation tests
cargo test --doc
```

***

## Essential Rust Keywords for C# Developers

Understanding Rust's keywords and their purposes helps C# developers navigate the language more effectively.

### Visibility and Access Control Keywords

#### C# Access Modifiers
```csharp
public class Example
{
    public int PublicField;           // Accessible everywhere
    private int privateField;        // Only within this class
    protected int protectedField;    // This class and subclasses
    internal int internalField;      // Within this assembly
    protected internal int protectedInternalField; // Combination
}
```

#### Rust Visibility Keywords
```rust
// pub - Makes items public (like C# public)
pub struct PublicStruct {
    pub public_field: i32,           // Public field
    private_field: i32,              // Private by default (no keyword)
}

pub mod my_module {
    pub(crate) fn crate_public() {}     // Public within current crate (like internal)
    pub(super) fn parent_public() {}    // Public to parent module
    pub(self) fn self_public() {}       // Public within current module (same as private)
    
    pub use super::PublicStruct;        // Re-export (like using alias)
}

// No direct equivalent to C# protected - use composition instead
```

### Memory and Ownership Keywords

#### C# Memory Keywords
```csharp
// ref - Pass by reference
public void Method(ref int value) { value = 10; }

// out - Output parameter
public bool TryParse(string input, out int result) { /* */ }

// in - Readonly reference (C# 7.2+)
public void ReadOnly(in LargeStruct data) { /* Cannot modify data */ }
```

#### Rust Ownership Keywords
```rust
// & - Immutable reference (like C# in parameter)
fn read_only(data: &Vec<i32>) {
    println!("Length: {}", data.len()); // Can read, cannot modify
}

// &mut - Mutable reference (like C# ref parameter)
fn modify(data: &mut Vec<i32>) {
    data.push(42); // Can modify
}

// move - Force move capture in closures
let data = vec![1, 2, 3];
let closure = move || {
    println!("{:?}", data); // data is moved into closure
};
// data is no longer accessible here

// Box - Heap allocation (like C# new for reference types)
let boxed_data = Box::new(42); // Allocate on heap
```

### Control Flow Keywords

#### C# Control Flow
```csharp
// return - Exit function with value
public int GetValue() { return 42; }

// yield return - Iterator pattern
public IEnumerable<int> GetNumbers()
{
    yield return 1;
    yield return 2;
}

// break/continue - Loop control
foreach (var item in items)
{
    if (item == null) continue;
    if (item.Stop) break;
}
```

#### Rust Control Flow Keywords
```rust
// return - Explicit return (usually not needed)
fn get_value() -> i32 {
    return 42; // Explicit return
    // OR just: 42 (implicit return)
}

// break/continue - Loop control with optional values
fn find_value() -> Option<i32> {
    loop {
        let value = get_next();
        if value < 0 { continue; }
        if value > 100 { break None; }      // Break with value
        if value == 42 { break Some(value); } // Break with success
    }
}

// loop - Infinite loop (like while(true))
loop {
    if condition { break; }
}

// while - Conditional loop
while condition {
    // code
}

// for - Iterator loop
for item in collection {
    // code
}
```

### Type Definition Keywords

#### C# Type Keywords
```csharp
// class - Reference type
public class MyClass { }

// struct - Value type
public struct MyStruct { }

// interface - Contract definition
public interface IMyInterface { }

// enum - Enumeration
public enum MyEnum { Value1, Value2 }

// delegate - Function pointer
public delegate void MyDelegate(int value);
```

#### Rust Type Keywords
```rust
// struct - Data structure (like C# class/struct combined)
struct MyStruct {
    field: i32,
}

// enum - Algebraic data type (much more powerful than C# enum)
enum MyEnum {
    Variant1,
    Variant2(i32),              // Can hold data
    Variant3 { x: i32, y: i32 }, // Struct-like variant
}

// trait - Interface definition (like C# interface but more powerful)
trait MyTrait {
    fn method(&self);
    
    // Default implementation (like C# 8+ default interface methods)
    fn default_method(&self) {
        println!("Default implementation");
    }
}

// type - Type alias (like C# using alias)
type UserId = u32;
type Result<T> = std::result::Result<T, MyError>;

// impl - Implementation block (no C# equivalent - methods defined separately)
impl MyStruct {
    fn new() -> MyStruct {
        MyStruct { field: 0 }
    }
}

impl MyTrait for MyStruct {
    fn method(&self) {
        println!("Implementation");
    }
}
```

### Function Definition Keywords

#### C# Function Keywords
```csharp
// static - Class method
public static void StaticMethod() { }

// virtual - Can be overridden
public virtual void VirtualMethod() { }

// override - Override base method
public override void VirtualMethod() { }

// abstract - Must be implemented
public abstract void AbstractMethod();

// async - Asynchronous method
public async Task<int> AsyncMethod() { return await SomeTask(); }
```

#### Rust Function Keywords
```rust
// fn - Function definition (like C# method but standalone)
fn regular_function() {
    println!("Hello");
}

// const fn - Compile-time function (like C# const but for functions)
const fn compile_time_function() -> i32 {
    42 // Can be evaluated at compile time
}

// async fn - Asynchronous function (like C# async)
async fn async_function() -> i32 {
    some_async_operation().await
}

// unsafe fn - Function that may violate memory safety
unsafe fn unsafe_function() {
    // Can perform unsafe operations
}

// extern fn - Foreign function interface
extern "C" fn c_compatible_function() {
    // Can be called from C
}
```

### Variable Declaration Keywords

#### C# Variable Keywords
```csharp
// var - Type inference
var name = "John"; // Inferred as string

// const - Compile-time constant
const int MaxSize = 100;

// readonly - Runtime constant
readonly DateTime createdAt = DateTime.Now;

// static - Class-level variable
static int instanceCount = 0;
```

#### Rust Variable Keywords
```rust
// let - Variable binding (like C# var)
let name = "John"; // Immutable by default

// let mut - Mutable variable binding
let mut count = 0; // Can be changed
count += 1;

// const - Compile-time constant (like C# const)
const MAX_SIZE: usize = 100;

// static - Global variable (like C# static)
static INSTANCE_COUNT: std::sync::atomic::AtomicUsize = 
    std::sync::atomic::AtomicUsize::new(0);
```

### Pattern Matching Keywords

#### C# Pattern Matching (C# 8+)
```csharp
// switch expression
string result = value switch
{
    1 => "One",
    2 => "Two",
    _ => "Other"
};

// is pattern
if (obj is string str)
{
    Console.WriteLine(str.Length);
}
```

#### Rust Pattern Matching Keywords
```rust
// match - Pattern matching (like C# switch but much more powerful)
let result = match value {
    1 => "One",
    2 => "Two",
    3..=10 => "Between 3 and 10", // Range patterns
    _ => "Other", // Wildcard (like C# _)
};

// if let - Conditional pattern matching
if let Some(value) = optional {
    println!("Got value: {}", value);
}

// while let - Loop with pattern matching
while let Some(item) = iterator.next() {
    println!("Item: {}", item);
}

// let with patterns - Destructuring
let (x, y) = point; // Destructure tuple
let Some(value) = optional else {
    return; // Early return if pattern doesn't match
};
```

### Memory Safety Keywords

#### C# Memory Keywords
```csharp
// unsafe - Disable safety checks
unsafe
{
    int* ptr = &variable;
    *ptr = 42;
}

// fixed - Pin managed memory
unsafe
{
    fixed (byte* ptr = array)
    {
        // Use ptr
    }
}
```

#### Rust Safety Keywords
```rust
// unsafe - Disable borrow checker (use sparingly!)
unsafe {
    let ptr = &variable as *const i32;
    let value = *ptr; // Dereference raw pointer
}

// Raw pointer types (no C# equivalent - usually not needed)
let ptr: *const i32 = &42;  // Immutable raw pointer
let ptr: *mut i32 = &mut 42; // Mutable raw pointer
```

### Common Rust Keywords Not in C#

```rust
// where - Generic constraints (more flexible than C# where)
fn generic_function<T>() 
where 
    T: Clone + Send + Sync,
{
    // T must implement Clone, Send, and Sync traits
}

// dyn - Dynamic trait objects (like C# object but type-safe)
let drawable: Box<dyn Draw> = Box::new(Circle::new());

// Self - Refer to the implementing type (like C# this but for types)
impl MyStruct {
    fn new() -> Self { // Self = MyStruct
        Self { field: 0 }
    }
}

// self - Method receiver
impl MyStruct {
    fn method(&self) { }        // Immutable borrow
    fn method_mut(&mut self) { } // Mutable borrow  
    fn consume(self) { }        // Take ownership
}

// crate - Refer to current crate root
use crate::models::User; // Absolute path from crate root

// super - Refer to parent module
use super::utils; // Import from parent module
```

### Keywords Summary for C# Developers

| Purpose | C# | Rust | Key Difference |
|---------|----|----|----------------|
| Visibility | `public`, `private`, `internal` | `pub`, default private | More granular with `pub(crate)` |
| Variables | `var`, `readonly`, `const` | `let`, `let mut`, `const` | Immutable by default |
| Functions | `method()` | `fn` | Standalone functions |
| Types | `class`, `struct`, `interface` | `struct`, `enum`, `trait` | Enums are algebraic types |
| Generics | `<T> where T : IFoo` | `<T> where T: Foo` | More flexible constraints |
| References | `ref`, `out`, `in` | `&`, `&mut` | Compile-time borrow checking |
| Patterns | `switch`, `is` | `match`, `if let` | Exhaustive matching required |

***

## Understanding Ownership

Ownership is Rust's most unique feature and the biggest conceptual shift for C# developers. Let's approach it step by step.

### C# Memory Model (Review)
```csharp
// C# - Automatic memory management
public void ProcessData()
{
    var data = new List<int> { 1, 2, 3, 4, 5 };
    ProcessList(data);
    // data is still accessible here
    Console.WriteLine(data.Count);  // Works fine
    
    // GC will clean up when no references remain
}

public void ProcessList(List<int> list)
{
    list.Add(6);  // Modifies the original list
}
```

### Rust Ownership Rules
1. **Each value has exactly one owner**
2. **When the owner goes out of scope, the value is dropped**
3. **Ownership can be transferred (moved)**

```rust
// Rust - Explicit ownership management
fn process_data() {
    let data = vec![1, 2, 3, 4, 5];  // data owns the vector
    process_list(data);              // Ownership moved to function
    // println!("{:?}", data);       // ❌ Error: data no longer owned here
}

fn process_list(mut list: Vec<i32>) {  // list now owns the vector
    list.push(6);
    // list is dropped here when function ends
}
```

### Understanding "Move" for C# Developers
```csharp
// C# - References are copied, objects stay in place
var original = new List<int> { 1, 2, 3 };
var reference = original;  // Both variables point to same object
original.Add(4);
Console.WriteLine(reference.Count);  // 4 - same object
```

```rust
// Rust - Ownership is transferred
let original = vec![1, 2, 3];
let moved = original;       // Ownership transferred
// println!("{:?}", original);  // ❌ Error: original no longer owns the data
println!("{:?}", moved);    // ✅ Works: moved now owns the data
```

### Copy Types vs Move Types
```rust
// Copy types (like C# value types) - copied, not moved
let x = 5;        // i32 implements Copy
let y = x;        // x is copied to y
println!("{}", x); // ✅ Works: x is still valid

// Move types (like C# reference types) - moved, not copied  
let s1 = String::from("hello");  // String doesn't implement Copy
let s2 = s1;                     // s1 is moved to s2
// println!("{}", s1);           // ❌ Error: s1 is no longer valid
```

### Practical Example: Swapping Values
```csharp
// C# - Simple reference swapping
public void SwapLists(ref List<int> a, ref List<int> b)
{
    var temp = a;
    a = b;
    b = temp;
}
```

```rust
// Rust - Ownership-aware swapping
fn swap_vectors(a: &mut Vec<i32>, b: &mut Vec<i32>) {
    std::mem::swap(a, b);  // Built-in swap function
}

// Or manual approach
fn manual_swap() {
    let mut a = vec![1, 2, 3];
    let mut b = vec![4, 5, 6];
    
    let temp = a;  // Move a to temp
    a = b;         // Move b to a
    b = temp;      // Move temp to b
    
    println!("a: {:?}, b: {:?}", a, b);
}
```

***

## Borrowing Basics

Borrowing is like getting a reference in C#, but with compile-time safety guarantees.

### C# Reference Parameters
```csharp
// C# - ref and out parameters
public void ModifyValue(ref int value)
{
    value += 10;
}

public void ReadValue(in int value)  // readonly reference
{
    Console.WriteLine(value);
}

public bool TryParse(string input, out int result)
{
    return int.TryParse(input, out result);
}
```

### Rust Borrowing
```rust
// Rust - borrowing with & and &mut
fn modify_value(value: &mut i32) {  // Mutable borrow
    *value += 10;
}

fn read_value(value: &i32) {        // Immutable borrow
    println!("{}", value);
}

fn main() {
    let mut x = 5;
    
    read_value(&x);      // Borrow immutably
    modify_value(&mut x); // Borrow mutably
    
    println!("{}", x);   // x is still owned here
}
```

### Borrowing Rules (Enforced at Compile Time!)
```rust
fn borrowing_rules() {
    let mut data = vec![1, 2, 3];
    
    // Rule 1: Multiple immutable borrows are OK
    let r1 = &data;
    let r2 = &data;
    println!("{:?} {:?}", r1, r2);  // ✅ Works
    
    // Rule 2: Only one mutable borrow at a time
    let r3 = &mut data;
    // let r4 = &mut data;  // ❌ Error: cannot borrow mutably twice
    // let r5 = &data;      // ❌ Error: cannot borrow immutably while borrowed mutably
    
    r3.push(4);  // Use the mutable borrow
    // r3 goes out of scope here
    
    // Rule 3: Can borrow again after previous borrows end
    let r6 = &data;  // ✅ Works now
    println!("{:?}", r6);
}
```

### C# vs Rust: Reference Safety
```csharp
// C# - Potential runtime errors
public class ReferenceSafety
{
    private List<int> data = new List<int>();
    
    public List<int> GetData() => data;  // Returns reference to internal data
    
    public void UnsafeExample()
    {
        var reference = GetData();
        
        // Another thread could modify data here!
        Thread.Sleep(1000);
        
        // reference might be invalid or changed
        reference.Add(42);  // Potential race condition
    }
}
```

```rust
// Rust - Compile-time safety
pub struct SafeContainer {
    data: Vec<i32>,
}

impl SafeContainer {
    // Return immutable borrow - caller can't modify
    pub fn get_data(&self) -> &Vec<i32> {
        &self.data
    }
    
    // Return mutable borrow - exclusive access guaranteed
    pub fn get_data_mut(&mut self) -> &mut Vec<i32> {
        &mut self.data
    }
}

fn safe_example() {
    let mut container = SafeContainer { data: vec![1, 2, 3] };
    
    let reference = container.get_data();
    // container.get_data_mut();  // ❌ Error: can't borrow mutably while immutably borrowed
    
    println!("{:?}", reference);  // Use immutable reference
    // reference goes out of scope here
    
    let mut_reference = container.get_data_mut();  // ✅ Now OK
    mut_reference.push(4);
}
```

***

## References vs Pointers

### C# Pointers (Unsafe Context)
```csharp
// C# unsafe pointers (rarely used)
unsafe void UnsafeExample()
{
    int value = 42;
    int* ptr = &value;  // Pointer to value
    *ptr = 100;         // Dereference and modify
    Console.WriteLine(value);  // 100
}
```

### Rust References (Safe by Default)
```rust
// Rust references (always safe)
fn safe_example() {
    let mut value = 42;
    let ptr = &mut value;  // Mutable reference
    *ptr = 100;           // Dereference and modify
    println!("{}", value); // 100
}

// No "unsafe" keyword needed - borrow checker ensures safety
```

### Lifetime Basics for C# Developers
```csharp
// C# - Can return references that might become invalid
public class LifetimeIssues
{
    public string GetFirstWord(string input)
    {
        return input.Split(' ')[0];  // Returns new string (safe)
    }
    
    public unsafe char* GetFirstChar(string input)
    {
        // This would be dangerous - returning pointer to managed memory
        fixed (char* ptr = input)
            return ptr;  // ❌ Bad: ptr becomes invalid after method ends
    }
}
```

```rust
// Rust - Lifetime checking prevents dangling references
fn get_first_word(input: &str) -> &str {
    input.split_whitespace().next().unwrap_or("")
    // ✅ Safe: returned reference has same lifetime as input
}

fn invalid_reference() -> &str {
    let temp = String::from("hello");
    &temp  // ❌ Compile error: temp doesn't live long enough
    // temp would be dropped at end of function
}

fn valid_reference() -> String {
    let temp = String::from("hello");
    temp  // ✅ Works: ownership is transferred to caller
}
```

***

## Move Semantics

### C# Value Types vs Reference Types
```csharp
// C# - Value types are copied
struct Point
{
    public int X { get; set; }
    public int Y { get; set; }
}

var p1 = new Point { X = 1, Y = 2 };
var p2 = p1;  // Copy
p2.X = 10;
Console.WriteLine(p1.X);  // Still 1

// C# - Reference types share the object
var list1 = new List<int> { 1, 2, 3 };
var list2 = list1;  // Reference copy
list2.Add(4);
Console.WriteLine(list1.Count);  // 4 - same object
```

### Rust Move Semantics
```rust
// Rust - Move by default for non-Copy types
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn move_example() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1;  // Move (not copy)
    // println!("{:?}", p1);  // ❌ Error: p1 was moved
    println!("{:?}", p2);    // ✅ Works
}

// To enable copying, implement Copy trait
#[derive(Debug, Copy, Clone)]
struct CopyablePoint {
    x: i32,
    y: i32,
}

fn copy_example() {
    let p1 = CopyablePoint { x: 1, y: 2 };
    let p2 = p1;  // Copy (because it implements Copy)
    println!("{:?}", p1);  // ✅ Works
    println!("{:?}", p2);  // ✅ Works
}
```

### When Values Are Moved
```rust
fn demonstrate_moves() {
    let s = String::from("hello");
    
    // 1. Assignment moves
    let s2 = s;  // s moved to s2
    
    // 2. Function calls move
    take_ownership(s2);  // s2 moved into function
    
    // 3. Returning from functions moves
    let s3 = give_ownership();  // Return value moved to s3
    
    println!("{}", s3);  // s3 is valid
}

fn take_ownership(s: String) {
    println!("{}", s);
    // s is dropped here
}

fn give_ownership() -> String {
    String::from("yours")  // Ownership moved to caller
}
```

### Avoiding Moves with Borrowing
```rust
fn demonstrate_borrowing() {
    let s = String::from("hello");
    
    // Borrow instead of move
    let len = calculate_length(&s);  // s is borrowed
    println!("'{}' has length {}", s, len);  // s is still valid
}

fn calculate_length(s: &String) -> usize {
    s.len()  // s is not owned, so it's not dropped
}
```

***

## Functions vs Methods

### C# Function Declaration
```csharp
// C# - Methods in classes
public class Calculator
{
    // Instance method
    public int Add(int a, int b)
    {
        return a + b;
    }
    
    // Static method
    public static int Multiply(int a, int b)
    {
        return a * b;
    }
    
    // Method with ref parameter
    public void Increment(ref int value)
    {
        value++;
    }
}
```

### Rust Function Declaration
```rust
// Rust - Standalone functions
fn add(a: i32, b: i32) -> i32 {
    a + b  // No 'return' needed for final expression
}

fn multiply(a: i32, b: i32) -> i32 {
    return a * b;  // Explicit return is also fine
}

// Function with mutable reference
fn increment(value: &mut i32) {
    *value += 1;
}

fn main() {
    let result = add(5, 3);
    println!("5 + 3 = {}", result);
    
    let mut x = 10;
    increment(&mut x);
    println!("After increment: {}", x);
}
```

### Expression vs Statement (Important!)
```csharp
// C# - Statements vs expressions
public int GetValue()
{
    if (condition)
    {
        return 42;  // Statement
    }
    return 0;       // Statement
}
```

```rust
// Rust - Everything can be an expression
fn get_value(condition: bool) -> i32 {
    if condition {
        42  // Expression (no semicolon)
    } else {
        0   // Expression (no semicolon)
    }
    // The if-else block itself is an expression that returns a value
}

// Or even simpler
fn get_value_ternary(condition: bool) -> i32 {
    if condition { 42 } else { 0 }
}
```

### Function Parameters and Return Types
```rust
// No parameters, no return value (returns unit type ())
fn say_hello() {
    println!("Hello!");
}

// Multiple parameters
fn greet(name: &str, age: u32) {
    println!("{} is {} years old", name, age);
}

// Multiple return values using tuple
fn divide_and_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
    (dividend / divisor, dividend % divisor)
}

fn main() {
    let (quotient, remainder) = divide_and_remainder(10, 3);
    println!("10 ÷ 3 = {} remainder {}", quotient, remainder);
}
```

***

## Control Flow Basics

### Conditional Statements
```csharp
// C# if statements
int x = 5;
if (x > 10)
{
    Console.WriteLine("Big number");
}
else if (x > 5)
{
    Console.WriteLine("Medium number");
}
else
{
    Console.WriteLine("Small number");
}

// C# ternary operator
string message = x > 10 ? "Big" : "Small";
```

```rust
// Rust if expressions
let x = 5;
if x > 10 {
    println!("Big number");
} else if x > 5 {
    println!("Medium number");
} else {
    println!("Small number");
}

// Rust if as expression (like ternary)
let message = if x > 10 { "Big" } else { "Small" };

// Multiple conditions
let message = if x > 10 {
    "Big"
} else if x > 5 {
    "Medium"
} else {
    "Small"
};
```

### Loops
```csharp
// C# loops
// For loop
for (int i = 0; i < 5; i++)
{
    Console.WriteLine(i);
}

// Foreach loop
var numbers = new[] { 1, 2, 3, 4, 5 };
foreach (var num in numbers)
{
    Console.WriteLine(num);
}

// While loop
int count = 0;
while (count < 3)
{
    Console.WriteLine(count);
    count++;
}
```

```rust
// Rust loops
// Range-based for loop
for i in 0..5 {  // 0 to 4 (exclusive end)
    println!("{}", i);
}

// Iterate over collection
let numbers = vec![1, 2, 3, 4, 5];
for num in numbers {  // Takes ownership
    println!("{}", num);
}

// Iterate over references (more common)
let numbers = vec![1, 2, 3, 4, 5];
for num in &numbers {  // Borrows elements
    println!("{}", num);
}

// While loop
let mut count = 0;
while count < 3 {
    println!("{}", count);
    count += 1;
}

// Infinite loop with break
let mut counter = 0;
loop {
    if counter >= 3 {
        break;
    }
    println!("{}", counter);
    counter += 1;
}
```

### Loop Control
```csharp
// C# loop control
for (int i = 0; i < 10; i++)
{
    if (i == 3) continue;
    if (i == 7) break;
    Console.WriteLine(i);
}
```

```rust
// Rust loop control
for i in 0..10 {
    if i == 3 { continue; }
    if i == 7 { break; }
    println!("{}", i);
}

// Loop labels (for nested loops)
'outer: for i in 0..3 {
    'inner: for j in 0..3 {
        if i == 1 && j == 1 {
            break 'outer;  // Break out of outer loop
        }
        println!("i: {}, j: {}", i, j);
    }
}
```

***

## Pattern Matching Introduction

Pattern matching is much more powerful in Rust than switch statements in C#.

### C# Switch Statements
```csharp
// C# traditional switch
int value = 2;
switch (value)
{
    case 1:
        Console.WriteLine("One");
        break;
    case 2:
        Console.WriteLine("Two");
        break;
    default:
        Console.WriteLine("Other");
        break;
}

// C# 8+ switch expressions
string result = value switch
{
    1 => "One",
    2 => "Two",
    _ => "Other"
};
```

### Rust Match Expressions
```rust
// Rust match (must be exhaustive)
let value = 2;
match value {
    1 => println!("One"),
    2 => println!("Two"),
    _ => println!("Other"),  // _ is wildcard (like default)
}

// Match as expression (like switch expression)
let result = match value {
    1 => "One",
    2 => "Two",
    _ => "Other",
};

// Match multiple values
match value {
    1 | 2 => println!("One or Two"),  // Multiple patterns
    3..=5 => println!("Three to Five"), // Range pattern
    _ => println!("Other"),
}
```

### Destructuring with Match
```csharp
// C# tuple deconstruction
var point = (3, 4);
var (x, y) = point;
Console.WriteLine($"x: {x}, y: {y}");

// C# pattern matching with tuples
string classify = point switch
{
    (0, 0) => "Origin",
    (var a, 0) => $"On X-axis at {a}",
    (0, var b) => $"On Y-axis at {b}",
    _ => "Somewhere else"
};
```

```rust
// Rust tuple destructuring with match
let point = (3, 4);
match point {
    (0, 0) => println!("Origin"),
    (x, 0) => println!("On X-axis at {}", x),
    (0, y) => println!("On Y-axis at {}", y),
    (x, y) => println!("Point at ({}, {})", x, y),
}

// Match guards (conditions)
match point {
    (x, y) if x == y => println!("On diagonal"),
    (x, y) if x > y => println!("Above diagonal"),
    _ => println!("Below diagonal"),
}
```

***

## Error Handling Basics

This is a fundamental shift from C#'s exception model to Rust's explicit error handling.

### C# Exception Handling
```csharp
// C# - Exception-based error handling
public class FileProcessor
{
    public string ReadConfig(string path)
    {
        try
        {
            return File.ReadAllText(path);
        }
        catch (FileNotFoundException)
        {
            throw new InvalidOperationException("Config file not found");
        }
        catch (UnauthorizedAccessException)
        {
            throw new InvalidOperationException("Cannot access config file");
        }
    }
    
    public int ParseNumber(string input)
    {
        if (int.TryParse(input, out int result))
        {
            return result;
        }
        throw new ArgumentException("Invalid number format");
    }
}
```

### Rust Result-Based Error Handling
```rust
use std::fs;
use std::num::ParseIntError;

// Define custom error type
#[derive(Debug)]
enum ConfigError {
    FileNotFound,
    AccessDenied,
    InvalidFormat,
}

// Function that returns Result
fn read_config(path: &str) -> Result<String, ConfigError> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(_) => Err(ConfigError::FileNotFound),  // Simplified for example
    }
}

// Function that can fail
fn parse_number(input: &str) -> Result<i32, ParseIntError> {
    input.parse::<i32>()  // Returns Result<i32, ParseIntError>
}

fn main() {
    // Handle errors explicitly
    match read_config("config.txt") {
        Ok(content) => println!("Config: {}", content),
        Err(ConfigError::FileNotFound) => println!("Config file not found"),
        Err(error) => println!("Config error: {:?}", error),
    }
    
    // Handle parsing errors
    match parse_number("42") {
        Ok(num) => println!("Number: {}", num),
        Err(error) => println!("Parse error: {}", error),
    }
}
```

### The ? Operator (Like C#'s await)
```csharp
// C# - Exception propagation (implicit)
public async Task<string> ProcessFileAsync(string path)
{
    var content = await File.ReadAllTextAsync(path);  // Throws on error
    var processed = ProcessContent(content);          // Throws on error
    return processed;
}
```

```rust
// Rust - Error propagation with ?
fn process_file(path: &str) -> Result<String, ConfigError> {
    let content = read_config(path)?;  // ? propagates error if Err
    let processed = process_content(&content)?;  // ? propagates error if Err
    Ok(processed)  // Wrap success value in Ok
}

fn process_content(content: &str) -> Result<String, ConfigError> {
    if content.is_empty() {
        Err(ConfigError::InvalidFormat)
    } else {
        Ok(content.to_uppercase())
    }
}
```

### Option<T> for Nullable Values
```csharp
// C# - Nullable reference types
public string? FindUserName(int userId)
{
    var user = database.FindUser(userId);
    return user?.Name;  // Returns null if user not found
}

public void ProcessUser(int userId)
{
    string? name = FindUserName(userId);
    if (name != null)
    {
        Console.WriteLine($"User: {name}");
    }
    else
    {
        Console.WriteLine("User not found");
    }
}
```

```rust
// Rust - Option<T> for optional values
fn find_user_name(user_id: u32) -> Option<String> {
    // Simulate database lookup
    if user_id == 1 {
        Some("Alice".to_string())
    } else {
        None
    }
}

fn process_user(user_id: u32) {
    match find_user_name(user_id) {
        Some(name) => println!("User: {}", name),
        None => println!("User not found"),
    }
    
    // Or use if let (pattern matching shorthand)
    if let Some(name) = find_user_name(user_id) {
        println!("User: {}", name);
    } else {
        println!("User not found");
    }
}
```

### Combining Option and Result
```rust
fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b != 0.0 {
        Some(a / b)
    } else {
        None
    }
}

fn parse_and_divide(a_str: &str, b_str: &str) -> Result<Option<f64>, ParseFloatError> {
    let a: f64 = a_str.parse()?;  // Return parse error if invalid
    let b: f64 = b_str.parse()?;  // Return parse error if invalid
    Ok(safe_divide(a, b))         // Return Ok(Some(result)) or Ok(None)
}

use std::num::ParseFloatError;

fn main() {
    match parse_and_divide("10.0", "2.0") {
        Ok(Some(result)) => println!("Result: {}", result),
        Ok(None) => println!("Division by zero"),
        Err(error) => println!("Parse error: {}", error),
    }
}
```

***

## Vec<T> vs List<T>

Vec<T> is Rust's equivalent to C#'s List<T>, but with ownership semantics.

### C# List<T>
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

### Rust Vec<T>
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
fn safe_access(vec: &Vec<i32>, index: usize) -> Option<i32> {
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

HashMap is Rust's equivalent to C#'s Dictionary<K,V>.

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

## Arrays and Slices

Understanding the difference between arrays, slices, and vectors is crucial.

### C# Arrays
```csharp
// C# arrays
int[] numbers = new int[5];         // Fixed size, heap allocated
int[] initialized = { 1, 2, 3, 4, 5 }; // Array literal

// Access
numbers[0] = 10;
int first = numbers[0];

// Length
int length = numbers.Length;

// Array as parameter (reference type)
void ProcessArray(int[] array)
{
    array[0] = 99;  // Modifies original
}
```

### Rust Arrays, Slices, and Vectors
```rust
// 1. Arrays - Fixed size, stack allocated
let numbers: [i32; 5] = [1, 2, 3, 4, 5];  // Type: [i32; 5]
let zeros = [0; 10];                       // 10 zeros

// Access
let first = numbers[0];
// numbers[0] = 10;  // ❌ Error: arrays are immutable by default

let mut mut_array = [1, 2, 3, 4, 5];
mut_array[0] = 10;  // ✅ Works with mut

// 2. Slices - Views into arrays or vectors
let slice: &[i32] = &numbers[1..4];  // Elements 1, 2, 3
let all_slice: &[i32] = &numbers;    // Entire array as slice

// 3. Vectors - Dynamic size, heap allocated (covered earlier)
let mut vec = vec![1, 2, 3, 4, 5];
vec.push(6);  // Can grow
```

### Slices as Function Parameters
```csharp
// C# - Method that works with arrays
public void ProcessNumbers(int[] numbers)
{
    for (int i = 0; i < numbers.Length; i++)
    {
        Console.WriteLine(numbers[i]);
    }
}

// Works with arrays only
ProcessNumbers(new int[] { 1, 2, 3 });
```

```rust
// Rust - Function that works with any sequence
fn process_numbers(numbers: &[i32]) {  // Slice parameter
    for (i, num) in numbers.iter().enumerate() {
        println!("Index {}: {}", i, num);
    }
}

fn main() {
    let array = [1, 2, 3, 4, 5];
    let vec = vec![1, 2, 3, 4, 5];
    
    // Same function works with both!
    process_numbers(&array);      // Array as slice
    process_numbers(&vec);        // Vector as slice
    process_numbers(&vec[1..4]);  // Partial slice
}
```

### String Slices (&str) Revisited
```rust
// String and &str relationship
fn string_slice_example() {
    let owned = String::from("Hello, World!");
    let slice: &str = &owned[0..5];      // "Hello"
    let slice2: &str = &owned[7..];      // "World!"
    
    println!("{}", slice);   // "Hello"
    println!("{}", slice2);  // "World!"
    
    // Function that accepts any string type
    print_string("String literal");      // &str
    print_string(&owned);               // String as &str
    print_string(slice);                // &str slice
}

fn print_string(s: &str) {
    println!("{}", s);
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

***

## Structs vs Classes

Structs in Rust are similar to classes in C#, but with some key differences around ownership and methods.

### C# Class Definition
```csharp
// C# class with properties and methods
public class Person
{
    public string Name { get; set; }
    public int Age { get; set; }
    public List<string> Hobbies { get; set; }
    
    public Person(string name, int age)
    {
        Name = name;
        Age = age;
        Hobbies = new List<string>();
    }
    
    public void AddHobby(string hobby)
    {
        Hobbies.Add(hobby);
    }
    
    public string GetInfo()
    {
        return $"{Name} is {Age} years old";
    }
}
```

### Rust Struct Definition
```rust
// Rust struct with associated functions and methods
#[derive(Debug)]  // Automatically implement Debug trait
pub struct Person {
    pub name: String,    // Public field
    pub age: u32,        // Public field
    hobbies: Vec<String>, // Private field (no pub)
}

impl Person {
    // Associated function (like static method)
    pub fn new(name: String, age: u32) -> Person {
        Person {
            name,
            age,
            hobbies: Vec::new(),
        }
    }
    
    // Method (takes &self, &mut self, or self)
    pub fn add_hobby(&mut self, hobby: String) {
        self.hobbies.push(hobby);
    }
    
    // Method that borrows immutably
    pub fn get_info(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
    
    // Getter for private field
    pub fn hobbies(&self) -> &Vec<String> {
        &self.hobbies
    }
}
```

### Creating and Using Instances
```csharp
// C# object creation and usage
var person = new Person("Alice", 30);
person.AddHobby("Reading");
person.AddHobby("Swimming");

Console.WriteLine(person.GetInfo());
Console.WriteLine($"Hobbies: {string.Join(", ", person.Hobbies)}");

// Modify properties directly
person.Age = 31;
```

```rust
// Rust struct creation and usage
let mut person = Person::new("Alice".to_string(), 30);
person.add_hobby("Reading".to_string());
person.add_hobby("Swimming".to_string());

println!("{}", person.get_info());
println!("Hobbies: {:?}", person.hobbies());

// Modify public fields directly
person.age = 31;

// Debug print the entire struct
println!("{:?}", person);
```

### Struct Initialization Patterns
```csharp
// C# object initialization
var person = new Person("Bob", 25)
{
    Hobbies = new List<string> { "Gaming", "Coding" }
};

// Anonymous types
var anonymous = new { Name = "Charlie", Age = 35 };
```

```rust
// Rust struct initialization
let person = Person {
    name: "Bob".to_string(),
    age: 25,
    hobbies: vec!["Gaming".to_string(), "Coding".to_string()],
};

// Struct update syntax (like object spread)
let older_person = Person {
    age: 26,
    ..person  // Use remaining fields from person (moves person!)
};

// Tuple structs (like anonymous types)
#[derive(Debug)]
struct Point(i32, i32);

let point = Point(10, 20);
println!("Point: ({}, {})", point.0, point.1);
```

***

## Methods and Associated Functions

Understanding the difference between methods and associated functions is key.

### C# Method Types
```csharp
public class Calculator
{
    private int memory = 0;
    
    // Instance method
    public int Add(int a, int b)
    {
        return a + b;
    }
    
    // Instance method that uses state
    public void StoreInMemory(int value)
    {
        memory = value;
    }
    
    // Static method
    public static int Multiply(int a, int b)
    {
        return a * b;
    }
    
    // Static factory method
    public static Calculator CreateWithMemory(int initialMemory)
    {
        var calc = new Calculator();
        calc.memory = initialMemory;
        return calc;
    }
}
```

### Rust Method Types
```rust
#[derive(Debug)]
pub struct Calculator {
    memory: i32,
}

impl Calculator {
    // Associated function (like static method) - no self parameter
    pub fn new() -> Calculator {
        Calculator { memory: 0 }
    }
    
    // Associated function with parameters
    pub fn with_memory(initial_memory: i32) -> Calculator {
        Calculator { memory: initial_memory }
    }
    
    // Method that borrows immutably (&self)
    pub fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
    
    // Method that borrows mutably (&mut self)
    pub fn store_in_memory(&mut self, value: i32) {
        self.memory = value;
    }
    
    // Method that takes ownership (self)
    pub fn into_memory(self) -> i32 {
        self.memory  // Calculator is consumed
    }
    
    // Getter method
    pub fn memory(&self) -> i32 {
        self.memory
    }
}

fn main() {
    // Associated functions called with ::
    let mut calc = Calculator::new();
    let calc2 = Calculator::with_memory(42);
    
    // Methods called with .
    let result = calc.add(5, 3);
    calc.store_in_memory(result);
    
    println!("Memory: {}", calc.memory());
    
    // Consuming method
    let memory_value = calc.into_memory();  // calc is no longer usable
    println!("Final memory: {}", memory_value);
}
```

### Method Receiver Types Explained
```rust
impl Person {
    // &self - Immutable borrow (most common)
    // Use when you only need to read the data
    pub fn get_name(&self) -> &str {
        &self.name
    }
    
    // &mut self - Mutable borrow
    // Use when you need to modify the data
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    
    // self - Take ownership (less common)
    // Use when you want to consume the struct
    pub fn consume(self) -> String {
        self.name  // Person is moved, no longer accessible
    }
}

fn method_examples() {
    let mut person = Person::new("Alice".to_string(), 30);
    
    // Immutable borrow
    let name = person.get_name();  // person can still be used
    println!("Name: {}", name);
    
    // Mutable borrow
    person.set_name("Alice Smith".to_string());  // person can still be used
    
    // Taking ownership
    let final_name = person.consume();  // person is no longer usable
    println!("Final name: {}", final_name);
}
```

***

## Implementing Behavior

### C# Interface Implementation
```csharp
// C# interface
public interface IDrawable
{
    void Draw();
    double GetArea();
}

public class Circle : IDrawable
{
    public double Radius { get; set; }
    
    public Circle(double radius)
    {
        Radius = radius;
    }
    
    public void Draw()
    {
        Console.WriteLine($"Drawing a circle with radius {Radius}");
    }
    
    public double GetArea()
    {
        return Math.PI * Radius * Radius;
    }
}
```

### Rust Trait Implementation (Preview)
```rust
// Rust trait (like interface)
trait Drawable {
    fn draw(&self);
    fn get_area(&self) -> f64;
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Circle {
        Circle { radius }
    }
}

// Implement trait for Circle
impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
    
    fn get_area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

fn main() {
    let circle = Circle::new(5.0);
    circle.draw();
    println!("Area: {}", circle.get_area());
}
```

### Multiple Implementations
```csharp
// C# - Class implementing multiple interfaces
public interface IComparable<T>
{
    int CompareTo(T other);
}

public class Person : IDrawable, IComparable<Person>
{
    public string Name { get; set; }
    public int Age { get; set; }
    
    public void Draw()
    {
        Console.WriteLine($"Drawing person: {Name}");
    }
    
    public double GetArea()
    {
        return 0.0; // People don't have area!
    }
    
    public int CompareTo(Person other)
    {
        return Age.CompareTo(other.Age);
    }
}
```

```rust
// Rust - Multiple trait implementations
use std::cmp::Ordering;

impl Drawable for Person {
    fn draw(&self) {
        println!("Drawing person: {}", self.name);
    }
    
    fn get_area(&self) -> f64 {
        0.0  // People don't have area!
    }
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.age.partial_cmp(&other.age)
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.age == other.age
    }
}

fn main() {
    let mut people = vec![
        Person::new("Alice".to_string(), 30),
        Person::new("Bob".to_string(), 25),
        Person::new("Charlie".to_string(), 35),
    ];
    
    people.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    for person in &people {
        person.draw();
    }
}
```

***

## Constructor Patterns

### C# Constructor Patterns
```csharp
public class Configuration
{
    public string DatabaseUrl { get; set; }
    public int MaxConnections { get; set; }
    public bool EnableLogging { get; set; }
    
    // Default constructor
    public Configuration()
    {
        DatabaseUrl = "localhost";
        MaxConnections = 10;
        EnableLogging = false;
    }
    
    // Parameterized constructor
    public Configuration(string databaseUrl, int maxConnections)
    {
        DatabaseUrl = databaseUrl;
        MaxConnections = maxConnections;
        EnableLogging = false;
    }
    
    // Factory method
    public static Configuration ForProduction()
    {
        return new Configuration("prod.db.server", 100)
        {
            EnableLogging = true
        };
    }
}
```

### Rust Constructor Patterns
```rust
#[derive(Debug)]
pub struct Configuration {
    pub database_url: String,
    pub max_connections: u32,
    pub enable_logging: bool,
}

impl Configuration {
    // Default constructor
    pub fn new() -> Configuration {
        Configuration {
            database_url: "localhost".to_string(),
            max_connections: 10,
            enable_logging: false,
        }
    }
    
    // Parameterized constructor
    pub fn with_database(database_url: String, max_connections: u32) -> Configuration {
        Configuration {
            database_url,
            max_connections,
            enable_logging: false,
        }
    }
    
    // Factory method
    pub fn for_production() -> Configuration {
        Configuration {
            database_url: "prod.db.server".to_string(),
            max_connections: 100,
            enable_logging: true,
        }
    }
    
    // Builder pattern method
    pub fn enable_logging(mut self) -> Configuration {
        self.enable_logging = true;
        self  // Return self for chaining
    }
    
    pub fn max_connections(mut self, count: u32) -> Configuration {
        self.max_connections = count;
        self
    }
}

// Default trait implementation
impl Default for Configuration {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    // Different construction patterns
    let config1 = Configuration::new();
    let config2 = Configuration::with_database("localhost:5432".to_string(), 20);
    let config3 = Configuration::for_production();
    
    // Builder pattern
    let config4 = Configuration::new()
        .enable_logging()
        .max_connections(50);
    
    // Using Default trait
    let config5 = Configuration::default();
    
    println!("{:?}", config4);
}
```

### Builder Pattern Implementation
```rust
// More complex builder pattern
#[derive(Debug)]
pub struct DatabaseConfig {
    host: String,
    port: u16,
    username: String,
    password: Option<String>,
    ssl_enabled: bool,
    timeout_seconds: u64,
}

pub struct DatabaseConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    username: Option<String>,
    password: Option<String>,
    ssl_enabled: bool,
    timeout_seconds: u64,
}

impl DatabaseConfigBuilder {
    pub fn new() -> Self {
        DatabaseConfigBuilder {
            host: None,
            port: None,
            username: None,
            password: None,
            ssl_enabled: false,
            timeout_seconds: 30,
        }
    }
    
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }
    
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }
    
    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }
    
    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }
    
    pub fn enable_ssl(mut self) -> Self {
        self.ssl_enabled = true;
        self
    }
    
    pub fn timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = seconds;
        self
    }
    
    pub fn build(self) -> Result<DatabaseConfig, String> {
        let host = self.host.ok_or("Host is required")?;
        let port = self.port.ok_or("Port is required")?;
        let username = self.username.ok_or("Username is required")?;
        
        Ok(DatabaseConfig {
            host,
            port,
            username,
            password: self.password,
            ssl_enabled: self.ssl_enabled,
            timeout_seconds: self.timeout_seconds,
        })
    }
}

fn main() {
    let config = DatabaseConfigBuilder::new()
        .host("localhost")
        .port(5432)
        .username("admin")
        .password("secret123")
        .enable_ssl()
        .timeout(60)
        .build()
        .expect("Failed to build config");
    
    println!("{:?}", config);
}
```

***

## Enums and Pattern Matching

Rust enums are much more powerful than C# enums - they can hold data and are the foundation of type-safe programming.

### C# Enum Limitations
```csharp
// C# enum - just named constants
public enum Status
{
    Pending,
    Approved,
    Rejected
}

// C# enum with backing values
public enum HttpStatusCode
{
    OK = 200,
    NotFound = 404,
    InternalServerError = 500
}

// Need separate classes for complex data
public abstract class Result
{
    public abstract bool IsSuccess { get; }
}

public class Success : Result
{
    public string Value { get; }
    public override bool IsSuccess => true;
    
    public Success(string value)
    {
        Value = value;
    }
}

public class Error : Result
{
    public string Message { get; }
    public override bool IsSuccess => false;
    
    public Error(string message)
    {
        Message = message;
    }
}
```

### Rust Enum Power
```rust
// Simple enum (like C# enum)
#[derive(Debug, PartialEq)]
enum Status {
    Pending,
    Approved,
    Rejected,
}

// Enum with data (this is where Rust shines!)
#[derive(Debug)]
enum Result<T, E> {
    Ok(T),      // Success variant holding value of type T
    Err(E),     // Error variant holding error of type E
}

// Complex enum with different data types
#[derive(Debug)]
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },   // Struct-like variant
    Write(String),             // Tuple-like variant
    ChangeColor(i32, i32, i32), // Multiple values
}

// Real-world example: HTTP Response
#[derive(Debug)]
enum HttpResponse {
    Ok { body: String, headers: Vec<String> },
    NotFound { path: String },
    InternalError { message: String, code: u16 },
    Redirect { location: String },
}
```

### Pattern Matching with Match
```csharp
// C# switch statement (limited)
public string HandleStatus(Status status)
{
    switch (status)
    {
        case Status.Pending:
            return "Waiting for approval";
        case Status.Approved:
            return "Request approved";
        case Status.Rejected:
            return "Request rejected";
        default:
            return "Unknown status"; // Always need default
    }
}

// C# pattern matching (C# 8+)
public string HandleResult(Result result)
{
    return result switch
    {
        Success success => $"Success: {success.Value}",
        Error error => $"Error: {error.Message}",
        _ => "Unknown result" // Still need catch-all
    };
}
```

```rust
// Rust match - exhaustive and powerful
fn handle_status(status: Status) -> String {
    match status {
        Status::Pending => "Waiting for approval".to_string(),
        Status::Approved => "Request approved".to_string(),
        Status::Rejected => "Request rejected".to_string(),
        // No default needed - compiler ensures exhaustiveness
    }
}

// Pattern matching with data extraction
fn handle_result<T, E>(result: Result<T, E>) -> String 
where 
    T: std::fmt::Debug,
    E: std::fmt::Debug,
{
    match result {
        Result::Ok(value) => format!("Success: {:?}", value),
        Result::Err(error) => format!("Error: {:?}", error),
        // Exhaustive - no default needed
    }
}

// Complex pattern matching
fn handle_message(msg: Message) -> String {
    match msg {
        Message::Quit => "Goodbye!".to_string(),
        Message::Move { x, y } => format!("Move to ({}, {})", x, y),
        Message::Write(text) => format!("Write: {}", text),
        Message::ChangeColor(r, g, b) => format!("Change color to RGB({}, {}, {})", r, g, b),
    }
}

// HTTP response handling
fn handle_http_response(response: HttpResponse) -> String {
    match response {
        HttpResponse::Ok { body, headers } => {
            format!("Success! Body: {}, Headers: {:?}", body, headers)
        },
        HttpResponse::NotFound { path } => {
            format!("404: Path '{}' not found", path)
        },
        HttpResponse::InternalError { message, code } => {
            format!("Error {}: {}", code, message)
        },
        HttpResponse::Redirect { location } => {
            format!("Redirect to: {}", location)
        },
    }
}
```

### Guards and Advanced Patterns
```rust
// Pattern matching with guards
fn describe_number(x: i32) -> String {
    match x {
        n if n < 0 => "negative".to_string(),
        0 => "zero".to_string(),
        n if n < 10 => "single digit".to_string(),
        n if n < 100 => "double digit".to_string(),
        _ => "large number".to_string(),
    }
}

// Matching ranges
fn describe_age(age: u32) -> String {
    match age {
        0..=12 => "child".to_string(),
        13..=19 => "teenager".to_string(),
        20..=64 => "adult".to_string(),
        65.. => "senior".to_string(),
    }
}

// Destructuring structs and tuples
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn describe_point(point: Point) -> String {
    match point {
        Point { x: 0, y: 0 } => "origin".to_string(),
        Point { x: 0, y } => format!("on y-axis at y={}", y),
        Point { x, y: 0 } => format!("on x-axis at x={}", x),
        Point { x, y } if x == y => format!("on diagonal at ({}, {})", x, y),
        Point { x, y } => format!("point at ({}, {})", x, y),
    }
}
```

### Option and Result Types
```csharp
// C# nullable reference types (C# 8+)
public class PersonService
{
    private Dictionary<int, string> people = new();
    
    public string? FindPerson(int id)
    {
        return people.TryGetValue(id, out string? name) ? name : null;
    }
    
    public string GetPersonOrDefault(int id)
    {
        return FindPerson(id) ?? "Unknown";
    }
    
    // Exception-based error handling
    public void SavePerson(int id, string name)
    {
        if (string.IsNullOrEmpty(name))
            throw new ArgumentException("Name cannot be empty");
        
        people[id] = name;
    }
}
```

```rust
use std::collections::HashMap;

// Rust uses Option<T> instead of null
struct PersonService {
    people: HashMap<i32, String>,
}

impl PersonService {
    fn new() -> Self {
        PersonService {
            people: HashMap::new(),
        }
    }
    
    // Returns Option<T> - no null!
    fn find_person(&self, id: i32) -> Option<&String> {
        self.people.get(&id)
    }
    
    // Pattern matching on Option
    fn get_person_or_default(&self, id: i32) -> String {
        match self.find_person(id) {
            Some(name) => name.clone(),
            None => "Unknown".to_string(),
        }
    }
    
    // Using Option methods (more functional style)
    fn get_person_or_default_functional(&self, id: i32) -> String {
        self.find_person(id)
            .map(|name| name.clone())
            .unwrap_or_else(|| "Unknown".to_string())
    }
    
    // Result<T, E> for error handling
    fn save_person(&mut self, id: i32, name: String) -> Result<(), String> {
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        
        self.people.insert(id, name);
        Ok(())
    }
    
    // Chaining operations
    fn get_person_length(&self, id: i32) -> Option<usize> {
        self.find_person(id).map(|name| name.len())
    }
}

fn main() {
    let mut service = PersonService::new();
    
    // Handle Result
    match service.save_person(1, "Alice".to_string()) {
        Ok(()) => println!("Person saved successfully"),
        Err(error) => println!("Error: {}", error),
    }
    
    // Handle Option
    match service.find_person(1) {
        Some(name) => println!("Found: {}", name),
        None => println!("Person not found"),
    }
    
    // Functional style with Option
    let name_length = service.get_person_length(1)
        .unwrap_or(0);
    println!("Name length: {}", name_length);
    
    // Question mark operator for early returns
    fn try_operation(service: &mut PersonService) -> Result<String, String> {
        service.save_person(2, "Bob".to_string())?; // Early return if error
        let name = service.find_person(2).ok_or("Person not found")?; // Convert Option to Result
        Ok(format!("Hello, {}", name))
    }
    
    match try_operation(&mut service) {
        Ok(message) => println!("{}", message),
        Err(error) => println!("Operation failed: {}", error),
    }
}
```

### Custom Error Types
```rust
// Define custom error enum
#[derive(Debug)]
enum PersonError {
    NotFound(i32),
    InvalidName(String),
    DatabaseError(String),
}

impl std::fmt::Display for PersonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PersonError::NotFound(id) => write!(f, "Person with ID {} not found", id),
            PersonError::InvalidName(name) => write!(f, "Invalid name: '{}'", name),
            PersonError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl std::error::Error for PersonError {}

// Enhanced PersonService with custom errors
impl PersonService {
    fn save_person_enhanced(&mut self, id: i32, name: String) -> Result<(), PersonError> {
        if name.is_empty() || name.len() > 50 {
            return Err(PersonError::InvalidName(name));
        }
        
        // Simulate database operation that might fail
        if id < 0 {
            return Err(PersonError::DatabaseError("Negative IDs not allowed".to_string()));
        }
        
        self.people.insert(id, name);
        Ok(())
    }
    
    fn find_person_enhanced(&self, id: i32) -> Result<&String, PersonError> {
        self.people.get(&id).ok_or(PersonError::NotFound(id))
    }
}

fn demo_error_handling() {
    let mut service = PersonService::new();
    
    // Handle different error types
    match service.save_person_enhanced(-1, "Invalid".to_string()) {
        Ok(()) => println!("Success"),
        Err(PersonError::NotFound(id)) => println!("Not found: {}", id),
        Err(PersonError::InvalidName(name)) => println!("Invalid name: {}", name),
        Err(PersonError::DatabaseError(msg)) => println!("DB Error: {}", msg),
    }
}
```

***

## Modules and Crates: Code Organization

Understanding Rust's module system is essential for organizing code and managing dependencies. For C# developers, this is analogous to understanding namespaces, assemblies, and NuGet packages.

### Rust Modules vs C# Namespaces

#### C# Namespace Organization
```csharp
// File: Models/User.cs
namespace MyApp.Models
{
    public class User
    {
        public string Name { get; set; }
        public int Age { get; set; }
    }
}

// File: Services/UserService.cs
using MyApp.Models;

namespace MyApp.Services
{
    public class UserService
    {
        public User CreateUser(string name, int age)
        {
            return new User { Name = name, Age = age };
        }
    }
}

// File: Program.cs
using MyApp.Models;
using MyApp.Services;

namespace MyApp
{
    class Program
    {
        static void Main(string[] args)
        {
            var service = new UserService();
            var user = service.CreateUser("Alice", 30);
        }
    }
}
```

#### Rust Module Organization
```rust
// File: src/models.rs
pub struct User {
    pub name: String,
    pub age: u32,
}

impl User {
    pub fn new(name: String, age: u32) -> User {
        User { name, age }
    }
}

// File: src/services.rs
use crate::models::User;

pub struct UserService;

impl UserService {
    pub fn create_user(name: String, age: u32) -> User {
        User::new(name, age)
    }
}

// File: src/lib.rs (or main.rs)
pub mod models;
pub mod services;

use models::User;
use services::UserService;

fn main() {
    let service = UserService;
    let user = UserService::create_user("Alice".to_string(), 30);
}
```

### Module Hierarchy and Visibility

#### C# Visibility Modifiers
```csharp
namespace MyApp.Data
{
    // public - accessible from anywhere
    public class Repository
    {
        // private - only within this class
        private string connectionString;
        
        // internal - within this assembly
        internal void Connect() { }
        
        // protected - this class and subclasses
        protected virtual void Initialize() { }
        
        // public - accessible from anywhere
        public void Save(object data) { }
    }
}
```

#### Rust Visibility Rules
```rust
// Everything is private by default in Rust
mod data {
    struct Repository {  // Private struct
        connection_string: String,  // Private field
    }
    
    impl Repository {
        fn new() -> Repository {  // Private function
            Repository {
                connection_string: "localhost".to_string(),
            }
        }
        
        pub fn connect(&self) {  // Public method
            // Only accessible within this module and its children
        }
        
        pub(crate) fn initialize(&self) {  // Crate-level public
            // Accessible anywhere in this crate
        }
        
        pub(super) fn internal_method(&self) {  // Parent module public
            // Accessible in parent module
        }
    }
    
    // Public struct - accessible from outside the module
    pub struct PublicRepository {
        pub data: String,  // Public field
        private_data: String,  // Private field (no pub)
    }
}

pub use data::PublicRepository;  // Re-export for external use
```

### Module File Organization

#### C# Project Structure
```
MyApp/
├── MyApp.csproj
├── Models/
│   ├── User.cs
│   └── Product.cs
├── Services/
│   ├── UserService.cs
│   └── ProductService.cs
├── Controllers/
│   └── ApiController.cs
└── Program.cs
```

#### Rust Module File Structure
```
my_app/
├── Cargo.toml
└── src/
    ├── main.rs (or lib.rs)
    ├── models/
    │   ├── mod.rs        // Module declaration
    │   ├── user.rs
    │   └── product.rs
    ├── services/
    │   ├── mod.rs        // Module declaration
    │   ├── user_service.rs
    │   └── product_service.rs
    └── controllers/
        ├── mod.rs
        └── api_controller.rs
```

#### Module Declaration Patterns
```rust
// src/models/mod.rs
pub mod user;      // Declares user.rs as a submodule
pub mod product;   // Declares product.rs as a submodule

// Re-export commonly used types
pub use user::User;
pub use product::Product;

// src/main.rs
mod models;     // Declares models/ as a module
mod services;   // Declares services/ as a module

// Import specific items
use models::{User, Product};
use services::UserService;

// Or import the entire module
use models::user::*;  // Import all public items from user module
```

***

## Crates vs .NET Assemblies

### Understanding Crates
In Rust, a **crate** is the fundamental unit of compilation and code distribution, similar to how an **assembly** works in .NET.

#### C# Assembly Model
```csharp
// MyLibrary.dll - Compiled assembly
namespace MyLibrary
{
    public class Calculator
    {
        public int Add(int a, int b) => a + b;
    }
}

// MyApp.exe - Executable assembly that references MyLibrary.dll
using MyLibrary;

class Program
{
    static void Main()
    {
        var calc = new Calculator();
        Console.WriteLine(calc.Add(2, 3));
    }
}
```

#### Rust Crate Model
```toml
# Cargo.toml for library crate
[package]
name = "my_calculator"
version = "0.1.0"
edition = "2021"

[lib]
name = "my_calculator"
```

```rust
// src/lib.rs - Library crate
pub struct Calculator;

impl Calculator {
    pub fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}
```

```toml
# Cargo.toml for binary crate that uses the library
[package]
name = "my_app"
version = "0.1.0"
edition = "2021"

[dependencies]
my_calculator = { path = "../my_calculator" }
```

```rust
// src/main.rs - Binary crate
use my_calculator::Calculator;

fn main() {
    let calc = Calculator;
    println!("{}", calc.add(2, 3));
}
```

### Crate Types Comparison

| C# Concept | Rust Equivalent | Purpose |
|------------|----------------|---------|
| Class Library (.dll) | Library crate | Reusable code |
| Console App (.exe) | Binary crate | Executable program |
| NuGet Package | Published crate | Distribution unit |
| Assembly (.dll/.exe) | Compiled crate | Compilation unit |
| Solution (.sln) | Workspace | Multi-project organization |

### Workspace vs Solution

#### C# Solution Structure
```xml
<!-- MySolution.sln structure -->
<Solution>
    <Project Include="WebApi/WebApi.csproj" />
    <Project Include="Business/Business.csproj" />
    <Project Include="DataAccess/DataAccess.csproj" />
    <Project Include="Tests/Tests.csproj" />
</Solution>
```

#### Rust Workspace Structure
```toml
# Cargo.toml at workspace root
[workspace]
members = [
    "web_api",
    "business",
    "data_access",
    "tests"
]

[workspace.dependencies]
serde = "1.0"           # Shared dependency versions
tokio = "1.0"
```

```toml
# web_api/Cargo.toml
[package]
name = "web_api"
version = "0.1.0"
edition = "2021"

[dependencies]
business = { path = "../business" }
serde = { workspace = true }    # Use workspace version
tokio = { workspace = true }
```

***

## Package Management: Cargo vs NuGet

### Dependency Declaration

#### C# NuGet Dependencies
```xml
<!-- MyApp.csproj -->
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <TargetFramework>net8.0</TargetFramework>
  </PropertyGroup>
  
  <PackageReference Include="Newtonsoft.Json" Version="13.0.3" />
  <PackageReference Include="Serilog" Version="3.0.1" />
  <PackageReference Include="Microsoft.AspNetCore.App" />
  
  <ProjectReference Include="../MyLibrary/MyLibrary.csproj" />
</Project>
```

#### Rust Cargo Dependencies
```toml
# Cargo.toml
[package]
name = "my_app"
version = "0.1.0"
edition = "2021"

[dependencies]
serde_json = "1.0"               # From crates.io (like NuGet)
serde = { version = "1.0", features = ["derive"] }  # With features
log = "0.4"
tokio = { version = "1.0", features = ["full"] }

# Local dependencies (like ProjectReference)
my_library = { path = "../my_library" }

# Git dependencies
my_git_crate = { git = "https://github.com/user/repo" }

# Development dependencies (like test packages)
[dev-dependencies]
criterion = "0.5"               # Benchmarking
proptest = "1.0"               # Property testing
```

### Version Management

#### C# Package Versioning
```xml
<!-- Centralized package management (Directory.Packages.props) -->
<Project>
  <PropertyGroup>
    <ManagePackageVersionsCentrally>true</ManagePackageVersionsCentrally>
  </PropertyGroup>
  
  <PackageVersion Include="Newtonsoft.Json" Version="13.0.3" />
  <PackageVersion Include="Serilog" Version="3.0.1" />
</Project>

<!-- packages.lock.json for reproducible builds -->
```

#### Rust Version Management
```toml
# Cargo.toml - Semantic versioning
[dependencies]
serde = "1.0"        # Compatible with 1.x.x (>=1.0.0, <2.0.0)
log = "0.4.17"       # Compatible with 0.4.x (>=0.4.17, <0.5.0)
regex = "=1.5.4"     # Exact version
chrono = "^0.4"      # Caret requirements (default)
uuid = "~1.3.0"      # Tilde requirements (>=1.3.0, <1.4.0)

# Cargo.lock - Exact versions for reproducible builds (auto-generated)
[[package]]
name = "serde"
version = "1.0.163"
# ... exact dependency tree
```

### Package Sources

#### C# Package Sources
```xml
<!-- nuget.config -->
<configuration>
  <packageSources>
    <add key="nuget.org" value="https://api.nuget.org/v3/index.json" />
    <add key="MyCompanyFeed" value="https://pkgs.dev.azure.com/company/_packaging/feed/nuget/v3/index.json" />
  </packageSources>
</configuration>
```

#### Rust Package Sources
```toml
# .cargo/config.toml
[source.crates-io]
replace-with = "my-awesome-registry"

[source.my-awesome-registry]
registry = "https://my-intranet:8080/index"

# Alternative registries
[registries]
my-registry = { index = "https://my-intranet:8080/index" }

# In Cargo.toml
[dependencies]
my_crate = { version = "1.0", registry = "my-registry" }
```

### Common Commands Comparison

| Task | C# Command | Rust Command |
|------|------------|-------------|
| Restore packages | `dotnet restore` | `cargo fetch` |
| Add package | `dotnet add package Newtonsoft.Json` | `cargo add serde_json` |
| Remove package | `dotnet remove package Newtonsoft.Json` | `cargo remove serde_json` |
| Update packages | `dotnet update` | `cargo update` |
| List packages | `dotnet list package` | `cargo tree` |
| Audit security | `dotnet list package --vulnerable` | `cargo audit` |
| Clean build | `dotnet clean` | `cargo clean` |

### Features: Conditional Compilation

#### C# Conditional Compilation
```csharp
#if DEBUG
    Console.WriteLine("Debug mode");
#elif RELEASE
    Console.WriteLine("Release mode");
#endif

// Project file features
<PropertyGroup Condition="'$(Configuration)'=='Debug'">
    <DefineConstants>DEBUG;TRACE</DefineConstants>
</PropertyGroup>
```

#### Rust Feature Gates
```toml
# Cargo.toml
[features]
default = ["json"]              # Default features
json = ["serde_json"]          # Feature that enables serde_json
xml = ["serde_xml"]            # Alternative serialization
advanced = ["json", "xml"]     # Composite feature

[dependencies]
serde_json = { version = "1.0", optional = true }
serde_xml = { version = "0.4", optional = true }
```

```rust
// Conditional compilation based on features
#[cfg(feature = "json")]
use serde_json;

#[cfg(feature = "xml")]
use serde_xml;

pub fn serialize_data(data: &MyStruct) -> String {
    #[cfg(feature = "json")]
    return serde_json::to_string(data).unwrap();
    
    #[cfg(feature = "xml")]
    return serde_xml::to_string(data).unwrap();
    
    #[cfg(not(any(feature = "json", feature = "xml")))]
    return "No serialization feature enabled".to_string();
}
```

### Using External Crates

#### Popular Crates for C# Developers

| C# Library | Rust Crate | Purpose |
|------------|------------|---------|
| Newtonsoft.Json | `serde_json` | JSON serialization |
| HttpClient | `reqwest` | HTTP client |
| Entity Framework | `diesel` / `sqlx` | ORM / SQL toolkit |
| NLog/Serilog | `log` + `env_logger` | Logging |
| xUnit/NUnit | Built-in `#[test]` | Unit testing |
| Moq | `mockall` | Mocking |
| Flurl | `url` | URL manipulation |
| Polly | `tower` | Resilience patterns |

#### Example: HTTP Client Migration
```csharp
// C# HttpClient usage
public class ApiClient
{
    private readonly HttpClient _httpClient;
    
    public async Task<User> GetUserAsync(int id)
    {
        var response = await _httpClient.GetAsync($"/users/{id}");
        var json = await response.Content.ReadAsStringAsync();
        return JsonConvert.DeserializeObject<User>(json);
    }
}
```

```rust
// Rust reqwest usage
use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    id: u32,
    name: String,
}

struct ApiClient {
    client: reqwest::Client,
}

impl ApiClient {
    async fn get_user(&self, id: u32) -> Result<User, reqwest::Error> {
        let user = self.client
            .get(&format!("https://api.example.com/users/{}", id))
            .send()
            .await?
            .json::<User>()
            .await?;
        
        Ok(user)
    }
}
```

***

## Traits - Rust's Interfaces

Traits are Rust's way of defining shared behavior, similar to interfaces in C# but more powerful.

### C# Interface Comparison
```csharp
// C# interface definition
public interface IAnimal
{
    string Name { get; }
    void MakeSound();
    
    // Default implementation (C# 8+)
    string Describe()
    {
        return $"{Name} makes a sound";
    }
}

// C# interface implementation
public class Dog : IAnimal
{
    public string Name { get; }
    
    public Dog(string name)
    {
        Name = name;
    }
    
    public void MakeSound()
    {
        Console.WriteLine("Woof!");
    }
    
    // Can override default implementation
    public string Describe()
    {
        return $"{Name} is a loyal dog";
    }
}

// Generic constraints
public void ProcessAnimal<T>(T animal) where T : IAnimal
{
    animal.MakeSound();
    Console.WriteLine(animal.Describe());
}
```

### Rust Trait Definition and Implementation
```rust
// Trait definition
trait Animal {
    fn name(&self) -> &str;
    fn make_sound(&self);
    
    // Default implementation
    fn describe(&self) -> String {
        format!("{} makes a sound", self.name())
    }
    
    // Default implementation using other trait methods
    fn introduce(&self) {
        println!("Hi, I'm {}", self.name());
        self.make_sound();
    }
}

// Struct definition
#[derive(Debug)]
struct Dog {
    name: String,
    breed: String,
}

impl Dog {
    fn new(name: String, breed: String) -> Dog {
        Dog { name, breed }
    }
}

// Trait implementation
impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn make_sound(&self) {
        println!("Woof!");
    }
    
    // Override default implementation
    fn describe(&self) -> String {
        format!("{} is a loyal {} dog", self.name, self.breed)
    }
}

// Another implementation
#[derive(Debug)]
struct Cat {
    name: String,
    indoor: bool,
}

impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn make_sound(&self) {
        println!("Meow!");
    }
    
    // Use default describe() implementation
}

// Generic function with trait bounds
fn process_animal<T: Animal>(animal: &T) {
    animal.make_sound();
    println!("{}", animal.describe());
    animal.introduce();
}

// Multiple trait bounds
fn process_animal_debug<T: Animal + std::fmt::Debug>(animal: &T) {
    println!("Debug: {:?}", animal);
    process_animal(animal);
}

fn main() {
    let dog = Dog::new("Buddy".to_string(), "Golden Retriever".to_string());
    let cat = Cat { name: "Whiskers".to_string(), indoor: true };
    
    process_animal(&dog);
    process_animal(&cat);
    
    process_animal_debug(&dog);
}
```

### Trait Objects and Dynamic Dispatch
```csharp
// C# dynamic polymorphism
public void ProcessAnimals(List<IAnimal> animals)
{
    foreach (var animal in animals)
    {
        animal.MakeSound(); // Dynamic dispatch
        Console.WriteLine(animal.Describe());
    }
}

// Usage
var animals = new List<IAnimal>
{
    new Dog("Buddy"),
    new Cat("Whiskers"),
    new Dog("Rex")
};

ProcessAnimals(animals);
```

```rust
// Rust trait objects for dynamic dispatch
fn process_animals(animals: &[Box<dyn Animal>]) {
    for animal in animals {
        animal.make_sound(); // Dynamic dispatch
        println!("{}", animal.describe());
    }
}

// Alternative: using references
fn process_animal_refs(animals: &[&dyn Animal]) {
    for animal in animals {
        animal.make_sound();
        println!("{}", animal.describe());
    }
}

fn main() {
    // Using Box<dyn Trait>
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog::new("Buddy".to_string(), "Golden Retriever".to_string())),
        Box::new(Cat { name: "Whiskers".to_string(), indoor: true }),
        Box::new(Dog::new("Rex".to_string(), "German Shepherd".to_string())),
    ];
    
    process_animals(&animals);
    
    // Using references
    let dog = Dog::new("Buddy".to_string(), "Golden Retriever".to_string());
    let cat = Cat { name: "Whiskers".to_string(), indoor: true };
    
    let animal_refs: Vec<&dyn Animal> = vec![&dog, &cat];
    process_animal_refs(&animal_refs);
}
```

### Derived Traits
```rust
// Automatically derive common traits
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Person {
    name: String,
    age: u32,
}

// What this generates (simplified):
impl std::fmt::Debug for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Person")
            .field("name", &self.name)
            .field("age", &self.age)
            .finish()
    }
}

impl Clone for Person {
    fn clone(&self) -> Self {
        Person {
            name: self.name.clone(),
            age: self.age,
        }
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.age == other.age
    }
}

// Usage
fn main() {
    let person1 = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    
    let person2 = person1.clone(); // Clone trait
    
    println!("{:?}", person1); // Debug trait
    println!("Equal: {}", person1 == person2); // PartialEq trait
}
```

### Common Standard Library Traits
```rust
use std::collections::HashMap;

// Display trait for user-friendly output
impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (age {})", self.name, self.age)
    }
}

// From trait for conversions
impl From<(String, u32)> for Person {
    fn from((name, age): (String, u32)) -> Self {
        Person { name, age }
    }
}

// Into trait is automatically implemented when From is implemented
fn create_person() {
    let person: Person = ("Alice".to_string(), 30).into();
    println!("{}", person);
}

// Iterator trait implementation
struct PersonIterator {
    people: Vec<Person>,
    index: usize,
}

impl Iterator for PersonIterator {
    type Item = Person;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.people.len() {
            let person = self.people[self.index].clone();
            self.index += 1;
            Some(person)
        } else {
            None
        }
    }
}

impl Person {
    fn iterator(people: Vec<Person>) -> PersonIterator {
        PersonIterator { people, index: 0 }
    }
}

fn main() {
    let people = vec![
        Person::from(("Alice".to_string(), 30)),
        Person::from(("Bob".to_string(), 25)),
        Person::from(("Charlie".to_string(), 35)),
    ];
    
    // Use our custom iterator
    for person in Person::iterator(people.clone()) {
        println!("{}", person); // Uses Display trait
    }
}
```

***

## Error Handling Deep Dive

### C# Exception Model
```csharp
public class FileProcessor
{
    public string ProcessFile(string path)
    {
        try
        {
            var content = File.ReadAllText(path);
            
            if (string.IsNullOrEmpty(content))
                throw new InvalidOperationException("File is empty");
            
            return content.ToUpper();
        }
        catch (FileNotFoundException)
        {
            throw new ApplicationException($"File not found: {path}");
        }
        catch (UnauthorizedAccessException)
        {
            throw new ApplicationException($"Access denied: {path}");
        }
        catch (Exception ex)
        {
            throw new ApplicationException($"Unexpected error: {ex.Message}");
        }
    }
    
    public async Task<List<string>> ProcessMultipleFiles(List<string> paths)
    {
        var results = new List<string>();
        
        foreach (var path in paths)
        {
            try
            {
                var result = ProcessFile(path);
                results.Add(result);
            }
            catch (Exception ex)
            {
                // Log error but continue with other files
                Console.WriteLine($"Error processing {path}: {ex.Message}");
            }
        }
        
        return results;
    }
}
```

### Rust Result-Based Error Handling
```rust
use std::fs;
use std::io;

#[derive(Debug)]
enum ProcessingError {
    FileNotFound(String),
    AccessDenied(String),
    EmptyFile(String),
    IoError(io::Error),
}

impl std::fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessingError::FileNotFound(path) => write!(f, "File not found: {}", path),
            ProcessingError::AccessDenied(path) => write!(f, "Access denied: {}", path),
            ProcessingError::EmptyFile(path) => write!(f, "File is empty: {}", path),
            ProcessingError::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl std::error::Error for ProcessingError {}

impl From<io::Error> for ProcessingError {
    fn from(error: io::Error) -> Self {
        ProcessingError::IoError(error)
    }
}

struct FileProcessor;

impl FileProcessor {
    fn process_file(path: &str) -> Result<String, ProcessingError> {
        // Use ? operator for early returns
        let content = fs::read_to_string(path)
            .map_err(|err| match err.kind() {
                io::ErrorKind::NotFound => ProcessingError::FileNotFound(path.to_string()),
                io::ErrorKind::PermissionDenied => ProcessingError::AccessDenied(path.to_string()),
                _ => ProcessingError::IoError(err),
            })?;
        
        if content.is_empty() {
            return Err(ProcessingError::EmptyFile(path.to_string()));
        }
        
        Ok(content.to_uppercase())
    }
    
    fn process_multiple_files(paths: &[&str]) -> Vec<Result<String, ProcessingError>> {
        paths.iter()
            .map(|&path| Self::process_file(path))
            .collect()
    }
    
    // Alternative: collect only successful results
    fn process_multiple_files_successful(paths: &[&str]) -> (Vec<String>, Vec<ProcessingError>) {
        let results: Vec<_> = Self::process_multiple_files(paths);
        
        let mut successes = Vec::new();
        let mut errors = Vec::new();
        
        for result in results {
            match result {
                Ok(content) => successes.push(content),
                Err(error) => {
                    eprintln!("Error: {}", error);
                    errors.push(error);
                }
            }
        }
        
        (successes, errors)
    }
}

fn main() {
    let paths = vec!["file1.txt", "file2.txt", "nonexistent.txt"];
    
    // Process individual file
    match FileProcessor::process_file("example.txt") {
        Ok(content) => println!("Content: {}", content),
        Err(error) => eprintln!("Error: {}", error),
    }
    
    // Process multiple files - keep all results
    let results = FileProcessor::process_multiple_files(&paths);
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(content) => println!("File {}: Success", i),
            Err(error) => println!("File {}: Error - {}", i, error),
        }
    }
    
    // Process multiple files - separate successes and errors
    let (successes, errors) = FileProcessor::process_multiple_files_successful(&paths);
    println!("Processed {} files successfully, {} errors", successes.len(), errors.len());
}
```

***

## Practical Migration Examples

Let's look at some real-world scenarios showing how common C# patterns translate to Rust.

### Configuration Management
```csharp
// C# configuration class
public class AppConfig
{
    public string DatabaseUrl { get; set; } = "localhost";
    public int Port { get; set; } = 5432;
    public List<string> AllowedHosts { get; set; } = new();
    public Dictionary<string, string> FeatureFlags { get; set; } = new();
    
    public static AppConfig LoadFromFile(string path)
    {
        try
        {
            var json = File.ReadAllText(path);
            return JsonSerializer.Deserialize<AppConfig>(json) ?? new AppConfig();
        }
        catch (Exception ex)
        {
            Console.WriteLine($"Failed to load config: {ex.Message}");
            return new AppConfig(); // Fall back to defaults
        }
    }
    
    public void Validate()
    {
        if (string.IsNullOrEmpty(DatabaseUrl))
            throw new InvalidOperationException("DatabaseUrl is required");
        
        if (Port <= 0 || Port > 65535)
            throw new InvalidOperationException("Port must be between 1 and 65535");
    }
}
```

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub port: u16,
    pub allowed_hosts: Vec<String>,
    pub feature_flags: HashMap<String, String>,
}

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound(String),
    ParseError(String),
    ValidationError(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::FileNotFound(path) => write!(f, "Config file not found: {}", path),
            ConfigError::ParseError(msg) => write!(f, "Failed to parse config: {}", msg),
            ConfigError::ValidationError(msg) => write!(f, "Invalid config: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            database_url: "localhost".to_string(),
            port: 5432,
            allowed_hosts: Vec::new(),
            feature_flags: HashMap::new(),
        }
    }
}

impl AppConfig {
    pub fn load_from_file(path: &str) -> Result<AppConfig, ConfigError> {
        let contents = fs::read_to_string(path)
            .map_err(|_| ConfigError::FileNotFound(path.to_string()))?;
        
        let config: AppConfig = serde_json::from_str(&contents)
            .map_err(|e| ConfigError::ParseError(e.to_string()))?;
        
        config.validate()?;
        Ok(config)
    }
    
    pub fn load_or_default(path: &str) -> AppConfig {
        Self::load_from_file(path)
            .unwrap_or_else(|error| {
                eprintln!("Failed to load config: {}", error);
                AppConfig::default()
            })
    }
    
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.database_url.is_empty() {
            return Err(ConfigError::ValidationError("DatabaseUrl is required".to_string()));
        }
        
        if self.port == 0 {
            return Err(ConfigError::ValidationError("Port must be greater than 0".to_string()));
        }
        
        Ok(())
    }
    
    pub fn get_feature_flag(&self, key: &str) -> Option<&String> {
        self.feature_flags.get(key)
    }
    
    pub fn is_feature_enabled(&self, key: &str) -> bool {
        self.get_feature_flag(key)
            .map(|value| value.to_lowercase() == "true")
            .unwrap_or(false)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Try to load config, fall back to defaults
    let config = AppConfig::load_or_default("config.json");
    println!("Config: {:?}", config);
    
    // Check feature flags
    if config.is_feature_enabled("debug_mode") {
        println!("Debug mode is enabled");
    }
    
    Ok(())
}
```

### Data Processing Pipeline
```csharp
// C# data processing
public class DataProcessor
{
    public async Task<List<ProcessedData>> ProcessAsync(List<RawData> data)
    {
        var results = new List<ProcessedData>();
        
        foreach (var item in data)
        {
            try
            {
                if (IsValid(item))
                {
                    var processed = await TransformAsync(item);
                    results.Add(processed);
                }
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Error processing item {item.Id}: {ex.Message}");
            }
        }
        
        return results;
    }
    
    private bool IsValid(RawData data)
    {
        return !string.IsNullOrEmpty(data.Value) && data.Timestamp > DateTime.MinValue;
    }
    
    private async Task<ProcessedData> TransformAsync(RawData data)
    {
        // Simulate async processing
        await Task.Delay(10);
        
        return new ProcessedData
        {
            Id = data.Id,
            ProcessedValue = data.Value.ToUpper(),
            ProcessedAt = DateTime.UtcNow
        };
    }
}

public class RawData
{
    public int Id { get; set; }
    public string Value { get; set; } = "";
    public DateTime Timestamp { get; set; }
}

public class ProcessedData
{
    public int Id { get; set; }
    public string ProcessedValue { get; set; } = "";
    public DateTime ProcessedAt { get; set; }
}
```

```rust
use std::time::{SystemTime, UNIX_EPOCH};
use tokio;

#[derive(Debug, Clone)]
pub struct RawData {
    pub id: u32,
    pub value: String,
    pub timestamp: u64,
}

#[derive(Debug)]
pub struct ProcessedData {
    pub id: u32,
    pub processed_value: String,
    pub processed_at: u64,
}

#[derive(Debug)]
pub enum ProcessingError {
    InvalidData(String),
    TransformationFailed(String),
}

impl std::fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessingError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            ProcessingError::TransformationFailed(msg) => write!(f, "Transformation failed: {}", msg),
        }
    }
}

impl std::error::Error for ProcessingError {}

pub struct DataProcessor;

impl DataProcessor {
    pub async fn process(data: Vec<RawData>) -> Vec<Result<ProcessedData, ProcessingError>> {
        // Use futures for concurrent processing
        let futures = data.into_iter().map(|item| async move {
            Self::validate(&item)?;
            Self::transform(item).await
        });
        
        // Collect all futures
        futures::future::join_all(futures).await
    }
    
    pub async fn process_successful_only(data: Vec<RawData>) -> Vec<ProcessedData> {
        let results = Self::process(data).await;
        
        results.into_iter()
            .filter_map(|result| match result {
                Ok(processed) => Some(processed),
                Err(error) => {
                    eprintln!("Processing error: {}", error);
                    None
                }
            })
            .collect()
    }
    
    fn validate(data: &RawData) -> Result<(), ProcessingError> {
        if data.value.is_empty() {
            return Err(ProcessingError::InvalidData("Value cannot be empty".to_string()));
        }
        
        if data.timestamp == 0 {
            return Err(ProcessingError::InvalidData("Invalid timestamp".to_string()));
        }
        
        Ok(())
    }
    
    async fn transform(data: RawData) -> Result<ProcessedData, ProcessingError> {
        // Simulate async processing
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        
        let processed_value = data.value.to_uppercase();
        
        if processed_value.len() > 1000 {
            return Err(ProcessingError::TransformationFailed("Processed value too long".to_string()));
        }
        
        let processed_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Ok(ProcessedData {
            id: data.id,
            processed_value,
            processed_at,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_data = vec![
        RawData { id: 1, value: "hello".to_string(), timestamp: 1234567890 },
        RawData { id: 2, value: "world".to_string(), timestamp: 1234567891 },
        RawData { id: 3, value: "".to_string(), timestamp: 1234567892 }, // Invalid
    ];
    
    // Process and handle errors explicitly
    let results = DataProcessor::process(raw_data.clone()).await;
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(processed) => println!("Item {}: {:?}", i, processed),
            Err(error) => println!("Item {}: Error - {}", i, error),
        }
    }
    
    // Process and keep only successful results
    let successful = DataProcessor::process_successful_only(raw_data).await;
    println!("Successfully processed {} items", successful.len());
    
    Ok(())
}
```

### HTTP Client Example
```csharp
// C# HTTP client
public class ApiClient
{
    private readonly HttpClient _httpClient;
    
    public ApiClient(HttpClient httpClient)
    {
        _httpClient = httpClient;
    }
    
    public async Task<T?> GetAsync<T>(string endpoint) where T : class
    {
        try
        {
            var response = await _httpClient.GetAsync(endpoint);
            
            if (response.IsSuccessStatusCode)
            {
                var json = await response.Content.ReadAsStringAsync();
                return JsonSerializer.Deserialize<T>(json);
            }
            
            Console.WriteLine($"HTTP Error: {response.StatusCode}");
            return null;
        }
        catch (Exception ex)
        {
            Console.WriteLine($"Request failed: {ex.Message}");
            return null;
        }
    }
    
    public async Task<bool> PostAsync<T>(string endpoint, T data)
    {
        try
        {
            var json = JsonSerializer.Serialize(data);
            var content = new StringContent(json, Encoding.UTF8, "application/json");
            
            var response = await _httpClient.PostAsync(endpoint, content);
            return response.IsSuccessStatusCode;
        }
        catch (Exception ex)
        {
            Console.WriteLine($"POST failed: {ex.Message}");
            return false;
        }
    }
}
```

```rust
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ApiError {
    NetworkError(reqwest::Error),
    HttpError(u16, String),
    ParseError(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::NetworkError(err) => write!(f, "Network error: {}", err),
            ApiError::HttpError(code, msg) => write!(f, "HTTP {} error: {}", code, msg),
            ApiError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}

impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> Self {
        ApiError::NetworkError(error)
    }
}

pub struct ApiClient {
    client: reqwest::Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        ApiClient {
            client: reqwest::Client::new(),
            base_url,
        }
    }
    
    pub async fn get<T>(&self, endpoint: &str) -> Result<T, ApiError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = format!("{}/{}", self.base_url, endpoint);
        
        let response = self.client.get(&url).send().await?;
        
        if response.status().is_success() {
            let data = response.json::<T>().await
                .map_err(|e| ApiError::ParseError(e.to_string()))?;
            Ok(data)
        } else {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            Err(ApiError::HttpError(status, body))
        }
    }
    
    pub async fn post<T, R>(&self, endpoint: &str, data: &T) -> Result<R, ApiError>
    where
        T: Serialize,
        R: for<'de> Deserialize<'de>,
    {
        let url = format!("{}/{}", self.base_url, endpoint);
        
        let response = self.client
            .post(&url)
            .json(data)
            .send()
            .await?;
        
        if response.status().is_success() {
            let result = response.json::<R>().await
                .map_err(|e| ApiError::ParseError(e.to_string()))?;
            Ok(result)
        } else {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            Err(ApiError::HttpError(status, body))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Serialize, Debug)]
struct CreateUserRequest {
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ApiClient::new("https://jsonplaceholder.typicode.com".to_string());
    
    // GET request
    match client.get::<User>("users/1").await {
        Ok(user) => println!("User: {:?}", user),
        Err(error) => eprintln!("Failed to get user: {}", error),
    }
    
    // POST request
    let new_user = CreateUserRequest {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
    };
    
    match client.post::<CreateUserRequest, User>("users", &new_user).await {
        Ok(created_user) => println!("Created user: {:?}", created_user),
        Err(error) => eprintln!("Failed to create user: {}", error),
    }
    
    Ok(())
}
```

***

## Learning Path and Next Steps

### Immediate Next Steps (Week 1-2)
1. **Set up your environment**
   - Install Rust via [rustup.rs](https://rustup.rs/)
   - Configure VS Code with rust-analyzer extension
   - Create your first `cargo new hello_world` project

2. **Master the basics**
   - Practice ownership with simple exercises
   - Write functions with different parameter types (`&str`, `String`, `&mut`)
   - Implement basic structs and methods

3. **Error handling practice**
   - Convert C# try-catch code to Result-based patterns
   - Practice with `?` operator and `match` statements
   - Implement custom error types

### Intermediate Goals (Month 1-2)
1. **Collections and iterators**
   - Master `Vec<T>`, `HashMap<K,V>`, and `HashSet<T>`
   - Learn iterator methods: `map`, `filter`, `collect`, `fold`
   - Practice with `for` loops vs iterator chains

2. **Traits and generics**
   - Implement common traits: `Debug`, `Clone`, `PartialEq`
   - Write generic functions and structs
   - Understand trait bounds and where clauses

3. **Project structure**
   - Organize code into modules
   - Understand `pub` visibility
   - Work with external crates from crates.io

### Advanced Topics (Month 3+)
1. **Concurrency**
   - Learn about `Send` and `Sync` traits
   - Use `std::thread` for basic parallelism
   - Explore `tokio` for async programming

2. **Memory management**
   - Understand `Rc<T>` and `Arc<T>` for shared ownership
   - Learn when to use `Box<T>` for heap allocation
   - Master lifetimes for complex scenarios

3. **Real-world projects**
   - Build a CLI tool with `clap`
   - Create a web API with `axum` or `warp`
   - Write a library and publish to crates.io

### Recommended Learning Resources

#### Books
- **"The Rust Programming Language"** (free online) - The official book
- **"Rust by Example"** (free online) - Hands-on examples
- **"Programming Rust"** by Jim Blandy - Deep technical coverage

#### Online Resources
- [Rust Playground](https://play.rust-lang.org/) - Try code in browser
- [Rustlings](https://github.com/rust-lang/rustlings) - Interactive exercises
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Practical examples

#### Practice Projects
1. **Command-line calculator** - Practice with enums and pattern matching
2. **File organizer** - Work with filesystem and error handling
3. **JSON processor** - Learn serde and data transformation
4. **HTTP server** - Understand async programming and networking
5. **Database library** - Master traits, generics, and error handling

### Common Pitfalls for C# Developers

#### Ownership Confusion
```rust
// DON'T: Trying to use moved values
fn wrong_way() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // ERROR: s was moved
}

// DO: Use references or clone when needed
fn right_way() {
    let s = String::from("hello");
    borrows_string(&s);
    println!("{}", s); // OK: s is still owned here
}

fn takes_ownership(s: String) { /* s is moved here */ }
fn borrows_string(s: &str) { /* s is borrowed here */ }
```

#### Fighting the Borrow Checker
```rust
// DON'T: Multiple mutable references
fn wrong_borrowing() {
    let mut v = vec![1, 2, 3];
    let r1 = &mut v;
    // let r2 = &mut v; // ERROR: cannot borrow as mutable more than once
}

// DO: Limit scope of mutable borrows
fn right_borrowing() {
    let mut v = vec![1, 2, 3];
    {
        let r1 = &mut v;
        r1.push(4);
    } // r1 goes out of scope here
    
    let r2 = &mut v; // OK: no other mutable borrows exist
    r2.push(5);
}
```

#### Expecting Null Values
```rust
// DON'T: Expecting null-like behavior
fn no_null_in_rust() {
    // let s: String = null; // NO null in Rust!
}

// DO: Use Option<T> explicitly
fn use_option_instead() {
    let maybe_string: Option<String> = None;
    
    match maybe_string {
        Some(s) => println!("Got string: {}", s),
        None => println!("No string available"),
    }
}
```

### Final Tips

1. **Embrace the compiler** - Rust's compiler errors are helpful, not hostile
2. **Start small** - Begin with simple programs and gradually add complexity
3. **Read other people's code** - Study popular crates on GitHub
4. **Ask for help** - The Rust community is welcoming and helpful
5. **Practice regularly** - Rust's concepts become natural with practice

Remember: Rust has a learning curve, but it pays off with memory safety, performance, and fearless concurrency. The ownership system that seems restrictive at first becomes a powerful tool for writing correct, efficient programs.

---

**Congratulations!** You now have a solid foundation for transitioning from C# to Rust. Start with simple projects, be patient with the learning process, and gradually work your way up to more complex applications. The safety and performance benefits of Rust make the initial learning investment worthwhile.

For the next phase of your learning journey, consider diving deeper into the [Advanced Rust Training for C# Programmers](./RustTrainingForCSharp.md) guide, which covers more sophisticated patterns, performance optimization, and real-world application architecture.