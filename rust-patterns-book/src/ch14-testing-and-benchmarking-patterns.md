<a id="testing-and-benchmarking-patterns"></a>

# 14. 테스트와 벤치마킹 패턴 🟢

> **이 장에서 배울 내용:**
> - 단위·통합·문서 테스트라는 Rust의 세 가지 테스트 층
> - proptest를 이용한 프로퍼티 기반 테스트로 엣지 케이스 찾기
> - criterion으로 신뢰할 수 있는 성능 측정
> - 무거운 프레임워크 없이 쓰는 모킹 전략

<a id="unit-tests-integration-tests-doc-tests"></a>

## 단위 테스트, 통합 테스트, 문서 테스트

Rust에는 언어에 내장된 세 가지 테스트 층이 있습니다.

```rust
// --- 단위 테스트: 코드와 같은 파일 ---
pub fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial_zero() {
        // (1..=0).product()는 1을 반환 — 빈 범위에 대한 곱셈 항등원
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn test_factorial_five() {
        assert_eq!(factorial(5), 120);
    }

    #[test]
    #[cfg(debug_assertions)] // 오버플로 검사는 디버그 모드에서만 켜짐
    #[should_panic(expected = "overflow")]
    fn test_factorial_overflow() {
        // ⚠️ 이 테스트는 디버그 모드에서만 통과합니다(오버플로 검사가 켜짐).
        // 릴리스 모드(`cargo test --release`)에서는 u64 산술이 조용히 래핑되어
        // 패닉이 발생하지 않습니다. 릴리스에서도 안전하게 하려면 `checked_mul`이나
        // `overflow-checks = true` 프로필 설정을 사용하세요.
        factorial(100); // 오버플로 시 패닉해야 함
    }

    #[test]
    fn test_with_result() -> Result<(), Box<dyn std::error::Error>> {
        // 테스트는 Result를 반환할 수 있음 — 안에서 ? 사용 가능!
        let value: u64 = "42".parse()?;
        assert_eq!(value, 42);
        Ok(())
    }
}
```

```rust
// --- 통합 테스트: tests/ 디렉터리 ---
// tests/integration_test.rs
// 크레이트의 공개 API만 테스트합니다

use my_crate::factorial;

#[test]
fn test_factorial_from_outside() {
    assert_eq!(factorial(10), 3_628_800);
}
```

```rust
// --- 문서 테스트: 문서 주석 안 ---
/// `n`의 팩토리얼을 계산합니다.
///
/// # 예제
///
/// ```
/// use my_crate::factorial;
/// assert_eq!(factorial(5), 120);
/// ```
///
/// # 패닉
///
/// 결과가 `u64`를 넘치면 패닉합니다.
///
/// ```should_panic
/// my_crate::factorial(100);
/// ```
pub fn factorial(n: u64) -> u64 {
    (1..=n).product()
}
// 문서 테스트는 `cargo test`로 컴파일·실행되며 예제가 거짓이 되지 않게 합니다.
```

<a id="test-fixtures-and-setup"></a>

### 픽스처와 준비 코드

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // 공유 준비 — 헬퍼 함수로 만듭니다
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

    // Drop으로 정리(RAII):
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
        let dir = TempDir::new(); // 생성
        std::fs::write(dir.path.join("test.txt"), "hello").unwrap();
        assert!(dir.path.join("test.txt").exists());
    } // 여기서 dir이 drop → 임시 디렉터리 정리
}
```

<a id="property-based-testing-proptest"></a>

### 프로퍼티 기반 테스트(proptest)

특정 값만이 아니라 *항상 성립해야 하는 성질*을 테스트합니다.

```rust
// Cargo.toml: proptest = "1"
use proptest::prelude::*;

fn reverse(v: &[i32]) -> Vec<i32> {
    v.iter().rev().cloned().collect()
}

proptest! {
    #[test]
    fn test_reverse_twice_is_identity(v in prop::collection::vec(any::<i32>(), 0..100)) {
        // 성질: 두 번 뒤집으면 원래대로
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
        assert_eq!(v, sorted_once); // 두 번 정렬 = 한 번 정렬
    }

    #[test]
    fn test_parse_roundtrip(x in any::<f64>().prop_filter("finite", |x| x.is_finite())) {
        // 성질: 포맷 후 파싱하면 같은 값
        let s = format!("{x}");
        let parsed: f64 = s.parse().unwrap();
        prop_assert!((x - parsed).abs() < f64::EPSILON);
    }
}
```

> **proptest를 쓸 때**: 입력 공간이 넓고 직접 생각하지 못한 엣지 케이스까지
> 커버하고 싶을 때입니다. proptest는 수백 개의 무작위 입력을 만들고,
> 실패 시 최소 재현 케이스로 줄입니다(shrink).

<a id="benchmarking-with-criterion"></a>

### criterion으로 벤치마킹

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

    // 구현 비교:
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

// 실행: cargo bench
// HTML 리포트는 target/criterion/에 생성됩니다
```

<a id="mocking-strategies-without-frameworks"></a>

### 프레임워크 없는 모킹 전략

Rust의 트레잇 시스템이 자연스러운 의존성 주입을 제공합니다 — 모킹 프레임워크가 필수는 아닙니다.

```rust
// 동작을 트레잇으로 정의
trait Clock {
    fn now(&self) -> std::time::Instant;
}

trait HttpClient {
    fn get(&self, url: &str) -> Result<String, String>;
}

// 프로덕션 구현
struct RealClock;
impl Clock for RealClock {
    fn now(&self) -> std::time::Instant { std::time::Instant::now() }
}

// 서비스는 추상에 의존
struct CacheService<C: Clock, H: HttpClient> {
    clock: C,
    client: H,
    ttl: std::time::Duration,
}

impl<C: Clock, H: HttpClient> CacheService<C, H> {
    fn fetch(&self, url: &str) -> Result<String, String> {
        // self.clock, self.client 사용 — 주입 가능
        self.client.get(url)
    }
}

// 테스트용 목 구현 — 프레임워크 불필요!
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

> **테스트 철학**: 통합 테스트에서는 실제 의존성을, 단위 테스트에서는 트레잇 기반 목을 선호하세요.
> 의존성 그래프가 매우 복잡하지 않다면 모킹 프레임워크는 피하고, Rust의 트레잇 제네릭이 대부분 자연스럽게 처리합니다.

> **핵심 정리 — 테스트**
> - 문서 테스트(`///`)는 문서이면서 회귀 테스트 — 컴파일되어 실행됩니다
> - `proptest`는 직접 쓰지 않았을 엣지 케이스를 무작위 입력으로 찾습니다
> - `criterion`은 통계적으로 엄밀한 벤치마크와 HTML 리포트를 제공합니다
> - 목은 트레잇 제네릭 + 테스트 더블로, 목 프레임워크 대신 구현합니다

> **참고:** 매크로 생성 코드 테스트는 [12장 — 매크로](ch12-macros-code-that-writes-code.md). 모듈 구성이 테스트 구조에 미치는 영향은 [15장 — API 설계](ch15-crate-architecture-and-api-design.md).

---

<a id="exercise-property-based-testing-with-proptest"></a>

### 연습: proptest로 프로퍼티 기반 테스트 ★★ (~25분)

정렬 불변식을 유지하는 `SortedVec<T: Ord>` 래퍼를 작성하세요. `proptest`로 다음을 검증합니다.
1. 임의의 삽입 시퀀스 후에도 내부 vec이 항상 정렬되어 있는지
2. `contains()`가 표준 라이브러리 `Vec::contains()`와 일치하는지
3. 길이가 삽입 횟수와 같은지

<details>
<summary>🔑 해답</summary>

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

