# 철학 — 왜 타입이 테스트를 이기는가 🟢

> **이 장에서 배울 내용:** 컴파일 타임 정확성의 세 가지 수준(값, 상태, 프로토콜), 타입 수준 증명 뒤에 있는 Curry–Howard 직관, 그리고 correct-by-construction 패턴이 **투자할 만한 경우와 그렇지 않은 경우**입니다.
>
> **교차 참조:** [ch02](ch02-typed-command-interfaces-request-determi.md)(타입이 있는 명령), [ch05](ch05-protocol-state-machines-type-state-for-r.md)(타입 상태), [ch13](ch13-reference-card.md)(레퍼런스 카드)

<a id="the-cost-of-runtime-checking"></a>
## 런타임 검사의 비용

진단 코드베이스에서 흔한 런타임 가드를 생각해 보세요.

```rust,ignore
fn read_sensor(sensor_type: &str, raw: &[u8]) -> f64 {
    match sensor_type {
        "temperature" => raw[0] as i8 as f64,          // signed byte
        "fan_speed"   => u16::from_le_bytes([raw[0], raw[1]]) as f64,
        "voltage"     => u16::from_le_bytes([raw[0], raw[1]]) as f64 / 1000.0,
        _             => panic!("unknown sensor type: {sensor_type}"),
    }
}
```

이 함수에는 컴파일러가 잡아내지 못하는 **네 가지 실패 모드**가 있습니다.

1. 오타: `"temperture"` → 런타임에 panic
2. 잘못된 `raw` 길이: `fan_speed`인데 1바이트만 있음 → 런타임에 panic
3. 호출자가 반환된 `f64`를 RPM인데 실제로는 °C로 쓰는 경우 → 논리 버그, 조용함
4. 새 센서 타입을 추가했는데 이 `match`를 갱신하지 않음 → 런타임에 panic

모든 실패 모드는 **배포 이후**에야 드러납니다. 테스트는 도움이 되지만, 누군가 작성한 경우만 커버합니다. 타입 시스템은 **모든** 경우를 커버하며, 아무도 상상하지 못한 경우까지 포함합니다.

<a id="three-levels-of-correctness"></a>
## 정확성의 세 가지 수준

<a id="level-1-value-correctness"></a>
### 수준 1 — 값의 정확성
**잘못된 값을 표현할 수 없게 만든다.**

```rust,ignore
// ❌ 어떤 u16이든 "포트"가 될 수 있음 — 0은 잘못됐지만 컴파일은 됨
fn connect(port: u16) { /* ... */ }

// ✅ 검증된 포트만 존재할 수 있음
pub struct Port(u16);  // private field

impl TryFrom<u16> for Port {
    type Error = &'static str;
    fn try_from(v: u16) -> Result<Self, Self::Error> {
        if v > 0 { Ok(Port(v)) } else { Err("port must be > 0") }
    }
}

fn connect(port: Port) { /* ... */ }
// Port(0)은 생성될 수 없음 — 불변식이 어디서나 유지됨
```

**하드웨어 예:** `SensorId(u8)` — 원시 센서 번호를 SDR 범위 안에 있다는 검증과 함께 감쌉니다.

<a id="level-2-state-correctness"></a>
### 수준 2 — 상태의 정확성
**잘못된 전이를 표현할 수 없게 만든다.**

```rust,ignore
use std::marker::PhantomData;

struct Disconnected;
struct Connected;

struct Socket<State> {
    fd: i32,
    _state: PhantomData<State>,
}

impl Socket<Disconnected> {
    fn connect(self, addr: &str) -> Socket<Connected> {
        // ... connect logic ...
        Socket { fd: self.fd, _state: PhantomData }
    }
}

impl Socket<Connected> {
    fn send(&mut self, data: &[u8]) { /* ... */ }
    fn disconnect(self) -> Socket<Disconnected> {
        Socket { fd: self.fd, _state: PhantomData }
    }
}

// Socket<Disconnected>에는 send() 메서드가 없음 — 시도하면 컴파일 에러
```

**하드웨어 예:** GPIO 핀 모드 — `Pin<Input>`에는 `read()`는 있고 `write()`는 없습니다.

<a id="level-3-protocol-correctness"></a>
### 수준 3 — 프로토콜의 정확성
**잘못된 상호작용을 표현할 수 없게 만든다.**

```rust,ignore
use std::io;

trait IpmiCmd {
    type Response;
    fn parse_response(&self, raw: &[u8]) -> io::Result<Self::Response>;
}

// 설명을 위해 단순화함 — net_fn(), cmd_byte(), payload(), parse_response()가
// 포함된 전체 트레잇은 ch02를 참고하세요.

struct ReadTemp { sensor_id: u8 }
impl IpmiCmd for ReadTemp {
    type Response = Celsius;
    fn parse_response(&self, raw: &[u8]) -> io::Result<Celsius> {
        Ok(Celsius(raw[0] as i8 as f64))
    }
}

# #[derive(Debug)] struct Celsius(f64);

fn execute<C: IpmiCmd>(cmd: &C, raw: &[u8]) -> io::Result<C::Response> {
    cmd.parse_response(raw)
}
// ReadTemp는 항상 Celsius를 반환 — 실수로 Rpm을 받을 수 없음
```

**하드웨어 예:** IPMI, Redfish, NVMe Admin 명령 — 요청 타입이 응답 타입을 결정합니다.

<a id="the-curry-howard-connection-simplified"></a>
## Curry–Howard 대응(간단히)

프로그래밍 언어 이론에서 **Curry–Howard 대응**은 타입이 명제이고 프로그램이 증명이라고 말합니다. 다음을 작성할 때:

```rust,ignore
fn execute<C: IpmiCmd>(cmd: &C) -> io::Result<C::Response>
```

단순히 함수를 쓰는 것이 아니라 **정리**를 서술하는 것입니다. "`IpmiCmd`를 구현하는 모든 명령 타입 `C`에 대해, 실행하면 정확히 `C::Response`가 나온다." 컴파일러는 코드를 빌드할 때마다 이 정리를 **증명**합니다. 증명이 실패하면 프로그램은 존재할 수 없습니다.

이론을 몰라도 패턴은 쓸 수 있습니다. 다만 Rust 타입 시스템이 왜 그렇게 강한지 설명해 줍니다 — 실수를 잡는 것이 아니라 **정확성을 증명**하는 것입니다.

<a id="when-not-to-use-these-patterns"></a>
## 이런 패턴을 쓰지 말아야 할 때

correct-by-construction이 항상 정답은 아닙니다.

| 상황 | 권장 |
|-----------|---------------|
| 안전에 중요한 경계(전원 시퀀싱, 암호화) | ✅ 항상 — 여기서 버그는 하드웨어를 망가뜨리거나 비밀을 유출합니다 |
| 모듈 간 공개 API | ✅ 보통 — 잘못된 사용은 컴파일 에러가 되어야 합니다 |
| 상태가 3개 이상인 상태 머신 | ✅ 보통 — 타입 상태가 잘못된 전이를 막습니다 |
| 한 함수 50줄 안의 내부 헬퍼 | ❌ 과함 — 단순 `assert!`로 충분합니다 |
| 프로토타입 / 미지의 하드웨어 탐색 | ❌ 먼저 원시 타입 — 동작을 이해한 뒤 정제합니다 |
| 사용자 대면 CLI 파싱 | ⚠️ 경계에서는 `clap` + `TryFrom`, 내부는 원시 타입도 무방 |

핵심 질문: **"이 버그가 프로덕션에서 나면 얼마나 심각한가?"**

- 팬이 멈춤 → GPU 과열 → **타입 사용**
- 잘못된 DER 레코드 → 고객에게 잘못된 데이터 → **타입 사용**
- 디버그 로그 문구가 약간 어긋남 → **`assert!`면 충분**

<a id="key-takeaways"></a>
## 핵심 정리

1. **정확성의 세 가지 수준** — 값(뉴타입), 상태(타입 상태), 프로토콜(연관 타입) — 각각 더 넓은 버그 클래스를 제거합니다.
2. **실무에서의 Curry–Howard** — 제네릭 함수 시그니처마다 컴파일러가 매 빌드마다 증명하는 정리가 있습니다.
3. **비용 질문** — "이 버그가 배포되면 얼마나 심각한가?"가 타입과 테스트 중 무엇이 맞는 도구인지 가늠합니다.
4. **타입은 테스트를 보완합니다** — 타입은 *범주* 전체를 없애고, 테스트는 특정 *값*과 엣지 케이스를 커버합니다.
5. **멈출 줄 알기** — 내부 헬퍼와 일회성 프로토타입에는 타입 수준 강제가 드물게 필요합니다.

---

