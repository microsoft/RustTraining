<a id="fourteen-tricks-from-the-trenches"></a>
# 실전에서 건진 열네 가지 요령 🟡

> **이 장에서 배울 내용:** sentinel 제거, sealed trait, 세션 타입, `Pin`, RAII, `#[must_use]`에 이르는 열네 가지 작지만 correct-by-construction 기법 — 각각 거의 비용 없이 특정 버그 클래스를 없앱니다.
>
> **상호 참조:** [ch02](ch02-typed-command-interfaces-request-determi.md)(sealed trait가 ch02 확장), [ch05](ch05-protocol-state-machines-type-state-for-r.md)(typestate 빌더가 ch05 확장), [ch07](ch07-validated-boundaries-parse-dont-validate.md)(FromStr가 ch07 확장)

## 실전에서 건진 열네 가지 요령

핵심 패턴 여덟 가지(ch02–ch09)가 주요 correct-by-construction 기법을 다룹니다. 이 장은 프로덕션 Rust 코드에 **반복해서 등장하는 작지만 가치 높은 요령 열네 가지**를 모았습니다 — 각각 거의 비용 없이 특정 버그 클래스를 제거합니다.

<a id="trick-1-sentinel--option-at-the-boundary"></a>
### 요령 1 — Sentinel을 경계에서 `Option`으로

하드웨어 프로토콜에는 sentinel 값이 가득합니다: IPMI는 "센서 없음"에 `0xFF`, PCI는 "디바이스 없음"에 `0xFFFF`, SMBIOS는 "알 수 없음"에 `0x00`을 씁니다. 이 sentinel을 그냥 정수로 코드 전체에 끌고 가면 모든 소비자가 마법 값을 기억해 검사해야 합니다. 비교 한 곳이라도 빠지면 유령 255°C 읽기나 엉뚱한 벤더 ID 매칭이 생깁니다.

**규칙:** **가장 처음 파싱 경계**에서 sentinel을 `Option`으로 바꾸고, **직렬화 경계**에서만 다시 sentinel으로 되돌립니다.

#### 안티 패턴(`pcie_tree/src/lspci.rs`에서)

```rust,ignore
// Sentinel이 내부에 남음 — 모든 비교가 기억해야 함
let mut current_vendor_id: u16 = 0xFFFF;
let mut current_device_id: u16 = 0xFFFF;

// ... later, parsing fails silently ...
current_vendor_id = u16::from_str_radix(hex, 16)
    .unwrap_or(0xFFFF);  // sentinel hides the error
```

`current_vendor_id`를 받는 모든 함수는 `0xFFFF`가 특수하다는 걸 알아야 합니다. 누군가 먼저 `0xFFFF`를 확인하지 않고 `if vendor_id == target_id`만 쓰면, 잘못된 입력이 `0xFFFF`로 파싱될 때 없는 디바이스가 조용히 매칭됩니다.

#### 올바른 패턴(`nic_sel/src/events.rs`에서)

```rust,ignore
pub struct ThermalEvent {
    pub record_id: u16,
    pub temperature: Option<u8>,  // None if sensor reports 0xFF
}

impl ThermalEvent {
    pub fn from_raw(record_id: u16, raw_temp: u8) -> Self {
        ThermalEvent {
            record_id,
            temperature: if raw_temp != 0xFF {
                Some(raw_temp)
            } else {
                None
            },
        }
    }
}
```

이제 모든 소비자가 `None`을 **반드시** 처리해야 합니다 — 컴파일러가 강제합니다.

```rust,ignore
// 안전 — 컴파일러가 누락 온도 처리를 보장
fn is_overtemp(temp: Option<u8>, threshold: u8) -> bool {
    temp.map_or(false, |t| t > threshold)
}

// None 처리를 잊으면 컴파일 오류:
// fn bad_check(temp: Option<u8>, threshold: u8) -> bool {
//     temp > threshold  // ERROR: can't compare Option<u8> with u8
// }
```

#### 실제 영향

`inventory/src/events.rs`는 GPU 열 알림에 같은 패턴을 씁니다.
```rust,ignore
temperature: if data[1] != 0xFF {
    Some(data[1] as i8)
} else {
    None
},
```

`pcie_tree/src/lspci.rs` 리팩터링은 단순합니다: `current_vendor_id: u16`을
`current_vendor_id: Option<u16>`로 바꾸고, `0xFFFF`를 `None`으로 바꾼 뒤
컴파일러가 고쳐야 할 모든 위치를 찾게 하면 됩니다.

| 이전 | 이후 |
|--------|-------|
| `let mut vendor_id: u16 = 0xFFFF` | `let mut vendor_id: Option<u16> = None` |
| `.unwrap_or(0xFFFF)` | `.ok()` (이미 `Option` 반환) |
| `if vendor_id != 0xFFFF { ... }` | `if let Some(vid) = vendor_id { ... }` |
| 직렬화: `vendor_id` | `vendor_id.unwrap_or(0xFFFF)` |

***

<a id="trick-2-sealed-traits"></a>
### 요령 2 — Sealed Traits

2장은 각 명령을 응답에 묶는 연관 타입으로 `IpmiCmd`를 소개했습니다. 하지만 구멍이 있습니다: **누구나** `IpmiCmd`를 구현할 수 있다면, 누군가 `parse_response`가 잘못된 타입을 반환하거나 패닉하는 `MaliciousCmd`를 쓸 수 있습니다. 전체 시스템의 타입 안전성은 모든 구현이 올바르다는 가정에 달려 있습니다.

**sealed trait**이 이 구멍을 막습니다. 아이디어는 단순합니다: 트레잇이 **오직 크레이트 안에서만** 구현 가능한 *비공개* 슈퍼트레잇을 요구하게 만듭니다.

```rust,ignore
// — 비공개 모듈: 크레이트 밖으로 내보내지 않음 —
mod private {
    pub trait Sealed {}
}

// — 공개 트레잇: Sealed 필요 — 외부에서는 구현 불가 —
pub trait IpmiCmd: private::Sealed {
    type Response;
    fn net_fn(&self) -> u8;
    fn cmd_byte(&self) -> u8;
    fn payload(&self) -> Vec<u8>;
    fn parse_response(&self, raw: &[u8]) -> io::Result<Self::Response>;
}
```

크레이트 안에서는 승인된 각 명령 타입에 대해 `Sealed`를 구현합니다.

```rust,ignore
pub struct ReadTemp { pub sensor_id: u8 }
impl private::Sealed for ReadTemp {}

impl IpmiCmd for ReadTemp {
    type Response = Celsius;
    fn net_fn(&self) -> u8 { 0x04 }
    fn cmd_byte(&self) -> u8 { 0x2D }
    fn payload(&self) -> Vec<u8> { vec![self.sensor_id] }
    fn parse_response(&self, raw: &[u8]) -> io::Result<Celsius> {
        if raw.is_empty() { return Err(io::Error::new(io::ErrorKind::InvalidData, "empty")); }
        Ok(Celsius(raw[0] as f64))
    }
}
```

외부 코드는 `IpmiCmd`를 보고 `execute()`를 호출할 수 있지만 구현할 수는 없습니다.

```rust,ignore
// 다른 크레이트:
struct EvilCmd;
// impl private::Sealed for EvilCmd {}  // ERROR: module `private` is private
// impl IpmiCmd for EvilCmd { ... }     // ERROR: `Sealed` is not satisfied
```

<a id="when-to-seal"></a>
#### 언제 seal할까

| Seal할 때… | Seal하지 말 때… |
|-----------|-----------------|
| 안전성이 올바른 구현에 달림(IpmiCmd, DiagModule) | 사용자가 시스템을 확장해야 함(사용자 정의 보고 포맷터 등) |
| 연관 타입이 불변식을 만족해야 함 | 트레잇이 단순 역량 마커(HasIpmi) |
| 정식 구현 집합을 직접 소유 | 서드파티 플러그인이 설계 목표 |

#### 실제 후보

- `IpmiCmd` — 잘못된 파싱이 타입이 있는 응답을 망가뜨릴 수 있음
- `DiagModule` — 프레임워크가 `run()`이 유효한 DER 레코드를 반환한다고 가정
- `SelEventFilter` — 깨진 필터가 중요한 SEL 이벤트를 삼킬 수 있음

***

<a id="trick-3-non_exhaustive-for-evolving-enums"></a>
### 요령 3 — 진화하는 열거형에 `#[non_exhaustive]`

`inventory/src/types.rs`의 `SkuVariant`는 현재 다섯 변형이 있습니다.

```rust,ignore
pub enum SkuVariant {
    S1001, S2001, S2002, S2003, S3001,
}
```

다음 세대에 `S4001`을 추가하면, 와일드카드 팔이 없는 외부 코드는 **조용히 컴파일에 실패**합니다 — 그게 의도입니다. 내부 코드는요? `#[non_exhaustive]` 없이 *같은 크레이트*의 `match`는 와일드카드 없이 컴파일되고, 새 변형을 추가하면 자신의 빌드가 깨집니다.

열거형에 `#[non_exhaustive]`를 붙이면 그 열거형을 매칭하는 **외부 크레이트**는 와일드카드 팔을 포함해야 합니다. 정의 크레이트 안에서는 `#[non_exhaustive]`가 효과 없음 — 여전히 완전한 매치를 쓸 수 있습니다.

**왜 유용한가:** 라이브러리 크레이트(또는 워크스페이스 공유 서브크레이트)에서 `SkuVariant`를 배포하면 하류 코드가 알 수 없는 미래 변형을 처리하도록 강제됩니다. 다음 세대에 `S4001`을 추가해도 하류는 이미 와일드카드가 있어 컴파일됩니다.

```rust,ignore
// gpu_sel 크레이트(정의 크레이트) 안:
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SkuVariant {
    S1001,
    S2001,
    S2002,
    S2003,
    S3001,
    // 다음 SKU가 나오면 여기에 추가.
    // 외부 소비자는 이미 와일드카드를 가짐 — 깨짐 없음.
}

// gpu_sel 내부 — 완전 매치 허용(와일드카드 불필요):
fn diag_path_internal(sku: SkuVariant) -> &'static str {
    match sku {
        SkuVariant::S1001 => "legacy_gen1",
        SkuVariant::S2001 => "gen2_accel_diag",
        SkuVariant::S2002 => "gen2_alt_diag",
        SkuVariant::S2003 => "gen2_alt_hf_diag",
        SkuVariant::S3001 => "gen3_accel_diag",
        // 정의 크레이트 안에서는 와일드카드 불필요.
        // 여기에 S4001을 추가하면 이 match에서 컴파일 오류 —
        // 원하는 동작 — 업데이트를 강제.
    }
}
```

```rust,ignore
// 바이너리 크레이트(inventory에 의존하는 하류):
fn diag_path_external(sku: inventory::SkuVariant) -> &'static str {
    match sku {
        inventory::SkuVariant::S1001 => "legacy_gen1",
        inventory::SkuVariant::S2001 => "gen2_accel_diag",
        inventory::SkuVariant::S2002 => "gen2_alt_diag",
        inventory::SkuVariant::S2003 => "gen2_alt_hf_diag",
        inventory::SkuVariant::S3001 => "gen3_accel_diag",
        _ => "generic_diag",  // 외부 크레이트는 #[non_exhaustive] 때문에 필수
    }
}
```

> **워크스페이스 팁:** 코드가 모두 한 크레이트에 있으면 `#[non_exhaustive]`는 도움이 안 됩니다 — 크레이트 경계에만 영향을 줍니다. 이 프로젝트처럼 큰 워크스페이스에서는 진화하는 열거형을 공유 크레이트(`core_lib` 또는 `inventory`)에 두어 속성이 다른 워크스페이스 크레이트 소비자를 보호하게 하세요.

#### 후보

| 열거형 | 모듈 | 이유 |
|------|--------|-----|
| `SkuVariant` | `inventory`, `net_inventory` | 세대마다 새 SKU |
| `SensorType` | `protocol_lib` | IPMI 스펙이 0xC0–0xFF를 OEM용으로 예약 |
| `CompletionCode` | `protocol_lib` | 커스텀 BMC 벤더가 코드 추가 |
| `Component` | `event_handler` | 새 하드웨어 범주(최근 NewSoC 추가 등) |

***

<a id="trick-4-typestate-builder"></a>
### 요령 4 — Typestate 빌더

5장은 *프로토콜*(세션 수명, 링크 트레이닝)에 type-state를 보여 주었습니다. 같은 생각은 *빌더*에도 적용됩니다 — 필수 필드가 모두 채워졌을 때만 `build()` / `finish()`를 호출할 수 있는 구조체입니다.

#### 유창(fluent) 빌더의 문제

`diag_framework/src/der.rs`의 `DerBuilder`는 오늘은 이렇게 생겼습니다(단순화):

```rust,ignore
// 현재 유창 빌더 — finish()가 항상 사용 가능
pub struct DerBuilder {
    der: Der,
}

impl DerBuilder {
    pub fn new(marker: &str, fault_code: u32) -> Self { ... }
    pub fn mnemonic(mut self, m: &str) -> Self { ... }
    pub fn fault_class(mut self, fc: &str) -> Self { ... }
    pub fn finish(self) -> Der { self.der }  // ← 항상 호출 가능!
}
```

에러 없이 컴파일되지만 불완전한 DER 레코드를 만듭니다.

```rust,ignore
let bad = DerBuilder::new("CSI_ERR", 62691)
    .finish();  // 앗 — mnemonic, fault_class 없음
```

#### Typestate 빌더: `finish()`는 두 필드 모두 필요

```rust,ignore
pub struct Missing;
pub struct Set<T>(T);

pub struct DerBuilder<Mnemonic, FaultClass> {
    marker: String,
    fault_code: u32,
    mnemonic: Mnemonic,
    fault_class: FaultClass,
    description: Option<String>,
}

// 생성자: 필수 필드 둘 다 Missing으로 시작
impl DerBuilder<Missing, Missing> {
    pub fn new(marker: &str, fault_code: u32) -> Self {
        DerBuilder {
            marker: marker.to_string(),
            fault_code,
            mnemonic: Missing,
            fault_class: Missing,
            description: None,
        }
    }
}

// mnemonic 설정(fault_class 상태와 무관)
impl<FC> DerBuilder<Missing, FC> {
    pub fn mnemonic(self, m: &str) -> DerBuilder<Set<String>, FC> {
        DerBuilder {
            marker: self.marker, fault_code: self.fault_code,
            mnemonic: Set(m.to_string()),
            fault_class: self.fault_class,
            description: self.description,
        }
    }
}

// fault_class 설정(mnemonic 상태와 무관)
impl<MN> DerBuilder<MN, Missing> {
    pub fn fault_class(self, fc: &str) -> DerBuilder<MN, Set<String>> {
        DerBuilder {
            marker: self.marker, fault_code: self.fault_code,
            mnemonic: self.mnemonic,
            fault_class: Set(fc.to_string()),
            description: self.description,
        }
    }
}

// 선택 필드 — 모든 상태에서 사용 가능
impl<MN, FC> DerBuilder<MN, FC> {
    pub fn description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }
}

/// 완전히 빌드된 DER 레코드.
pub struct Der {
    pub marker: String,
    pub fault_code: u32,
    pub mnemonic: String,
    pub fault_class: String,
    pub description: Option<String>,
}

// finish() ONLY available when both required fields are Set
impl DerBuilder<Set<String>, Set<String>> {
    pub fn finish(self) -> Der {
        Der {
            marker: self.marker,
            fault_code: self.fault_code,
            mnemonic: self.mnemonic.0,
            fault_class: self.fault_class.0,
            description: self.description,
        }
    }
}
```

이제 버그가 있는 호출은 컴파일 오류입니다.

```rust,ignore
// ✅ 컴파일 — 필수 필드 둘 다 설정(순서 무관)
let der = DerBuilder::new("CSI_ERR", 62691)
    .fault_class("GPU Module")   // 순서는 상관없음
    .mnemonic("ACCEL_CARD_ER691")
    .description("Thermal throttle")
    .finish();

// ❌ 컴파일 오류 — DerBuilder<Set<String>, Missing>에 finish() 없음
let bad = DerBuilder::new("CSI_ERR", 62691)
    .mnemonic("ACCEL_CARD_ER691")
    .finish();  // ERROR: method `finish` not found
```

#### 언제 typestate 빌더를 쓸까

| 쓸 때… | 굳이 안 써도 될 때… |
|-----------|-------------------|
| 필드 생략이 조용한 버그를 만듦(DER에 mnemonic 없음) | 모든 필드에 타당한 기본값이 있음 |
| 빌더가 공개 API의 일부 | 빌더가 테스트 전용 뼈대 |
| 필수 필드가 2–3개 이상 | 필수 필드가 하나(`new()`에서 받기) |

***

<a id="trick-5-fromstr-as-a-validation-boundary"></a>
### 요령 5 — 검증 경계로서의 `FromStr`

7장은 이진 데이터(FRU, SEL)에 `TryFrom<&[u8]>`을 보여 주었습니다.
**문자열** 입력 — 설정 파일, CLI 인자, JSON 필드 — 에 대응하는 경계는 `FromStr`입니다.

#### 문제

```rust,ignore
// C++ / 검증 없는 Rust: 조용히 기본값으로 떨어짐
fn route_diag(level: &str) -> DiagMode {
    if level == "quick" { ... }
    else if level == "standard" { ... }
    else { QuickMode }  // 설정 오타? ¯\_(ツ)_/¯
}
```

설정에 `"diag_level": "extendedd"`(오타)가 있으면 조용히 `QuickMode`가 됩니다.

#### 패턴(`config_loader/src/diag.rs`에서)

```rust,ignore
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagLevel {
    Quick,
    Standard,
    Extended,
    Stress,
}

impl FromStr for DiagLevel {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "quick"    | "1" => Ok(DiagLevel::Quick),
            "standard" | "2" => Ok(DiagLevel::Standard),
            "extended" | "3" => Ok(DiagLevel::Extended),
            "stress"   | "4" => Ok(DiagLevel::Stress),
            other => Err(format!("unknown diag level: '{other}'")),
        }
    }
}
```

오타는 즉시 잡힙니다.

```rust,ignore
let level: DiagLevel = "extendedd".parse()?;
// Err("unknown diag level: 'extendedd'")
```

#### 세 가지 이점

1. **빠른 실패:** 잘못된 입력이 진단 로직 세 겹 안이 아니라 **파싱 경계**에서 잡힙니다.
2. **별칭이 명시적:** `"MEM"`, `"DIMM"`, `"MEMORY"`가 모두 `Component::Memory`로 매핑 — match 팔이 문서화 역할을 합니다.
3. **`.parse()`가 간결:** `FromStr`이 `str::parse()`와 결합해 한 줄이 됩니다: `let level: DiagLevel = config["level"].parse()?;`

#### 코드베이스 실제 사용

프로젝트에는 이미 `FromStr` 구현이 8개 있습니다.

| 타입 | 모듈 | 주요 별칭 |
|------|--------|----------------|
| `DiagLevel` | `config_loader` | `"1"` = Quick, `"4"` = Stress |
| `Component` | `event_handler` | `"MEM"` / `"DIMM"` = Memory, `"SSD"` / `"NVME"` = Disk |
| `SkuVariant` | `net_inventory` | `"Accel-X1"` = S2001, `"Accel-M1"` = S2002, `"Accel-Z1"` = S3001 |
| `SkuVariant` | `inventory` | 동일 별칭(별도 모듈, 같은 패턴) |
| `FaultStatus` | `config_loader` | Fault lifecycle states |
| `DiagAction` | `config_loader` | Remediation action types |
| `ActionType` | `config_loader` | Action categories |
| `DiagMode` | `cluster_diag` | Multi-node test modes |

`TryFrom`과 대비:

| | `TryFrom<&[u8]>` | `FromStr` |
|---|---|---|
| 입력 | 원시 바이트(이진 프로토콜) | 문자열(설정, CLI, JSON) |
| 전형적 출처 | IPMI, PCIe 설정 공간, FRU | JSON 필드, 환경 변수, 사용자 입력 |
| 장 | ch07 | ch11 |
| 공통 | `Result` — 호출자가 잘못된 입력 처리 |

***

<a id="trick-6-const-generics-for-compile-time-size-validation"></a>
### 요령 6 — 컴파일 타임 크기 검증을 위한 const 제네릭

하드웨어 버퍼, 레지스터 뱅크, 프로토콜 프레임에 고정 크기가 있으면
const 제네릭으로 컴파일러가 강제할 수 있습니다.

```rust,ignore
/// A fixed-size register bank. The size is part of the type.
/// `RegisterBank<256>` and `RegisterBank<4096>` are different types.
pub struct RegisterBank<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> RegisterBank<N> {
    /// Read a register at the given offset.
    /// Compile-time: N is known, so the array size is fixed.
    /// Runtime: only the offset is checked.
    pub fn read(&self, offset: usize) -> Option<u8> {
        self.data.get(offset).copied()
    }
}

// PCIe conventional config space: 256 bytes
type PciConfigSpace = RegisterBank<256>;

// PCIe extended config space: 4096 bytes
type PcieExtConfigSpace = RegisterBank<4096>;

// These are different types — can't accidentally pass one for the other:
fn read_extended_cap(config: &PcieExtConfigSpace, offset: usize) -> Option<u8> {
    config.read(offset)
}
// read_extended_cap(&pci_config, 0x100);
//                   ^^^^^^^^^^^ expected RegisterBank<4096>, found RegisterBank<256> ❌
```

**const 제네릭으로 컴파일 타임 단언:**

```rust,ignore
/// NVMe admin commands use 4096-byte buffers. Enforce at compile time.
pub struct NvmeBuffer<const N: usize> {
    data: Box<[u8; N]>,
}

impl<const N: usize> NvmeBuffer<N> {
    pub fn new() -> Self {
        // 런타임 단언: 512 또는 4096만 허용
        assert!(N == 4096 || N == 512, "NVMe buffers must be 512 or 4096 bytes");
        NvmeBuffer { data: Box::new([0u8; N]) }
    }
}
// NvmeBuffer::<1024>::new();  // 이 형태는 런타임에 패닉
// 진짜 컴파일 타임 강제는 요령 9(const 단언) 참고.
```

> **언제 쓸까:** 고정 크기 프로토콜 버퍼(NVMe, PCIe 설정 공간),
> DMA 디스크립터, 하드웨어 FIFO 깊이. 크기가 런타임에 바뀌면 안 되는 하드웨어 상수인 모든 곳.

***

<a id="trick-7-safe-wrappers-around-unsafe"></a>
### 요령 7 — `unsafe` 주변의 안전 래퍼

프로젝트에는 현재 `unsafe` 블록이 없습니다. 하지만 accel-mgmt/accel-query에
MMIO, DMA, FFI를 추가하면 `unsafe`가 필요합니다. correct-by-construction 접근:
**모든 `unsafe` 블록을 안전한 추상으로 감싸** 위험을 한곳에 모으고 감사 가능하게 만듭니다.

```rust,ignore
/// MMIO 매핑 레지스터. 포인터는 매핑 수명 동안 유효.
/// unsafe는 이 모듈에만 — 호출자는 안전한 메서드만 사용.
pub struct MmioRegion {
    base: *mut u8,
    len: usize,
}

impl MmioRegion {
    /// # Safety
    /// - `base` must be a valid pointer to an MMIO-mapped region
    /// - The region must remain mapped for the lifetime of this struct
    /// - No other code may alias this region
    pub unsafe fn new(base: *mut u8, len: usize) -> Self {
        MmioRegion { base, len }
    }

    /// 안전한 읽기 — 범위 검사로 MMIO 벗어남 방지
    pub fn read_u32(&self, offset: usize) -> Option<u32> {
        if offset + 4 > self.len { return None; }
        // SAFETY: offset is bounds-checked above, base is valid per new() contract
        Some(unsafe {
            core::ptr::read_volatile(self.base.add(offset) as *const u32)
        })
    }

    /// 안전한 쓰기 — 범위 검사로 MMIO 벗어남 방지
    pub fn write_u32(&self, offset: usize, value: u32) -> bool {
        if offset + 4 > self.len { return false; }
        // SAFETY: offset is bounds-checked above, base is valid per new() contract
        unsafe {
            core::ptr::write_volatile(self.base.add(offset) as *mut u32, value);
        }
        true
    }
}
```

**ch09 팬텀 타입과 결합해 타입이 있는 MMIO:**

```rust,ignore
use std::marker::PhantomData;

pub struct ReadOnly;
pub struct ReadWrite;

pub struct TypedMmio<Perm> {
    region: MmioRegion,
    _perm: PhantomData<Perm>,
}

impl TypedMmio<ReadOnly> {
    pub fn read_u32(&self, offset: usize) -> Option<u32> {
        self.region.read_u32(offset)
    }
    // 쓰기 메서드 없음 — ReadOnly 영역에 쓰면 컴파일 오류
}

impl TypedMmio<ReadWrite> {
    pub fn read_u32(&self, offset: usize) -> Option<u32> {
        self.region.read_u32(offset)
    }
    pub fn write_u32(&self, offset: usize, value: u32) -> bool {
        self.region.write_u32(offset, value)
    }
}
```

> **`unsafe` 래퍼 가이드라인:**
>
> | 규칙 | 이유 |
> |------|-----|
> | `# Safety` 불변식을 문서화한 `unsafe fn new()` 하나 | 호출자 책임을 한 번만 |
> | 나머지 메서드는 모두 safe | 호출자가 UB를 유발할 수 없음 |
> | 모든 `unsafe` 블록에 `# SAFETY:` 주석 | 감사자가 국소적으로 검증 |
> | `#[deny(unsafe_op_in_unsafe_fn)]` 모듈로 감싸기 | `unsafe fn` 안에서도 연산마다 `unsafe` 필요 |
> | 래퍼에 `cargo +nightly miri test` 실행 | 메모리 모델 준수 검증 |

---

<a id="checkpoint-tricks-17"></a>
### 체크포인트: 요령 1–7

일곱 가지 실용 요령을 얻었습니다. 빠른 점수표:

| 요령 | 없애는 버그 클래스 | 도입 비용 |
|:-----:|----------------------|:---------------:|
| 1 | Sentinel 혼동(0xFF) | 낮음 — 경계에서 `match` 하나 |
| 2 | 무단 트레잇 구현 | 낮음 — `Sealed` 슈퍼트레잇 추가 |
| 3 | 열거형 성장 후 하류 깨짐 | 낮음 — 한 줄 속성 |
| 4 | 빌더 필드 누락 | 중간 — 추가 타입 매개변수 |
| 5 | 문자열 설정 오타 | 낮음 — `impl FromStr` |
| 6 | 잘못된 버퍼 크기 | 낮음 — const 제네릭 매개변수 |
| 7 | 코드베이스에 `unsafe` 산재 | 중간 — 래퍼 모듈 |

요령 8–14는 **더 고급** — async, const 평가, 세션 타입, `Pin`, `Drop`을 다룹니다. 필요하면 여기서 쉬세요; 위 기법만으로도 내일 바로 쓸 가치 높고 노력 적은 승리입니다.

***

<a id="trick-8-async-type-state-machines"></a>
### 요령 8 — 비동기 Type-State 머신

하드웨어 드라이버가 `async`를 쓰면(예: 비동기 BMC, 비동기 NVMe I/O) type-state는 여전히 통하지만, `.await` 지점을 넘나드는 소유권에 주의해야 합니다.

```rust,ignore
use std::marker::PhantomData;

pub struct Idle;
pub struct Authenticating;
pub struct Active;

pub struct AsyncSession<S> {
    host: String,
    _state: PhantomData<S>,
}

impl AsyncSession<Idle> {
    pub fn new(host: &str) -> Self {
        AsyncSession { host: host.to_string(), _state: PhantomData }
    }

    /// Idle → Authenticating → Active 전환.
    /// 세션은 .await를 넘어 future 안으로 이동(소비)된다.
    pub async fn authenticate(self, user: &str, pass: &str)
        -> Result<AsyncSession<Active>, String>
    {
        // 1단계: 자격 증명 전송(Idle 세션 소비)
        let pending: AsyncSession<Authenticating> = AsyncSession {
            host: self.host,
            _state: PhantomData,
        };

        // 비동기 BMC 인증 시뮬레이션
        // tokio::time::sleep(Duration::from_secs(1)).await;

        // 2단계: Active 세션 반환
        Ok(AsyncSession {
            host: pending.host,
            _state: PhantomData,
        })
    }
}

impl AsyncSession<Active> {
    pub async fn send_command(&mut self, cmd: &[u8]) -> Vec<u8> {
        // 비동기 I/O...
        vec![0x00]
    }
}

// 사용 예:
// let session = AsyncSession::new("192.168.1.100");
// let mut session = session.authenticate("admin", "pass").await?;
// let resp = session.send_command(&[0x04, 0x2D]).await;
```

**비동기 type-state 핵심 규칙:**

| 규칙 | 이유 |
|------|-----|
| 전환 메서드는 `&mut self`가 아니라 값(`self`)으로 받기 | `.await` 너머로 소유권 이동 |
| 복구 가능한 오류는 `Result<NextState, (Error, PrevState)>` | 호출자가 이전 상태에서 재시도 |
| 상태를 여러 future에 쪼개지 말 것 | 한 future가 한 세션 소유 |
| `tokio::spawn`이면 `Send + 'static` bounds | 세션이 스레드 간 이동 가능해야 함 |

> **주의:** 오류 시 재시도하려 **이전** 상태를 돌려받아야 하면
> `Result<AsyncSession<Active>, (Error, AsyncSession<Idle>)>`처럼 반환해
> 호출자가 소유권을 되찾게 하세요. 그렇지 않으면 실패한 `.await` 뒤 세션이 영구히 drop됩니다.

***

<a id="trick-9-refinement-types-via-const-assertions"></a>
### 요령 9 — Const 단언으로 정제 타입(refinement)

숫자 제약이 **런타임 데이터가 아니라** 컴파일 타임 불변식이면
`const` 평가로 강제합니다. 요령 6(타입 수준 크기 구분)과 다릅니다 — 여기서는
컴파일 타임에 *잘못된 값을 거부*합니다.

```rust,ignore
/// A sensor ID that must be in the IPMI SDR range (0x01..=0xFE).
/// The constraint is checked at compile time when `N` is const.
pub struct SdrSensorId<const N: u8>;

impl<const N: u8> SdrSensorId<N> {
    /// Compile-time validation: panics during compilation if N is out of range.
    pub const fn validate() {
        assert!(N >= 0x01, "Sensor ID must be >= 0x01");
        assert!(N <= 0xFE, "Sensor ID must be <= 0xFE (0xFF is reserved)");
    }

    pub const VALIDATED: () = Self::validate();

    pub const fn value() -> u8 { N }
}

// Usage:
fn read_sensor_const<const N: u8>() -> f64 {
    let _ = SdrSensorId::<N>::VALIDATED;  // compile-time check
    // read sensor N...
    42.0
}

// read_sensor_const::<0x20>();   // ✅ compiles — 0x20 is valid
// read_sensor_const::<0x00>();   // ❌ compile error — "Sensor ID must be >= 0x01"
// read_sensor_const::<0xFF>();   // ❌ compile error — 0xFF is reserved
```

**더 단순한 형태 — 팬 ID 상한:**

```rust,ignore
pub struct BoundedFanId<const N: u8>;

impl<const N: u8> BoundedFanId<N> {
    pub const VALIDATED: () = assert!(N < 8, "Server has at most 8 fans (0..7)");

    pub const fn id() -> u8 {
        let _ = Self::VALIDATED;
        N
    }
}

// BoundedFanId::<3>::id();   // ✅
// BoundedFanId::<10>::id();  // ❌ compile error
```

> **언제 쓸까:** 컴파일 타임에 알려진 하드웨어 고정 ID(센서 ID, 팬 슬롯, PCIe 슬롯 번호).
> 값이 런타임 데이터(설정, 사용자 입력)에서 오면 `TryFrom` / `FromStr`(ch07, 요령 5)을 쓰세요.

***

<a id="trick-10-session-types-for-channel-communication"></a>
### 요령 10 — 채널 통신을 위한 세션 타입

두 컴포넌트가 채널로 통신할 때(예: 진단 오케스트레이터 ↔ 워커 스레드)
**세션 타입**이 프로토콜을 타입 시스템에 인코딩합니다.

```rust,ignore
use std::marker::PhantomData;

// 프로토콜: 클라이언트가 Request, 서버가 Response, 그다음 종료.
pub struct SendRequest;
pub struct RecvResponse;
pub struct Done;

/// 타입이 있는 채널 끝점. `S`는 현재 프로토콜 상태.
pub struct Chan<S> {
    // 실제 코드: mpsc::Sender/Receiver 쌍을 감쌈
    _state: PhantomData<S>,
}

impl Chan<SendRequest> {
    /// 요청 전송 — RecvResponse 상태로 전환.
    pub fn send(self, request: DiagRequest) -> Chan<RecvResponse> {
        // ... 채널로 전송 ...
        Chan { _state: PhantomData }
    }
}

impl Chan<RecvResponse> {
    /// 응답 수신 — Done 상태로 전환.
    pub fn recv(self) -> (DiagResponse, Chan<Done>) {
        // ... 채널에서 수신 ...
        (DiagResponse { passed: true }, Chan { _state: PhantomData })
    }
}

impl Chan<Done> {
    /// 채널 닫기 — 프로토콜이 완료된 뒤에만 가능.
    pub fn close(self) { /* drop */ }
}

pub struct DiagRequest { pub test_name: String }
pub struct DiagResponse { pub passed: bool }

// 프로토콜은 반드시 순서대로:
fn orchestrator(chan: Chan<SendRequest>) {
    let chan = chan.send(DiagRequest { test_name: "gpu_stress".into() });
    let (response, chan) = chan.recv();
    chan.close();
    println!("Result: {}", if response.passed { "PASS" } else { "FAIL" });
}

// send 전에 recv 불가:
// fn wrong_order(chan: Chan<SendRequest>) {
//     chan.recv();  // ❌ no method `recv` on Chan<SendRequest>
// }
```

> **언제 쓸까:** 스레드 간 진단 프로토콜, BMC 명령 순서, 순서가 중요한 요청-응답 패턴.
> 복잡한 다중 메시지 프로토콜은 [`session-types`](https://crates.io/crates/session-types)
> 또는 [`rumpsteak`](https://crates.io/crates/rumpsteak) 크레이트를 고려하세요.

***

<a id="trick-11-pin-for-self-referential-state-machines"></a>
### 요령 11 — 자기 참조 상태 머신을 위한 `Pin`

일부 type-state 머신은 자기 데이터 안을 가리키는 참조가 필요합니다(예: 소유 버퍼 안 위치를 추적하는 파서). Rust는 구조체를 옮기면 내부 포인터가 무효가 되어 이를 보통 금지합니다. `Pin<T>`는 값이 **이동되지 않음**을 보장해 이를 해결합니다.

```rust,ignore
use std::pin::Pin;
use std::marker::PhantomPinned;

/// 소유 버퍼 안을 가리키는 참조를 갖는 스트리밍 파서.
/// 고정(pinned)되면 이동할 수 없음 — 내부 참조가 유효로 유지.
pub struct StreamParser {
    buffer: Vec<u8>,
    /// `buffer` 안을 가리킴. 고정된 동안만 유효.
    cursor: *const u8,
    _pin: PhantomPinned,  // Unpin 옵트아웃 — 실수로 unpin 방지
}

impl StreamParser {
    pub fn new(data: Vec<u8>) -> Pin<Box<Self>> {
        let parser = StreamParser {
            buffer: data,
            cursor: std::ptr::null(),
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(parser);

        // 고정된 버퍼 안을 가리키도록 커서 설정
        let cursor = boxed.buffer.as_ptr();
        // SAFETY: we have exclusive access and the parser is pinned
        unsafe {
            let mut_ref = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).cursor = cursor;
        }

        boxed
    }

    /// 다음 바이트 읽기 — Pin<&mut Self>로만 호출 가능.
    pub fn next_byte(self: Pin<&mut Self>) -> Option<u8> {
        // 파서가 이동할 수 없으므로 커서가 유효
        if self.cursor.is_null() { return None; }
        // ... advance cursor through buffer ...
        Some(42) // 스텁
    }
}

// 사용 예:
// let mut parser = StreamParser::new(vec![0x01, 0x02, 0x03]);
// let byte = parser.as_mut().next_byte();
```

**핵심 통찰:** `Pin`은 자기 참조 구조체 문제에 대한 correct-by-construction 해법입니다. 없으면 `unsafe`와 수동 라이프타임 추적이 필요합니다. 있으면 컴파일러가 이동을 막고 내부 포인터 불변식을 유지합니다.

| `Pin`을 쓸 때… | `Pin`을 쓰지 말 때… |
|-----------------|----------------------|
| 상태 머신이 구조체 내부 참조를 씀 | 모든 필드가 독립적으로 소유됨 |
| `.await`를 넘어 빌리는 async future | 자기 참조 불필요 |
| 메모리에서 옮기면 안 되는 DMA 디스크립터 | 데이터를 자유롭게 옮겨도 됨 |
| 내부 커서가 있는 하드웨어 링 버퍼 | 단순 인덱스 순회로 충분 |

***

<a id="trick-12-raii-drop-as-a-correctness-guarantee"></a>
### 요령 12 — 정확성 보장으로서의 RAII / `Drop`

Rust의 `Drop` 트레잇은 correct-by-construction 메커니즘입니다: 정리 코드는 컴파일러가 자동 삽입하므로 **잊을 수 없습니다**. 정확히 한 번 해제해야 하는 하드웨어 리소스에 특히 유용합니다.

```rust,ignore
use std::io;

/// 끝나면 반드시 닫아야 하는 IPMI 세션.
/// `Drop` 구현이 패닉이나 조기 `?` 반환에도 정리를 보장.
pub struct IpmiSession {
    handle: u32,
}

impl IpmiSession {
    pub fn open(host: &str) -> io::Result<Self> {
        // ... negotiate IPMI session ...
        Ok(IpmiSession { handle: 42 })
    }

    pub fn send_raw(&self, _data: &[u8]) -> io::Result<Vec<u8>> {
        Ok(vec![0x00])
    }
}

impl Drop for IpmiSession {
    fn drop(&mut self) {
        // Close Session 명령: 패닉/조기 반환에도 항상 실행.
        // C에서는 CloseSession()을 잊으면 BMC 세션 슬롯이 누수.
        let _ = self.send_raw(&[0x06, 0x3C]);
        eprintln!("[RAII] session {} closed", self.handle);
    }
}
// 사용 예:
fn diagnose(host: &str) -> io::Result<()> {
    let session = IpmiSession::open(host)?;
    session.send_raw(&[0x04, 0x2D, 0x20])?;
    // 명시적 close 불필요 — 여기서 Drop 자동 실행
    Ok(())
    // send_raw가 Err(...)여도 세션은 닫힘.
}
```

**RAII가 없애는 C/C++ 실패 모드:**

```text
C:     session = ipmi_open(host);
       ipmi_send(session, data);
       if (error) return -1;        // 🐛 leaked session — forgot close()
       ipmi_close(session);

Rust:  let session = IpmiSession::open(host)?;
       session.send_raw(data)?;     // ✅ Drop runs on ? return
       // Drop always runs — leak is impossible
```

**순서가 있는 정리를 위해 RAII와 type-state(ch05) 결합:**

제네릭 매개변수마다 `Drop`을 특수화할 수 없습니다(Rust E0366).
대신 상태마다 **별도 래퍼 타입**을 씁니다.

```rust,ignore
use std::marker::PhantomData;

pub struct Open;
pub struct Locked;

pub struct GpuContext<S> {
    device_id: u32,
    _state: PhantomData<S>,
}

impl GpuContext<Open> {
    pub fn lock_clocks(self) -> LockedGpu {
        // ... lock GPU clocks for stable benchmarking ...
        LockedGpu { device_id: self.device_id }
    }
}

/// 잠긴 상태용 별도 타입 — 자체 Drop 보유.
/// `impl Drop for GpuContext<Locked>`는 불가(E0366)이므로
/// 잠긴 리소스를 소유하는 별도 래퍼를 씀.
pub struct LockedGpu {
    device_id: u32,
}

impl LockedGpu {
    pub fn run_benchmark(&self) -> f64 {
        // ... benchmark with locked clocks ...
        42.0
    }
}

impl Drop for LockedGpu {
    fn drop(&mut self) {
        // drop 시 클럭 해제 — 잠긴 래퍼에서만 실행.
        eprintln!("[RAII] GPU {} clocks unlocked", self.device_id);
    }
}

// GpuContext<Open>은 특별한 Drop 없음 — 풀 클럭 없음.
// LockedGpu는 패닉·조기 반환에도 drop 시 항상 언락.
```

> **`impl Drop for GpuContext<Locked>`를 왜 안 쓰나?** Rust는 `Drop` 구현이
> 제네릭 타입의 *모든* 인스턴스에 적용되기를 요구합니다. 상태별 정리를 얻으려면:
>
> | 접근 | 장점 | 단점 |
> |----------|------|------|
> | 별도 래퍼 타입(위) | 깔끔, 제로 코스트 | 타입 이름 추가 |
> | 제네릭 `Drop` + 런타임 `TypeId` 검사 | 단일 타입 | `'static` 필요, 런타임 비용 |
> | `enum` 상태 + `Drop`에서 완전 매치 | 단일 제네릭 타입 | 런타임 디스패치, 타입 안전성 감소 |

> **언제 쓸까:** BMC 세션, GPU 클럭 잠금, DMA 버퍼 매핑, 파일 핸들,
> 뮤텍스 가드, 필수 해제 단계가 있는 모든 리소스. `fn close(&mut self)`나
> `fn cleanup()`을 쓰기 시작했다면 거의 확실히 `Drop`이어야 합니다.

***

<a id="trick-13-error-type-hierarchies-as-correctness"></a>
### 요령 13 — 정확성으로서의 에러 타입 계층

잘 설계된 에러 타입은 에러 삼킴을 막고 호출자가 각 실패 모드를 적절히 처리하게 합니다. 구조화된 에러에 `thiserror`를 쓰는 것은 correct-by-construction 패턴입니다 — 컴파일러가 완전한 매칭을 강제합니다.

```toml
# Cargo.toml
[dependencies]
thiserror = "1"
# 애플리케이션 수준 에러 처리(선택):
# anyhow = "1"
```

```rust,ignore
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DiagError {
    #[error("IPMI communication failed: {0}")]
    Ipmi(#[from] IpmiError),

    #[error("sensor {sensor_id:#04x} reading out of range: {value}")]
    SensorRange { sensor_id: u8, value: f64 },

    #[error("GPU {gpu_id} not responding")]
    GpuTimeout { gpu_id: u32 },

    #[error("configuration invalid: {0}")]
    Config(String),
}

#[derive(Debug, Error)]
pub enum IpmiError {
    #[error("session authentication failed")]
    AuthFailed,

    #[error("command {net_fn:#04x}/{cmd:#04x} timed out")]
    Timeout { net_fn: u8, cmd: u8 },

    #[error("completion code {0:#04x}")]
    CompletionCode(u8),
}

// 호출자는 각 변형을 처리해야 함 — 조용한 삼킴 없음:
fn run_thermal_check() -> Result<(), DiagError> {
    // IpmiError를 반환하면 #[from]으로 DiagError::Ipmi로 자동 변환.
    let temp = read_cpu_temp()?;
    if temp > 105.0 {
        return Err(DiagError::SensorRange {
            sensor_id: 0x20,
            value: temp,
        });
    }
    Ok(())
}

# fn read_cpu_temp() -> Result<f64, DiagError> { Ok(42.0) }
```

**왜 이것이 correct-by-construction인가:**

| 구조화 에러 없음 | `thiserror` 열거형 있음 |
|--------------------------|----------------------|
| `fn op() -> Result<T, String>` | `fn op() -> Result<T, DiagError>` |
| 호출자는 불투명한 문자열만 받음 | 호출자가 구체 변형에 매치 |
| 인증 실패와 타임아웃 구분 불가 | `DiagError::Ipmi(IpmiError::AuthFailed)` vs `Timeout` |
| 로깅이 에러를 삼킴 | `match`가 각 케이스 처리 강제 |
| 새 에러 변형 → 아무도 모름 | 새 변형 → 컴파일러가 매치 누락 경고 |

**`anyhow` vs `thiserror` 선택:**

| `thiserror`를 쓸 때… | `anyhow`를 쓸 때… |
|-----------------------|-------------------|
| 라이브러리/크레이트 작성 | 바이너리/CLI 작성 |
| 호출자가 에러 변형에 매치해야 함 | 호출자는 로그만 하고 종료 |
| 에러 타입이 공개 API의 일부 | 내부 에러 배관 |
| `protocol_lib`, `accel_diag`, `thermal_diag` | `diag_tool` 메인 바이너리 |

> **언제 쓸까:** 워크스페이스의 각 크레이트는 `thiserror`로 자체 에러 열거형을 정의하는 것이 좋습니다. 최상위 바이너리 크레이트는 `anyhow`로 집계할 수 있습니다. 라이브러리 호출자에게 컴파일 타임 에러 처리 보장을 주면서 바이너리는 쓰기 편하게 유지됩니다.

***

<a id="trick-14-must_use-for-enforcing-consumption"></a>
### 요령 14 — 소비 강제를 위한 `#[must_use]`

`#[must_use]` 속성은 무시된 반환값을 컴파일러 경고로 바꿉니다. 이 가이드의 모든 패턴과 짝을 이루는 가벼운 correct-by-construction 도구입니다.

```rust,ignore
/// 반드시 사용해야 하는 보정 토큰 — 조용히 drop하면 버그.
#[must_use = "calibration token must be passed to calibrate(), not dropped"]
pub struct CalibrationToken {
    _private: (),
}

/// 반드시 확인해야 하는 진단 결과 — 실패 무시는 버그.
#[must_use = "diagnostic result must be inspected for failures"]
pub struct DiagResult {
    pub passed: bool,
    pub details: String,
}

/// 중요한 값을 반환하는 함수에도 표시:
#[must_use = "the authenticated session must be used or explicitly closed"]
pub fn authenticate(user: &str, pass: &str) -> Result<Session, AuthError> {
    // ...
#   unimplemented!()
}
#
# pub struct Session;
# pub struct AuthError;
```

**컴파일러가 알려 주는 것:**

```text
warning: unused `CalibrationToken` that must be used
  --> src/main.rs:5:5
   |
5  |     CalibrationToken { _private: () };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: calibration token must be passed to calibrate(), not dropped
```

**`#[must_use]`를 붙일 패턴:**

| 패턴 | 무엇에 붙일지 | 이유 |
|---------|-----------------|-----|
| 단일 사용 토큰(ch03) | `CalibrationToken`, `FusePayload` | 사용 없이 drop = 로직 버그 |
| Capability 토큰(ch04) | `AdminToken` | 인증만 하고 토큰 무시 |
| Type-State 전환 | `authenticate()`, `activate()` 반환 타입 | 세션만 만들고 안 씀 |
| 결과 | `DiagResult`, `SensorReading` | 실패를 조용히 삼킴 |
| RAII 핸들(요령 12) | `IpmiSession`, `LockedGpu` | 열기만 하고 사용 안 함 |

> **경험 법칙:** 사용 없이 drop하는 것이 항상 버그면 `#[must_use]`를 붙입니다. 의도적인 경우가 있으면(예: `Vec`) 붙이지 않습니다. `_` 접두사(`let _ = foo()`)는 경고를 명시적으로 인정하고 끄는 것 — 의도적 drop이면 괜찮습니다.

<a id="key-takeaways"></a>
## 핵심 정리

1. **Sentinel → 경계에서 `Option`** — 파싱 시 마법 값을 `Option`으로 바꿈; 컴파일러가 `None` 처리를 강제.
2. **Sealed trait이 구현 구멍을 막음** — 비공개 슈퍼트레잇으로 크레이트 밖에서는 트레잇 구현 불가.
3. **`#[non_exhaustive]` + `#[must_use]`는 한 줄짜리 고가치 어노테이션** — 진화하는 열거형과 소비되는 토큰에 추가.
4. **Typestate 빌더가 필수 필드를 강제** — 필수 타입 매개변수가 모두 `Set`일 때만 `finish()` 존재.
5. **각 요령이 특정 버그 클래스를 겨냄** — 점진적으로 도입; 아키텍처 전체를 고칠 필요 없음.

---

