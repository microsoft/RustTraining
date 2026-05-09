<a id="avoiding-unchecked-indexing"></a>
## 검사되지 않은 인덱싱 피하기

> **이 장에서 배우는 것:** Rust에서 `vec[i]`가 왜 위험한지(범위를 벗어나면 `panic`), 그리고 `.get()`, 이터레이터, `HashMap`의 `entry()` API 같은 안전한 대안을 배웁니다. C++의 정의되지 않은 동작을 Rust에서는 명시적인 처리로 바꿉니다.

- C++에서는 `vec[i]`가 UB를 만들 수 있고, `map[key]`는 키가 없을 때 조용히 값을 삽입할 수 있습니다. Rust의 `[]`는 범위를 벗어나면 `panic`합니다.
- **규칙**: 인덱스가 유효하다고 *증명*할 수 있는 경우가 아니면 `[]` 대신 `.get()`을 사용하세요.

<a id="c-rust-comparison"></a>
### C++ → Rust 비교
```cpp
// C++ — 조용한 UB 또는 암묵적 삽입
std::vector<int> v = {1, 2, 3};
int x = v[10];        // UB! operator[]에는 범위 검사가 없음

std::map<std::string, int> m;
int y = m["missing"]; // 값 0과 함께 키를 조용히 삽입!
```

```rust
// Rust — 안전한 대안
let v = vec![1, 2, 3];

// 나쁨: 범위를 벗어나면 panic
// let x = v[10];

// 좋음: Option<&i32> 반환
let x = v.get(10);              // None — panic 없음
let x = v.get(1).copied().unwrap_or(0);  // 2, 없으면 0
```

<a id="real-example-safe-byte-parsing-from-production-rust-code"></a>
### 실제 예: 프로덕션 Rust 코드의 안전한 바이트 파싱
```rust
// 예시: diagnostics.rs
// 바이너리 SEL 레코드 파싱 — 버퍼가 예상보다 짧을 수 있음
let sensor_num = bytes.get(7).copied().unwrap_or(0);
let ppin = cpu_ppin.get(i).map(|s| s.as_str()).unwrap_or("");
```

<a id="real-example-chained-safe-lookups-with-and_then"></a>
### 실제 예: `.and_then()`으로 안전한 조회를 연쇄하기
```rust
// 예시: profile.rs — 이중 조회: HashMap → Vec
pub fn get_processor(&self, location: &str) -> Option<&Processor> {
    self.processor_by_location
        .get(location)                              // HashMap → Option<&usize>
        .and_then(|&idx| self.processors.get(idx))   // Vec → Option<&Processor>
}
// 두 조회 모두 Option을 반환하므로 panic도 UB도 없음
```

<a id="real-example-safe-json-navigation"></a>
### 실제 예: 안전한 JSON 탐색
```rust
// 예시: framework.rs — 모든 JSON 키 조회는 Option을 반환
let manufacturer = product_fru
    .get("Manufacturer")            // Option<&Value>
    .and_then(|v| v.as_str())       // Option<&str>
    .unwrap_or(UNKNOWN_VALUE)       // &str (안전한 기본값)
    .to_string();
```
C++ 패턴인 `json["SystemInfo"]["ProductFru"]["Manufacturer"]`와 비교해 보세요. 중간 키 하나라도 빠지면 `nlohmann::json::out_of_range` 예외가 발생할 수 있습니다.

<a id="when-bracket-indexing-is-acceptable"></a>
### `[]`를 써도 되는 경우
- **범위 검사를 마친 뒤**: `if i < v.len() { v[i] }`
- **테스트 코드에서**: panic이 의도한 동작일 때
- **상수 인덱스에서**: `assert!(!v.is_empty());` 직후 `let first = v[0];`

----

<a id="safe-value-extraction-with-unwrap_or"></a>
## `unwrap_or`로 안전하게 값 꺼내기

- `unwrap()`은 `None`이나 `Err`에서 panic합니다. 프로덕션 코드에서는 안전한 대안을 우선 사용하세요.

<a id="the-unwrap-family"></a>
### `unwrap` 계열 메서드
| **메서드** | **None/Err일 때 동작** | **사용 시점** |
|-----------|------------------------|-------------|
| `.unwrap()` | **panic** | 테스트 전용, 혹은 절대 실패하지 않음을 증명할 수 있을 때 |
| `.expect("msg")` | 메시지와 함께 panic | panic이 정당하다면 이유를 함께 적고 싶을 때 |
| `.unwrap_or(default)` | `default` 반환 | 값싼 상수 기본값이 있을 때 |
| `.unwrap_or_else(\|\| expr)` | 클로저 호출 | 기본값 계산 비용이 클 때 |
| `.unwrap_or_default()` | `Default::default()` 반환 | 타입이 `Default`를 구현할 때 |

<a id="real-example-parsing-with-safe-defaults"></a>
### 실제 예: 안전한 기본값을 두고 파싱하기
```rust
// 예시: peripherals.rs
// 정규식 캡처 그룹이 매치되지 않을 수 있으므로 안전한 기본값 제공
let bus_hex = caps.get(1).map(|m| m.as_str()).unwrap_or("00");
let fw_status = caps.get(5).map(|m| m.as_str()).unwrap_or("0x0");
let bus = u8::from_str_radix(bus_hex, 16).unwrap_or(0);
```

<a id="real-example-unwrap_or_else-with-fallback-struct"></a>
### 실제 예: 기본 구조체를 만드는 `unwrap_or_else`
```rust
// 예시: framework.rs
// 전체 함수를 Option을 반환하는 클로저로 감싼 뒤
// 중간에 하나라도 실패하면 기본 구조체를 반환:
(|| -> Option<BaseboardFru> {
    let content = std::fs::read_to_string(path).ok()?;
    let json: serde_json::Value = serde_json::from_str(&content).ok()?;
    // ... .get()? 체인으로 필드 추출
    Some(baseboard_fru)
})()
.unwrap_or_else(|| BaseboardFru {
    manufacturer: String::new(),
    model: String::new(),
    product_part_number: String::new(),
    serial_number: String::new(),
    asset_tag: String::new(),
})
```

<a id="real-example-unwrap_or_default-on-config-deserialization"></a>
### 실제 예: 설정 역직렬화에서 `unwrap_or_default`
```rust
// 예시: framework.rs
// JSON 설정 파싱에 실패해도 Default로 대체 — 크래시 없음
Ok(json) => serde_json::from_str(&json).unwrap_or_default(),
```
이에 대응하는 C++ 코드는 보통 `nlohmann::json::parse()`를 `try/catch`로 감싼 뒤, `catch` 블록에서 수동으로 기본값 구조체를 만들어 반환합니다.

----

<a id="functional-transforms-map-map_err-find_map"></a>
## 함수형 변환: `map`, `map_err`, `find_map`

- `Option`과 `Result`의 이 메서드들을 사용하면 값을 꺼내지 않고도 내부 값을 변환할 수 있어, 중첩된 `if/else` 대신 선형적인 체인을 만들 수 있습니다.

<a id="quick-reference"></a>
### 빠른 참고표
| **메서드** | **대상** | **하는 일** | **C++ 대응 개념** |
|-----------|-------|---------|-------------------|
| `.map(\|v\| ...)` | `Option` / `Result` | `Some`/`Ok` 값을 변환 | `if (opt) { *opt = transform(*opt); }` |
| `.map_err(\|e\| ...)` | `Result` | `Err` 값을 변환 | `catch` 블록에서 문맥 추가 |
| `.and_then(\|v\| ...)` | `Option` / `Result` | `Option`/`Result`를 반환하는 연산을 이어 붙임 | 중첩된 if 검사 |
| `.find_map(\|v\| ...)` | Iterator | `find` + `map`을 한 번에 수행 | `if + break`가 있는 루프 |
| `.filter(\|v\| ...)` | `Option` / Iterator | 조건을 만족하는 값만 유지 | `if (!predicate) return nullopt;` |
| `.ok()?` | `Result` | `Result → Option`으로 바꾸고 `None` 전파 | `if (result.has_error()) return nullopt;` |

<a id="real-example-and_then-chain-for-json-field-extraction"></a>
### 실제 예: JSON 필드 추출을 위한 `.and_then()` 체인
```rust
// 예시: framework.rs — 직렬 번호를 여러 후보에서 순차적으로 찾기
let sys_info = json.get("SystemInfo")?;

// 먼저 BaseboardFru.BoardSerialNumber 시도
if let Some(serial) = sys_info
    .get("BaseboardFru")
    .and_then(|b| b.get("BoardSerialNumber"))
    .and_then(|v| v.as_str())
    .filter(valid_serial)     // 비어 있지 않고 유효한 직렬 번호만 허용
{
    return Some(serial.to_string());
}

// 실패하면 BoardFru.SerialNumber로 폴백
sys_info
    .get("BoardFru")
    .and_then(|b| b.get("SerialNumber"))
    .and_then(|v| v.as_str())
    .filter(valid_serial)
    .map(|s| s.to_string())   // Some일 때만 &str → String 변환
```
C++라면 `if (json.contains("BaseboardFru")) { if (json["BaseboardFru"].contains("BoardSerialNumber")) { ... } }` 같은 피라미드 구조가 되기 쉽습니다.

<a id="real-example-find_map-search-transform-in-one-pass"></a>
### 실제 예: `find_map` - 검색과 변환을 한 번에
```rust
// 예시: context.rs — 센서 + 소유자에 맞는 SDR 레코드 찾기
pub fn find_for_event(&self, sensor_number: u8, owner_id: u8) -> Option<&SdrRecord> {
    self.by_sensor.get(&sensor_number).and_then(|indices| {
        indices.iter().find_map(|&i| {
            let record = &self.records[i];
            if record.sensor_owner_id() == Some(owner_id) {
                Some(record)
            } else {
                None
            }
        })
    })
}
```
`find_map`은 `find`와 `map`을 합친 것입니다. 첫 번째 일치 항목을 찾으면 즉시 멈추고 원하는 형태로 변환합니다. C++에서는 보통 `if`와 `break`가 들어간 `for` 루프로 작성합니다.

<a id="real-example-map_err-for-error-context"></a>
### 실제 예: 에러 문맥을 더하는 `map_err`
```rust
// 예시: main.rs — 에러를 전파하기 전에 문맥 추가
let json_str = serde_json::to_string_pretty(&config)
    .map_err(|e| format!("Failed to serialize config: {}", e))?;
```
이는 `serde_json::Error`를 단순히 넘기지 않고, *무엇을 하다가* 실패했는지 설명하는 `String`으로 바꿔 줍니다.

----

<a id="json-handling-nlohmannjson--serde"></a>
## JSON 처리: `nlohmann::json` → `serde`

- C++ 팀은 JSON 파싱에 `nlohmann::json`을 자주 사용합니다. Rust에서는 **serde** + **serde_json**을 사용하며, JSON 스키마를 *타입 시스템 안에* 담아낸다는 점에서 훨씬 강력합니다.

<a id="c-nlohmann-vs-rust-serde-comparison"></a>
### C++ (`nlohmann`) vs Rust (`serde`) 비교

```cpp
// nlohmann::json을 쓰는 C++ — 런타임 필드 접근
#include <nlohmann/json.hpp>
using json = nlohmann::json;

struct Fan {
    std::string logical_id;
    std::vector<std::string> sensor_ids;
};

Fan parse_fan(const json& j) {
    Fan f;
    f.logical_id = j.at("LogicalID").get<std::string>();    // 없으면 throw
    if (j.contains("SDRSensorIdHexes")) {                   // 기본값 처리를 수동으로 작성
        f.sensor_ids = j["SDRSensorIdHexes"].get<std::vector<std::string>>();
    }
    return f;
}
```

```rust
// serde를 쓰는 Rust — 컴파일 시점 스키마, 자동 필드 매핑
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fan {
    pub logical_id: String,
    #[serde(rename = "SDRSensorIdHexes", default)]  // JSON 키 → Rust 필드
    pub sensor_ids: Vec<String>,                     // 없으면 빈 Vec
    #[serde(default)]
    pub sensor_names: Vec<String>,                   // 없으면 빈 Vec
}

// 한 줄이면 전체 parse 함수 대체 가능:
let fan: Fan = serde_json::from_str(json_str)?;
```

<a id="key-serde-attributes-real-examples-from-production-rust-code"></a>
### 핵심 serde 속성들 (프로덕션 Rust 코드의 실제 예시)

| **속성** | **목적** | **C++ 대응 개념** |
|--------------|------------|--------------------|
| `#[serde(default)]` | 누락된 필드에 `Default::default()` 사용 | `if (j.contains(key)) { ... } else { default; }` |
| `#[serde(rename = "Key")]` | JSON 키 이름을 Rust 필드명에 매핑 | 수동 `j.at("Key")` 접근 |
| `#[serde(flatten)]` | 알 수 없는 키를 `HashMap`으로 흡수 | `for (auto& [k,v] : j.items()) { ... }` |
| `#[serde(skip)]` | 이 필드는 직렬화/역직렬화에서 제외 | JSON에 저장하지 않음 |
| `#[serde(tag = "type")]` | 내부 태그 방식 enum (구분자 필드) | `if (j["type"] == "gpu") { ... }` |

<a id="real-example-full-config-struct"></a>
### 실제 예: 전체 설정 구조체
```rust
// 예시: diag.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagConfig {
    pub sku: SkuConfig,
    #[serde(default)]
    pub level: DiagLevel,            // 없으면 DiagLevel::default()
    #[serde(default)]
    pub modules: ModuleConfig,       // 없으면 ModuleConfig::default()
    #[serde(default)]
    pub output_dir: String,          // 없으면 ""
    #[serde(default, flatten)]
    pub options: HashMap<String, serde_json::Value>,  // 알 수 없는 키를 흡수
}

// 로딩은 3줄이면 충분함 (C++ + nlohmann이라면 보통 20줄 이상)
let content = std::fs::read_to_string(path)?;
let config: DiagConfig = serde_json::from_str(&content)?;
Ok(config)
```

<a id="enum-deserialization-with-serde-tag-type"></a>
### `#[serde(tag = "type")]`를 이용한 enum 역직렬화
```rust
// 예시: components.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]                   // JSON: {"type": "Gpu", "product": ...}
pub enum PcieDeviceKind {
    Gpu { product: GpuProduct, manufacturer: GpuManufacturer },
    Nic { product: NicProduct, manufacturer: NicManufacturer },
    NvmeDrive { drive_type: StorageDriveType, capacity_gb: u32 },
    // ... variant 9개 더
}
// serde가 "type" 필드를 보고 자동으로 분기 — 수동 if/else 체인 불필요
```
C++ 대응 코드는 보통 `if (j["type"] == "Gpu") { parse_gpu(j); } else if (j["type"] == "Nic") { parse_nic(j); } ...` 같은 형태입니다.

<a id="exercise-json-deserialization-with-serde"></a>
# 연습문제: serde를 이용한 JSON 역직렬화

- 아래 JSON을 역직렬화할 수 있는 `ServerConfig` 구조체를 정의하세요.
```json
{
    "hostname": "diag-node-01",
    "port": 8080,
    "debug": true,
    "modules": ["accel_diag", "nic_diag", "cpu_diag"]
}
```
- `#[derive(Deserialize)]`와 `serde_json::from_str()`를 사용해 파싱하세요.
- `debug` 필드에는 `#[serde(default)]`를 붙여, 값이 없을 때 `false`가 되도록 하세요.
- **보너스**: `#[serde(default)]`와 함께 `DiagLevel { Quick, Full, Extended }` enum 필드를 추가하고, 기본값이 `Quick`가 되게 해 보세요.

**시작 코드** (`cargo add serde --features derive` 와 `cargo add serde_json` 필요):
```rust
use serde::Deserialize;

// TODO: Default 구현이 있는 DiagLevel enum 정의

// TODO: serde 속성이 붙은 ServerConfig struct 정의

fn main() {
    let json_input = r#"{
        "hostname": "diag-node-01",
        "port": 8080,
        "debug": true,
        "modules": ["accel_diag", "nic_diag", "cpu_diag"]
    }"#;

    // TODO: 역직렬화한 설정을 출력
    // TODO: "debug" 필드가 없는 JSON도 파싱해 기본값이 false인지 확인
}
```

<details><summary>해답 (클릭하여 펼치기)</summary>

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
enum DiagLevel {
    #[default]
    Quick,
    Full,
    Extended,
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    hostname: String,
    port: u16,
    #[serde(default)]       // 없으면 false가 기본값
    debug: bool,
    modules: Vec<String>,
    #[serde(default)]       // 없으면 DiagLevel::Quick이 기본값
    level: DiagLevel,
}

fn main() {
    let json_input = r#"{
        "hostname": "diag-node-01",
        "port": 8080,
        "debug": true,
        "modules": ["accel_diag", "nic_diag", "cpu_diag"]
    }"#;

    let config: ServerConfig = serde_json::from_str(json_input)
        .expect("Failed to parse JSON");
    println!("{config:#?}");

    // 선택 필드가 빠진 경우도 테스트
    let minimal = r#"{
        "hostname": "node-02",
        "port": 9090,
        "modules": []
    }"#;
    let config2: ServerConfig = serde_json::from_str(minimal)
        .expect("Failed to parse minimal JSON");
    println!("debug (default): {}", config2.debug);    // false
    println!("level (default): {:?}", config2.level);  // Quick
}
// 출력:
// ServerConfig {
//     hostname: "diag-node-01",
//     port: 8080,
//     debug: true,
//     modules: ["accel_diag", "nic_diag", "cpu_diag"],
//     level: Quick,
// }
// debug (default): false
// level (default): Quick
```

</details>

----
