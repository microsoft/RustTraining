# 14. Testing and Benchmarking Patterns 🟢

> **What you'll learn:**
> - Rust's three test tiers: unit, integration, and doc tests
> - Property-based testing with proptest for discovering edge cases
> - Benchmarking with criterion for reliable performance measurement
> - Mocking strategies without heavyweight frameworks

## Unit Tests, Integration Tests, Doc Tests

Rust has three testing tiers built into the language:

```rust
// --- Unit tests: in the same file as the code ---
pub fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial_zero() {
        // (1..=0).product() returns 1 — the multiplication identity for empty ranges
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn test_factorial_five() {
        assert_eq!(factorial(5), 120);
    }

    #[test]
    #[cfg(debug_assertions)] // overflow checks are only enabled in debug mode
    #[should_panic(expected = "overflow")]
    fn test_factorial_overflow() {
        // ⚠️ This test only passes in debug mode (overflow checks enabled).
        // In release mode (`cargo test --release`), u64 arithmetic wraps
        // silently and no panic occurs. Use `checked_mul` or the
        // `overflow-checks = true` profile setting for release-mode safety.
        factorial(100); // Should panic on overflow
    }

    #[test]
    fn test_with_result() -> Result<(), Box<dyn std::error::Error>> {
        // Tests can return Result — ? works inside!
        let value: u64 = "42".parse()?;
        assert_eq!(value, 42);
        Ok(())
    }
}
```

```rust
// --- Integration tests: in tests/ directory ---
// tests/integration_test.rs
// These test your crate's PUBLIC API only

use my_crate::factorial;

#[test]
fn test_factorial_from_outside() {
    assert_eq!(factorial(10), 3_628_800);
}
```

```rust
// --- Doc tests: in documentation comments ---
/// Computes the factorial of `n`.
///
/// # Examples
///
/// ```
/// use my_crate::factorial;
/// assert_eq!(factorial(5), 120);
/// ```
///
/// # Panics
///
/// Panics if the result overflows `u64`.
///
/// ```should_panic
/// my_crate::factorial(100);
/// ```
pub fn factorial(n: u64) -> u64 {
    (1..=n).product()
}
// Doc tests are compiled and run by `cargo test` — they keep examples honest.
```

### Test Fixtures and Setup

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Shared setup — create a helper function
    fn setup_database() -> TestDb {
        let db = TestDb::new_in_memory();
        db.run_migrations();
        db.seed_test_data();
        db
    }

    #[test]
    fn test_user_creation() {
        let db = setup_database();
        let user = db.create_user("Alice", "alice@test.com").unwrap();
        assert_eq!(user.name, "Alice");
    }

    #[test]
    fn test_user_deletion() {
        let db = setup_database();
        db.create_user("Bob", "bob@test.com").unwrap();
        assert!(db.delete_user("Bob").is_ok());
        assert!(db.get_user("Bob").is_none());
    }

    // Cleanup with Drop (RAII):
    struct TempDir {
        path: std::path::PathBuf,
    }

    impl TempDir {
        fn new() -> Self {
            // Cargo.toml: rand = "0.8"
            let path = std::env::temp_dir().join(format!("test_{}", rand::random::<u32>()));
            std::fs::create_dir_all(&path).unwrap();
            TempDir { path }
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn test_file_operations() {
        let dir = TempDir::new(); // Created
        std::fs::write(dir.path.join("test.txt"), "hello").unwrap();
        assert!(dir.path.join("test.txt").exists());
    } // dir dropped here → temp directory cleaned up
}
```

### Property-Based Testing (proptest)

Instead of testing specific values, test *properties* that should always hold:

```rust
// Cargo.toml: proptest = "1"
use proptest::prelude::*;

fn reverse(v: &[i32]) -> Vec<i32> {
    v.iter().rev().cloned().collect()
}

proptest! {
    #[test]
    fn test_reverse_twice_is_identity(v in prop::collection::vec(any::<i32>(), 0..100)) {
        // Property: reversing twice gives back the original
        assert_eq!(reverse(&reverse(&v)), v);
    }

    #[test]
    fn test_reverse_preserves_length(v in prop::collection::vec(any::<i32>(), 0..100)) {
        assert_eq!(reverse(&v).len(), v.len());
    }

    #[test]
    fn test_sort_is_idempotent(mut v in prop::collection::vec(any::<i32>(), 0..100)) {
        v.sort();
        let sorted_once = v.clone();
        v.sort();
        assert_eq!(v, sorted_once); // Sorting twice = sorting once
    }

    #[test]
    fn test_parse_roundtrip(x in any::<f64>().prop_filter("finite", |x| x.is_finite())) {
        // Property: formatting then parsing gives back the same value
        let s = format!("{x}");
        let parsed: f64 = s.parse().unwrap();
        prop_assert!((x - parsed).abs() < f64::EPSILON);
    }
}
```

> **When to use proptest**: When you're testing a function with a large input
> space and want confidence it works for edge cases you didn't think of.
> proptest generates hundreds of random inputs and shrinks failures to the
> minimal reproducing case.

### Benchmarking with criterion

```rust
// Cargo.toml:
// [dev-dependencies]
// criterion = { version = "0.5", features = ["html_reports"] }
//
// [[bench]]
// name = "my_benchmarks"
// harness = false

// benches/my_benchmarks.rs
use criterion::{criterion_group, criterion_main, Criterion, black_box};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn bench_fibonacci(c: &mut Criterion) {
    c.bench_function("fibonacci 20", |b| {
        b.iter(|| fibonacci(black_box(20)))
    });

    // Compare different implementations:
    let mut group = c.benchmark_group("fibonacci_compare");
    for size in [10, 15, 20, 25] {
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(size),
            &size,
            |b, &size| b.iter(|| fibonacci(black_box(size))),
        );
    }
    group.finish();
}

criterion_group!(benches, bench_fibonacci);
criterion_main!(benches);

// Run: cargo bench
// Produces HTML reports in target/criterion/
```

### Mocking Strategies without Frameworks

Rust's trait system provides natural dependency injection — no mocking framework required:

```rust
// Define behavior as a trait
trait Clock {
    fn now(&self) -> std::time::Instant;
}

trait HttpClient {
    fn get(&self, url: &str) -> Result<String, String>;
}

// Production implementations
struct RealClock;
impl Clock for RealClock {
    fn now(&self) -> std::time::Instant { std::time::Instant::now() }
}

// Service depends on abstractions
struct CacheService<C: Clock, H: HttpClient> {
    clock: C,
    client: H,
    ttl: std::time::Duration,
}

impl<C: Clock, H: HttpClient> CacheService<C, H> {
    fn fetch(&self, url: &str) -> Result<String, String> {
        // Uses self.clock and self.client — injectable
        self.client.get(url)
    }
}

// Test with mock implementations — no framework needed!
#[cfg(test)]
mod tests {
    use super::*;

    struct MockClock {
        fixed_time: std::time::Instant,
    }
    impl Clock for MockClock {
        fn now(&self) -> std::time::Instant { self.fixed_time }
    }

    struct MockHttpClient {
        response: String,
    }
    impl HttpClient for MockHttpClient {
        fn get(&self, _url: &str) -> Result<String, String> {
            Ok(self.response.clone())
        }
    }

    #[test]
    fn test_cache_service() {
        let service = CacheService {
            clock: MockClock { fixed_time: std::time::Instant::now() },
            client: MockHttpClient { response: "cached data".into() },
            ttl: std::time::Duration::from_secs(300),
        };

        assert_eq!(service.fetch("http://example.com").unwrap(), "cached data");
    }
}
```

> **Test philosophy**: Prefer real dependencies in integration tests, trait-based
> mocks in unit tests. Avoid mocking frameworks unless your dependency graph is
> complex — Rust's trait generics handle most cases naturally.

> **Key Takeaways — Testing**
> - Doc tests (`///`) double as documentation and regression tests — they're compiled and run
> - `proptest` generates random inputs to find edge cases you'd never write manually
> - `criterion` provides statistically rigorous benchmarks with HTML reports
> - Mock via trait generics + test doubles, not mock frameworks

> **See also:** [Ch 12 — Macros](ch13-macros-code-that-writes-code.md) for testing macro-generated code. [Ch 14 — API Design](ch15-crate-architecture-and-api-design.md) for how module layout affects test organization.

---

### Exercise: Property-Based Testing with proptest ★★ (~25 min)

Write a `SortedVec<T: Ord>` wrapper that maintains a sorted invariant. Use `proptest` to verify that:
1. After any sequence of insertions, the internal vec is always sorted
2. `contains()` agrees with the stdlib `Vec::contains()`
3. The length equals the number of insertions

<details>
<summary>🔑 Solution</summary>

```rust,ignore
#[derive(Debug)]
struct SortedVec<T: Ord> {
    inner: Vec<T>,
}

impl<T: Ord> SortedVec<T> {
    fn new() -> Self { SortedVec { inner: Vec::new() } }

    fn insert(&mut self, value: T) {
        let pos = self.inner.binary_search(&value).unwrap_or_else(|p| p);
        self.inner.insert(pos, value);
    }

    fn contains(&self, value: &T) -> bool {
        self.inner.binary_search(value).is_ok()
    }

    fn len(&self) -> usize { self.inner.len() }
    fn as_slice(&self) -> &[T] { &self.inner }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn always_sorted(values in proptest::collection::vec(-1000i32..1000, 0..100)) {
            let mut sv = SortedVec::new();
            for v in &values {
                sv.insert(*v);
            }
            for w in sv.as_slice().windows(2) {
                prop_assert!(w[0] <= w[1]);
            }
            prop_assert_eq!(sv.len(), values.len());
        }

        #[test]
        fn contains_matches_stdlib(values in proptest::collection::vec(0i32..50, 1..30)) {
            let mut sv = SortedVec::new();
            for v in &values {
                sv.insert(*v);
            }
            for v in &values {
                prop_assert!(sv.contains(v));
            }
            prop_assert!(!sv.contains(&9999));
        }
    }
}
```

</details>

***

