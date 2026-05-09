<a id="rust-closures"></a>
## Rust 클로저

> **학습할 내용:** 소유권을 인식하는 캡처 방식(`Fn`/`FnMut`/`FnOnce`)을 가진 Rust 클로저와
> C# 람다의 차이, LINQ를 대체하는 zero-cost Rust 이터레이터, lazy와 eager 평가,
> 그리고 `rayon`을 이용한 병렬 이터레이션을 배웁니다.
>
> **난이도:** 🟡 중급

Rust의 클로저는 C#의 람다와 delegate와 비슷하지만, 캡처가 소유권 규칙을 따르는 점이 다릅니다.

### C# 람다와 델리게이트
```csharp
// C# - 람다는 참조로 캡처
Func<int, int> doubler = x => x * 2;
Action<string> printer = msg => Console.WriteLine(msg);

// 바깥 변수를 캡처하는 클로저
int multiplier = 3;
Func<int, int> multiply = x => x * multiplier;
Console.WriteLine(multiply(5)); // 15

// LINQ는 람다를 광범위하게 사용한다
var evens = numbers.Where(n => n % 2 == 0).ToList();
```

### Rust 클로저
```rust
// Rust 클로저 - 소유권을 인식함
let doubler = |x: i32| x * 2;
let printer = |msg: &str| println!("{}", msg);

// 참조 캡처(불변 캡처의 기본 동작)
let multiplier = 3;
let multiply = |x: i32| x * multiplier; // borrows multiplier
println!("{}", multiply(5)); // 15
println!("{}", multiplier); // still accessible

// move로 캡처
let data = vec![1, 2, 3];
let owns_data = move || {
    println!("{:?}", data); // data moved into closure
};
owns_data();
// println!("{:?}", data); // ERROR: data was moved

// 이터레이터와 함께 클로저 사용
let numbers = vec![1, 2, 3, 4, 5];
let evens: Vec<&i32> = numbers.iter().filter(|&&n| n % 2 == 0).collect();
```

### 클로저 타입
```rust
// Fn - 캡처한 값을 불변으로 빌림
fn apply_fn(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

// FnMut - 캡처한 값을 가변으로 빌림
fn apply_fn_mut(mut f: impl FnMut(i32), values: &[i32]) {
    for &v in values {
        f(v);
    }
}

// FnOnce - 캡처한 값의 소유권을 가져감
fn apply_fn_once(f: impl FnOnce() -> Vec<i32>) -> Vec<i32> {
    f() // 한 번만 호출 가능
}

fn main() {
    // Fn 예제
    let multiplier = 3;
    let result = apply_fn(|x| x * multiplier, 5);
    
    // FnMut 예제
    let mut sum = 0;
    apply_fn_mut(|x| sum += x, &[1, 2, 3, 4, 5]);
    println!("Sum: {}", sum); // 15
    
    // FnOnce 예제
    let data = vec![1, 2, 3];
    let result = apply_fn_once(move || data); // moves data
}
```

***

<a id="linq-vs-rust-iterators"></a>
## LINQ와 Rust 이터레이터

### C# LINQ(Language Integrated Query)
```csharp
// C# LINQ - 선언형 데이터 처리
var numbers = new[] { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 };

var result = numbers
    .Where(n => n % 2 == 0)           // 짝수만 필터링
    .Select(n => n * n)               // 제곱
    .Where(n => n > 10)               // 10보다 큰 값만
    .OrderByDescending(n => n)        // 내림차순 정렬
    .Take(3)                          // 처음 3개만
    .ToList();                        // 실제 컬렉션으로 생성

// 복잡한 객체에 대한 LINQ
var users = GetUsers();
var activeAdults = users
    .Where(u => u.IsActive && u.Age >= 18)
    .GroupBy(u => u.Department)
    .Select(g => new {
        Department = g.Key,
        Count = g.Count(),
        AverageAge = g.Average(u => u.Age)
    })
    .OrderBy(x => x.Department)
    .ToList();

// Async LINQ(추가 라이브러리 사용)
var results = await users
    .ToAsyncEnumerable()
    .WhereAwait(async u => await IsActiveAsync(u.Id))
    .SelectAwait(async u => await EnrichUserAsync(u))
    .ToListAsync();
```

### Rust 이터레이터
```rust
// Rust 이터레이터 - lazy하고 zero-cost인 추상화
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let result: Vec<i32> = numbers
    .iter()
    .filter(|&&n| n % 2 == 0)        // 짝수만 필터링
    .map(|&n| n * n)                 // 제곱
    .filter(|&n| n > 10)             // 10보다 큰 값만
    .collect::<Vec<_>>()             // Vec로 수집
    .into_iter()
    .rev()                           // 뒤집기(내림차순)
    .take(3)                         // 처음 3개만
    .collect();                      // 실제 컬렉션 생성

// 복잡한 이터레이터 체인
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct User {
    name: String,
    age: u32,
    department: String,
    is_active: bool,
}

fn process_users(users: Vec<User>) -> HashMap<String, (usize, f64)> {
    users
        .into_iter()
        .filter(|u| u.is_active && u.age >= 18)
        .fold(HashMap::new(), |mut acc, user| {
            let entry = acc.entry(user.department.clone()).or_insert((0, 0.0));
            entry.0 += 1;  // 개수
            entry.1 += user.age as f64;  // 나이 합계
            acc
        })
        .into_iter()
        .map(|(dept, (count, sum))| (dept, (count, sum / count as f64)))  // 평균
        .collect()
}

// rayon을 이용한 병렬 처리
use rayon::prelude::*;

fn parallel_processing(numbers: Vec<i32>) -> Vec<i32> {
    numbers
        .par_iter()                  // 병렬 이터레이터
        .filter(|&&n| n % 2 == 0)
        .map(|&n| expensive_computation(n))
        .collect()
}

fn expensive_computation(n: i32) -> i32 {
    // 무거운 계산을 흉내 냄
    (0..1000).fold(n, |acc, _| acc + 1)
}
```

```mermaid
graph TD
    subgraph "C# LINQ의 특징"
        CS_LINQ["LINQ 식"]
        CS_EAGER["대개 eager 평가<br/>(ToList(), ToArray())"]
        CS_REFLECTION["[주의] 일부 런타임 reflection<br/>Expression tree"]
        CS_ALLOCATIONS["[주의] 중간 컬렉션 생성<br/>GC 압박"]
        CS_ASYNC["[장점] async 지원<br/>(추가 라이브러리 필요)"]
        CS_SQL["[장점] LINQ to SQL/EF 연동"]
        
        CS_LINQ --> CS_EAGER
        CS_LINQ --> CS_REFLECTION
        CS_LINQ --> CS_ALLOCATIONS
        CS_LINQ --> CS_ASYNC
        CS_LINQ --> CS_SQL
    end
    
    subgraph "Rust 이터레이터의 특징"
        RUST_ITER["이터레이터 체인"]
        RUST_LAZY["[장점] lazy 평가<br/>.collect() 전까지 작업 안 함"]
        RUST_ZERO["[장점] zero-cost 추상화<br/>최적의 루프로 컴파일"]
        RUST_NO_ALLOC["[장점] 중간 할당 없음<br/>스택 기반 처리"]
        RUST_PARALLEL["[장점] 쉬운 병렬화<br/>(rayon 크레이트)"]
        RUST_FUNCTIONAL["[장점] 함수형 스타일<br/>기본적으로 불변"]
        
        RUST_ITER --> RUST_LAZY
        RUST_ITER --> RUST_ZERO
        RUST_ITER --> RUST_NO_ALLOC
        RUST_ITER --> RUST_PARALLEL
        RUST_ITER --> RUST_FUNCTIONAL
    end
    
    subgraph "성능 비교"
        CS_PERF["C# LINQ 성능<br/>[주의] 할당 오버헤드<br/>[주의] 가상 디스패치<br/>[장점] 대부분의 경우 충분히 빠름"]
        RUST_PERF["Rust 이터레이터 성능<br/>[장점] 손으로 최적화한 수준의 속도<br/>[장점] 할당 없음<br/>[장점] 컴파일 타임 최적화"]
    end
    
    style CS_REFLECTION fill:#ffcdd2,color:#000
    style CS_ALLOCATIONS fill:#fff3e0,color:#000
    style RUST_ZERO fill:#c8e6c9,color:#000
    style RUST_LAZY fill:#c8e6c9,color:#000
    style RUST_NO_ALLOC fill:#c8e6c9,color:#000
    style CS_PERF fill:#fff3e0,color:#000
    style RUST_PERF fill:#c8e6c9,color:#000
```

***


<details>
<summary><strong>🏋️ 연습문제: LINQ를 이터레이터로 옮기기</strong> (펼쳐서 보기)</summary>

**도전 과제:** 다음 C# LINQ 파이프라인을 Rust의 관용적인 이터레이터 코드로 옮겨 보세요.

```csharp
// C# - Rust로 옮기기
record Employee(string Name, string Dept, int Salary);

var result = employees
    .Where(e => e.Salary > 50_000)
    .GroupBy(e => e.Dept)
    .Select(g => new {
        Department = g.Key,
        Count = g.Count(),
        AvgSalary = g.Average(e => e.Salary)
    })
    .OrderByDescending(x => x.AvgSalary)
    .ToList();
```

<details>
<summary>🔑 해설</summary>

```rust
use std::collections::HashMap;

struct Employee { name: String, dept: String, salary: u32 }

#[derive(Debug)]
struct DeptStats { department: String, count: usize, avg_salary: f64 }

fn department_stats(employees: &[Employee]) -> Vec<DeptStats> {
    let mut by_dept: HashMap<&str, Vec<u32>> = HashMap::new();
    for e in employees.iter().filter(|e| e.salary > 50_000) {
        by_dept.entry(&e.dept).or_default().push(e.salary);
    }

    let mut stats: Vec<DeptStats> = by_dept
        .into_iter()
        .map(|(dept, salaries)| {
            let count = salaries.len();
            let avg = salaries.iter().sum::<u32>() as f64 / count as f64;
            DeptStats { department: dept.to_string(), count, avg_salary: avg }
        })
        .collect();

    stats.sort_by(|a, b| b.avg_salary.partial_cmp(&a.avg_salary).unwrap());
    stats
}
```

**핵심 요점:**
- Rust 표준 이터레이터에는 내장 `group_by`가 없으므로 `HashMap` + `fold`/`for`가 관용적인 패턴입니다.
- 더 LINQ 같은 문법이 필요하면 `itertools` 크레이트의 `.group_by()`를 사용할 수 있습니다.
- 이터레이터 체인은 zero-cost이므로 컴파일러가 단순한 루프로 최적화합니다.

</details>
</details>


<!-- ch12.0a: itertools — LINQ Power Tools -->
<a id="itertools-the-missing-linq-operations"></a>
## `itertools`: 부족한 LINQ 연산 채우기

Rust 표준 이터레이터는 `map`, `filter`, `fold`, `take`, `collect`를 잘 제공합니다. 하지만 C# 개발자가 `GroupBy`, `Zip`, `Chunk`, `SelectMany`, `Distinct`에 익숙하다면 빈자리가 바로 느껴질 것입니다. 그 틈을 **`itertools`** 크레이트가 메워 줍니다.

```toml
# Cargo.toml
[dependencies]
itertools = "0.12"
```

### 나란히 보기: LINQ vs `itertools`

```csharp
// C# - GroupBy
var byDept = employees.GroupBy(e => e.Department)
    .Select(g => new { Dept = g.Key, Count = g.Count() });

// C# - Chunk(배치 처리)
var batches = items.Chunk(100);  // IEnumerable<T[]>

// C# - Distinct / DistinctBy
var unique = users.DistinctBy(u => u.Email);

// C# - SelectMany(평탄화)
var allTags = posts.SelectMany(p => p.Tags);

// C# - Zip
var pairs = names.Zip(scores, (n, s) => new { Name = n, Score = s });

// C# - sliding window
var windows = data.Zip(data.Skip(1), data.Skip(2))
    .Select(triple => (triple.First + triple.Second + triple.Third) / 3.0);
```

```rust
use itertools::Itertools;

// Rust - group_by(정렬된 입력 필요)
let by_dept = employees.iter()
    .sorted_by_key(|e| &e.department)
    .group_by(|e| &e.department);
for (dept, group) in &by_dept {
    println!("{}: {} employees", dept, group.count());
}

// Rust - chunks(배치 처리)
let batches = items.iter().chunks(100);
for batch in &batches {
    process_batch(batch.collect::<Vec<_>>());
}

// Rust - unique / unique_by
let unique: Vec<_> = users.iter().unique_by(|u| &u.email).collect();

// Rust - flat_map(SelectMany에 대응 - 표준 라이브러리에 포함!)
let all_tags: Vec<&str> = posts.iter().flat_map(|p| &p.tags).collect();

// Rust - zip(표준 라이브러리에 포함!)
let pairs: Vec<_> = names.iter().zip(scores.iter()).collect();

// Rust - tuple_windows(sliding window)
let moving_avg: Vec<f64> = data.iter()
    .tuple_windows::<(_, _, _)>()
    .map(|(a, b, c)| (*a + *b + *c) as f64 / 3.0)
    .collect();
```

### `itertools` 빠른 참고표

| LINQ 메서드 | `itertools` 대응 | 비고 |
|------------|---------------------|-------|
| `GroupBy(key)` | `.sorted_by_key().group_by()` | LINQ와 달리 정렬된 입력이 필요 |
| `Chunk(n)` | `.chunks(n)` | 이터레이터의 이터레이터를 반환 |
| `Distinct()` | `.unique()` | Requires `Eq + Hash` |
| `DistinctBy(key)` | `.unique_by(key)` | |
| `SelectMany()` | `.flat_map()` | 표준 라이브러리에 포함 - 크레이트 불필요 |
| `Zip()` | `.zip()` | 표준 라이브러리에 포함 |
| `Aggregate()` | `.fold()` | 표준 라이브러리에 포함 |
| `Any()` / `All()` | `.any()` / `.all()` | 표준 라이브러리에 포함 |
| `First()` / `Last()` | `.next()` / `.last()` | 표준 라이브러리에 포함 |
| `Skip(n)` / `Take(n)` | `.skip(n)` / `.take(n)` | 표준 라이브러리에 포함 |
| `OrderBy()` | `.sorted()` / `.sorted_by()` | `itertools` 제공(`std`에는 없음) |
| `ThenBy()` | `.sorted_by(\|a,b\| a.x.cmp(&b.x).then(a.y.cmp(&b.y)))` | `Ordering::then` 체이닝 |
| `Intersect()` | `HashSet` intersection | 직접 대응하는 이터레이터 메서드는 없음 |
| `Concat()` | `.chain()` | 표준 라이브러리에 포함 |
| Sliding window | `.tuple_windows()` | 고정 크기 튜플 반환 |
| Cartesian product | `.cartesian_product()` | `itertools` |
| Interleave | `.interleave()` | `itertools` |
| Permutations | `.permutations(k)` | `itertools` |

### 실전 예제: 로그 분석 파이프라인

```rust
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct LogEntry { level: String, module: String, message: String }

fn analyze_logs(entries: &[LogEntry]) {
    // 가장 시끄러운 모듈 상위 5개(LINQ의 GroupBy + OrderByDescending + Take와 비슷함)
    let noisy: Vec<_> = entries.iter()
        .into_group_map_by(|e| &e.module) // itertools: 바로 HashMap으로 그룹화
        .into_iter()
        .sorted_by(|a, b| b.1.len().cmp(&a.1.len()))
        .take(5)
        .collect();

    for (module, entries) in &noisy {
        println!("{}: {} entries", module, entries.len());
    }

    // 100개 단위 윈도우마다 에러 비율(sliding window)
    let error_rates: Vec<f64> = entries.iter()
        .map(|e| if e.level == "ERROR" { 1.0 } else { 0.0 })
        .collect::<Vec<_>>()
        .windows(100)  // std 슬라이스 메서드
        .map(|w| w.iter().sum::<f64>() / 100.0)
        .collect();

    // 연속으로 같은 메시지가 나오는 경우 중복 제거
    let deduped: Vec<_> = entries.iter().dedup_by(|a, b| a.message == b.message).collect();
    println!("Deduped {} → {} entries", entries.len(), deduped.len());
}
```

***


