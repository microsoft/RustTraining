# Async Rust: From Futures to Production

## Speaker Intro

- Principal Firmware Architect in Microsoft SCHIE (Silicon and Cloud Hardware Infrastructure Engineering) team
- Industry veteran with expertise in security, systems programming (firmware, operating systems, hypervisors), CPU and platform architecture, and C++ systems
- Started programming in Rust in 2017 (@AWS EC2), and have been in love with the language ever since

---

A deep-dive guide to asynchronous programming in Rust. Unlike most async tutorials that start with `tokio::main` and hand-wave the internals, this guide builds understanding from first principles — the `Future` trait, polling, state machines — then progresses to real-world patterns, runtime selection, and production pitfalls.

## Who This Is For
- Rust developers who can write synchronous Rust but find async confusing
- Developers from C#, Go, Python, or JavaScript who know `async/await` but not Rust's model
- Anyone who's been bitten by `Future is not Send`, `Pin<Box<dyn Future>>`, or "why does my program hang?"

## Prerequisites

You should be comfortable with:
- Ownership, borrowing, and lifetimes
- Traits and generics (including `impl Trait`)
- Using `Result<T, E>` and the `?` operator
- Basic multi-threading (`std::thread::spawn`, `Arc`, `Mutex`)

No prior async Rust experience is needed.

## How to Use This Book

**Read linearly the first time.** Parts I–III build on each other. Each chapter has:

| Symbol | Meaning |
|--------|---------|
| 🟢 | Beginner — foundational concept |
| 🟡 | Intermediate — requires earlier chapters |
| 🔴 | Advanced — deep internals or production patterns |

Each chapter includes:
- A **"What you'll learn"** block at the top
- **Mermaid diagrams** for visual learners
- An **inline exercise** with a hidden solution
- **Key Takeaways** summarizing the core ideas
- **Cross-references** to related chapters

## Pacing Guide

| Chapters | Topic | Suggested Time | Checkpoint |
|----------|-------|----------------|------------|
| 1–5 | How Async Works | 6–8 hours | You can explain `Future`, `Poll`, `Pin`, and why Rust has no built-in runtime |
| 6–10 | The Ecosystem | 6–8 hours | You can build futures by hand, choose a runtime, and use tokio's API |
| 11–13 | Production Async | 6–8 hours | You can write production-grade async code with streams, proper error handling, and graceful shutdown |
| Capstone | Chat Server | 4–6 hours | You've built a real async application integrating all concepts |

**Total estimated time: 22–30 hours**

## Working Through Exercises

Every content chapter has an inline exercise. The capstone (Ch 16) integrates everything into a single project. For maximum learning:

1. **Try the exercise before expanding the solution** — struggling is where learning happens
2. **Type the code, don't copy-paste** — muscle memory matters for Rust's syntax
3. **Run every example** — `cargo new async-exercises` and test as you go

## Table of Contents

### Part I: How Async Works

- [1. Why Async is Different in Rust](ch01-why-async-is-different-in-rust.md) 🟢 — The fundamental difference: Rust has no built-in runtime
- [2. The Future Trait](ch02-the-future-trait.md) 🟡 — `poll()`, `Waker`, and the contract that makes it all work
- [3. How Poll Works](ch03-how-poll-works.md) 🟡 — The polling state machine and a minimal executor
- [4. Pin and Unpin](ch04-pin-and-unpin.md) 🔴 — Why self-referential structs need pinning
- [5. The State Machine Reveal](ch05-the-state-machine-reveal.md) 🟢 — What the compiler actually generates from `async fn`

### Part II: The Ecosystem

- [6. Building Futures by Hand](ch06-building-futures-by-hand.md) 🟡 — TimerFuture, Join, Select from scratch
- [7. Executors and Runtimes](ch07-executors-and-runtimes.md) 🟡 — tokio, smol, async-std, embassy — how to choose
- [8. Tokio Deep Dive](ch08-tokio-deep-dive.md) 🟡 — Runtime flavors, spawn, channels, sync primitives
- [9. When Tokio Isn't the Right Fit](ch09-when-tokio-isnt-the-right-fit.md) 🟡 — LocalSet, FuturesUnordered, runtime-agnostic design
- [10. Async Traits](ch10-async-traits.md) 🟡 — RPITIT, dyn dispatch, trait_variant, async closures

### Part III: Production Async

- [11. Streams and AsyncIterator](ch11-streams-and-asynciterator.md) 🟡 — Async iteration, AsyncRead/Write, stream combinators
- [12. Common Pitfalls](ch12-common-pitfalls.md) 🔴 — 9 production bugs and how to avoid them
- [13. Production Patterns](ch13-production-patterns.md) 🔴 — Graceful shutdown, backpressure, Tower middleware
- [14. Async Is an Optimization, Not an Architecture](ch14-async-is-an-optimization-not-an-architecture.md) 🔴 — Sync core / async shell, the function coloring tax

### Appendices

- [Summary and Reference Card](ch16-summary-and-reference-card.md) — Quick-lookup tables and decision trees
- [Capstone Project: Async Chat Server](ch17-capstone-project.md) — Build a complete async application

***


