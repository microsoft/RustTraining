# 15. Crate Architecture and API Design 🟡

> **What you'll learn:**
> - Module layout conventions and re-export strategies
> - The public API design checklist for polished crates
> - Ergonomic parameter patterns: `impl Into`, `AsRef`, `Cow`
> - "Parse, don't validate" with `TryFrom` and validated types
> - Feature flags, conditional compilation, and workspace organization

## Module Layout Conventions

```text
my_crate/
├── Cargo.toml
├── src/
│   ├── lib.rs          # Crate root — re-exports and public API
│   ├── config.rs       # Feature module
│   ├── parser/         # Complex module with sub-modules
│   │   ├── mod.rs      # or parser.rs at parent level (Rust 2018+)
│   │   ├── lexer.rs
│   │   └── ast.rs
│   ├── error.rs        # Error types
│   └── utils.rs        # Internal helpers (pub(crate))
├── tests/
│   └── integration.rs  # Integration tests
├── benches/
│   └── perf.rs         # Benchmarks
└── examples/
    └── basic.rs        # cargo run --example basic
```

```rust
// lib.rs — curate your public API with re-exports:
mod config;
mod error;
mod parser;
mod utils;

// Re-export what users need:
pub use config::Config;
pub use error::Error;
pub use parser::Parser;

// Public types are at the crate root — users write:
// use my_crate::Config;
// NOT: use my_crate::config::Config;
```

**Visibility modifiers**:

| Modifier | Visible To |
|----------|-----------|
| `pub` | Everyone |
| `pub(crate)` | This crate only |
| `pub(super)` | Parent module |
| `pub(in path)` | Specific ancestor module |
| (none) | Current module and its children |

### Public API Design Checklist

1. **Accept references, return owned** — `fn process(input: &str) -> String`
2. **Use `impl Trait` for parameters** — `fn read(r: impl Read)` instead of `fn read<R: Read>(r: R)` for cleaner signatures
3. **Return `Result`, not `panic!`** — let callers decide how to handle errors
4. **Implement standard traits** — `Debug`, `Display`, `Clone`, `Default`, `From`/`Into`
5. **Make invalid states unrepresentable** — use type states and newtypes
6. **Follow the builder pattern for complex configuration** — with type-state if fields are required
7. **Seal traits you don't want users to implement** — `pub trait Sealed: private::Sealed {}`
8. **Mark types and functions `#[must_use]`** — prevents silent discard of important `Result`s, guards, or values. Apply to any type where ignoring the return value is almost certainly a bug:
   ```rust
   #[must_use = "dropping the guard immediately releases the lock"]
   pub struct LockGuard<'a, T> { /* ... */ }

   #[must_use]
   pub fn validate(input: &str) -> Result<ValidInput, ValidationError> { /* ... */ }
   ```

```rust
// Sealed trait pattern — users can use but not implement:
mod private {
    pub trait Sealed {}
}

pub trait DatabaseDriver: private::Sealed {
    fn connect(&self, url: &str) -> Connection;
}

// Only types in THIS crate can implement Sealed → only we can implement DatabaseDriver
pub struct PostgresDriver;
impl private::Sealed for PostgresDriver {}
impl DatabaseDriver for PostgresDriver {
    fn connect(&self, url: &str) -> Connection { /* ... */ }
}
```

> **`#[non_exhaustive]`** — mark public enums and structs so that adding variants
> or fields is not a breaking change. Downstream crates must use a wildcard arm
> (`_ =>`) in match statements, and cannot construct the type with struct literal
> syntax:
> ```rust
> #[non_exhaustive]
> pub enum DiagError {
>     Timeout,
>     HardwareFault,
>     // Adding a new variant in a future release is NOT a semver break.
> }
> ```

### Ergonomic Parameter Patterns — `impl Into`, `AsRef`, `Cow`

One of Rust's most impactful API patterns is accepting the **most general type** in
function parameters, so callers don't need repetitive `.to_string()`, `&*s`, or `.as_ref()`
at every call site. This is the Rust-specific version of "be liberal in what you accept."

#### `impl Into<T>` — Accept Anything Convertible

```rust
// ❌ Friction: callers must convert manually
fn connect(host: String, port: u16) -> Connection {
    // ...
}
connect("localhost".to_string(), 5432);  // Annoying .to_string()
connect(hostname.clone(), 5432);          // Unnecessary clone if we already have String

// ✅ Ergonomic: accept anything that converts to String
fn connect(host: impl Into<String>, port: u16) -> Connection {
    let host = host.into();  // Convert once, inside the function
    // ...
}
connect("localhost", 5432);     // &str — zero friction
connect(hostname, 5432);        // String — moved, no clone
```

This works because Rust's `From`/`Into` trait pair provides blanket conversions.
When you accept `impl Into<T>`, you're saying: "give me anything that knows how to
become a `T`."

#### `AsRef<T>` — Borrow as a Reference

`AsRef<T>` is the borrowing counterpart to `Into<T>`. Use it when you only need
to *read* the data, not take ownership:

```rust
use std::path::Path;

// ❌ Forces callers to convert to &Path
fn file_exists(path: &Path) -> bool {
    path.exists()
}
file_exists(Path::new("/tmp/test.txt"));  // Awkward

// ✅ Accept anything that can behave as a &Path
fn file_exists(path: impl AsRef<Path>) -> bool {
    path.as_ref().exists()
}
file_exists("/tmp/test.txt");                    // &str ✅
file_exists(String::from("/tmp/test.txt"));      // String ✅
file_exists(Path::new("/tmp/test.txt"));         // &Path ✅
file_exists(PathBuf::from("/tmp/test.txt"));     // PathBuf ✅

// Same pattern for string-like parameters:
fn log_message(msg: impl AsRef<str>) {
    println!("[LOG] {}", msg.as_ref());
}
log_message("hello");                    // &str ✅
log_message(String::from("hello"));      // String ✅
```

#### `Cow<T>` — Clone on Write

`Cow<'a, T>` (Clone on Write) delays allocation until mutation is needed.
It holds either a borrowed `&T` or an owned `T::Owned`. This is perfect when
most calls don't need to modify the data:

```rust
use std::borrow::Cow;

/// Normalizes a diagnostic message — only allocates if changes are needed.
fn normalize_message(msg: &str) -> Cow<'_, str> {
    if msg.contains('\t') || msg.contains('\r') {
        // Must allocate — we need to modify the content
        Cow::Owned(msg.replace('\t', "    ").replace('\r', ""))
    } else {
        // No allocation — just borrow the original
        Cow::Borrowed(msg)
    }
}

// Most messages pass through without allocation:
let clean = normalize_message("All tests passed");          // Borrowed — free
let fixed = normalize_message("Error:\tfailed\r\n");        // Owned — allocated

// Cow<str> implements Deref<Target=str>, so it works like &str:
println!("{}", clean);
println!("{}", fixed.to_uppercase());
```

#### Quick Reference: Which to Use

```text
Do you need ownership of the data inside the function?
├── YES → impl Into<T>
│         "Give me anything that can become a T"
└── NO  → Do you only need to read it?
     ├── YES → impl AsRef<T> or &T
     │         "Give me anything I can borrow as a &T"
     └── MAYBE (might need to modify sometimes?)
          └── Cow<'_, T>
              "Borrow if possible, clone only when you must"
```

| Pattern | Ownership | Allocation | When to use |
|---------|-----------|------------|-------------|
| `&str` | Borrowed | Never | Simple string params |
| `impl AsRef<str>` | Borrowed | Never | Accept String, &str, etc. — read only |
| `impl Into<String>` | Owned | On conversion | Accept &str, String — will store/own |
| `Cow<'_, str>` | Either | Only if modified | Processing that usually doesn't modify |
| `&[u8]` / `impl AsRef<[u8]>` | Borrowed | Never | Byte-oriented APIs |

> **`Borrow<T>` vs `AsRef<T>`**: Both provide `&T`, but `Borrow<T>` additionally
> guarantees that `Eq`, `Ord`, and `Hash` are **consistent** between the original
> and borrowed form. This is why `HashMap<String, V>::get()` accepts `&Q where String: Borrow<Q>` — not `AsRef`. Use `Borrow` when the borrowed form is used
> as a lookup key; use `AsRef` for general "give me a reference" parameters.

#### Composing Conversions in APIs

```rust
/// A well-designed diagnostic API using ergonomic parameters:
pub struct DiagRunner {
    name: String,
    config_path: PathBuf,
    results: HashMap<String, TestResult>,
}

impl DiagRunner {
    /// Accept any string-like type for name, any path-like type for config.
    pub fn new(
        name: impl Into<String>,
        config_path: impl Into<PathBuf>,
    ) -> Self {
        DiagRunner {
            name: name.into(),
            config_path: config_path.into(),
        }
    }

    /// Accept any AsRef<str> for read-only lookup.
    pub fn get_result(&self, test_name: impl AsRef<str>) -> Option<&TestResult> {
        self.results.get(test_name.as_ref())
    }
}

// All of these work with zero caller friction:
let runner = DiagRunner::new("GPU Diag", "/etc/diag_tool/config.json");
let runner = DiagRunner::new(format!("Diag-{}", node_id), config_path);
let runner = DiagRunner::new(name_string, path_buf);
```

***

## Case Study: Designing a Public Crate API — Before & After

A real-world example of evolving a stringly-typed internal API into an ergonomic, type-safe public API. Consider a configuration parser crate:

**Before** (stringly-typed, easy to misuse):

```rust
// ❌ All parameters are strings — no compile-time validation
pub fn parse_config(path: &str, format: &str, strict: bool) -> Result<Config, String> {
    // What formats are valid? "json"? "JSON"? "Json"?
    // Is path a file path or URL?
    // What does "strict" even mean?
    todo!()
}
```

**After** (type-safe, self-documenting):

```rust
use std::path::Path;

/// Supported configuration formats.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]  // Adding formats won't break downstream
pub enum Format {
    Json,
    Toml,
    Yaml,
}

/// Controls parsing strictness.
#[derive(Debug, Clone, Copy, Default)]
pub enum Strictness {
    /// Reject unknown fields (default for libraries)
    #[default]
    Strict,
    /// Ignore unknown fields (useful for forward-compatible configs)
    Lenient,
}

pub fn parse_config(
    path: &Path,          // Type-enforced: must be a filesystem path
    format: Format,       // Enum: impossible to pass invalid format
    strictness: Strictness,  // Named alternatives, not a bare bool
) -> Result<Config, ConfigError> {
    todo!()
}
```

**What improved**:

| Aspect | Before | After |
|--------|--------|-------|
| Format validation | Runtime string comparison | Compile-time enum |
| Path type | Raw `&str` (could be anything) | `&Path` (filesystem-specific) |
| Strictness | Mystery `bool` | Self-documenting enum |
| Error type | `String` (opaque) | `ConfigError` (structured) |
| Extensibility | Breaking changes | `#[non_exhaustive]` |

> **Rule of thumb**: If you find yourself writing a `match` on string values,
> consider replacing the parameter with an enum. If a parameter is a boolean
> that isn't obvious from context, use a two-variant enum instead.

***

### Parse Don't Validate — `TryFrom` and Validated Types

"Parse, don't validate" is a principle that says: **don't check data and then pass
around the raw unchecked form — instead, parse it into a type that can only exist
if the data is valid.** Rust's `TryFrom` trait is the standard tool for this.

#### The Problem: Validation Without Enforcement

```rust
// ❌ Validate-then-use: nothing prevents using an invalid value after the check
fn process_port(port: u16) {
    if port == 0 || port > 65535 {
        panic!("Invalid port");           // We checked, but...
    }
    start_server(port);                    // What if someone calls start_server(0) directly?
}

// ❌ Stringly-typed: an email is just a String — any garbage gets through
fn send_email(to: String, body: String) {
    // Is `to` actually a valid email? We don't know.
    // Someone could pass "not-an-email" and we only find out at the SMTP server.
}
```

#### The Solution: Parse Into Validated Newtypes with `TryFrom`

```rust
use std::convert::TryFrom;
use std::fmt;

/// A validated TCP port number (1–65535).
/// If you have a `Port`, it is guaranteed valid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Port(u16);

impl TryFrom<u16> for Port {
    type Error = PortError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(PortError::Zero)
        } else {
            Ok(Port(value))
        }
    }
}

impl Port {
    pub fn get(&self) -> u16 { self.0 }
}

#[derive(Debug)]
pub enum PortError {
    Zero,
    InvalidFormat,
}

impl fmt::Display for PortError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PortError::Zero => write!(f, "port must be non-zero"),
            PortError::InvalidFormat => write!(f, "invalid port format"),
        }
    }
}

impl std::error::Error for PortError {}

// Now the type system enforces validity:
fn start_server(port: Port) {
    // No validation needed — Port can only be constructed via TryFrom,
    // which already verified it's valid.
    println!("Listening on port {}", port.get());
}

// Usage:
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = Port::try_from(8080)?;   // ✅ Validated once at the boundary
    start_server(port);                  // No re-validation anywhere downstream

    let bad = Port::try_from(0);         // ❌ Err(PortError::Zero)
    Ok(())
}
```

#### Real-World Example: Validated IPMI Address

```rust
/// A validated IPMI slave address (0x20–0xFE, even only).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IpmiAddr(u8);

#[derive(Debug)]
pub enum IpmiAddrError {
    Odd(u8),
    OutOfRange(u8),
}

impl fmt::Display for IpmiAddrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpmiAddrError::Odd(v) => write!(f, "IPMI address 0x{v:02X} must be even"),
            IpmiAddrError::OutOfRange(v) => {
                write!(f, "IPMI address 0x{v:02X} out of range (0x20..=0xFE)")
            }
        }
    }
}

impl TryFrom<u8> for IpmiAddr {
    type Error = IpmiAddrError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value % 2 != 0 {
            Err(IpmiAddrError::Odd(value))
        } else if value < 0x20 || value > 0xFE {
            Err(IpmiAddrError::OutOfRange(value))
        } else {
            Ok(IpmiAddr(value))
        }
    }
}

impl IpmiAddr {
    pub fn get(&self) -> u8 { self.0 }
}

// Downstream code never needs to re-check:
fn send_ipmi_command(addr: IpmiAddr, cmd: u8, data: &[u8]) -> Result<Vec<u8>, IpmiError> {
    // addr.get() is guaranteed to be a valid, even IPMI address
    raw_ipmi_send(addr.get(), cmd, data)
}
```

#### Parsing Strings with `FromStr`

For types that are commonly parsed from text (CLI args, config files), implement `FromStr`:

```rust
use std::str::FromStr;

impl FromStr for Port {
    type Err = PortError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n: u16 = s.parse().map_err(|_| PortError::InvalidFormat)?;
        Port::try_from(n)
    }
}

// Now works with .parse():
let port: Port = "8080".parse()?;   // Validates in one step

// And with clap CLI parsing:
// #[derive(Parser)]
// struct Args {
//     #[arg(short, long)]
//     port: Port,   // clap calls FromStr automatically
// }
```

#### `TryFrom` Chain for Complex Validation

```rust
// Stub types for this example — in production these would be in
// separate modules with their own TryFrom implementations.
```

```rust
# struct Hostname(String);
# impl TryFrom<String> for Hostname {
#     type Error = String;
#     fn try_from(s: String) -> Result<Self, String> { Ok(Hostname(s)) }
# }
# struct Timeout(u64);
# impl TryFrom<u64> for Timeout {
#     type Error = String;
#     fn try_from(ms: u64) -> Result<Self, String> {
#         if ms == 0 { Err("timeout must be > 0".into()) } else { Ok(Timeout(ms)) }
#     }
# }
# struct RawConfig { host: String, port: u16, timeout_ms: u64 }
# #[derive(Debug)]
# enum ConfigError {
#     InvalidHost(String),
#     InvalidPort(PortError),
#     InvalidTimeout(String),
# }
# impl From<std::io::Error> for ConfigError {
#     fn from(e: std::io::Error) -> Self { ConfigError::InvalidHost(e.to_string()) }
# }
# impl From<serde_json::Error> for ConfigError {
#     fn from(e: serde_json::Error) -> Self { ConfigError::InvalidHost(e.to_string()) }
# }
/// A validated configuration that can only exist if all fields are valid.
pub struct ValidConfig {
    pub host: Hostname,
    pub port: Port,
    pub timeout_ms: Timeout,
}

impl TryFrom<RawConfig> for ValidConfig {
    type Error = ConfigError;

    fn try_from(raw: RawConfig) -> Result<Self, Self::Error> {
        Ok(ValidConfig {
            host: Hostname::try_from(raw.host)
                .map_err(ConfigError::InvalidHost)?,
            port: Port::try_from(raw.port)
                .map_err(ConfigError::InvalidPort)?,
            timeout_ms: Timeout::try_from(raw.timeout_ms)
                .map_err(ConfigError::InvalidTimeout)?,
        })
    }
}

// Parse once at the boundary, use the validated type everywhere:
fn load_config(path: &str) -> Result<ValidConfig, ConfigError> {
    let raw: RawConfig = serde_json::from_str(&std::fs::read_to_string(path)?)?;
    ValidConfig::try_from(raw)  // All validation happens here
}
```

#### Summary: Validate vs Parse

| Approach | Data checked? | Compiler enforces validity? | Re-validation needed? |
|----------|:---:|:---:|:---:|
| Runtime checks (if/assert) | ✅ | ❌ | Every function boundary |
| Validated newtype + `TryFrom` | ✅ | ✅ | Never — type is proof |

The rule: **parse at the boundary, use validated types everywhere inside.**
Raw strings, integers, and byte slices enter your system, get parsed into
validated types via `TryFrom`/`FromStr`, and from that point forward the type
system guarantees they're valid.

### Feature Flags and Conditional Compilation

```toml
# Cargo.toml
[features]
default = ["json"]          # Enabled by default
json = ["dep:serde_json"]   # Enables JSON support
xml = ["dep:quick-xml"]     # Enables XML support
full = ["json", "xml"]      # Meta-feature: enables all

[dependencies]
serde = "1"
serde_json = { version = "1", optional = true }
quick-xml = { version = "0.31", optional = true }
```

```rust
// Conditional compilation based on features:
#[cfg(feature = "json")]
pub fn to_json<T: serde::Serialize>(value: &T) -> String {
    serde_json::to_string(value).unwrap()
}

#[cfg(feature = "xml")]
pub fn to_xml<T: serde::Serialize>(value: &T) -> String {
    quick_xml::se::to_string(value).unwrap()
}

// Compile error if a required feature isn't enabled:
#[cfg(not(any(feature = "json", feature = "xml")))]
compile_error!("At least one format feature (json, xml) must be enabled");
```

**Best practices**:
- Keep `default` features minimal — users can opt in
- Use `dep:` syntax (Rust 1.60+) for optional dependencies to avoid creating implicit features
- Document features in your README and crate-level docs

### Workspace Organization

For large projects, use a Cargo workspace to share dependencies and build artifacts:

```toml
# Root Cargo.toml
[workspace]
members = [
    "core",         # Shared types and traits
    "parser",       # Parsing library
    "server",       # Binary — the main application
    "client",       # Client library
    "cli",          # CLI binary
]

# Shared dependency versions:
[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
tracing = "0.1"

# In each member's Cargo.toml:
# [dependencies]
# serde = { workspace = true }
```

**Benefits**:

- Single `Cargo.lock` — all crates use the same dependency versions
- `cargo test --workspace` runs all tests
- Shared build cache — compiling one crate benefits all
- Clean dependency boundaries between components

### `.cargo/config.toml`: Project-Level Configuration

The `.cargo/config.toml` file (at the workspace root or in `$HOME/.cargo/`)
customizes Cargo behavior without modifying `Cargo.toml`:

```toml
# .cargo/config.toml

# Default target for this workspace
[build]
target = "x86_64-unknown-linux-gnu"

# Custom runner — e.g., run via QEMU for cross-compiled binaries
[target.aarch64-unknown-linux-gnu]
runner = "qemu-aarch64-static"
linker = "aarch64-linux-gnu-gcc"

# Cargo aliases — custom shortcut commands
[alias]
xt = "test --workspace --release"        # cargo xt = run all tests in release
ci = "clippy --workspace -- -D warnings" # cargo ci = lint with errors on warnings
cov = "llvm-cov --workspace"             # cargo cov = coverage (requires cargo-llvm-cov)

# Environment variables for build scripts
[env]
IPMI_LIB_PATH = "/usr/lib/bmc"

# Use a custom registry (for internal packages)
# [registries.internal]
# index = "https://gitlab.internal/crates/index"
```

Common configuration patterns:

| Setting | Purpose | Example |
|---------|---------|---------|
| `[build] target` | Default compilation target | `x86_64-unknown-linux-musl` for static builds |
| `[target.X] runner` | How to run the binary | `"qemu-aarch64-static"` for cross-compiled |
| `[target.X] linker` | Which linker to use | `"aarch64-linux-gnu-gcc"` |
| `[alias]` | Custom `cargo` subcommands | `xt = "test --workspace"` |
| `[env]` | Build-time environment variables | Library paths, feature toggles |
| `[net] offline` | Prevent network access | `true` for air-gapped builds |

### Compile-Time Environment Variables: `env!()` and `option_env!()`

Rust can embed environment variables into the binary at compile time — useful for
version strings, build metadata, and configuration:

```rust
// env!() — panics at compile time if the variable is missing
const VERSION: &str = env!("CARGO_PKG_VERSION"); // "0.1.0" from Cargo.toml
const PKG_NAME: &str = env!("CARGO_PKG_NAME");   // Crate name from Cargo.toml

// option_env!() — returns Option<&str>, doesn't panic if missing
const BUILD_SHA: Option<&str> = option_env!("GIT_SHA");
const BUILD_TIME: Option<&str> = option_env!("BUILD_TIMESTAMP");

fn print_version() {
    println!("{PKG_NAME} v{VERSION}");
    if let Some(sha) = BUILD_SHA {
        println!("  commit: {sha}");
    }
    if let Some(time) = BUILD_TIME {
        println!("  built:  {time}");
    }
}
```

Cargo automatically sets many useful environment variables:

| Variable | Value | Use case |
|----------|-------|----------|
| `CARGO_PKG_VERSION` | `"1.2.3"` | Version reporting |
| `CARGO_PKG_NAME` | `"diag_tool"` | Binary identification |
| `CARGO_PKG_AUTHORS` | From `Cargo.toml` | About/help text |
| `CARGO_MANIFEST_DIR` | Absolute path to `Cargo.toml` | Locating test data files |
| `OUT_DIR` | Build output directory | `build.rs` code generation target |
| `TARGET` | Target triple | Platform-specific logic in `build.rs` |

You can set custom env vars from `build.rs`:
```rust
// build.rs
fn main() {
    println!("cargo::rustc-env=GIT_SHA={}", git_sha());
    println!("cargo::rustc-env=BUILD_TIMESTAMP={}", timestamp());
}
```

### `cfg_attr`: Conditional Attributes

`cfg_attr` applies an attribute **only when** a condition is true. This is more
targeted than `#[cfg()]`, which includes/excludes entire items:

```rust
// Derive Serialize only when the "serde" feature is enabled:
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct DiagResult {
    pub fc: u32,
    pub passed: bool,
    pub message: String,
}
// Without "serde" feature: no serde dependency needed at all
// With "serde" feature: DiagResult is serializable

// Conditional attribute for testing:
#[cfg_attr(test, derive(PartialEq))]  // Only derive PartialEq in test builds
pub struct LargeStruct { /* ... */ }

// Platform-specific function attributes:
#[cfg_attr(target_os = "linux", link_name = "ioctl")]
#[cfg_attr(target_os = "freebsd", link_name = "__ioctl")]
extern "C" fn platform_ioctl(fd: i32, request: u64) -> i32;
```

| Pattern | What it does |
|---------|-------------|
| `#[cfg(feature = "x")]` | Include/exclude the entire item |
| `#[cfg_attr(feature = "x", derive(Foo))]` | Add `derive(Foo)` only when feature "x" is on |
| `#[cfg_attr(test, allow(unused))]` | Suppress warnings only in test builds |
| `#[cfg_attr(doc, doc = "...")]` | Documentation visible only in `cargo doc` |

### `cargo deny` and `cargo audit`: Supply-Chain Security

```bash
# Install security audit tools
cargo install cargo-deny
cargo install cargo-audit

# Check for known vulnerabilities in dependencies
cargo audit

# Comprehensive checks: licenses, bans, advisories, sources
cargo deny check
```

Configure `cargo deny` with a `deny.toml` at the workspace root:

```toml
# deny.toml
[advisories]
vulnerability = "deny"      # Fail on known vulnerabilities
unmaintained = "warn"        # Warn on unmaintained crates

[licenses]
allow = ["MIT", "Apache-2.0", "BSD-2-Clause", "BSD-3-Clause"]
deny = ["GPL-3.0"]          # Reject copyleft licenses

[bans]
multiple-versions = "warn"  # Warn if multiple versions of same crate
deny = [
    { name = "openssl" },   # Force use of rustls instead
]

[sources]
allow-git = []              # No git dependencies in production
```

| Tool | Purpose | When to run |
|------|---------|-------------|
| `cargo audit` | Check for known CVEs in dependencies | CI pipeline, pre-release |
| `cargo deny check` | Licenses, bans, advisories, sources | CI pipeline |
| `cargo deny check licenses` | License compliance only | Before open-sourcing |
| `cargo deny check bans` | Prevent specific crates | Enforce architecture decisions |

### Doc Tests: Tests Inside Documentation

Rust doc comments (`///`) can contain code blocks that are **compiled and run as tests**:

```rust
/// Parses a diagnostic fault code from a string.
///
/// # Examples
///
/// ```
/// use my_crate::parse_fc;
///
/// let fc = parse_fc("FC:12345").unwrap();
/// assert_eq!(fc, 12345);
/// ```
///
/// Invalid input returns an error:
///
/// ```
/// use my_crate::parse_fc;
///
/// assert!(parse_fc("not-a-fc").is_err());
/// ```
pub fn parse_fc(input: &str) -> Result<u32, ParseError> {
    input.strip_prefix("FC:")
        .ok_or(ParseError::MissingPrefix)?
        .parse()
        .map_err(ParseError::InvalidNumber)
}
```

```bash
cargo test --doc  # Run only doc tests
cargo test        # Runs unit + integration + doc tests
```

**Module-level documentation** uses `//!` at the top of a file:

```rust
//! # Diagnostic Framework
//!
//! This crate provides the core diagnostic execution engine.
//! It supports running diagnostic tests, collecting results,
//! and reporting to the BMC via IPMI.
//!
//! ## Quick Start
//!
//! ```no_run
//! use diag_framework::Framework;
//!
//! let mut fw = Framework::new("config.json")?;
//! fw.run_all_tests()?;
//! ```
```

### Benchmarking with Criterion

> **Full coverage**: See the [Benchmarking with criterion](ch14-testing-and-benchmarking-patterns.md#benchmarking-with-criterion)
> section in Chapter 13 (Testing and Benchmarking Patterns) for complete
> `criterion` setup, API examples, and a comparison table vs `cargo bench`.
> Below is a quick-reference for architecture-specific usage.

When benchmarking your crate's public API, place benchmarks in `benches/` and
keep them focused on the hot path — typically parsers, serializers, or
validation boundaries:

```bash
cargo bench                  # Run all benchmarks
cargo bench -- parse_config  # Run specific benchmark
# Results in target/criterion/ with HTML reports
```

> **Key Takeaways — Architecture & API Design**
> - Accept the most general type (`impl Into`, `impl AsRef`, `Cow`); return the most specific
> - Parse Don't Validate: use `TryFrom` to create types that are valid by construction
> - `#[non_exhaustive]` on public enums prevents breaking changes when adding variants
> - `#[must_use]` catches silent discards of important values

> **See also:** [Ch 9 — Error Handling](ch10-error-handling-patterns.md) for error type design in public APIs. [Ch 13 — Testing](ch14-testing-and-benchmarking-patterns.md) for testing your crate's public API.

---

### Exercise: Crate API Refactoring ★★ (~30 min)

Refactor the following "stringly-typed" API into one that uses `TryFrom`, newtypes, and builder pattern:

```rust,ignore
// BEFORE: Easy to misuse
fn create_server(host: &str, port: &str, max_conn: &str) -> Server { ... }
```

Design a `ServerConfig` with validated types `Host`, `Port` (1–65535), and `MaxConnections` (1–10000) that reject invalid values at parse time.

<details>
<summary>🔑 Solution</summary>

```rust
#[derive(Debug, Clone)]
struct Host(String);

impl TryFrom<&str> for Host {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, String> {
        if s.is_empty() { return Err("host cannot be empty".into()); }
        if s.contains(' ') { return Err("host cannot contain spaces".into()); }
        Ok(Host(s.to_string()))
    }
}

#[derive(Debug, Clone, Copy)]
struct Port(u16);

impl TryFrom<u16> for Port {
    type Error = String;
    fn try_from(p: u16) -> Result<Self, String> {
        if p == 0 { return Err("port must be >= 1".into()); }
        Ok(Port(p))
    }
}

#[derive(Debug, Clone, Copy)]
struct MaxConnections(u32);

impl TryFrom<u32> for MaxConnections {
    type Error = String;
    fn try_from(n: u32) -> Result<Self, String> {
        if n == 0 || n > 10_000 {
            return Err(format!("max_connections must be 1–10000, got {n}"));
        }
        Ok(MaxConnections(n))
    }
}

#[derive(Debug)]
struct ServerConfig {
    host: Host,
    port: Port,
    max_connections: MaxConnections,
}

impl ServerConfig {
    fn new(host: Host, port: Port, max_connections: MaxConnections) -> Self {
        ServerConfig { host, port, max_connections }
    }
}

fn main() {
    let config = ServerConfig::new(
        Host::try_from("localhost").unwrap(),
        Port::try_from(8080).unwrap(),
        MaxConnections::try_from(100).unwrap(),
    );
    println!("{config:?}");

    // Invalid values caught at parse time:
    assert!(Host::try_from("").is_err());
    assert!(Port::try_from(0).is_err());
    assert!(MaxConnections::try_from(99999).is_err());
}
```

</details>

***

