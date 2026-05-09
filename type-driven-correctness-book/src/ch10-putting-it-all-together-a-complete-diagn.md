<a id="putting-it-all-together-a-complete-diagnostic-platform"></a>
# 모두 합치기 — 완전한 진단 플랫폼 🟡

> **이 장에서 배울 내용:** 2–9장의 핵심 패턴 일곱 가지(typed commands, capability token, type-state, single-use types, dimensional types, validated boundaries, phantom types)가 **하나의 진단 워크플로**로 어떻게 조합되는지 — 인증, 세션, 타입이 있는 명령, 감사 토큰, 차원이 있는 결과, 검증된 데이터, 팬텀 타입 레지스터 — **런타임 오버헤드 합계는 0**입니다.
>
> **상호 참조:** 핵심 패턴 장 전부(ch02–ch09), [ch14](ch14-testing-type-level-guarantees.md)(이 보장들을 테스트하는 방법)

<a id="goal"></a>
## 목표

이 장은 2–9장의 **패턴 일곱 가지**를 **하나의 현실적인 진단 워크플로**로 묶습니다. 다음을 수행하는 서버 헬스 체크를 만듭니다.

1. **인증**(capability token — ch04)
2. **IPMI 세션 열기**(type-state — ch05)
3. **타입이 있는 명령 전송**(typed commands — ch02)
4. 감사 로깅에 **단일 사용 토큰**(single-use types — ch03)
5. **차원이 있는 결과** 반환(dimensional analysis — ch06)
6. **FRU 데이터 검증**(validated boundaries — ch07)
7. **타입이 있는 레지스터 읽기**(phantom types — ch09)

```rust,ignore
use std::marker::PhantomData;
use std::io;
// ──── 패턴 1: 차원 타입(ch06) ────

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Celsius(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rpm(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Volts(pub f64);

// ──── 패턴 2: 타입이 있는 명령(ch02) ────

/// ch02와 같은 트레잇 형태이며, 일관성을 위해 메서드를 사용합니다(연관 상수 아님).
/// 값이 타입마다 고정이면 연관 상수(`const NETFN: u8`)도 동일하게 유효한 대안입니다.
pub trait IpmiCmd {
    type Response;
    fn net_fn(&self) -> u8;
    fn cmd_byte(&self) -> u8;
    fn payload(&self) -> Vec<u8>;
    fn parse_response(&self, raw: &[u8]) -> io::Result<Self::Response>;
}

pub struct ReadTemp { pub sensor_id: u8 }
impl IpmiCmd for ReadTemp {
    type Response = Celsius;   // ← 차원 타입!
    fn net_fn(&self) -> u8 { 0x04 }
    fn cmd_byte(&self) -> u8 { 0x2D }
    fn payload(&self) -> Vec<u8> { vec![self.sensor_id] }
    fn parse_response(&self, raw: &[u8]) -> io::Result<Celsius> {
        if raw.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "empty"));
        }
        Ok(Celsius(raw[0] as f64))
    }
}

pub struct ReadFanSpeed { pub fan_id: u8 }
impl IpmiCmd for ReadFanSpeed {
    type Response = Rpm;
    fn net_fn(&self) -> u8 { 0x04 }
    fn cmd_byte(&self) -> u8 { 0x2D }
    fn payload(&self) -> Vec<u8> { vec![self.fan_id] }
    fn parse_response(&self, raw: &[u8]) -> io::Result<Rpm> {
        if raw.len() < 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "need 2 bytes"));
        }
        Ok(Rpm(u16::from_le_bytes([raw[0], raw[1]]) as f64))
    }
}

// ──── 패턴 3: Capability Token(ch04) ────

pub struct AdminToken { _private: () }

pub fn authenticate(user: &str, pass: &str) -> Result<AdminToken, &'static str> {
    if user == "admin" && pass == "secret" {
        Ok(AdminToken { _private: () })
    } else {
        Err("authentication failed")
    }
}

// ──── 패턴 4: Type-State 세션(ch05) ────

pub struct Idle;
pub struct Active;

pub struct Session<State> {
    host: String,
    _state: PhantomData<State>,
}

impl Session<Idle> {
    pub fn connect(host: &str) -> Self {
        Session { host: host.to_string(), _state: PhantomData }
    }

    pub fn activate(
        self,
        _admin: &AdminToken,  // ← capability token 필요
    ) -> Result<Session<Active>, String> {
        println!("Session activated on {}", self.host);
        Ok(Session { host: self.host, _state: PhantomData })
    }
}

impl Session<Active> {
    /// 타입이 있는 명령 실행 — Active 세션에서만 사용 가능.
    /// ch02와 일관되게 전송 오류 전파를 위해 io::Result를 반환합니다.
    pub fn execute<C: IpmiCmd>(&mut self, cmd: &C) -> io::Result<C::Response> {
        let raw_response = self.raw_send(cmd.net_fn(), cmd.cmd_byte(), &cmd.payload())?;
        cmd.parse_response(&raw_response)
    }

    fn raw_send(&self, _nf: u8, _cmd: u8, _data: &[u8]) -> io::Result<Vec<u8>> {
        Ok(vec![42, 0x1E]) // 스텁: 원시 IPMI 응답
    }

    pub fn close(self) { println!("Session closed"); }
}

// ──── 패턴 5: 단일 사용 감사 토큰(ch03) ────

/// 각 진단 실행마다 고유한 감사 토큰.
/// Clone, Copy 아님 — 감사 항목이 유일하도록 보장.
pub struct AuditToken {
    run_id: u64,
}

impl AuditToken {
    pub fn issue(run_id: u64) -> Self {
        AuditToken { run_id }
    }

    /// 토큰을 소비해 감사 로그 항목을 씁니다.
    pub fn log(self, message: &str) {
        println!("[AUDIT run_id={}] {}", self.run_id, message);
        // 토큰 소비 — 같은 run_id로 두 번 로그할 수 없음
    }
}

// ──── 패턴 6: 검증된 경계(ch07) ────
// ch07의 전체 ValidFru에서 단순화 — 이 복합 예제에 필요한 필드만.
// 전체 TryFrom<RawFruData> 버전은 ch07을 참고하세요.

pub struct ValidFru {
    pub board_serial: String,
    pub product_name: String,
}

impl ValidFru {
    pub fn parse(raw: &[u8]) -> Result<Self, &'static str> {
        if raw.len() < 8 { return Err("FRU too short"); }
        if raw[0] != 0x01 { return Err("bad FRU version"); }
        Ok(ValidFru {
            board_serial: "SN12345".to_string(),  // 스텁
            product_name: "ServerX".to_string(),
        })
    }
}

// ──── 패턴 7: 팬텀 타입 레지스터(ch09) ────

pub struct Width16;
pub struct Reg<W> { offset: u16, _w: PhantomData<W> }

impl Reg<Width16> {
    pub fn read(&self) -> u16 { 0x8086 } // 스텁
}

pub struct PcieDev {
    pub vendor_id: Reg<Width16>,
    pub device_id: Reg<Width16>,
}

impl PcieDev {
    pub fn new() -> Self {
        PcieDev {
            vendor_id: Reg { offset: 0x00, _w: PhantomData },
            device_id: Reg { offset: 0x02, _w: PhantomData },
        }
    }
}

// ──── 복합 워크플로 ────

fn full_diagnostic() -> Result<(), String> {
    // 1. 인증 → capability token 획득
    let admin = authenticate("admin", "secret")
        .map_err(|e| e.to_string())?;

    // 2. 연결 및 세션 활성화(type-state: Idle → Active)
    let session = Session::connect("192.168.1.100");
    let mut session = session.activate(&admin)?;  // AdminToken 필요

    // 3. 타입이 있는 명령 전송(응답 타입이 명령과 일치)
    let temp: Celsius = session.execute(&ReadTemp { sensor_id: 0 })
        .map_err(|e| e.to_string())?;
    let fan: Rpm = session.execute(&ReadFanSpeed { fan_id: 1 })
        .map_err(|e| e.to_string())?;

    // 타입 불일치는 여기서 잡힘:
    // let wrong: Volts = session.execute(&ReadTemp { sensor_id: 0 })?;
    //  ❌ ERROR: expected Celsius, found Volts

    // 4. 팬텀 타입 PCIe 레지스터 읽기
    let pcie = PcieDev::new();
    let vid: u16 = pcie.vendor_id.read();  // u16 보장

    // 5. 경계에서 FRU 데이터 검증
    let raw_fru = vec![0x01, 0x00, 0x00, 0x01, 0x01, 0x00, 0x00, 0xFD];
    let fru = ValidFru::parse(&raw_fru)
        .map_err(|e| e.to_string())?;

    // 6. 단일 사용 감사 토큰 발급
    let audit = AuditToken::issue(1001);

    // 7. 보고서 생성(모든 데이터가 타입이 있고 검증됨)
    let report = format!(
        "Server: {} (SN: {}), VID: 0x{:04X}, CPU: {:?}, Fan: {:?}",
        fru.product_name, fru.board_serial, vid, temp, fan,
    );

    // 8. 감사 토큰 소비 — 두 번 로그할 수 없음
    audit.log(&report);
    // audit.log("oops");  // ❌ use of moved value

    // 9. 세션 종료(type-state: Active → drop)
    session.close();

    Ok(())
}
```

<a id="what-the-compiler-proves"></a>
### 컴파일러가 증명하는 것

| 버그 종류 | 방지 방법 | 패턴 |
|-----------|-----------|------|
| 미인증 접근 | `activate()`에 `&AdminToken` 필요 | Capability token |
| 잘못된 세션 상태에서 명령 | `execute()`는 `Session<Active>`에만 존재 | Type-state |
| 잘못된 응답 타입 | `ReadTemp::Response = Celsius`, 트레잇으로 고정 | Typed commands |
| 단위 혼동(°C vs RPM) | `Celsius` ≠ `Rpm` ≠ `Volts` | Dimensional types |
| 레지스터 폭 불일치 | `Reg<Width16>`이 `u16` 반환 | Phantom types |
| 검증되지 않은 데이터 처리 | 먼저 `ValidFru::parse()` 호출 필요 | Validated boundary |
| 감사 항목 중복 | `log`에서 `AuditToken` 소비 | Single-use type |
| 전원 시퀀싱 순서 위반 | 각 단계가 이전 토큰 필요 | Capability tokens(ch04) |

**이 모든 보장의 합계 런타임 오버헤드: 0입니다.**

모든 검사는 컴파일 타임에 일어납니다. 생성된 어셈블리는 검사 없는 순수 C 코드와 동일하지만, **C는 버그가 있을 수 있고 여기서는 없습니다.**

<a id="key-takeaways"></a>
## 핵심 정리

1. **일곱 패턴이 매끄럽게 조합됩니다** — capability token, type-state, typed commands, single-use types, dimensional types, validated boundaries, phantom types가 함께 동작합니다.
2. **컴파일러가 여덟 가지 버그 클래스를 불가능함을 증명합니다** — 위 "컴파일러가 증명하는 것" 표를 참고하세요.
3. **합계 런타임 오버헤드는 0** — 생성된 어셈블리는 검사 없는 C 코드와 같습니다.
4. **각 패턴은 그 자체로 유용합니다** — 일곱 가지를 모두 쓸 필요는 없습니다. 점진적으로 도입하세요.
5. **통합 장은 설계 템플릿입니다** — 자신만의 타입이 있는 진단 워크플로의 출발점으로 쓰세요.
6. **IPMI에서 Redfish로 확장** — ch17과 ch18은 같은 일곱 패턴(여기에 ch08 capability mixin 추가)을 전체 Redfish 클라이언트·서버에 적용합니다. 여기 IPMI 워크플로는 기초이고, Redfish 워크스루는 여러 데이터 소스와 스키마 버전 제약이 있는 프로덕션 시스템으로 조합이 어떻게 커지는지 보여 줍니다.

---

