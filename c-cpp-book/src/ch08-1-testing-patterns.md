## C++ 개발자를 위한 테스트 패턴

> **이 장에서 배우는 것:** Rust의 내장 테스트 프레임워크입니다. `#[test]`, `#[should_panic]`, `Result`를 반환하는 테스트, 테스트 데이터용 builder 패턴, trait 기반 모킹, `proptest`를 이용한 property testing, `insta`를 이용한 snapshot testing, 그리고 integration test 구성 방식을 다룹니다. Google Test + CMake를 대체하는 zero-config 테스트 경험입니다.

C++ 테스트는 보통 Google Test, Catch2, Boost.Test 같은 외부 프레임워크와 복잡한 빌드 통합에 의존합니다. 반면 Rust의 테스트 프레임워크는 **언어와 툴체인에 내장**되어 있어, 별도 의존성도 CMake 통합도 테스트 러너 설정도 필요 없습니다.

### `#[test]` 외의 테스트 속성

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_pass() {
        assert_eq!(2 + 2, 4);
    }

    // 패닉을 기대 - GTest의 EXPECT_DEATH와 유사
    #[test]
    #[should_panic]
    fn out_of_bounds_panics() {
        let v = vec![1, 2, 3];
        let _ = v[10]; // 패닉 발생 - 테스트는 통과
    }

    // 특정 메시지 일부를 포함하는 패닉을 기대
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn specific_panic_message() {
        let v = vec![1, 2, 3];
        let _ = v[10];
    }

    // Result<(), E>를 반환하는 테스트 - unwrap() 대신 ? 사용
    #[test]
    fn test_with_result() -> Result<(), String> {
        let value: u32 = "42".parse().map_err(|e| format!("{e}"))?;
        assert_eq!(value, 42);
        Ok(())
    }

    // 느린 테스트는 기본적으로 제외 - `cargo test -- --ignored`로 실행
    #[test]
    #[ignore]
    fn slow_integration_test() {
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
```

```bash
cargo test                          # 제외되지 않은 모든 테스트 실행
cargo test -- --ignored             # ignored 테스트만 실행
cargo test -- --include-ignored     # ignored 포함 모든 테스트 실행
cargo test test_name                # 이름 패턴에 맞는 테스트만 실행
cargo test -- --nocapture           # 테스트 중 println! 출력 보기
cargo test -- --test-threads=1      # 테스트를 직렬 실행
```

### 테스트 데이터용 builder 패턴

C++에서는 Google Test fixture(`class MyTest : public ::testing::Test`)를 쓰곤 합니다. Rust에서는 builder 함수나 `Default` 트레잇을 쓰면 됩니다.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // 적당한 기본값을 가진 테스트 데이터 생성 함수
    fn make_gpu_event(severity: Severity, fault_code: u32) -> DiagEvent {
        DiagEvent {
            source: "accel_diag".to_string(),
            severity,
            message: format!("Test event FC:{fault_code}"),
            fault_code,
        }
    }

    // 미리 구성된 이벤트 묶음
    fn sample_events() -> Vec<DiagEvent> {
        vec![
            make_gpu_event(Severity::Critical, 67956),
            make_gpu_event(Severity::Warning, 32709),
            make_gpu_event(Severity::Info, 10001),
        ]
    }

    #[test]
    fn filter_critical_events() {
        let events = sample_events();
        let critical: Vec<_> = events.iter()
            .filter(|e| e.severity == Severity::Critical)
            .collect();
        assert_eq!(critical.len(), 1);
        assert_eq!(critical[0].fault_code, 67956);
    }
}
```

### trait 기반 모킹

C++에서는 Google Mock이나 수동 virtual override로 mocking을 합니다. Rust에서는 의존성에 대한 trait를 정의하고, 테스트에서 다른 구현을 주입하면 됩니다.

```rust
// 프로덕션 trait
trait SensorReader {
    fn read_temperature(&self, sensor_id: u32) -> Result<f64, String>;
}

// 프로덕션 구현
struct HwSensorReader;
impl SensorReader for HwSensorReader {
    fn read_temperature(&self, sensor_id: u32) -> Result<f64, String> {
        // 실제 하드웨어 호출...
        Ok(72.5)
    }
}

// 테스트용 mock - 예측 가능한 값을 반환
#[cfg(test)]
struct MockSensorReader {
    temperatures: std::collections::HashMap<u32, f64>,
}

#[cfg(test)]
impl SensorReader for MockSensorReader {
    fn read_temperature(&self, sensor_id: u32) -> Result<f64, String> {
        self.temperatures.get(&sensor_id)
            .copied()
            .ok_or_else(|| format!("Unknown sensor {sensor_id}"))
    }
}

// 테스트 대상 함수 - reader 구현체에 대해 제네릭
fn check_overtemp(reader: &impl SensorReader, ids: &[u32], threshold: f64) -> Vec<u32> {
    ids.iter()
        .filter(|&&id| reader.read_temperature(id).unwrap_or(0.0) > threshold)
        .copied()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_overtemp_sensors() {
        let mut mock = MockSensorReader { temperatures: Default::default() };
        mock.temperatures.insert(0, 72.5);
        mock.temperatures.insert(1, 91.0); // 임계치 초과
        mock.temperatures.insert(2, 65.0);

        let hot = check_overtemp(&mock, &[0, 1, 2], 80.0);
        assert_eq!(hot, vec![1]);
    }
}
```

### 테스트에서 임시 파일과 디렉터리 사용

C++ 테스트는 플랫폼별 임시 디렉터리를 직접 다루는 경우가 많습니다. Rust는 `tempfile` 크레이트를 쓰면 됩니다.

```rust
// Cargo.toml: [dev-dependencies]
// tempfile = "3"

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn parse_config_from_file() -> Result<(), Box<dyn std::error::Error>> {
        // drop 시 자동 삭제되는 임시 파일 생성
        let mut file = NamedTempFile::new()?;
        writeln!(file, r#"{{"sku": "ServerNode", "level": "Quick"}}"#)?;

        let config = load_config(file.path().to_str().unwrap())?;
        assert_eq!(config.sku, "ServerNode");
        Ok(())
        // 여기서 file 삭제 - 별도 cleanup 불필요
    }
}
```

### `proptest`를 이용한 property-based testing

특정 테스트 케이스를 직접 쓰는 대신, 모든 입력에 대해 성립해야 하는 **성질(property)** 을 기술할 수 있습니다. `proptest`는 랜덤 입력을 생성하고, 실패 케이스를 최소화해 보여줍니다.

```rust
// Cargo.toml: [dev-dependencies]
// proptest = "1"

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    fn parse_and_format(n: u32) -> String {
        format!("{n}")
    }

    proptest! {
        #[test]
        fn roundtrip_u32(n: u32) {
            let formatted = parse_and_format(n);
            let parsed: u32 = formatted.parse().unwrap();
            prop_assert_eq!(n, parsed);
        }

        #[test]
        fn string_contains_no_null(s in "[a-zA-Z0-9 ]{0,100}") {
            prop_assert!(!s.contains('\0'));
        }
    }
}
```

### `insta`를 이용한 snapshot testing

JSON이나 포맷된 문자열처럼 출력이 복잡한 경우, `insta`가 기준 snapshot을 자동 생성하고 관리해 줍니다.

```rust
// Cargo.toml: [dev-dependencies]
// insta = { version = "1", features = ["json"] }

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    #[test]
    fn der_entry_format() {
        let entry = DerEntry {
            fault_code: 67956,
            component: "GPU".to_string(),
            message: "ECC error detected".to_string(),
        };
        // 첫 실행: tests/snapshots/에 snapshot 생성
        // 이후 실행: 저장된 snapshot과 비교
        assert_json_snapshot!(entry);
    }
}
```

```bash
cargo insta test              # 테스트 실행 + 새/변경 snapshot 검토
cargo insta review            # snapshot 변경 인터랙티브 리뷰
```

### C++ vs Rust 테스트 비교

| **C++ (Google Test)** | **Rust** | **비고** |
|----------------------|---------|----------|
| `TEST(Suite, Name) { }` | `#[test] fn name() { }` | suite/class 계층이 필요 없음 |
| `ASSERT_EQ(a, b)` | `assert_eq!(a, b)` | 내장 매크로 |
| `ASSERT_NEAR(a, b, eps)` | `assert!((a - b).abs() < eps)` | 또는 `approx` 크레이트 |
| `EXPECT_THROW(expr, type)` | `#[should_panic(expected = "...")]` | 세밀한 제어가 필요하면 `catch_unwind` |
| `EXPECT_DEATH(expr, "msg")` | `#[should_panic(expected = "msg")]` | |
| `class Fixture : public ::testing::Test` | builder 함수 + `Default` | 상속 불필요 |
| Google Mock `MOCK_METHOD` | trait + 테스트 구현체 | 더 명시적 |
| `INSTANTIATE_TEST_SUITE_P` | `proptest!` 또는 매크로 생성 테스트 | |
| `SetUp()` / `TearDown()` | `Drop` 기반 RAII | 테스트 끝에서 자동 정리 |
| 별도 테스트 바이너리 + CMake | `cargo test` | 설정 거의 없음 |
| `ctest --output-on-failure` | `cargo test -- --nocapture` | |

----

### 통합 테스트: `tests/` 디렉터리

단위 테스트는 코드 옆의 `#[cfg(test)]` 모듈에 둡니다. **통합 테스트**는 크레이트 루트의 `tests/` 디렉터리에 별도로 두며, 외부 사용자 관점에서 라이브러리의 공개 API를 테스트합니다.

```text
my_crate/
├── src/
│   └── lib.rs          # 라이브러리 코드
├── tests/
│   ├── smoke.rs        # 각 .rs 파일은 별도 테스트 바이너리
│   ├── regression.rs
│   └── common/
│       └── mod.rs      # 공유 헬퍼 (테스트 바이너리는 아님)
└── Cargo.toml
```

```rust
// tests/smoke.rs — 외부 사용자처럼 공개 API만 테스트
use my_crate::DiagEngine;

#[test]
fn engine_starts_successfully() {
    let engine = DiagEngine::new("test_config.json");
    assert!(engine.is_ok());
}

#[test]
fn engine_rejects_invalid_config() {
    let engine = DiagEngine::new("nonexistent.json");
    assert!(engine.is_err());
}
```

```rust
// tests/common/mod.rs — 공유 헬퍼, 테스트 바이너리 아님
pub fn setup_test_environment() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("config.json"), r#"{"log_level": "debug"}"#).unwrap();
    dir
}
```

```rust
// tests/regression.rs — 공유 헬퍼 사용 가능
mod common;

#[test]
fn regression_issue_42() {
    let env = common::setup_test_environment();
    let engine = my_crate::DiagEngine::new(
        env.path().join("config.json").to_str().unwrap()
    );
    assert!(engine.is_ok());
}
```

**통합 테스트 실행**
```bash
cargo test                          # 단위 + 통합 테스트 모두 실행
cargo test --test smoke             # tests/smoke.rs만 실행
cargo test --test regression        # tests/regression.rs만 실행
cargo test --lib                    # 단위 테스트만 실행
```

> **단위 테스트와의 핵심 차이:** 통합 테스트는 private 함수나 `pub(crate)` 아이템에 접근할 수 없습니다. 공개 API만으로 충분히 테스트할 수 있는지가 설계 품질의 신호가 됩니다. C++ 식으로 말하면 `friend` 없이 공개 헤더만 보고 테스트하는 것에 가깝습니다.

----
