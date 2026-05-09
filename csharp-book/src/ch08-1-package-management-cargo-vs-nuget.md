<a id="package-management-cargo-vs-nuget"></a>
## 패키지 관리: Cargo vs NuGet

> **이 장에서 배울 내용:** `Cargo.toml`과 `.csproj`의 대응 관계, 버전 지정 방식, `Cargo.lock`,
> 조건부 컴파일을 위한 feature flag, 그리고 자주 쓰는 Cargo 명령과 NuGet/dotnet 명령의 대응 관계.
>
> **난이도:** 🟢 초급

### 의존성 선언

#### C# NuGet 의존성
```xml
<!-- MyApp.csproj -->
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <TargetFramework>net8.0</TargetFramework>
  </PropertyGroup>
  
  <PackageReference Include="Newtonsoft.Json" Version="13.0.3" />
  <PackageReference Include="Serilog" Version="3.0.1" />
  <PackageReference Include="Microsoft.AspNetCore.App" />
  
  <ProjectReference Include="../MyLibrary/MyLibrary.csproj" />
</Project>
```

#### Rust Cargo 의존성
```toml
# Cargo.toml
[package]
name = "my_app"
version = "0.1.0"
edition = "2021"

[dependencies]
serde_json = "1.0"                                      # crates.io에서 가져옴(NuGet과 유사)
serde = { version = "1.0", features = ["derive"] }      # feature 포함
log = "0.4"
tokio = { version = "1.0", features = ["full"] }

# 로컬 의존성(ProjectReference와 유사)
my_library = { path = "../my_library" }

# Git 의존성
my_git_crate = { git = "https://github.com/user/repo" }

# 개발 의존성(테스트 패키지와 유사)
[dev-dependencies]
criterion = "0.5"               # 벤치마킹
proptest = "1.0"                # 프로퍼티 테스트
```

### 버전 관리

#### C# 패키지 버전 관리
```xml
<!-- 중앙 집중식 패키지 관리(Directory.Packages.props) -->
<Project>
  <PropertyGroup>
    <ManagePackageVersionsCentrally>true</ManagePackageVersionsCentrally>
  </PropertyGroup>
  
  <PackageVersion Include="Newtonsoft.Json" Version="13.0.3" />
  <PackageVersion Include="Serilog" Version="3.0.1" />
</Project>

<!-- 재현 가능한 빌드를 위한 packages.lock.json -->
```

#### Rust 버전 관리
```toml
# Cargo.toml - 시맨틱 버전 관리
[dependencies]
serde = "1.0"        # 1.x.x와 호환(>=1.0.0, <2.0.0)
log = "0.4.17"       # 0.4.x와 호환(>=0.4.17, <0.5.0)
regex = "=1.5.4"     # 정확한 버전
chrono = "^0.4"      # caret 요구사항(기본값)
uuid = "~1.3.0"      # tilde 요구사항(>=1.3.0, <1.4.0)

# Cargo.lock - 재현 가능한 빌드를 위한 정확한 버전(auto-generated)
[[package]]
name = "serde"
version = "1.0.163"
# ... 정확한 의존성 트리
```

### 패키지 소스

#### C# 패키지 소스
```xml
<!-- nuget.config -->
<configuration>
  <packageSources>
    <add key="nuget.org" value="https://api.nuget.org/v3/index.json" />
    <add key="MyCompanyFeed" value="https://pkgs.dev.azure.com/company/_packaging/feed/nuget/v3/index.json" />
  </packageSources>
</configuration>
```

#### Rust 패키지 소스
```toml
# .cargo/config.toml
[source.crates-io]
replace-with = "my-awesome-registry"

[source.my-awesome-registry]
registry = "https://my-intranet:8080/index"

# 대체 레지스트리
[registries]
my-registry = { index = "https://my-intranet:8080/index" }

# Cargo.toml 안에서 사용
[dependencies]
my_crate = { version = "1.0", registry = "my-registry" }
```

### 자주 쓰는 명령 비교

| 작업 | C# 명령 | Rust 명령 |
|------|---------|-----------|
| 패키지 복원 | `dotnet restore` | `cargo fetch` |
| 패키지 추가 | `dotnet add package Newtonsoft.Json` | `cargo add serde_json` |
| 패키지 제거 | `dotnet remove package Newtonsoft.Json` | `cargo remove serde_json` |
| 패키지 업데이트 | `dotnet update` | `cargo update` |
| 패키지 목록 보기 | `dotnet list package` | `cargo tree` |
| 보안 감사 | `dotnet list package --vulnerable` | `cargo audit` |
| 빌드 정리 | `dotnet clean` | `cargo clean` |

### 기능(Features): 조건부 컴파일

#### C# 조건부 컴파일
```csharp
#if DEBUG
    Console.WriteLine("Debug mode");
#elif RELEASE
    Console.WriteLine("Release mode");
#endif

// 프로젝트 파일의 기능 토글
<PropertyGroup Condition="'$(Configuration)'=='Debug'">
    <DefineConstants>DEBUG;TRACE</DefineConstants>
</PropertyGroup>
```

#### Rust Feature 게이트
```toml
# Cargo.toml
[features]
default = ["json"]              # 기본 feature
json = ["serde_json"]           # serde_json을 활성화하는 feature
xml = ["serde_xml"]             # 대체 직렬화 방식
advanced = ["json", "xml"]      # 합성 feature

[dependencies]
serde_json = { version = "1.0", optional = true }
serde_xml = { version = "0.4", optional = true }
```

```rust
// feature에 따른 조건부 컴파일
#[cfg(feature = "json")]
use serde_json;

#[cfg(feature = "xml")]
use serde_xml;

pub fn serialize_data(data: &MyStruct) -> String {
    #[cfg(feature = "json")]
    return serde_json::to_string(data).unwrap();
    
    #[cfg(feature = "xml")]
    return serde_xml::to_string(data).unwrap();
    
    #[cfg(not(any(feature = "json", feature = "xml")))]
    return "No serialization feature enabled".to_string();
}
```

### 외부 크레이트 사용하기

#### C# 개발자에게 익숙한 대표 크레이트

| C# 라이브러리 | Rust 크레이트 | 용도 |
|---------------|---------------|------|
| Newtonsoft.Json | `serde_json` | JSON 직렬화 |
| HttpClient | `reqwest` | HTTP 클라이언트 |
| Entity Framework | `diesel` / `sqlx` | ORM / SQL 툴킷 |
| NLog/Serilog | `log` + `env_logger` | 로깅 |
| xUnit/NUnit | 내장 `#[test]` | 단위 테스트 |
| Moq | `mockall` | 목킹 |
| Flurl | `url` | URL 조작 |
| Polly | `tower` | 복원력 패턴 |

#### 예제: HTTP 클라이언트 마이그레이션
```csharp
// C# HttpClient 사용 예
public class ApiClient
{
    private readonly HttpClient _httpClient;
    
    public async Task<User> GetUserAsync(int id)
    {
        var response = await _httpClient.GetAsync($"/users/{id}");
        var json = await response.Content.ReadAsStringAsync();
        return JsonConvert.DeserializeObject<User>(json);
    }
}
```

```rust
// Rust reqwest 사용 예
use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    id: u32,
    name: String,
}

struct ApiClient {
    client: reqwest::Client,
}

impl ApiClient {
    async fn get_user(&self, id: u32) -> Result<User, reqwest::Error> {
        let user = self.client
            .get(&format!("https://api.example.com/users/{}", id))
            .send()
            .await?
            .json::<User>()
            .await?;
        
        Ok(user)
    }
}
```

***
