## Installation and Setup

> **What you'll learn:** How to install Rust and set up your IDE, the Cargo build system vs MSBuild/NuGet,
> your first Rust program compared to C#, and how to read command-line input.
>
> **Difficulty:** 🟢 Beginner

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
```text
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

## Reading Input and CLI Arguments

Every C# developer knows `Console.ReadLine()`. Here's how to handle user input, environment variables, and command-line arguments in Rust.

### Console Input
```csharp
// C# — reading user input
Console.Write("Enter your name: ");
string? name = Console.ReadLine();  // Returns string? in .NET 6+
Console.WriteLine($"Hello, {name}!");

// Parsing input
Console.Write("Enter a number: ");
if (int.TryParse(Console.ReadLine(), out int number))
{
    Console.WriteLine($"You entered: {number}");
}
else
{
    Console.WriteLine("That's not a valid number.");
}
```

```rust
use std::io::{self, Write};

fn main() {
    // Reading a line of input
    print!("Enter your name: ");
    io::stdout().flush().unwrap(); // flush because print! doesn't auto-flush

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim(); // remove trailing newline
    println!("Hello, {name}!");

    // Parsing input
    print!("Enter a number: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    match input.trim().parse::<i32>() {
        Ok(number) => println!("You entered: {number}"),
        Err(_)     => println!("That's not a valid number."),
    }
}
```

### Command-Line Arguments
```csharp
// C# — reading CLI args
static void Main(string[] args)
{
    if (args.Length < 1)
    {
        Console.WriteLine("Usage: program <filename>");
        return;
    }
    string filename = args[0];
    Console.WriteLine($"Processing {filename}");
}
```

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    //  args[0] = program name (like C#'s Assembly name)
    //  args[1..] = actual arguments

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]); // eprintln! → stderr
        std::process::exit(1);
    }
    let filename = &args[1];
    println!("Processing {filename}");
}
```

### Environment Variables
```csharp
// C#
string dbUrl = Environment.GetEnvironmentVariable("DATABASE_URL") ?? "localhost";
```

```rust
use std::env;

let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "localhost".to_string());
// env::var returns Result<String, VarError> — no nulls!
```

### Production CLI Apps with `clap`

For anything beyond trivial argument parsing, use the **`clap`** crate — it's the Rust equivalent of `System.CommandLine` or libraries like `CommandLineParser`.

```toml
# Cargo.toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

```rust
use clap::Parser;

/// A simple file processor — this doc comment becomes the help text
#[derive(Parser, Debug)]
#[command(name = "processor", version, about)]
struct Args {
    /// Input file to process
    #[arg(short, long)]
    input: String,

    /// Output file (defaults to stdout)
    #[arg(short, long)]
    output: Option<String>,

    /// Enable verbose logging
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Number of worker threads
    #[arg(short = 'j', long, default_value_t = 4)]
    threads: usize,
}

fn main() {
    let args = Args::parse(); // auto-parses, validates, generates --help

    if args.verbose {
        println!("Input:   {}", args.input);
        println!("Output:  {:?}", args.output);
        println!("Threads: {}", args.threads);
    }

    // Use args.input, args.output, etc.
}
```

```bash
# Auto-generated help:
$ processor --help
A simple file processor

Usage: processor [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>      Input file to process
  -o, --output <OUTPUT>    Output file (defaults to stdout)
  -v, --verbose            Enable verbose logging
  -j, --threads <THREADS>  Number of worker threads [default: 4]
  -h, --help               Print help
  -V, --version            Print version
```

```csharp
// C# equivalent with System.CommandLine (more boilerplate):
var inputOption = new Option<string>("--input", "Input file") { IsRequired = true };
var verboseOption = new Option<bool>("--verbose", "Enable verbose logging");
var rootCommand = new RootCommand("A simple file processor");
rootCommand.AddOption(inputOption);
rootCommand.AddOption(verboseOption);
rootCommand.SetHandler((input, verbose) => { /* ... */ }, inputOption, verboseOption);
await rootCommand.InvokeAsync(args);
// clap's derive macro approach is more concise and type-safe
```

| C# | Rust | Notes |
|----|------|-------|
| `Console.ReadLine()` | `io::stdin().read_line(&mut buf)` | Must provide buffer, returns `Result` |
| `int.TryParse(s, out n)` | `s.parse::<i32>()` | Returns `Result<i32, ParseIntError>` |
| `args[0]` | `env::args().nth(1)` | Rust args[0] = program name |
| `Environment.GetEnvironmentVariable` | `env::var("KEY")` | Returns `Result`, not nullable |
| `System.CommandLine` | `clap` | Derive-based, auto-generates help |

***

