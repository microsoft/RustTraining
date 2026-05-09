<a id="case-study-3-framework-communication--lifetime-borrowing"></a>
# 사례 연구 3: 프레임워크 통신 → 라이프타임 대여

> **이 장에서 배우는 것:** C++의 raw pointer 기반 프레임워크 통신 패턴을 Rust의 라이프타임 기반 대여 시스템으로 어떻게 바꾸는지 살펴봅니다. dangling pointer 위험을 없애면서도 zero-cost abstraction은 유지하는 방법을 배웁니다.

<a id="the-c-pattern-raw-pointer-to-framework"></a>
## C++ 패턴: 프레임워크를 가리키는 raw pointer
```cpp
// C++ 원본: 모든 진단 모듈이 프레임워크를 가리키는 raw pointer를 저장한다
class DiagBase {
protected:
    DiagFramework* m_pFramework;  // Raw pointer — 누가 이걸 소유하는가?
public:
    DiagBase(DiagFramework* fw) : m_pFramework(fw) {}
    
    void LogEvent(uint32_t code, const std::string& msg) {
        m_pFramework->GetEventLog()->Record(code, msg);  // 아직 살아 있기를 바랄 뿐!
    }
};
// 문제: m_pFramework는 라이프타임 보장이 없는 raw pointer다
// 모듈이 여전히 참조 중인데 framework가 먼저 파괴되면 → UB
```

<a id="the-rust-solution-diagcontext-with-lifetime-borrowing"></a>
## Rust 해법: 라이프타임 대여를 사용하는 `DiagContext`
```rust
// 예시: module.rs — 저장하지 말고, 빌려서 쓴다

/// 실행 중 진단 모듈에 전달되는 컨텍스트.
/// 라이프타임 'a는 framework가 context보다 오래 살아 있음을 보장한다.
pub struct DiagContext<'a> {
    pub der_log: &'a mut EventLogManager,
    pub config: &'a ModuleConfig,
    pub framework_opts: &'a HashMap<String, String>,
}

/// 모듈은 context를 매개변수로 받는다 — framework pointer를 저장하지 않는다
pub trait DiagModule {
    fn id(&self) -> &str;
    fn execute(&mut self, ctx: &mut DiagContext) -> DiagResult<()>;
    fn pre_execute(&mut self, _ctx: &mut DiagContext) -> DiagResult<()> {
        Ok(())
    }
    fn post_execute(&mut self, _ctx: &mut DiagContext) -> DiagResult<()> {
        Ok(())
    }
}
```

<a id="key-insight"></a>
### 핵심 통찰
- C++ 모듈은 프레임워크를 가리키는 포인터를 **저장**합니다. 위험한 이유는 프레임워크가 먼저 파괴될 수도 있기 때문입니다.
- Rust 모듈은 컨텍스트를 함수 매개변수로 **전달받습니다**. borrow checker가 호출 동안 프레임워크가 살아 있음을 보장합니다.
- raw pointer도 없고, 라이프타임 모호성도 없고, "아직 살아 있겠지"라는 기대도 없습니다.

----

<a id="case-study-4-god-object--composable-state"></a>
# 사례 연구 4: God object → 조합 가능한 상태

<a id="the-c-pattern-monolithic-framework-class"></a>
## C++ 패턴: 단일 거대 프레임워크 클래스
```cpp
// C++ 원본: framework가 곧 god object다
class DiagFramework {
    // Health-monitor trap processing
    std::vector<AlertTriggerInfo> m_alertTriggers;
    std::vector<WarnTriggerInfo> m_warnTriggers;
    bool m_healthMonHasBootTimeError;
    uint32_t m_healthMonActionCounter;
    
    // GPU diagnostics
    std::map<uint32_t, GpuPcieInfo> m_gpuPcieMap;
    bool m_isRecoveryContext;
    bool m_healthcheckDetectedDevices;
    // ... GPU 관련 필드가 30개 이상 더 있음
    
    // PCIe tree
    std::shared_ptr<CPcieTreeLinux> m_pPcieTree;
    
    // Event logging
    CEventLogMgr* m_pEventLogMgr;
    
    // ... 그 밖의 여러 메서드
    void HandleGpuEvents();
    void HandleNicEvents();
    void RunGpuDiag();
    // 모든 것이 모든 것에 의존한다
};
```

<a id="the-rust-solution-composable-state-structs"></a>
## Rust 해법: 조합 가능한 상태 구조체
```rust
// 예시: main.rs — 상태를 역할별 구조체로 분해했다

#[derive(Default)]
struct HealthMonitorState {
    alert_triggers: Vec<AlertTriggerInfo>,
    warn_triggers: Vec<WarnTriggerInfo>,
    health_monitor_action_counter: u32,
    health_monitor_has_boot_time_error: bool,
    // health-monitor 관련 필드만 담는다
}

#[derive(Default)]
struct GpuDiagState {
    gpu_pcie_map: HashMap<u32, GpuPcieInfo>,
    is_recovery_context: bool,
    healthcheck_detected_devices: bool,
    // GPU 관련 필드만 담는다
}

/// framework는 모든 것을 평평하게 들고 있지 않고, 이런 상태를 조합한다
struct DiagFramework {
    ctx: DiagContext,                // 실행 컨텍스트
    args: Args,                      // CLI 인자
    pcie_tree: Option<DeviceTree>,  // shared_ptr 불필요
    event_log_mgr: EventLogManager,  // raw pointer가 아니라 소유
    fc_manager: FcManager,           // fault code 관리
    health: HealthMonitorState,      // health-monitor 상태 — 전용 구조체
    gpu: GpuDiagState,               // GPU 상태 — 전용 구조체
}
```

<a id="key-insight-1"></a>
### 핵심 통찰
- **테스트 용이성**: 각 상태 구조체를 독립적으로 단위 테스트할 수 있습니다.
- **가독성**: `m_alertTriggers`보다 `self.health.alert_triggers`가 소유 관계를 더 분명히 드러냅니다.
- **두려움 없는 리팩터링**: `GpuDiagState`를 바꿔도 health-monitor 처리에 우발적으로 영향을 주지 않습니다.
- **메서드 스프 없음**: health-monitor 상태만 필요하면 전체 framework가 아니라 `&mut HealthMonitorState`만 받으면 됩니다.

----

<a id="case-study-5-trait-objects--when-they-are-right"></a>
# 사례 연구 5: trait object가 정말 맞는 경우

- 모든 것을 enum으로 만들 필요는 없습니다. **진단 모듈 플러그인 시스템**은 trait object가 진짜로 적합한 사례입니다.
- 왜냐하면 진단 모듈은 **확장에 열려 있기 때문**입니다. 프레임워크를 수정하지 않고도 새 모듈을 추가할 수 있습니다.

```rust
// 예시: framework.rs — 여기서는 Vec<Box<dyn DiagModule>>가 올바른 선택이다
pub struct DiagFramework {
    modules: Vec<Box<dyn DiagModule>>,        // 런타임 다형성
    pre_diag_modules: Vec<Box<dyn DiagModule>>,
    event_log_mgr: EventLogManager,
    // ...
}

impl DiagFramework {
    /// 진단 모듈 등록 — DiagModule을 구현한 어떤 타입이든 가능
    pub fn register_module(&mut self, module: Box<dyn DiagModule>) {
        info!("Registering module: {}", module.id());
        self.modules.push(module);
    }
}
```

<a id="when-to-use-each-pattern"></a>
### 각 패턴을 언제 써야 하는가

| **사용 사례** | **패턴** | **이유** |
|-------------|-----------|--------|
| 컴파일 시점에 고정된 variant 집합 | `enum` + `match` | exhaustive checking, vtable 없음 |
| 하드웨어 이벤트 타입(Degrade, Fatal, Boot, ...) | `enum GpuEventKind` | 모든 variant를 알고 있고, 성능이 중요함 |
| PCIe 디바이스 타입(GPU, NIC, Switch, ...) | `enum PcieDeviceKind` | 집합이 고정되어 있고, 각 variant가 서로 다른 데이터를 가짐 |
| 플러그인/모듈 시스템(확장 가능) | `Box<dyn Trait>` | 프레임워크를 바꾸지 않고 새 모듈을 추가 가능 |
| 테스트용 mocking | `Box<dyn Trait>` | 테스트 대역을 주입할 수 있음 |

<a id="exercise-think-before-you-translate"></a>
### 연습문제: 번역하기 전에 먼저 설계하라
다음 C++ 코드를 보세요:
```cpp
class Shape { public: virtual double area() = 0; };
class Circle : public Shape { double r; double area() override { return 3.14*r*r; } };
class Rect : public Shape { double w, h; double area() override { return w*h; } };
std::vector<std::unique_ptr<Shape>> shapes;
```
**질문**: Rust로 옮길 때 `enum Shape`를 써야 할까요, 아니면 `Vec<Box<dyn Shape>>`를 써야 할까요?

<details><summary>해답 (클릭하여 펼치기)</summary>

**정답**: `enum Shape`입니다. 도형 집합이 **닫혀 있기 때문**입니다(컴파일 시점에 이미 모두 앎). 사용자가 런타임에 새 도형 타입을 추가할 수 있을 때만 `Box<dyn Shape>`를 씁니다.

```rust
// 올바른 Rust 번역:
enum Shape {
    Circle { r: f64 },
    Rect { w: f64, h: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { r } => std::f64::consts::PI * r * r,
            Shape::Rect { w, h } => w * h,
        }
    }
}

fn main() {
    let shapes: Vec<Shape> = vec![
        Shape::Circle { r: 5.0 },
        Shape::Rect { w: 3.0, h: 4.0 },
    ];
    for shape in &shapes {
        println!("Area: {:.2}", shape.area());
    }
}
// Output:
// Area: 78.54
// Area: 12.00
```

</details>

----

<a id="translation-metrics-and-lessons-learned"></a>
# 번역 지표와 배운 점

<a id="what-we-learned"></a>
## 우리가 배운 것
1. **기본값은 enum 디스패치여야 한다**. 약 10만 줄의 C++에서 `Box<dyn Trait>`가 진짜로 필요했던 곳은 약 25곳뿐이었습니다(플러그인 시스템, 테스트 mocking). 나머지 약 900개의 virtual 메서드는 enum + match로 바뀌었습니다.
2. **arena 패턴은 참조 사이클을 없앤다**. `shared_ptr`와 `enable_shared_from_this`는 소유권이 불분명하다는 신호입니다. 먼저 누가 데이터를 **소유하는지**부터 생각하세요.
3. **포인터를 저장하지 말고 컨텍스트를 전달하라**. 라이프타임으로 범위가 정해진 `DiagContext<'a>`는 모든 모듈에 `Framework*`를 저장하는 것보다 더 안전하고 더 명확합니다.
4. **God object를 분해하라**. 구조체에 필드가 30개 이상 있다면, 사실은 외투 하나를 같이 입은 3~4개의 구조체일 가능성이 큽니다.
5. **컴파일러를 페어 프로그래머처럼 활용하라**. `dynamic_cast` 호출이 약 400번 있었다는 것은 런타임 실패 가능성이 약 400곳 있었다는 뜻입니다. Rust에서 그에 대응하는 것이 0번이라는 것은 런타임 타입 에러도 0이라는 뜻입니다.

<a id="the-hardest-parts"></a>
## 가장 어려웠던 부분
- **라이프타임 표기**: raw pointer에 익숙하면 올바른 대여를 잡는 데 시간이 걸립니다. 하지만 일단 컴파일되면 그 코드는 맞습니다.
- **borrow checker와의 씨름**: 같은 시점에 두 곳에서 `&mut self`를 원하게 됩니다. 해법은 상태를 별도 구조체로 분해하는 것입니다.
- **직역의 유혹을 이겨내기**: 어디에나 `Vec<Box<dyn Base>>`를 쓰고 싶어집니다. 질문하세요. "이 variant 집합은 닫혀 있는가?" → 그렇다면 enum을 쓰세요.

<a id="recommendation-for-c-teams"></a>
## C++ 팀을 위한 권장 사항
1. 작은 독립 모듈부터 시작하세요(God object는 제외).
2. 먼저 자료구조를 옮기고, 그다음 동작을 옮기세요.
3. 컴파일러가 길잡이가 되게 하세요. 에러 메시지가 매우 훌륭합니다.
4. `dyn Trait`보다 먼저 `enum`을 떠올리세요.
5. 통합하기 전에 [Rust playground](https://play.rust-lang.org/)에서 패턴을 먼저 실험해 보세요.

----
