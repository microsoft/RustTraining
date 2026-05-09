<a id="testing-type-level-guarantees"></a>
# 타입 수준 보장 테스트하기 🟡

> **배울 내용:** 잘못된 코드가 *컴파일되지 않아야 한다*는 것을 어떻게 테스트하는지(trybuild), 검증된 경계를 어떻게 퍼즈하는지(proptest), RAII 불변식을 어떻게 검증하는지, 그리고 `cargo-show-asm`으로 제로 코스트 추상화를 어떻게 증명하는지.
>
> **교차 참조:** [ch03](ch03-single-use-types-cryptographic-guarantee.md)(논스에 대한 compile-fail), [ch07](ch07-validated-boundaries-parse-dont-validate.md)(경계에 대한 proptest), [ch05](ch05-protocol-state-machines-type-state-for-r.md)(세션에 대한 RAII)

<a id="testing-type-level-guarantees-section"></a>
## 타입 수준 보장 테스트하기

Correct-by-construction 패턴은 버그를 런타임에서 컴파일 타임으로 옮깁니다. 그런데
**잘못된 코드가 실제로 컴파일에 실패하는지**는 어떻게 테스트할까요? 검증된 경계가
퍼징 아래에서도 유지되는지는 어떻게 보장할까요? 이 장은 타입 수준 정확성을 보완하는
테스트 도구를 다룹니다.

<a id="compile-fail-tests-with-trybuild"></a>
### `trybuild`로 컴파일 실패 테스트

[`trybuild`](https://crates.io/crates/trybuild) 크레이트는 특정 코드가 **컴파일되지 않아야 한다**고
단언할 수 있게 해줍니다. 리팩터링 후에도 타입 수준 불변식을 유지하려면 필수입니다 —
누군가 실수로 단일 사용 `Nonce`에 `Clone`을 추가하면 compile-fail 테스트가 잡아냅니다.

**설정:**

```toml
# Cargo.toml
[dev-dependencies]
trybuild = "1"
```

**테스트 파일 (`tests/compile_fail.rs`):**

```rust,ignore
#[test]
fn type_safety_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
```

**테스트 케이스: 논스 재사용은 컴파일되면 안 됨 (`tests/ui/nonce_reuse.rs`):**

```rust,ignore
// tests/ui/nonce_reuse.rs
use my_crate::Nonce;

fn main() {
    let nonce = Nonce::new();
    encrypt(nonce);
    encrypt(nonce); // should fail: use of moved value
}

fn encrypt(_n: Nonce) {}
```

**예상 에러 (`tests/ui/nonce_reuse.stderr`):**

```text
error[E0382]: use of moved value: `nonce`
 --> tests/ui/nonce_reuse.rs:6:13
  |
4 |     let nonce = Nonce::new();
  |         ----- move occurs because `nonce` has type `Nonce`, which does not implement the `Copy` trait
5 |     encrypt(nonce);
  |             ----- value moved here
6 |     encrypt(nonce); // should fail: use of moved value
  |             ^^^^^ value used here after move
```

**장별 compile-fail 테스트 케이스 추가:**

| 패턴 (장) | 테스트 단언 | 파일 |
|-------------------|---------------|------|
| Single-Use Nonce (ch03) | 논스를 두 번 쓸 수 없음 | `nonce_reuse.rs` |
| Capability Token (ch04) | 토큰 없이 `admin_op()` 호출 불가 | `missing_token.rs` |
| Type-State (ch05) | `Session<Idle>`에서 `send_command()` 불가 | `wrong_state.rs` |
| Dimensional (ch06) | `Celsius + Rpm` 불가 | `unit_mismatch.rs` |
| Sealed Trait (Trick 2) | 외부 크레이트에서 sealed 트레잇 impl 불가 | `unseal_attempt.rs` |
| Non-Exhaustive (Trick 3) | 외부 match에서 와일드카드 없으면 실패 | `missing_wildcard.rs` |

**CI 연동:**

```yaml
# .github/workflows/ci.yml
- name: Run compile-fail tests
  run: cargo test --test compile_fail
```

<a id="property-based-testing-of-validated-boundaries"></a>
### 검증된 경계에 대한 프로퍼티 기반 테스트

검증된 경계(ch07)는 데이터를 한 번 파싱하고 잘못된 입력은 거부합니다. 그런데
검증이 **모든** 잘못된 입력을 잡는지 어떻게 알까요? [`proptest`](https://crates.io/crates/proptest)를
쓴 프로퍼티 기반 테스트는 수천 개의 무작위 입력을 생성해 경계를 압박합니다:

```toml
# Cargo.toml
[dev-dependencies]
proptest = "1"
```

```rust,ignore
use proptest::prelude::*;

/// ch07에서: ValidFru는 스펙에 맞는 FRU 페이로드를 감쌉니다.
/// 이 테스트는 ch07의 ValidFru 전체를 사용하며 board_area(),
/// product_area(), format_version() 메서드를 호출합니다.
/// 참고: ch07은 TryFrom<RawFruData>를 정의하므로 먼저 원시 바이트로 감쌉니다.

proptest! {
    /// 검증을 통과한 임의의 바이트 시퀀스는 패닉 없이 사용 가능해야 함.
    #[test]
    fn valid_fru_never_panics(data in proptest::collection::vec(any::<u8>(), 0..1024)) {
        if let Ok(fru) = ValidFru::try_from(RawFruData(data)) {
            // 검증된 FRU에서는 절대 패닉하면 안 됨
            // (ch07의 ValidFru impl 메서드):
            let _ = fru.format_version();
            let _ = fru.board_area();
            let _ = fru.product_area();
        }
    }

    /// 라운드트립: format_version은 재파싱 후에도 보존됨.
    #[test]
    fn fru_round_trip(data in valid_fru_strategy()) {
        let raw = RawFruData(data.clone());
        let fru = ValidFru::try_from(raw).unwrap();
        let version = fru.format_version();
        // 같은 바이트를 다시 파싱 — 버전은 동일해야 함
        let reparsed = ValidFru::try_from(RawFruData(data)).unwrap();
        prop_assert_eq!(version, reparsed.format_version());
    }
}

/// 사용자 정의 전략: FRU 스펙 헤더를 만족하는 바이트 벡터 생성.
/// 헤더 형식은 ch07의 `TryFrom<RawFruData>` 검증과 일치:
///   - 바이트 0: version = 0x01
///   - 바이트 1–6: 영역 오프셋 (×8 = 실제 바이트 오프셋)
///   - 바이트 7: 체크섬 (바이트 0–7의 합 ≡ 0 mod 256)
/// 본문은 무작위이지만 오프셋이 범위 안에 들어갈 만큼 충분히 큼.
fn valid_fru_strategy() -> impl Strategy<Value = Vec<u8>> {
    let header = vec![0x01, 0x00, 0x01, 0x02, 0x00, 0x00, 0x00];
    proptest::collection::vec(any::<u8>(), 64..256)
        .prop_map(move |body| {
            let mut fru = header.clone();
            let sum: u8 = fru.iter().fold(0u8, |a, &b| a.wrapping_add(b));
            fru.push(0u8.wrapping_sub(sum));
            fru.extend_from_slice(&body);
            fru
        })
}
```

**Correct-by-construction 코드를 위한 테스트 피라미드:**

```text
┌───────────────────────────────────┐
│    컴파일 실패 테스트 (trybuild)   │ ← "잘못된 코드는 컴파일되면 안 됨"
├───────────────────────────────────┤
│  프로퍼티 테스트 (proptest/quickcheck) │ ← "유효한 입력은 절대 패닉 없음"
├───────────────────────────────────┤
│    단위 테스트 (#[test])           │ ← "특정 입력은 기대한 출력"
├───────────────────────────────────┤
│    타입 시스템 (ch02–13 패턴)      │ ← "전체 버그 클래스가 존재할 수 없음"
└───────────────────────────────────┘
```

<a id="raii-verification"></a>
### RAII 검증

RAII(Trick 12)는 정리(cleanup)를 보장합니다. 이를 테스트하려면 `Drop` 구현이
실제로 호출되는지 확인합니다:

```rust,ignore
use std::sync::atomic::{AtomicBool, Ordering};

// 참고: 이 테스트는 전역 AtomicBool을 쓰므로 서로 병렬로 실행되면 안 됩니다.
// `#[serial_test::serial]`을 쓰거나 `cargo test -- --test-threads=1`로 실행하세요.
// 또는 전역을 아예 쓰지 않고 클로저로 `Arc<AtomicBool>`을 넘기는 방법도 있습니다.
static DROPPED: AtomicBool = AtomicBool::new(false);

struct TestSession;
impl Drop for TestSession {
    fn drop(&mut self) {
        DROPPED.store(true, Ordering::SeqCst);
    }
}

#[test]
fn session_drops_on_early_return() {
    DROPPED.store(false, Ordering::SeqCst);
    let result: Result<(), &str> = (|| {
        let _session = TestSession;
        Err("simulated failure")?;
        Ok(())
    })();
    assert!(result.is_err());
    assert!(DROPPED.load(Ordering::SeqCst), "Drop must fire on early return");
}

#[test]
fn session_drops_on_panic() {
    DROPPED.store(false, Ordering::SeqCst);
    let result = std::panic::catch_unwind(|| {
        let _session = TestSession;
        panic!("simulated panic");
    });
    assert!(result.is_err());
    assert!(DROPPED.load(Ordering::SeqCst), "Drop must fire on panic");
}
```

<a id="applying-to-your-codebase"></a>
### 코드베이스에 적용하기

워크스페이스에 타입 수준 테스트를 추가할 때 우선순위 계획입니다:

| 크레이트 | 테스트 유형 | 무엇을 테스트할지 |
|-------|-----------|-------------|
| `protocol_lib` | Compile-fail | `Session<Idle>`는 `send_command()` 불가 |
| `protocol_lib` | Property | 임의 바이트 시퀀스 → `TryFrom`은 성공하거나 Err (패닉 없음) |
| `thermal_diag` | Compile-fail | `HasSpi` mixin 없이 `FanReading` 생성 불가 |
| `accel_diag` | Property | GPU 센서 파싱: 무작위 바이트 → 검증 또는 거부 |
| `config_loader` | Property | 무작위 문자열 → `DiagLevel`의 `FromStr`은 절대 패닉 없음 |
| `pci_topology` | Compile-fail | `Register<Width16>`을 `Width32` 자리에 넘길 수 없음 |
| `event_handler` | Compile-fail | 감사 토큰은 복제 불가 |
| `diag_framework` | Compile-fail | `DerBuilder<Missing, _>`는 `finish()` 호출 불가 |

<a id="zero-cost-abstraction-proof-by-assembly"></a>
### 제로 코스트 추상화: 어셈블리로 증명

흔한 의문: "뉴타입과 팬텀 타입이 런타임 오버헤드를 더하나요?"
답은 **아니오** — 원시 프리미티브와 동일한 어셈블리로 컴파일됩니다.
검증 방법은 다음과 같습니다:

**설치:**

```bash
cargo install cargo-show-asm
```

**예: 뉴타입 vs 원시 u32:**

```rust,ignore
// src/lib.rs
#[derive(Clone, Copy)]
pub struct Rpm(pub u32);

#[derive(Clone, Copy)]
pub struct Celsius(pub f64);

// 뉴타입 산술
#[inline(never)]
pub fn add_rpm(a: Rpm, b: Rpm) -> Rpm {
    Rpm(a.0 + b.0)
}

// 원시 산술 (비교용)
#[inline(never)]
pub fn add_raw(a: u32, b: u32) -> u32 {
    a + b
}
```

**실행:**

```bash
cargo asm my_crate::add_rpm
cargo asm my_crate::add_raw
```

**결과 — 동일한 어셈블리:**

```asm
; add_rpm (newtype)           ; add_raw (raw u32)
my_crate::add_rpm:            my_crate::add_raw:
  lea eax, [rdi + rsi]         lea eax, [rdi + rsi]
  ret                          ret
```

`Rpm` 래퍼는 컴파일 타임에 완전히 사라집니다. `PhantomData<S>`(0바이트), ZST 토큰(0바이트),
이 가이드 전반의 다른 타입 수준 마커도 마찬가지입니다.

**직접 타입에 대해 검증:**

```bash
# 특정 함수의 어셈블리 표시
cargo asm --lib ipmi_lib::session::execute

# PhantomData가 0바이트를 더하는지 표시
cargo asm --lib --rust ipmi_lib::session::IpmiSession
```

> **핵심:** 이 가이드의 모든 패턴은 **런타임 비용이 0**입니다.
> 타입 시스템이 모든 일을 하고 컴파일 중에 완전히 지워집니다.
> Haskell의 안전성과 C의 성능을 함께 얻는 셈입니다.

<a id="key-takeaways"></a>
## 핵심 정리

1. **trybuild는 잘못된 코드가 컴파일되지 않음을 테스트** — 리팩터 후에도 타입 수준 불변식을 유지하는 데 필수입니다.
2. **proptest는 검증 경계를 퍼즈** — 수천 개의 무작위 입력으로 `TryFrom` 구현을 압박합니다.
3. **RAII 검증은 Drop이 실행되는지 테스트** — Arc 카운터나 목 플래그로 정리가 일어났음을 증명합니다.
4. **cargo-show-asm은 제로 코스트를 증명** — 팬텀 타입, ZST, 뉴타입은 원시 C와 같은 어셈블리를 냅니다.
5. **"불가능한" 상태마다 compile-fail 테스트 추가** — 누군가 실수로 단일 사용 타입에 `Clone`을 파생하면 테스트가 잡습니다.

---

*타입 주도 정확성 in Rust 끝*
