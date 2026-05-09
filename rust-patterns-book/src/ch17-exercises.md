<a id="exercises"></a>

## 연습문제

<a id="exercise-1-type-safe-state-machine"></a>

### 연습 1: 타입 안전 상태 머신 ★★ (~30분)

타입 상태 패턴으로 신호등 상태 머신을 만드세요. `빨강 → 초록 → 노랑 → 빨강`만 허용되고 다른 순서는 불가능해야 합니다.

<details>
<summary>🔑 해답</summary>

```rust
use std::marker::PhantomData;

struct Red;
struct Green;
struct Yellow;

struct TrafficLight<State> {
    _state: PhantomData<State>,
}

impl TrafficLight<Red> {
    fn new() -> Self {
        println!("🔴 Red — STOP");
        TrafficLight { _state: PhantomData }
    }

    fn go(self) -> TrafficLight<Green> {
        println!("🟢 Green — GO");
        TrafficLight { _state: PhantomData }
    }
}

impl TrafficLight<Green> {
    fn caution(self) -> TrafficLight<Yellow> {
        println!("🟡 Yellow — CAUTION");
        TrafficLight { _state: PhantomData }
    }
}

impl TrafficLight<Yellow> {
    fn stop(self) -> TrafficLight<Red> {
        println!("🔴 Red — STOP");
        TrafficLight { _state: PhantomData }
    }
}

fn main() {
    let light = TrafficLight::new();
    let light = light.go();
    let light = light.caution();
    let light = light.stop();

    // light.caution(); // ❌ 컴파일 에러: Red에 `caution` 없음
    // TrafficLight::new().stop(); // ❌ 컴파일 에러: Red에 `stop` 없음
}
```

**핵심**: 잘못된 전환은 런타임 패닉이 아니라 컴파일 에러입니다.

</details>

---

<a id="exercise-2-unit-of-measure-with-phantomdata"></a>

### 연습 2: PhantomData로 단위 계량 ★★ (~30분)

4장의 단위 계량 패턴을 확장합니다:
- `Meters`, `Seconds`, `Kilograms`
- 같은 단위끼리 덧셈
- 곱셈: `Meters * Meters = SquareMeters`
- 나눗셈: `Meters / Seconds = MetersPerSecond`

<details>
<summary>🔑 해답</summary>

```rust
use std::marker::PhantomData;
use std::ops::{Add, Mul, Div};

#[derive(Clone, Copy)]
struct Meters;
#[derive(Clone, Copy)]
struct Seconds;
#[derive(Clone, Copy)]
struct Kilograms;
#[derive(Clone, Copy)]
struct SquareMeters;
#[derive(Clone, Copy)]
struct MetersPerSecond;

#[derive(Debug, Clone, Copy)]
struct Qty<U> {
    value: f64,
    _unit: PhantomData<U>,
}

impl<U> Qty<U> {
    fn new(v: f64) -> Self { Qty { value: v, _unit: PhantomData } }
}

impl<U> Add for Qty<U> {
    type Output = Qty<U>;
    fn add(self, rhs: Self) -> Self::Output { Qty::new(self.value + rhs.value) }
}

impl Mul<Qty<Meters>> for Qty<Meters> {
    type Output = Qty<SquareMeters>;
    fn mul(self, rhs: Qty<Meters>) -> Qty<SquareMeters> {
        Qty::new(self.value * rhs.value)
    }
}

impl Div<Qty<Seconds>> for Qty<Meters> {
    type Output = Qty<MetersPerSecond>;
    fn div(self, rhs: Qty<Seconds>) -> Qty<MetersPerSecond> {
        Qty::new(self.value / rhs.value)
    }
}

fn main() {
    let width = Qty::<Meters>::new(5.0);
    let height = Qty::<Meters>::new(3.0);
    let area = width * height;
    println!("Area: {:.1} m²", area.value);

    let dist = Qty::<Meters>::new(100.0);
    let time = Qty::<Seconds>::new(9.58);
    let speed = dist / time;
    println!("Speed: {:.2} m/s", speed.value);

    let sum = width + height;
    println!("Sum: {:.1} m", sum.value);

    // let bad = width + time; // ❌ 컴파일 에러: Meters + Seconds 불가
}
```

</details>

---

<a id="exercise-3-channel-based-worker-pool"></a>

### 연습 3: 채널 기반 워커 풀 ★★★ (~45분)

다음을 만족하는 워커 풀을 채널로 구현하세요:
- 디스패처가 `Job`을 채널로 보냄
- N개의 워커가 작업을 꺼내 결과를 되돌려보냄
- `crossbeam-channel`(없으면 `std::sync::mpsc`)

<details>
<summary>🔑 해답</summary>

```rust
use std::sync::mpsc;
use std::thread;

struct Job {
    id: u64,
    data: String,
}

struct JobResult {
    job_id: u64,
    output: String,
    worker_id: usize,
}

fn worker_pool(jobs: Vec<Job>, num_workers: usize) -> Vec<JobResult> {
    let (job_tx, job_rx) = mpsc::channel::<Job>();
    let (result_tx, result_rx) = mpsc::channel::<JobResult>();

    let job_rx = std::sync::Arc::new(std::sync::Mutex::new(job_rx));

    let mut handles = Vec::new();
    for worker_id in 0..num_workers {
        let job_rx = job_rx.clone();
        let result_tx = result_tx.clone();
        handles.push(thread::spawn(move || {
            loop {
                let job = {
                    let rx = job_rx.lock().unwrap();
                    rx.recv()
                };
                match job {
                    Ok(job) => {
                        let output = format!("processed '{}' by worker {worker_id}", job.data);
                        result_tx.send(JobResult {
                            job_id: job.id,
                            output,
                            worker_id,
                        }).unwrap();
                    }
                    Err(_) => break,
                }
            }
        }));
    }
    drop(result_tx);

    let num_jobs = jobs.len();
    for job in jobs {
        job_tx.send(job).unwrap();
    }
    drop(job_tx);

    let mut results = Vec::new();
    for result in result_rx {
        results.push(result);
    }
    assert_eq!(results.len(), num_jobs);

    for h in handles { h.join().unwrap(); }
    results
}

fn main() {
    let jobs: Vec<Job> = (0..20).map(|i| Job {
        id: i,
        data: format!("task-{i}"),
    }).collect();

    let results = worker_pool(jobs, 4);
    for r in &results {
        println!("[worker {}] job {}: {}", r.worker_id, r.job_id, r.output);
    }
}
```

</details>

---

<a id="exercise-4-higher-order-combinator-pipeline"></a>

### 연습 4: 고차 조합자 파이프라인 ★★ (~25분)

변환을 연쇄하는 `Pipeline` 구조체를 만드세요. `.pipe(f)`로 변환을 추가하고 `.execute(input)`으로 전체를 실행합니다.

<details>
<summary>🔑 해답</summary>

```rust
struct Pipeline<T> {
    transforms: Vec<Box<dyn Fn(T) -> T>>,
}

impl<T: 'static> Pipeline<T> {
    fn new() -> Self {
        Pipeline { transforms: Vec::new() }
    }

    fn pipe(mut self, f: impl Fn(T) -> T + 'static) -> Self {
        self.transforms.push(Box::new(f));
        self
    }

    fn execute(self, input: T) -> T {
        self.transforms.into_iter().fold(input, |val, f| f(val))
    }
}

fn main() {
    let result = Pipeline::new()
        .pipe(|s: String| s.trim().to_string())
        .pipe(|s| s.to_uppercase())
        .pipe(|s| format!(">>> {s} <<<"))
        .execute("  hello world  ".to_string());

    println!("{result}");

    let result = Pipeline::new()
        .pipe(|x: i32| x * 2)
        .pipe(|x| x + 10)
        .pipe(|x| x * x)
        .execute(5);

    println!("{result}");
}
```

**보너스**: 단계마다 타입이 바뀌는 제네릭 파이프라인은 다른 설계가 필요합니다 — 각 `.pipe()`가 다른 출력 타입의 `Pipeline`을 반환합니다(더 고급 제네릭 필요).

</details>

---

<a id="exercise-5-error-hierarchy-with-thiserror"></a>

### 연습 5: thiserror로 에러 계층 ★★ (~30분)

I/O, 파싱(JSON·CSV), 검증에서 실패할 수 있는 파일 처리 앱의 에러 타입 계층을 설계하세요. `thiserror`를 쓰고 `?` 전파를 보여주세요.

<details>
<summary>🔑 해답</summary>

```rust,ignore
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("CSV error at line {line}: {message}")]
    Csv { line: usize, message: String },

    #[error("validation error: {field} — {reason}")]
    Validation { field: String, reason: String },
}

fn read_file(path: &str) -> Result<String, AppError> {
    Ok(std::fs::read_to_string(path)?)
}

fn parse_json(content: &str) -> Result<serde_json::Value, AppError> {
    Ok(serde_json::from_str(content)?)
}

fn validate_name(value: &serde_json::Value) -> Result<String, AppError> {
    let name = value.get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::Validation {
            field: "name".into(),
            reason: "must be a non-null string".into(),
        })?;

    if name.is_empty() {
        return Err(AppError::Validation {
            field: "name".into(),
            reason: "must not be empty".into(),
        });
    }

    Ok(name.to_string())
}

fn process_file(path: &str) -> Result<String, AppError> {
    let content = read_file(path)?;
    let json = parse_json(&content)?;
    let name = validate_name(&json)?;
    Ok(name)
}

fn main() {
    match process_file("config.json") {
        Ok(name) => println!("Name: {name}"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
```

</details>

---

<a id="exercise-6-generic-trait-with-associated-types"></a>

### 연습 6: 연관 타입이 있는 제네릭 트레잇 ★★★ (~40분)

연관 `Error`, `Id` 타입을 가진 `Repository` 트레잇을 설계하세요. 인메모리 저장소에 구현하고 컴파일 타임 안전성을 보여주세요.

<details>
<summary>🔑 해답</summary>

```rust
use std::collections::HashMap;

trait Repository {
    type Item;
    type Id;
    type Error;

    fn get(&self, id: &Self::Id) -> Result<Option<&Self::Item>, Self::Error>;
    fn insert(&mut self, item: Self::Item) -> Result<Self::Id, Self::Error>;
    fn delete(&mut self, id: &Self::Id) -> Result<bool, Self::Error>;
}

#[derive(Debug, Clone)]
struct User {
    name: String,
    email: String,
}

struct InMemoryUserRepo {
    data: HashMap<u64, User>,
    next_id: u64,
}

impl InMemoryUserRepo {
    fn new() -> Self {
        InMemoryUserRepo { data: HashMap::new(), next_id: 1 }
    }
}

impl Repository for InMemoryUserRepo {
    type Item = User;
    type Id = u64;
    type Error = std::convert::Infallible;

    fn get(&self, id: &u64) -> Result<Option<&User>, Self::Error> {
        Ok(self.data.get(id))
    }

    fn insert(&mut self, item: User) -> Result<u64, Self::Error> {
        let id = self.next_id;
        self.next_id += 1;
        self.data.insert(id, item);
        Ok(id)
    }

    fn delete(&mut self, id: &u64) -> Result<bool, Self::Error> {
        Ok(self.data.remove(id).is_some())
    }
}

fn create_and_fetch<R: Repository>(repo: &mut R, item: R::Item) -> Result<(), R::Error>
where
    R::Item: std::fmt::Debug,
    R::Id: std::fmt::Debug,
{
    let id = repo.insert(item)?;
    println!("Inserted with id: {id:?}");
    let retrieved = repo.get(&id)?;
    println!("Retrieved: {retrieved:?}");
    Ok(())
}

fn main() {
    let mut repo = InMemoryUserRepo::new();
    create_and_fetch(&mut repo, User {
        name: "Alice".into(),
        email: "alice@example.com".into(),
    }).unwrap();
}
```

</details>

---

<a id="exercise-7-safe-wrapper-around-unsafe"></a>

### 연습 7: Unsafe 주변의 안전한 래퍼(11장) ★★★ (~45분)

스택에 고정 용량을 두는 `FixedVec<T, const N: usize>`를 작성하세요.
요구사항:
- `push(&mut self, value: T) -> Result<(), T>` — 가득 차면 `Err(value)`
- `pop(&mut self) -> Option<T>` — 마지막 요소 제거·반환
- `as_slice(&self) -> &[T]` — 초기화된 요소만 빌림
- 공개 API는 모두 safe; unsafe는 `SAFETY:` 주석과 함께 캡슐화
- `Drop`에서 초기화된 요소 정리

**힌트**: `MaybeUninit<T>`와 `[const { MaybeUninit::uninit() }; N]` 사용.

<details>
<summary>🔑 해답</summary>

```rust
use std::mem::MaybeUninit;

pub struct FixedVec<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> FixedVec<T, N> {
    pub fn new() -> Self {
        FixedVec {
            data: [const { MaybeUninit::uninit() }; N],
            len: 0,
        }
    }

    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.len >= N { return Err(value); }
        // SAFETY: len < N이므로 data[len]는 범위 안.
        self.data[self.len] = MaybeUninit::new(value);
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 { return None; }
        self.len -= 1;
        // SAFETY: data[len]는 감소 전에 초기화되어 있었음.
        Some(unsafe { self.data[self.len].assume_init_read() })
    }

    pub fn as_slice(&self) -> &[T] {
        // SAFETY: data[0..len]는 모두 초기화되었고, MaybeUninit<T>는 T와 같은 레이아웃.
        unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const T, self.len) }
    }

    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }
}

impl<T, const N: usize> Drop for FixedVec<T, N> {
    fn drop(&mut self) {
        // SAFETY: data[0..len]는 초기화됨 — 각각 drop.
        for i in 0..self.len {
            unsafe { self.data[i].assume_init_drop(); }
        }
    }
}

fn main() {
    let mut v = FixedVec::<String, 4>::new();
    v.push("hello".into()).unwrap();
    v.push("world".into()).unwrap();
    assert_eq!(v.as_slice(), &["hello", "world"]);
    assert_eq!(v.pop(), Some("world".into()));
    assert_eq!(v.len(), 1);
}
```

</details>

---

<a id="exercise-8-declarative-macro-map"></a>

### 연습 8: 선언적 매크로 — `map!`(12장) ★ (~15분)

키–값 쌍으로 `HashMap`을 만드는 `map!` 매크로를 작성하세요. `vec![]`와 비슷하게:

```rust
let m = map! {
    "host" => "localhost",
    "port" => "8080",
};
assert_eq!(m.get("host"), Some(&"localhost"));
assert_eq!(m.len(), 2);
```

요구사항:
- 후행 쉼표 허용
- 빈 호출 `map!{}` 지원
- `Into<K>`, `Into<V>`인 타입에 최대한 유연하게

<details>
<summary>🔑 해답</summary>

```rust
macro_rules! map {
    () => {
        std::collections::HashMap::new()
    };
    ( $( $key:expr => $val:expr ),+ $(,)? ) => {{
        let mut m = std::collections::HashMap::new();
        $( m.insert($key, $val); )+
        m
    }};
}

fn main() {
    let config = map! {
        "host" => "localhost",
        "port" => "8080",
        "timeout" => "30",
    };
    assert_eq!(config.len(), 3);
    assert_eq!(config["host"], "localhost");

    let empty: std::collections::HashMap<String, String> = map!();
    assert!(empty.is_empty());

    let scores = map! {
        1 => 100,
        2 => 200,
    };
    assert_eq!(scores[&1], 100);
}
```

</details>

---

<a id="exercise-9-custom-serde-deserialization"></a>

### 연습 9: 사용자 정의 serde 역직렬화(10장) ★★★ (~45분)

`"30s"`, `"5m"`, `"2h"` 같은 사람이 읽기 쉬운 문자열에서 역직렬화하는 `Duration` 래퍼를 설계하세요. 같은 형식으로 다시 직렬화합니다.

<details>
<summary>🔑 해답</summary>

```rust,ignore
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
struct HumanDuration(std::time::Duration);

impl HumanDuration {
    fn from_str(s: &str) -> Result<Self, String> {
        let s = s.trim();
        if s.is_empty() { return Err("empty duration string".into()); }

        let (num_str, suffix) = s.split_at(
            s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len())
        );
        let value: u64 = num_str.parse()
            .map_err(|_| format!("invalid number: {num_str}"))?;

        let duration = match suffix {
            "s" | "sec"  => std::time::Duration::from_secs(value),
            "m" | "min"  => std::time::Duration::from_secs(value * 60),
            "h" | "hr"   => std::time::Duration::from_secs(value * 3600),
            "ms"         => std::time::Duration::from_millis(value),
            other        => return Err(format!("unknown suffix: {other}")),
        };
        Ok(HumanDuration(duration))
    }
}

impl fmt::Display for HumanDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let secs = self.0.as_secs();
        if secs == 0 {
            write!(f, "{}ms", self.0.as_millis())
        } else if secs % 3600 == 0 {
            write!(f, "{}h", secs / 3600)
        } else if secs % 60 == 0 {
            write!(f, "{}m", secs / 60)
        } else {
            write!(f, "{}s", secs)
        }
    }
}

impl Serialize for HumanDuration {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for HumanDuration {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        HumanDuration::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    timeout: HumanDuration,
    retry_interval: HumanDuration,
}

fn main() {
    let json = r#"{ "timeout": "30s", "retry_interval": "5m" }"#;
    let config: Config = serde_json::from_str(json).unwrap();

    assert_eq!(config.timeout.0, std::time::Duration::from_secs(30));
    assert_eq!(config.retry_interval.0, std::time::Duration::from_secs(300));

    let serialized = serde_json::to_string(&config).unwrap();
    assert!(serialized.contains("30s"));
    assert!(serialized.contains("5m"));
    println!("Config: {serialized}");
}
```

</details>

<a id="exercise-10-concurrent-fetcher-with-timeout"></a>

### 연습 10 — 타임아웃이 있는 동시 fetch ★★ (~25분)

16장 연습과 동일: 세 개의 `tokio::spawn`과 `try_join!`, `timeout`을 사용하세요.

**학습 목표**: `tokio::spawn`, `try_join!`, `timeout`, 태스크 경계 너머 에러 전파.

<details>
<summary>힌트</summary>

스폰된 태스크는 `Result<String, _>`를 돌려줍니다. `try_join!`으로 세 개를 풀고,
전체를 `timeout()`으로 감쌉니다 — `Elapsed`는 데드라인 초과입니다.

</details>

<details>
<summary>해답</summary>

```rust,ignore
use tokio::time::{sleep, timeout, Duration};

async fn fake_fetch(name: &'static str, delay_ms: u64) -> Result<String, String> {
    sleep(Duration::from_millis(delay_ms)).await;
    Ok(format!("{name}: OK"))
}

async fn fetch_all() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let deadline = Duration::from_secs(5);

    let (a, b, c) = timeout(deadline, async {
        let h1 = tokio::spawn(fake_fetch("svc-a", 100));
        let h2 = tokio::spawn(fake_fetch("svc-b", 200));
        let h3 = tokio::spawn(fake_fetch("svc-c", 150));
        tokio::try_join!(h1, h2, h3)
    })
    .await??;

    Ok(vec![a?, b?, c?])
}

#[tokio::main]
async fn main() {
    let results = fetch_all().await.unwrap();
    for r in &results {
        println!("{r}");
    }
}
```

</details>

<a id="exercise-11-async-channel-pipeline"></a>

### 연습 11 — async 채널 파이프라인 ★★★ (~40분)

`tokio::sync::mpsc`로 생산자 → 변환기 → 소비자 파이프라인을 만드세요:

1. **생산자**: 1..=20을 채널 A(용량 4)로 전송
2. **변환기**: A에서 읽어 제곱한 뒤 채널 B로 전송
3. **소비자**: B에서 읽어 `Vec<u64>`로 모아 반환

세 단계 모두 `tokio::spawn`으로 동시 실행. 용량 제한 채널로 백프레셔를 보여주고, 최종 벡터가 `[1, 4, 9, ..., 400]`과 같은지 검증합니다.

**학습 목표**: `mpsc::channel`, 백프레셔, `move` 클로저와 `tokio::spawn`, 채널 종료로 정상 종료.

<details>
<summary>해답</summary>

```rust,ignore
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx_a, mut rx_a) = mpsc::channel::<u64>(4);
    let (tx_b, mut rx_b) = mpsc::channel::<u64>(4);

    let producer = tokio::spawn(async move {
        for i in 1..=20u64 {
            tx_a.send(i).await.unwrap();
        }
    });

    let transformer = tokio::spawn(async move {
        while let Some(val) = rx_a.recv().await {
            tx_b.send(val * val).await.unwrap();
        }
    });

    let consumer = tokio::spawn(async move {
        let mut results = Vec::new();
        while let Some(val) = rx_b.recv().await {
            results.push(val);
        }
        results
    });

    producer.await.unwrap();
    transformer.await.unwrap();
    let results = consumer.await.unwrap();

    let expected: Vec<u64> = (1..=20).map(|x: u64| x * x).collect();
    assert_eq!(results, expected);
    println!("Pipeline complete: {results:?}");
}
```

</details>

***

