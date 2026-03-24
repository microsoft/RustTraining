# Concurrency

## The Concurrency Landscape

| TypeScript | Rust | Model |
|-----------|------|-------|
| Event loop + callbacks | `tokio` / `async-std` | Cooperative async |
| `Promise` / `async`/`await` | `Future` / `async`/`await` | Async I/O |
| `Worker` threads | `std::thread` | OS threads |
| `SharedArrayBuffer` | `Arc<Mutex<T>>` | Shared state |
| `postMessage` | `mpsc::channel` | Message passing |

💡 **Key insight**: TypeScript is single-threaded by design — concurrency means interleaving
I/O, not parallel computation. Rust gives you both: async for I/O concurrency and threads for
CPU parallelism, with compile-time safety guarantees for both.

## Threads

```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("hello from a thread!");
    42
});

let result = handle.join().unwrap(); // waits for thread, gets return value
assert_eq!(result, 42);
```

### Moving Data into Threads

```rust
let data = vec![1, 2, 3];

let handle = thread::spawn(move || {
    // `move` transfers ownership of `data` into this thread
    println!("sum: {}", data.iter().sum::<i32>());
});

// data is no longer accessible here — it was moved
handle.join().unwrap();
```

## Message Passing — Channels

🟦 **TypeScript** (Worker threads):
```typescript
worker.postMessage({ type: "process", data: items });
worker.on("message", (result) => { /* ... */ });
```

🦀 **Rust** (channels):
```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send("hello from thread").unwrap();
    tx.send("another message").unwrap();
});

// Receive messages
for msg in rx {
    println!("received: {msg}");
}
```

`mpsc` stands for *multiple producer, single consumer*. Clone the sender for multiple
producers:

```rust
let (tx, rx) = mpsc::channel();
let tx2 = tx.clone();

thread::spawn(move || tx.send("from thread 1").unwrap());
thread::spawn(move || tx2.send("from thread 2").unwrap());

for msg in rx {
    println!("{msg}");
}
```

## Shared State — `Mutex` and `Arc`

### `Mutex<T>` — Mutual exclusion

```rust
use std::sync::Mutex;

let counter = Mutex::new(0);

{
    let mut num = counter.lock().unwrap();  // acquire lock
    *num += 1;
}   // lock is released when `num` goes out of scope
```

### `Arc<T>` — Atomic Reference Counting

To share data across threads, wrap it in `Arc` (atomic `Rc`):

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("final count: {}", *counter.lock().unwrap()); // 10
```

## `Send` and `Sync` — Compile-Time Thread Safety

Rust prevents data races at compile time using two marker traits:

- **`Send`**: A type can be transferred to another thread.
- **`Sync`**: A type can be referenced from multiple threads.

Most types are both `Send` and `Sync`. Notable exceptions:
- `Rc<T>` is neither (use `Arc<T>` instead).
- `Cell<T>` / `RefCell<T>` are `Send` but not `Sync`.
- Raw pointers are neither.

If you try to send a non-`Send` type to another thread, you get a compile error — not a
runtime data race.

```rust
use std::rc::Rc;

let data = Rc::new(42);
thread::spawn(move || {
    println!("{data}"); // ❌ compile error: Rc<i32> cannot be sent between threads
});
```

## `RwLock` — Multiple Readers, One Writer

When reads are far more common than writes:

```rust
use std::sync::RwLock;

let config = RwLock::new(Config::default());

// Many readers can hold the lock simultaneously
let cfg = config.read().unwrap();

// Only one writer, and it blocks readers
let mut cfg = config.write().unwrap();
cfg.debug = true;
```

🏋️ **Exercise**: Create a program that spawns 4 threads, each generating 1000 random numbers.
Use a channel to send the numbers to the main thread, which collects them into a sorted `Vec`.
