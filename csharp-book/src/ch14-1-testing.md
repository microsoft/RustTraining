<a id="testing-in-rust-vs-c"></a>
## Rust의 테스트 vs C#

> **이 장에서 배우는 것:** 내장 `#[test]`와 xUnit의 차이, `rstest`를 이용한 매개변수화 테스트(`Theory`와 유사), `proptest`를 이용한 프로퍼티 테스트, `mockall` 기반 목 객체 처리, 그리고 async 테스트 패턴을 배웁니다.
>
> **난이도:** 🟡 중급

<a id="unit-tests"></a>
### 단위 테스트
```csharp
// C# — xUnit
using Xunit;

public class CalculatorTests
{
    [Fact]
    public void Add_ReturnsSum()
    {
        var calc = new Calculator();
        Assert.Equal(5, calc.Add(2, 3));
    }

    [Theory]
    [InlineData(1, 2, 3)]
    [InlineData(0, 0, 0)]
    [InlineData(-1, 1, 0)]
    public void Add_Theory(int a, int b, int expected)
    {
        Assert.Equal(expected, new Calculator().Add(a, b));
    }
}
```

```rust
// Rust — 내장 테스트 지원, 별도 프레임워크가 필요 없다
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]  // `cargo test` 중에만 컴파일된다
mod tests {
    use super::*;  // 부모 모듈에서 가져온다

    #[test]
    fn add_returns_sum() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn add_negative_numbers() {
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    #[should_panic(expected = "overflow")]
    fn add_overflow_panics() {
        let _ = add(i32::MAX, 1); // debug 모드에서는 panic 발생
    }
}
```

<a id="parameterized-tests-like-theory"></a>
### 매개변수화 테스트 (`[Theory]`와 유사)
```rust
// 매개변수화 테스트에는 `rstest` 크레이트를 사용한다
use rstest::rstest;

#[rstest]
#[case(1, 2, 3)]
#[case(0, 0, 0)]
#[case(-1, 1, 0)]
fn test_add(#[case] a: i32, #[case] b: i32, #[case] expected: i32) {
    assert_eq!(add(a, b), expected);
}

// Fixture - 테스트 setup 메서드와 비슷하다
#[rstest]
fn test_with_fixture(#[values(1, 2, 3)] x: i32) {
    assert!(x > 0);
}
```

<a id="assertions-comparison"></a>
### assert 매크로 비교

| C# (xUnit) | Rust | 설명 |
|-------------|------|-------|
| `Assert.Equal(expected, actual)` | `assert_eq!(expected, actual)` | 실패 시 diff를 출력 |
| `Assert.NotEqual(a, b)` | `assert_ne!(a, b)` | |
| `Assert.True(condition)` | `assert!(condition)` | |
| `Assert.Contains("sub", str)` | `assert!(str.contains("sub"))` | |
| `Assert.Throws<T>(() => ...)` | `#[should_panic]` | 또는 `std::panic::catch_unwind` 사용 |
| `Assert.Null(obj)` | `assert!(option.is_none())` | null이 없으므로 `Option` 사용 |

<a id="test-organization"></a>
### 테스트 구성

```text
my_crate/
├── src/
│   ├── lib.rs          # #[cfg(test)] mod tests { } 안에 단위 테스트 작성
│   └── parser.rs       # 각 모듈은 자체 테스트 모듈을 가질 수 있다
├── tests/              # 통합 테스트 (각 파일은 별도 crate)
│   ├── parser_test.rs  # 외부 소비자처럼 공개 API를 테스트
│   └── api_test.rs
└── benches/            # 벤치마크 (criterion 크레이트 사용)
    └── my_benchmark.rs
```

```rust
// tests/parser_test.rs — 통합 테스트
// PUBLIC API만 접근할 수 있다 (어셈블리 외부에서 테스트하는 것과 비슷하다)
use my_crate::parser;

#[test]
fn test_parse_valid_input() {
    let result = parser::parse("valid input");
    assert!(result.is_ok());
}
```

<a id="async-tests"></a>
### 비동기 테스트
```csharp
// C# — xUnit으로 작성한 async 테스트
[Fact]
public async Task GetUser_ReturnsUser()
{
    var service = new UserService();
    var user = await service.GetUserAsync(1);
    Assert.Equal("Alice", user.Name);
}
```

```rust
// Rust — tokio를 이용한 async 테스트
#[tokio::test]
async fn get_user_returns_user() {
    let service = UserService::new();
    let user = service.get_user(1).await.unwrap();
    assert_eq!(user.name, "Alice");
}
```

<a id="mocking-with-mockall"></a>
### `mockall`로 목 객체 만들기
```rust
use mockall::automock;

#[automock]                         // MockUserRepo 구조체를 생성한다
trait UserRepo {
    fn find_by_id(&self, id: u32) -> Option<User>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_returns_user_from_repo() {
        let mut mock = MockUserRepo::new();
        mock.expect_find_by_id()
            .with(mockall::predicate::eq(1))
            .returning(|_| Some(User { name: "Alice".into() }));

        let service = UserService::new(mock);
        let user = service.get_user(1).unwrap();
        assert_eq!(user.name, "Alice");
    }
}
```

```csharp
// C# — Moq에 해당하는 예
var mock = new Mock<IUserRepo>();
mock.Setup(r => r.FindById(1)).Returns(new User { Name = "Alice" });
var service = new UserService(mock.Object);
Assert.Equal("Alice", service.GetUser(1).Name);
```

<details>
<summary><strong>🏋️ 연습문제: 포괄적인 테스트 작성하기</strong> (클릭하여 펼치기)</summary>

**문제**: 아래 함수에 대해 정상 경로, 빈 입력, 숫자 문자열, 유니코드 케이스를 모두 다루는 테스트를 작성하세요.

```rust
pub fn title_case(input: &str) -> String {
    input.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(c) => format!("{}{}", c.to_uppercase(), chars.as_str().to_lowercase()),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
```

<details>
<summary>🔑 해답</summary>

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(title_case("hello world"), "Hello World");
    }

    #[test]
    fn empty_input() {
        assert_eq!(title_case(""), "");
    }

    #[test]
    fn single_word() {
        assert_eq!(title_case("rust"), "Rust");
    }

    #[test]
    fn already_title_case() {
        assert_eq!(title_case("Hello World"), "Hello World");
    }

    #[test]
    fn all_caps() {
        assert_eq!(title_case("HELLO WORLD"), "Hello World");
    }

    #[test]
    fn extra_whitespace() {
        // split_whitespace가 여러 공백을 처리한다
        assert_eq!(title_case("  hello   world  "), "Hello World");
    }

    #[test]
    fn unicode() {
        assert_eq!(title_case("café résumé"), "Café Résumé");
    }

    #[test]
    fn numeric_words() {
        assert_eq!(title_case("hello 42 world"), "Hello 42 World");
    }
}
```

**핵심 요점**: Rust의 내장 테스트 프레임워크만으로도 대부분의 단위 테스트를 처리할 수 있습니다. 매개변수화 테스트에는 `rstest`, 목 객체에는 `mockall`을 쓰면 되므로 xUnit 같은 큰 프레임워크가 꼭 필요하지 않습니다.

</details>
</details>


<!-- ch14a.1: Property Testing with proptest -->
<a id="property-testing-proving-correctness-at-scale"></a>
## 프로퍼티 테스트: 규모 있게 정확성 증명하기

**FsCheck**에 익숙한 C# 개발자라면 프로퍼티 기반 테스트를 바로 알아볼 것입니다. 개별 테스트 케이스를 직접 나열하는 대신, **모든 가능한 입력**에 대해 성립해야 하는 *성질(property)*을 서술하고, 프레임워크가 수천 개의 무작위 입력을 생성해 이를 깨뜨리려 시도합니다.

<a id="why-property-testing-matters"></a>
### 왜 프로퍼티 테스트가 중요한가
```csharp
// C# — 손으로 쓴 단위 테스트는 특정 케이스만 검사한다
[Fact]
public void Reverse_Twice_Returns_Original()
{
    var list = new List<int> { 1, 2, 3 };
    list.Reverse();
    list.Reverse();
    Assert.Equal(new[] { 1, 2, 3 }, list);
}
// 하지만 빈 리스트는? 원소가 하나뿐인 경우는? 10,000개 원소는? 음수는?
// 이런 경우를 모두 다루려면 손으로 수십 개의 케이스를 써야 한다.
```

```rust
// Rust — proptest가 수천 개의 입력을 자동으로 만든다
use proptest::prelude::*;

fn reverse<T: Clone>(v: &[T]) -> Vec<T> {
    v.iter().rev().cloned().collect()
}

proptest! {
    #[test]
    fn reverse_twice_is_identity(ref v in prop::collection::vec(any::<i32>(), 0..1000)) {
        let reversed_twice = reverse(&reverse(v));
        prop_assert_eq!(v, &reversed_twice);
    }
    // proptest는 이것을 수백 개의 임의 Vec<i32>로 실행한다:
    // [], [0], [i32::MIN, i32::MAX], [42; 999], 무작위 시퀀스 등
    // 실패하면 가장 작은 실패 입력으로 SHRINK한다!
}
```

<a id="getting-started-with-proptest"></a>
### `proptest` 시작하기
```toml
# Cargo.toml
[dev-dependencies]
proptest = "1.4"
```

<a id="common-patterns-for-c-developers"></a>
### C# 개발자가 자주 쓰는 패턴

```rust
use proptest::prelude::*;

// 1. roundtrip 성질: serialize → deserialize = 항등
// (JsonSerializer.Serialize → Deserialize 테스트와 유사)
proptest! {
    #[test]
    fn json_roundtrip(name in "[a-zA-Z]{1,50}", age in 0u32..150) {
        let user = User { name: name.clone(), age };
        let json = serde_json::to_string(&user).unwrap();
        let parsed: User = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(user, parsed);
    }
}

// 2. invariant 성질: 출력이 항상 특정 조건을 만족한다
proptest! {
    #[test]
    fn sort_output_is_sorted(ref v in prop::collection::vec(any::<i32>(), 0..500)) {
        let mut sorted = v.clone();
        sorted.sort();
        // 인접한 모든 쌍이 정렬 순서를 만족해야 한다
        for window in sorted.windows(2) {
            prop_assert!(window[0] <= window[1]);
        }
    }
}

// 3. oracle 성질: 두 구현을 비교한다
proptest! {
    #[test]
    fn fast_path_matches_slow_path(input in "[0-9a-f]{1,100}") {
        let result_fast = parse_hex_fast(&input);
        let result_slow = parse_hex_slow(&input);
        prop_assert_eq!(result_fast, result_slow);
    }
}

// 4. 사용자 정의 전략: 도메인 특화 테스트 데이터를 생성한다
fn valid_email() -> impl Strategy<Value = String> {
    ("[a-z]{1,20}", "[a-z]{1,10}", prop::sample::select(vec!["com", "org", "io"]))
        .prop_map(|(user, domain, tld)| format!("{}@{}.{}", user, domain, tld))
}

proptest! {
    #[test]
    fn email_parsing_accepts_valid_emails(email in valid_email()) {
        let result = Email::new(&email);
        prop_assert!(result.is_ok(), "Failed to parse: {}", email);
    }
}
```

<a id="proptest-vs-fscheck-comparison"></a>
### `proptest` vs `FsCheck` 비교

| 기능 | C# FsCheck | Rust proptest |
|---------|-----------|---------------|
| 무작위 입력 생성 | `Arb.Generate<T>()` | `any::<T>()` |
| 사용자 정의 생성기 | `Arb.Register<T>()` | `impl Strategy<Value = T>` |
| 실패 시 shrinking | 자동 | 자동 |
| 문자열 패턴 | 수동 | `"[regex]"` 전략 |
| 컬렉션 생성 | `Gen.ListOf` | `prop::collection::vec(strategy, range)` |
| 생성기 조합 | `Gen.Select` | `.prop_map()`, `.prop_flat_map()` |
| 설정(케이스 수) | `Config.MaxTest` | `proptest!` 블록 안의 `#![proptest_config(ProptestConfig::with_cases(10000))]` |

<a id="when-to-use-property-testing-vs-unit-testing"></a>
### 프로퍼티 테스트와 단위 테스트를 언제 쓸까

| **unit test**를 쓸 때 | **proptest**를 쓸 때 |
|------------------------|----------------------|
| 특정 edge case를 검증할 때 | 모든 입력에 대한 invariant를 검증할 때 |
| 에러 메시지/코드를 테스트할 때 | roundtrip 성질(parse ↔ format)을 검증할 때 |
| 통합 테스트나 mock 테스트를 할 때 | 두 구현을 비교할 때 |
| 동작이 정확한 값에 의존할 때 | "모든 X에 대해 P가 성립한다"를 검증할 때 |

---

<a id="integration-tests-the-tests-directory"></a>
## 통합 테스트: `tests/` 디렉터리

단위 테스트는 `src/` 내부에서 `#[cfg(test)]`와 함께 작성합니다. 통합 테스트는 별도의 `tests/` 디렉터리에 두며, 크레이트의 **공개 API**를 테스트합니다. 이는 C#의 통합 테스트가 프로젝트를 외부 어셈블리처럼 참조하는 방식과 비슷합니다.

```
my_crate/
├── src/
│   ├── lib.rs          // 공개 API
│   └── internal.rs     // 비공개 구현
├── tests/
│   ├── smoke.rs        // 각 파일이 별도 테스트 바이너리다
│   ├── api_tests.rs
│   └── common/
│       └── mod.rs      // 공용 테스트 헬퍼
└── Cargo.toml
```

<a id="writing-integration-tests"></a>
### 통합 테스트 작성하기

`tests/` 안의 각 파일은 여러분의 라이브러리에 의존하는 별도 크레이트로 컴파일됩니다.

```rust
// tests/smoke.rs — my_crate의 pub 항목만 접근할 수 있다
use my_crate::{process_order, Order, OrderResult};

#[test]
fn process_valid_order_returns_confirmation() {
    let order = Order::new("SKU-001", 3);
    let result = process_order(order);
    assert!(matches!(result, OrderResult::Confirmed { .. }));
}
```

<a id="shared-test-helpers"></a>
### 공용 테스트 헬퍼

공통 setup 코드는 `tests/common/mod.rs`에 두세요. (`tests/common.rs`에 두면 그 파일 자체가 테스트 파일로 취급됩니다.)

```rust
// tests/common/mod.rs
use my_crate::Config;

pub fn test_config() -> Config {
    Config::builder()
        .database_url("sqlite::memory:")
        .build()
        .expect("test config must be valid")
}
```

```rust
// tests/api_tests.rs
mod common;

use my_crate::App;

#[test]
fn app_starts_with_test_config() {
    let config = common::test_config();
    let app = App::new(config);
    assert!(app.is_healthy());
}
```

<a id="running-specific-test-types"></a>
### 특정 테스트 종류만 실행하기

```bash
cargo test                  # run all tests (unit + integration)
cargo test --lib            # 단위 테스트만 실행 (dotnet test --filter Category=Unit와 유사)
cargo test --test smoke     # tests/smoke.rs만 실행
cargo test --test api_tests # tests/api_tests.rs만 실행
```

**C#와의 핵심 차이:** 통합 테스트 파일은 크레이트의 `pub` API만 접근할 수 있습니다. private 함수는 보이지 않기 때문에, 자연스럽게 공개 인터페이스를 통해 테스트하게 되고 이는 대개 더 좋은 테스트 설계로 이어집니다.

***


