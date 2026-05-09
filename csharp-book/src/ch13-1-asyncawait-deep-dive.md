<a id="async-programming-c-task-vs-rust-future"></a>
## 비동기 프로그래밍: C# `Task` vs Rust `Future`

> **이 장에서 배우는 것:** Rust의 지연 평가 `Future`와 C#의 즉시 실행 `Task`의 차이, executor 모델(tokio), `Drop` + `select!` 기반 취소와 `CancellationToken`의 차이, 그리고 실제 서비스에서 쓰는 동시 요청 패턴을 배웁니다.
>
> **난이도:** 🔴 고급

C# 개발자는 `async`/`await`에 매우 익숙합니다. Rust도 같은 키워드를 쓰지만, 실행 모델은 근본적으로 다릅니다.

<a id="the-executor-model"></a>
### Executor 모델

```csharp
// C# — 런타임이 기본 스레드 풀과 태스크 스케줄러를 제공한다
// async/await가 별도 설정 없이 바로 동작한다
public async Task<string> FetchDataAsync(string url)
{
    using var client = new HttpClient();
    return await client.GetStringAsync(url);  // .NET 스레드 풀이 스케줄링한다
}
// .NET이 스레드 풀, 태스크 스케줄링, 동기화 컨텍스트를 관리한다
```

```rust
// Rust — 기본 내장 async 런타임이 없다. 직접 executor를 선택해야 한다.
// 가장 널리 쓰이는 것은 tokio다.
async fn fetch_data(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

// async 코드를 실행하려면 반드시 런타임이 필요하다:
#[tokio::main]  // 이 매크로가 tokio 런타임을 설정한다
async fn main() {
    let data = fetch_data("https://example.com").await.unwrap();
    println!("{}", &data[..100]);
}
```

<a id="future-vs-task"></a>
### `Future` vs `Task`

| 항목 | C# `Task<T>` | Rust `Future<Output = T>` |
|---|---|---|
| **실행** | 생성 즉시 시작 | **지연 평가** - `.await`되기 전까지 아무 일도 하지 않음 |
| **런타임** | 내장됨(CLR 스레드 풀) | 외부 런타임 필요(tokio, async-std 등) |
| **취소** | `CancellationToken` | `Future`를 drop하거나 `tokio::select!` 사용 |
| **상태 머신** | 컴파일러가 생성 | 컴파일러가 생성 |
| **메모리 배치** | 힙 할당 | 박싱되기 전까지는 스택에 존재 |

```rust
// 중요: Rust의 Future는 지연 평가된다!
async fn compute() -> i32 { println!("Computing!"); 42 }

let future = compute();  // 아무것도 출력되지 않는다! 아직 Future가 poll되지 않았다.
let result = future.await; // 이제서야 "Computing!"이 출력된다
```

```csharp
// C# Task는 즉시 시작한다!
var task = ComputeAsync();  // "Computing!"이 즉시 출력된다
var result = await task;    // 완료를 기다리기만 한다
```

<a id="cancellation-cancellationtoken-vs-drop--select"></a>
### 취소: `CancellationToken` vs `Drop` / `select!`

```csharp
// C# — CancellationToken을 이용한 협력적 취소
public async Task ProcessAsync(CancellationToken ct)
{
    while (!ct.IsCancellationRequested)
    {
        await Task.Delay(1000, ct);  // 취소되면 예외를 던진다
        DoWork();
    }
}

var cts = new CancellationTokenSource(TimeSpan.FromSeconds(5));
await ProcessAsync(cts.Token);
```

```rust
// Rust — Future를 drop해서 취소하거나, tokio::select!를 사용한다
use tokio::time::{sleep, Duration};

async fn process() {
    loop {
        sleep(Duration::from_secs(1)).await;
        do_work();
    }
}

// select!를 이용한 타임아웃 패턴
async fn run_with_timeout() {
    tokio::select! {
        _ = process() => { println!("Completed"); }
        _ = sleep(Duration::from_secs(5)) => { println!("Timed out!"); }
    }
    // select!가 timeout 분기를 고르면 process() Future는 DROP된다
    // 즉, 자동으로 정리되며 별도의 CancellationToken이 필요 없다
}
```

<a id="real-world-pattern-concurrent-requests-with-timeout"></a>
### 실전 패턴: 타임아웃이 있는 동시 요청

```csharp
// C# — 타임아웃이 있는 동시 HTTP 요청
public async Task<string[]> FetchAllAsync(string[] urls, CancellationToken ct)
{
    var tasks = urls.Select(url => httpClient.GetStringAsync(url, ct));
    return await Task.WhenAll(tasks);
}
```

```rust
// Rust — tokio::join! 또는 futures::join_all을 사용한 동시 요청
use futures::future::join_all;

async fn fetch_all(urls: &[&str]) -> Vec<Result<String, reqwest::Error>> {
    let futures = urls.iter().map(|url| reqwest::get(*url));
    let responses = join_all(futures).await;

    let mut results = Vec::new();
    for resp in responses {
        results.push(resp?.text().await);
    }
    results
}

// 타임아웃 적용:
async fn fetch_all_with_timeout(urls: &[&str]) -> Result<Vec<String>, &'static str> {
    tokio::time::timeout(
        Duration::from_secs(10),
        async {
            let futures: Vec<_> = urls.iter()
                .map(|url| async { reqwest::get(*url).await?.text().await })
                .collect();
            let results = join_all(futures).await;
            results.into_iter().collect::<Result<Vec<_>, _>>()
        }
    )
    .await
    .map_err(|_| "Request timed out")?
    .map_err(|_| "Request failed")
}
```

<details>
<summary><strong>🏋️ 연습문제: 비동기 타임아웃 패턴</strong> (클릭하여 펼치기)</summary>

**문제**: 두 URL에 동시에 요청을 보내고, 먼저 응답한 쪽의 결과를 반환하며 나머지는 취소하는 async 함수를 작성하세요. C#의 `Task.WhenAny`에 해당합니다.

<details>
<summary>🔑 해답</summary>

```rust
use tokio::time::{sleep, Duration};

// 비동기 fetch를 흉내 낸 예제
async fn fetch(url: &str, delay_ms: u64) -> String {
    sleep(Duration::from_millis(delay_ms)).await;
    format!("Response from {url}")
}

async fn fetch_first(url1: &str, url2: &str) -> String {
    tokio::select! {
        result = fetch(url1, 200) => {
            println!("URL 1 won");
            result
        }
        result = fetch(url2, 500) => {
            println!("URL 2 won");
            result
        }
    }
    // 지는 쪽 분기의 Future는 자동으로 drop되어 취소된다
}

#[tokio::main]
async fn main() {
    let result = fetch_first("https://fast.api", "https://slow.api").await;
    println!("{result}");
}
```

**핵심 요점**: `tokio::select!`는 Rust에서 `Task.WhenAny`에 해당하는 도구입니다. 여러 Future를 동시에 경쟁시키고, 가장 먼저 끝난 하나를 선택한 뒤 나머지는 drop(취소)합니다.

</details>
</details>

<a id="spawning-independent-tasks-with-tokiospawn"></a>
### `tokio::spawn`으로 독립적인 태스크 생성하기

C#에서 `Task.Run`은 호출자와 독립적으로 실행되는 작업을 시작합니다. Rust에서 이에 대응하는 것이 `tokio::spawn`입니다.

```rust
use tokio::task;

async fn background_work() {
    // 호출자의 Future가 drop되더라도 독립적으로 실행된다
    let handle = task::spawn(async {
        tokio::time::sleep(Duration::from_secs(2)).await;
        42
    });

    // 생성한 태스크가 도는 동안 다른 일을 한다...
    println!("Doing other work");

    // 필요할 때 결과를 await한다
    let result = handle.await.unwrap(); // 42
}
```

```csharp
// C#에서의 대응 예
var task = Task.Run(async () => {
    await Task.Delay(2000);
    return 42;
});
// 다른 일을 한다...
var result = await task;
```

**핵심 차이**: 일반 `async {}` 블록은 지연 평가되어 await되기 전까지 아무 일도 하지 않습니다. 반면 `tokio::spawn`은 C#의 `Task.Run`처럼 런타임 위에서 즉시 실행을 시작합니다.

<a id="pin-why-rust-async-has-a-concept-c-doesnt"></a>
### `Pin`: 왜 Rust async에는 C#에 없는 개념이 있을까

C# 개발자는 보통 `Pin`을 직접 마주치지 않습니다. CLR의 가비지 컬렉터가 객체를 자유롭게 이동시키고 모든 참조를 자동으로 갱신해 주기 때문입니다. Rust에는 GC가 없습니다. 컴파일러가 `async fn`을 상태 머신으로 바꾸면, 그 구조체 안에는 자기 자신의 필드를 가리키는 내부 포인터가 들어갈 수 있습니다. 이 구조체가 이동하면 그 포인터는 무효가 됩니다.

`Pin<T>`는 다음을 보장하는 래퍼입니다. **"이 값은 메모리에서 더 이상 이동하지 않는다."**

```rust
// Pin은 이런 문맥에서 보게 된다:
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
    //           ^^^^^^^^^^^^^^ 고정됨 - 내부 참조가 유효하게 유지된다
}

// 트레잇에서 박싱된 Future를 반환할 때:
fn make_future() -> Pin<Box<dyn Future<Output = i32> + Send>> {
    Box::pin(async { 42 })
}
```

**실전에서는 `Pin`을 직접 다룰 일이 거의 없습니다.** `async fn`과 `.await` 문법이 대부분을 처리해 줍니다. 보통 다음 경우에만 보게 됩니다.
- 컴파일러 에러 메시지 안에서(제안에 따라 수정)
- `tokio::select!`를 쓸 때(`pin!()` 매크로 사용)
- `dyn Future`를 반환하는 트레잇 메서드를 만들 때(`Box::pin(async { ... })` 사용)

> **더 깊게 보고 싶다면:** 별도 자료인 [Async Rust Training](../../async-book/src/ch04-pin-and-unpin.md)에서 `Pin`, `Unpin`, self-referential struct, structural pinning을 자세히 다룹니다.

***


