# Rust crates and modules

> **What you'll learn:** How Rust organizes code into modules and crates — privacy-by-default visibility, `pub` modifiers, workspaces, and the `crates.io` ecosystem. Replaces C/C++ header files, `#include`, and CMake dependency management.

- Modules are the fundamental organizational unit of code within crates
    - Each source file (.rs) is its own module, and can create nested modules using the ```mod``` keyword.
    - All types in a (sub-) module are **private** by default, and aren't externally visible within the same crate unless they are explicitly marked as ```pub``` (public). The scope of ```pub``` can be further restricted to ```pub(crate)```, etc
    - Even if a type is public, it doesn't automatically become visible within the scope of another module unless it's imported using the ```use``` keyword. Child submodules can reference types in the parent scope using the ```use super::```
    - Source files (.rs) aren't automatically included in the crate **unless** they are explicitly listed in ```main.rs``` (executable) or ```lib.rs```

# Exercise: Modules and functions
- We'll take a look at modifying our [hello world](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=522d86dbb8c4af71ff2ec081fb76aee7) to call another function
    - As previously mentioned, function are defined with the ```fn``` keyword. The ```->``` keyword declares that the function returns a value (the default is void) with the type ```u32``` (unsigned 32-bit integer)
    - Functions are scoped by module, i.e., two functions with exact same name in two modules won't have a name collision
        - The module scoping extends to all types (for example, a ```struct foo``` in ```mod a { struct foo; }``` is a distinct type (```a::foo```) from ```mod b { struct foo; }``` (```b::foo```))

**Starter code** — complete the functions:
```rust
mod math {
    // TODO: implement pub fn add(a: u32, b: u32) -> u32
}

fn greet(name: &str) -> String {
    // TODO: return "Hello, <name>! The secret number is <math::add(21,21)>"
    todo!()
}

fn main() {
    println!("{}", greet("Rustacean"));
}
```

<details><summary>Solution (click to expand)</summary>

```rust
mod math {
    pub fn add(a: u32, b: u32) -> u32 {
        a + b
    }
}

fn greet(name: &str) -> String {
    format!("Hello, {}! The secret number is {}", name, math::add(21, 21))
}

fn main() {
    println!("{}", greet("Rustacean"));
}
// Output: Hello, Rustacean! The secret number is 42
```

</details>


## Workspaces and crates (packages)

- Any significant Rust project should use workspaces to organize component crates
    - A workspace is simply a collection of local crates that will be used to build the target binaries. The `Cargo.toml` at the workspace root should have a pointer to the constituent packages (crates)

```toml
[workspace]
resolver = "2"
members = ["package1", "package2"]
```

```text
workspace_root/
|-- Cargo.toml      # Workspace configuration
|-- package1/
|   |-- Cargo.toml  # Package 1 configuration
|   `-- src/
|       `-- lib.rs  # Package 1 source code
|-- package2/
|   |-- Cargo.toml  # Package 2 configuration
|   `-- src/
|       `-- main.rs # Package 2 source code
```

---
## Exercise: Using workspaces and package dependencies
- We'll create a simple package and use it from our ```hello world``` program`
- Create the workspace directory
```bash
mkdir workspace
cd workspace
```
- Create a file called Cargo.toml and add the following to it. This creates an empty workspace
```toml
[workspace]
resolver = "2"
members = []
```
- Add the packages (```cargo new --lib``` specifies a library instead of an executable`)
```bash
cargo new hello
cargo new --lib hellolib
```

## Exercise: Using workspaces and package dependencies
- Take a look at the generated Cargo.toml in ```hello``` and ```hellolib```. Notice that both of them have been to the upper level ```Cargo.toml```
- The presence of ```lib.rs``` in ```hellolib``` implies a library package (see https://doc.rust-lang.org/cargo/reference/cargo-targets.html for customization options)
- Adding a dependency on ```hellolib``` in ```Cargo.toml``` for ```hello```
```toml
[dependencies]
hellolib = {path = "../hellolib"}
```
- Using ```add()``` from ```hellolib```
```rust
fn main() {
    println!("Hello, world! {}", hellolib::add(21, 21));
}
```

<details><summary>Solution (click to expand)</summary>

The complete workspace setup:

```bash
# Terminal commands
mkdir workspace && cd workspace

# Create workspace Cargo.toml
cat > Cargo.toml << 'EOF'
[workspace]
resolver = "2"
members = ["hello", "hellolib"]
EOF

cargo new hello
cargo new --lib hellolib
```

```toml
# hello/Cargo.toml — add dependency
[dependencies]
hellolib = {path = "../hellolib"}
```

```rust
// hellolib/src/lib.rs — already has add() from cargo new --lib
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
```

```rust,ignore
// hello/src/main.rs
fn main() {
    println!("Hello, world! {}", hellolib::add(21, 21));
}
// Output: Hello, world! 42
```

</details>

# Using community crates from crates.io
- Rust has a vibrant ecosystem of community crates (see https://crates.io/)
    - The Rust philosophy is to keep the standard library compact and outsource functionality to community crates
    - There is no hard and fast rule about using community crates, but the rule of thumb should be to ensure that the crate has a decent maturity level (indicated by the version number), and that it's being actively maintained. Reach out to internal sources if in doubt about a crate
- Every crate published on ```crates.io``` has a major and minor version
    - Crates are expected to observe the major and minor ```SemVer``` guidelines defined here: https://doc.rust-lang.org/cargo/reference/semver.html
    - The TL;DR version is that there should be no breaking changes for the same minor version. For example, v0.11 must be compatible with v0.15 (but v0.20 may have breaking changes)

# Crates dependencies and SemVer
- Crates can define dependencies on a specific versions of a crate, specific minor or major version, or don't care. The following examples show the ```Cargo.toml``` entries for declaring a dependency on the ```rand``` crate
- At least ```0.10.0```, but anything ```< 0.11.0``` is fine
```toml
[dependencies]
rand = { version = "0.10.0"}
```
- Only ```0.10.0```, and nothing else
```toml
[dependencies]
rand = { version = "=0.10.0"}
```
- Don't care; ```cargo``` will select the latest version
```toml
[dependencies]
rand = { version = "*"}
```
- Reference: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
----
# Exercise: Using the rand crate
- Modify the ```helloworld``` example to print a random number
- Use ```cargo add rand``` to add a dependency
- Use ```https://docs.rs/rand/latest/rand/``` as a reference for the API

**Starter code** — add this to `main.rs` after running `cargo add rand`:
```rust,ignore
use rand::RngExt;

fn main() {
    let mut rng = rand::rng();
    // TODO: Generate and print a random u32 in 1..=100
    // TODO: Generate and print a random bool
    // TODO: Generate and print a random f64
}
```

<details><summary>Solution (click to expand)</summary>

```rust
use rand::RngExt;

fn main() {
    let mut rng = rand::rng();
    let n: u32 = rng.random_range(1..=100);
    println!("Random number (1-100): {n}");

    // Generate a random boolean
    let b: bool = rng.random();
    println!("Random bool: {b}");

    // Generate a random float between 0.0 and 1.0
    let f: f64 = rng.random();
    println!("Random float: {f:.4}");
}
```

</details>

# Cargo.toml and Cargo.lock
- As mentioned previously, Cargo.lock is automatically generated from Cargo.toml
    - The main idea behind Cargo.lock is to ensure reproducible builds. For example, if ```Cargo.toml``` had specified a version of ```0.10.0```, cargo is free to choose any version that is ```< 0.11.0```
    - Cargo.lock contains the *specific* version of the rand crate that was used during the build.
    - The recommendation is to include ```Cargo.lock``` in the git repo to ensure reproducible builds

## Cargo test feature
- Rust unit tests reside in the same source file (by convention), and are usually grouped into separate module
    - The test code is never included in the actual binary. This is made possible by the ```cfg``` (configuration) feature. Configurations are useful for creating platform specific code (```Linux``` vs. ```Windows```) for example
    - Tests can be executed with ```cargo test```. Reference: https://doc.rust-lang.org/reference/conditional-compilation.html

```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
// Will be included only during testing
#[cfg(test)]
mod tests {
    use super::*; // This makes all types in the parent scope visible
    #[test]
    fn it_works() {
        let result = add(2, 2); // Alternatively, super::add(2, 2);
        assert_eq!(result, 4);
    }
}
```

# Other Cargo features
- ```cargo``` has several other useful features including:
    - ```cargo clippy``` is a great way of linting Rust code. In general, warnings should be fixed (or rarely suppressed if really warranted)
    - ```cargo format``` executes the ```rustfmt``` tool to format source code. Using the tool ensures standard formatting of checked-in code and puts an end to debates about style
    - ```cargo doc``` can be used to generate documentation from the ```///``` style comments. The documentation for all crates on ```crates.io``` was generated using this method

### Build Profiles: Controlling Optimization

In C, you pass `-O0`, `-O2`, `-Os`, `-flto` to `gcc`/`clang`. In Rust, you configure
build profiles in `Cargo.toml`:

```toml
# Cargo.toml — build profile configuration

[profile.dev]
opt-level = 0          # No optimization (fast compile, like -O0)
debug = true           # Full debug symbols (like -g)

[profile.release]
opt-level = 3          # Maximum optimization (like -O3)
lto = "fat"            # Link-Time Optimization (like -flto)
strip = true           # Strip symbols (like the strip command)
codegen-units = 1      # Single codegen unit — slower compile, better optimization
panic = "abort"        # No unwind tables (smaller binary)
```

| C/GCC Flag | Cargo.toml Key | Values |
|------------|---------------|--------|
| `-O0` / `-O2` / `-O3` | `opt-level` | `0`, `1`, `2`, `3`, `"s"`, `"z"` |
| `-flto` | `lto` | `false`, `"thin"`, `"fat"` |
| `-g` / no `-g` | `debug` | `true`, `false`, `"line-tables-only"` |
| `strip` command | `strip` | `"none"`, `"debuginfo"`, `"symbols"`, `true`/`false` |
| — | `codegen-units` | `1` = best opt, slowest compile |

```bash
cargo build              # Uses [profile.dev]
cargo build --release    # Uses [profile.release]
```

### Build Scripts (`build.rs`): Linking C Libraries

In C, you use Makefiles or CMake to link libraries and run code generation.
Rust uses a `build.rs` file at the crate root:

```rust
// build.rs — runs before compiling the crate

fn main() {
    // Link a system C library (like -lbmc_ipmi in gcc)
    println!("cargo::rustc-link-lib=bmc_ipmi");

    // Where to find the library (like -L/usr/lib/bmc)
    println!("cargo::rustc-link-search=/usr/lib/bmc");

    // Re-run if the C header changes
    println!("cargo::rerun-if-changed=wrapper.h");
}
```

You can even compile C source files directly from a Rust crate:

```toml
# Cargo.toml
[build-dependencies]
cc = "1"  # C compiler integration
```

```rust
// build.rs
fn main() {
    cc::Build::new()
        .file("src/c_helpers/ipmi_raw.c")
        .include("/usr/include/bmc")
        .compile("ipmi_raw");   // Produces libipmi_raw.a, linked automatically
    println!("cargo::rerun-if-changed=src/c_helpers/ipmi_raw.c");
}
```

| C / Make / CMake | Rust `build.rs` |
|-----------------|-----------------|
| `-lfoo` | `println!("cargo::rustc-link-lib=foo")` |
| `-L/path` | `println!("cargo::rustc-link-search=/path")` |
| Compile C source | `cc::Build::new().file("foo.c").compile("foo")` |
| Generate code | Write files to `$OUT_DIR`, then `include!()` |

### Cross-Compilation

In C, cross-compilation requires installing a separate toolchain (`arm-linux-gnueabihf-gcc`)
and configuring Make/CMake. In Rust:

```bash
# Install a cross-compilation target
rustup target add aarch64-unknown-linux-gnu

# Cross-compile
cargo build --target aarch64-unknown-linux-gnu --release
```

Specify the linker in `.cargo/config.toml`:

```toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

| C Cross-Compile | Rust Equivalent |
|-----------------|-----------------|
| `apt install gcc-aarch64-linux-gnu` | `rustup target add aarch64-unknown-linux-gnu` + install linker |
| `CC=aarch64-linux-gnu-gcc make` | `.cargo/config.toml` `[target.X] linker = "..."` |
| `#ifdef __aarch64__` | `#[cfg(target_arch = "aarch64")]` |
| Separate Makefile targets | `cargo build --target ...` |

### Feature Flags: Conditional Compilation

C uses `#ifdef` and `-DFOO` for conditional compilation. Rust uses feature flags
defined in `Cargo.toml`:

```toml
# Cargo.toml
[features]
default = ["json"]         # Enabled by default
json = ["dep:serde_json"]  # Optional dependency
verbose = []               # Flag with no dependency
gpu = ["dep:cuda-sys"]     # Optional GPU support
```

```rust
// Code gated on features:
#[cfg(feature = "json")]
pub fn parse_config(data: &str) -> Result<Config, Error> {
    serde_json::from_str(data).map_err(Error::from)
}

#[cfg(feature = "verbose")]
macro_rules! verbose {
    ($($arg:tt)*) => { eprintln!("[VERBOSE] {}", format!($($arg)*)); }
}
#[cfg(not(feature = "verbose"))]
macro_rules! verbose {
    ($($arg:tt)*) => {}; // Compiles to nothing
}
```

| C Preprocessor | Rust Feature Flags |
|---------------|-------------------|
| `gcc -DDEBUG` | `cargo build --features verbose` |
| `#ifdef DEBUG` | `#[cfg(feature = "verbose")]` |
| `#define MAX 100` | `const MAX: u32 = 100;` |
| `#ifdef __linux__` | `#[cfg(target_os = "linux")]` |

### Integration Tests vs Unit Tests

Unit tests live next to the code with `#[cfg(test)]`. **Integration tests** live in
`tests/` and test your crate's **public API only**:

```rust
// tests/smoke_test.rs — no #[cfg(test)] needed
use my_crate::parse_config;

#[test]
fn parse_valid_config() {
    let config = parse_config("test_data/valid.json").unwrap();
    assert_eq!(config.max_retries, 5);
}
```

| Aspect | Unit Tests (`#[cfg(test)]`) | Integration Tests (`tests/`) |
|--------|----------------------------|------------------------------|
| Location | Same file as code | Separate `tests/` directory |
| Access | Private + public items | **Public API only** |
| Run command | `cargo test` | `cargo test --test smoke_test` |


### Testing Patterns and Strategies

C firmware teams typically write tests in CUnit, CMocka, or custom frameworks with a
lot of boilerplate. Rust's built-in test harness is far more capable. This section
covers patterns you'll need for production code.

#### `#[should_panic]` — Testing Expected Failures

```rust
// Test that certain conditions cause panics (like C's assert failures)
#[test]
#[should_panic(expected = "index out of bounds")]
fn test_bounds_check() {
    let v = vec![1, 2, 3];
    let _ = v[10];  // Should panic
}

#[test]
#[should_panic(expected = "temperature exceeds safe limit")]
fn test_thermal_shutdown() {
    fn check_temperature(celsius: f64) {
        if celsius > 105.0 {
            panic!("temperature exceeds safe limit: {celsius}°C");
        }
    }
    check_temperature(110.0);
}
```

#### `#[ignore]` — Slow or Hardware-Dependent Tests

```rust
// Mark tests that require special conditions (like C's #ifdef HARDWARE_TEST)
#[test]
#[ignore = "requires GPU hardware"]
fn test_gpu_ecc_scrub() {
    // This test only runs on machines with GPUs
    // Run with: cargo test -- --ignored
    // Run with: cargo test -- --include-ignored  (runs ALL tests)
}
```

#### Result-Returning Tests (replacing `unwrap` chains)

```rust
// Instead of many unwrap() calls that hide the actual failure:
#[test]
fn test_config_parsing() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{"hostname": "node-01", "port": 8080}"#;
    let config: ServerConfig = serde_json::from_str(json)?;  // ? instead of unwrap()
    assert_eq!(config.hostname, "node-01");
    assert_eq!(config.port, 8080);
    Ok(())  // Test passes if we reach here without error
}
```

#### Test Fixtures with Builder Functions

C uses `setUp()`/`tearDown()` functions. Rust uses helper functions and `Drop`:

```rust
struct TestFixture {
    temp_dir: std::path::PathBuf,
    config: Config,
}

impl TestFixture {
    fn new() -> Self {
        let temp_dir = std::env::temp_dir().join(format!("test_{}", std::process::id()));
        std::fs::create_dir_all(&temp_dir).unwrap();
        let config = Config {
            log_dir: temp_dir.clone(),
            max_retries: 3,
            ..Default::default()
        };
        Self { temp_dir, config }
    }
}

impl Drop for TestFixture {
    fn drop(&mut self) {
        // Automatic cleanup — like C's tearDown() but can't be forgotten
        let _ = std::fs::remove_dir_all(&self.temp_dir);
    }
}

#[test]
fn test_with_fixture() {
    let fixture = TestFixture::new();
    // Use fixture.config, fixture.temp_dir...
    assert!(fixture.temp_dir.exists());
    // fixture is automatically dropped here → cleanup runs
}
```

#### Mocking Traits for Hardware Interfaces

In C, mocking hardware requires preprocessor tricks or function pointer swapping.
In Rust, traits make this natural:

```rust
// Production trait for IPMI communication
trait IpmiTransport {
    fn send_command(&self, cmd: u8, data: &[u8]) -> Result<Vec<u8>, String>;
}

// Real implementation (used in production)
struct RealIpmi { /* BMC connection details */ }
impl IpmiTransport for RealIpmi {
    fn send_command(&self, cmd: u8, data: &[u8]) -> Result<Vec<u8>, String> {
        // Actually talks to BMC hardware
        todo!("Real IPMI call")
    }
}

// Mock implementation (used in tests)
struct MockIpmi {
    responses: std::collections::HashMap<u8, Vec<u8>>,
}
impl IpmiTransport for MockIpmi {
    fn send_command(&self, cmd: u8, _data: &[u8]) -> Result<Vec<u8>, String> {
        self.responses.get(&cmd)
            .cloned()
            .ok_or_else(|| format!("No mock response for cmd 0x{cmd:02x}"))
    }
}

// Generic function that works with both real and mock
fn read_sensor_temperature(transport: &dyn IpmiTransport) -> Result<f64, String> {
    let response = transport.send_command(0x2D, &[])?;
    if response.len() < 2 {
        return Err("Response too short".into());
    }
    Ok(response[0] as f64 + (response[1] as f64 / 256.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperature_reading() {
        let mut mock = MockIpmi { responses: std::collections::HashMap::new() };
        mock.responses.insert(0x2D, vec![72, 128]); // 72.5°C

        let temp = read_sensor_temperature(&mock).unwrap();
        assert!((temp - 72.5).abs() < 0.01);
    }

    #[test]
    fn test_short_response() {
        let mock = MockIpmi { responses: std::collections::HashMap::new() };
        // No response configured → error
        assert!(read_sensor_temperature(&mock).is_err());
    }
}
```

#### Property-Based Testing with `proptest`

Instead of testing specific values, test **properties** that must always hold:

```rust
// Cargo.toml: [dev-dependencies] proptest = "1"
use proptest::prelude::*;

fn parse_sensor_id(s: &str) -> Option<u32> {
    s.strip_prefix("sensor_")?.parse().ok()
}

fn format_sensor_id(id: u32) -> String {
    format!("sensor_{id}")
}

proptest! {
    #[test]
    fn roundtrip_sensor_id(id in 0u32..10000) {
        // Property: format then parse should give back the original
        let formatted = format_sensor_id(id);
        let parsed = parse_sensor_id(&formatted);
        prop_assert_eq!(parsed, Some(id));
    }

    #[test]
    fn parse_rejects_garbage(s in "[^s].*") {
        // Property: strings not starting with 's' should never parse
        let result = parse_sensor_id(&s);
        prop_assert!(result.is_none());
    }
}
```

#### C vs Rust Testing Comparison

| C Testing | Rust Equivalent |
|-----------|----------------|
| `CUnit`, `CMocka`, custom framework | Built-in `#[test]` + `cargo test` |
| `setUp()` / `tearDown()` | Builder function + `Drop` trait |
| `#ifdef TEST` mock functions | Trait-based dependency injection |
| `assert(x == y)` | `assert_eq!(x, y)` with auto diff output |
| Separate test executable | Same binary, conditional compilation with `#[cfg(test)]` |
| `valgrind --leak-check=full ./test` | `cargo test` (memory safe by default) + `cargo miri test` |
| Code coverage: `gcov` / `lcov` | `cargo tarpaulin` or `cargo llvm-cov` |
| Test discovery: manual registration | Automatic — any `#[test]` fn is discovered |



