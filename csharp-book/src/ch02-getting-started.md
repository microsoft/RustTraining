<a id="installation-and-setup"></a>
## 설치와 환경 구성

> **이 절에서 배울 내용:** Rust 설치 방법과 IDE 설정, Cargo 빌드 시스템과 MSBuild/NuGet의 차이, C#과 비교한 첫 Rust 프로그램, 그리고 명령줄 입력을 읽는 방법을 배웁니다.
>
> **난이도:** 🟢 입문

<a id="installing-rust"></a>
### Rust 설치하기
```bash
# Rust 설치 (Windows, macOS, Linux에서 동작)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows에서는 https://rustup.rs/ 에서 직접 내려받아도 된다
```

<a id="rust-tools-vs-c-tools"></a>
### Rust 도구와 C# 도구 비교
| C# 도구 | Rust 대응 도구 | 용도 |
|---------|----------------|---------|
| `dotnet new` | `cargo new` | 새 프로젝트 생성 |
| `dotnet build` | `cargo build` | 프로젝트 컴파일 |
| `dotnet run` | `cargo run` | 프로젝트 실행 |
| `dotnet test` | `cargo test` | 테스트 실행 |
| NuGet | Crates.io | 패키지 저장소 |
| MSBuild | Cargo | 빌드 시스템 |
| Visual Studio | VS Code + rust-analyzer | IDE |

<a id="ide-setup"></a>
### IDE 설정
1. **VS Code** (입문자에게 추천)
   - `rust-analyzer` 확장 설치
   - 디버깅용 `CodeLLDB` 설치

2. **Visual Studio** (Windows)
   - Rust 지원 확장 설치

3. **JetBrains RustRover** (풀 IDE)
   - C#의 Rider와 비슷한 경험

***

<a id="your-first-rust-program"></a>
## 첫 Rust 프로그램

<a id="c-hello-world"></a>
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

<a id="rust-hello-world"></a>
### Rust의 Hello World
```rust
// main.rs
fn main() {
    println!("Hello, World!");
}
```

<a id="key-differences-for-c-developers"></a>
### C# 개발자가 먼저 알아둘 핵심 차이
1. **클래스가 꼭 필요하지 않다** - 함수는 최상위 수준에 바로 둘 수 있습니다.
2. **네임스페이스가 없다** - 대신 모듈 시스템을 사용합니다.
3. **`println!`은 매크로다** - `!`에 주목하세요.
4. **`println!` 뒤 세미콜론에 주의** - 표현식과 문장의 차이와 연결됩니다.
5. **명시적 반환 타입이 없다** - `main`은 `()`(unit type)을 반환합니다.

<a id="creating-your-first-project"></a>
### 첫 프로젝트 만들기
```bash
# 새 프로젝트 생성 (C#의 'dotnet new console'에 해당)
cargo new hello_rust
cd hello_rust

# 생성되는 프로젝트 구조:
# hello_rust/
# ├── Cargo.toml      (.csproj 파일과 비슷)
# └── src/
#     └── main.rs     (Program.cs와 비슷)

# 프로젝트 실행 (C#의 'dotnet run'에 해당)
cargo run
```

***

<a id="cargo-vs-nugetmsbuild"></a>
## Cargo vs NuGet/MSBuild

<a id="project-configuration"></a>
### 프로젝트 설정 파일

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
serde_json = "1.0"    # Newtonsoft.Json에 해당
log = "0.4"           # Serilog에 해당
```

<a id="common-cargo-commands"></a>
### 자주 쓰는 Cargo 명령
```bash
# 새 프로젝트 생성
cargo new my_project
cargo new my_project --lib  # 라이브러리 프로젝트 생성

# 빌드와 실행
cargo build          # 'dotnet build'와 비슷
cargo run            # 'dotnet run'과 비슷
cargo test           # 'dotnet test'와 비슷

# 패키지 관리
cargo add serde      # 의존성 추가 ('dotnet add package'와 비슷)
cargo update         # 의존성 업데이트

# 릴리스 빌드
cargo build --release  # 최적화 빌드
cargo run --release    # 최적화 버전 실행

# 문서 생성
cargo doc --open     # 문서를 생성하고 열기
```

<a id="workspace-vs-solution"></a>
### Workspace와 Solution

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

<a id="reading-input-and-cli-arguments"></a>
## 입력 읽기와 CLI 인수

모든 C# 개발자는 `Console.ReadLine()`을 알고 있습니다. 이제 Rust에서 사용자 입력, 환경 변수, 명령줄 인수를 다루는 방법을 살펴봅시다.

<a id="console-input"></a>
### 콘솔 입력
```csharp
// C# - 사용자 입력 읽기
Console.Write("Enter your name: ");
string name = Console.ReadLine();
Console.WriteLine($"Hello, {name}!");

// 입력 파싱
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
    // 한 줄 입력 읽기
    print!("Enter your name: ");
    io::stdout().flush().unwrap(); // print!는 자동 flush되지 않으므로 직접 호출

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim(); // 마지막 개행 제거
    println!("Hello, {name}!");

    // 입력 파싱
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

<a id="command-line-arguments"></a>
### 명령줄 인수
```csharp
// C# - CLI 인수 읽기
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
    // args[0] = 프로그램 이름 (C#의 Assembly 이름과 비슷)
    // args[1..] = 실제 인수들

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]); // eprintln! -> stderr
        std::process::exit(1);
    }
    let filename = &args[1];
    println!("Processing {filename}");
}
```

<a id="environment-variables"></a>
### 환경 변수
```csharp
// C#
string dbUrl = Environment.GetEnvironmentVariable("DATABASE_URL") ?? "localhost";
```

```rust
use std::env;

let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "localhost".to_string());
// env::var는 Result<String, VarError>를 반환한다 - null이 아니다!
```

<a id="production-cli-apps-with-clap"></a>
### `clap`으로 만드는 실전 CLI 앱

단순한 수준을 넘어서는 인수 파싱이 필요하다면 **`clap`** 크레이트를 쓰세요. Rust에서 `System.CommandLine`이나 `CommandLineParser`와 비슷한 역할을 합니다.

```toml
# Cargo.toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

```rust
use clap::Parser;

/// 간단한 파일 처리기 - 이 doc comment가 help 텍스트가 된다
#[derive(Parser, Debug)]
#[command(name = "processor", version, about)]
struct Args {
    /// 처리할 입력 파일
    #[arg(short, long)]
    input: String,

    /// 출력 파일 (기본값은 stdout)
    #[arg(short, long)]
    output: Option<String>,

    /// 상세 로그 활성화
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// 워커 스레드 수
    #[arg(short = 'j', long, default_value_t = 4)]
    threads: usize,
}

fn main() {
    let args = Args::parse(); // 자동 파싱, 검증, --help 생성

    if args.verbose {
        println!("Input:   {}", args.input);
        println!("Output:  {:?}", args.output);
        println!("Threads: {}", args.threads);
    }

    // args.input, args.output 등을 사용
}
```

```bash
# 자동 생성되는 help 예시:
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
// C#에서의 System.CommandLine 대응 예시 (보일러플레이트가 더 많다):
var inputOption = new Option<string>("--input", "Input file") { IsRequired = true };
var verboseOption = new Option<bool>("--verbose", "Enable verbose logging");
var rootCommand = new RootCommand("A simple file processor");
rootCommand.AddOption(inputOption);
rootCommand.AddOption(verboseOption);
rootCommand.SetHandler((input, verbose) => { /* ... */ }, inputOption, verboseOption);
await rootCommand.InvokeAsync(args);
// clap의 derive macro 접근은 더 간결하고 타입 안전하다
```

| C# | Rust | 비고 |
|----|------|-------|
| `Console.ReadLine()` | `io::stdin().read_line(&mut buf)` | 버퍼를 직접 넘겨야 하며, `Result`를 반환 |
| `int.TryParse(s, out n)` | `s.parse::<i32>()` | `Result<i32, ParseIntError>`를 반환 |
| `args[0]` | `env::args().nth(1)` | Rust의 `args[0]`는 프로그램 이름 |
| `Environment.GetEnvironmentVariable` | `env::var("KEY")` | nullable이 아니라 `Result`를 반환 |
| `System.CommandLine` | `clap` | derive 기반이며 help를 자동 생성 |

***