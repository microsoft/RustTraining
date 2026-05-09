<a id="logging-and-tracing-ecosystem"></a>
<a id="logging-and-tracing-syslogprintf--log--tracing"></a>
## 로깅과 트레이싱: syslog/printf → `log` + `tracing`

> **이 장에서 배우는 것:** Rust의 2계층 로깅 아키텍처(파사드 + 백엔드), `log`와 `tracing` 크레이트, span을 이용한 구조적 로깅, 그리고 이것이 `printf`/`syslog` 중심 디버깅을 어떻게 대체하는지 배웁니다.

C++ 진단 코드는 보통 `printf`, `syslog`, 또는 자체 로깅 프레임워크를 사용합니다.
Rust는 표준화된 2계층 로깅 아키텍처를 제공합니다. 즉 **파사드(facade)** 크레이트(`log` 또는 `tracing`)와 **백엔드**(실제 로거 구현체)로 나뉩니다.

<a id="the-log-facade--rusts-universal-logging-api"></a>
### `log` 파사드 - Rust의 범용 로깅 API

`log` 크레이트는 syslog의 심각도 레벨과 비슷한 매크로를 제공합니다. 라이브러리는 `log` 매크로를 사용하고, 바이너리는 원하는 백엔드를 선택합니다.

```rust
// Cargo.toml
// [dependencies]
// log = "0.4"
// env_logger = "0.11"    # 여러 백엔드 중 하나

use log::{info, warn, error, debug, trace};

fn check_sensor(id: u32, temp: f64) {
    trace!("Reading sensor {id}");           // 가장 세밀한 레벨
    debug!("Sensor {id} raw value: {temp}"); // 개발 중 확인용 세부 정보

    if temp > 85.0 {
        warn!("Sensor {id} high temperature: {temp}°C");
    }
    if temp > 95.0 {
        error!("Sensor {id} CRITICAL: {temp}°C — initiating shutdown");
    }
    info!("Sensor {id} check complete");     // 정상 동작 로그
}

fn main() {
    // 백엔드 초기화 - 보통 main()에서 한 번만 수행
    env_logger::init();  // RUST_LOG 환경 변수로 제어

    check_sensor(0, 72.5);
    check_sensor(1, 91.0);
}
```

```bash
# 환경 변수로 로그 레벨 제어
RUST_LOG=debug cargo run          # debug 이상 출력
RUST_LOG=warn cargo run           # warn, error만 출력
RUST_LOG=my_crate=trace cargo run # 모듈별 필터링
RUST_LOG=my_crate::gpu=debug,warn cargo run  # 레벨 혼합
```

<a id="c-comparison"></a>
### C++ 비교

| C++ | Rust (`log`) | 설명 |
|-----|-------------|------|
| `printf("DEBUG: %s\n", msg)` | `debug!("{msg}")` | 포맷이 컴파일 타임에 검사됨 |
| `syslog(LOG_ERR, "...")` | `error!("...")` | 출력 위치는 백엔드가 결정 |
| 로그 호출 주위를 `#ifdef DEBUG`로 감쌈 | `trace!` / `debug!`를 `max_level`에서 제거 | 비활성화되면 zero-cost |
| 직접 만든 `Logger::log(level, msg)` | `log::info!("...")` - 모든 크레이트가 같은 API 사용 | 범용 파사드, 교체 가능한 백엔드 |
| 파일별 로그 상세도 | `RUST_LOG=crate::module=level` | 환경 변수 기반, 재컴파일 불필요 |

<a id="the-tracing-crate--structured-logging-with-spans"></a>
### `tracing` 크레이트 - span을 갖춘 구조적 로깅

`tracing`은 `log`를 **구조적 필드**와 **span**(시간 범위가 있는 스코프)으로 확장합니다.
이것은 특히 컨텍스트를 함께 추적하고 싶은 진단 코드에서 매우 유용합니다.

```rust
// Cargo.toml
// [dependencies]
// tracing = "0.1"
// tracing-subscriber = { version = "0.3", features = ["env-filter"] }

use tracing::{info, warn, error, instrument, info_span};

#[instrument(skip(data), fields(gpu_id = gpu_id, data_len = data.len()))]
fn run_gpu_test(gpu_id: u32, data: &[u8]) -> Result<(), String> {
    info!("Starting GPU test");

    let span = info_span!("ecc_check", gpu_id);
    let _guard = span.enter();  // 이 스코프 안의 모든 로그에 gpu_id가 포함됨

    if data.is_empty() {
        error!(gpu_id, "No test data provided");
        return Err("empty data".to_string());
    }

    // 구조적 필드 - 문자열 보간이 아니라 기계적으로 파싱 가능
    info!(
        gpu_id,
        temp_celsius = 72.5,
        ecc_errors = 0,
        "ECC check passed"
    );

    Ok(())
}

fn main() {
    // tracing subscriber 초기화
    tracing_subscriber::fmt()
        .with_env_filter("debug")  // 또는 RUST_LOG 환경 변수 사용
        .with_target(true)          // 모듈 경로 표시
        .with_thread_ids(true)      // 스레드 ID 표시
        .init();

    let _ = run_gpu_test(0, &[1, 2, 3]);
}
```

`tracing-subscriber` 사용 시 출력 예:
```rust
2026-02-15T10:30:00.123Z DEBUG ThreadId(01) run_gpu_test{gpu_id=0 data_len=3}: my_crate: Starting GPU test
2026-02-15T10:30:00.124Z  INFO ThreadId(01) run_gpu_test{gpu_id=0 data_len=3}:ecc_check{gpu_id=0}: my_crate: ECC check passed gpu_id=0 temp_celsius=72.5 ecc_errors=0
```

<a id="instrument--automatic-span-creation"></a>
### `#[instrument]` - span 자동 생성

`#[instrument]` attribute는 함수 이름과 인자를 포함하는 span을 자동으로 만들어 줍니다.

```rust
use tracing::instrument;

#[instrument]
fn parse_sel_record(record_id: u16, sensor_type: u8, data: &[u8]) -> Result<(), String> {
    // 이 함수 안의 모든 로그에는 자동으로 다음 정보가 포함된다:
    // record_id, sensor_type, 그리고 data(Debug 가능 시)
    tracing::debug!("Parsing SEL record");
    Ok(())
}

// skip: 큰 인자나 민감한 인자를 span에서 제외
// fields: 계산된 필드를 추가
#[instrument(skip(raw_buffer), fields(buf_len = raw_buffer.len()))]
fn decode_ipmi_response(raw_buffer: &[u8]) -> Result<Vec<u8>, String> {
    tracing::trace!("Decoding {} bytes", raw_buffer.len());
    Ok(raw_buffer.to_vec())
}
```

<a id="log-vs-tracing--which-to-use"></a>
### `log` vs `tracing` - 무엇을 써야 할까

| 항목 | `log` | `tracing` |
|--------|-------|-----------|
| **복잡도** | 단순함 - 매크로 5개 | 더 풍부함 - span, field, instrument 제공 |
| **구조적 데이터** | 문자열 보간만 가능 | 키-값 필드 지원: `info!(gpu_id = 0, "msg")` |
| **시간 추적 / span** | 없음 | 있음 - `#[instrument]`, `span.enter()` |
| **Async 지원** | 기본 수준 | 1급 지원 - span이 `.await`를 넘어 전파됨 |
| **호환성** | 범용 파사드 | `log`와 호환 가능 (`log` bridge 제공) |
| **권장 사용처** | 단순한 애플리케이션, 라이브러리 | 진단 도구, async 코드, observability |

> **권장 사항:** 프로덕션용 진단 프로젝트(구조적 출력을 쓰는 진단 도구)에는 `tracing`을 사용하세요. 의존성을 최소화하고 싶은 단순 라이브러리에는 `log`가 적합합니다. `tracing`에는 호환 레이어가 있어서, `log` 매크로를 사용하는 라이브러리도 `tracing` subscriber와 함께 계속 동작합니다.

<a id="backend-options"></a>
### 백엔드 선택지

| 백엔드 크레이트 | 출력 | 사용 사례 |
|--------------|--------|----------|
| `env_logger` | stderr, 컬러 출력 | 개발 환경, 단순 CLI 도구 |
| `tracing-subscriber` | stderr, 포맷된 출력 | `tracing` 기반 프로덕션 |
| `syslog` | 시스템 syslog | Linux 시스템 서비스 |
| `tracing-journald` | systemd journal | systemd가 관리하는 서비스 |
| `tracing-appender` | 회전하는 로그 파일 | 장시간 실행되는 데몬 |
| `tracing-opentelemetry` | OpenTelemetry collector | 분산 트레이싱 |

----

