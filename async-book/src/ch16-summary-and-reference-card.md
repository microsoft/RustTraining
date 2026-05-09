<a id="summary-and-reference-card"></a>
# 요약 및 레퍼런스 카드

<a id="quick-reference-card"></a>
## 빠른 레퍼런스 카드

<a id="async-mental-model"></a>
### Async 사고 모델

```text
┌─────────────────────────────────────────────────────┐
│  async fn → 상태 머신(enum) → impl Future          │
│  .await   → 내부 future를 poll()                   │
│  executor → loop { poll(); sleep_until_woken(); }  │
│  waker    → "실행기야, 나를 다시 poll해 줘"        │
│  Pin      → "메모리에서 움직이지 않겠다고 약속"   │
└─────────────────────────────────────────────────────┘
```

<a id="common-patterns-cheat-sheet"></a>
### 흔한 패턴 치트시트

| 목표 | 사용법 |
|------|-----|
| 두 future를 동시에 실행 | `tokio::join!(a, b)` |
| 두 future를 경쟁시키기 | `tokio::select! { ... }` |
| 백그라운드 태스크 스폰 | `tokio::spawn(async { ... })` |
| async 안에서 blocking 코드 실행 | `tokio::task::spawn_blocking(\\|\\| { ... })` |
| 동시성 제한 | `Semaphore::new(N)` |
| 많은 태스크 결과 수집 | `JoinSet` |
| 태스크 간 상태 공유 | `Arc<Mutex<T>>` 또는 채널 |
| 우아한 종료 | `watch::channel` + `select!` |
| stream을 N개씩 동시 처리 | `.buffer_unordered(N)` |
| future에 타임아웃 걸기 | `tokio::time::timeout(dur, fut)` |
| 백오프로 재시도 | 커스텀 combinator (13장 참고) |

<a id="pinning-quick-reference"></a>
### Pinning 빠른 참조

| 상황 | 사용법 |
|-----------|-----|
| 힙에 future pin하기 | `Box::pin(fut)` |
| 스택에 future pin하기 | `tokio::pin!(fut)` |
| `Unpin` 타입 pin하기 | `Pin::new(&mut val)` — 안전하고 추가 비용이 없음 |
| pin된 트레잇 객체 반환 | `-> Pin<Box<dyn Future<Output = T> + Send>>` |

<a id="channel-selection-guide"></a>
### 채널 선택 가이드

| 채널 | Producer | Consumer | 값 형태 | 이럴 때 사용 |
|---------|-----------|-----------|--------|----------|
| `mpsc` | N | 1 | Stream | 작업 큐, 이벤트 버스 |
| `oneshot` | 1 | 1 | 단일 값 | 요청/응답, 완료 알림 |
| `broadcast` | N | N | 모든 수신자가 모두 받음 | fan-out 알림, 종료 신호 |
| `watch` | 1 | N | 최신 값만 유지 | 설정 업데이트, 상태 값 공유 |

<a id="mutex-selection-guide"></a>
### Mutex 선택 가이드

| Mutex | 이럴 때 사용 |
|-------|----------|
| `std::sync::Mutex` | lock을 잠깐만 잡고, `.await`를 절대 넘기지 않을 때 |
| `tokio::sync::Mutex` | lock을 `.await` 너머로 유지해야 할 때 |
| `parking_lot::Mutex` | 경쟁이 심하고 `.await`는 없으며 성능이 중요할 때 |
| `tokio::sync::RwLock` | 읽는 쪽이 많고 쓰는 쪽이 적으며, lock이 `.await`를 넘을 수 있을 때 |

<a id="decision-quick-reference"></a>
### 빠른 결정 가이드

```text
동시성이 필요한가?
├── I/O 바운드 → async/await
├── CPU 바운드 → rayon / std::thread
└── 혼합형 → CPU 부분은 spawn_blocking 사용

런타임은 무엇을 고를까?
├── 서버 애플리케이션 → tokio
├── 라이브러리 → runtime-agnostic (futures crate)
├── 임베디드 → embassy
└── 최소 구성 → smol

future를 동시에 돌려야 하는가?
├── 'static + Send 가능 → tokio::spawn
├── 'static + !Send 가능 → LocalSet
├── 'static일 수 없음 → FuturesUnordered
└── 추적/중단이 필요함 → JoinSet
```

<a id="common-error-messages-and-fixes"></a>
### 흔한 에러 메시지와 해결책

| 에러 | 원인 | 해결책 |
|-------|-------|-----|
| `future is not Send` | `.await`를 넘겨 `!Send` 타입을 들고 있음 | 값을 `.await` 전에 drop되도록 스코프를 나누거나, `current_thread` 런타임을 사용 |
| `borrowed value does not live long enough` in spawn | `tokio::spawn`은 `'static`을 요구함 | `Arc`, `clone()`, 또는 `FuturesUnordered` 사용 |
| `the trait Future is not implemented for ()` | `.await`를 빠뜨림 | async 호출 뒤에 `.await` 추가 |
| `cannot borrow as mutable` in poll | self-referential borrow 문제 | `Pin<&mut Self>`를 올바르게 사용 (4장 참고) |
| 프로그램이 조용히 멈춤 | `waker.wake()` 호출을 빼먹음 | 모든 `Pending` 경로에서 waker를 등록하고 깨우는지 확인 |

<a id="further-reading"></a>
### 더 읽어볼 자료

| 자료 | 읽는 이유 |
|----------|-----|
| [Tokio Tutorial](https://tokio.rs/tokio/tutorial) | 공식 실습형 가이드로, 첫 프로젝트에 특히 좋음 |
| [Async Book (official)](https://rust-lang.github.io/async-book/) | 언어 수준에서 `Future`, `Pin`, `Stream`을 다룸 |
| [Jon Gjengset — Crust of Rust: async/await](https://www.youtube.com/watch?v=ThjvMReOXYM) | 라이브 코딩과 함께 내부 동작을 2시간 동안 깊게 파고듦 |
| [Alice Ryhl — Actors with Tokio](https://ryhl.io/blog/actors-with-tokio/) | 상태를 가진 서비스를 위한 프로덕션 아키텍처 패턴 |
| [Without Boats — Pin, Unpin, and why Rust needs them](https://without.boats/blog/pin/) | 언어 설계 관점에서 `Pin`이 왜 필요한지 설명한 원문 |
| [Tokio mini-Redis](https://github.com/tokio-rs/mini-redis) | 학습용으로 훌륭한 완성형 async Rust 프로젝트 |
| [Tower documentation](https://docs.rs/tower) | `axum`, `tonic`, `hyper`가 쓰는 미들웨어/서비스 아키텍처 |

***

*Async Rust 학습 가이드 끝*