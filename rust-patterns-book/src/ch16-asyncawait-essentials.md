<a id="asyncawait-essentials"></a>

# 16. Async/Await 핵심 🔴

> **이 장에서 배울 내용:**
> - Rust의 `Future` 트레잇이 Go 고루틴·Python asyncio와 어떻게 다른지
> - Tokio 빠른 시작: 태스크 생성, `join!`, 런타임 설정
> - 흔한 async 함정과 고치는 법
> - 블로킹 작업을 `spawn_blocking`으로 넘길 때

<a id="futures-runtimes-and-async-fn"></a>

## Future, 런타임, `async fn`

Rust의 async 모델은 Go 고루틴이나 Python `asyncio`와 *근본적으로 다릅니다*.
시작하려면 세 가지만 이해하면 됩니다:

1. **`Future`는 게으른 상태 머신** — `async fn`을 호출해도 바로 실행되지 않고,
   `Future`를 반환하며 poll되어야 합니다.
2. **런타임이 필요** — future를 poll하려면 `tokio`, `async-std`, `smol` 등이 필요합니다.
   표준 라이브러리는 `Future`만 정의하고 런타임은 제공하지 않습니다.
3. **`async fn`은 문법 설탕** — 컴파일러가 상태 머신으로 바꿔 `Future`를 구현합니다.

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

async fn fetch_data(url: &str) -> Result<Vec<u8>, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    Ok(bytes.to_vec())
}
```

<a id="tokio-quick-start"></a>

### Tokio 빠른 시작

```toml
```

# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }

```rust,ignore
use tokio::time::{sleep, Duration};
use tokio::task;

#[tokio::main]
async fn main() {
    let handle_a = task::spawn(async {
        sleep(Duration::from_millis(100)).await;
        "task A done"
    });

    let handle_b = task::spawn(async {
        sleep(Duration::from_millis(50)).await;
        "task B done"
    });

    let (a, b) = tokio::join!(handle_a, handle_b);
    println!("{}, {}", a.unwrap(), b.unwrap());
}
```

<a id="async-common-pitfalls"></a>

### async 흔한 함정

| 함정 | 이유 | 해결 |
|---------|---------------|-----|
| async 안에서 블로킹 | `std::thread::sleep`이나 CPU 작업이 executor를 막음 | `tokio::task::spawn_blocking` 또는 `rayon` |
| `Send` 바운드 에러 | `.await`를 넘나드는 future 안에 `!Send` 타입(`Rc`, `MutexGuard` 등) | `.await` 전에 `!Send` 값을 드롭하도록 구조 변경 |
| future를 poll 안 함 | `async fn`만 호출하고 `.await`나 spawn 없음 — 아무 일도 안 함 | 항상 `.await`하거나 `tokio::spawn` |
| `.await`를 넘겨 `MutexGuard` 유지 | `std::sync::MutexGuard`는 `!Send` | `tokio::sync::Mutex` 쓰거나 guard를 `.await` 전에 드롭 |
| 순차 실행에 의한 성능 | `let a = foo().await; let b = bar().await;`는 순차 | 동시에 돌리려면 `tokio::join!` 또는 `tokio::spawn` |

```rust
async fn bad() {
    std::thread::sleep(std::time::Duration::from_secs(5));
}

async fn good() {
    tokio::task::spawn_blocking(|| {
        std::thread::sleep(std::time::Duration::from_secs(5));
    }).await.unwrap();
}
```

> **async 전체**: `Stream`, `select!`, 취소 안전성, 구조적 동시성, `tower` 미들웨어는
> 별도 **Async Rust Training** 가이드를 보세요. 여기서는 기본적인 읽기·쓰기만 다룹니다.

<a id="spawning-and-structured-concurrency"></a>

### 스폰과 구조적 동시성

Tokio의 `spawn`은 새 비동기 태스크를 만듭니다 — `thread::spawn`과 비슷하지만 훨씬 가볍습니다.

```rust,ignore
use tokio::task;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let h1 = task::spawn(async {
        sleep(Duration::from_millis(200)).await;
        "fetched user profile"
    });

    let h2 = task::spawn(async {
        sleep(Duration::from_millis(100)).await;
        "fetched order history"
    });

    let h3 = task::spawn(async {
        sleep(Duration::from_millis(150)).await;
        "fetched recommendations"
    });

    let (r1, r2, r3) = tokio::join!(h1, h2, h3);
    println!("{}", r1.unwrap());
    println!("{}", r2.unwrap());
    println!("{}", r3.unwrap());
}
```

**`join!` vs `try_join!` vs `select!`**:

| 매크로 | 동작 | 쓸 때 |
|-------|----------|----------|
| `join!` | 모든 future 대기 | 전부 완료되어야 할 때 |
| `try_join!` | 전부 대기, 첫 `Err`에서 중단 | `Result`를 반환할 때 |
| `select!` | **첫** future 완료 시 반환 | 타임아웃, 취소 |

```rust,ignore
use tokio::time::{timeout, Duration};

async fn fetch_with_timeout() -> Result<String, Box<dyn std::error::Error>> {
    let result = timeout(Duration::from_secs(5), async {
        tokio::time::sleep(Duration::from_millis(100)).await;
        Ok::<_, Box<dyn std::error::Error>>("data".to_string())
    }).await??;

    Ok(result)
}
```

<a id="send-bounds-and-why-futures-must-be-send"></a>

### `Send` 바운드와 스폰된 future가 `Send`이어야 하는 이유

`tokio::spawn`한 future는 다른 OS 스레드에서 resume될 수 있어 `Send`이어야 합니다.

```rust,ignore
use std::rc::Rc;

async fn not_send() {
    let rc = Rc::new(42);
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    println!("{}", rc);
}

async fn fixed_drop() {
    let data = {
        let rc = Rc::new(42);
        *rc
    };
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    println!("{}", data);
}

async fn fixed_arc() {
    let arc = std::sync::Arc::new(42);
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    println!("{}", arc);
}
```

> **참고:** 동기 채널은 [5장 — 채널](ch05-channels-and-message-passing.md). OS 스레드와 async 태스크는 [6장 — 동시성](ch06-concurrency-vs-parallelism-vs-threads.md).

> **핵심 정리 — Async**
> - `async fn`은 게으른 `Future`를 반환 — `.await`나 spawn 전까지 실행 안 됨
> - async 안의 CPU·블로킹 작업은 `tokio::task::spawn_blocking`
> - `.await`를 넘겨 `std::sync::MutexGuard`를 들고 가지 말 것 — `tokio::sync::Mutex` 사용
> - 스폰된 future는 `Send` — `.await` 지점 전에 `!Send` 타입을 드롭

---

<a id="exercise-concurrent-fetcher-with-timeout"></a>

### 연습: 타임아웃이 있는 동시 fetch ★★ (~25분)

`tokio::spawn`으로 세 태스크를 띄우고, 각각 `tokio::time::sleep`으로 네트워크 호출을 흉내 냅니다.
`tokio::try_join!`을 `tokio::time::timeout(Duration::from_secs(5), ...)`로 감싸고,
`Result<Vec<String>, ...>`를 반환하세요. 태스크 실패나 데드라인 초과 시 에러입니다.

<details>
<summary>🔑 해답</summary>

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

***

