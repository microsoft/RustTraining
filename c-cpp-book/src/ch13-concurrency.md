<a id="rust-concurrency"></a>
# Rust 동시성

> **이 장에서 배우는 것:** Rust의 동시성 모델, 즉 스레드, `Send`/`Sync` 마커 트레잇, `Mutex<T>`, `Arc<T>`, 채널, 그리고 컴파일러가 어떻게 데이터 레이스를 컴파일 시점에 막는지를 배웁니다. 사용하지 않는 스레드 안전성에 대한 런타임 오버헤드는 없습니다.

- Rust는 C++의 `std::thread` 와 비슷하게 동시성을 기본 지원합니다
    - 핵심 차이: Rust는 `Send` 와 `Sync` 마커 트레잇을 통해 **데이터 레이스를 컴파일 시점에 방지**합니다
    - C++에서는 뮤텍스 없이 `std::vector` 를 여러 스레드가 공유해도 컴파일은 되지만 UB입니다. Rust에서는 애초에 컴파일되지 않습니다.
    - Rust의 `Mutex<T>` 는 접근만 감싸는 것이 아니라 **데이터 자체**를 감쌉니다. 즉, 잠그지 않고는 그 데이터에 접근할 수 없습니다
- `thread::spawn()` 을 사용하면 클로저 `||` 를 병렬로 실행하는 별도 스레드를 만들 수 있습니다
```rust
use std::thread;
use std::time::Duration;
fn main() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("Count in thread: {i}!");
            thread::sleep(Duration::from_millis(5));
        }
    });

    for i in 0..5 {
        println!("Main thread: {i}");
        thread::sleep(Duration::from_millis(5));
    }

    handle.join().unwrap(); // The handle.join() ensures that the spawned thread exits
}
```

# Rust 동시성
- 환경으로부터 값을 빌려와야 할 때는 ```thread::scope()``` 를 사용할 수 있습니다. ```thread::scope``` 는 내부 스레드가 끝날 때까지 기다리기 때문에 이런 패턴이 가능합니다
- 문제가 무엇인지 보려면 이 예제를 ```thread::scope``` 없이 실행해 보세요
```rust
use std::thread;
fn main() {
  let a = [0, 1, 2];
  thread::scope(|scope| {
      scope.spawn(|| {
          for x in &a {
            println!("{x}");
          }
      });
  });
}
```
----
# Rust 동시성
- ```move``` 를 사용해 소유권을 스레드로 넘길 수도 있습니다. `[i32; 3]` 같은 `Copy` 타입에서는 `move` 키워드가 데이터를 클로저 안으로 복사하며, 원본은 계속 사용할 수 있습니다
```rust
use std::thread;
fn main() {
  let mut a = [0, 1, 2];
  let handle = thread::spawn(move || {
      for x in a {
        println!("{x}");
      }
  });
  a[0] = 42;    // Doesn't affect the copy sent to the thread
  handle.join().unwrap();
}
```

# Rust 동시성
- ```Arc<T>``` 는 여러 스레드 사이에서 *읽기 전용* 참조를 공유할 때 사용할 수 있습니다
    - ```Arc``` 는 Atomic Reference Counted의 약자입니다. 참조 카운트가 0이 될 때까지 값이 해제되지 않습니다
    - ```Arc::clone()``` 는 데이터를 복제하지 않고 참조 카운트만 증가시킵니다
```rust
use std::sync::Arc;
use std::thread;
fn main() {
    let a = Arc::new([0, 1, 2]);
    let mut handles = Vec::new();
    for i in 0..2 {
        let arc = Arc::clone(&a);
        handles.push(thread::spawn(move || {
            println!("Thread: {i} {arc:?}");
        }));
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
}
```

# Rust 동시성
- ```Arc<T>``` 는 ```Mutex<T>``` 와 함께 사용해 가변 참조를 제공할 수 있습니다.
    - ```Mutex``` 는 보호 대상 데이터를 감싸며, 락을 잡은 스레드만 접근할 수 있게 보장합니다.
    - `MutexGuard` 는 스코프를 벗어날 때 자동으로 해제됩니다 (RAII). 참고로 `std::mem::forget` 으로는 guard를 누수시킬 수 있으므로, "누수될 수 없다"보다는 "unlock을 잊기 어렵다"가 더 정확한 표현입니다.
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            // MutexGuard dropped here — lock released automatically
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", *counter.lock().unwrap());
    // Output: Final count: 5
}
```

<a id="rust-concurrency-rwlock"></a>
# Rust 동시성: RwLock
- `RwLock<T>` 는 **여러 동시 읽기** 또는 **하나의 배타적 쓰기** 를 허용합니다. C++의 읽기/쓰기 락 패턴 (`std::shared_mutex`) 에 해당합니다
    - 읽기가 쓰기보다 훨씬 많은 경우 (예: 설정, 캐시)에는 `RwLock` 을 사용하세요
    - 읽기/쓰기 빈도가 비슷하거나 임계 구역이 짧다면 `Mutex` 가 더 적합합니다
```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let config = Arc::new(RwLock::new(String::from("v1.0")));
    let mut handles = Vec::new();

    // Spawn 5 readers — all can run concurrently
    for i in 0..5 {
        let config = Arc::clone(&config);
        handles.push(thread::spawn(move || {
            let val = config.read().unwrap();  // Multiple readers OK
            println!("Reader {i}: {val}");
        }));
    }

    // One writer — blocks until all readers finish
    {
        let config = Arc::clone(&config);
        handles.push(thread::spawn(move || {
            let mut val = config.write().unwrap();  // Exclusive access
            *val = String::from("v2.0");
            println!("Writer: updated to {val}");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
```

<a id="rust-concurrency-mutex-poisoning"></a>
# Rust 동시성: Mutex poisoning
- 어떤 스레드가 `Mutex` 나 `RwLock` 을 잡은 상태에서 **panic** 하면, 그 락은 **poisoned** 상태가 됩니다
    - 이후 `.lock()` 호출은 `Err(PoisonError)` 를 반환합니다. 데이터가 불일치 상태일 수 있기 때문입니다
    - 데이터가 여전히 유효하다고 확신한다면 `.into_inner()` 로 복구할 수 있습니다
    - C++의 `std::mutex` 에는 이런 poisoning 개념이 없으므로 직접적인 대응 개념은 없습니다
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    let data2 = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let mut guard = data2.lock().unwrap();
        guard.push(4);
        panic!("oops!");  // Lock is now poisoned
    });

    let _ = handle.join();  // Thread panicked

    // Subsequent lock attempts return Err(PoisonError)
    match data.lock() {
        Ok(guard) => println!("Data: {guard:?}"),
        Err(poisoned) => {
            println!("Lock was poisoned! Recovering...");
            let guard = poisoned.into_inner();  // Access data anyway
            println!("Recovered data: {guard:?}");  // [1, 2, 3, 4] — push succeeded before panic
        }
    }
}
```

<a id="rust-concurrency-atomics"></a>
# Rust 동시성: Atomics
- 단순한 카운터나 플래그에는 `std::sync::atomic` 타입을 사용하면 `Mutex` 의 오버헤드를 피할 수 있습니다
    - `AtomicBool`, `AtomicI32`, `AtomicU64`, `AtomicUsize` 등이 있습니다
    - C++의 `std::atomic<T>` 에 대응하며, 메모리 순서 모델도 동일합니다 (`Relaxed`, `Acquire`, `Release`, `SeqCst`)
```rust
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(AtomicU64::new(0));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter: {}", counter.load(Ordering::SeqCst));
    // Output: Counter: 10000
}
```

| 프리미티브 | 언제 쓰나 | C++ 대응 |
|-----------|-------------|----------------|
| `Mutex<T>` | 일반적인 가변 공유 상태 | `std::mutex` + 수동 데이터 연결 |
| `RwLock<T>` | 읽기 비중이 큰 작업 부하 | `std::shared_mutex` |
| `Atomic*` | 단순 카운터, 플래그, lock-free 패턴 | `std::atomic<T>` |
| `Condvar` | 어떤 조건이 참이 될 때까지 대기 | `std::condition_variable` |

<a id="rust-concurrency-condvar"></a>
# Rust 동시성: Condvar
- `Condvar` (condition variable) 는 다른 스레드가 조건 변화를 알릴 때까지 스레드를 **잠들게** 할 수 있습니다
    - 항상 `Mutex` 와 함께 사용합니다. 패턴은 보통 "락 획득, 조건 확인, 준비되지 않았으면 대기, 준비되면 동작" 입니다
    - C++의 `std::condition_variable` / `std::condition_variable::wait` 에 대응합니다
    - **spurious wakeup** 을 처리해야 하므로 항상 반복문 안에서 조건을 다시 확인해야 합니다 (또는 `wait_while`/`wait_until` 사용)
```rust
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));

    // Spawn a worker that waits for a signal
    let pair2 = Arc::clone(&pair);
    let worker = thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut ready = lock.lock().unwrap();
        // wait: sleeps until signaled (always re-check in a loop for spurious wakeups)
        while !*ready {
            ready = cvar.wait(ready).unwrap();
        }
        println!("Worker: condition met, proceeding!");
    });

    // Main thread does some work, then signals the worker
    thread::sleep(std::time::Duration::from_millis(100));
    {
        let (lock, cvar) = &*pair;
        let mut ready = lock.lock().unwrap();
        *ready = true;
        cvar.notify_one();  // Wake one waiting thread (notify_all() wakes all)
    }

    worker.join().unwrap();
}
```

> **Condvar와 채널을 언제 쓸까:** 스레드들이 가변 상태를 공유하고, 그 상태에 대한 어떤 조건을 기다려야 할 때 (예: "버퍼가 비어 있지 않음") 는 `Condvar` 를 사용하세요. 스레드 사이에 *메시지* 를 주고받아야 할 때는 채널 (`mpsc`) 이 더 적합합니다. 일반적으로는 채널이 더 이해하기 쉽습니다.

# Rust 동시성
- Rust 채널은 ```Sender``` 와 ```Receiver``` 사이에서 메시지를 주고받는 데 사용할 수 있습니다
    - 이를 ```mpsc``` 또는 ```Multi-producer, Single-Consumer``` 패턴이라고 합니다
    - ```send()``` 와 ```recv()``` 는 모두 스레드를 block할 수 있습니다
```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    tx.send(10).unwrap();
    tx.send(20).unwrap();
    
    println!("Received: {:?}", rx.recv());
    println!("Received: {:?}", rx.recv());

    let tx2 = tx.clone();
    tx2.send(30).unwrap();
    println!("Received: {:?}", rx.recv());
}
```

# Rust 동시성
- 채널은 스레드와 함께 사용할 수 있습니다
```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    for _ in 0..2 {
        let tx2 = tx.clone();
        thread::spawn(move || {
            let thread_id = thread::current().id();
            for i in 0..10 {
                tx2.send(format!("Message {i}")).unwrap();
                println!("{thread_id:?}: sent Message {i}");
            }
            println!("{thread_id:?}: done");
        });
    }

        // Drop the original sender so rx.iter() terminates when all cloned senders are dropped
    drop(tx);

    thread::sleep(Duration::from_millis(100));

    for msg in rx.iter() {
        println!("Main: got {msg}");
    }
}
```



<a id="why-rust-prevents-data-races-send-and-sync"></a>
## Rust가 데이터 레이스를 막는 이유: Send와 Sync

- Rust는 두 개의 마커 트레잇으로 스레드 안전성을 컴파일 시점에 강제합니다:
    - `Send`: 어떤 타입이 다른 스레드로 **이동** 되어도 안전하면 `Send` 입니다
    - `Sync`: 어떤 타입이 여러 스레드 사이에서 (`&T` 를 통해) **공유** 되어도 안전하면 `Sync` 입니다
- 대부분의 타입은 자동으로 `Send + Sync` 가 됩니다. 대표적인 예외는 다음과 같습니다:
    - `Rc<T>` 는 `Send` 도 `Sync` 도 아닙니다 (스레드에서는 `Arc<T>` 사용)
    - `Cell<T>` 와 `RefCell<T>` 는 `Sync` 가 아닙니다 (`Mutex<T>` 나 `RwLock<T>` 사용)
    - raw pointer (`*const T`, `*mut T`) 는 `Send` 도 `Sync` 도 아닙니다
- 그래서 컴파일러가 `Rc<T>` 를 스레드 사이에서 쓰지 못하게 막습니다. 실제로 `Send` 를 구현하지 않기 때문입니다
- `Arc<Mutex<T>>` 는 `Rc<RefCell<T>>` 의 스레드 안전 버전이라고 볼 수 있습니다

> **직관적으로 이해하기** *(Jon Gjengset)*: 값을 장난감이라고 생각해 봅시다.
> **`Send`** = 내 장난감을 다른 아이 (스레드) 에게 **건네줘도 된다**. 즉, 소유권 이전이 안전하다.
> **`Sync`** = 다른 아이들이 내 장난감을 **동시에 가지고 놀아도 된다**. 즉, 참조 공유가 안전하다.
> `Rc<T>` 는 약한 (원자적이지 않은) 참조 카운터를 가지므로, 넘겨주거나 공유하면 카운트가 망가질 수 있습니다. 그래서 `Send` 도 `Sync` 도 아닙니다.


<a id="exercise-multi-threaded-word-count"></a>
# 연습문제: 멀티스레드 단어 수 세기

🔴 **Challenge** — 스레드, Arc, Mutex, HashMap을 함께 사용하는 문제

- 텍스트 줄들이 담긴 `Vec<String>` 이 주어졌을 때, 각 줄마다 하나의 스레드를 생성해 그 줄의 단어 수를 세어 보세요
- 결과를 모으는 데 `Arc<Mutex<HashMap<String, usize>>>` 를 사용하세요
- 모든 줄에 대한 총 단어 수를 출력하세요
- **Bonus**: 공유 상태 대신 채널 (`mpsc`) 로도 구현해 보세요

<details><summary>해답 (클릭하여 펼치기)</summary>

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let lines = vec![
        "the quick brown fox".to_string(),
        "jumps over the lazy dog".to_string(),
        "the fox is quick".to_string(),
    ];

    let word_counts: Arc<Mutex<HashMap<String, usize>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let mut handles = vec![];
    for line in &lines {
        let line = line.clone();
        let counts = Arc::clone(&word_counts);
        handles.push(thread::spawn(move || {
            for word in line.split_whitespace() {
                let mut map = counts.lock().unwrap();
                *map.entry(word.to_lowercase()).or_insert(0) += 1;
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let counts = word_counts.lock().unwrap();
    let total: usize = counts.values().sum();
    println!("Word frequencies: {counts:#?}");
    println!("Total words: {total}");
}
// Output (order may vary):
// Word frequencies: {
//     "the": 3,
//     "quick": 2,
//     "brown": 1,
//     "fox": 2,
//     "jumps": 1,
//     "over": 1,
//     "lazy": 1,
//     "dog": 1,
//     "is": 1,
// }
// Total words: 13
```

</details>


