# 5. 채널과 메시지 전달 🟢

> **이 장에서 배울 내용:**
> - `std::sync::mpsc` 기본과 crossbeam-channel으로 올릴 때
> - 여러 소스 메시지를 다루는 `select!` 채널 선택
> - 유한·무한 채널과 백프레셔 전략
> - 동시 상태를 캡슐화하는 액터 패턴

<a id="stdsyncmpsc-the-standard-channel"></a>
## std::sync::mpsc — 표준 채널

Rust 표준 라이브러리는 다중 생산자·단일 소비자 채널을 제공합니다:

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // 채널 생성: tx(송신), rx(수신)
    let (tx, rx) = mpsc::channel();

    // 생산자 스레드
    let tx1 = tx.clone(); // 여러 생산자용 클론
    thread::spawn(move || {
        for i in 0..5 {
            tx1.send(format!("producer-1: msg {i}")).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // 두 번째 생산자
    thread::spawn(move || {
        for i in 0..5 {
            tx.send(format!("producer-2: msg {i}")).unwrap();
            thread::sleep(Duration::from_millis(150));
        }
    });

    // 소비자: 모든 메시지 수신
    for msg in rx {
        // 모든 송신자가 drop되면 rx 이터레이터 종료
        println!("Received: {msg}");
    }
    println!("All producers done.");
}
```

**주요 성질**:
- 기본은 **무한 버퍼**(소비자가 느리면 메모리가 찰 수 있음)
- `mpsc::sync_channel(N)`은 **유한** 채널과 백프레셔를 만듦
- `rx.recv()`는 메시지가 올 때까지 현재 스레드를 블록
- `rx.try_recv()`는 준비된 것이 없으면 즉시 `Err(TryRecvError::Empty)` 반환
- 모든 `Sender`가 drop되면 채널이 닫힘

```rust
// 백프레셔가 있는 유한 채널:
let (tx, rx) = mpsc::sync_channel(10); // 메시지 10개 버퍼

thread::spawn(move || {
    for i in 0..1000 {
        tx.send(i).unwrap(); // 버퍼가 가득 차면 블록 — 자연스러운 백프레셔
    }
});
```

<a id="crossbeam-channel-the-production-workhorse"></a>
### crossbeam-channel — 프로덕션 실무용

`crossbeam-channel`은 프로덕션에서 채널을 쓸 때 사실상 표준입니다. `std::sync::mpsc`보다 빠르고 다중 소비자(`mpmc`)를 지원합니다:

```rust,ignore
// Cargo.toml:
//   [dependencies]
//   crossbeam-channel = "0.5"
use crossbeam_channel::{bounded, unbounded, select, Sender, Receiver};
use std::thread;
use std::time::Duration;

fn main() {
    // 유한 MPMC 채널
    let (tx, rx) = bounded::<String>(100);

    // 여러 생산자
    for id in 0..4 {
        let tx = tx.clone();
        thread::spawn(move || {
            for i in 0..10 {
                tx.send(format!("worker-{id}: item-{i}")).unwrap();
            }
        });
    }
    drop(tx); // 원본 송신자를 drop해야 채널이 닫힐 수 있음

    // 여러 소비자(std::sync::mpsc에서는 불가!)
    let rx2 = rx.clone();
    let consumer1 = thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            println!("[consumer-1] {msg}");
        }
    });
    let consumer2 = thread::spawn(move || {
        while let Ok(msg) = rx2.recv() {
            println!("[consumer-2] {msg}");
        }
    });

    consumer1.join().unwrap();
    consumer2.join().unwrap();
}
```

<a id="channel-selection-select"></a>
### 채널 선택(select!)

여러 채널을 동시에 대기 — Go의 `select`와 같습니다:

```rust,ignore
use crossbeam_channel::{bounded, tick, after, select};
use std::time::Duration;

fn main() {
    let (work_tx, work_rx) = bounded::<String>(10);
    let ticker = tick(Duration::from_secs(1));        // 주기적 틱
    let deadline = after(Duration::from_secs(10));     // One-shot 타임아웃

    // 생산자
    let tx = work_tx.clone();
    std::thread::spawn(move || {
        for i in 0..100 {
            tx.send(format!("job-{i}")).unwrap();
            std::thread::sleep(Duration::from_millis(500));
        }
    });
    drop(work_tx);

    loop {
        select! {
            recv(work_rx) -> msg => {
                match msg {
                    Ok(job) => println!("Processing: {job}"),
                    Err(_) => {
                        println!("Work channel closed");
                        break;
                    }
                }
            },
            recv(ticker) -> _ => {
                println!("Tick — heartbeat");
            },
            recv(deadline) -> _ => {
                println!("Deadline reached — shutting down");
                break;
            },
        }
    }
}
```

> **Go 비교**: Go의 채널 위 `select` 문과 같습니다.
> crossbeam의 `select!`는 기아를 막기 위해 순서를 무작위로 섞습니다(Go와 동일).

<a id="bounded-vs-unbounded-and-backpressure"></a>
### 유한 vs 무한과 백프레셔

| 타입 | 가득 찰 때 동작 | 메모리 | 사용 사례 |
|------|-------------------|--------|----------|
| **무한** | 블록 안 함(힙 증가) | 무한 경고 | 드묾 — 생산자가 소비자보다 느릴 때만 |
| **유한** | `send()`가 공간 날 때까지 블록 | 고정 | 프로덕션 기본 — OOM 방지 |
| **랑데부**(bounded(0)) | `send()`가 수신 준비될 때까지 블록 | 없음 | 동기화 / 직접 전달 |

```rust
// 랑데부 채널 — 용량 0, 직접 전달
let (tx, rx) = crossbeam_channel::bounded(0);
// tx.send(x)는 rx.recv()가 호출될 때까지 블록되고, 그 반대도 마찬가지.
// 두 스레드를 정확히 맞춥니다.
```

**규칙**: 생산자가 소비자보다 빠를 수 없다고 증명할 수 없는 한 프로덕션에서는 항상 유한 채널을 쓰세요.

<a id="actor-pattern-with-channels"></a>
### 채널을 쓰는 액터 패턴

액터 패턴은 가변 상태 접근을 채널로 직렬화합니다 — 뮤텍스가 필요 없을 수 있습니다:

```rust
use std::sync::mpsc;
use std::thread;

// 액터가 받을 수 있는 메시지
enum CounterMsg {
    Increment,
    Decrement,
    Get(mpsc::Sender<i64>), // 응답 채널
}

struct CounterActor {
    count: i64,
    rx: mpsc::Receiver<CounterMsg>,
}

impl CounterActor {
    fn new(rx: mpsc::Receiver<CounterMsg>) -> Self {
        CounterActor { count: 0, rx }
    }

    fn run(mut self) {
        while let Ok(msg) = self.rx.recv() {
            match msg {
                CounterMsg::Increment => self.count += 1,
                CounterMsg::Decrement => self.count -= 1,
                CounterMsg::Get(reply) => {
                    let _ = reply.send(self.count);
                }
            }
        }
    }
}

// 액터 핸들 — 클론이 싸고 Send + Sync
#[derive(Clone)]
struct Counter {
    tx: mpsc::Sender<CounterMsg>,
}

impl Counter {
    fn spawn() -> Self {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || CounterActor::new(rx).run());
        Counter { tx }
    }

    fn increment(&self) { let _ = self.tx.send(CounterMsg::Increment); }
    fn decrement(&self) { let _ = self.tx.send(CounterMsg::Decrement); }

    fn get(&self) -> i64 {
        let (reply_tx, reply_rx) = mpsc::channel();
        self.tx.send(CounterMsg::Get(reply_tx)).unwrap();
        reply_rx.recv().unwrap()
    }
}

fn main() {
    let counter = Counter::spawn();

    // 여러 스레드가 카운터를 안전하게 사용 — 뮤텍스 없음!
    let handles: Vec<_> = (0..10).map(|_| {
        let counter = counter.clone();
        thread::spawn(move || {
            for _ in 0..1000 {
                counter.increment();
            }
        })
    }).collect();

    for h in handles { h.join().unwrap(); }
    println!("Final count: {}", counter.get()); // 10000
}
```

> **액터 vs 뮤텍스**: 상태 불변식이 복잡하거나 연산이 길거나, 락 순서를 생각하지 않고
> 접근을 직렬화하고 싶을 때 액터가 좋습니다. 짧은 임계 구역은 뮤텍스가 더 단순합니다.

> **핵심 정리 — 채널**
> - `crossbeam-channel`이 프로덕션 실무용 — `std::sync::mpsc`보다 빠르고 기능이 많음
> - `select!`가 복잡한 다중 소스 폴링을 선언적 채널 선택으로 바꿈
> - 유한 채널이 자연스러운 백프레셔; 무한 채널은 OOM 위험

> **더 보기:** 스레드, Mutex, 공유 상태는 [6장 — 동시성](ch06-concurrency-vs-parallelism-vs-threads.md). async 채널(`tokio::sync::mpsc`)은 [15장 — Async](ch15-asyncawait-essentials.md).

---

<a id="exercise-channel-based-worker-pool"></a>
### 연습: 채널 기반 워커 풀 ★★★ (~45분)

채널로 워커 풀을 만드세요:
- 디스패처가 `Job` 구조체를 채널로 보냄
- N개 워커가 작업을 소비하고 결과를 돌려보냄
- `std::sync::mpsc`와 `Arc<Mutex<Receiver>>`로 작업 스틸링

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
                            job_id: job.id, output, worker_id,
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

    let results: Vec<_> = result_rx.into_iter().collect();
    assert_eq!(results.len(), num_jobs);

    for h in handles { h.join().unwrap(); }
    results
}

fn main() {
    let jobs: Vec<Job> = (0..20).map(|i| Job {
        id: i, data: format!("task-{i}"),
    }).collect();

    let results = worker_pool(jobs, 4);
    for r in &results {
        println!("[worker {}] job {}: {}", r.worker_id, r.job_id, r.output);
    }
}
```

</details>

***
