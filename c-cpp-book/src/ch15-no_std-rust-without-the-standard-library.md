<a id="no_std-rust-without-the-standard-library"></a>
# `no_std` — 표준 라이브러리 없는 Rust

> **이 장에서 배우는 것:** `#![no_std]`를 사용해 베어메탈과 임베디드 타깃용 Rust를 작성하는 방법, `core`와 `alloc` 크레이트의 분리, panic handler, 그리고 이것이 `libc` 없는 임베디드 C와 어떻게 대응되는지를 배웁니다.

임베디드 C 경험이 있다면 이미 `libc` 없이, 혹은 아주 작은 런타임만 두고 작업하는 데 익숙할 것입니다. Rust에도 이에 대응하는 1급 기능이 있습니다. 바로 **`#![no_std]`** 속성입니다.

<a id="what-is-no_std"></a>
## `no_std`란 무엇인가?

크레이트 루트에 `#![no_std]`를 추가하면 컴파일러는 암묵적인 `extern crate std;`를 제거하고, **`core`**(그리고 선택적으로 **`alloc`**)에만 링크합니다.

| 계층 | 제공하는 것 | OS / 힙 필요 여부 |
|------|-------------|-------------------|
| `core` | 원시 타입, `Option`, `Result`, `Iterator`, 수학 함수, `slice`, `str`, atomics, `fmt` | **아니오** — 베어메탈에서 동작 |
| `alloc` | `Vec`, `String`, `Box`, `Rc`, `Arc`, `BTreeMap` | 전역 할당자는 필요하지만 **OS는 필요 없음** |
| `std` | `HashMap`, `fs`, `net`, `thread`, `io`, `env`, `process` | **예** — OS 필요 |

> **임베디드 개발자를 위한 감각적인 기준:** C 프로젝트가 `-lc`에 링크되고 `malloc`을 사용한다면 대개 `core` + `alloc`을 쓸 수 있습니다. `malloc` 없이 베어메탈에서 돌아간다면 `core`만 사용하는 쪽이 맞습니다.

<a id="declaring-no_std"></a>
## `no_std` 선언하기

```rust
// src/lib.rs  (또는 #![no_main]을 쓰는 바이너리라면 src/main.rs)
#![no_std]

// 그래도 `core`에 있는 것들은 그대로 쓸 수 있다:
use core::fmt;
use core::result::Result;
use core::option::Option;

// 할당자가 있다면 힙 타입을 opt-in 할 수 있다:
extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;
```

베어메탈 바이너리라면 `#![no_main]`과 panic handler도 필요합니다.

```rust
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {} // panic 시 멈춤 — 보드에 맞게 reset/LED blink 등으로 바꿀 수 있다
}

// 엔트리 포인트는 HAL / 링커 스크립트에 따라 달라진다
```

<a id="what-you-lose-and-alternatives"></a>
## 무엇을 잃고, 무엇으로 대체하는가

| `std` 기능 | `no_std` 대안 |
|------------|---------------|
| `println!` | UART에 `core::write!` 사용 / `defmt` |
| `HashMap` | `heapless::FnvIndexMap`(고정 용량) 또는 `BTreeMap`(`alloc` 사용 시) |
| `Vec` | `heapless::Vec`(스택 할당, 고정 용량) |
| `String` | `heapless::String` 또는 `&str` |
| `std::io::Read/Write` | `embedded_io::Read/Write` |
| `thread::spawn` | 인터럽트 핸들러, RTIC 태스크 |
| `std::time` | 하드웨어 타이머 주변장치 |
| `std::fs` | Flash / EEPROM 드라이버 |

<a id="notable-no_std-crates-for-embedded"></a>
## 임베디드에서 눈여겨볼 `no_std` 크레이트

| 크레이트 | 용도 | 비고 |
|----------|------|------|
| [`heapless`](https://crates.io/crates/heapless) | 고정 용량 `Vec`, `String`, `Queue`, `Map` | 할당자 불필요 — 전부 스택 기반 |
| [`defmt`](https://crates.io/crates/defmt) | probe/ITM 경유 고효율 로깅 | `printf`와 비슷하지만 포맷팅을 호스트에서 지연 수행 |
| [`embedded-hal`](https://crates.io/crates/embedded-hal) | 하드웨어 추상화 트레잇(SPI, I²C, GPIO, UART) | 한 번 구현하면 어떤 MCU에서든 재사용 가능 |
| [`cortex-m`](https://crates.io/crates/cortex-m) | ARM Cortex-M intrinsic 및 레지스터 접근 | CMSIS와 비슷한 저수준 레이어 |
| [`cortex-m-rt`](https://crates.io/crates/cortex-m-rt) | Cortex-M용 런타임 / 스타트업 코드 | `startup.s`를 대체 |
| [`rtic`](https://crates.io/crates/rtic) | Real-Time Interrupt-driven Concurrency | 컴파일 타임 태스크 스케줄링, 제로 오버헤드 |
| [`embassy`](https://crates.io/crates/embassy-executor) | 임베디드용 async executor | 베어메탈에서 `async/await` 사용 |
| [`postcard`](https://crates.io/crates/postcard) | `no_std`용 serde 직렬화(바이너리) | 문자열 기반 `serde_json`을 쓰기 어려울 때 대안 |
| [`thiserror`](https://crates.io/crates/thiserror) | `Error` 트레잇용 derive 매크로 | v2부터 `no_std`에서 동작; `anyhow`보다 적합 |
| [`smoltcp`](https://crates.io/crates/smoltcp) | `no_std` TCP/IP 스택 | OS 없이 네트워킹이 필요할 때 |

<a id="c-vs-rust-bare-metal-comparison"></a>
## C vs Rust: 베어메탈 비교

전형적인 임베디드 C의 blinky 예제는 다음과 같습니다.

```c
// C — 베어메탈, 벤더 HAL 사용
#include "stm32f4xx_hal.h"

void SysTick_Handler(void) {
    HAL_GPIO_TogglePin(GPIOA, GPIO_PIN_5);
}

int main(void) {
    HAL_Init();
    __HAL_RCC_GPIOA_CLK_ENABLE();
    GPIO_InitTypeDef gpio = { .Pin = GPIO_PIN_5, .Mode = GPIO_MODE_OUTPUT_PP };
    HAL_GPIO_Init(GPIOA, &gpio);
    HAL_SYSTICK_Config(HAL_RCC_GetHCLKFreq() / 1000);
    while (1) {}
}
```

이에 대응하는 Rust 예제는 다음과 같습니다(`embedded-hal` + 보드 크레이트 사용).

```rust
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _; // panic handler: 무한 루프
use stm32f4xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();
    let mut delay = dp.TIM2.delay_ms(&clocks);

    loop {
        led.toggle();
        delay.delay_ms(500u32);
    }
}
```

**C 개발자 관점의 핵심 차이점:**
- `Peripherals::take()`는 `Option`을 반환합니다. 덕분에 singleton 패턴이 컴파일 타임에 강제되어 이중 초기화 버그를 막을 수 있습니다.
- `.split()`은 각 핀의 소유권을 이동시킵니다. 따라서 두 모듈이 같은 핀을 동시에 구동하는 실수를 할 수 없습니다.
- 모든 레지스터 접근은 타입 검사를 거칩니다. 읽기 전용 레지스터에 실수로 쓰는 일을 막을 수 있습니다.
- borrow checker는 `main`과 인터럽트 핸들러 사이의 데이터 레이스를 막아 줍니다(RTIC 사용 시 특히 강력합니다).

<a id="when-to-use-no_std-vs-std"></a>
## 언제 `no_std`를 쓰고 언제 `std`를 쓸 것인가

```mermaid
flowchart TD
    A[타깃에 OS가 있는가?] -->|예| B[`std` 사용]
    A -->|아니오| C[힙 할당자가 있는가?]
    C -->|예| D["#![no_std] + extern crate alloc 사용"]
    C -->|아니오| E["`core`만 사용하는 #![no_std]"]
    B --> F[전체 `Vec`, `HashMap`, 스레드, fs, net 사용 가능]
    D --> G[`Vec`, `String`, `Box`, `BTreeMap` 사용 가능, 단 fs/net/threads 없음]
    E --> H[고정 크기 배열, `heapless` 컬렉션, 동적 할당 없음]
```

<a id="exercise-no_std-ring-buffer"></a>
# 연습문제: `no_std` 링 버퍼

🔴 **도전** — `no_std` 환경에서 제네릭, `MaybeUninit`, `#[cfg(test)]`를 함께 다룹니다.

임베디드 시스템에서는 동적 할당 없이 동작하는 고정 크기 링 버퍼(순환 버퍼)가 자주 필요합니다. `core`만 사용해서(`alloc` 없이, `std` 없이) 하나 구현해 보세요.

**요구 사항:**
- 요소 타입 `T: Copy`에 대해 제네릭이어야 한다
- 고정 용량 `N`(const generic)
- `push(&mut self, item: T)` — 가득 찼을 때 가장 오래된 요소를 덮어쓴다
- `pop(&mut self) -> Option<T>` — 가장 오래된 요소를 반환한다
- `len(&self) -> usize`
- `is_empty(&self) -> bool`
- `#![no_std]`로 컴파일되어야 한다

```rust
// 시작 코드
#![no_std]

use core::mem::MaybeUninit;

pub struct RingBuffer<T: Copy, const N: usize> {
    buf: [MaybeUninit<T>; N],
    head: usize,  // 다음 쓰기 위치
    tail: usize,  // 다음 읽기 위치
    count: usize,
}

impl<T: Copy, const N: usize> RingBuffer<T, N> {
    pub const fn new() -> Self {
        todo!()
    }
    pub fn push(&mut self, item: T) {
        todo!()
    }
    pub fn pop(&mut self) -> Option<T> {
        todo!()
    }
    pub fn len(&self) -> usize {
        todo!()
    }
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}
```

<details>
<summary>해답</summary>

```rust
#![no_std]

use core::mem::MaybeUninit;

pub struct RingBuffer<T: Copy, const N: usize> {
    buf: [MaybeUninit<T>; N],
    head: usize,
    tail: usize,
    count: usize,
}

impl<T: Copy, const N: usize> RingBuffer<T, N> {
    pub const fn new() -> Self {
        Self {
            // SAFETY: MaybeUninit는 초기화를 요구하지 않는다
            buf: unsafe { MaybeUninit::uninit().assume_init() },
            head: 0,
            tail: 0,
            count: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        self.buf[self.head] = MaybeUninit::new(item);
        self.head = (self.head + 1) % N;
        if self.count == N {
            // 버퍼가 가득 찼다 — 가장 오래된 값을 덮어쓰기 위해 tail을 전진
            self.tail = (self.tail + 1) % N;
        } else {
            self.count += 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        // SAFETY: push()로 이전에 기록한 위치만 읽는다
        let item = unsafe { self.buf[self.tail].assume_init() };
        self.tail = (self.tail + 1) % N;
        self.count -= 1;
        Some(item)
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_push_pop() {
        let mut rb = RingBuffer::<u32, 4>::new();
        assert!(rb.is_empty());

        rb.push(10);
        rb.push(20);
        rb.push(30);
        assert_eq!(rb.len(), 3);

        assert_eq!(rb.pop(), Some(10));
        assert_eq!(rb.pop(), Some(20));
        assert_eq!(rb.pop(), Some(30));
        assert_eq!(rb.pop(), None);
    }

    #[test]
    fn overwrite_on_full() {
        let mut rb = RingBuffer::<u8, 3>::new();
        rb.push(1);
        rb.push(2);
        rb.push(3);
        // 버퍼가 가득 찬 상태: [1, 2, 3]

        rb.push(4); // 1을 덮어써서 [4, 2, 3], tail이 전진
        assert_eq!(rb.len(), 3);
        assert_eq!(rb.pop(), Some(2)); // 살아남은 것 중 가장 오래된 값
        assert_eq!(rb.pop(), Some(3));
        assert_eq!(rb.pop(), Some(4));
        assert_eq!(rb.pop(), None);
    }
}
```

**이 예제가 임베디드 C 개발자에게 중요한 이유:**
- `MaybeUninit`는 Rust에서 초기화되지 않은 메모리에 대응하는 도구입니다. C의 `char buf[N];`처럼 컴파일러가 자동으로 0으로 채우지 않습니다.
- `unsafe` 블록은 최소한으로 제한되어 있으며(딱 2줄), 각각 `// SAFETY:` 주석이 붙어 있습니다.
- `const fn new()` 덕분에 런타임 생성자 없이도 `static` 변수 안에서 링 버퍼를 만들 수 있습니다.
- 코드 자체는 `no_std`여도 테스트는 호스트에서 `cargo test`로 실행할 수 있습니다.

</details>
