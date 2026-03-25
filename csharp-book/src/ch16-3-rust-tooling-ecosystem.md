## Essential Rust Tooling for C# Developers

> **What you'll learn:** Rust's development tools mapped to their C# equivalents — Clippy (Roslyn analyzers),
> rustfmt (dotnet format), cargo doc (XML docs), cargo watch (dotnet watch), and VS Code extensions.
>
> **Difficulty:** 🟢 Beginner

### Tool Comparison

| C# Tool | Rust Equivalent | Install | Purpose |
|---------|----------------|---------|---------|
| Roslyn analyzers | **Clippy** | `rustup component add clippy` | Lint + style suggestions |
| `dotnet format` | **rustfmt** | `rustup component add rustfmt` | Auto-formatting |
| XML doc comments | **`cargo doc`** | Built-in | Generate HTML docs |
| OmniSharp / Roslyn | **rust-analyzer** | VS Code extension | IDE support |
| `dotnet watch` | **cargo-watch** | `cargo install cargo-watch` | Auto-rebuild on save |
| — | **cargo-expand** | `cargo install cargo-expand` | See macro expansion |
| `dotnet audit` | **cargo-audit** | `cargo install cargo-audit` | Security vulnerability scan |

### Clippy: Your Automated Code Reviewer
```bash
# Run Clippy on your project
cargo clippy

# Treat warnings as errors (CI/CD)
cargo clippy -- -D warnings

# Auto-fix suggestions
cargo clippy --fix
```

```rust
// Clippy catches hundreds of anti-patterns:

// Before Clippy:
if x == true { }           // warning: equality check with bool
let _ = vec.len() == 0;    // warning: use .is_empty() instead
for i in 0..vec.len() { }  // warning: use .iter().enumerate()

// After Clippy suggestions:
if x { }
let _ = vec.is_empty();
for (i, item) in vec.iter().enumerate() { }
```

### rustfmt: Consistent Formatting
```bash
# Format all files
cargo fmt

# Check formatting without changing (CI/CD)
cargo fmt -- --check
```

```toml
# rustfmt.toml — customize formatting (like .editorconfig)
max_width = 100
tab_spaces = 4
use_field_init_shorthand = true
```

### cargo doc: Documentation Generation
```bash
# Generate and open docs (including dependencies)
cargo doc --open

# Run documentation tests
cargo test --doc
```

```rust
/// Calculate the area of a circle.
///
/// # Arguments
/// * `radius` - The radius of the circle (must be non-negative)
///
/// # Examples
/// ```
/// let area = my_crate::circle_area(5.0);
/// assert!((area - 78.54).abs() < 0.01);
/// ```
///
/// # Panics
/// Panics if `radius` is negative.
pub fn circle_area(radius: f64) -> f64 {
    assert!(radius >= 0.0, "radius must be non-negative");
    std::f64::consts::PI * radius * radius
}
// The code in /// ``` blocks is compiled and run during `cargo test`!
```

### cargo watch: Auto-Rebuild
```bash
# Rebuild on file changes (like dotnet watch)
cargo watch -x check          # Type-check only (fastest)
cargo watch -x test           # Run tests on save
cargo watch -x 'run -- args'  # Run program on save
cargo watch -x clippy         # Lint on save
```

### cargo expand: See What Macros Generate
```bash
# See the expanded output of derive macros
cargo expand --lib            # Expand lib.rs
cargo expand module_name      # Expand specific module
```

### Recommended VS Code Extensions

| Extension | Purpose |
|-----------|---------|
| **rust-analyzer** | Code completion, inline errors, refactoring |
| **CodeLLDB** | Debugger (like Visual Studio debugger) |
| **Even Better TOML** | Cargo.toml syntax highlighting |
| **crates** | Show latest crate versions in Cargo.toml |
| **Error Lens** | Inline error/warning display |

***

For deeper exploration of advanced topics mentioned in this guide, see the companion training documents:

- **[Rust Patterns](../../rust-patterns-book/src/SUMMARY.md)** — Pin projections, custom allocators, arena patterns, lock-free data structures, and advanced unsafe patterns
- **[Async Rust](../../async-book/src/SUMMARY.md)** — Deep dive into tokio, async cancellation safety, stream processing, and production async architectures
- **[Rust for C/C++ Programmers](../../c-cpp-book/src/SUMMARY.md)** — Useful if your team also has C or C++ experience; covers move semantics, RAII, FFI, embedded, and `no_std`