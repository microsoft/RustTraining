# Type-Driven Correctness in Rust

## Speaker Intro

- Principal Firmware Architect in Microsoft SCHIE (Silicon and Cloud Hardware Infrastructure Engineering) team
- Industry veteran with expertise in security, systems programming (firmware, operating systems, hypervisors), CPU and platform architecture, and C++ systems
- Started programming in Rust in 2017 (@AWS EC2), and have been in love with the language ever since

---

A practical guide to using Rust's type system to make entire classes of bugs **impossible to compile**. While the companion [Rust Patterns](../rust-patterns-book/src/SUMMARY.md) book covers the mechanics (traits, associated types, type-state), this guide shows how to **apply** those mechanics to real-world domains — hardware diagnostics, cryptography, protocol validation, and embedded systems.

Every pattern here follows one principle: **push invariants from runtime checks into the type system so the compiler enforces them.**

## How to Use This Book

### Difficulty Legend

| Symbol | Level | Audience |
|:------:|-------|----------|
| 🟢 | Introductory | Comfortable with ownership + traits |
| 🟡 | Intermediate | Familiar with generics + associated types |
| 🔴 | Advanced | Ready for type-state, phantom types, and session types |

### Pacing Guide

| Goal | Path | Time |
|------|------|------|
| **Quick overview** | ch01, ch13 (reference card) | 30 min |
| **IPMI / BMC developer** | ch02, ch05, ch07, ch10, ch17 | 2.5 hrs |
| **GPU / PCIe developer** | ch02, ch06, ch09, ch10, ch15 | 2.5 hrs |
| **Redfish implementer** | ch02, ch05, ch07, ch08, ch17, ch18 | 3 hrs |
| **Framework / infrastructure** | ch04, ch08, ch11, ch14, ch18 | 2.5 hrs |
| **New to correct-by-construction** | ch01 → ch10 in order, then ch12 exercises | 4 hrs |
| **Full deep dive** | All chapters sequentially | 7 hrs |

### Annotated Table of Contents

| Ch | Title | Difficulty | Key Idea |
|----|-------|:----------:|----------|
| 1 | The Philosophy — Why Types Beat Tests | 🟢 | Three levels of correctness; Curry-Howard intuition |
| 2 | Typed Command Interfaces | 🟡 | Associated types bind request → response |
| 3 | Single-Use Types | 🟡 | Move semantics as linear types for crypto |
| 4 | Capability Tokens | 🟡 | Zero-sized proof-of-authority tokens |
| 5 | Protocol State Machines | 🔴 | Type-state for IPMI sessions + PCIe LTSSM |
| 6 | Dimensional Analysis | 🟢 | Newtype wrappers prevent unit mix-ups |
| 7 | Validated Boundaries | 🟡 | Parse once at the edge, carry proof in types |
| 8 | Capability Mixins | 🟡 | Ingredient traits + blanket impls |
| 9 | Phantom Types | 🟡 | PhantomData for register width, DMA direction |
| 10 | Putting It All Together | 🟡 | All 7 patterns in one diagnostic platform |
| 11 | Fourteen Tricks from the Trenches | 🟡 | Sentinel→Option, sealed traits, builders, etc. |
| 12 | Exercises | 🟡 | Six capstone problems with solutions |
| 13 | Reference Card | — | Pattern catalogue + decision flowchart |
| 14 | Testing Type-Level Guarantees | 🟡 | trybuild, proptest, cargo-show-asm |
| 15 | Const Fn | 🟠 | Compile-time proofs for memory maps, registers, bitfields |
| 16 | Send & Sync | 🟠 | Compile-time concurrency proofs |
| 17 | Redfish Client Walkthrough | 🟡 | Eight patterns composed into a type-safe Redfish client |
| 18 | Redfish Server Walkthrough | 🟡 | Builder type-state, source tokens, health rollup, mixins |

## Prerequisites

| Concept | Where to learn it |
|---------|-------------------|
| Ownership and borrowing | [Rust Patterns](../rust-patterns-book/src/SUMMARY.md), ch01 |
| Traits and associated types | [Rust Patterns](../rust-patterns-book/src/SUMMARY.md), ch02 |
| Newtypes and type-state | [Rust Patterns](../rust-patterns-book/src/SUMMARY.md), ch03 |
| PhantomData | [Rust Patterns](../rust-patterns-book/src/SUMMARY.md), ch04 |
| Generics and trait bounds | [Rust Patterns](../rust-patterns-book/src/SUMMARY.md), ch01 |

## The Correct-by-Construction Spectrum

```text
← Less Safe                                                    More Safe →

Runtime checks      Unit tests        Property tests      Correct by Construction
─────────────       ──────────        ──────────────      ──────────────────────

if temp > 100 {     #[test]           proptest! {         struct Celsius(f64);
  panic!("too       fn test_temp() {    |t in 0..200| {   // Can't confuse with Rpm
  hot");              assert!(          assert!(...)       // at the type level
}                     check(42));     }
                    }                 }
                                                          Invalid program?
Invalid program?    Invalid program?  Invalid program?    Won't compile.
Crashes in prod.    Fails in CI.      Fails in CI         Never exists.
                                      (probabilistic).
```

This guide operates at the rightmost position — where bugs don't exist because the type system **cannot express them**.

---

