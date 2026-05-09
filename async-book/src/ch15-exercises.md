<a id="exercises"></a>
## 연습문제

<a id="exercise-1-async-echo-server"></a>
### 연습문제 1: Async 에코 서버

여러 클라이언트를 동시에 처리하는 TCP 에코 서버를 만들어 보세요.

**요구 사항**:
- `127.0.0.1:8080`에서 리슨하기
- 연결을 받아 각 줄을 그대로 다시 돌려주기
- 클라이언트 연결 종료를 우아하게 처리하기
- 클라이언트 연결/해제 시 로그 출력하기

<details>
<summary>해답 (클릭하여 펼치기)</summary>

```rust
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Echo server listening on :8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("[{addr}] Connected");

        tokio::spawn(async move {
            let (reader, mut writer) = socket.into_split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(0) => {
                        println!("[{addr}] Disconnected");
                        break;
                    }
                    Ok(_) => {
                        print!("[{addr}] Echo: {line}");
                        if writer.write_all(line.as_bytes()).await.is_err() {
                            println!("[{addr}] Write error, disconnecting");
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("[{addr}] Read error: {e}");
                        break;
                    }
                }
            }
        });
    }
}
```

</details>

---

<a id="exercise-2-concurrent-url-fetcher-with-rate-limiting"></a>
### 연습문제 2: 속도 제한이 있는 동시 URL 가져오기

URL 목록을 동시에 가져오되, 동시에 최대 5개의 요청만 처리하도록 만들어 보세요.

<details>
<summary>해답 (클릭하여 펼치기)</summary>

```rust
use futures::stream::{self, StreamExt};
use tokio::time::{sleep, Duration};

async fn fetch_urls(urls: Vec<String>) -> Vec<Result<String, String>> {
    // buffer_unordered(5)는 동시에 최대 5개의 future만 poll되도록 보장한다.
    // 이 경우 별도의 Semaphore는 필요 없다.
    let results: Vec<_> = stream::iter(urls)
        .map(|url| {
            async move {
                println!("Fetching: {url}");

                match reqwest::get(&url).await {
                    Ok(resp) => match resp.text().await {
                        Ok(body) => Ok(body),
                        Err(e) => Err(format!("{url}: {e}")),
                    },
                    Err(e) => Err(format!("{url}: {e}")),
                }
            }
        })
        .buffer_unordered(5) // 이것만으로 동시성 5개 제한이 걸린다
        .collect()
        .await;

    results
}

// 참고: 서로 독립적으로 스폰된 태스크(tokio::spawn)들 사이의 동시성을
// 제한해야 할 때는 Semaphore를 사용하라. stream을 처리할 때는
// buffer_unordered를 사용하라. 같은 제한에 둘을 함께 쓰지 마라.
```

</details>

---

<a id="exercise-3-graceful-shutdown-with-worker-pool"></a>
### 연습문제 3: 워커 풀과 우아한 종료

다음을 갖춘 작업 처리기를 만들어 보세요.
- 채널 기반 작업 큐
- 큐를 소비하는 N개의 워커 태스크
- Ctrl+C 시 우아한 종료: 새 작업 수락 중단, 진행 중 작업 마무리

<details>
<summary>해답 (클릭하여 펼치기)</summary>

```rust
use tokio::sync::{mpsc, watch};
use tokio::time::{sleep, Duration};

struct WorkItem {
    id: u64,
    payload: String,
}

#[tokio::main]
async fn main() {
    let (work_tx, work_rx) = mpsc::channel::<WorkItem>(100);
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    // 워커 4개 스폰
    let mut worker_handles = Vec::new();
    let work_rx = std::sync::Arc::new(tokio::sync::Mutex::new(work_rx));

    for id in 0..4 {
        let rx = work_rx.clone();
        let mut shutdown = shutdown_rx.clone();
        let handle = tokio::spawn(async move {
            loop {
                let item = {
                    let mut rx = rx.lock().await;
                    tokio::select! {
                        item = rx.recv() => item,
                        _ = shutdown.changed() => {
                            if *shutdown.borrow() { None } else { continue }
                        }
                    }
                };

                match item {
                    Some(work) => {
                        println!("Worker {id}: processing item {}", work.id);
                        sleep(Duration::from_millis(200)).await; // 작업 시뮬레이션
                        println!("Worker {id}: done with item {}", work.id);
                    }
                    None => {
                        println!("Worker {id}: channel closed, exiting");
                        break;
                    }
                }
            }
        });
        worker_handles.push(handle);
    }

    // producer: 작업 몇 개를 투입
    let producer = tokio::spawn(async move {
        for i in 0..20 {
            let _ = work_tx.send(WorkItem {
                id: i,
                payload: format!("task-{i}"),
            }).await;
            sleep(Duration::from_millis(50)).await;
        }
    });

    // Ctrl+C 대기
    tokio::signal::ctrl_c().await.unwrap();
    println!("\nShutdown signal received!");
    shutdown_tx.send(true).unwrap();
    producer.abort(); // producer 태스크 취소

    // 워커들이 끝날 때까지 대기
    for handle in worker_handles {
        let _ = handle.await;
    }
    println!("All workers shut down. Goodbye!");
}
```

</details>

---

<a id="exercise-4-build-a-simple-async-mutex-from-scratch"></a>
### 연습문제 4: 간단한 Async Mutex를 처음부터 구현하기

채널을 사용해(`tokio::sync::Mutex` 없이) async를 인지하는 mutex를 구현해 보세요.

**힌트:** 용량 1짜리 `tokio::sync::mpsc` 채널을 세마포어처럼 써 보는 발상에서 출발해 보세요.

<details>
<summary>해답 (클릭하여 펼치기)</summary>

```rust
use std::cell::UnsafeCell;
use std::sync::Arc;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

pub struct SimpleAsyncMutex<T> {
    data: Arc<UnsafeCell<T>>,
    semaphore: Arc<Semaphore>,
}

// SAFETY: T에 대한 접근은 세마포어(최대 permit 1개)로 직렬화된다.
unsafe impl<T: Send> Send for SimpleAsyncMutex<T> {}
unsafe impl<T: Send> Sync for SimpleAsyncMutex<T> {}

pub struct SimpleGuard<T> {
    data: Arc<UnsafeCell<T>>,
    _permit: OwnedSemaphorePermit, // guard가 drop되면 lock이 해제된다
}

impl<T> SimpleAsyncMutex<T> {
    pub fn new(value: T) -> Self {
        SimpleAsyncMutex {
            data: Arc::new(UnsafeCell::new(value)),
            semaphore: Arc::new(Semaphore::new(1)),
        }
    }

    pub async fn lock(&self) -> SimpleGuard<T> {
        let permit = self.semaphore.clone().acquire_owned().await.unwrap();
        SimpleGuard {
            data: self.data.clone(),
            _permit: permit,
        }
    }
}

impl<T> std::ops::Deref for SimpleGuard<T> {
    type Target = T;
    fn deref(&self) -> &T {
        // SAFETY: 유일한 semaphore permit을 들고 있으므로
        // 다른 SimpleGuard는 존재할 수 없고, 배타적 접근이 보장된다.
        unsafe { &*self.data.get() }
    }
}

impl<T> std::ops::DerefMut for SimpleGuard<T> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: 같은 이유로 단일 permit이 배타성을 보장한다.
        unsafe { &mut *self.data.get() }
    }
}

// SimpleGuard가 drop되면 _permit도 drop되고,
// 그러면 semaphore permit이 반환되어 다음 lock()이 진행될 수 있다.

// 사용 예:
// let mutex = SimpleAsyncMutex::new(vec![1, 2, 3]);
// {
//     let mut guard = mutex.lock().await;
//     guard.push(4);
// } // 여기서 permit 해제
```

**핵심 포인트:** async mutex는 대개 세마포어 위에 구축됩니다. 세마포어가 async 대기 메커니즘을 제공합니다. 잠겨 있는 동안 `acquire()`는 permit이 반환될 때까지 태스크를 일시 중단합니다. `tokio::sync::Mutex`도 내부적으로 같은 아이디어를 사용합니다.

> **왜 `UnsafeCell`이고 `std::sync::Mutex`가 아닐까?** 이 연습문제의 이전 버전은 `Arc<Mutex<T>>`에 `Deref`/`DerefMut`에서 `.lock().unwrap()`을 호출하는 방식이었습니다. 하지만 그 코드는 컴파일되지 않습니다. 반환되는 `&T`가 즉시 drop되는 임시 `MutexGuard`를 빌리기 때문입니다. `UnsafeCell`은 그 중간 guard를 없애 주고, 세마포어 기반 직렬화 덕분에 이 `unsafe`는 타당해집니다.

</details>

---

<a id="exercise-5-stream-pipeline"></a>
### 연습문제 5: 스트림 파이프라인

stream을 사용해 데이터 처리 파이프라인을 만들어 보세요.
1. `1..=100` 숫자 생성
2. 짝수만 필터링
3. 각 값을 제곱
4. 한 번에 10개씩 동시에 처리(`sleep`으로 시뮬레이션)
5. 결과 수집

<details>
<summary>해답 (클릭하여 펼치기)</summary>

```rust
use futures::stream::{self, StreamExt};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let results: Vec<u64> = stream::iter(1u64..=100)
        // 2단계: 짝수 필터링
        .filter(|x| futures::future::ready(x % 2 == 0))
        // 3단계: 각 값 제곱
        .map(|x| x * x)
        // 4단계: 동시에 처리(async 작업 시뮬레이션)
        .map(|x| async move {
            sleep(Duration::from_millis(50)).await;
            println!("Processed: {x}");
            x
        })
        .buffer_unordered(10) // 동시성 10개
        // 5단계: 결과 수집
        .collect()
        .await;

    println!("Got {} results", results.len());
    println!("Sum: {}", results.iter().sum::<u64>());
}
```

</details>

---

<a id="exercise-6-implement-select-with-timeout"></a>
### 연습문제 6: 타임아웃이 있는 Select 구현하기

`tokio::select!`나 `tokio::time::timeout`을 쓰지 말고, 하나의 future와 마감 시각을 경쟁시켜 타임아웃 시 `Either::Right(())`, 그렇지 않으면 `Either::Left(result)`를 반환하는 함수를 구현해 보세요.

**힌트:** 6장의 `Select` combinator와 같은 장의 `TimerFuture`를 바탕으로 만들어 보세요.

<details>
<summary>해답 (클릭하여 펼치기)</summary>

```rust,ignore
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

pub enum Either<A, B> {
    Left(A),
    Right(B),
}

pub struct Timeout<F> {
    future: F,
    timer: TimerFuture, // 6장에서 만든 타입
}

impl<F: Future + Unpin> Timeout<F> {
    pub fn new(future: F, duration: Duration) -> Self {
        Timeout {
            future,
            timer: TimerFuture::new(duration),
        }
    }
}

impl<F: Future + Unpin> Future for Timeout<F> {
    type Output = Either<F::Output, ()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 메인 future가 끝났는지 확인
        if let Poll::Ready(val) = Pin::new(&mut self.future).poll(cx) {
            return Poll::Ready(Either::Left(val));
        }

        // 타이머가 만료됐는지 확인
        if let Poll::Ready(()) = Pin::new(&mut self.timer).poll(cx) {
            return Poll::Ready(Either::Right(()));
        }

        Poll::Pending
    }
}

// 사용 예:
// match Timeout::new(fetch_data(), Duration::from_secs(5)).await {
//     Either::Left(data) => println!("Got data: {data}"),
//     Either::Right(()) => println!("Timed out!"),
// }
```

**핵심 포인트:** `select`/`timeout`은 결국 두 future를 poll해서 어느 쪽이 먼저 끝나는지 보는 일입니다. async 생태계 전체는 이 단순한 원시 구성 요소, 즉 poll, `Pending`/`Ready`, `Waker` 위에 세워집니다.

</details>

***