## Thread Safety: Convention vs Type System Guarantees

> **What you'll learn:** How Rust enforces thread safety at compile time vs C#'s convention-based approach,
> `Arc<Mutex<T>>` vs `lock`, channels vs `ConcurrentQueue`, `Send`/`Sync` traits,
> scoped threads, and the bridge to async/await.
>
> **Difficulty:** 🔴 Advanced

> **Deep dive**: For production async patterns (stream processing, graceful shutdown, connection pooling, cancellation safety), see the companion [Async Rust Training](../../async-book/src/summary.md) guide.
>
> **Prerequisites**: [Ownership & Borrowing](ch07-ownership-and-borrowing.md) and [Smart Pointers](ch07-3-smart-pointers-beyond-single-ownership.md) (Rc vs Arc decision tree).

### C# - Thread Safety by Convention
```csharp
// C# collections aren't thread-safe by default
public class UserService
{
    private readonly List<string> items = new();
    private readonly Dictionary<int, User> cache = new();

    // This can cause data races:
    public void AddItem(string item)
    {
        items.Add(item);  // Not thread-safe!
    }

    // Must use locks manually:
    private readonly object lockObject = new();

    public void SafeAddItem(string item)
    {
        lock (lockObject)
        {
            items.Add(item);  // Safe, but runtime overhead
        }
        // Easy to forget the lock elsewhere
    }

    // ConcurrentCollection helps but limited:
    private readonly ConcurrentBag<string> safeItems = new();
    
    public void ConcurrentAdd(string item)
    {
        safeItems.Add(item);  // Thread-safe but limited operations
    }

    // Complex shared state management
    private readonly ConcurrentDictionary<int, User> threadSafeCache = new();
    private volatile bool isShutdown = false;
    
    public async Task ProcessUser(int userId)
    {
        if (isShutdown) return;  // Race condition possible!
        
        var user = await GetUser(userId);
        threadSafeCache.TryAdd(userId, user);  // Must remember which collections are safe
    }

    // Thread-local storage requires careful management
    private static readonly ThreadLocal<Random> threadLocalRandom = 
        new ThreadLocal<Random>(() => new Random());
        
    public int GetRandomNumber()
    {
        return threadLocalRandom.Value.Next();  // Safe but manual management
    }
}

// Event handling with potential race conditions
public class EventProcessor
{
    public event Action<string> DataReceived;
    private readonly List<string> eventLog = new();
    
    public void OnDataReceived(string data)
    {
        // Race condition - event might be null between check and invocation
        if (DataReceived != null)
        {
            DataReceived(data);
        }
        // Modern C# (6+) mitigates the null race with: DataReceived?.Invoke(data);
        // but the underlying event-delegate model still allows races on the list below
        
        // Another race condition - list not thread-safe
        eventLog.Add($"Processed: {data}");
    }
}
```

### Rust - Thread Safety Guaranteed by Type System
```rust
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::collections::HashMap;
use tokio::sync::{mpsc, broadcast};

// Rust prevents data races at compile time
pub struct UserService {
    items: Arc<Mutex<Vec<String>>>,
    cache: Arc<RwLock<HashMap<i32, User>>>,
}

impl UserService {
    pub fn new() -> Self {
        UserService {
            items: Arc::new(Mutex::new(Vec::new())),
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub fn add_item(&self, item: String) {
        let mut items = self.items.lock().unwrap();
        items.push(item);
        // Lock automatically released when `items` goes out of scope
    }
    
    // Multiple readers, single writer - automatically enforced
    pub async fn get_user(&self, user_id: i32) -> Option<User> {
        let cache = self.cache.read().unwrap();
        cache.get(&user_id).cloned()
    }
    
    pub async fn cache_user(&self, user_id: i32, user: User) {
        let mut cache = self.cache.write().unwrap();
        cache.insert(user_id, user);
    }
    
    // Clone the Arc for thread sharing
    pub fn process_in_background(&self) {
        let items = Arc::clone(&self.items);
        
        thread::spawn(move || {
            let items = items.lock().unwrap();
            for item in items.iter() {
                println!("Processing: {}", item);
            }
        });
    }
}

// Channel-based communication - no shared state needed
pub struct MessageProcessor {
    sender: mpsc::UnboundedSender<String>,
}

impl MessageProcessor {
    pub fn new() -> (Self, mpsc::UnboundedReceiver<String>) {
        let (tx, rx) = mpsc::unbounded_channel();
        (MessageProcessor { sender: tx }, rx)
    }
    
    pub fn send_message(&self, message: String) -> Result<(), mpsc::error::SendError<String>> {
        self.sender.send(message)
    }
}

// This won't compile - Rust prevents sharing mutable data unsafely:
fn impossible_data_race() {
    let mut items = vec![1, 2, 3];
    
    // This won't compile - cannot move `items` into multiple closures
    /*
    thread::spawn(move || {
        items.push(4);  // ERROR: use of moved value
    });
    
    thread::spawn(move || {
        items.push(5);  // ERROR: use of moved value  
    });
    */
}

// Safe concurrent data processing
use rayon::prelude::*;

fn parallel_processing() {
    let data = vec![1, 2, 3, 4, 5];
    
    // Parallel iteration - guaranteed thread-safe
    let results: Vec<i32> = data
        .par_iter()
        .map(|&x| x * x)
        .collect();
        
    println!("{:?}", results);
}

// Async concurrency with message passing
async fn async_message_passing() {
    let (tx, mut rx) = mpsc::channel(100);
    
    // Producer task
    let producer = tokio::spawn(async move {
        for i in 0..10 {
            if tx.send(i).await.is_err() {
                break;
            }
        }
    });
    
    // Consumer task  
    let consumer = tokio::spawn(async move {
        while let Some(value) = rx.recv().await {
            println!("Received: {}", value);
        }
    });
    
    // Wait for both tasks
    let (producer_result, consumer_result) = tokio::join!(producer, consumer);
    producer_result.unwrap();
    consumer_result.unwrap();
}

#[derive(Clone)]
struct User {
    id: i32,
    name: String,
}
```

```mermaid
graph TD
    subgraph "C# Thread Safety Challenges"
        CS_MANUAL["Manual synchronization"]
        CS_LOCKS["lock statements"]
        CS_CONCURRENT["ConcurrentCollections"]
        CS_VOLATILE["volatile fields"]
        CS_FORGET["😰 Easy to forget locks"]
        CS_DEADLOCK["💀 Deadlock possible"]
        CS_RACE["🏃 Race conditions"]
        CS_OVERHEAD["⚡ Runtime overhead"]
        
        CS_MANUAL --> CS_LOCKS
        CS_MANUAL --> CS_CONCURRENT
        CS_MANUAL --> CS_VOLATILE
        CS_LOCKS --> CS_FORGET
        CS_LOCKS --> CS_DEADLOCK
        CS_FORGET --> CS_RACE
        CS_LOCKS --> CS_OVERHEAD
    end
    
    subgraph "Rust Type System Guarantees"
        RUST_OWNERSHIP["Ownership system"]
        RUST_BORROWING["Borrow checker"]
        RUST_SEND["Send trait"]
        RUST_SYNC["Sync trait"]
        RUST_ARC["Arc<Mutex<T>>"]
        RUST_CHANNELS["Message passing"]
        RUST_SAFE["✅ Data races impossible"]
        RUST_FAST["⚡ Zero-cost abstractions"]
        
        RUST_OWNERSHIP --> RUST_BORROWING
        RUST_BORROWING --> RUST_SEND
        RUST_SEND --> RUST_SYNC
        RUST_SYNC --> RUST_ARC
        RUST_ARC --> RUST_CHANNELS
        RUST_CHANNELS --> RUST_SAFE
        RUST_SAFE --> RUST_FAST
    end
    
    style CS_FORGET fill:#ffcdd2,color:#000
    style CS_DEADLOCK fill:#ffcdd2,color:#000
    style CS_RACE fill:#ffcdd2,color:#000
    style RUST_SAFE fill:#c8e6c9,color:#000
    style RUST_FAST fill:#c8e6c9,color:#000
```

***


<details>
<summary><strong>🏋️ Exercise: Thread-Safe Counter</strong> (click to expand)</summary>

**Challenge**: Implement a thread-safe counter that can be incremented from 10 threads simultaneously. Each thread increments 1000 times. The final count should be exactly 10,000.

<details>
<summary>🔑 Solution</summary>

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0u64));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                let mut count = counter.lock().unwrap();
                *count += 1;
            }
        }));
    }

    for h in handles { h.join().unwrap(); }
    assert_eq!(*counter.lock().unwrap(), 10_000);
    println!("Final count: {}", counter.lock().unwrap());
}
```

**Or with atomics (faster, no locking):**
```rust
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(AtomicU64::new(0));
    let handles: Vec<_> = (0..10).map(|_| {
        let counter = Arc::clone(&counter);
        thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        })
    }).collect();

    for h in handles { h.join().unwrap(); }
    assert_eq!(counter.load(Ordering::SeqCst), 10_000);
}
```

**Key takeaway**: `Arc<Mutex<T>>` is the general pattern. For simple counters, `AtomicU64` avoids lock overhead entirely.

</details>
</details>

### Why Rust prevents data races: Send and Sync

Rust uses two marker traits to enforce thread safety **at compile time** — there is no C# equivalent:

- `Send`: A type can be safely **transferred** to another thread (e.g., moved into a closure passed to `thread::spawn`)
- `Sync`: A type can be safely **shared** (via `&T`) between threads

Most types are automatically `Send + Sync`. Notable exceptions:
- `Rc<T>` is **neither** Send nor Sync — the compiler will refuse to let you pass it to `thread::spawn` (use `Arc<T>` instead)
- `Cell<T>` and `RefCell<T>` are **not** Sync — use `Mutex<T>` or `RwLock<T>` for thread-safe interior mutability
- Raw pointers (`*const T`, `*mut T`) are **neither** Send nor Sync

In C#, `List<T>` is not thread-safe but the compiler won't stop you from sharing it across threads. In Rust, the equivalent mistake is a **compile error**, not a runtime race condition.

### Scoped threads: borrowing from the stack

`thread::scope()` lets spawned threads borrow local variables — no `Arc` needed:

```rust
use std::thread;

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    
    // Scoped threads can borrow 'data' — scope waits for all threads to finish
    thread::scope(|s| {
        s.spawn(|| println!("Thread 1: {data:?}"));
        s.spawn(|| println!("Thread 2: sum = {}", data.iter().sum::<i32>()));
    });
    // 'data' is still valid here — threads are guaranteed to have finished
}
```

This is similar to C#'s `Parallel.ForEach` in that the calling code waits for completion, but Rust's borrow checker **proves** there are no data races at compile time.

### Bridging to async/await

C# developers typically reach for `Task` and `async/await` rather than raw threads. Rust has both paradigms:

| C# | Rust | When to use |
|----|------|-------------|
| `Thread` | `std::thread::spawn` | CPU-bound work, OS thread per task |
| `Task.Run` | `tokio::spawn` | Async task on a runtime |
| `async/await` | `async/await` | I/O-bound concurrency |
| `lock` | `Mutex<T>` | Sync mutual exclusion |
| `SemaphoreSlim` | `tokio::sync::Semaphore` | Async concurrency limiting |
| `Interlocked` | `std::sync::atomic` | Lock-free atomic operations |
| `CancellationToken` | `tokio_util::sync::CancellationToken` | Cooperative cancellation |

> The next chapter ([Async/Await Deep Dive](ch13-1-asyncawait-deep-dive.md)) covers Rust's async model in detail — including how it differs from C#'s `Task`-based model.

