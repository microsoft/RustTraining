# 14. Async는 아키텍처가 아니라 최적화다

> **배울 내용**
> - async가 코드베이스 전체로 번지는 이유와 그것이 기능이 아니라 설계 비용인 이유
> - 대부분의 코드를 테스트하고 디버깅하기 쉽게 유지하는 "동기 core, async shell" 패턴
> - 비즈니스 로직 중간에 I/O가 필요한 어려운 경우를 다루는 방법
> - `spawn_blocking`이 해결책인 경우와 설계 냄새인 경우
> - async가 정말 core 로직에 들어가야 하는 경우
> - sync-first 라이브러리가 async-first 라이브러리보다 조합하기 쉬운 이유

지금까지 13개 장에 걸쳐 async Rust를 배웠다. 하지만 가장 중요한 결론은 이것이다. **대부분의 코드는 async일 필요가 없다.**

## 함수 색칠 문제

Bob Nystrom의 ["What Color is Your Function?"](https://journal.stuffwithstuff.com/2015/02/01/what-color-is-your-function/)은 핵심 문제를 잘 설명한다. async 함수는 sync 함수를 호출할 수 있지만, sync 함수는 async 함수를 직접 호출할 수 없다. 호출 체인의 어느 한 함수가 async가 되면 그 위에 있는 함수들도 대개 async가 되어야 한다.

Rust에서는 이 문제가 C#이나 JavaScript보다 더 강하게 드러난다. async는 함수 시그니처뿐 아니라 타입까지 바꾸기 때문이다.

| Sync 코드 | Async 대응 | 달라지는 점 |
|---|---|---|
| `fn process(&self)` | `async fn process(&self)` | 호출자도 async가 되어야 한다 |
| `&mut T` | `Arc<Mutex<T>>` | spawned task는 보통 `'static + Send`가 필요하다 |
| `std::sync::Mutex` | `tokio::sync::Mutex` | `.await`를 사이에 두고 잠금을 유지하면 다른 타입이 필요하다 |
| `impl Trait` 반환 | `impl Future<Output = T> + Send` | RPITIT 이후 단순해졌지만 여전히 색이 칠해진다 |
| `#[test]` | `#[tokio::test]` | 테스트에 runtime이 필요하다 |
| 스택 트레이스 5프레임 | 스택 트레이스 25프레임 | runtime 내부 프레임이 섞인다 |

이 표의 모든 행은 누군가가 선택하고, 맞게 쓰고, 계속 유지해야 하는 비용이다. 그 대부분은 비즈니스 로직과 직접 관계가 없다. Java의 Project Loom이나 Go의 goroutine은 동기처럼 보이는 코드를 runtime이 저렴하게 multiplex하도록 한다. Rust는 zero-cost 제어를 위해 명시적 async를 선택했지만, 그 제어에는 복잡도 비용이 따른다. 이 비용은 기본값처럼 지불할 것이 아니라 의식적으로 지불해야 한다.

## "하지만 thread는 비싸지 않나요?"

흔한 반론은 "thread가 비싸니까 async가 필요하다"는 것이다. 대부분의 팀이 운영하는 규모에서는 대체로 틀린 말이다.

- **스택 메모리:** Linux 기본값 기준 OS thread는 보통 8MB의 가상 주소 공간을 예약하지만, 실제 물리 메모리는 접근한 페이지만 commit된다. 대부분 idle인 thread는 수십 KB 수준만 사용할 수 있다.
- **컨텍스트 스위치:** 현대 하드웨어에서는 대략 마이크로초 단위다. 동시 요청이 수십 개 수준이면 잡음에 가깝고, 초당 10만 번 수준이 되면 측정 가능한 비용이 된다.
- **생성 비용:** thread 생성은 비싸지만 thread pool을 쓰면 대부분 amortize된다.

async가 복잡도를 정당화하는 정직한 기준점은 보통 **1천에서 1만 개 이상의 대부분 idle인 동시 연결** 근처다. 이 범위에서는 epoll, io_uring 같은 모델과 per-connection stack 절약이 실제 이점이 된다. 그 아래에서는 thread pool이 더 단순하고 디버깅하기 쉬우며 충분히 빠른 경우가 많다.

## 어려운 예: 로직 중간에 I/O가 필요할 때

순수 함수인 `fn add(a: i32, b: i32) -> i32`가 async일 필요가 없다는 말은 별로 흥미롭지 않다. 실제로 어려운 경우는 비즈니스 로직 중간에 I/O가 필요한 것처럼 보일 때다. 예를 들어 재고 확인, 환율 조회, 고객 등급 조회가 주문 처리 흐름 중간에 끼어든다.

async를 core까지 밀어 넣으면 코드는 자연스러워 보인다.

```rust
pub async fn process_order(order: Order) -> Result<Receipt, OrderError> {
    validate_items(&order)?;
    validate_quantities(&order)?;

    let stock = inventory_client.check(&order.items).await?;
    if !stock.all_available() {
        return Err(OrderError::OutOfStock(stock.missing()));
    }

    let pricing = calculate_pricing(&order, &stock);
    let discount = discount_service.lookup(order.customer_id).await?;
    let final_price = pricing.apply_discount(discount);

    Ok(Receipt::new(order, final_price))
}
```

나쁜 코드는 아니다. 하지만 순수한 검증, 가격 계산, receipt 생성이 모두 async context 안으로 끌려 들어갔다. 테스트에는 runtime이 필요하고, 상위 호출자도 async로 물든다.

대안은 *무엇을 결정할지*와 *어떻게 가져올지*를 분리하는 것이다.

```rust
pub fn validate_order(order: &Order) -> Result<ValidatedOrder, OrderError> {
    validate_items(order)?;
    validate_quantities(order)?;
    Ok(ValidatedOrder::from(order))
}

pub fn check_stock(
    order: &ValidatedOrder,
    stock: &StockResult,
) -> Result<StockedOrder, OrderError> {
    if !stock.all_available() {
        return Err(OrderError::OutOfStock(stock.missing()));
    }
    Ok(StockedOrder::from(order, stock))
}

pub fn finalize(order: &StockedOrder, discount: Discount) -> Receipt {
    let pricing = calculate_pricing(order);
    let final_price = pricing.apply_discount(discount);
    Receipt::new(order, final_price)
}
```

```rust
pub async fn process_order(order: Order) -> Result<Receipt, OrderError> {
    let validated = core::validate_order(&order)?;
    let stock = inventory_client.check(validated.items()).await?;
    let stocked = core::check_stock(&validated, &stock)?;
    let discount = discount_service.lookup(order.customer_id).await?;
    Ok(core::finalize(&stocked, discount))
}
```

async shell은 I/O orchestration만 담당하고, core는 동기 함수로 남는다. 이 구조에서는 핵심 규칙을 runtime 없이 테스트할 수 있고, 실패한 테스트의 스택 트레이스도 짧다.

## 의존성을 값으로 가져오기

core 함수가 직접 database client나 HTTP client를 받기 시작하면 다시 async가 번진다. 대신 core가 필요한 데이터를 값으로 받게 만들면 경계가 깔끔해진다.

```rust
pub struct PricingInput {
    pub items: Vec<Item>,
    pub stock: StockResult,
    pub discount: Discount,
}

pub fn price_order(input: PricingInput) -> Result<Money, PricingError> {
    let subtotal = input.items.iter().map(Item::price).sum();
    apply_discount(subtotal, input.discount)
}
```

I/O는 shell에서 끝낸다. core에는 이미 가져온 결과만 전달한다. 이 방식은 cache, fixture, property test와도 잘 맞는다.

## `spawn_blocking`은 언제 맞는가

`spawn_blocking`은 async runtime 안에서 CPU-bound 작업이나 blocking API를 호출해야 할 때 사용할 수 있는 도구다.

```rust
let parsed = tokio::task::spawn_blocking(move || parse_large_file(bytes))
    .await
    .map_err(|e| Error::Join(e))??;
```

이것이 적절한 경우는 다음과 같다.

- async server 안에서 CPU-heavy parsing, compression, hashing을 수행해야 한다.
- synchronous library를 당장 바꿀 수 없지만 async handler와 연결해야 한다.
- blocking 작업이 명확하게 제한되어 있고, core 설계를 오염시키지 않는다.

반대로 모든 중요한 비즈니스 로직을 `spawn_blocking`으로 감싸고 있다면 설계를 다시 봐야 한다. 그것은 "core가 sync로 남아야 한다"는 신호일 수 있다.

## async가 core에 들어가도 되는 경우

async를 core에서 완전히 금지하라는 뜻은 아니다. 다음 경우에는 async가 도메인 자체의 일부일 수 있다.

- protocol state machine이 network backpressure, timeout, cancellation을 직접 모델링한다.
- stream processing이 핵심 개념이고, item이 도착하는 시간이 의미를 가진다.
- cancellation safety가 비즈니스 불변식의 일부다.
- library의 주된 추상화가 `AsyncRead`, `AsyncWrite`, `Stream` 같은 async trait 위에 세워져 있다.

이 경우에도 기준은 같다. async가 도메인의 언어를 더 정확하게 만들면 core에 둘 수 있다. 단지 편해서, 또는 이미 handler가 async라서 넣는다면 shell에 머무르게 하라.

## Sync-first API가 더 잘 조합된다

sync 함수는 async 코드에서 쉽게 호출할 수 있다. 반대로 async 함수는 sync 코드에서 호출하기 어렵다. 그래서 library API는 가능하면 sync-first가 더 넓게 조합된다.

```rust
pub fn parse_config(bytes: &[u8]) -> Result<Config, Error> {
    // pure parsing
}

pub async fn load_config(path: &Path) -> Result<Config, Error> {
    let bytes = tokio::fs::read(path).await?;
    parse_config(&bytes)
}
```

이 설계에서는 CLI, test, build script, server handler가 같은 `parse_config`를 공유한다. async wrapper는 편의 기능일 뿐이다.

## 실전 규칙

- 기본값은 sync core다.
- I/O boundary에서 async shell을 만든다.
- core 함수에는 client가 아니라 이미 가져온 값을 전달한다.
- pure validation, parsing, pricing, formatting은 async로 만들지 않는다.
- `spawn_blocking`은 경계에 두고, 설계를 숨기는 용도로 남용하지 않는다.
- async가 도메인 의미를 표현할 때만 core에 넣는다.

## 요약

async Rust는 강력하지만, 아키텍처 원칙이 아니라 최적화 도구다. 네트워크 연결 수, backpressure, cancellation, runtime scheduling이 실제 문제일 때 async는 훌륭한 선택이다. 하지만 대부분의 비즈니스 규칙, parser, formatter, validator, 계산 로직은 sync로 남기는 편이 더 단순하고 테스트하기 쉽다.

가장 유지보수하기 좋은 Rust 서비스는 보통 이렇게 생겼다.

```text
HTTP / queue / runtime
        |
        v
thin async shell
        |
        v
sync domain core
```

async를 배웠다는 것은 모든 곳에 async를 쓰는 능력이 아니다. 어디에 쓰지 않을지 판단할 수 있다는 뜻이다.
