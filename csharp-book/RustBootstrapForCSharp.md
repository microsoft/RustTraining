# C# 개발자를 위한 Rust 부트스트랩

C# 경험이 있는 개발자를 위한 Rust의 체계적인 입문서입니다. 검증된 교육 순서를 따라 개념을 단계적으로 쌓아, Rust가 *어떻게* 동작하는지뿐 아니라 *왜* 그렇게 설계되었는지까지 이해할 수 있도록 돕습니다.

## 과정 개요
- **Rust가 필요한 이유** — C# 개발자에게 Rust가 왜 중요한지
- **시작하기** — 설치, 툴링, 첫 프로그램
- **기본 구성 요소** — 타입, 변수, 제어 흐름
- **자료구조** — 배열, 튜플, 구조체
- **패턴 매칭과 enum** — Rust의 필수 개념
- **모듈과 크레이트** — 코드 구성과 의존성(.NET 어셈블리와 대비)
- **트레잇과 제네릭** — 고급 타입 시스템
- **에러 처리** — Rust의 안전성 접근
- **메모리 관리** — 소유권, 대여, 라이프타임
- **실무 마이그레이션** — 실제 사례

## 목차

### 1. 소개와 동기
- [빠른 참고: Rust와 C#](#quick-reference-rust-vs-c)
- [C# 개발자에게 Rust가 필요한 이유](#the-case-for-rust-for-c-developers)
- [Rust가 해결하는 C#의 대표적인 문제점](#common-c-pain-points-that-rust-addresses)
- [언제 C#보다 Rust를 선택해야 하는가](#when-to-choose-rust-over-c)

### 2. 시작하기
- [설치와 환경 구성](#installation-and-setup)
- [첫 Rust 프로그램](#your-first-rust-program)
- [Cargo와 NuGet·MSBuild](#cargo-vs-nugetmsbuild)
- [C# 개발자를 위한 IDE 설정](#ide-setup-for-c-developers)

### 3. 기본 타입과 변수
- [내장 타입 비교](#built-in-types-comparison)
- [변수와 가변성](#variables-and-mutability)
- [문자열 타입: String과 &str](#string-types-string-vs-str)
- [주석과 문서화](#comments-and-documentation)

### 4. 제어 흐름
- [조건문](#conditional-statements)
- [반복문과 순회](#loops-and-iteration)
- [표현식 블록](#expression-blocks)
- [함수와 메서드](#functions-vs-methods)

### 5. 자료구조
- [배열과 슬라이스](#arrays-and-slices)
- [튜플](#tuples)
- [구조체와 클래스](#structs-vs-classes)
- [참조와 대여 기초](#references-and-borrowing-basics)

### 6. 패턴 매칭과 열거형
- [열거형과 C# enum](#enums-vs-c-enums)
- [`match` 표현식](#match-expressions)
- [null 안전을 위한 `Option<T>`](#optiont-for-null-safety)
- [에러 처리를 위한 `Result<T, E>`](#resultt-e-for-error-handling)

### 7. 모듈과 크레이트
- [Rust 모듈과 C# 네임스페이스](#rust-modules-vs-c-namespaces)
- [크레이트와 .NET 어셈블리](#crates-vs-net-assemblies)
- [패키지 관리: Cargo와 NuGet](#package-management-cargo-vs-nuget)
- [가시성과 접근 제어](#visibility-and-access-control)

### 8. 트레잇과 제네릭
- [트레잇과 인터페이스](#traits-vs-interfaces)
- [제네릭 타입과 함수](#generic-types-and-functions)
- [트레잇 경계와 제약](#trait-bounds-and-constraints)
- [자주 쓰는 표준 라이브러리 트레잇](#common-standard-library-traits)

### 9. 컬렉션과 에러 처리
- [`Vec<T>`와 `List<T>` 비교](#vect-vs-listt)
- [`HashMap`과 `Dictionary` 비교](#hashmap-vs-dictionary)
- [이터레이터 패턴](#iterator-patterns)
- [에러 처리 종합](#comprehensive-error-handling)

### 10. 메모리 관리
- [소유권 이해하기](#understanding-ownership)
- [이동 시맨틱과 참조 시맨틱](#move-semantics-vs-reference-semantics)
- [대여와 라이프타임](#borrowing-and-lifetimes)
- [스마트 포인터](#smart-pointers)

### 11. 실무 마이그레이션 예시
- [설정 관리](#configuration-management)
- [데이터 처리 파이프라인](#data-processing-pipelines)
- [HTTP 클라이언트와 API](#http-clients-and-apis)
- [파일 I/O와 직렬화](#file-io-and-serialization)

### 12. 다음 단계와 모범 사례
- [Rust와 C#의 테스트](#testing-in-rust-vs-c)
- [C# 개발자가 흔히 하는 실수](#common-pitfalls-for-c-developers)
- [학습 경로와 자료](#learning-path-and-resources)
- [고급 주제로 나아가기](#moving-to-advanced-topics)

***

<a id="quick-reference-rust-vs-c"></a>
## 빠른 참고: Rust와 C#

| **개념** | **C#** | **Rust** | **핵심 차이** |
|-------------|--------|----------|-------------------|
| 메모리 관리 | 가비지 컬렉터 | 소유권 시스템 | 제로 코스트, 결정적 정리 |
| null 참조 | 어디서나 `null` | `Option<T>` | 컴파일 타임 null 안전 |
| 에러 처리 | 예외 | `Result<T, E>` | 명시적, 숨은 제어 흐름 없음 |
| 가변성 | 기본이 가변 | 기본이 불변 | 가변은 선택 |
| 타입 시스템 | 참조/값 타입 | 소유권 타입 | 이동 시맨틱, 대여 |
| 어셈블리 | GAC, 앱 도메인 | 크레이트 | 정적 링크, 런타임 없음 |
| 네임스페이스 | `using System.IO` | `use std::fs` | 모듈 시스템 |
| 인터페이스 | `interface IFoo` | `trait Foo` | 기본 구현 가능 |
| 제네릭 | `List<T>` where T : class | `Vec<T>` where T: Clone | 제로 코스트 추상화 |
| 스레딩 | lock, async/await | 소유권 + Send/Sync | 데이터 경쟁 방지 |
| 성능 | JIT 컴파일 | AOT 컴파일 | 예측 가능, GC 멈춤 없음 |

***

<a id="the-case-for-rust-for-c-developers"></a>
## C# 개발자에게 Rust가 필요한 이유

### 런타임 부담 없는 성능
```csharp
// C# — 생산성은 높지만 런타임 오버헤드가 있음
public class DataProcessor
{
    private List<int> data = new List<int>();
    
    public void ProcessLargeDataset()
    {
        // 할당이 GC를 유발함
        for (int i = 0; i < 10_000_000; i++)
        {
            data.Add(i * 2); // GC 부담
        }
        // 처리 중 예측 불가능한 GC 멈춤
    }
}
// 실행 시간: 가변 (GC 때문에 50–200ms)
// 메모리: ~80MB (GC 오버헤드 포함)
// 예측 가능성: 낮음 (GC 멈춤)
```

```rust
// Rust — 같은 표현력, 런타임 오버헤드 없음
struct DataProcessor {
    data: Vec<i32>,
}

impl DataProcessor {
    fn process_large_dataset(&mut self) {
        // 제로 코스트 추상화
        for i in 0..10_000_000 {
            self.data.push(i * 2); // GC 부담 없음
        }
        // 결정적인 성능
    }
}
// 실행 시간: 일정 (~30ms)
// 메모리: ~40MB (정확한 할당량)
// 예측 가능성: 높음 (GC 없음)
```

### 런타임 검사 없이 메모리 안전
```csharp
// C# — 런타임 안전성과 오버헤드
public class UnsafeOperations
{
    public string ProcessArray(int[] array)
    {
        // 런타임 범위 검사
        if (array.Length > 0)
        {
            return array[0].ToString(); // NullReferenceException 가능
        }
        return null; // null 전파
    }
    
    public void ProcessConcurrently()
    {
        var list = new List<int>();
        
        // 데이터 경쟁 가능, 신중한 락 필요
        Parallel.For(0, 1000, i =>
        {
            lock (list) // 런타임 오버헤드
            {
                list.Add(i);
            }
        });
    }
}
```

```rust
// Rust — 컴파일 타임 안전성, 런타임 비용 없음
struct SafeOperations;

impl SafeOperations {
    // 컴파일 타임 null 안전, 런타임 검사 없음
    fn process_array(array: &[i32]) -> Option<String> {
        array.first().map(|x| x.to_string())
        // null 참조 불가
        // 안전하다고 증명되면 범위 검사는 최적화로 제거 가능
    }
    
    fn process_concurrently() {
        use std::sync::Mutex;
        use std::thread;
        
        let data = Mutex::new(Vec::new());
        
        // 데이터 경쟁은 컴파일 타임에 방지
        let handles: Vec<_> = (0..1000).map(|i| {
            let data = &data;
            thread::spawn(move || {
                data.lock().unwrap().push(i);
                // 단일 스레드일 때는 락 오버헤드 없음
            })
        }).collect();
        
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
```

***

<a id="common-c-pain-points-that-rust-addresses"></a>
## Rust가 해결하는 C#의 대표적인 문제점

### 1. 십억 달러 실수: null 참조
```csharp
// C# — NullReferenceException은 런타임 폭탄
public class UserService
{
    public string GetUserDisplayName(User user)
    {
        // 아무 곳에서나 NullReferenceException 가능
        return user.Profile.DisplayName.ToUpper();
        //     ^^^^^ ^^^^^^^ ^^^^^^^^^^^ ^^^^^^^
        //     런타임에 null일 수 있음
    }
    
    // nullable 참조 타입(C# 8+)을 써도
    public string GetDisplayName(User? user)
    {
        return user?.Profile?.DisplayName?.ToUpper() ?? "Unknown";
        // 여전히 런타임에 null일 수 있음
    }
}
```

```rust
// Rust — 컴파일 타임에 null 안전 보장
struct UserService;

impl UserService {
    fn get_user_display_name(user: &User) -> Option<String> {
        user.profile.as_ref()?
            .display_name.as_ref()
            .map(|name| name.to_uppercase())
        // 컴파일러가 None 분기 처리를 강제
        // null 포인터 예외는 구조상 불가능
    }
    
    fn get_display_name_safe(user: Option<&User>) -> String {
        user.and_then(|u| u.profile.as_ref())
            .and_then(|p| p.display_name.as_ref())
            .map(|name| name.to_uppercase())
            .unwrap_or_else(|| "Unknown".to_string())
        // 명시적 처리, 예상 밖 동작 없음
    }
}
```

### 2. 숨은 예외와 제어 흐름
```csharp
// C# — 예외는 어디서든 던질 수 있음
public async Task<UserData> GetUserDataAsync(int userId)
{
    // 각 호출이 서로 다른 예외를 던질 수 있음
    var user = await userRepository.GetAsync(userId);        // SqlException
    var permissions = await permissionService.GetAsync(user); // HttpRequestException  
    var preferences = await preferenceService.GetAsync(user); // TimeoutException
    
    return new UserData(user, permissions, preferences);
    // 호출자는 어떤 예외를 기대해야 할지 알 수 없음
}
```

```rust
// Rust — 모든 에러가 함수 시그니처에 명시됨
#[derive(Debug)]
enum UserDataError {
    DatabaseError(String),
    NetworkError(String),
    Timeout,
    UserNotFound(i32),
}

async fn get_user_data(user_id: i32) -> Result<UserData, UserDataError> {
    // 모든 에러가 명시되고 처리됨
    let user = user_repository.get(user_id).await
        .map_err(UserDataError::DatabaseError)?;
    
    let permissions = permission_service.get(&user).await
        .map_err(UserDataError::NetworkError)?;
    
    let preferences = preference_service.get(&user).await
        .map_err(|_| UserDataError::Timeout)?;
    
    Ok(UserData::new(user, permissions, preferences))
    // 호출자는 가능한 에러를 정확히 알 수 있음
}
```

### 3. GC 때문에 예측하기 어려운 성능
```csharp
// C# — GC는 언제든 멈출 수 있음
public class HighFrequencyTrader
{
    private List<Trade> trades = new List<Trade>();
    
    public void ProcessMarketData(MarketTick tick)
    {
        // 할당이 최악의 순간에 GC를 유발할 수 있음
        var analysis = new MarketAnalysis(tick);
        trades.Add(new Trade(analysis.Signal, tick.Price));
        
        // 중요한 시장 순간에 여기서 GC가 멈출 수 있음
        // 멈춤 시간: 힙 크기에 따라 1–100ms
    }
}
```

```rust
// Rust — 예측 가능하고 결정적인 성능
struct HighFrequencyTrader {
    trades: Vec<Trade>,
}

impl HighFrequencyTrader {
    fn process_market_data(&mut self, tick: MarketTick) {
        // 할당 없음, 예측 가능한 성능
        let analysis = MarketAnalysis::from(tick);
        self.trades.push(Trade::new(analysis.signal(), tick.price));
        
        // GC 멈춤 없음, 일관된 서브 마이크로초 지연
        // 타입 시스템이 성능을 보장
    }
}
```

***

<a id="when-to-choose-rust-over-c"></a>
## 언제 C#보다 Rust를 선택해야 하는가

### ✅ Rust를 선택할 때:
- **성능이 결정적**: 실시간 시스템, 고빈도 트레이딩, 게임 엔진
- **메모리 사용이 중요**: 임베디드, 클라우드 비용, 모바일
- **예측 가능성이 필요**: 의료 기기, 자동차, 금융 시스템
- **보안이 최우선**: 암호, 네트워크 보안, 시스템 수준 코드
- **장기 실행 서비스**: GC 멈춤이 문제일 때
- **자원이 제한된 환경**: IoT, 엣지 컴퓨팅
- **시스템 프로그래밍**: CLI 도구, 데이터베이스, 웹 서버, OS

### ✅ C#에 머무를 때:
- **빠른 애플리케이션 개발**: 업무용 앱, CRUD
- **대규모 기존 코드베이스**: 이전 비용이 너무 클 때
- **팀 역량**: Rust 학습 곡선이 이득을 못 이길 때
- **엔터프라이즈 연동**: .NET Framework/Windows 의존이 클 때
- **GUI 애플리케이션**: WPF, WinUI, Blazor 생태계
- **출시 속도**: 개발 속도가 성능보다 우선일 때

### 🔄 둘 다 고려 (하이브리드):
- **성능이 중요한 부분은 Rust**: C#에서 P/Invoke로 호출
- **비즈니스 로직은 C#**: 익숙하고 생산적인 개발
- **점진적 이전**: 새 서비스부터 Rust로 시작

***

## 실제 사례: 기업이 Rust를 선택하는 이유

### Dropbox: 스토리지 인프라
- **이전 (Python)**: CPU 사용량·메모리 오버헤드 높음
- **이후 (Rust)**: 성능 10배, 메모리 50% 감소
- **결과**: 인프라 비용 수백만 달러 절감

### Discord: 음성/영상 백엔드  
- **이전 (Go)**: GC 멈춤으로 오디오 끊김
- **이후 (Rust)**: 일관된 저지연 성능
- **결과**: 사용자 경험 개선, 서버 비용 감소

### Microsoft: Windows 구성 요소
- **Windows의 Rust**: 파일 시스템, 네트워킹 스택 등
- **이점**: 성능 비용 없이 메모리 안전
- **영향**: 보안 취약점 감소, 성능은 동일

### C# 개발자에게 왜 중요한가:
1. **보완 역량**: Rust와 C#은 다른 문제를 푼다
2. **커리어**: 시스템 프로그래밍 역량의 가치 상승
3. **성능 이해**: 제로 코스트 추상화 학습
4. **안전 사고방식**: 소유권 사고를 다른 언어에도 적용
5. **클라우드 비용**: 성능이 인프라 비용에 직결

***

<a id="installation-and-setup"></a>
## 설치와 환경 구성

### Rust 설치
```bash
# Rust 설치 (Windows, macOS, Linux)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows에서는 https://rustup.rs/ 에서 받을 수도 있음
```

### Rust 도구와 C# 도구
| C# 도구 | Rust 대응 | 용도 |
|---------|----------------|---------|
| `dotnet new` | `cargo new` | 새 프로젝트 생성 |
| `dotnet build` | `cargo build` | 프로젝트 컴파일 |
| `dotnet run` | `cargo run` | 프로젝트 실행 |
| `dotnet test` | `cargo test` | 테스트 실행 |
| NuGet | Crates.io | 패키지 저장소 |
| MSBuild | Cargo | 빌드 시스템 |
| Visual Studio | VS Code + rust-analyzer | IDE |

<a id="ide-setup-for-c-developers"></a>
### C# 개발자를 위한 IDE 설정
1. **VS Code** (초보자에게 권장)
   - "rust-analyzer" 확장 설치
   - 디버깅용 "CodeLLDB" 설치

2. **Visual Studio** (Windows)
   - Rust 지원 확장 설치

3. **JetBrains RustRover** (풀 IDE)
   - Rider와 비슷한 경험

***

<a id="your-first-rust-program"></a>
## 첫 Rust 프로그램

### C# 예제: Hello World
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

### Rust 예제: Hello World
```rust
// main.rs
fn main() {
    println!("Hello, World!");
}
```

### C# 개발자가 알아둘 차이
1. **클래스가 필수는 아님** — 함수를 최상위에 둘 수 있음
2. **네임스페이스 없음** — 대신 모듈 시스템
3. **`println!`은 매크로** — `!`에 주목
4. **`println!` 뒤에 세미콜론 없음** — 표현식과 문
5. **명시적 반환 타입 없음** — `main`은 `()`(유닛 타입) 반환

### 첫 프로젝트 만들기
```bash
# 새 프로젝트 ('dotnet new console'와 유사)
cargo new hello_rust
cd hello_rust

# 생성된 구조:
# hello_rust/
# ├── Cargo.toml      (.csproj와 유사)
# └── src/
#     └── main.rs     (Program.cs와 유사)

# 실행 ('dotnet run'과 유사)
cargo run
```

***

<a id="cargo-vs-nugetmsbuild"></a>
## Cargo와 NuGet·MSBuild

### 프로젝트 설정

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
serde_json = "1.0"    # Newtonsoft.Json처럼
log = "0.4"           # Serilog처럼
```

### 자주 쓰는 Cargo 명령
```bash
# 새 프로젝트
cargo new my_project
cargo new my_project --lib  # 라이브러리 프로젝트

# 빌드와 실행
cargo build          # 'dotnet build'와 유사
cargo run            # 'dotnet run'과 유사
cargo test           # 'dotnet test'와 유사

# 패키지 관리
cargo add serde      # 의존성 추가 ('dotnet add package'와 유사)
cargo update         # 의존성 업데이트

# 릴리스 빌드
cargo build --release  # 최적화 빌드
cargo run --release    # 최적화 버전 실행

# 문서
cargo doc --open     # 문서 생성 후 열기
```

### 워크스페이스와 솔루션

**C# 솔루션 (.sln)**
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

**Rust 워크스페이스 (Cargo.toml)**
```toml
[workspace]
members = [
    "web_api",
    "business", 
    "tests"
]
```

***

<a id="variables-and-mutability"></a>
## 변수와 가변성

### C# 변수 선언
```csharp
// C# — 변수는 기본이 가변
int count = 0;           // 가변
count = 5;               // ✅ 가능

readonly int maxSize = 100;  // 초기화 후 불변
// maxSize = 200;        // ❌ 컴파일 오류

const int BUFFER_SIZE = 1024; // 컴파일 타임 상수
```

### Rust 변수 선언
```rust
// Rust — 변수는 기본이 불변
let count = 0;           // 기본 불변
// count = 5;            // ❌ 컴파일 오류: 불변 변수에 두 번 대입 불가

let mut count = 0;       // 명시적으로 가변
count = 5;               // ✅ 가능

const BUFFER_SIZE: usize = 1024; // 컴파일 타임 상수
```

### C# 개발자가 마음가짐을 바꿀 점
```rust
// 'let'을 기본으로 'readonly'처럼 생각하세요
let name = "John";       // readonly string name = "John;"과 유사
let mut age = 30;        // int age = 30;과 유사

// 변수 섀도잉(Rust만의 특징)
let spaces = "   ";      // 문자열
let spaces = spaces.len(); // 이제 숫자(usize)
// 이것은 가변과 다름 — 새 변수를 만드는 것
```

### 실습 예: 카운터
```csharp
// C# 버전
public class Counter
{
    private int value = 0;
    
    public void Increment()
    {
        value++;  // 변경
    }
    
    public int GetValue() => value;
}
```

```rust
// Rust 버전
pub struct Counter {
    value: i32,  // 기본은 비공개 필드
}

impl Counter {
    pub fn new() -> Counter {
        Counter { value: 0 }
    }
    
    pub fn increment(&mut self) {  // 변경하려면 &mut 필요
        self.value += 1;
    }
    
    pub fn get_value(&self) -> i32 {
        self.value
    }
}
```

***

<a id="built-in-types-comparison"></a>
## 내장 타입 비교

### 기본(primitive) 타입

| C# 타입 | Rust 타입 | 크기 | 범위 |
|---------|-----------|------|------|
| `byte` | `u8` | 8비트 | 0~255 |
| `sbyte` | `i8` | 8비트 | -128~127 |
| `short` | `i16` | 16비트 | -32,768~32,767 |
| `ushort` | `u16` | 16비트 | 0~65,535 |
| `int` | `i32` | 32비트 | -2³¹~2³¹-1 |
| `uint` | `u32` | 32비트 | 0~2³²-1 |
| `long` | `i64` | 64비트 | -2⁶³~2⁶³-1 |
| `ulong` | `u64` | 64비트 | 0~2⁶⁴-1 |
| `float` | `f32` | 32비트 | IEEE 754 부동소수 |
| `double` | `f64` | 64비트 | IEEE 754 부동소수 |
| `bool` | `bool` | 1비트 | true/false |
| `char` | `char` | 32비트 | 유니코드 스칼라 값 |

### 크기 타입 (중요!)
```csharp
// C# — int는 항상 32비트
int arrayIndex = 0;
long fileSize = file.Length;
```

```rust
// Rust — 크기 타입은 포인터 크기와 맞춤(32비트 또는 64비트)
let array_index: usize = 0;    // C의 size_t와 유사
let file_size: u64 = file.len(); // 명시적 64비트
```

### 타입 추론
```csharp
// C# — var 키워드
var name = "John";        // string
var count = 42;           // int
var price = 29.99;        // double
```

```rust
// Rust — 자동 타입 추론
let name = "John";        // &str (문자열 슬라이스)
let count = 42;           // i32 (기본 정수)
let price = 29.99;        // f64 (기본 부동소수)

// 명시적 타입
let count: u32 = 42;
let price: f32 = 29.99;
```

### 배열과 컬렉션 개요
```csharp
// C# — 참조 타입, 힙 할당
int[] numbers = new int[5];        // 고정 크기
List<int> list = new List<int>();  // 동적 크기
```

```rust
// Rust — 여러 선택지
let numbers: [i32; 5] = [1, 2, 3, 4, 5];  // 스택 배열, 고정 크기
let mut list: Vec<i32> = Vec::new();       // 힙 벡터, 동적 크기
```

***

<a id="string-types-string-vs-str"></a>
## 문자열 타입: String과 &str

C# 개발자에게 가장 헷갈리는 주제 중 하나이므로 차근차근 정리합니다.

### C# 문자열 다루기
```csharp
// C# — 단순한 문자열 모델
string name = "John";           // 문자열 리터럴
string greeting = "Hello, " + name;  // 연결
string upper = name.ToUpper();  // 메서드 호출
```

### Rust 문자열 타입
```rust
// Rust — 두 가지 주요 문자열 타입

// 1. &str (문자열 슬라이스) — C#의 ReadOnlySpan<char>에 가깝다
let name: &str = "John";        // 리터럴(불변, 대여)

// 2. String — StringBuilder나 가변 문자열에 가깝다
let mut greeting = String::new();       // 빈 문자열
greeting.push_str("Hello, ");          // 추가
greeting.push_str(name);               // 추가

// 또는 직접 생성
let greeting = String::from("Hello, John");
let greeting = "Hello, John".to_string();  // &str을 String으로
```

### 언제 무엇을 쓸까?

| 상황 | 사용 | C# 대응 |
|----------|-----|---------------|
| 문자열 리터럴 | `&str` | `string` 리터럴 |
| 함수 인자(읽기 전용) | `&str` | `string` 또는 `ReadOnlySpan<char>` |
| 소유·가변 문자열 | `String` | `StringBuilder` |
| 소유 문자열 반환 | `String` | `string` |

### 실습 예
```rust
// 임의의 문자열 타입을 받는 함수
fn greet(name: &str) {  // String과 &str 모두 가능
    println!("Hello, {}!", name);
}

fn main() {
    let literal = "John";                    // &str
    let owned = String::from("Jane");        // String
    
    greet(literal);                          // 가능
    greet(&owned);                           // 가능 (String을 &str로 대여)
    greet("Bob");                            // 가능
}

// 소유 문자열을 반환하는 함수
fn create_greeting(name: &str) -> String {
    format!("Hello, {}!", name)  // format! 매크로는 String 반환
}
```

### C# 개발자: 이렇게 생각하면 됩니다
```rust
// &str은 ReadOnlySpan<char>처럼 문자열 데이터에 대한 뷰
// String은 소유하고 수정할 수 있는 char[]에 가깝다

let borrowed: &str = "I don't own this data";
let owned: String = String::from("I own this data");

// 서로 변환
let owned_copy: String = borrowed.to_string();  // 소유 쪽으로 복사
let borrowed_view: &str = &owned;               // 소유에서 대여
```

***

<a id="comments-and-documentation"></a>
## 주석과 문서화

### 일반 주석
```csharp
// C# 주석
// 한 줄 주석
/* 여러 줄
   주석 */

/// <summary>
/// XML 문서 주석
/// </summary>
/// <param name="name">사용자 이름</param>
/// <returns>인사 문자열</returns>
public string Greet(string name)
{
    return $"Hello, {name}!";
}
```

```rust
// Rust 주석
// 한 줄 주석
/* 여러 줄
   주석 */

/// 문서 주석 (C#의 ///와 유사)
/// 이름으로 사용자에게 인사합니다.
/// 
/// # 인자
/// 
/// * `name` — 문자열 슬라이스로 된 사용자 이름
/// 
/// # 반환
/// 
/// 인사를 담은 `String`
/// 
/// # 예시
/// 
/// ```
/// let greeting = greet("Alice");
/// assert_eq!(greeting, "Hello, Alice!");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

### 문서 생성
```bash
# 문서 생성 (C#의 XML 문서와 유사)
cargo doc --open

# 문서에 있는 예제 코드 테스트
cargo test --doc
```

***

## C# 개발자를 위한 Rust 필수 키워드

Rust 키워드와 역할을 이해하면 C# 개발자가 언어를 훨씬 수월하게 탐색할 수 있습니다.

### 가시성과 접근 제어 키워드

#### C# 접근 한정자
```csharp
public class Example
{
    public int PublicField;           // 어디서나 접근
    private int privateField;        // 이 클래스 안에서만
    protected int protectedField;    // 이 클래스와 하위 클래스
    internal int internalField;      // 이 어셈블리 안에서
    protected internal int protectedInternalField; // 조합
}
```

#### Rust 가시성 키워드
```rust
// pub — 항목을 공개 (C# public과 유사)
pub struct PublicStruct {
    pub public_field: i32,           // 공개 필드
    private_field: i32,              // 기본은 비공개 (키워드 없음)
}

pub mod my_module {
    pub(crate) fn crate_public() {}     // 현재 크레이트 내 공개 (internal과 유사)
    pub(super) fn parent_public() {}    // 부모 모듈에 공개
    pub(self) fn self_public() {}       // 현재 모듈 내 공개 (private과 동일)
    
    pub use super::PublicStruct;        // 재공개 (using 별칭과 유사)
}

// C# protected는 직접 대응 없음 — 합성으로 대체
```

<a id="smart-pointers"></a>
### 메모리와 소유권 키워드

#### C# 메모리 관련 키워드
```csharp
// ref — 참조로 전달
public void Method(ref int value) { value = 10; }

// out — 출력 매개변수
public bool TryParse(string input, out int result) { /* */ }

// in — 읽기 전용 참조 (C# 7.2+)
public void ReadOnly(in LargeStruct data) { /* 데이터 수정 불가 */ }
```

#### Rust 소유권 키워드
```rust
// & — 불변 참조 (C# in 매개변수와 유사)
fn read_only(data: &Vec<i32>) {
    println!("Length: {}", data.len()); // 읽기만 가능
}

// &mut — 가변 참조 (C# ref 매개변수와 유사)
fn modify(data: &mut Vec<i32>) {
    data.push(42); // 수정 가능
}

// move — 클로저에서 이동 캡처 강제
let data = vec![1, 2, 3];
let closure = move || {
    println!("{:?}", data); // data가 클로저로 이동
};
// 이후 data는 사용 불가

// Box — 힙 할당 (C#에서 참조 타입에 new 하는 것과 유사)
let boxed_data = Box::new(42); // 힙에 할당
```

### 제어 흐름 키워드

#### C# 제어 흐름
```csharp
// return — 값과 함께 함수 종료
public int GetValue() { return 42; }

// yield return — 이터레이터 패턴
public IEnumerable<int> GetNumbers()
{
    yield return 1;
    yield return 2;
}

// break/continue — 반복 제어
foreach (var item in items)
{
    if (item == null) continue;
    if (item.Stop) break;
}
```

#### Rust 제어 흐름 키워드
```rust
// return — 명시적 반환 (보통 불필요)
fn get_value() -> i32 {
    return 42; // 명시적 반환
    // 또는 그냥: 42 (암시적 반환)
}

// break/continue — 값을 줄 수 있는 반복 제어
fn find_value() -> Option<i32> {
    loop {
        let value = get_next();
        if value < 0 { continue; }
        if value > 100 { break None; }      // 값과 함께 break
        if value == 42 { break Some(value); } // 성공 시 break
    }
}

// loop — 무한 반복 (while(true)와 유사)
loop {
    if condition { break; }
}

// while — 조건 반복
while condition {
    // code
}

// for — 이터레이터 반복
for item in collection {
    // code
}
```

### 타입 정의 키워드

#### C# 타입 키워드
```csharp
// class — 참조 타입
public class MyClass { }

// struct — 값 타입
public struct MyStruct { }

// interface — 계약 정의
public interface IMyInterface { }

// enum — 열거
public enum MyEnum { Value1, Value2 }

// delegate — 함수 포인터
public delegate void MyDelegate(int value);
```

#### Rust 타입 키워드
```rust
// struct — 자료 구조 (C# class/struct를 합친 느낌)
struct MyStruct {
    field: i32,
}

// enum — 대수적 데이터 타입 (C# enum보다 훨씬 강력함)
enum MyEnum {
    Variant1,
    Variant2(i32),              // 데이터를 담을 수 있음
    Variant3 { x: i32, y: i32 }, // 구조체형 변형
}

// trait — 인터페이스 정의 (C# 인터페이스보다 강력함)
trait MyTrait {
    fn method(&self);
    
    // 기본 구현 (C# 8+ 기본 인터페이스 메서드와 유사)
    fn default_method(&self) {
        println!("Default implementation");
    }
}

// type — 타입 별칭 (C# using 별칭과 유사)
type UserId = u32;
type Result<T> = std::result::Result<T, MyError>;

// impl — 구현 블록 (C#에 직접 대응 없음 — 메서드는 따로 정의)
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

### 함수 정의 키워드

#### C# 함수 관련 키워드
```csharp
// static — 클래스 메서드
public static void StaticMethod() { }

// virtual — 재정의 가능
public virtual void VirtualMethod() { }

// override — 기본 메서드 재정의
public override void VirtualMethod() { }

// abstract — 반드시 구현
public abstract void AbstractMethod();

// async — 비동기 메서드
public async Task<int> AsyncMethod() { return await SomeTask(); }
```

#### Rust 함수 키워드
```rust
// fn — 함수 정의 (C# 메서드이지만 독립 함수도 가능)
fn regular_function() {
    println!("Hello");
}

// const fn — 컴파일 타임 함수 (C# const를 함수에 적용한 느낌)
const fn compile_time_function() -> i32 {
    42 // 컴파일 타임에 평가 가능
}

// async fn — 비동기 함수 (C# async와 유사)
async fn async_function() -> i32 {
    some_async_operation().await
}

// unsafe fn — 메모리 안전을 깰 수 있는 함수
unsafe fn unsafe_function() {
    // unsafe 연산 가능
}

// extern fn — 외부 함수 인터페이스
extern "C" fn c_compatible_function() {
    // C에서 호출 가능
}
```

### 변수 선언 키워드

#### C# 변수 키워드
```csharp
// var — 타입 추론
var name = "John"; // string으로 추론

// const — 컴파일 타임 상수
const int MaxSize = 100;

// readonly — 런타임 이후 불변
readonly DateTime createdAt = DateTime.Now;

// static — 클래스 수준 변수
static int instanceCount = 0;
```

#### Rust 변수 키워드
```rust
// let — 변수 바인딩 (C# var와 유사)
let name = "John"; // 기본 불변

// let mut — 가변 바인딩
let mut count = 0; // 변경 가능
count += 1;

// const — 컴파일 타임 상수 (C# const와 유사)
const MAX_SIZE: usize = 100;

// static — 전역 변수 (C# static과 유사)
static INSTANCE_COUNT: std::sync::atomic::AtomicUsize = 
    std::sync::atomic::AtomicUsize::new(0);
```

### 패턴 매칭 키워드

#### C# 패턴 매칭 (C# 8+)
```csharp
// switch 식
string result = value switch
{
    1 => "One",
    2 => "Two",
    _ => "Other"
};

// is 패턴
if (obj is string str)
{
    Console.WriteLine(str.Length);
}
```

#### Rust 패턴 매칭 키워드
```rust
// match — 패턴 매칭 (C# switch보다 훨씬 강력함)
let result = match value {
    1 => "One",
    2 => "Two",
    3..=10 => "Between 3 and 10", // 범위 패턴
    _ => "Other", // 와일드카드 (C# _와 유사)
};

// if let — 조건부 패턴 매칭
if let Some(value) = optional {
    println!("Got value: {}", value);
}

// while let — 패턴 매칭 반복
while let Some(item) = iterator.next() {
    println!("Item: {}", item);
}

// 패턴이 있는 let — 구조 분해
let (x, y) = point; // 튜플 분해
let Some(value) = optional else {
    return; // 패턴이 맞지 않으면 조기 반환
};
```

### 메모리 안전 키워드

#### C# 메모리(Unsafe) 키워드
```csharp
// unsafe — 안전 검사 끄기
unsafe
{
    int* ptr = &variable;
    *ptr = 42;
}

// fixed — 관리 메모리 고정
unsafe
{
    fixed (byte* ptr = array)
    {
        // ptr 사용
    }
}
```

#### Rust 안전/unsafe 키워드
```rust
// unsafe — 대여 검사기 끄기 (신중하게!)
unsafe {
    let ptr = &variable as *const i32;
    let value = *ptr; // 원시 포인터 역참조
}

// 원시 포인터 타입 (C# 직접 대응 없음 — 보통 불필요)
let ptr: *const i32 = &42;  // 불변 원시 포인터
let ptr: *mut i32 = &mut 42; // 가변 원시 포인터
```

### C#에 없는 Rust 키워드

```rust
// where — 제네릭 제약 (C# where보다 유연함)
fn generic_function<T>() 
where 
    T: Clone + Send + Sync,
{
    // T는 Clone, Send, Sync 트레잇을 구현해야 함
}

// dyn — 동적 트레잇 객체 (C# object보다 타입이 안전함)
let drawable: Box<dyn Draw> = Box::new(Circle::new());

// Self — 구현 중인 타입을 가리킴 (C# this는 인스턴스용)
impl MyStruct {
    fn new() -> Self { // Self = MyStruct
        Self { field: 0 }
    }
}

// self — 메서드 수신자
impl MyStruct {
    fn method(&self) { }        // 불변 대여
    fn method_mut(&mut self) { } // 가변 대여  
    fn consume(self) { }        // 소유권 가져가기
}

// crate — 현재 크레이트 루트
use crate::models::User; // 크레이트 루트 기준 절대 경로

// super — 부모 모듈
use super::utils; // 부모 모듈에서 가져오기
```

### C# 개발자를 위한 키워드 요약

| 용도 | C# | Rust | 핵심 차이 |
|---------|----|----|----------------|
| 가시성 | `public`, `private`, `internal` | `pub`, 기본 비공개 | `pub(crate)` 등으로 더 세분화 |
| 변수 | `var`, `readonly`, `const` | `let`, `let mut`, `const` | 기본이 불변 |
| 함수 | `method()` | `fn` | 독립 함수 가능 |
| 타입 | `class`, `struct`, `interface` | `struct`, `enum`, `trait` | enum은 대수적 타입 |
| 제네릭 | `<T> where T : IFoo` | `<T> where T: Foo` | 제약 표현이 유연함 |
| 참조 | `ref`, `out`, `in` | `&`, `&mut` | 컴파일 타임 대여 검사 |
| 패턴 | `switch`, `is` | `match`, `if let` | 완전 매칭 요구 |

***

<a id="understanding-ownership"></a>
## 소유권 이해하기

소유권은 Rust만의 특징이며 C# 개발자에게 가장 큰 개념 전환입니다. 단계적으로 살펴봅니다.

### C# 메모리 모델 (복습)
```csharp
// C# — 자동 메모리 관리
public void ProcessData()
{
    var data = new List<int> { 1, 2, 3, 4, 5 };
    ProcessList(data);
    // data는 여전히 여기서 사용 가능
    Console.WriteLine(data.Count);  // 문제 없음
    
    // 참조가 없어지면 GC가 정리
}

public void ProcessList(List<int> list)
{
    list.Add(6);  // 원본 리스트를 수정
}
```

### Rust 소유권 규칙
1. **값은 정확히 한 명의 소유자만 가진다**
2. **소유자가 스코프를 벗어나면 값이 drop된다**
3. **소유권은 이전(move)될 수 있다**

```rust
// Rust — 명시적 소유권 관리
fn process_data() {
    let data = vec![1, 2, 3, 4, 5];  // data가 벡터를 소유
    process_list(data);              // 소유권이 함수로 이동
    // println!("{:?}", data);       // ❌ 오류: data는 더 이상 소유하지 않음
}

fn process_list(mut list: Vec<i32>) {  // list가 벡터를 소유
    list.push(6);
    // 함수 종료 시 list가 drop됨
}
```

### C# 개발자를 위한 "이동" 이해
```csharp
// C# — 참조는 복사되고 객체는 그대로
var original = new List<int> { 1, 2, 3 };
var reference = original;  // 둘 다 같은 객체를 가리킴
original.Add(4);
Console.WriteLine(reference.Count);  // 4 — 같은 객체
```

```rust
// Rust — 소유권이 이전됨
let original = vec![1, 2, 3];
let moved = original;       // 소유권 이전
// println!("{:?}", original);  // ❌ 오류: original은 더 이상 데이터를 소유하지 않음
println!("{:?}", moved);    // ✅ moved가 소유
```

### Copy 타입과 Move 타입
```rust
// Copy 타입 (C# 값 타입에 가깝다) — 이동이 아니라 복사
let x = 5;        // i32는 Copy
let y = x;        // x가 y로 복사됨
println!("{}", x); // ✅ x는 여전히 유효

// Move 타입 (C# 참조 타입에 가깝다) — 복사가 아니라 이동  
let s1 = String::from("hello");  // String은 Copy 아님
let s2 = s1;                     // s1이 s2로 이동
// println!("{}", s1);           // ❌ 오류: s1은 더 이상 유효하지 않음
```

### 실습 예: 값 교환
```csharp
// C# — 참조 교환은 단순
public void SwapLists(ref List<int> a, ref List<int> b)
{
    var temp = a;
    a = b;
    b = temp;
}
```

```rust
// Rust — 소유권을 고려한 교환
fn swap_vectors(a: &mut Vec<i32>, b: &mut Vec<i32>) {
    std::mem::swap(a, b);  // 내장 swap
}

// 수동으로 하려면
fn manual_swap() {
    let mut a = vec![1, 2, 3];
    let mut b = vec![4, 5, 6];
    
    let temp = a;  // a를 temp로 이동
    a = b;         // b를 a로 이동
    b = temp;      // temp를 b로 이동
    
    println!("a: {:?}, b: {:?}", a, b);
}
```

***

<a id="references-and-borrowing-basics"></a>
## 대여 기초

대여는 C#의 참조와 비슷하지만, 컴파일 타임에 안전성을 보장합니다.

### C# 참조 매개변수
```csharp
// C# — ref, out 매개변수
public void ModifyValue(ref int value)
{
    value += 10;
}

public void ReadValue(in int value)  // 읽기 전용 참조
{
    Console.WriteLine(value);
}

public bool TryParse(string input, out int result)
{
    return int.TryParse(input, out result);
}
```

### Rust 대여
```rust
// Rust — &와 &mut로 대여
fn modify_value(value: &mut i32) {  // 가변 대여
    *value += 10;
}

fn read_value(value: &i32) {        // 불변 대여
    println!("{}", value);
}

fn main() {
    let mut x = 5;
    
    read_value(&x);       // 불변 대여
    modify_value(&mut x); // 가변 대여
    
    println!("{}", x);    // 여기서는 x가 여전히 소유됨
}
```

### 대여 규칙 (컴파일 타임에 강제!)
```rust
fn borrowing_rules() {
    let mut data = vec![1, 2, 3];
    
    // 규칙 1: 불변 대여는 여러 개 가능
    let r1 = &data;
    let r2 = &data;
    println!("{:?} {:?}", r1, r2);  // ✅ 가능
    
    // 규칙 2: 가변 대여는 동시에 하나만
    let r3 = &mut data;
    // let r4 = &mut data;  // ❌ 오류: 가변 대여를 두 번 할 수 없음
    // let r5 = &data;      // ❌ 오류: 가변 대여 중에는 불변 대여 불가
    
    r3.push(4);  // 가변 대여 사용
    // r3는 여기서 스코프 종료
    
    // 규칙 3: 이전 대여가 끝나면 다시 대여 가능
    let r6 = &data;  // ✅ 이제 가능
    println!("{:?}", r6);
}
```

### C#과 Rust: 참조 안전성
```csharp
// C# — 런타임 오류 가능성
public class ReferenceSafety
{
    private List<int> data = new List<int>();
    
    public List<int> GetData() => data;  // 내부 데이터에 대한 참조 반환
    
    public void UnsafeExample()
    {
        var reference = GetData();
        
        // 다른 스레드가 여기서 data를 수정할 수 있음!
        Thread.Sleep(1000);
        
        // reference가 무효이거나 바뀌었을 수 있음
        reference.Add(42);  // 잠재적 데이터 경쟁
    }
}
```

```rust
// Rust — 컴파일 타임 안전성
pub struct SafeContainer {
    data: Vec<i32>,
}

impl SafeContainer {
    // 불변 대여 반환 — 호출자는 수정 불가
    pub fn get_data(&self) -> &Vec<i32> {
        &self.data
    }
    
    // 가변 대여 반환 — 배타적 접근 보장
    pub fn get_data_mut(&mut self) -> &mut Vec<i32> {
        &mut self.data
    }
}

fn safe_example() {
    let mut container = SafeContainer { data: vec![1, 2, 3] };
    
    let reference = container.get_data();
    // container.get_data_mut();  // ❌ 오류: 불변 대여 중에는 가변 대여 불가
    
    println!("{:?}", reference);  // 불변 참조 사용
    // reference는 여기서 스코프 종료
    
    let mut_reference = container.get_data_mut();  // ✅ 이제 가능
    mut_reference.push(4);
}
```

***

<a id="borrowing-and-lifetimes"></a>
## 참조와 포인터

### C# 포인터 (unsafe 컨텍스트)
```csharp
// C# unsafe 포인터 (드물게 사용)
unsafe void UnsafeExample()
{
    int value = 42;
    int* ptr = &value;  // 값을 가리키는 포인터
    *ptr = 100;         // 역참조 후 수정
    Console.WriteLine(value);  // 100
}
```

### Rust 참조 (기본적으로 안전)
```rust
// Rust 참조(항상 안전 경로)
fn safe_example() {
    let mut value = 42;
    let ptr = &mut value;  // 가변 참조
    *ptr = 100;            // 역참조 후 수정
    println!("{}", value); // 100
}

// unsafe 키워드 없음 — 대여 검사기가 안전성 보장
```

### C# 개발자를 위한 라이프타임 기초
```csharp
// C# — 무효가 될 수 있는 참조를 반환할 수 있음
public class LifetimeIssues
{
    public string GetFirstWord(string input)
    {
        return input.Split(' ')[0];  // 새 문자열 반환(안전)
    }
    
    public unsafe char* GetFirstChar(string input)
    {
        // 위험 — 관리되는 메모리에 대한 포인터 반환
        fixed (char* ptr = input)
            return ptr;  // ❌ 나쁨: 메서드 종료 후 ptr 무효
    }
}
```

```rust
// Rust — 라이프타임 검사로 댕글링 참조 방지
fn get_first_word(input: &str) -> &str {
    input.split_whitespace().next().unwrap_or("")
    // ✅ 안전: 반환 참조의 수명은 input과 같음
}

fn invalid_reference() -> &str {
    let temp = String::from("hello");
    &temp  // ❌ 컴파일 오류: temp가 충분히 오래 살지 않음
    // 함수 끝에서 temp가 drop됨
}

fn valid_reference() -> String {
    let temp = String::from("hello");
    temp  // ✅ 동작: 소유권이 호출자로 이전됨
}
```

***

<a id="move-semantics-vs-reference-semantics"></a>
## 이동 시맨틱

### C# 값 타입과 참조 타입
```csharp
// C# — 값 타입은 복사됨
struct Point
{
    public int X { get; set; }
    public int Y { get; set; }
}

var p1 = new Point { X = 1, Y = 2 };
var p2 = p1;  // 복사
p2.X = 10;
Console.WriteLine(p1.X);  // 여전히 1

// C# — 참조 타입은 객체를 공유
var list1 = new List<int> { 1, 2, 3 };
var list2 = list1;  // 참조 복사
list2.Add(4);
Console.WriteLine(list1.Count);  // 4 — 같은 객체
```

### Rust 이동 시맨틱
```rust
// Rust — Copy가 아닌 타입은 기본이 이동
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn move_example() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1;  // 이동(복사 아님)
    // println!("{:?}", p1);  // ❌ 오류: p1은 이동됨
    println!("{:?}", p2);    // ✅ 가능
}

// 복사를 쓰려면 Copy 트레잇 구현
#[derive(Debug, Copy, Clone)]
struct CopyablePoint {
    x: i32,
    y: i32,
}

fn copy_example() {
    let p1 = CopyablePoint { x: 1, y: 2 };
    let p2 = p1;  // 복사(Copy 구현 때문)
    println!("{:?}", p1);  // ✅ 가능
    println!("{:?}", p2);  // ✅ 가능
}
```

### 값이 이동되는 경우
```rust
fn demonstrate_moves() {
    let s = String::from("hello");
    
    // 1. 대입으로 이동
    let s2 = s;  // s가 s2로 이동
    
    // 2. 함수 호출로 이동
    take_ownership(s2);  // s2가 함수 안으로 이동
    
    // 3. 함수 반환으로 이동
    let s3 = give_ownership();  // 반환값이 s3로 이동
    
    println!("{}", s3);  // s3는 유효
}

fn take_ownership(s: String) {
    println!("{}", s);
    // 여기서 s drop
}

fn give_ownership() -> String {
    String::from("yours")  // 소유권이 호출자로 이동
}
```

### 대여로 이동 피하기
```rust
fn demonstrate_borrowing() {
    let s = String::from("hello");
    
    // 이동 대신 대여
    let len = calculate_length(&s);  // s 대여
    println!("'{}' has length {}", s, len);  // s는 여전히 유효
}

fn calculate_length(s: &String) -> usize {
    s.len()  // 소유하지 않으므로 drop되지 않음
}
```

***

<a id="functions-vs-methods"></a>
## 함수와 메서드

### C# 함수 선언
```csharp
// C# — 클래스 안의 메서드
public class Calculator
{
    // 인스턴스 메서드
    public int Add(int a, int b)
    {
        return a + b;
    }
    
    // 정적 메서드
    public static int Multiply(int a, int b)
    {
        return a * b;
    }
    
    // ref 매개변수가 있는 메서드
    public void Increment(ref int value)
    {
        value++;
    }
}
```

### Rust 함수 선언
```rust
// Rust — 독립 함수
fn add(a: i32, b: i32) -> i32 {
    a + b  // 마지막 식에는 return 불필요
}

fn multiply(a: i32, b: i32) -> i32 {
    return a * b;  // 명시적 return도 가능
}

// 가변 참조를 받는 함수
fn increment(value: &mut i32) {
    *value += 1;
}

fn main() {
    let result = add(5, 3);
    println!("5 + 3 = {}", result);
    
    let mut x = 10;
    increment(&mut x);
    println!("증가 후: {}", x);
}
```

<a id="expression-blocks"></a>
### 표현식과 문 (중요!)
```csharp
// C# — 문과 식
public int GetValue()
{
    if (condition)
    {
        return 42;  // 문
    }
    return 0;       // 문
}
```

```rust
// Rust — 대부분이 표현식이 될 수 있음
fn get_value(condition: bool) -> i32 {
    if condition {
        42  // 표현식(세미콜론 없음)
    } else {
        0   // 표현식(세미콜론 없음)
    }
    // if-else 블록 자체가 값을 내는 표현식
}

// 더 짧게
fn get_value_ternary(condition: bool) -> i32 {
    if condition { 42 } else { 0 }
}
```

### 함수 매개변수와 반환 타입
```rust
// 매개변수 없음, 반환 없음(유닛 타입 () 반환)
fn say_hello() {
    println!("Hello!");
}

// 여러 매개변수
fn greet(name: &str, age: u32) {
    println!("{} is {} years old", name, age);
}

// 튜플로 여러 값 반환
fn divide_and_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
    (dividend / divisor, dividend % divisor)
}

fn main() {
    let (quotient, remainder) = divide_and_remainder(10, 3);
    println!("10 ÷ 3 = {} remainder {}", quotient, remainder);
}
```

***

<a id="conditional-statements"></a>
## 제어 흐름 기초

### 조건문
```csharp
// C# if
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

// C# 삼항 연산자
string message = x > 10 ? "Big" : "Small";
```

```rust
// Rust if 표현식
let x = 5;
if x > 10 {
    println!("Big number");
} else if x > 5 {
    println!("Medium number");
} else {
    println!("Small number");
}

// Rust — 삼항처럼 쓰는 if 표현식
let message = if x > 10 { "Big" } else { "Small" };

// 여러 조건
let message = if x > 10 {
    "Big"
} else if x > 5 {
    "Medium"
} else {
    "Small"
};
```

<a id="loops-and-iteration"></a>
### 반복문
```csharp
// C# 반복문
// for
for (int i = 0; i < 5; i++)
{
    Console.WriteLine(i);
}

// foreach
var numbers = new[] { 1, 2, 3, 4, 5 };
foreach (var num in numbers)
{
    Console.WriteLine(num);
}

// while
int count = 0;
while (count < 3)
{
    Console.WriteLine(count);
    count++;
}
```

```rust
// Rust 반복문
// 범위 기반 for
for i in 0..5 {  // 0부터 4까지(끝 미포함)
    println!("{}", i);
}

// 컬렉션 순회
let numbers = vec![1, 2, 3, 4, 5];
for num in numbers {  // 소유권 이동
    println!("{}", num);
}

// 참조로 순회(더 흔함)
let numbers = vec![1, 2, 3, 4, 5];
for num in &numbers {  // 요소 대여
    println!("{}", num);
}

// while
let mut count = 0;
while count < 3 {
    println!("{}", count);
    count += 1;
}

// 무한 루프 + break
let mut counter = 0;
loop {
    if counter >= 3 {
        break;
    }
    println!("{}", counter);
    counter += 1;
}
```

### 루프 제어
```csharp
// C# 루프 제어
for (int i = 0; i < 10; i++)
{
    if (i == 3) continue;
    if (i == 7) break;
    Console.WriteLine(i);
}
```

```rust
// Rust 루프 제어
for i in 0..10 {
    if i == 3 { continue; }
    if i == 7 { break; }
    println!("{}", i);
}

// 루프 레이블(중첩 루프용)
'outer: for i in 0..3 {
    'inner: for j in 0..3 {
        if i == 1 && j == 1 {
            break 'outer;  // 바깥 루프까지 빠져나감
        }
        println!("i: {}, j: {}", i, j);
    }
}
```

***

<a id="match-expressions"></a>
## 패턴 매칭 입문

Rust의 패턴 매칭은 C#의 `switch`보다 훨씬 강력합니다.

### C# `switch` 문
```csharp
// C# 전통 switch
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

// C# 8+ switch 식
string result = value switch
{
    1 => "One",
    2 => "Two",
    _ => "Other"
};
```

### Rust `match` 표현식
```rust
// Rust match (완전 매칭 필요)
let value = 2;
match value {
    1 => println!("One"),
    2 => println!("Two"),
    _ => println!("Other"),  // _ 와일드카드(default와 유사)
}

// switch 식처럼 값을 내는 match
let result = match value {
    1 => "One",
    2 => "Two",
    _ => "Other",
};

// 여러 패턴
match value {
    1 | 2 => println!("One or Two"),
    3..=5 => println!("Three to Five"), // 범위 패턴
    _ => println!("Other"),
}
```

### `match`로 구조 분해
```csharp
// C# 튜플 분해
var point = (3, 4);
var (x, y) = point;
Console.WriteLine($"x: {x}, y: {y}");

// C# 튜플 패턴 매칭
string classify = point switch
{
    (0, 0) => "Origin",
    (var a, 0) => $"On X-axis at {a}",
    (0, var b) => $"On Y-axis at {b}",
    _ => "Somewhere else"
};
```

```rust
// Rust — match로 튜플 분해
let point = (3, 4);
match point {
    (0, 0) => println!("Origin"),
    (x, 0) => println!("On X-axis at {}", x),
    (0, y) => println!("On Y-axis at {}", y),
    (x, y) => println!("Point at ({}, {})", x, y),
}

// 가드(조건)
match point {
    (x, y) if x == y => println!("On diagonal"),
    (x, y) if x > y => println!("Above diagonal"),
    _ => println!("Below diagonal"),
}
```

***

## 에러 처리 기초

C#의 예외 모델에서 Rust의 명시적 에러 처리로 넘어가는 핵심 전환입니다.

### C# 예외 처리
```csharp
// C# — 예외 기반 에러 처리
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

<a id="resultt-e-for-error-handling"></a>
### Rust `Result` 기반 에러 처리
```rust
use std::fs;
use std::num::ParseIntError;

// 사용자 정의 에러 타입
#[derive(Debug)]
enum ConfigError {
    FileNotFound,
    AccessDenied,
    InvalidFormat,
}

// Result를 반환하는 함수
fn read_config(path: &str) -> Result<String, ConfigError> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(_) => Err(ConfigError::FileNotFound),  // 예시용 단순화
    }
}

// 실패할 수 있는 함수
fn parse_number(input: &str) -> Result<i32, ParseIntError> {
    input.parse::<i32>()  // Result<i32, ParseIntError> 반환
}

fn main() {
    // 에러를 명시적으로 처리
    match read_config("config.txt") {
        Ok(content) => println!("설정: {}", content),
        Err(ConfigError::FileNotFound) => println!("설정 파일을 찾을 수 없음"),
        Err(error) => println!("설정 오류: {:?}", error),
    }
    
    // 파싱 에러 처리
    match parse_number("42") {
        Ok(num) => println!("숫자: {}", num),
        Err(error) => println!("파싱 오류: {}", error),
    }
}
```

### `?` 연산자 (C#의 `await`와 비슷한 전파 역할)
```csharp
// C# — 예외 전파(암시적)
public async Task<string> ProcessFileAsync(string path)
{
    var content = await File.ReadAllTextAsync(path);  // 실패 시 예외
    var processed = ProcessContent(content);          // 실패 시 예외
    return processed;
}
```

```rust
// Rust — ?로 에러 전파
fn process_file(path: &str) -> Result<String, ConfigError> {
    let content = read_config(path)?;  // Err이면 전파
    let processed = process_content(&content)?;  // Err이면 전파
    Ok(processed)  // 성공 값을 Ok로 감쌈
}

fn process_content(content: &str) -> Result<String, ConfigError> {
    if content.is_empty() {
        Err(ConfigError::InvalidFormat)
    } else {
        Ok(content.to_uppercase())
    }
}
```

<a id="optiont-for-null-safety"></a>
### null 안전을 위한 `Option<T>`
```csharp
// C# — nullable 참조 타입
public string? FindUserName(int userId)
{
    var user = database.FindUser(userId);
    return user?.Name;  // 사용자 없으면 null
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
// Rust — Optional 값은 Option<T>
fn find_user_name(user_id: u32) -> Option<String> {
    // DB 조회 시뮬레이션
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
    
    // if let (패턴 매칭 축약)
    if let Some(name) = find_user_name(user_id) {
        println!("User: {}", name);
    } else {
        println!("User not found");
    }
}
```

### `Option`과 `Result` 조합
```rust
fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b != 0.0 {
        Some(a / b)
    } else {
        None
    }
}

fn parse_and_divide(a_str: &str, b_str: &str) -> Result<Option<f64>, ParseFloatError> {
    let a: f64 = a_str.parse()?;  // 잘못된 입력이면 파싱 오류를 그대로 반환
    let b: f64 = b_str.parse()?;  // 잘못된 입력이면 파싱 오류를 그대로 반환
    Ok(safe_divide(a, b))         // Ok(Some(결과)) 또는 나눗셈 불가 시 Ok(None)
}

use std::num::ParseFloatError;

fn main() {
    match parse_and_divide("10.0", "2.0") {
        Ok(Some(result)) => println!("결과: {}", result),
        Ok(None) => println!("0으로 나눔"),
        Err(error) => println!("파싱 오류: {}", error),
    }
}
```

***

<a id="vect-vs-listt"></a>
## `Vec<T>`와 `List<T>` 비교

`Vec<T>`는 C#의 `List<T>`에 대응하지만 소유권 시맨틱이 붙습니다.

### C#의 `List<T>`
```csharp
// C# List<T> — 참조 타입, 힙 할당
var numbers = new List<int>();
numbers.Add(1);
numbers.Add(2);
numbers.Add(3);

// 메서드에 넘기면 참조가 복사됨
ProcessList(numbers);
Console.WriteLine(numbers.Count);  // 여전히 사용 가능

void ProcessList(List<int> list)
{
    list.Add(4);  // 원본 리스트 수정
    Console.WriteLine($"Count in method: {list.Count}");
}
```

### Rust의 `Vec<T>`
```rust
// Rust Vec<T> — 소유 타입, 힙 할당
let mut numbers = Vec::new();
numbers.push(1);
numbers.push(2);
numbers.push(3);

// 소유권을 넘기는 메서드
process_vec(numbers);
// println!("{:?}", numbers);  // ❌ 오류: numbers는 이동됨

// 대여하는 메서드
let mut numbers = vec![1, 2, 3];  // vec! 매크로
process_vec_borrowed(&mut numbers);
println!("{:?}", numbers);  // ✅ 여전히 사용 가능

fn process_vec(mut vec: Vec<i32>) {  // 소유권 이전
    vec.push(4);
    println!("Count in method: {}", vec.len());
    // 여기서 vec drop
}

fn process_vec_borrowed(vec: &mut Vec<i32>) {  // 가변 대여
    vec.push(4);
    println!("Count in method: {}", vec.len());
}
```

### 벡터 생성과 초기화
```csharp
// C# List 초기화
var numbers = new List<int> { 1, 2, 3, 4, 5 };
var empty = new List<int>();
var sized = new List<int>(10);  // 초기 용량

// 다른 컬렉션에서
var fromArray = new List<int>(new[] { 1, 2, 3 });
```

```rust
// Rust Vec 초기화
let numbers = vec![1, 2, 3, 4, 5];  // vec! 매크로
let empty: Vec<i32> = Vec::new();   // 빈 벡터는 타입 필요
let sized = Vec::with_capacity(10); // 용량 미리 할당

// 이터레이터에서
let from_range: Vec<i32> = (1..=5).collect();
let from_array = vec![1, 2, 3];
```

### 자주 쓰는 연산 비교
```csharp
// C# List 연산
var list = new List<int> { 1, 2, 3 };

list.Add(4);                    // 요소 추가
list.Insert(0, 0);              // 인덱스에 삽입
list.Remove(2);                 // 첫 일치 제거
list.RemoveAt(1);               // 인덱스로 제거
list.Clear();                   // 전부 제거

int first = list[0];            // 인덱스 접근
int count = list.Count;         // 개수
bool contains = list.Contains(3); // 포함 여부
```

```rust
// Rust Vec 연산
let mut vec = vec![1, 2, 3];

vec.push(4);                    // 요소 추가
vec.insert(0, 0);               // 인덱스에 삽입
vec.retain(|&x| x != 2);        // 조건으로 제거(함수형)
vec.remove(1);                  // 인덱스로 제거
vec.clear();                    // 전부 제거

let first = vec[0];             // 인덱스 접근(범위 밖이면 패닉)
let safe_first = vec.get(0);    // 안전 접근, Option<&T>
let count = vec.len();          // 개수
let contains = vec.contains(&3); // 포함 여부
```

### 안전한 접근 패턴
```csharp
// C# — 예외 기반 범위 검사
public int SafeAccess(List<int> list, int index)
{
    try
    {
        return list[index];
    }
    catch (ArgumentOutOfRangeException)
    {
        return -1;  // 기본값
    }
}
```

```rust
// Rust — Option 기반 안전 접근
fn safe_access(vec: &Vec<i32>, index: usize) -> Option<i32> {
    vec.get(index).copied()  // Option<i32>
}

fn main() {
    let vec = vec![1, 2, 3];
    
    // 안전 접근
    match vec.get(10) {
        Some(value) => println!("Value: {}", value),
        None => println!("Index out of bounds"),
    }
    
    // unwrap_or 사용
    let value = vec.get(10).copied().unwrap_or(-1);
    println!("Value: {}", value);
}
```

***

<a id="hashmap-vs-dictionary"></a>
## `HashMap`과 `Dictionary` 비교

`HashMap`은 C#의 `Dictionary<K,V>`에 대응합니다.

### C#의 `Dictionary`
```csharp
// C# Dictionary<TKey, TValue>
var scores = new Dictionary<string, int>
{
    ["Alice"] = 100,
    ["Bob"] = 85,
    ["Charlie"] = 92
};

// 추가·갱신
scores["Dave"] = 78;
scores["Alice"] = 105;  // 기존 키 갱신

// 안전 접근
if (scores.TryGetValue("Eve", out int score))
{
    Console.WriteLine($"Eve's score: {score}");
}
else
{
    Console.WriteLine("Eve not found");
}

// 순회
foreach (var kvp in scores)
{
    Console.WriteLine($"{kvp.Key}: {kvp.Value}");
}
```

### Rust의 `HashMap`
```rust
use std::collections::HashMap;

// HashMap 생성·초기화
let mut scores = HashMap::new();
scores.insert("Alice".to_string(), 100);
scores.insert("Bob".to_string(), 85);
scores.insert("Charlie".to_string(), 92);

// 이터레이터로부터
let scores: HashMap<String, i32> = [
    ("Alice".to_string(), 100),
    ("Bob".to_string(), 85),
    ("Charlie".to_string(), 92),
].into_iter().collect();

// 추가/갱신
let mut scores = scores;  // 가변으로
scores.insert("Dave".to_string(), 78);
scores.insert("Alice".to_string(), 105);  // 기존 키 갱신

// 안전 접근
match scores.get("Eve") {
    Some(score) => println!("Eve's score: {}", score),
    None => println!("Eve not found"),
}

// 순회
for (name, score) in &scores {
    println!("{}: {}", name, score);
}
```

### `HashMap` 연산
```csharp
// C# Dictionary 연산
var dict = new Dictionary<string, int>();

dict["key"] = 42;                    // 삽입/갱신
bool exists = dict.ContainsKey("key"); // 존재 여부
bool removed = dict.Remove("key");    // 제거
dict.Clear();                        // 전부 비우기

// 기본값과 함께 가져오기
int value = dict.GetValueOrDefault("missing", 0);
```

```rust
use std::collections::HashMap;

// Rust HashMap 연산
let mut map = HashMap::new();

map.insert("key".to_string(), 42);   // 삽입/갱신
let exists = map.contains_key("key"); // 존재 여부
let removed = map.remove("key");      // 제거, Option<V> 반환
map.clear();                         // 전부 비우기

// Entry API
let mut map = HashMap::new();
map.entry("key".to_string()).or_insert(42);  // 없으면 삽입
map.entry("key".to_string()).and_modify(|v| *v += 1); // 있으면 수정

// 기본값
let value = map.get("missing").copied().unwrap_or(0);
```

### `HashMap` 키·값과 소유권
```rust
// HashMap과 소유권
fn ownership_example() {
    let mut map = HashMap::new();
    
    // String 키·값은 맵으로 이동
    let key = String::from("name");
    let value = String::from("Alice");
    
    map.insert(key, value);
    // println!("{}", key);   // ❌ 오류: key 이동됨
    // println!("{}", value); // ❌ 오류: value 이동됨
    
    // 참조로 접근
    if let Some(name) = map.get("name") {
        println!("Name: {}", name);  // 값 대여
    }
}

// &str 키(소유권 이전 없음)
fn string_slice_keys() {
    let mut map = HashMap::new();
    
    map.insert("name", "Alice");     // &str 키와 값
    map.insert("age", "30");
    
    // 문자열 리터럴은 소유권 문제 없음
    println!("Name exists: {}", map.contains_key("name"));
}
```

***

<a id="arrays-and-slices"></a>
## 배열과 슬라이스

배열·슬라이스·벡터의 차이를 이해하는 것이 중요합니다.

### C# 배열
```csharp
// C# 배열
int[] numbers = new int[5];         // 고정 크기, 힙 할당
int[] initialized = { 1, 2, 3, 4, 5 }; // 배열 리터럴

// 접근
numbers[0] = 10;
int first = numbers[0];

// 길이
int length = numbers.Length;

// 매개변수(참조 타입)
void ProcessArray(int[] array)
{
    array[0] = 99;  // 원본 수정
}
```

### Rust 배열, 슬라이스, 벡터
```rust
// 1. 배열 — 고정 크기, 스택
let numbers: [i32; 5] = [1, 2, 3, 4, 5];  // 타입: [i32; 5]
let zeros = [0; 10];                       // 0 열 개

// 접근
let first = numbers[0];
// numbers[0] = 10;  // ❌ 오류: 기본은 불변

let mut mut_array = [1, 2, 3, 4, 5];
mut_array[0] = 10;  // ✅ mut이면 가능

// 2. 슬라이스 — 배열·벡터에 대한 뷰
let slice: &[i32] = &numbers[1..4];  // 인덱스 1,2,3
let all_slice: &[i32] = &numbers;    // 전체를 슬라이스로

// 3. 벡터 — 동적 크기, 힙(앞에서 다룸)
let mut vec = vec![1, 2, 3, 4, 5];
vec.push(6);  // 확장 가능
```

### 함수 매개변수로 쓰는 슬라이스
```csharp
// C# — 배열 전용 메서드
public void ProcessNumbers(int[] numbers)
{
    for (int i = 0; i < numbers.Length; i++)
    {
        Console.WriteLine(numbers[i]);
    }
}

// 배열에만 적용
ProcessNumbers(new int[] { 1, 2, 3 });
```

```rust
// Rust — 임의 시퀀스에 공통으로 사용
fn process_numbers(numbers: &[i32]) {  // 슬라이스 매개변수
    for (i, num) in numbers.iter().enumerate() {
        println!("Index {}: {}", i, num);
    }
}

fn main() {
    let array = [1, 2, 3, 4, 5];
    let vec = vec![1, 2, 3, 4, 5];
    
    // 같은 함수로 둘 다 처리
    process_numbers(&array);      // 배열을 슬라이스로
    process_numbers(&vec);        // 벡터를 슬라이스로
    process_numbers(&vec[1..4]);  // 부분 슬라이스
}
```

### 문자열 슬라이스 `&str` 다시 보기
```rust
// String과 &str 관계
fn string_slice_example() {
    let owned = String::from("Hello, World!");
    let slice: &str = &owned[0..5];      // "Hello"
    let slice2: &str = &owned[7..];      // "World!"
    
    println!("{}", slice);   // "Hello"
    println!("{}", slice2);  // "World!"
    
    // 임의 문자열 타입을 받는 함수
    print_string("String literal");      // &str
    print_string(&owned);               // String을 &str로
    print_string(slice);                // &str 슬라이스
}

fn print_string(s: &str) {
    println!("{}", s);
}
```

***

<a id="iterator-patterns"></a>
## 컬렉션 다루기

### 순회 패턴
```csharp
// C# 순회
var numbers = new List<int> { 1, 2, 3, 4, 5 };

// 인덱스 있는 for
for (int i = 0; i < numbers.Count; i++)
{
    Console.WriteLine($"Index {i}: {numbers[i]}");
}

// foreach
foreach (int num in numbers)
{
    Console.WriteLine(num);
}

// LINQ
var doubled = numbers.Select(x => x * 2).ToList();
var evens = numbers.Where(x => x % 2 == 0).ToList();
```

```rust
// Rust 순회
let numbers = vec![1, 2, 3, 4, 5];

// 인덱스와 함께
for (i, num) in numbers.iter().enumerate() {
    println!("Index {}: {}", i, num);
}

// 값 순회
for num in &numbers {  // 요소 대여
    println!("{}", num);
}

// 이터레이터 메서드(LINQ와 유사)
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
let evens: Vec<i32> = numbers.iter().filter(|&x| x % 2 == 0).cloned().collect();

// 소비 이터레이터로 한 번에
let doubled: Vec<i32> = numbers.into_iter().map(|x| x * 2).collect();
```

### `Iterator`, `IntoIterator`, `iter()` 비교
```rust
// 서로 다른 순회 방식
fn iteration_methods() {
    let vec = vec![1, 2, 3, 4, 5];
    
    // 1. iter() — 요소 대여 (&T)
    for item in vec.iter() {
        println!("{}", item);  // item은 &i32
    }
    // vec는 아직 사용 가능
    
    // 2. into_iter() — 소유권 가져감 (T)
    for item in vec.into_iter() {
        println!("{}", item);  // item은 i32
    }
    // vec는 더 이상 사용 불가
    
    let mut vec = vec![1, 2, 3, 4, 5];
    
    // 3. iter_mut() — 가변 대여 (&mut T)
    for item in vec.iter_mut() {
        *item *= 2;  // item은 &mut i32
    }
    println!("{:?}", vec);  // [2, 4, 6, 8, 10]
}
```

### 결과 모으기
```csharp
// C# — 오류가 날 수 있는 파싱
public List<int> ParseNumbers(List<string> inputs)
{
    var results = new List<int>();
    foreach (string input in inputs)
    {
        if (int.TryParse(input, out int result))
        {
            results.Add(result);
        }
        // 잘못된 입력은 조용히 건너뜀
    }
    return results;
}
```

```rust
// Rust — collect로 명시적 에러 처리
fn parse_numbers(inputs: Vec<String>) -> Result<Vec<i32>, std::num::ParseIntError> {
    inputs.into_iter()
        .map(|s| s.parse::<i32>())  // Result<i32, ParseIntError>
        .collect()                  // Result<Vec<i32>, ParseIntError>로 수집
}

// 대안: 오류 항목 제외
fn parse_numbers_filter(inputs: Vec<String>) -> Vec<i32> {
    inputs.into_iter()
        .filter_map(|s| s.parse::<i32>().ok())  // Ok만 유지
        .collect()
}

fn main() {
    let inputs = vec!["1".to_string(), "2".to_string(), "invalid".to_string(), "4".to_string()];
    
    // 첫 오류에서 실패하는 버전
    match parse_numbers(inputs.clone()) {
        Ok(numbers) => println!("All parsed: {:?}", numbers),
        Err(error) => println!("Parse error: {}", error),
    }
    
    // 오류를 건너뛰는 버전
    let numbers = parse_numbers_filter(inputs);
    println!("Successfully parsed: {:?}", numbers);  // [1, 2, 4]
}
```

***

<a id="structs-vs-classes"></a>
## 구조체와 클래스

Rust의 구조체는 C#의 클래스와 비슷하지만, 소유권과 메서드 정의 방식에서 중요한 차이가 있습니다.

### C# 클래스 정의
```csharp
// 프로퍼티와 메서드가 있는 C# 클래스
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

### Rust 구조체 정의
```rust
// 연관 함수와 메서드가 있는 Rust 구조체
#[derive(Debug)]  // Debug 트레잇 자동 구현
pub struct Person {
    pub name: String,    // 공개 필드
    pub age: u32,        // 공개 필드
    hobbies: Vec<String>, // 비공개 필드(pub 없음)
}

impl Person {
    // 연관 함수(정적 메서드와 유사)
    pub fn new(name: String, age: u32) -> Person {
        Person {
            name,
            age,
            hobbies: Vec::new(),
        }
    }
    
    // 메서드(&self, &mut self, self)
    pub fn add_hobby(&mut self, hobby: String) {
        self.hobbies.push(hobby);
    }
    
    // 불변 대여 메서드
    pub fn get_info(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
    
    // 비공개 필드용 getter
    pub fn hobbies(&self) -> &Vec<String> {
        &self.hobbies
    }
}
```

### 인스턴스 생성과 사용
```csharp
// C# 객체 생성과 사용
var person = new Person("Alice", 30);
person.AddHobby("Reading");
person.AddHobby("Swimming");

Console.WriteLine(person.GetInfo());
Console.WriteLine($"Hobbies: {string.Join(", ", person.Hobbies)}");

// 프로퍼티 직접 수정
person.Age = 31;
```

```rust
// Rust struct 생성·사용
let mut person = Person::new("Alice".to_string(), 30);
person.add_hobby("Reading".to_string());
person.add_hobby("Swimming".to_string());

println!("{}", person.get_info());
println!("Hobbies: {:?}", person.hobbies());

// 공개 필드 직접 수정
person.age = 31;

// 전체 구조체 디버그 출력
println!("{:?}", person);
```

### 구조체 초기화 패턴
```csharp
// C# 객체 이니셜라이저
var person = new Person("Bob", 25)
{
    Hobbies = new List<string> { "Gaming", "Coding" }
};

// 익명 타입
var anonymous = new { Name = "Charlie", Age = 35 };
```

```rust
// Rust 구조체 초기화
let person = Person {
    name: "Bob".to_string(),
    age: 25,
    hobbies: vec!["Gaming".to_string(), "Coding".to_string()],
};

// 구조체 업데이트 문법(객체 스프레드와 유사)
let older_person = Person {
    age: 26,
    ..person  // 나머지 필드는 person에서 가져옴(person이 이동됨!)
};
```

<a id="tuples"></a>

```rust
// 튜플 구조체(익명 타입과 비슷한 용도)
#[derive(Debug)]
struct Point(i32, i32);

let point = Point(10, 20);
println!("Point: ({}, {})", point.0, point.1);
```

***

<a id="methods-and-associated-functions"></a>
## 메서드와 연관 함수

메서드와 연관 함수의 차이를 이해하는 것이 중요합니다.

### C# 메서드 종류
```csharp
public class Calculator
{
    private int memory = 0;
    
    // 인스턴스 메서드
    public int Add(int a, int b)
    {
        return a + b;
    }
    
    // 상태를 쓰는 인스턴스 메서드
    public void StoreInMemory(int value)
    {
        memory = value;
    }
    
    // 정적 메서드
    public static int Multiply(int a, int b)
    {
        return a * b;
    }
    
    // 정적 팩토리 메서드
    public static Calculator CreateWithMemory(int initialMemory)
    {
        var calc = new Calculator();
        calc.memory = initialMemory;
        return calc;
    }
}
```

### Rust 메서드 종류
```rust
#[derive(Debug)]
pub struct Calculator {
    memory: i32,
}

impl Calculator {
    // 연관 함수(정적 메서드와 유사) — self 없음
    pub fn new() -> Calculator {
        Calculator { memory: 0 }
    }
    
    // 매개변수 있는 연관 함수
    pub fn with_memory(initial_memory: i32) -> Calculator {
        Calculator { memory: initial_memory }
    }
    
    // 불변 대여 메서드 (&self)
    pub fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
    
    // 가변 대여 메서드 (&mut self)
    pub fn store_in_memory(&mut self, value: i32) {
        self.memory = value;
    }
    
    // 소유권을 가져가는 메서드 (self)
    pub fn into_memory(self) -> i32 {
        self.memory  // Calculator 소비됨
    }
    
    // getter
    pub fn memory(&self) -> i32 {
        self.memory
    }
}

fn main() {
    // 연관 함수는 ::로 호출
    let mut calc = Calculator::new();
    let calc2 = Calculator::with_memory(42);
    
    // 메서드는 .로 호출
    let result = calc.add(5, 3);
    calc.store_in_memory(result);
    
    println!("Memory: {}", calc.memory());
    
    // 소비하는 메서드
    let memory_value = calc.into_memory();  // 이후 calc 사용 불가
    println!("Final memory: {}", memory_value);
}
```

### 메서드 수신자(`self`) 유형
```rust
impl Person {
    // &self — 불변 대여(가장 흔함)
    // 읽기만 할 때
    pub fn get_name(&self) -> &str {
        &self.name
    }
    
    // &mut self — 가변 대여
    // 수정할 때
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    
    // self — 소유권 이전(덜 흔함)
    // struct를 소비할 때
    pub fn consume(self) -> String {
        self.name  // Person 이동, 이후 접근 불가
    }
}

fn method_examples() {
    let mut person = Person::new("Alice".to_string(), 30);
    
    // 불변 대여
    let name = person.get_name();  // person은 계속 사용 가능
    println!("Name: {}", name);
    
    // 가변 대여
    person.set_name("Alice Smith".to_string());  // person은 계속 사용 가능
    
    // 소유권 이전
    let final_name = person.consume();  // person은 더 이상 사용 불가
    println!("Final name: {}", final_name);
}
```

***

## 동작 구현하기

### C# 인터페이스 구현
```csharp
// C# 인터페이스
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

### Rust 트레잇 구현(미리보기)
```rust
// Rust trait (인터페이스와 유사)
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

// Circle에 트레잇 구현
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

### 여러 구현
```csharp
// C# — 여러 인터페이스 구현
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
        return 0.0; // 사람에게는 면적이 없다는 뜻의 예시
    }
    
    public int CompareTo(Person other)
    {
        return Age.CompareTo(other.Age);
    }
}
```

```rust
// Rust — 여러 트레잇 구현
use std::cmp::Ordering;

impl Drawable for Person {
    fn draw(&self) {
        println!("Drawing person: {}", self.name);
    }
    
    fn get_area(&self) -> f64 {
        0.0  // 사람에게는 면적이 없다는 뜻의 예시
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

<a id="constructor-patterns"></a>
## 생성자 패턴

### C# 생성자 패턴
```csharp
public class Configuration
{
    public string DatabaseUrl { get; set; }
    public int MaxConnections { get; set; }
    public bool EnableLogging { get; set; }
    
    // 기본 생성자
    public Configuration()
    {
        DatabaseUrl = "localhost";
        MaxConnections = 10;
        EnableLogging = false;
    }
    
    // 매개변수 있는 생성자
    public Configuration(string databaseUrl, int maxConnections)
    {
        DatabaseUrl = databaseUrl;
        MaxConnections = maxConnections;
        EnableLogging = false;
    }
    
    // 팩토리 메서드
    public static Configuration ForProduction()
    {
        return new Configuration("prod.db.server", 100)
        {
            EnableLogging = true
        };
    }
}
```

### Rust 생성자 패턴
```rust
#[derive(Debug)]
pub struct Configuration {
    pub database_url: String,
    pub max_connections: u32,
    pub enable_logging: bool,
}

impl Configuration {
    // 기본 생성자
    pub fn new() -> Configuration {
        Configuration {
            database_url: "localhost".to_string(),
            max_connections: 10,
            enable_logging: false,
        }
    }
    
    // 매개변수 있는 생성자
    pub fn with_database(database_url: String, max_connections: u32) -> Configuration {
        Configuration {
            database_url,
            max_connections,
            enable_logging: false,
        }
    }
    
    // 팩토리 메서드
    pub fn for_production() -> Configuration {
        Configuration {
            database_url: "prod.db.server".to_string(),
            max_connections: 100,
            enable_logging: true,
        }
    }
    
    // 빌더식 메서드
    pub fn enable_logging(mut self) -> Configuration {
        self.enable_logging = true;
        self  // 체이닝을 위해 self 반환
    }
    
    pub fn max_connections(mut self, count: u32) -> Configuration {
        self.max_connections = count;
        self
    }
}

// Default 트레잇 구현
impl Default for Configuration {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    // 다양한 생성 패턴
    let config1 = Configuration::new();
    let config2 = Configuration::with_database("localhost:5432".to_string(), 20);
    let config3 = Configuration::for_production();
    
    // 빌더 패턴
    let config4 = Configuration::new()
        .enable_logging()
        .max_connections(50);
    
    // Default 트레잇 사용
    let config5 = Configuration::default();
    
    println!("{:?}", config4);
}
```

### 빌더 패턴 구현
```rust
// 좀 더 복잡한 빌더 패턴
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
        .expect("설정 빌드 실패");
    
    println!("{:?}", config);
}
```

***

<a id="enums-vs-c-enums"></a>
## 열거형과 패턴 매칭

Rust의 `enum`은 C#의 `enum`보다 훨씬 강하며 데이터를 담을 수 있고, 타입 안전 프로그래밍의 기반이 됩니다.

### C# `enum`의 한계
```csharp
// C# enum — 이름 붙은 상수에 가깝다
public enum Status
{
    Pending,
    Approved,
    Rejected
}

// 기본값이 있는 C# enum
public enum HttpStatusCode
{
    OK = 200,
    NotFound = 404,
    InternalServerError = 500
}

// 복잡한 데이터는 별도 클래스가 필요
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

### Rust `enum`의 힘
```rust
// 단순 enum(C# enum과 비슷)
#[derive(Debug, PartialEq)]
enum Status {
    Pending,
    Approved,
    Rejected,
}

// 데이터를 담는 enum(Rust가 두드러지는 부분)
#[derive(Debug)]
enum Result<T, E> {
    Ok(T),      // 성공 변형, 타입 T
    Err(E),     // 실패 변형, 타입 E
}

// 여러 형태의 데이터를 담는 복합 enum
#[derive(Debug)]
enum Message {
    Quit,                       // 데이터 없음
    Move { x: i32, y: i32 },   // 구조체형 변형
    Write(String),             // 튜플형 변형
    ChangeColor(i32, i32, i32), // 여러 값
}

// 예: HTTP 응답
#[derive(Debug)]
enum HttpResponse {
    Ok { body: String, headers: Vec<String> },
    NotFound { path: String },
    InternalError { message: String, code: u16 },
    Redirect { location: String },
}
```

### `match`로 패턴 매칭
```csharp
// C# switch (제한적)
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
            return "Unknown status"; // 항상 default 필요
    }
}

// C# pattern matching (C# 8+)
public string HandleResult(Result result)
{
    return result switch
    {
        Success success => $"Success: {success.Value}",
        Error error => $"Error: {error.Message}",
        _ => "Unknown result" // 여전히 와일드카드 필요
    };
}
```

```rust
// Rust match — 완전하고 강력함
fn handle_status(status: Status) -> String {
    match status {
        Status::Pending => "Waiting for approval".to_string(),
        Status::Approved => "Request approved".to_string(),
        Status::Rejected => "Request rejected".to_string(),
        // default 불필요 — 컴파일러가 완전 매칭 강제
    }
}

// 데이터 추출 패턴 매칭
fn handle_result<T, E>(result: Result<T, E>) -> String 
where 
    T: std::fmt::Debug,
    E: std::fmt::Debug,
{
    match result {
        Result::Ok(value) => format!("Success: {:?}", value),
        Result::Err(error) => format!("Error: {:?}", error),
        // 완전 매칭 — default 불필요
    }
}

// 복잡한 패턴 매칭
fn handle_message(msg: Message) -> String {
    match msg {
        Message::Quit => "Goodbye!".to_string(),
        Message::Move { x, y } => format!("Move to ({}, {})", x, y),
        Message::Write(text) => format!("Write: {}", text),
        Message::ChangeColor(r, g, b) => format!("Change color to RGB({}, {}, {})", r, g, b),
    }
}

// HTTP 응답 처리
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

### 가드와 고급 패턴
```rust
// 가드가 있는 패턴 매칭
fn describe_number(x: i32) -> String {
    match x {
        n if n < 0 => "negative".to_string(),
        0 => "zero".to_string(),
        n if n < 10 => "single digit".to_string(),
        n if n < 100 => "double digit".to_string(),
        _ => "large number".to_string(),
    }
}

// 범위 매칭
fn describe_age(age: u32) -> String {
    match age {
        0..=12 => "child".to_string(),
        13..=19 => "teenager".to_string(),
        20..=64 => "adult".to_string(),
        65.. => "senior".to_string(),
    }
}

// struct·tuple 구조 분해
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

### `Option`과 `Result` 타입
```csharp
// C# nullable 참조 타입(C# 8+)
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
    
    // 예외 기반 에러 처리
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

// Rust는 null 대신 Option<T>를 씀
struct PersonService {
    people: HashMap<i32, String>,
}

impl PersonService {
    fn new() -> Self {
        PersonService {
            people: HashMap::new(),
        }
    }
    
    // Option<T> 반환 — null 없음
    fn find_person(&self, id: i32) -> Option<&String> {
        self.people.get(&id)
    }
    
    // Option에 대한 패턴 매칭
    fn get_person_or_default(&self, id: i32) -> String {
        match self.find_person(id) {
            Some(name) => name.clone(),
            None => "Unknown".to_string(),
        }
    }
    
    // Option 메서드(함수형 스타일)
    fn get_person_or_default_functional(&self, id: i32) -> String {
        self.find_person(id)
            .map(|name| name.clone())
            .unwrap_or_else(|| "Unknown".to_string())
    }
    
    // 에러 처리용 Result<T, E>
    fn save_person(&mut self, id: i32, name: String) -> Result<(), String> {
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        
        self.people.insert(id, name);
        Ok(())
    }
    
    // 연산 체이닝
    fn get_person_length(&self, id: i32) -> Option<usize> {
        self.find_person(id).map(|name| name.len())
    }
}

fn main() {
    let mut service = PersonService::new();
    
    // Result 처리
    match service.save_person(1, "Alice".to_string()) {
        Ok(()) => println!("Person saved successfully"),
        Err(error) => println!("Error: {}", error),
    }
    
    // Option 처리
    match service.find_person(1) {
        Some(name) => println!("Found: {}", name),
        None => println!("Person not found"),
    }
    
    // Option을 함수형 스타일로
    let name_length = service.get_person_length(1)
        .unwrap_or(0);
    println!("Name length: {}", name_length);
    
    // ? 연산자로 조기 반환
    fn try_operation(service: &mut PersonService) -> Result<String, String> {
        service.save_person(2, "Bob".to_string())?; // 오류면 여기서 반환
        let name = service.find_person(2).ok_or("Person not found")?; // Option → Result
        Ok(format!("Hello, {}", name))
    }
    
    match try_operation(&mut service) {
        Ok(message) => println!("{}", message),
        Err(error) => println!("Operation failed: {}", error),
    }
}
```

### 사용자 정의 에러 타입
```rust
// 사용자 정의 에러 enum
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

// 사용자 정의 에러를 쓰는 PersonService 확장
impl PersonService {
    fn save_person_enhanced(&mut self, id: i32, name: String) -> Result<(), PersonError> {
        if name.is_empty() || name.len() > 50 {
            return Err(PersonError::InvalidName(name));
        }
        
        // 실패할 수 있는 DB 연산 시뮬레이션
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
    
    // 여러 에러 종류 처리
    match service.save_person_enhanced(-1, "Invalid".to_string()) {
        Ok(()) => println!("Success"),
        Err(PersonError::NotFound(id)) => println!("Not found: {}", id),
        Err(PersonError::InvalidName(name)) => println!("Invalid name: {}", name),
        Err(PersonError::DatabaseError(msg)) => println!("DB Error: {}", msg),
    }
}
```

***

<a id="rust-modules-vs-c-namespaces"></a>
## 모듈과 크레이트: 코드 구성

Rust 모듈 시스템은 코드를 정리하고 의존성을 관리하는 데 필수이며, C# 개발자에게는 네임스페이스·어셈블리·NuGet 패키지를 함께 이해하는 것과 비슷합니다.

#### C# 네임스페이스 구성
```csharp
// 파일: Models/User.cs
namespace MyApp.Models
{
    public class User
    {
        public string Name { get; set; }
        public int Age { get; set; }
    }
}

// 파일: Services/UserService.cs
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

// 파일: Program.cs
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

#### Rust 모듈 구성
```rust
// 파일: src/models.rs
pub struct User {
    pub name: String,
    pub age: u32,
}

impl User {
    pub fn new(name: String, age: u32) -> User {
        User { name, age }
    }
}

// 파일: src/services.rs
use crate::models::User;

pub struct UserService;

impl UserService {
    pub fn create_user(name: String, age: u32) -> User {
        User::new(name, age)
    }
}

// 파일: src/lib.rs (또는 main.rs)
pub mod models;
pub mod services;

use models::User;
use services::UserService;

fn main() {
    let service = UserService;
    let user = UserService::create_user("Alice".to_string(), 30);
}
```

<a id="visibility-and-access-control"></a>
### 모듈 계층과 가시성

#### C# 접근 한정자
```csharp
namespace MyApp.Data
{
    // public — 어디서나
    public class Repository
    {
        // private — 이 클래스 안에서만
        private string connectionString;
        
        // internal — 이 어셈블리 안에서
        internal void Connect() { }
        
        // protected — 이 클래스와 파생 클래스
        protected virtual void Initialize() { }
        
        // public — 어디서나
        public void Save(object data) { }
    }
}
```

#### Rust 가시성 규칙
```rust
// Rust는 기본이 모두 비공개
mod data {
    struct Repository {  // 비공개 struct
        connection_string: String,  // 비공개 필드
    }
    
    impl Repository {
        fn new() -> Repository {  // 비공개 함수
            Repository {
                connection_string: "localhost".to_string(),
            }
        }
        
        pub fn connect(&self) {  // 공개 메서드
            // 이 모듈과 하위 모듈에서만 접근
        }
        
        pub(crate) fn initialize(&self) {  // 크레이트 전체 공개
            // 이 크레이트 어디서나 접근
        }
        
        pub(super) fn internal_method(&self) {  // 부모 모듈에 공개
            // 부모 모듈에서 접근
        }
    }
    
    // 공개 struct — 모듈 밖에서도 접근
    pub struct PublicRepository {
        pub data: String,  // 공개 필드
        private_data: String,  // 비공개 필드(pub 없음)
    }
}

pub use data::PublicRepository;  // 외부용 재공개
```

### 모듈 파일 구성

#### C# 프로젝트 구조
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

#### Rust 모듈 파일 구조
```
my_app/
├── Cargo.toml
└── src/
    ├── main.rs (or lib.rs)
    ├── models/
    │   ├── mod.rs        // 모듈 선언
    │   ├── user.rs
    │   └── product.rs
    ├── services/
    │   ├── mod.rs        // 모듈 선언
    │   ├── user_service.rs
    │   └── product_service.rs
    └── controllers/
        ├── mod.rs
        └── api_controller.rs
```

#### 모듈 선언 패턴
```rust
// src/models/mod.rs
pub mod user;      // user.rs를 하위 모듈로 선언
pub mod product;   // product.rs를 하위 모듈로 선언

// 자주 쓰는 타입 재공개
pub use user::User;
pub use product::Product;

// src/main.rs
mod models;     // models/ 디렉터리를 모듈로
mod services;   // services/ 디렉터리를 모듈로

// 특정 항목 가져오기
use models::{User, Product};
use services::UserService;

// 모듈 전체
use models::user::*;  // user 모듈의 공개 항목 전부
```

***

<a id="crates-vs-net-assemblies"></a>
## 크레이트와 .NET 어셈블리

### 크레이트 이해하기

Rust에서 **크레이트**는 컴파일과 코드 배포의 기본 단위이며, .NET의 **어셈블리**와 비슷한 위치에 있습니다.

#### C# 어셈블리 모델
```csharp
// MyLibrary.dll — 컴파일된 어셈블리
namespace MyLibrary
{
    public class Calculator
    {
        public int Add(int a, int b) => a + b;
    }
}

// MyApp.exe — MyLibrary.dll을 참조하는 실행 파일 어셈블리
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

#### Rust 크레이트 모델
```toml
# 라이브러리 크레이트용 Cargo.toml
[package]
name = "my_calculator"
version = "0.1.0"
edition = "2021"

[lib]
name = "my_calculator"
```

```rust
// src/lib.rs — 라이브러리 크레이트
pub struct Calculator;

impl Calculator {
    pub fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}
```

```toml
# 위 라이브러리를 쓰는 바이너리 크레이트 Cargo.toml
[package]
name = "my_app"
version = "0.1.0"
edition = "2021"

[dependencies]
my_calculator = { path = "../my_calculator" }
```

```rust
// src/main.rs — 바이너리 크레이트
use my_calculator::Calculator;

fn main() {
    let calc = Calculator;
    println!("{}", calc.add(2, 3));
}
```

### 크레이트 유형 비교

| C# 개념 | Rust 대응 | 용도 |
|------------|----------------|---------|
| 클래스 라이브러리(.dll) | 라이브러리 크레이트 | 재사용 코드 |
| 콘솔 앱(.exe) | 바이너리 크레이트 | 실행 파일 |
| NuGet 패키지 | 배포된 크레이트 | 배포 단위 |
| 어셈블리(.dll/.exe) | 컴파일된 크레이트 | 컴파일 단위 |
| 솔루션(.sln) | 워크스페이스 | 다중 프로젝트 구성 |

### 워크스페이스와 솔루션

#### C# 솔루션 구조
```xml
<!-- MySolution.sln structure -->
<Solution>
    <Project Include="WebApi/WebApi.csproj" />
    <Project Include="Business/Business.csproj" />
    <Project Include="DataAccess/DataAccess.csproj" />
    <Project Include="Tests/Tests.csproj" />
</Solution>
```

#### Rust 워크스페이스 구조
```toml
# 워크스페이스 루트 Cargo.toml
[workspace]
members = [
    "web_api",
    "business",
    "data_access",
    "tests"
]

[workspace.dependencies]
serde = "1.0"           # 공유 의존성 버전
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
serde = { workspace = true }    # 워크스페이스 버전 사용
tokio = { workspace = true }
```

***

<a id="package-management-cargo-vs-nuget"></a>
## 패키지 관리: Cargo와 NuGet

### 의존성 선언

#### C# NuGet 의존성
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

#### Rust Cargo 의존성
```toml
# Cargo.toml
[package]
name = "my_app"
version = "0.1.0"
edition = "2021"

[dependencies]
serde_json = "1.0"               # crates.io에서 (NuGet과 유사)
serde = { version = "1.0", features = ["derive"] }  # 기능(feature) 지정
log = "0.4"
tokio = { version = "1.0", features = ["full"] }

# 로컬 의존성(ProjectReference와 유사)
my_library = { path = "../my_library" }

# Git 의존성
my_git_crate = { git = "https://github.com/user/repo" }

# 개발 전용 의존성(테스트용 패키지와 유사)
[dev-dependencies]
criterion = "0.5"               # 벤치마크
proptest = "1.0"               # 프로퍼티 기반 테스트
```

### 버전 관리

#### C# 패키지 버전
```xml
<!-- 중앙 집중식 패키지 관리(Directory.Packages.props) -->
<Project>
  <PropertyGroup>
    <ManagePackageVersionsCentrally>true</ManagePackageVersionsCentrally>
  </PropertyGroup>
  
  <PackageVersion Include="Newtonsoft.Json" Version="13.0.3" />
  <PackageVersion Include="Serilog" Version="3.0.1" />
</Project>

<!-- 재현 가능한 빌드를 위한 packages.lock.json -->
```

#### Rust 버전 관리
```toml
# Cargo.toml — 시맨틱 버전
[dependencies]
serde = "1.0"        # 1.x.x와 호환 (>=1.0.0, <2.0.0)
log = "0.4.17"       # 0.4.x와 호환 (>=0.4.17, <0.5.0)
regex = "=1.5.4"     # 정확한 버전
chrono = "^0.4"      # 캐럿 요구(기본)
uuid = "~1.3.0"      # 틸드 요구 (>=1.3.0, <1.4.0)

# Cargo.lock — 재현 가능 빌드를 위한 정확한 버전(자동 생성)
[[package]]
name = "serde"
version = "1.0.163"
# ... 의존성 트리
```

### 패키지 소스

#### C# 패키지 소스
```xml
<!-- nuget.config -->
<configuration>
  <packageSources>
    <add key="nuget.org" value="https://api.nuget.org/v3/index.json" />
    <add key="MyCompanyFeed" value="https://pkgs.dev.azure.com/company/_packaging/feed/nuget/v3/index.json" />
  </packageSources>
</configuration>
```

#### Rust 패키지 소스
```toml
# .cargo/config.toml
[source.crates-io]
replace-with = "my-awesome-registry"

[source.my-awesome-registry]
registry = "https://my-intranet:8080/index"

# 대체 레지스트리
[registries]
my-registry = { index = "https://my-intranet:8080/index" }

# Cargo.toml 예시
[dependencies]
my_crate = { version = "1.0", registry = "my-registry" }
```

### 자주 쓰는 명령 비교

| 작업 | C# | Rust |
|------|------------|-------------|
| 패키지 복원 | `dotnet restore` | `cargo fetch` |
| 패키지 추가 | `dotnet add package Newtonsoft.Json` | `cargo add serde_json` |
| 패키지 제거 | `dotnet remove package Newtonsoft.Json` | `cargo remove serde_json` |
| 패키지 업데이트 | `dotnet update` | `cargo update` |
| 패키지 목록 | `dotnet list package` | `cargo tree` |
| 보안 감사 | `dotnet list package --vulnerable` | `cargo audit` |
| 빌드 정리 | `dotnet clean` | `cargo clean` |

### 기능(feature): 조건부 컴파일

#### C# 조건부 컴파일
```csharp
#if DEBUG
    Console.WriteLine("Debug mode");
#elif RELEASE
    Console.WriteLine("Release mode");
#endif

// 프로젝트 파일에서 기능 정의
<PropertyGroup Condition="'$(Configuration)'=='Debug'">
    <DefineConstants>DEBUG;TRACE</DefineConstants>
</PropertyGroup>
```

#### Rust 기능 게이트
```toml
# Cargo.toml
[features]
default = ["json"]              # 기본 기능 묶음
json = ["serde_json"]          # serde_json을 켜는 기능
xml = ["serde_xml"]            # 대안 직렬화
advanced = ["json", "xml"]     # 복합 기능

[dependencies]
serde_json = { version = "1.0", optional = true }
serde_xml = { version = "0.4", optional = true }
```

```rust
// 기능에 따른 조건부 컴파일
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
    return "직렬화 기능이 켜져 있지 않습니다".to_string();
}
```

### 외부 크레이트 사용

#### C# 개발자에게 유용한 크레이트

| C# 라이브러리 | Rust 크레이트 | 용도 |
|------------|------------|---------|
| Newtonsoft.Json | `serde_json` | JSON 직렬화 |
| HttpClient | `reqwest` | HTTP 클라이언트 |
| Entity Framework | `diesel` / `sqlx` | ORM / SQL 도구 |
| NLog/Serilog | `log` + `env_logger` | 로깅 |
| xUnit/NUnit | 내장 `#[test]` | 단위 테스트 |
| Moq | `mockall` | 모킹 |
| Flurl | `url` | URL 처리 |
| Polly | `tower` | 복원력 패턴 |

#### 예: HTTP 클라이언트 전환
```csharp
// C# HttpClient 사용 예
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
// Rust reqwest 사용 예
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

<a id="traits-vs-interfaces"></a>
<a id="generic-types-and-functions"></a>
<a id="trait-bounds-and-constraints"></a>
<a id="common-standard-library-traits"></a>
## 트레잇 — Rust의 인터페이스

트레잇은 Rust에서 공통 동작을 정의하는 방식이며, C#의 인터페이스와 비슷하지만 더 강합니다.

### C# 인터페이스 비교
```csharp
// C# 인터페이스 정의
public interface IAnimal
{
    string Name { get; }
    void MakeSound();
    
    // 기본 구현(C# 8+)
    string Describe()
    {
        return $"{Name} makes a sound";
    }
}

// C# 인터페이스 구현
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
    
    // 기본 구현 재정의 가능
    public string Describe()
    {
        return $"{Name} is a loyal dog";
    }
}

// 제네릭 제약
public void ProcessAnimal<T>(T animal) where T : IAnimal
{
    animal.MakeSound();
    Console.WriteLine(animal.Describe());
}
```

### Rust 트레잇 정의와 구현
```rust
// 트레잇 정의
trait Animal {
    fn name(&self) -> &str;
    fn make_sound(&self);
    
    // 기본 구현
    fn describe(&self) -> String {
        format!("{} makes a sound", self.name())
    }
    
    // 다른 트레잇 메서드를 쓰는 기본 구현
    fn introduce(&self) {
        println!("Hi, I'm {}", self.name());
        self.make_sound();
    }
}

// struct 정의
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

// 트레잇 구현
impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn make_sound(&self) {
        println!("Woof!");
    }
    
    // 기본 구현 재정의
    fn describe(&self) -> String {
        format!("{} is a loyal {} dog", self.name, self.breed)
    }
}

// 다른 구현
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
    
    // 기본 describe() 사용
}

// 트레잇 경계가 있는 제네릭 함수
fn process_animal<T: Animal>(animal: &T) {
    animal.make_sound();
    println!("{}", animal.describe());
    animal.introduce();
}

// 여러 트레잇 경계
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

### 트레잇 객체와 동적 디스패치
```csharp
// C# 동적 다형성
public void ProcessAnimals(List<IAnimal> animals)
{
    foreach (var animal in animals)
    {
        animal.MakeSound(); // 동적 디스패치
        Console.WriteLine(animal.Describe());
    }
}

// 사용
var animals = new List<IAnimal>
{
    new Dog("Buddy"),
    new Cat("Whiskers"),
    new Dog("Rex")
};

ProcessAnimals(animals);
```

```rust
// Rust 트레잇 객체로 동적 디스패치
fn process_animals(animals: &[Box<dyn Animal>]) {
    for animal in animals {
        animal.make_sound(); // 동적 디스패치
        println!("{}", animal.describe());
    }
}

// 대안: 참조 사용
fn process_animal_refs(animals: &[&dyn Animal]) {
    for animal in animals {
        animal.make_sound();
        println!("{}", animal.describe());
    }
}

fn main() {
    // Box<dyn Trait> 사용
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog::new("Buddy".to_string(), "Golden Retriever".to_string())),
        Box::new(Cat { name: "Whiskers".to_string(), indoor: true }),
        Box::new(Dog::new("Rex".to_string(), "German Shepherd".to_string())),
    ];
    
    process_animals(&animals);
    
    // 참조 사용
    let dog = Dog::new("Buddy".to_string(), "Golden Retriever".to_string());
    let cat = Cat { name: "Whiskers".to_string(), indoor: true };
    
    let animal_refs: Vec<&dyn Animal> = vec![&dog, &cat];
    process_animal_refs(&animal_refs);
}
```

### 파생 트레잇
```rust
// 공통 트레잇 자동 파생
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Person {
    name: String,
    age: u32,
}

// 생성되는 코드(단순화):
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

// 사용
fn main() {
    let person1 = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    
    let person2 = person1.clone(); // Clone 트레잇
    
    println!("{:?}", person1); // Debug 트레잇
    println!("Equal: {}", person1 == person2); // PartialEq 트레잇
}
```

### 자주 쓰는 표준 라이브러리 트레잇
```rust
use std::collections::HashMap;

// Display — 사람이 읽기 좋은 출력
impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (age {})", self.name, self.age)
    }
}

// From — 변환용
impl From<(String, u32)> for Person {
    fn from((name, age): (String, u32)) -> Self {
        Person { name, age }
    }
}

// From이 있으면 Into는 자동 구현
fn create_person() {
    let person: Person = ("Alice".to_string(), 30).into();
    println!("{}", person);
}

// Iterator 구현
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
    
    // 사용자 정의 이터레이터
    for person in Person::iterator(people.clone()) {
        println!("{}", person); // Display 사용
    }
}
```

***

<a id="comprehensive-error-handling"></a>
## 에러 처리 심화

### C# 예외 모델
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
                // 오류 로그만 남기고 다른 파일은 계속 처리
                Console.WriteLine($"Error processing {path}: {ex.Message}");
            }
        }
        
        return results;
    }
}
```

### Rust `Result` 기반 에러 처리
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
        // ? 연산자로 조기 반환
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
    
    // 대안: 성공한 결과만 모으기
    fn process_multiple_files_successful(paths: &[&str]) -> (Vec<String>, Vec<ProcessingError>) {
        let results: Vec<_> = Self::process_multiple_files(paths);
        
        let mut successes = Vec::new();
        let mut errors = Vec::new();
        
        for result in results {
            match result {
                Ok(content) => successes.push(content),
                Err(error) => {
                    eprintln!("오류: {}", error);
                    errors.push(error);
                }
            }
        }
        
        (successes, errors)
    }
}

fn main() {
    let paths = vec!["file1.txt", "file2.txt", "nonexistent.txt"];
    
    // 단일 파일 처리
    match FileProcessor::process_file("example.txt") {
        Ok(content) => println!("내용: {}", content),
        Err(error) => eprintln!("오류: {}", error),
    }
    
    // 여러 파일 — 결과 전부 유지
    let results = FileProcessor::process_multiple_files(&paths);
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(_content) => println!("파일 {}: 성공", i),
            Err(error) => println!("파일 {}: 오류 - {}", i, error),
        }
    }
    
    // 여러 파일 — 성공과 실패 분리
    let (successes, errors) = FileProcessor::process_multiple_files_successful(&paths);
    println!("성공 {}개, 오류 {}개", successes.len(), errors.len());
}
```

***

<a id="configuration-management"></a>
<a id="data-processing-pipelines"></a>
<a id="http-clients-and-apis"></a>
<a id="file-io-and-serialization"></a>
## 실무 마이그레이션 예시

흔한 C# 패턴이 Rust에서는 어떻게 옮겨지는지, 실제에 가까운 시나리오로 살펴봅니다.

### 설정 관리
```csharp
// C# 설정 클래스
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
            return new AppConfig(); // 기본값으로 대체
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
    // 설정 로드 시도, 실패 시 기본값
    let config = AppConfig::load_or_default("config.json");
    println!("Config: {:?}", config);
    
    // 기능 플래그 확인
    if config.is_feature_enabled("debug_mode") {
        println!("Debug mode is enabled");
    }
    
    Ok(())
}
```

### 데이터 처리 파이프라인
```csharp
// C# 데이터 처리
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
        // 비동기 처리 시뮬레이션
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
        // 동시 처리를 위해 futures 사용
        let futures = data.into_iter().map(|item| async move {
            Self::validate(&item)?;
            Self::transform(item).await
        });
        
        // 모든 future 수집
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
        // 비동기 처리 시뮬레이션
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
        RawData { id: 3, value: "".to_string(), timestamp: 1234567892 }, // 잘못된 입력
    ];
    
    // 오류를 명시적으로 처리
    let results = DataProcessor::process(raw_data.clone()).await;
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(processed) => println!("항목 {}: {:?}", i, processed),
            Err(error) => println!("항목 {}: 오류 - {}", i, error),
        }
    }
    
    // 성공한 것만 남기기
    let successful = DataProcessor::process_successful_only(raw_data).await;
    println!("성공적으로 처리한 항목: {}개", successful.len());
    
    Ok(())
}
```

### HTTP 클라이언트 예제
```csharp
// C# HTTP 클라이언트
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
    
    // GET 요청
    match client.get::<User>("users/1").await {
        Ok(user) => println!("User: {:?}", user),
        Err(error) => eprintln!("Failed to get user: {}", error),
    }
    
    // POST 요청
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

<a id="testing-in-rust-vs-c"></a>
<a id="common-pitfalls-for-c-developers"></a>
<a id="learning-path-and-resources"></a>
<a id="moving-to-advanced-topics"></a>
## 학습 경로와 다음 단계

### 당장 할 일(1–2주)
1. **환경 갖추기**
   - [rustup.rs](https://rustup.rs/)로 Rust 설치
   - VS Code에 rust-analyzer 확장 구성
   - 첫 `cargo new hello_world` 프로젝트 만들기

2. **기초 익히기**
   - 간단한 연습으로 소유권 다루기
   - 서로 다른 매개변수 타입(`&str`, `String`, `&mut`)으로 함수 작성
   - 기본 struct와 메서드 구현

3. **에러 처리 연습**
   - C# try-catch를 `Result` 패턴으로 옮겨 보기
   - `?` 연산자와 `match`로 연습
   - 사용자 정의 에러 타입 구현

### 중간 목표(1–2개월)
1. **컬렉션과 이터레이터**
   - `Vec<T>`, `HashMap<K,V>`, `HashSet<T>` 익히기
   - `map`, `filter`, `collect`, `fold` 등 이터레이터 메서드
   - `for` 루프와 이터레이터 체인 비교 연습

2. **트레잇과 제네릭**
   - `Debug`, `Clone`, `PartialEq` 등 자주 쓰는 트레잇 구현
   - 제네릭 함수·struct 작성
   - 트레잇 경계와 `where` 절 이해

3. **프로젝트 구조**
   - 모듈로 코드 나누기
   - `pub` 가시성 이해
   - crates.io의 외부 크레이트 사용

### 고급 주제(3개월 이후)
1. **동시성**
   - `Send`와 `Sync` 트레잇
   - 기본 병렬 처리에 `std::thread`
   - async에는 `tokio` 등 탐색

2. **메모리 관리**
   - 공유 소유권에 `Rc<T>`, `Arc<T>`
   - 힙 할당에 `Box<T>`를 쓰는 때
   - 복잡한 시나리오의 라이프타임

3. **실전 프로젝트**
   - `clap`으로 CLI 도구
   - `axum` 또는 `warp`으로 웹 API
   - 라이브러리 작성 후 crates.io 배포

### 추천 학습 자료

#### 책
- **The Rust Programming Language**(공식 명칭, 한국어로는 “Rust 프로그래밍 언어”로도 부름, 온라인 무료) — 공식 입문서
- **Rust by Example**(온라인 무료) — 예제 중심
- **Programming Rust**(짐 블랜디 등 저) — 기술 깊이

#### 온라인
- [Rust Playground](https://play.rust-lang.org/) — 브라우저에서 실행
- [Rustlings](https://github.com/rust-lang/rustlings) — 대화형 연습
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — 실용 예제

#### 연습 프로젝트
1. **명령줄 계산기** — enum과 패턴 매칭
2. **파일 정리 도구** — 파일 시스템과 에러 처리
3. **JSON 처리기** — serde와 데이터 변환
4. **HTTP 서버** — async와 네트워킹
5. **DB 연동 라이브러리** — 트레잇·제네릭·에러 처리

### C# 개발자가 흔히 하는 실수

#### 소유권 혼동
```rust
// 나쁜 예: 이동된 값을 다시 쓰려 함
fn wrong_way() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // 오류: s는 이동됨
}

// 좋은 예: 필요하면 참조나 clone
fn right_way() {
    let s = String::from("hello");
    borrows_string(&s);
    println!("{}", s); // OK: 여기서는 여전히 소유
}

fn takes_ownership(s: String) { /* s 이동 */ }
fn borrows_string(s: &str) { /* s 대여 */ }
```

#### 대여 검사기와 싸우기
```rust
// 나쁜 예: 가변 참조 여러 개
fn wrong_borrowing() {
    let mut v = vec![1, 2, 3];
    let r1 = &mut v;
    // let r2 = &mut v; // 오류: 가변 대여는 동시에 하나
}

// 좋은 예: 가변 대여 범위를 짧게
fn right_borrowing() {
    let mut v = vec![1, 2, 3];
    {
        let r1 = &mut v;
        r1.push(4);
    } // r1 종료
    
    let r2 = &mut v; // OK: 다른 가변 대여 없음
    r2.push(5);
}
```

#### null을 기대하기
```rust
// 나쁜 예: null 같은 동작을 기대
fn no_null_in_rust() {
    // let s: String = null; // Rust에 null 없음
}

// 좋은 예: Option<T>로 명시
fn use_option_instead() {
    let maybe_string: Option<String> = None;
    
    match maybe_string {
        Some(s) => println!("Got string: {}", s),
        None => println!("No string available"),
    }
}
```

### 마무리 팁

1. **컴파일러를 동료로** — Rust 에러 메시지는 적대적이 아니라 도움이 됩니다.
2. **작게 시작** — 단순한 프로그램부터 복잡도를 올립니다.
3. **남의 코드 읽기** — GitHub의 인기 크레이트를 읽습니다.
4. **질문하기** — Rust 커뮤니티는 환영하는 편입니다.
5. **꾸준히** — 개념은 반복할수록 자연스러워집니다.

Rust는 학습 곡선이 있지만, 메모리 안전·성능·무공포 동시성에서 보답합니다. 처음엔 제약처럼 느껴지는 소유권 규칙도, 익숙해지면 정확하고 효율적인 코드를 쓰는 도구가 됩니다.

---

**축하합니다.** C#에서 Rust로 넘어가기 위한 기반이 마련되었습니다. 작은 프로젝트부터 시작하고, 학습 과정에 여유를 두며, 점차 복잡한 애플리케이션으로 확장해 보세요. Rust의 안전과 성능 이점은 초기 투자를 충분히 가치 있게 합니다.

다음 단계로는 [C# 프로그래머를 위한 고급 Rust 학습](./RustTrainingForCSharp.md)에서 더 정교한 패턴, 성능 최적화, 실무 아키텍처를 다루는 안내를 참고할 수 있습니다.