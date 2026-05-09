<a id="when-and-why-to-use-unsafe"></a>
## `unsafe`를 언제, 왜 써야 하는가

> **이 장에서 배울 내용:** `unsafe`가 무엇을 허용하며 왜 존재하는지, PyO3로 Python 확장을 만드는 방법,
> Rust의 테스트 프레임워크와 `pytest`의 차이, `mockall`을 이용한 목 객체 작성, 그리고 벤치마킹의 기본 감각까지 함께 익힙니다.
>
> **난이도:** 🔴 고급

Rust의 `unsafe`는 탈출구입니다. 컴파일러에게 "당신이 검증할 수 없는 일을 내가 하고 있지만, 그 코드가 올바르다고 내가 보증하겠다"라고 선언하는 셈입니다. Python에는 직접적인 대응물이 없습니다. Python은 메모리에 직접 접근할 수 있게 해주지 않기 때문입니다.

```mermaid
flowchart TB
    subgraph Safe ["안전한 Rust (코드의 99%)"]
        S1["애플리케이션 로직"]
        S2["pub fn safe_api\(&self\) -> Result"]
    end
    subgraph Unsafe ["`unsafe` 블록 (작고, 감사 가능하게)"]
        U1["raw pointer 역참조"]
        U2["C/Python으로의 FFI 호출"]
    end
    subgraph External ["외부 세계 (C / Python / OS)"]
        E1["libc / PyO3 / 시스템 호출"]
    end
    S1 --> S2
    S2 --> U1
    S2 --> U2
    U1 --> E1
    U2 --> E1
    style Safe fill:#d4edda,stroke:#28a745
    style Unsafe fill:#fff3cd,stroke:#ffc107
    style External fill:#f8d7da,stroke:#dc3545
```

> **전형적인 패턴**: 안전한 API가 아주 작은 `unsafe` 블록을 감쌉니다. 호출자는 `unsafe`를 직접 볼 필요가 없습니다. Python의 `ctypes`에는 이런 경계가 없어서, 모든 FFI 호출이 사실상 암묵적으로 unsafe입니다.
>
> 📌 **함께 보면 좋은 장**: [13장 - 동시성](ch13-concurrency.md)에서는 컴파일러가 스레드 안전성을 검사할 때 사용하는 `unsafe` 자동 트레잇 `Send`/`Sync`를 다룹니다.

### `unsafe`가 허용하는 것
```rust
// unsafe lets you do FIVE things that safe Rust forbids:
// 1. Dereference raw pointers
// 2. Call unsafe functions/methods
// 3. Access mutable static variables
// 4. Implement unsafe traits
// 5. Access union fields

// Example: calling a C function
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    let result = unsafe { abs(-42) };  // Safe Rust can't verify C code
    println!("{result}");               // 42
}
```

### `unsafe`를 써야 하는 때
```rust
// 1. FFI — calling C libraries (most common reason)
// 2. Performance-critical inner loops (rare)
// 3. Data structures the borrow checker can't express (rare)

// As a Python developer, you'll mostly encounter unsafe in:
// - PyO3 internals (Python ↔ Rust bridge)
// - C library bindings
// - Low-level system calls

// Rule of thumb: if you're writing application code (not library code),
// you should almost never need unsafe. If you think you do, ask in the
// Rust community first — there's usually a safe alternative.
```

***

<a id="pyo3-rust-extensions-for-python"></a>
## PyO3: Python을 위한 Rust 확장

PyO3는 Python과 Rust를 이어주는 다리입니다. Python에서 호출할 수 있는 Rust 함수와 클래스를 만들 수 있으므로, 느린 Python 병목 구간을 Rust로 치환할 때 특히 강력합니다.

### Rust로 Python 확장 만들기
```bash
# 설정
pip install maturin    # Rust Python 확장을 빌드하는 도구
maturin init           # 프로젝트 구조 생성

# 프로젝트 구조:
# my_extension/
# ├── Cargo.toml
# ├── pyproject.toml
# └── src/
#     └── lib.rs
```

```toml
# Cargo.toml
[package]
name = "my_extension"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]    # Python용 공유 라이브러리

[dependencies]
pyo3 = { version = "0.22", features = ["extension-module"] }
```

```rust
// src/lib.rs — Rust functions callable from Python
use pyo3::prelude::*;

/// Rust로 작성한 빠른 Fibonacci 함수.
#[pyfunction]
fn fibonacci(n: u64) -> u64 {
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..n {
        let temp = b;
        b = a.wrapping_add(b);
        a = temp;
    }
    a
}

/// n 이하의 모든 소수를 찾는다(에라토스테네스의 체).
#[pyfunction]
fn primes_up_to(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    if n > 0 { is_prime[1] = false; }
    for i in 2..=((n as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in (i * i..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    (2..=n).filter(|&i| is_prime[i]).collect()
}

/// Python에서 사용할 수 있는 Rust 클래스.
#[pyclass]
struct Counter {
    value: i64,
}

#[pymethods]
impl Counter {
    #[new]
    fn new(start: i64) -> Self {
        Counter { value: start }
    }

    fn increment(&mut self) {
        self.value += 1;
    }

    fn get_value(&self) -> i64 {
        self.value
    }

    fn __repr__(&self) -> String {
        format!("Counter(value={})", self.value)
    }
}

/// Python 모듈 정의.
#[pymodule]
fn my_extension(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    m.add_function(wrap_pyfunction!(primes_up_to, m)?)?;
    m.add_class::<Counter>()?;
    Ok(())
}
```

### Python에서 사용하기
```bash
# 빌드 및 설치:
maturin develop --release   # 현재 venv에 빌드해서 설치
```

```python
# Python — use the Rust extension like any Python module
import my_extension

# Call Rust function
result = my_extension.fibonacci(50)
print(result)  # 12586269025 — computed in microseconds

# Use Rust class
counter = my_extension.Counter(0)
counter.increment()
counter.increment()
print(counter.get_value())  # 2
print(counter)              # Counter(value=2)

# Performance comparison:
import time

# Python version
def py_primes(n):
    sieve = [True] * (n + 1)
    for i in range(2, int(n**0.5) + 1):
        if sieve[i]:
            for j in range(i*i, n+1, i):
                sieve[j] = False
    return [i for i in range(2, n+1) if sieve[i]]

start = time.perf_counter()
py_result = py_primes(10_000_000)
py_time = time.perf_counter() - start

start = time.perf_counter()
rs_result = my_extension.primes_up_to(10_000_000)
rs_time = time.perf_counter() - start

print(f"Python: {py_time:.3f}s")    # ~3.5s
print(f"Rust:   {rs_time:.3f}s")    # ~0.05s — 70x faster!
print(f"Same results: {py_result == rs_result}")  # True
```

### PyO3 빠른 참조

| Python 개념 | PyO3 attribute | 설명 |
|-------------|----------------|------|
| 함수 | `#[pyfunction]` | Python에 노출되는 함수 |
| 클래스 | `#[pyclass]` | Python에서 보이는 클래스 |
| 메서드 | `#[pymethods]` | `pyclass`의 메서드 모음 |
| `__init__` | `#[new]` | 생성자 |
| `__repr__` | `fn __repr__()` | 문자열 표현 |
| `__str__` | `fn __str__()` | 표시용 문자열 |
| `__len__` | `fn __len__()` | 길이 |
| `__getitem__` | `fn __getitem__()` | 인덱싱 |
| 프로퍼티 | `#[getter]` / `#[setter]` | 속성 접근 |
| 정적 메서드 | `#[staticmethod]` | `self` 없음 |
| 클래스 메서드 | `#[classmethod]` | `cls`를 받음 |

### FFI 안전성 패턴

Rust 코드를 Python에 노출할 때(PyO3든 raw C FFI든) 아래 규칙을 지키면 가장 흔한 버그를 피할 수 있습니다.

1. **panic이 FFI 경계를 넘지 않게 하라**. Rust panic이 Python(또는 C) 쪽으로 unwinding되면 **정의되지 않은 동작**입니다. PyO3는 `#[pyfunction]`에서 이를 자동으로 처리해주지만, raw `extern "C"` 함수에서는 직접 막아야 합니다.

   ```rust
   #[no_mangle]
   pub extern "C" fn raw_ffi_function() -> i32 {
       match std::panic::catch_unwind(|| {
           // actual logic
           42
       }) {
           Ok(result) => result,
           Err(_) => -1,  // Return error code instead of panicking into C/Python
       }
   }
   ```

2. **공유 구조체에는 `#[repr(C)]`를 붙여라**. Python/C가 구조체 필드를 직접 읽는다면 C 호환 레이아웃을 보장하기 위해 반드시 `#[repr(C)]`가 필요합니다. 반대로 PyO3의 `#[pyclass]`처럼 opaque pointer만 넘기는 경우에는 필요하지 않습니다.

3. **raw FFI에는 `extern "C"`가 필요하다**. 호출 규약이 C/Python이 기대하는 것과 정확히 맞아야 하기 때문입니다. PyO3의 `#[pyfunction]`은 이 부분도 내부에서 처리해줍니다.

> **PyO3의 장점**: panic 포착, 타입 변환, GIL 관리 등 대부분의 안전성 문제를 PyO3가 감싸줍니다. 특별한 이유가 없다면 raw FFI보다 PyO3를 우선 선택하세요.

***

<!-- ch14a: Testing -->
<a id="unit-tests-vs-pytest"></a>
## 단위 테스트 vs `pytest`

### `pytest`로 하는 Python 테스트
```python
# test_calculator.py
import pytest
from calculator import add, divide

def test_add():
    assert add(2, 3) == 5

def test_add_negative():
    assert add(-1, 1) == 0

def test_divide():
    assert divide(10, 2) == 5.0

def test_divide_by_zero():
    with pytest.raises(ZeroDivisionError):
        divide(1, 0)

# Parameterized tests
@pytest.mark.parametrize("a,b,expected", [
    (1, 2, 3),
    (0, 0, 0),
    (-1, -1, -2),
    (100, 200, 300),
])
def test_add_parametrized(a, b, expected):
    assert add(a, b) == expected

# Fixtures
@pytest.fixture
def sample_data():
    return [1, 2, 3, 4, 5]

def test_sum(sample_data):
    assert sum(sample_data) == 15
```

```bash
# 테스트 실행
pytest                      # 전체 테스트 실행
pytest test_calculator.py   # 파일 하나만 실행
pytest -k "test_add"        # 이름이 맞는 테스트만 실행
pytest -v                   # 자세한 출력
pytest --tb=short           # 짧은 traceback
```

### Rust 내장 테스트
```rust
// src/calculator.rs — tests live in the SAME file!
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

// Tests go in a #[cfg(test)] module — only compiled during `cargo test`
#[cfg(test)]
mod tests {
    use super::*;  // Import everything from parent module

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));
    }

    #[test]
    fn test_divide_by_zero() {
        assert!(divide(1.0, 0.0).is_err());
    }

    // Test that something panics (like pytest.raises)
    #[test]
    #[should_panic(expected = "out of bounds")]
    fn test_out_of_bounds() {
        let v = vec![1, 2, 3];
        let _ = v[99];  // Panics
    }
}
```

```bash
# 테스트 실행
cargo test                         # 전체 테스트 실행
cargo test test_add                # 이름이 맞는 테스트만 실행
cargo test -- --nocapture          # println! 출력까지 표시
cargo test -p my_crate             # 워크스페이스에서 크레이트 하나만 테스트
cargo test -- --test-threads=1     # 순차 실행(부작용 있는 테스트용)
```

### 테스트 빠른 참조

| `pytest` | Rust | 설명 |
|----------|------|------|
| `assert x == y` | `assert_eq!(x, y)` | 동등성 비교 |
| `assert x != y` | `assert_ne!(x, y)` | 부등성 비교 |
| `assert condition` | `assert!(condition)` | 불리언 검증 |
| `assert condition, "msg"` | `assert!(condition, "msg")` | 메시지 포함 검증 |
| `pytest.raises(E)` | `#[should_panic]` | panic 기대 |
| `@pytest.fixture` | 테스트 내부 설정 또는 helper fn | 내장 fixture는 없음 |
| `@pytest.mark.parametrize` | `rstest` 크레이트 | 매개변수화 테스트 |
| `conftest.py` | `tests/common/mod.rs` | 공용 테스트 도우미 |
| `pytest.skip()` | `#[ignore]` | 테스트 건너뛰기 |
| `tmp_path` fixture | `tempfile` 크레이트 | 임시 디렉터리 |

***

## `rstest`로 매개변수화 테스트하기
```rust
// Cargo.toml: rstest = "0.23"

use rstest::rstest;

// Like @pytest.mark.parametrize
#[rstest]
#[case(1, 2, 3)]
#[case(0, 0, 0)]
#[case(-1, -1, -2)]
#[case(100, 200, 300)]
fn test_add(#[case] a: i32, #[case] b: i32, #[case] expected: i32) {
    assert_eq!(add(a, b), expected);
}

// Like @pytest.fixture
use rstest::fixture;

#[fixture]
fn sample_data() -> Vec<i32> {
    vec![1, 2, 3, 4, 5]
}

#[rstest]
fn test_sum(sample_data: Vec<i32>) {
    assert_eq!(sample_data.iter().sum::<i32>(), 15);
}
```

***

## `mockall`로 목 객체 만들기
```python
# Python — mocking with unittest.mock
from unittest.mock import Mock, patch

def test_fetch_user():
    mock_db = Mock()
    mock_db.get_user.return_value = {"name": "Alice"}

    result = fetch_user_name(mock_db, 1)
    assert result == "Alice"
    mock_db.get_user.assert_called_once_with(1)
```

```rust
// Rust — mocking with mockall crate
// Cargo.toml: mockall = "0.13"

use mockall::{automock, predicate::*};

#[automock]                          // Generates MockDatabase automatically
trait Database {
    fn get_user(&self, id: i64) -> Option<User>;
}

fn fetch_user_name(db: &dyn Database, id: i64) -> Option<String> {
    db.get_user(id).map(|u| u.name)
}

#[test]
fn test_fetch_user() {
    let mut mock = MockDatabase::new();
    mock.expect_get_user()
        .with(eq(1))                   // assert_called_with(1)
        .times(1)                      // assert_called_once
        .returning(|_| Some(User { name: "Alice".into() }));

    let result = fetch_user_name(&mock, 1);
    assert_eq!(result, Some("Alice".to_string()));
}
```

---

## 연습문제

<details>
<summary><strong>🏋️ 연습문제: unsafe 위에 안전한 래퍼 만들기</strong> (펼쳐서 보기)</summary>

**도전 과제**: `&mut [i32]`를 받아 가운데 지점에서 둘로 나눈 두 개의 가변 슬라이스 `(&mut [i32], &mut [i32])`를 반환하는 안전한 함수 `split_at_mid`를 작성해보세요. 내부 구현에서는 raw pointer와 `unsafe`를 사용해 `split_at_mut`가 하는 일을 흉내 내고, 바깥에는 안전한 API를 제공하세요.

<details>
<summary>🔑 해답</summary>

```rust
fn split_at_mid(slice: &mut [i32]) -> (&mut [i32], &mut [i32]) {
    let mid = slice.len() / 2;
    let ptr = slice.as_mut_ptr();
    let len = slice.len();

    assert!(mid <= len); // Safety check before unsafe

    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut data = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mid(&mut data);
    left[0] = 99;
    right[0] = 88;
    println!("left: {left:?}, right: {right:?}");
    // left: [99, 2, 3], right: [88, 5, 6]
}
```

**핵심 정리**: `unsafe` 블록은 아주 작고, 그 앞에 `assert!`로 전제 조건을 지켰습니다. 공개 API는 완전히 안전하므로 호출자는 `unsafe`를 볼 필요가 없습니다. 이것이 Rust의 전형적인 패턴입니다. 내부는 unsafe일 수 있어도, 외부 인터페이스는 안전하게 유지합니다. Python의 `ctypes`는 이런 보장을 제공하지 않습니다.

</details>
</details>

***
