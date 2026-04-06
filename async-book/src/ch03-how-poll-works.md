# 3. How Poll Works 🟡

> **What you'll learn:**
> - The executor's poll loop: poll → pending → wake → poll again
> - How to build a minimal executor from scratch
> - Spurious wake rules and why they matter
> - Utility functions: `poll_fn()` and `yield_now()`

## The Polling State Machine

The executor runs a loop: poll a future, if it's `Pending`, park it until its waker fires, then poll again. This is fundamentally different from OS threads where the kernel handles scheduling.

```mermaid
stateDiagram-v2
    [*] --> Idle : Future created
    Idle --> Polling : executor calls poll()
    Polling --> Complete : Ready(value)
    Polling --> Waiting : Pending
    Waiting --> Polling : waker.wake() called
    Complete --> [*] : Value returned
```

> **Important:** While in the *Waiting* state the future **must** have registered
> the waker with an I/O source. No registration = hang forever.

### A Minimal Executor

To demystify executors, let's build the simplest possible one:

```rust
use std::future::Future;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::pin::Pin;

/// The simplest possible executor: busy-loop poll until Ready
fn block_on<F: Future>(mut future: F) -> F::Output {
    // Pin the future on the stack
    // SAFETY: `future` is never moved after this point — we only
    // access it through the pinned reference until it completes.
    let mut future = unsafe { Pin::new_unchecked(&mut future) };

    // Create a no-op waker (just keeps polling — inefficient but simple)
    fn noop_raw_waker() -> RawWaker {
        fn no_op(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker { noop_raw_waker() }
        let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
        RawWaker::new(std::ptr::null(), vtable)
    }

    // SAFETY: noop_raw_waker() returns a valid RawWaker with a correct vtable.
    let waker = unsafe { Waker::from_raw(noop_raw_waker()) };
    let mut cx = Context::from_waker(&waker);

    // Busy-loop until the future completes
    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(value) => return value,
            Poll::Pending => {
                // A real executor would park the thread here
                // and wait for waker.wake() — we just spin
                std::thread::yield_now();
            }
        }
    }
}

// Usage:
fn main() {
    let result = block_on(async {
        println!("Hello from our mini executor!");
        42
    });
    println!("Got: {result}");
}
```

> **Don't use this in production!** It busy-loops, wasting CPU. Real executors
> (tokio, smol) use `epoll`/`kqueue`/`io_uring` to sleep until I/O is ready.
> But this shows the core idea: an executor is just a loop that calls `poll()`.

### Wake-Up Notifications

A real executor is event-driven. When all futures are `Pending`, the executor sleeps. The waker is an interrupt mechanism:

```rust
// Conceptual model of a real executor's main loop:
fn executor_loop(tasks: &mut TaskQueue) {
    loop {
        // 1. Poll all tasks that have been woken
        while let Some(task) = tasks.get_woken_task() {
            match task.poll() {
                Poll::Ready(result) => task.complete(result),
                Poll::Pending => { /* task stays in queue, waiting for wake */ }
            }
        }

        // 2. Sleep until something wakes us up (epoll_wait, kevent, etc.)
        //    This is where mio/polling does the heavy lifting
        tasks.wait_for_events(); // blocks until an I/O event or waker fires
    }
}
```

### Spurious Wakes

A future may be polled even when its I/O isn't ready. This is called a *spurious wake*. Futures must handle this correctly:

```rust
impl Future for MyFuture {
    type Output = Data;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Data> {
        // ✅ CORRECT: Always re-check the actual condition
        if let Some(data) = self.try_read_data() {
            Poll::Ready(data)
        } else {
            // Re-register the waker (it might have changed!)
            self.register_waker(cx.waker());
            Poll::Pending
        }

        // ❌ WRONG: Assuming poll means data is ready
        // let data = self.read_data(); // might block or panic
        // Poll::Ready(data)
    }
}
```

**Rules for implementing `poll()`**:
1. **Never block** — return `Pending` immediately if not ready
2. **Always re-register the waker** — it may have changed between polls
3. **Handle spurious wakes** — check the actual condition, don't assume readiness
4. **Don't poll after `Ready`** — behavior is **unspecified** (may panic, return `Pending`, or repeat `Ready`). Only `FusedFuture` guarantees safe post-completion polling

<details>
<summary><strong>🏋️ Exercise: Spurious-Wake-Safe Flag Future</strong> (click to expand)</summary>

**Challenge**: Implement a `FlagFuture` that wraps a shared `Arc<AtomicBool>` flag. When polled, it checks whether the flag is `true`. If so, it completes with `Ready(())`. If not, it stores the waker and returns `Pending`. The twist: the future must handle **spurious wakes** correctly — it must re-check the flag on every poll, never assuming the flag is set just because it was woken.

*Hint*: You'll need an `Arc<Mutex<Option<Waker>>>` (or similar) so an external thread can set the flag and wake the future. Use `poll_fn` for a concise alternative solution.

<details>
<summary>🔑 Solution</summary>

```rust
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Context, Poll, Waker};

struct FlagFuture {
    flag: Arc<AtomicBool>,
    waker_slot: Arc<Mutex<Option<Waker>>>,
}

impl FlagFuture {
    fn new(flag: Arc<AtomicBool>, waker_slot: Arc<Mutex<Option<Waker>>>) -> Self {
        FlagFuture { flag, waker_slot }
    }
}

impl Future for FlagFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Always re-check the actual condition — never trust the wake alone
        if self.flag.load(Ordering::Acquire) {
            return Poll::Ready(());
        }

        // Store/update the waker so we get notified
        let mut slot = self.waker_slot.lock().unwrap();
        *slot = Some(cx.waker().clone());

        // Re-check after storing the waker to avoid a race:
        // the flag could have been set between our first check
        // and storing the waker
        if self.flag.load(Ordering::Acquire) {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

// The setter side (e.g., another thread or task):
fn set_flag(flag: &AtomicBool, waker_slot: &Mutex<Option<Waker>>) {
    flag.store(true, Ordering::Release);
    if let Some(waker) = waker_slot.lock().unwrap().take() {
        waker.wake();
    }
}

// Equivalent using poll_fn:
// async fn wait_for_flag(flag: Arc<AtomicBool>, waker_slot: Arc<Mutex<Option<Waker>>>) {
//     std::future::poll_fn(|cx| {
//         if flag.load(Ordering::Acquire) {
//             return Poll::Ready(());
//         }
//         *waker_slot.lock().unwrap() = Some(cx.waker().clone());
//         if flag.load(Ordering::Acquire) { Poll::Ready(()) } else { Poll::Pending }
//     }).await
// }
```

**Key takeaway**: The double-check pattern (check → store waker → check again) is essential to avoid a race between the condition changing and the waker being registered. This is the real-world pattern that all I/O futures use internally, and it demonstrates why handling spurious wakes matters.

</details>
</details>

### Handy Utilities: `poll_fn` and `yield_now`

Two utilities from the standard library and tokio that avoid writing full `Future` impls:

```rust
use std::future::poll_fn;
use std::task::Poll;

// poll_fn: create a one-off future from a closure
let value = poll_fn(|cx| {
    // Do something with cx.waker(), return Ready or Pending
    Poll::Ready(42)
}).await;

// Real-world use: bridge a callback-based API into async
async fn read_when_ready(source: &MySource) -> Data {
    poll_fn(|cx| source.poll_read(cx)).await
}
```

```rust
// yield_now: voluntarily yield control to the executor
// Useful in CPU-heavy async loops to avoid starving other tasks
async fn cpu_heavy_work(items: &[Item]) {
    for (i, item) in items.iter().enumerate() {
        process(item); // CPU work

        // Every 100 items, yield to let other tasks run
        if i % 100 == 0 {
            tokio::task::yield_now().await;
        }
    }
}
```

> **When to use `yield_now()`**: If your async function does CPU work in a loop
> without any `.await` points, it monopolizes the executor thread. Insert
> `yield_now().await` periodically to enable cooperative multitasking.

> **Key Takeaways — How Poll Works**
> - An executor repeatedly calls `poll()` on futures that have been woken
> - Futures must handle **spurious wakes** — always re-check the actual condition
> - `poll_fn()` lets you create ad-hoc futures from closures
> - `yield_now()` is a cooperative scheduling escape hatch for CPU-heavy async code

> **See also:** [Ch 2 — The Future Trait](ch02-the-future-trait.md) for the trait definition, [Ch 5 — The State Machine Reveal](ch05-the-state-machine-reveal.md) for what the compiler generates

***


