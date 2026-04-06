<div style="background-color: #d9d9d9; padding: 16px; border-radius: 6px; color: #000000;">

**License** This project is dual-licensed under the [MIT License](LICENSE) and [Creative Commons Attribution 4.0 International (CC-BY-4.0)](LICENSE-DOCS).

</div>

<div style="background-color: #d9d9d9; padding: 16px; border-radius: 6px; color: #000000;">

**Trademarks** This project may contain trademarks or logos for projects, products, or services. Authorized use of Microsoft trademarks or logos is subject to and must follow [Microsoft's Trademark & Brand Guidelines](https://www.microsoft.com/en-us/legal/intellectualproperty/trademarks/usage/general). Use of Microsoft trademarks or logos in modified versions of this project must not cause confusion or imply Microsoft sponsorship. Any use of third-party trademarks or logos are subject to those third-party's policies.

</div>

# Rust Training Books

Seven training courses covering Rust from different programming backgrounds, plus deep-dives on async, advanced patterns, and engineering practices.

This material combines original content with ideas and examples inspired by some of the best resources in the Rust ecosystem. The goal is to present an in-depth, technically accurate curriculum that weaves together knowledge scattered across books, blogs, conference talks, and video series into a cohesive, pedagogically structured experience.

> **Disclaimer:** These books are training material, not an authoritative reference. While we strive for accuracy, always verify critical details against the [official Rust documentation](https://doc.rust-lang.org/) and the [Rust Reference](https://doc.rust-lang.org/reference/).

### Inspirations & Acknowledgments

- [**The Rust Programming Language**](https://doc.rust-lang.org/book/) — the foundation everything builds on
- [**Jon Gjengset**](https://www.youtube.com/c/JonGjengset) — deep-dive streams on advanced Rust internals, `Crust of Rust` series
- [**withoutboats**](https://without.boats/blog/) — async design, `Pin`, and the futures model
- [**fasterthanlime (Amos)**](https://fasterthanli.me/) — systems programming from first principles, engaging long-form explorations
- [**Mara Bos**](https://marabos.nl/) — *Rust Atomics and Locks*, concurrency primitives
- [**Aleksey Kladov (matklad)**](https://matklad.github.io/) — Rust analyzer insights, API design, error handling patterns
- [**Niko Matsakis**](https://smallcultfollowing.com/babysteps/) — language design, borrow checker internals, Polonius
- [**Rust by Example**](https://doc.rust-lang.org/rust-by-example/) and [**Rustonomicon**](https://doc.rust-lang.org/nomicon/) — practical patterns and unsafe deep-dives
- [**This Week in Rust**](https://this-week-in-rust.org/) — community discoveries that shaped many examples
- …and many others in the **Rust community at large** whose blog posts, conference talks, RFCs, and forum discussions have informed this material — too numerous to list individually, but deeply appreciated

## 📖 Start Reading

Pick the book that matches your background. Books are grouped by complexity so you can chart a learning path:

| Level | Description |
|-------|-------------|
| 🟢 **Bridge** | Learn Rust coming from another language — start here |
| 🔵 **Deep Dive** | Focused exploration of a major Rust subsystem |
| 🟡 **Advanced** | Patterns and techniques for experienced Rustaceans |
| 🟣 **Expert** | Cutting-edge type-level and correctness techniques |
| 🟤 **Practices** | Engineering, tooling, and production readiness |

| Book | Level | Who it's for |
|------|-------|-------------|
| [**Rust for C/C++ Programmers**](c-cpp-book/src/SUMMARY.md) | 🟢 Bridge | Move semantics, RAII, FFI, embedded, no_std |
| [**Rust for C# Programmers**](csharp-book/src/SUMMARY.md) | 🟢 Bridge | Swift / C# / Java → ownership & type system |
| [**Rust for Python Programmers**](python-book/src/SUMMARY.md) | 🟢 Bridge | Dynamic → static typing, GIL-free concurrency |
| [**Async Rust**](async-book/src/SUMMARY.md) | 🔵 Deep Dive | Tokio, streams, cancellation safety |
| [**Rust Patterns**](rust-patterns-book/src/SUMMARY.md) | 🟡 Advanced | Pin, allocators, lock-free structures, unsafe |
| [**Type-Driven Correctness**](type-driven-correctness-book/src/SUMMARY.md) | 🟣 Expert | Type-state, phantom types, capability tokens |
| [**Rust Engineering Practices**](engineering-book/src/SUMMARY.md) | 🟤 Practices | Build scripts, cross-compilation, CI/CD, Miri |

Each book has 15–16 chapters with Mermaid diagrams, editable Rust playgrounds, exercises, and full-text search.

> **Tip:** You can read the markdown source directly on GitHub, or browse the rendered site with sidebar navigation and search at the [GitHub Pages site](https://microsoft.github.io/RustTraining/).
>
> **Local serving:** For the best reading experience (keyboard navigation between chapters, instant search, offline access), clone the repo and run:
> ```
> # Install Rust via rustup if you don't have it yet:
> # https://rustup.rs/
>
> cargo install mdbook@0.4.52 mdbook-mermaid@0.14.0
> cargo xtask serve          # builds all books and opens a local server
> ```

---

## 🔧 For Maintainers

<details>
<summary>Building, serving, and editing the books locally</summary>

### Prerequisites

Install [Rust via **rustup**](https://rustup.rs/) if you haven't already, then:

```bash
cargo install mdbook@0.4.52 mdbook-mermaid@0.14.0
```

### Build & serve

```bash
cargo xtask build               # Build all books into site/ (local preview)
cargo xtask serve               # Build and serve at http://localhost:3000
cargo xtask deploy              # Build all books into docs/ (for GitHub Pages)
cargo xtask clean               # Remove site/ and docs/
```

To build or serve a single book:

```bash
cd c-cpp-book && mdbook serve --open    # http://localhost:3000
```

### Deployment

The site auto-deploys to GitHub Pages on push to `main` via `.github/workflows/pages.yml`. No manual steps needed.

</details>
