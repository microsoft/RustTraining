<a id="collapsing-assignment-pyramids"></a>
<a id="collapsing-assignment-pyramids-with-closures"></a>
## 클로저로 중첩된 대입 피라미드 줄이기

> **이 장에서 배우는 것:** Rust의 표현식 기반 문법과 클로저를 이용해, 깊게 중첩된 C++ `if/else` 검증 체인을 어떻게 깔끔하고 선형적인 코드로 펼칠 수 있는지 배웁니다.

- C++에서는 변수 대입을 위해 여러 블록으로 나뉜 `if/else` 체인이 자주 필요합니다. 특히 검증이나 fallback 로직이 들어가면 더 그렇습니다. Rust는 표현식 기반 문법과 클로저를 이용해 이런 구조를 평평하고 선형적인 코드로 줄여 줍니다.

<a id="pattern-1-tuple-assignment-with-if-expression"></a>
### 패턴 1: `if` 표현식을 이용한 튜플 대입
```cpp
// C++ — 여러 블록의 if/else 체인에 걸쳐 세 변수를 설정
uint32_t fault_code;
const char* der_marker;
const char* action;
if (is_c44ad) {
    fault_code = 32709; der_marker = "CSI_WARN"; action = "No action";
} else if (error.is_hardware_error()) {
    fault_code = 67956; der_marker = "CSI_ERR"; action = "Replace GPU";
} else {
    fault_code = 32709; der_marker = "CSI_WARN"; action = "No action";
}
```

```rust
// Rust 대응: accel_fieldiag.rs
// 하나의 표현식으로 세 값을 한 번에 대입
let (fault_code, der_marker, recommended_action) = if is_c44ad {
    (32709u32, "CSI_WARN", "No action")
} else if error.is_hardware_error() {
    (67956u32, "CSI_ERR", "Replace GPU")
} else {
    (32709u32, "CSI_WARN", "No action")
};
```

<a id="pattern-2-iife-immediately-invoked-function-expression-for-fallible-chains"></a>
### 패턴 2: 실패 가능한 체인을 위한 IIFE(즉시 호출 함수 표현식)
```cpp
// C++ — JSON 탐색에서 흔한 중첩 피라미드
std::string get_part_number(const nlohmann::json& root) {
    if (root.contains("SystemInfo")) {
        auto& sys = root["SystemInfo"];
        if (sys.contains("BaseboardFru")) {
            auto& bb = sys["BaseboardFru"];
            if (bb.contains("ProductPartNumber")) {
                return bb["ProductPartNumber"].get<std::string>();
            }
        }
    }
    return "UNKNOWN";
}
```

```rust
// Rust 대응: framework.rs
// 클로저 + ? 연산자로 중첩 피라미드를 선형 코드로 줄인다
let part_number = (|| -> Option<String> {
    let path = self.args.sysinfo.as_ref()?;
    let content = std::fs::read_to_string(path).ok()?;
    let json: serde_json::Value = serde_json::from_str(&content).ok()?;
    let ppn = json
        .get("SystemInfo")?
        .get("BaseboardFru")?
        .get("ProductPartNumber")?
        .as_str()?;
    Some(ppn.to_string())
})()
.unwrap_or_else(|| "UNKNOWN".to_string());
```
이 클로저는 각 단계에서 `?`로 조기 반환할 수 있는 `Option<String>` 스코프를 만듭니다. `.unwrap_or_else()`는 마지막에서 한 번만 fallback을 제공합니다.

<a id="pattern-3-iterator-chain-replacing-manual-loop--push_back"></a>
### 패턴 3: 수동 루프 + `push_back`을 대체하는 iterator 체인
```cpp
// C++ — 중간 변수를 두는 수동 루프
std::vector<std::tuple<std::vector<std::string>, std::string, std::string>> gpu_info;
for (const auto& [key, info] : gpu_pcie_map) {
    std::vector<std::string> bdfs;
    // ... bdf_path를 파싱해서 bdfs에 채운다
    std::string serial = info.serial_number.value_or("UNKNOWN");
    std::string model = info.model_number.value_or(model_name);
    gpu_info.push_back({bdfs, serial, model});
}
```

```rust
// Rust 대응: peripherals.rs
// 한 줄 체인: values() → map → collect
let gpu_info: Vec<(Vec<String>, String, String, String)> = self
    .gpu_pcie_map
    .values()
    .map(|info| {
        let bdfs: Vec<String> = info.bdf_path
            .split(')')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim_start_matches('(').to_string())
            .collect();
        let serial = info.serial_number.clone()
            .unwrap_or_else(|| "UNKNOWN".to_string());
        let model = info.model_number.clone()
            .unwrap_or_else(|| model_name.to_string());
        let gpu_bdf = format!("{}:{}:{}.{}",
            info.bdf.segment, info.bdf.bus, info.bdf.device, info.bdf.function);
        (bdfs, serial, model, gpu_bdf)
    })
    .collect();
```

<a id="pattern-4-filtercollect-replacing-loop--if-condition-continue"></a>
### 패턴 4: 루프 + `if (condition) continue`를 대체하는 `.filter().collect()`
```cpp
// C++
std::vector<TestResult*> failures;
for (auto& t : test_results) {
    if (!t.is_pass()) {
        failures.push_back(&t);
    }
}
```

```rust
// Rust — accel_diag/src/healthcheck.rs에서 가져온 예
pub fn failed_tests(&self) -> Vec<&TestResult> {
    self.test_results.iter().filter(|t| !t.is_pass()).collect()
}
```

<a id="summary-when-to-use-each-pattern"></a>
### 요약: 각 패턴을 언제 쓸까
| **C++ 패턴** | **Rust 대체 방식** | **핵심 이점** |
|----------------|-------------------------|------------------------|
| 여러 블록에 걸친 변수 대입 | `let (a, b) = if ... { } else { };` | 모든 변수를 원자적으로 한 번에 바인딩 |
| 중첩된 `if (contains)` 피라미드 | `?` 연산자를 쓰는 IIFE 클로저 | 선형적이고 평평하며 조기 반환 가능 |
| `for` 루프 + `push_back` | `.iter().map(\|\|).collect()` | 중간 `mut Vec`가 필요 없음 |
| `for` + `if (cond) continue` | `.iter().filter(\|\|).collect()` | 의도가 선언적으로 드러남 |
| `for` + `if + break` (첫 번째 항목 찾기) | `.iter().find_map(\|\|)` | 한 번에 검색과 변환 수행 |

----

<a id="capstone-exercise-diagnostic-event-pipeline"></a>
# 캡스톤 연습문제: 진단 이벤트 파이프라인

🔴 **도전** - enum, trait, iterator, 에러 처리, 제네릭을 함께 쓰는 종합 연습문제

이 종합 연습문제는 enum, trait, iterator, 에러 처리, 제네릭을 한데 모아 봅니다. 실제 Rust 프로덕션 코드에서 자주 보이는 패턴과 비슷한, 간단한 진단 이벤트 처리 파이프라인을 직접 만들어 봅니다.

**요구사항:**
1. `Display`를 구현한 `enum Severity { Info, Warning, Critical }`를 정의하고, `source: String`, `severity: Severity`, `message: String`, `fault_code: u32`를 담는 `struct DiagEvent`를 정의하세요.
2. `fn should_include(&self, event: &DiagEvent) -> bool` 메서드를 가진 `trait EventFilter`를 정의하세요.
3. 두 가지 필터를 구현하세요: `SeverityFilter`(지정한 심각도 이상만 통과)와 `SourceFilter`(특정 source 문자열에서 온 이벤트만 통과).
4. **모든** 필터를 통과한 이벤트에 대해서만 포맷된 리포트 문자열을 반환하는 `fn process_events(events: &[DiagEvent], filters: &[&dyn EventFilter]) -> Vec<String>` 함수를 작성하세요.
5. `"source:severity:fault_code:message"` 형식의 문자열을 파싱하는 `fn parse_event(line: &str) -> Result<DiagEvent, String>` 함수를 작성하세요. 입력이 잘못되면 `Err`를 반환해야 합니다.

**시작 코드:**
```rust
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Severity {
    Info,
    Warning,
    Critical,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct DiagEvent {
    source: String,
    severity: Severity,
    message: String,
    fault_code: u32,
}

trait EventFilter {
    fn should_include(&self, event: &DiagEvent) -> bool;
}

struct SeverityFilter {
    min_severity: Severity,
}
// TODO: SeverityFilter에 대해 EventFilter 구현

struct SourceFilter {
    source: String,
}
// TODO: SourceFilter에 대해 EventFilter 구현

fn process_events(events: &[DiagEvent], filters: &[&dyn EventFilter]) -> Vec<String> {
    // TODO: 모든 필터를 통과한 이벤트만 남기고 다음 형식으로 포맷
    // "[SEVERITY] source (FC:fault_code): message"
    todo!()
}

fn parse_event(line: &str) -> Result<DiagEvent, String> {
    // "source:severity:fault_code:message"를 파싱
    // 잘못된 입력이면 Err 반환
    todo!()
}

fn main() {
    let raw_lines = vec![
        "accel_diag:Critical:67956:ECC uncorrectable error detected",
        "nic_diag:Warning:32709:Link speed degraded",
        "accel_diag:Info:10001:Self-test passed",
        "cpu_diag:Critical:55012:Thermal throttling active",
        "accel_diag:Warning:32710:PCIe link width reduced",
    ];

    // 모든 줄을 파싱하고, 성공한 것만 모으며, 에러는 보고한다
    let events: Vec<DiagEvent> = raw_lines.iter()
        .filter_map(|line| match parse_event(line) {
            Ok(e) => Some(e),
            Err(e) => { eprintln!("Parse error: {e}"); None }
        })
        .collect();

    // 필터 적용: accel_diag에서 온 Warning/Critical 이상 이벤트만
    let sev_filter = SeverityFilter { min_severity: Severity::Warning };
    let src_filter = SourceFilter { source: "accel_diag".to_string() };
    let filters: Vec<&dyn EventFilter> = vec![&sev_filter, &src_filter];

    let report = process_events(&events, &filters);
    for line in &report {
        println!("{line}");
    }
    println!("--- {} event(s) matched ---", report.len());
}
```

<details><summary>해답 (클릭하여 펼치기)</summary>

```rust
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Severity {
    Info,
    Warning,
    Critical,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Info => write!(f, "INFO"),
            Severity::Warning => write!(f, "WARNING"),
            Severity::Critical => write!(f, "CRITICAL"),
        }
    }
}

impl Severity {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "Info" => Ok(Severity::Info),
            "Warning" => Ok(Severity::Warning),
            "Critical" => Ok(Severity::Critical),
            other => Err(format!("Unknown severity: {other}")),
        }
    }
}

#[derive(Debug, Clone)]
struct DiagEvent {
    source: String,
    severity: Severity,
    message: String,
    fault_code: u32,
}

trait EventFilter {
    fn should_include(&self, event: &DiagEvent) -> bool;
}

struct SeverityFilter {
    min_severity: Severity,
}

impl EventFilter for SeverityFilter {
    fn should_include(&self, event: &DiagEvent) -> bool {
        event.severity >= self.min_severity
    }
}

struct SourceFilter {
    source: String,
}

impl EventFilter for SourceFilter {
    fn should_include(&self, event: &DiagEvent) -> bool {
        event.source == self.source
    }
}

fn process_events(events: &[DiagEvent], filters: &[&dyn EventFilter]) -> Vec<String> {
    events.iter()
        .filter(|e| filters.iter().all(|f| f.should_include(e)))
        .map(|e| format!("[{}] {} (FC:{}): {}", e.severity, e.source, e.fault_code, e.message))
        .collect()
}

fn parse_event(line: &str) -> Result<DiagEvent, String> {
    let parts: Vec<&str> = line.splitn(4, ':').collect();
    if parts.len() != 4 {
        return Err(format!("Expected 4 colon-separated fields, got {}", parts.len()));
    }
    let fault_code = parts[2].parse::<u32>()
        .map_err(|e| format!("Invalid fault code '{}': {e}", parts[2]))?;
    Ok(DiagEvent {
        source: parts[0].to_string(),
        severity: Severity::from_str(parts[1])?,
        fault_code,
        message: parts[3].to_string(),
    })
}

fn main() {
    let raw_lines = vec![
        "accel_diag:Critical:67956:ECC uncorrectable error detected",
        "nic_diag:Warning:32709:Link speed degraded",
        "accel_diag:Info:10001:Self-test passed",
        "cpu_diag:Critical:55012:Thermal throttling active",
        "accel_diag:Warning:32710:PCIe link width reduced",
    ];

    let events: Vec<DiagEvent> = raw_lines.iter()
        .filter_map(|line| match parse_event(line) {
            Ok(e) => Some(e),
            Err(e) => { eprintln!("Parse error: {e}"); None }
        })
        .collect();

    let sev_filter = SeverityFilter { min_severity: Severity::Warning };
    let src_filter = SourceFilter { source: "accel_diag".to_string() };
    let filters: Vec<&dyn EventFilter> = vec![&sev_filter, &src_filter];

    let report = process_events(&events, &filters);
    for line in &report {
        println!("{line}");
    }
    println!("--- {} event(s) matched ---", report.len());
}
// 출력:
// [CRITICAL] accel_diag (FC:67956): ECC uncorrectable error detected
// [WARNING] accel_diag (FC:32710): PCIe link width reduced
// --- 2 event(s) matched ---
```

</details>

----

