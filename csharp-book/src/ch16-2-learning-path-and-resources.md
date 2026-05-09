<a id="learning-path-and-next-steps"></a>
## 학습 경로와 다음 단계

> **이 장에서 배울 내용:** 구조화된 학습 로드맵(1-2주차, 1-3개월+), 추천 도서와 자료,
> C# 개발자가 흔히 부딪히는 함정(소유권 혼란, borrow checker와 씨름하기),
> 그리고 `ILogger`와 비교한 `tracing` 기반 구조화된 관측 가능성을 정리합니다.
>
> **난이도:** 🟢 기초

### 바로 시작할 다음 단계 (1-2주차)
1. **환경을 설정하세요**
   - [rustup.rs](https://rustup.rs/)로 Rust 설치
   - VS Code에 `rust-analyzer` 확장 설치 및 설정
   - 첫 `cargo new hello_world` 프로젝트 만들기

2. **기초를 확실히 다지세요**
   - 간단한 연습문제로 소유권 감각 익히기
   - 서로 다른 매개변수 타입(`&str`, `String`, `&mut`)으로 함수 작성해보기
   - 기본 구조체와 메서드 구현해보기

3. **에러 처리 연습을 하세요**
   - C# `try-catch` 코드를 `Result` 기반 패턴으로 바꿔보기
   - `?` 연산자와 `match` 구문 사용해보기
   - 커스텀 에러 타입 구현해보기

### 중간 목표 (1-2개월차)
1. **컬렉션과 이터레이터**
   - `Vec<T>`, `HashMap<K,V>`, `HashSet<T>`에 익숙해지기
   - `map`, `filter`, `collect`, `fold` 같은 이터레이터 메서드 익히기
   - `for` 루프와 이터레이터 체인을 비교하며 연습하기

2. **트레잇과 제네릭**
   - `Debug`, `Clone`, `PartialEq` 같은 흔한 트레잇 구현해보기
   - 제네릭 함수와 구조체 작성해보기
   - 트레잇 바운드와 `where` 절 이해하기

3. **프로젝트 구조**
   - 코드를 모듈로 나누어 구성하기
   - `pub` 가시성 이해하기
   - crates.io의 외부 크레이트 사용해보기

### 고급 주제 (3개월차+)
1. **동시성**
   - `Send`와 `Sync` 트레잇 이해하기
   - 기본 병렬성을 위해 `std::thread` 사용해보기
   - async 프로그래밍을 위해 `tokio` 탐구하기

2. **메모리 관리**
   - 공유 소유권을 위한 `Rc<T>`와 `Arc<T>` 이해하기
   - 힙 할당이 필요할 때 `Box<T>`를 언제 쓰는지 익히기
   - 복잡한 시나리오를 위해 라이프타임 숙달하기

3. **실전 프로젝트**
   - `clap`으로 CLI 도구 만들기
   - `axum`이나 `warp`로 웹 API 만들기
   - 라이브러리를 작성해 crates.io에 배포해보기

### 추천 학습 자료

#### 책
- **"The Rust Programming Language"** (온라인 무료) - 공식 입문서
- **"Rust by Example"** (온라인 무료) - 손으로 따라치는 실습 예제
- **"Programming Rust"** by Jim Blandy - 깊이 있는 기술 해설

#### 온라인 자료
- [Rust Playground](https://play.rust-lang.org/) - 브라우저에서 바로 코드 실행
- [Rustlings](https://github.com/rust-lang/rustlings) - 상호작용형 연습 문제
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - 실전 예제 모음

#### 연습 프로젝트
1. **명령줄 계산기** - enum과 패턴 매칭 연습
2. **파일 정리 도구** - 파일 시스템과 에러 처리 연습
3. **JSON 처리기** - `serde`와 데이터 변환 익히기
4. **HTTP 서버** - async 프로그래밍과 네트워킹 이해하기
5. **데이터베이스 라이브러리** - 트레잇, 제네릭, 에러 처리 숙달하기

<a id="common-pitfalls-for-c-developers"></a>
### C# 개발자가 흔히 겪는 함정

#### 소유권 혼란
```rust
// [ERROR] move된 값을 다시 사용하려고 함
fn wrong_way() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // ERROR: s가 move됨
}

// [OK] 필요할 때 참조나 clone 사용
fn right_way() {
    let s = String::from("hello");
    borrows_string(&s);
    println!("{}", s); // OK: 여기서는 여전히 s를 소유하고 있음
}

fn takes_ownership(s: String) { /* s is moved here */ }
fn borrows_string(s: &str) { /* s is borrowed here */ }
```

#### borrow checker와 싸우기
```rust
// [ERROR] 여러 개의 mutable 참조를 동시에 만드려고 함
fn wrong_borrowing() {
    let mut v = vec![1, 2, 3];
    let r1 = &mut v;
    // let r2 = &mut v; // ERROR: mutable로 두 번 이상 빌릴 수 없음
}

// [OK] mutable 대여의 스코프를 좁히기
fn right_borrowing() {
    let mut v = vec![1, 2, 3];
    {
        let r1 = &mut v;
        r1.push(4);
    } // 여기서 r1 스코프 종료
    
    let r2 = &mut v; // OK: 다른 mutable 대여가 없음
    r2.push(5);
}
```

#### `null` 값을 기대하기
```rust
// [ERROR] null 같은 동작을 기대함
fn no_null_in_rust() {
    // let s: String = null; // Rust에는 null이 없습니다!
}

// [OK] Option<T>를 명시적으로 사용
fn use_option_instead() {
    let maybe_string: Option<String> = None;
    
    match maybe_string {
        Some(s) => println!("Got string: {}", s),
        None => println!("No string available"),
    }
}
```

### 마지막 팁

1. **컴파일러를 믿으세요** - Rust의 컴파일 에러는 적대적이 아니라 도움이 됩니다
2. **작게 시작하세요** - 단순한 프로그램부터 만들고 조금씩 복잡도를 올리세요
3. **다른 사람의 코드를 읽으세요** - GitHub의 인기 크레이트를 살펴보세요
4. **도움을 요청하세요** - Rust 커뮤니티는 친절하고 도움을 잘 줍니다
5. **꾸준히 연습하세요** - Rust의 개념은 반복할수록 자연스러워집니다

기억하세요. Rust는 학습 곡선이 있지만, 그 대가로 메모리 안전성, 성능, 그리고 두려움 없는 동시성을 제공합니다. 처음에는 답답하게 느껴지는 소유권 시스템도 익숙해지면 올바르고 효율적인 프로그램을 쓰게 해주는 강력한 도구가 됩니다.

---

**축하합니다!** 이제 C#에서 Rust로 넘어가기 위한 탄탄한 기초를 갖추었습니다. 단순한 프로젝트부터 시작하고, 학습 과정에서 조급해하지 말고, 점차 더 복잡한 애플리케이션으로 나아가세요. Rust의 안전성과 성능은 초기 학습 투자 이상의 가치를 돌려줍니다.


<!-- ch16.2a: Structured Observability with tracing -->
<a id="structured-observability-tracing-vs-ilogger-and-serilog"></a>
## 구조화된 관측 가능성: `tracing` vs ILogger와 Serilog

C# 개발자는 `ILogger`, `Serilog`, `NLog`를 통한 **구조화된 로깅**에 익숙합니다. 이런 도구는 로그 메시지에 타입이 있는 키-값 속성을 함께 실어 보냅니다. Rust의 `log` 크레이트는 기본적인 레벨 기반 로깅을 제공하지만, span, async 인식, 분산 트레이싱 지원까지 포함한 **구조화된 관측 가능성**의 사실상 표준은 **`tracing`** 입니다.

### 왜 `log`보다 `tracing`인가

| 기능 | `log` 크레이트 | `tracing` 크레이트 | C# 대응 개념 |
|------|----------------|--------------------|--------------|
| 레벨 기반 메시지 | ✅ `info!()`, `error!()` | ✅ `info!()`, `error!()` | `ILogger.LogInformation()` |
| 구조화된 필드 | ❌ 문자열 보간만 가능 | ✅ 타입이 있는 키-값 필드 | Serilog `Log.Information("{User}", user)` |
| Span (스코프 문맥) | ❌ | ✅ `#[instrument]`, `span!()` | `ILogger.BeginScope()` |
| async 인식 | ❌ `.await`를 지나면 문맥 손실 | ✅ span이 `.await`를 따라감 | `Activity` / `DiagnosticSource` |
| 분산 트레이싱 | ❌ | ✅ OpenTelemetry 연동 | `System.Diagnostics.Activity` |
| 다양한 출력 형식 | 기본 수준 | JSON, pretty, compact, OTLP | Serilog sink |

### 시작하기
```toml
# Cargo.toml
[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
```

### 기본 사용법: 구조화된 로깅
```csharp
// C# Serilog
Log.Information("Processing order {OrderId} for {Customer}, total {Total:C}",
    orderId, customer.Name, order.Total);
// 출력: Processing order 12345 for Alice, total $99.95
// JSON: {"OrderId": 12345, "Customer": "Alice", "Total": 99.95, ...}
```

```rust
use tracing::{info, warn, error, debug, instrument};

// 구조화된 필드: 문자열 보간이 아니라 타입이 있는 필드
info!(order_id = 12345, customer = "Alice", total = 99.95,
      "Processing order");
// 출력: INFO Processing order order_id=12345 customer="Alice" total=99.95
// JSON: {"order_id": 12345, "customer": "Alice", "total": 99.95, ...}

// 동적 값
let order_id = 12345;
info!(order_id, "Order received");  // field 이름 = 변수 이름 축약형

// 조건부 필드
if let Some(promo) = promo_code {
    info!(order_id, promo_code = %promo, "Promo applied");
    //                        ^ %는 Display 포매팅 사용
    //                        ?는 Debug 포매팅 사용
}
```

### Span: async 코드에서 특히 강력한 기능

Span은 함수 호출과 `.await` 지점을 넘어 필드를 이어서 들고 가는 스코프 문맥입니다. `ILogger.BeginScope()`와 비슷하지만 async 환경에서 안전하게 동작합니다.

```csharp
// C# - Activity / BeginScope
using var activity = new Activity("ProcessOrder").Start();
activity.SetTag("order_id", orderId);

using (_logger.BeginScope(new Dictionary<string, object> { ["OrderId"] = orderId }))
{
    _logger.LogInformation("Starting processing");
    await ProcessPaymentAsync();
    _logger.LogInformation("Payment complete");  // OrderId가 계속 스코프 안에 있음
}
```

```rust
use tracing::{info, instrument, Instrument};

// #[instrument]는 함수 인자를 필드로 담은 span을 자동으로 만듭니다
#[instrument(skip(db), fields(customer_name))]
async fn process_order(order_id: u64, db: &Database) -> Result<(), AppError> {
    let order = db.get_order(order_id).await?;
    
    // 현재 span에 필드를 동적으로 추가
    tracing::Span::current().record("customer_name", &order.customer_name.as_str());
    
    info!("Starting processing");
    process_payment(&order).await?;        // .await를 지나도 span 문맥 유지
    info!(items = order.items.len(), "Payment complete");
    Ok(())
}
// 이 함수 안의 모든 로그 메시지에는 자동으로 다음 필드가 포함됩니다:
//   order_id=12345 customer_name="Alice"
// 중첩된 async 호출 안에서도 마찬가지입니다!

// 수동으로 span 만들기 (BeginScope와 유사)
async fn batch_process(orders: Vec<u64>, db: &Database) {
    for order_id in orders {
        let span = tracing::info_span!("process_order", order_id);
        
        // .instrument(span)은 해당 future에 span을 붙입니다
        process_order(order_id, db)
            .instrument(span)
            .await
            .unwrap_or_else(|e| error!("Failed: {e}"));
    }
}
```

### Subscriber 설정 (Serilog sink에 해당)

```rust
use tracing_subscriber::{fmt, EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

fn init_tracing() {
    // 개발 환경: 사람이 읽기 쉬운 컬러 출력
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "my_app=debug,tower_http=info".into()))
        .with(fmt::layer().pretty())  // 컬러 + 들여쓰기된 span 출력
        .init();
}

fn init_tracing_production() {
    // 프로덕션: 로그 수집용 JSON 출력 (Serilog JSON sink와 비슷함)
    tracing_subscriber::registry()
        .with(EnvFilter::new("my_app=info"))
        .with(fmt::layer().json())  // 구조화된 JSON
        .init();
    // 출력: {"timestamp":"...","level":"INFO","fields":{"order_id":123},...}
}
```

```bash
# 환경 변수로 로그 레벨 제어 (Serilog MinimumLevel과 유사)
RUST_LOG=my_app=debug,hyper=warn cargo run
RUST_LOG=trace cargo run  # 모든 로그
```

### Serilog → tracing 마이그레이션 치트시트

| Serilog / ILogger | tracing | 비고 |
|-------------------|---------|------|
| `Log.Information("{Key}", val)` | `info!(key = val, "message")` | 필드는 문자열 보간이 아니라 타입을 유지 |
| `Log.ForContext("Key", val)` | `span.record("key", val)` | 현재 span에 필드 추가 |
| `using BeginScope(...)` | `#[instrument]` 또는 `info_span!()` | `#[instrument]`를 쓰면 자동 |
| `.WriteTo.Console()` | `fmt::layer()` | 사람이 읽기 쉬운 출력 |
| `.WriteTo.Seq()` / `.File()` | `fmt::layer().json()` + 파일 리다이렉션 | 또는 `tracing-appender` 사용 |
| `.Enrich.WithProperty()` | `span!(Level::INFO, "name", key = val)` | span 필드 |
| `LogEventLevel.Debug` | `tracing::Level::DEBUG` | 같은 개념 |
| `{@Object}` destructuring | `field = ?value` (Debug) 또는 `%value` (Display) | `?` = Debug, `%` = Display |

### OpenTelemetry 연동
```toml
# 분산 트레이싱용 (System.Diagnostics + OTLP exporter와 유사)
[dependencies]
tracing-opentelemetry = "0.22"
opentelemetry = "0.21"
opentelemetry-otlp = "0.14"
```

```rust
// 콘솔 출력과 함께 OpenTelemetry 레이어 추가
use tracing_opentelemetry::OpenTelemetryLayer;

fn init_otel() {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .expect("Failed to create OTLP tracer");

    tracing_subscriber::registry()
        .with(OpenTelemetryLayer::new(tracer))  // span을 Jaeger/Tempo로 전송
        .with(fmt::layer())                     // 콘솔에도 출력
        .init();
}
// 이제 #[instrument] span이 자동으로 분산 trace가 됩니다!
```

***


