## Capstone Project: Build a CLI Weather Tool

> **What you'll learn:** How to combine everything — structs, traits, error handling, async, modules,
> serde, and CLI argument parsing — into a working Rust application. This mirrors the kind of tool
> a C# developer would build with `HttpClient`, `System.Text.Json`, and `System.CommandLine`.
>
> **Difficulty:** 🟡 Intermediate

This capstone pulls together concepts from every part of the book. You'll build `weather-cli`, a command-line tool that fetches weather data from an API and displays it. The project is structured as a mini-crate with proper module layout, error types, and tests.

### Project Overview

```mermaid
graph TD
    CLI["main.rs<br>clap CLI parser"] --> Client["client.rs<br>reqwest + tokio"]
    Client -->|"HTTP GET"| API["Weather API"]
    Client -->|"JSON → struct"| Model["weather.rs<br>serde Deserialize"]
    Model --> Display["display.rs<br>fmt::Display"]
    CLI --> Err["error.rs<br>thiserror"]
    Client --> Err

    style CLI fill:#bbdefb,color:#000
    style Err fill:#ffcdd2,color:#000
    style Model fill:#c8e6c9,color:#000
```

**What you'll build:**
```
$ weather-cli --city "Seattle"
🌧  Seattle: 12°C, Overcast clouds
    Humidity: 82%  Wind: 5.4 m/s
```

**Concepts exercised:**
| Book Chapter | Concept Used Here |
|---|---|
| Ch05 (Structs) | `WeatherReport`, `Config` data types |
| Ch08 (Modules) | `src/lib.rs`, `src/client.rs`, `src/display.rs` |
| Ch09 (Errors) | Custom `WeatherError` with `thiserror` |
| Ch10 (Traits) | `Display` impl for formatted output |
| Ch11 (From/Into) | JSON deserialization via `serde` |
| Ch12 (Iterators) | Processing API response arrays |
| Ch13 (Async) | `reqwest` + `tokio` for HTTP calls |
| Ch14-1 (Testing) | Unit tests + integration test |

---

### Step 1: Project Setup

```bash
cargo new weather-cli
cd weather-cli
```

Add dependencies to `Cargo.toml`:
```toml
[package]
name = "weather-cli"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4", features = ["derive"] }   # CLI args (like System.CommandLine)
reqwest = { version = "0.12", features = ["json"] } # HTTP client (like HttpClient)
serde = { version = "1", features = ["derive"] }    # Serialization (like System.Text.Json)
serde_json = "1"
thiserror = "2"                                      # Error types
tokio = { version = "1", features = ["full"] }       # Async runtime
```

```csharp
// C# equivalent dependencies:
// dotnet add package System.CommandLine
// dotnet add package System.Net.Http.Json
// (System.Text.Json and HttpClient are built-in)
```

### Step 2: Define Your Data Types

Create `src/weather.rs`:
```rust
use serde::Deserialize;

/// Raw API response (matches JSON shape)
#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub main: MainData,
    pub weather: Vec<WeatherCondition>,
    pub wind: WindData,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct MainData {
    pub temp: f64,
    pub humidity: u32,
}

#[derive(Deserialize, Debug)]
pub struct WeatherCondition {
    pub description: String,
    pub icon: String,
}

#[derive(Deserialize, Debug)]
pub struct WindData {
    pub speed: f64,
}

/// Our domain type (clean, decoupled from API)
#[derive(Debug, Clone)]
pub struct WeatherReport {
    pub city: String,
    pub temp_celsius: f64,
    pub description: String,
    pub humidity: u32,
    pub wind_speed: f64,
}

impl From<ApiResponse> for WeatherReport {
    fn from(api: ApiResponse) -> Self {
        let description = api.weather
            .first()
            .map(|w| w.description.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        WeatherReport {
            city: api.name,
            temp_celsius: api.main.temp,
            description,
            humidity: api.main.humidity,
            wind_speed: api.wind.speed,
        }
    }
}
```

```csharp
// C# equivalent:
// public record ApiResponse(MainData Main, List<WeatherCondition> Weather, ...);
// public record WeatherReport(string City, double TempCelsius, ...);
// Manual mapping or AutoMapper
```

**Key difference:** `#[derive(Deserialize)]` + `From` impl replaces C#'s `JsonSerializer.Deserialize<T>()` + AutoMapper. Both happen at compile time in Rust — no reflection.

### Step 3: Error Type

Create `src/error.rs`:
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WeatherError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("City not found: {0}")]
    CityNotFound(String),

    #[error("API key not set — export WEATHER_API_KEY")]
    MissingApiKey,
}

pub type Result<T> = std::result::Result<T, WeatherError>;
```

### Step 4: HTTP Client

Create `src/client.rs`:
```rust
use crate::error::{WeatherError, Result};
use crate::weather::{ApiResponse, WeatherReport};

pub struct WeatherClient {
    api_key: String,
    http: reqwest::Client,
}

impl WeatherClient {
    pub fn new(api_key: String) -> Self {
        WeatherClient {
            api_key,
            http: reqwest::Client::new(),
        }
    }

    pub async fn get_weather(&self, city: &str) -> Result<WeatherReport> {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
            city, self.api_key
        );

        let response = self.http.get(&url).send().await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(WeatherError::CityNotFound(city.to_string()));
        }

        let api_data: ApiResponse = response.json().await?;
        Ok(WeatherReport::from(api_data))
    }
}
```

```csharp
// C# equivalent:
// var response = await _httpClient.GetAsync(url);
// if (response.StatusCode == HttpStatusCode.NotFound)
//     throw new CityNotFoundException(city);
// var data = await response.Content.ReadFromJsonAsync<ApiResponse>();
```

**Key differences:**
- `?` operator replaces `try/catch` — errors propagate automatically via `Result`
- `WeatherReport::from(api_data)` uses the `From` trait instead of AutoMapper
- No `IHttpClientFactory` — `reqwest::Client` handles connection pooling internally

### Step 5: Display Formatting

Create `src/display.rs`:
```rust
use std::fmt;
use crate::weather::WeatherReport;

impl fmt::Display for WeatherReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let icon = weather_icon(&self.description);
        writeln!(f, "{}  {}: {:.0}°C, {}",
            icon, self.city, self.temp_celsius, self.description)?;
        write!(f, "    Humidity: {}%  Wind: {:.1} m/s",
            self.humidity, self.wind_speed)
    }
}

fn weather_icon(description: &str) -> &str {
    let desc = description.to_lowercase();
    if desc.contains("clear") { "☀️" }
    else if desc.contains("cloud") { "☁️" }
    else if desc.contains("rain") || desc.contains("drizzle") { "🌧" }
    else if desc.contains("snow") { "❄️" }
    else if desc.contains("thunder") { "⛈" }
    else { "🌡" }
}
```

### Step 6: Wire It All Together

`src/lib.rs`:
```rust
pub mod client;
pub mod display;
pub mod error;
pub mod weather;
```

`src/main.rs`:
```rust
use clap::Parser;
use weather_cli::{client::WeatherClient, error::WeatherError};

#[derive(Parser)]
#[command(name = "weather-cli", about = "Fetch weather from the command line")]
struct Cli {
    /// City name to look up
    #[arg(short, long)]
    city: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let api_key = match std::env::var("WEATHER_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Error: {}", WeatherError::MissingApiKey);
            std::process::exit(1);
        }
    };

    let client = WeatherClient::new(api_key);

    match client.get_weather(&cli.city).await {
        Ok(report) => println!("{report}"),
        Err(WeatherError::CityNotFound(city)) => {
            eprintln!("City not found: {city}");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}
```

### Step 7: Tests

```rust
// In src/weather.rs or tests/weather_test.rs
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_api_response() -> ApiResponse {
        serde_json::from_str(r#"{
            "main": {"temp": 12.3, "humidity": 82},
            "weather": [{"description": "overcast clouds", "icon": "04d"}],
            "wind": {"speed": 5.4},
            "name": "Seattle"
        }"#).unwrap()
    }

    #[test]
    fn api_response_to_weather_report() {
        let report = WeatherReport::from(sample_api_response());
        assert_eq!(report.city, "Seattle");
        assert!((report.temp_celsius - 12.3).abs() < 0.01);
        assert_eq!(report.description, "overcast clouds");
    }

    #[test]
    fn display_format_includes_icon() {
        let report = WeatherReport {
            city: "Test".into(),
            temp_celsius: 20.0,
            description: "clear sky".into(),
            humidity: 50,
            wind_speed: 3.0,
        };
        let output = format!("{report}");
        assert!(output.contains("☀️"));
        assert!(output.contains("20°C"));
    }

    #[test]
    fn empty_weather_array_defaults_to_unknown() {
        let json = r#"{
            "main": {"temp": 0.0, "humidity": 0},
            "weather": [],
            "wind": {"speed": 0.0},
            "name": "Nowhere"
        }"#;
        let api: ApiResponse = serde_json::from_str(json).unwrap();
        let report = WeatherReport::from(api);
        assert_eq!(report.description, "Unknown");
    }
}
```

---

### Final File Layout

```
weather-cli/
├── Cargo.toml
├── src/
│   ├── main.rs        # CLI entry point (clap)
│   ├── lib.rs         # Module declarations
│   ├── client.rs      # HTTP client (reqwest + tokio)
│   ├── weather.rs     # Data types + From impl + tests
│   ├── display.rs     # Display formatting
│   └── error.rs       # WeatherError + Result alias
└── tests/
    └── integration.rs # Integration tests
```

Compare to the C# equivalent:
```
WeatherCli/
├── WeatherCli.csproj
├── Program.cs
├── Services/
│   └── WeatherClient.cs
├── Models/
│   ├── ApiResponse.cs
│   └── WeatherReport.cs
└── Tests/
    └── WeatherTests.cs
```

**The Rust version is remarkably similar in structure.** The main differences are:
- `mod` declarations instead of namespaces
- `Result<T, E>` instead of exceptions
- `From` trait instead of AutoMapper
- Explicit `#[tokio::main]` instead of built-in async runtime

### Bonus: Integration Test Stub

Create `tests/integration.rs` to test the public API without hitting a real server:

```rust
// tests/integration.rs
use weather_cli::weather::WeatherReport;

#[test]
fn weather_report_display_roundtrip() {
    let report = WeatherReport {
        city: "Seattle".into(),
        temp_celsius: 12.3,
        description: "overcast clouds".into(),
        humidity: 82,
        wind_speed: 5.4,
    };

    let output = format!("{report}");
    assert!(output.contains("Seattle"));
    assert!(output.contains("12°C"));
    assert!(output.contains("82%"));
}
```

Run with `cargo test` — Rust discovers tests in both `src/` (`#[cfg(test)]` modules) and `tests/` (integration tests) automatically. No test framework configuration needed — compare that to setting up xUnit/NUnit in C#.

---

### Extension Challenges

Once it works, try these to deepen your skills:

1. **Add caching** — Store the last API response in a file. On startup, check if it's less than 10 minutes old and skip the HTTP call. This exercises `std::fs`, `serde_json::to_writer`, and `SystemTime`.

2. **Add multiple cities** — Accept `--city "Seattle,Portland,Vancouver"` and fetch all concurrently with `tokio::join!`. This exercises concurrent async.

3. **Add a `--format json` flag** — Output the report as JSON instead of human-readable text using `serde_json::to_string_pretty`. This exercises conditional formatting and `Serialize`.

4. **Write an integration test** — Create `tests/integration.rs` that tests the full flow with a mock HTTP server using `wiremock`. This exercises the `tests/` directory pattern from ch14-1.

***
