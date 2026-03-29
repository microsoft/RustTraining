# Applied Walkthrough — Type-Safe Redfish Server 🟡

> **What you'll learn:** How to compose response builder type-state, source-availability tokens, dimensional serialization, health rollup, schema versioning, and typed action dispatch into a Redfish server that **cannot produce a schema-non-compliant response** — the mirror of the client walkthrough in [ch17](ch17-redfish-applied-walkthrough.md).
>
> **Cross-references:** [ch02](ch02-typed-command-interfaces-request-determi.md) (typed commands — inverted for action dispatch), [ch04](ch04-capability-tokens-zero-cost-proof-of-aut.md) (capability tokens — source availability), [ch06](ch06-dimensional-analysis-making-the-compiler.md) (dimensional types — serialization side), [ch07](ch07-validated-boundaries-parse-dont-validate.md) (validated boundaries — inverted: "construct, don't serialize"), [ch09](ch09-phantom-types-for-resource-tracking.md) (phantom types — schema versioning), [ch11](ch11-fourteen-tricks-from-the-trenches.md) (trick 3 — `#[non_exhaustive]`, trick 4 — builder type-state), [ch17](ch17-redfish-applied-walkthrough.md) (client counterpart)

## The Mirror Problem

Chapter 17 asks: *"How do I consume Redfish correctly?"* This chapter asks the
mirror question: *"How do I produce Redfish correctly?"*

On the client side, the danger is **trusting** bad data. On the server side, the
danger is **emitting** bad data — and every client in the fleet trusts what you
send.

A single `GET /redfish/v1/Systems/1` response must fuse data from many sources:

```mermaid
flowchart LR
    subgraph Sources
        SMBIOS["SMBIOS<br>Type 1, Type 17"]
        SDR["IPMI Sensors<br>(SDR + readings)"]
        SEL["IPMI SEL<br>(critical events)"]
        PCIe["PCIe Config<br>Space"]
        FW["Firmware<br>Version Table"]
        PWR["Power State<br>Register"]
    end

    subgraph Server["Redfish Server"]
        Handler["GET handler"]
        Builder["ComputerSystem<br>Builder"]
    end

    SMBIOS -->|"Name, UUID, Serial"| Handler
    SDR -->|"Temperatures, Fans"| Handler
    SEL -->|"Health escalation"| Handler
    PCIe -->|"Device links"| Handler
    FW -->|"BIOS version"| Handler
    PWR -->|"PowerState"| Handler
    Handler --> Builder
    Builder -->|".build()"| JSON["Schema-compliant<br>JSON response"]

    style JSON fill:#c8e6c9,color:#000
    style Builder fill:#e1f5fe,color:#000
```

In C, this is a 500-line handler that calls into six subsystems, manually builds
a JSON tree with `json_object_set()`, and hopes every required field was populated.
Forget one? The response violates the Redfish schema. Get the unit wrong? Every
client sees corrupted telemetry.

```c
// C — the assembly problem
json_t *get_computer_system(const char *id) {
    json_t *obj = json_object();
    json_object_set_new(obj, "@odata.type",
        json_string("#ComputerSystem.v1_13_0.ComputerSystem"));

    // 🐛 Forgot to set "Name" — schema requires it
    // 🐛 Forgot to set "UUID" — schema requires it

    smbios_type1_t *t1 = smbios_get_type1();
    if (t1) {
        json_object_set_new(obj, "Manufacturer",
            json_string(t1->manufacturer));
    }

    json_object_set_new(obj, "PowerState",
        json_string(get_power_state()));  // at least this one is always available

    // 🐛 Reading is in raw ADC counts, not Celsius — no type to catch it
    double cpu_temp = read_sensor(SENSOR_CPU_TEMP);
    // This number ends up in a Thermal response somewhere else...
    // but nothing ties it to "Celsius" at the type level

    // 🐛 Health is manually computed — forgot to include PSU status
    json_object_set_new(obj, "Status",
        build_status("Enabled", "OK")); // should be "Critical" — PSU is failing

    return obj; // missing 2 required fields, wrong health, raw units
}
```

Four bugs in one handler. On the client side, each bug affects **one** client.
On the server side, each bug affects **every** client that queries this BMC.

---

## Section 1 — Response Builder Type-State: "Construct, Don't Serialize" (ch07 Inverted)

Chapter 7 teaches "parse, don't validate" — validate inbound data once, carry the
proof in a type. The server-side mirror is **"construct, don't serialize"** — build
the outbound response through a builder that gates `.build()` on all required fields
being present.

```rust,ignore
use std::marker::PhantomData;

// ──── Type-level field tracking ────

pub struct HasField;
pub struct MissingField;

// ──── Response Builder ────

/// Builder for a ComputerSystem Redfish resource.
/// Type parameters track which REQUIRED fields have been supplied.
/// Optional fields don't need type-level tracking.
pub struct ComputerSystemBuilder<Name, Uuid, PowerState, Status> {
    // Required fields — tracked at the type level
    name: Option<String>,
    uuid: Option<String>,
    power_state: Option<PowerStateValue>,
    status: Option<ResourceStatus>,
    // Optional fields — not tracked (always settable)
    manufacturer: Option<String>,
    model: Option<String>,
    serial_number: Option<String>,
    bios_version: Option<String>,
    processor_summary: Option<ProcessorSummary>,
    memory_summary: Option<MemorySummary>,
    _markers: PhantomData<(Name, Uuid, PowerState, Status)>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub enum PowerStateValue { On, Off, PoweringOn, PoweringOff }

#[derive(Debug, Clone, serde::Serialize)]
pub struct ResourceStatus {
    #[serde(rename = "State")]
    pub state: StatusState,
    #[serde(rename = "Health")]
    pub health: HealthValue,
    #[serde(rename = "HealthRollup", skip_serializing_if = "Option::is_none")]
    pub health_rollup: Option<HealthValue>,
}

#[derive(Debug, Clone, Copy, serde::Serialize)]
pub enum StatusState { Enabled, Disabled, Absent, StandbyOffline, Starting }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
pub enum HealthValue { OK, Warning, Critical }

#[derive(Debug, Clone, serde::Serialize)]
pub struct ProcessorSummary {
    #[serde(rename = "Count")]
    pub count: u32,
    #[serde(rename = "Status")]
    pub status: ResourceStatus,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MemorySummary {
    #[serde(rename = "TotalSystemMemoryGiB")]
    pub total_gib: f64,
    #[serde(rename = "Status")]
    pub status: ResourceStatus,
}

// ──── Constructor: all fields start MissingField ────

impl ComputerSystemBuilder<MissingField, MissingField, MissingField, MissingField> {
    pub fn new() -> Self {
        ComputerSystemBuilder {
            name: None, uuid: None, power_state: None, status: None,
            manufacturer: None, model: None, serial_number: None,
            bios_version: None, processor_summary: None, memory_summary: None,
            _markers: PhantomData,
        }
    }
}

// ──── Required field setters — each transitions one type parameter ────

impl<U, P, S> ComputerSystemBuilder<MissingField, U, P, S> {
    pub fn name(self, name: String) -> ComputerSystemBuilder<HasField, U, P, S> {
        ComputerSystemBuilder {
            name: Some(name), uuid: self.uuid,
            power_state: self.power_state, status: self.status,
            manufacturer: self.manufacturer, model: self.model,
            serial_number: self.serial_number, bios_version: self.bios_version,
            processor_summary: self.processor_summary,
            memory_summary: self.memory_summary, _markers: PhantomData,
        }
    }
}

impl<N, P, S> ComputerSystemBuilder<N, MissingField, P, S> {
    pub fn uuid(self, uuid: String) -> ComputerSystemBuilder<N, HasField, P, S> {
        ComputerSystemBuilder {
            name: self.name, uuid: Some(uuid),
            power_state: self.power_state, status: self.status,
            manufacturer: self.manufacturer, model: self.model,
            serial_number: self.serial_number, bios_version: self.bios_version,
            processor_summary: self.processor_summary,
            memory_summary: self.memory_summary, _markers: PhantomData,
        }
    }
}

impl<N, U, S> ComputerSystemBuilder<N, U, MissingField, S> {
    pub fn power_state(self, ps: PowerStateValue)
        -> ComputerSystemBuilder<N, U, HasField, S>
    {
        ComputerSystemBuilder {
            name: self.name, uuid: self.uuid,
            power_state: Some(ps), status: self.status,
            manufacturer: self.manufacturer, model: self.model,
            serial_number: self.serial_number, bios_version: self.bios_version,
            processor_summary: self.processor_summary,
            memory_summary: self.memory_summary, _markers: PhantomData,
        }
    }
}

impl<N, U, P> ComputerSystemBuilder<N, U, P, MissingField> {
    pub fn status(self, status: ResourceStatus)
        -> ComputerSystemBuilder<N, U, P, HasField>
    {
        ComputerSystemBuilder {
            name: self.name, uuid: self.uuid,
            power_state: self.power_state, status: Some(status),
            manufacturer: self.manufacturer, model: self.model,
            serial_number: self.serial_number, bios_version: self.bios_version,
            processor_summary: self.processor_summary,
            memory_summary: self.memory_summary, _markers: PhantomData,
        }
    }
}

// ──── Optional field setters — available in any state ────

impl<N, U, P, S> ComputerSystemBuilder<N, U, P, S> {
    pub fn manufacturer(mut self, m: String) -> Self {
        self.manufacturer = Some(m); self
    }
    pub fn model(mut self, m: String) -> Self {
        self.model = Some(m); self
    }
    pub fn serial_number(mut self, s: String) -> Self {
        self.serial_number = Some(s); self
    }
    pub fn bios_version(mut self, v: String) -> Self {
        self.bios_version = Some(v); self
    }
    pub fn processor_summary(mut self, ps: ProcessorSummary) -> Self {
        self.processor_summary = Some(ps); self
    }
    pub fn memory_summary(mut self, ms: MemorySummary) -> Self {
        self.memory_summary = Some(ms); self
    }
}

// ──── .build() ONLY exists when all required fields are HasField ────

impl ComputerSystemBuilder<HasField, HasField, HasField, HasField> {
    pub fn build(self, id: &str) -> serde_json::Value {
        let mut obj = serde_json::json!({
            "@odata.id": format!("/redfish/v1/Systems/{id}"),
            "@odata.type": "#ComputerSystem.v1_13_0.ComputerSystem",
            "Id": id,
            // Type-state guarantees these are Some — .unwrap() is safe here.
            // In production, prefer .expect("guaranteed by type state").
            "Name": self.name.unwrap(),
            "UUID": self.uuid.unwrap(),
            "PowerState": self.power_state.unwrap(),
            "Status": self.status.unwrap(),
        });

        // Optional fields — included only if present
        if let Some(m) = self.manufacturer {
            obj["Manufacturer"] = serde_json::json!(m);
        }
        if let Some(m) = self.model {
            obj["Model"] = serde_json::json!(m);
        }
        if let Some(s) = self.serial_number {
            obj["SerialNumber"] = serde_json::json!(s);
        }
        if let Some(v) = self.bios_version {
            obj["BiosVersion"] = serde_json::json!(v);
        }
        // NOTE: .unwrap() on to_value() is used for brevity.
        // Production code should propagate serialization errors with `?`.
        if let Some(ps) = self.processor_summary {
            obj["ProcessorSummary"] = serde_json::to_value(ps).unwrap();
        }
        if let Some(ms) = self.memory_summary {
            obj["MemorySummary"] = serde_json::to_value(ms).unwrap();
        }

        obj
    }
}

//
// ── The Compiler Enforces Completeness ──
//
// ✅ All required fields set — .build() is available:
// ComputerSystemBuilder::new()
//     .name("PowerEdge R750".into())
//     .uuid("4c4c4544-...".into())
//     .power_state(PowerStateValue::On)
//     .status(ResourceStatus { ... })
//     .manufacturer("Dell".into())        // optional — fine to include
//     .build("1")
//
// ❌ Missing "Name" — compile error:
// ComputerSystemBuilder::new()
//     .uuid("4c4c4544-...".into())
//     .power_state(PowerStateValue::On)
//     .status(ResourceStatus { ... })
//     .build("1")
//   ERROR: method `build` not found for
//   `ComputerSystemBuilder<MissingField, HasField, HasField, HasField>`
```

**Bug class eliminated:** schema-non-compliant responses. The handler physically
cannot serialize a `ComputerSystem` without supplying every required field. The
compiler error message even tells you *which* field is missing — it's right there
in the type parameter: `MissingField` in the `Name` position.

---

## Section 2 — Source-Availability Tokens (Capability Tokens, ch04 — New Twist)

In ch04 and ch17, capability tokens prove **authorization** — "the caller is
allowed to do this." On the server side, the same pattern proves **availability** —
"this data source was successfully initialized."

Each subsystem the BMC queries can fail independently. SMBIOS tables might be
corrupt. The sensor subsystem might still be initializing. PCIe bus scan might
have timed out. Encode each as a proof token:

```rust,ignore
/// Proof that SMBIOS tables were successfully parsed.
/// Only produced by the SMBIOS init function.
pub struct SmbiosReady {
    _private: (),
}

/// Proof that IPMI sensor subsystem is responsive.
pub struct SensorsReady {
    _private: (),
}

/// Proof that PCIe bus scan completed.
pub struct PcieReady {
    _private: (),
}

/// Proof that the SEL was successfully read.
pub struct SelReady {
    _private: (),
}

// ──── Data source initialization ────

pub struct SmbiosTables {
    pub product_name: String,
    pub manufacturer: String,
    pub serial_number: String,
    pub uuid: String,
}

pub struct SensorCache {
    pub cpu_temp: Celsius,
    pub inlet_temp: Celsius,
    pub fan_readings: Vec<(String, Rpm)>,
    pub psu_power: Vec<(String, Watts)>,
}

/// Rich SEL summary — per-subsystem health derived from typed events.
/// Built by the consumer pipeline in ch07's SEL section.
/// Replaces the lossy `has_critical_events: bool` with typed granularity.
pub struct TypedSelSummary {
    pub total_entries: u32,
    pub processor_health: HealthValue,
    pub memory_health: HealthValue,
    pub power_health: HealthValue,
    pub thermal_health: HealthValue,
    pub fan_health: HealthValue,
    pub storage_health: HealthValue,
    pub security_health: HealthValue,
}

pub fn init_smbios() -> Option<(SmbiosReady, SmbiosTables)> {
    // Read SMBIOS entry point, parse tables...
    // Returns None if tables are absent or corrupt
    Some((
        SmbiosReady { _private: () },
        SmbiosTables {
            product_name: "PowerEdge R750".into(),
            manufacturer: "Dell Inc.".into(),
            serial_number: "SVC1234567".into(),
            uuid: "4c4c4544-004d-5610-804c-b2c04f435031".into(),
        },
    ))
}

pub fn init_sensors() -> Option<(SensorsReady, SensorCache)> {
    // Initialize SDR repository, read all sensors...
    // Returns None if IPMI subsystem is not responsive
    Some((
        SensorsReady { _private: () },
        SensorCache {
            cpu_temp: Celsius(68.0),
            inlet_temp: Celsius(24.0),
            fan_readings: vec![
                ("Fan1".into(), Rpm(8400)),
                ("Fan2".into(), Rpm(8200)),
            ],
            psu_power: vec![
                ("PSU1".into(), Watts(285.0)),
                ("PSU2".into(), Watts(290.0)),
            ],
        },
    ))
}

pub fn init_sel() -> Option<(SelReady, TypedSelSummary)> {
    // In production: read SEL entries, parse via ch07's TryFrom,
    // classify via classify_event_health(), aggregate via summarize_sel().
    Some((
        SelReady { _private: () },
        TypedSelSummary {
            total_entries: 42,
            processor_health: HealthValue::OK,
            memory_health: HealthValue::OK,
            power_health: HealthValue::OK,
            thermal_health: HealthValue::OK,
            fan_health: HealthValue::OK,
            storage_health: HealthValue::OK,
            security_health: HealthValue::OK,
        },
    ))
}
```

Now, functions that populate builder fields from a data source **require the
corresponding proof token**:

```rust,ignore
/// Populate SMBIOS-sourced fields. Requires proof SMBIOS is available.
fn populate_from_smbios<P, S>(
    builder: ComputerSystemBuilder<MissingField, MissingField, P, S>,
    _proof: &SmbiosReady,
    tables: &SmbiosTables,
) -> ComputerSystemBuilder<HasField, HasField, P, S> {
    builder
        .name(tables.product_name.clone())
        .uuid(tables.uuid.clone())
        .manufacturer(tables.manufacturer.clone())
        .serial_number(tables.serial_number.clone())
}

/// Fallback when SMBIOS is unavailable — supplies required fields
/// with safe defaults.
fn populate_smbios_fallback<P, S>(
    builder: ComputerSystemBuilder<MissingField, MissingField, P, S>,
) -> ComputerSystemBuilder<HasField, HasField, P, S> {
    builder
        .name("Unknown System".into())
        .uuid("00000000-0000-0000-0000-000000000000".into())
}
```

The handler chooses the path based on which tokens are available:

```rust,ignore
fn build_computer_system(
    smbios: &Option<(SmbiosReady, SmbiosTables)>,
    power_state: PowerStateValue,
    health: ResourceStatus,
) -> serde_json::Value {
    let builder = ComputerSystemBuilder::new()
        .power_state(power_state)
        .status(health);

    let builder = match smbios {
        Some((proof, tables)) => populate_from_smbios(builder, proof, tables),
        None => populate_smbios_fallback(builder),
    };

    // Both paths produce HasField for Name and UUID.
    // .build() is available either way.
    builder.build("1")
}
```

**Bug class eliminated:** calling into a subsystem that failed initialization.
If SMBIOS didn't parse, you don't have a `SmbiosReady` token — the compiler forces
you through the fallback path. No runtime `if (smbios != NULL)` to forget.

### Combining Source Tokens with Capability Mixins (ch08)

With multiple Redfish resource types to serve (ComputerSystem, Chassis, Manager,
Thermal, Power), source-population logic repeats across handlers. The **mixin**
pattern from ch08 eliminates this duplication. Declare what sources a handler has,
and blanket impls provide the population methods automatically:

```rust,ignore
/// ── Ingredient Traits (ch08) for data sources ──

pub trait HasSmbios {
    fn smbios(&self) -> &(SmbiosReady, SmbiosTables);
}

pub trait HasSensors {
    fn sensors(&self) -> &(SensorsReady, SensorCache);
}

pub trait HasSel {
    fn sel(&self) -> &(SelReady, TypedSelSummary);
}

/// ── Mixin: any handler with SMBIOS + Sensors gets identity population ──

pub trait IdentityMixin: HasSmbios {
    fn populate_identity<P, S>(
        &self,
        builder: ComputerSystemBuilder<MissingField, MissingField, P, S>,
    ) -> ComputerSystemBuilder<HasField, HasField, P, S> {
        let (_, tables) = self.smbios();
        builder
            .name(tables.product_name.clone())
            .uuid(tables.uuid.clone())
            .manufacturer(tables.manufacturer.clone())
            .serial_number(tables.serial_number.clone())
    }
}

/// Auto-implement for any type that has SMBIOS capability.
impl<T: HasSmbios> IdentityMixin for T {}

/// ── Mixin: any handler with Sensors + SEL gets health rollup ──

pub trait HealthMixin: HasSensors + HasSel {
    fn compute_health(&self) -> ResourceStatus {
        let (_, cache) = self.sensors();
        let (_, sel_summary) = self.sel();
        compute_system_health(
            Some(&(SensorsReady { _private: () }, cache.clone())).as_ref(),
            Some(&(SelReady { _private: () }, sel_summary.clone())).as_ref(),
        )
    }
}

impl<T: HasSensors + HasSel> HealthMixin for T {}

/// ── Concrete handler owns available sources ──

struct FullPlatformHandler {
    smbios: (SmbiosReady, SmbiosTables),
    sensors: (SensorsReady, SensorCache),
    sel: (SelReady, TypedSelSummary),
}

impl HasSmbios  for FullPlatformHandler {
    fn smbios(&self) -> &(SmbiosReady, SmbiosTables) { &self.smbios }
}
impl HasSensors for FullPlatformHandler {
    fn sensors(&self) -> &(SensorsReady, SensorCache) { &self.sensors }
}
impl HasSel     for FullPlatformHandler {
    fn sel(&self) -> &(SelReady, TypedSelSummary) { &self.sel }
}

// FullPlatformHandler automatically gets:
//   IdentityMixin::populate_identity()   (via HasSmbios)
//   HealthMixin::compute_health()        (via HasSensors + HasSel)
//
// A SensorsOnlyHandler that impls HasSensors but NOT HasSel
// would get IdentityMixin (if it has SMBIOS) but NOT HealthMixin.
// Calling .compute_health() on it → compile error.
```

This directly mirrors ch08's `BaseBoardController` pattern: ingredient traits
declare what you have, mixin traits provide behavior via blanket impls, and
the compiler gates each mixin on its prerequisites. Adding a new data
source (e.g., `HasNvme`) plus a mixin (e.g., `StorageMixin: HasNvme + HasSel`)
gives health rollup for storage to every handler that has both — automatically.

---

## Section 3 — Dimensional Types at the Serialization Boundary (ch06)

On the client side (ch17 §4), dimensional types prevent **reading** °C as RPM.
On the server side, they prevent **writing** RPM into a Celsius JSON field. This
is arguably more dangerous — a wrong value on the server propagates to every client.

```rust,ignore
use serde::Serialize;

// ──── Dimensional types from ch06, with Serialize ────

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize)]
pub struct Celsius(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize)]
pub struct Rpm(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize)]
pub struct Watts(pub f64);

// ──── Redfish Thermal response members ────
// Field types enforce which unit belongs in which JSON property.

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TemperatureMember {
    pub member_id: String,
    pub name: String,
    pub reading_celsius: Celsius,           // ← must be Celsius
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_threshold_critical: Option<Celsius>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_threshold_fatal: Option<Celsius>,
    pub status: ResourceStatus,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct FanMember {
    pub member_id: String,
    pub name: String,
    pub reading: Rpm,                       // ← must be Rpm
    pub reading_units: &'static str,        // always "RPM"
    pub status: ResourceStatus,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PowerControlMember {
    pub member_id: String,
    pub name: String,
    pub power_consumed_watts: Watts,        // ← must be Watts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_capacity_watts: Option<Watts>,
    pub status: ResourceStatus,
}

// ──── Building a Thermal response from sensor cache ────

fn build_thermal_response(
    _proof: &SensorsReady,
    cache: &SensorCache,
) -> serde_json::Value {
    let temps = vec![
        TemperatureMember {
            member_id: "0".into(),
            name: "CPU Temp".into(),
            reading_celsius: cache.cpu_temp,     // Celsius → Celsius ✅
            upper_threshold_critical: Some(Celsius(95.0)),
            upper_threshold_fatal: Some(Celsius(105.0)),
            status: ResourceStatus {
                state: StatusState::Enabled,
                health: if cache.cpu_temp < Celsius(95.0) {
                    HealthValue::OK
                } else {
                    HealthValue::Critical
                },
                health_rollup: None,
            },
        },
        TemperatureMember {
            member_id: "1".into(),
            name: "Inlet Temp".into(),
            reading_celsius: cache.inlet_temp,   // Celsius → Celsius ✅
            upper_threshold_critical: Some(Celsius(42.0)),
            upper_threshold_fatal: None,
            status: ResourceStatus {
                state: StatusState::Enabled,
                health: HealthValue::OK,
                health_rollup: None,
            },
        },

        // ❌ Compile error — can't put Rpm in a Celsius field:
        // TemperatureMember {
        //     reading_celsius: cache.fan_readings[0].1,  // Rpm ≠ Celsius
        //     ...
        // }
    ];

    let fans: Vec<FanMember> = cache.fan_readings.iter().enumerate().map(|(i, (name, rpm))| {
        FanMember {
            member_id: i.to_string(),
            name: name.clone(),
            reading: *rpm,                       // Rpm → Rpm ✅
            reading_units: "RPM",
            status: ResourceStatus {
                state: StatusState::Enabled,
                health: if *rpm > Rpm(1000) { HealthValue::OK } else { HealthValue::Critical },
                health_rollup: None,
            },
        }
    }).collect();

    serde_json::json!({
        "@odata.type": "#Thermal.v1_7_0.Thermal",
        "Temperatures": temps,
        "Fans": fans,
    })
}
```

**Bug class eliminated:** unit confusion at serialization. The Redfish schema says
`ReadingCelsius` is in °C. The Rust type system says `reading_celsius` must be
`Celsius`. If a developer accidentally passes `Rpm(8400)` or `Watts(285.0)`, the
compiler catches it before the value ever reaches JSON.

---

## Section 4 — Health Rollup as a Typed Fold

Redfish `Status.Health` is a *rollup* — the worst health of all sub-components.
In C, this is typically a series of `if` checks that inevitably misses a source.
With typed enums and `Ord`, the rollup is a one-line fold — and the compiler
ensures every source contributes:

```rust,ignore
/// Roll up health from multiple sources.
/// Ord on HealthValue: OK < Warning < Critical.
/// Returns the worst (max) value.
fn rollup(sources: &[HealthValue]) -> HealthValue {
    sources.iter().copied().max().unwrap_or(HealthValue::OK)
}

/// Compute system-level health from all sub-components.
/// Takes explicit references to every source — the caller must provide ALL of them.
fn compute_system_health(
    sensors: Option<&(SensorsReady, SensorCache)>,
    sel: Option<&(SelReady, TypedSelSummary)>,
) -> ResourceStatus {
    let mut inputs = Vec::new();

    // ── Live sensor readings ──
    if let Some((_proof, cache)) = sensors {
        // Temperature health (dimensional: Celsius comparison)
        if cache.cpu_temp > Celsius(95.0) {
            inputs.push(HealthValue::Critical);
        } else if cache.cpu_temp > Celsius(85.0) {
            inputs.push(HealthValue::Warning);
        } else {
            inputs.push(HealthValue::OK);
        }

        // Fan health (dimensional: Rpm comparison)
        for (_name, rpm) in &cache.fan_readings {
            if *rpm < Rpm(500) {
                inputs.push(HealthValue::Critical);
            } else if *rpm < Rpm(1000) {
                inputs.push(HealthValue::Warning);
            } else {
                inputs.push(HealthValue::OK);
            }
        }

        // PSU health (dimensional: Watts comparison)
        for (_name, watts) in &cache.psu_power {
            if *watts > Watts(800.0) {
                inputs.push(HealthValue::Critical);
            } else {
                inputs.push(HealthValue::OK);
            }
        }
    }

    // ── SEL per-subsystem health (from ch07's TypedSelSummary) ──
    // Each subsystem's health was derived by exhaustive matching over
    // every sensor type and event variant. No information was lost.
    if let Some((_proof, sel_summary)) = sel {
        inputs.push(sel_summary.processor_health);
        inputs.push(sel_summary.memory_health);
        inputs.push(sel_summary.power_health);
        inputs.push(sel_summary.thermal_health);
        inputs.push(sel_summary.fan_health);
        inputs.push(sel_summary.storage_health);
        inputs.push(sel_summary.security_health);
    }

    let health = rollup(&inputs);

    ResourceStatus {
        state: StatusState::Enabled,
        health,
        health_rollup: Some(health),
    }
}
```

**Bug class eliminated:** incomplete health rollup. In C, forgetting to include PSU
status in the health calculation is a silent bug — the system reports "OK" while a
PSU is failing. Here, `compute_system_health` takes explicit references to every
data source. The SEL contribution is no longer a lossy `bool` — it's seven
per-subsystem `HealthValue` fields derived by exhaustive matching in ch07's consumer
pipeline. Adding a new SEL sensor type forces the classifier to handle it; adding a
new subsystem field forces the rollup to include it.

---

## Section 5 — Schema Versioning with Phantom Types (ch09)

If the BMC advertises `ComputerSystem.v1_13_0`, the response **must** include
properties introduced in that schema version (`LastResetTime`, `BootProgress`).
Advertising v1.13 without those fields is a Redfish Interop Validator failure.
Phantom version markers make this a compile-time contract:

```rust,ignore
use std::marker::PhantomData;

// ──── Schema Version Markers ────

pub struct V1_5;
pub struct V1_13;

// ──── Version-Aware Response ────

pub struct ComputerSystemResponse<V> {
    pub base: ComputerSystemBase,
    _version: PhantomData<V>,
}

pub struct ComputerSystemBase {
    pub id: String,
    pub name: String,
    pub uuid: String,
    pub power_state: PowerStateValue,
    pub status: ResourceStatus,
    pub manufacturer: Option<String>,
    pub serial_number: Option<String>,
    pub bios_version: Option<String>,
}

// Methods available on ALL versions:
impl<V> ComputerSystemResponse<V> {
    pub fn base_json(&self) -> serde_json::Value {
        serde_json::json!({
            "Id": self.base.id,
            "Name": self.base.name,
            "UUID": self.base.uuid,
            "PowerState": self.base.power_state,
            "Status": self.base.status,
        })
    }
}

// ──── v1.13-specific fields ────

/// Date and time of the last system reset.
pub struct LastResetTime(pub String);

/// Boot progress information.
pub struct BootProgress {
    pub last_state: String,
    pub last_state_time: String,
}

impl ComputerSystemResponse<V1_13> {
    /// LastResetTime — REQUIRED in v1.13+.
    /// This method only exists on V1_13. If the BMC advertises v1.13
    /// and the handler doesn't call this, the field is missing.
    pub fn last_reset_time(&self) -> LastResetTime {
        // Read from RTC or boot timestamp register
        LastResetTime("2026-03-16T08:30:00Z".to_string())
    }

    /// BootProgress — REQUIRED in v1.13+.
    pub fn boot_progress(&self) -> BootProgress {
        BootProgress {
            last_state: "OSRunning".to_string(),
            last_state_time: "2026-03-16T08:32:00Z".to_string(),
        }
    }

    /// Build the full v1.13 JSON response, including version-specific fields.
    pub fn to_json(&self) -> serde_json::Value {
        let mut obj = self.base_json();
        obj["@odata.type"] =
            serde_json::json!("#ComputerSystem.v1_13_0.ComputerSystem");

        let reset_time = self.last_reset_time();
        obj["LastResetTime"] = serde_json::json!(reset_time.0);

        let boot = self.boot_progress();
        obj["BootProgress"] = serde_json::json!({
            "LastState": boot.last_state,
            "LastStateTime": boot.last_state_time,
        });

        obj
    }
}

impl ComputerSystemResponse<V1_5> {
    /// v1.5 JSON — no LastResetTime, no BootProgress.
    pub fn to_json(&self) -> serde_json::Value {
        let mut obj = self.base_json();
        obj["@odata.type"] =
            serde_json::json!("#ComputerSystem.v1_5_0.ComputerSystem");
        obj
    }

    // last_reset_time() doesn't exist here.
    // Calling it → compile error:
    //   let resp: ComputerSystemResponse<V1_5> = ...;
    //   resp.last_reset_time();
    //   ❌ ERROR: method `last_reset_time` not found for
    //            `ComputerSystemResponse<V1_5>`
}
```

**Bug class eliminated:** schema version mismatch. If the BMC is configured to
advertise v1.13, use `ComputerSystemResponse<V1_13>` and the compiler ensures
every v1.13-required field is produced. Downgrade to v1.5? Change the type
parameter — the v1.13 methods vanish, and no dead fields leak into the response.

---

## Section 6 — Typed Action Dispatch (ch02 Inverted)

In ch02, the typed command pattern binds `Request → Response` on the **client**
side. On the **server** side, the same pattern validates incoming action payloads
and dispatches them type-safely — the inverse direction.

```rust,ignore
use serde::Deserialize;

// ──── Action Trait (mirror of ch02's IpmiCmd trait) ────

/// A Redfish action: the framework deserializes Params from the POST body,
/// then calls execute(). If the JSON doesn't match Params, deserialization
/// fails — execute() is never called with bad input.
pub trait RedfishAction {
    /// The expected JSON body structure.
    type Params: serde::de::DeserializeOwned;
    /// The result of executing the action.
    type Result: serde::Serialize;

    fn execute(&self, params: Self::Params) -> Result<Self::Result, RedfishError>;
}

#[derive(Debug)]
pub enum RedfishError {
    InvalidPayload(String),
    ActionFailed(String),
}

// ──── ComputerSystem.Reset ────

pub struct ComputerSystemReset;

#[derive(Debug, Deserialize)]
pub enum ResetType {
    On,
    ForceOff,
    GracefulShutdown,
    GracefulRestart,
    ForceRestart,
    ForceOn,
    PushPowerButton,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResetParams {
    pub reset_type: ResetType,
}

impl RedfishAction for ComputerSystemReset {
    type Params = ResetParams;
    type Result = ();

    fn execute(&self, params: ResetParams) -> Result<(), RedfishError> {
        match params.reset_type {
            ResetType::GracefulShutdown => {
                // Send ACPI shutdown to host
                println!("Initiating ACPI shutdown");
                Ok(())
            }
            ResetType::ForceOff => {
                // Assert power-off to host
                println!("Forcing power off");
                Ok(())
            }
            ResetType::On | ResetType::ForceOn => {
                println!("Powering on");
                Ok(())
            }
            ResetType::GracefulRestart => {
                println!("ACPI restart");
                Ok(())
            }
            ResetType::ForceRestart => {
                println!("Forced restart");
                Ok(())
            }
            ResetType::PushPowerButton => {
                println!("Simulating power button press");
                Ok(())
            }
            // Exhaustive — compiler catches missing variants
        }
    }
}

// ──── Manager.ResetToDefaults ────

pub struct ManagerResetToDefaults;

#[derive(Debug, Deserialize)]
pub enum ResetToDefaultsType {
    ResetAll,
    PreserveNetworkAndUsers,
    PreserveNetwork,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResetToDefaultsParams {
    pub reset_to_defaults_type: ResetToDefaultsType,
}

impl RedfishAction for ManagerResetToDefaults {
    type Params = ResetToDefaultsParams;
    type Result = ();

    fn execute(&self, params: ResetToDefaultsParams) -> Result<(), RedfishError> {
        match params.reset_to_defaults_type {
            ResetToDefaultsType::ResetAll => {
                println!("Full factory reset");
                Ok(())
            }
            ResetToDefaultsType::PreserveNetworkAndUsers => {
                println!("Reset preserving network + users");
                Ok(())
            }
            ResetToDefaultsType::PreserveNetwork => {
                println!("Reset preserving network config");
                Ok(())
            }
        }
    }
}

// ──── Generic Action Dispatcher ────

fn dispatch_action<A: RedfishAction>(
    action: &A,
    raw_body: &str,
) -> Result<A::Result, RedfishError> {
    // Deserialization validates the payload structure.
    // If the JSON doesn't match A::Params, this fails
    // and execute() is never called.
    let params: A::Params = serde_json::from_str(raw_body)
        .map_err(|e| RedfishError::InvalidPayload(e.to_string()))?;

    action.execute(params)
}

// ── Usage ──

fn handle_reset_action(body: &str) -> Result<(), RedfishError> {
    // Type-safe: ResetParams is validated by serde before execute()
    dispatch_action(&ComputerSystemReset, body)?;
    Ok(())

    // Invalid JSON: {"ResetType": "Explode"}
    // → serde error: "unknown variant `Explode`"
    // → execute() never called

    // Missing field: {}
    // → serde error: "missing field `ResetType`"
    // → execute() never called
}
```

**Bug classes eliminated:**
- **Invalid action payload:** serde rejects unknown enum variants and missing fields
  before `execute()` is called. No manual `if (body["ResetType"] == ...)` chains.
- **Missing variant handling:** `match params.reset_type` is exhaustive — adding a
  new `ResetType` variant forces every action handler to be updated.
- **Type confusion:** `ComputerSystemReset` expects `ResetParams`;
  `ManagerResetToDefaults` expects `ResetToDefaultsParams`. The trait system prevents
  passing one action's params to another action's handler.

---

## Section 7 — Putting It All Together: The GET Handler

Here's the complete handler that composes all six sections into a single
schema-compliant response:

```rust,ignore
/// Complete GET /redfish/v1/Systems/1 handler.
///
/// Every required field is enforced by the builder type-state.
/// Every data source is gated by availability tokens.
/// Every unit is locked to its dimensional type.
/// Every health input feeds the typed rollup.
fn handle_get_computer_system(
    smbios: &Option<(SmbiosReady, SmbiosTables)>,
    sensors: &Option<(SensorsReady, SensorCache)>,
    sel: &Option<(SelReady, TypedSelSummary)>,
    power_state: PowerStateValue,
    bios_version: Option<String>,
) -> serde_json::Value {
    // ── 1. Health rollup (Section 4) ──
    // Folds health from sensors + SEL into a single typed status
    let health = compute_system_health(
        sensors.as_ref(),
        sel.as_ref(),
    );

    // ── 2. Builder type-state (Section 1) ──
    let builder = ComputerSystemBuilder::new()
        .power_state(power_state)
        .status(health);

    // ── 3. Source-availability tokens (Section 2) ──
    let builder = match smbios {
        Some((proof, tables)) => {
            // SMBIOS available — populate from hardware
            populate_from_smbios(builder, proof, tables)
        }
        None => {
            // SMBIOS unavailable — safe defaults
            populate_smbios_fallback(builder)
        }
    };

    // ── 4. Optional enrichment from sensors (Section 3) ──
    let builder = if let Some((_proof, cache)) = sensors {
        builder
            .processor_summary(ProcessorSummary {
                count: 2,
                status: ResourceStatus {
                    state: StatusState::Enabled,
                    health: if cache.cpu_temp < Celsius(95.0) {
                        HealthValue::OK
                    } else {
                        HealthValue::Critical
                    },
                    health_rollup: None,
                },
            })
    } else {
        builder
    };

    let builder = match bios_version {
        Some(v) => builder.bios_version(v),
        None => builder,
    };

    // ── 5. Build (Section 1) ──
    // .build() is available because both paths (SMBIOS present / absent)
    // produce HasField for Name and UUID. The compiler verified this.
    builder.build("1")
}

// ──── Server Startup ────

fn main() {
    // Initialize all data sources — each returns an availability token
    let smbios = init_smbios();
    let sensors = init_sensors();
    let sel = init_sel();

    // Simulate handler call
    let response = handle_get_computer_system(
        &smbios,
        &sensors,
        &sel,
        PowerStateValue::On,
        Some("2.10.1".into()),
    );

    // NOTE: .unwrap() is used for brevity — handle errors in production.
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
}
```

**Expected output:**

```json
{
  "@odata.id": "/redfish/v1/Systems/1",
  "@odata.type": "#ComputerSystem.v1_13_0.ComputerSystem",
  "Id": "1",
  "Name": "PowerEdge R750",
  "UUID": "4c4c4544-004d-5610-804c-b2c04f435031",
  "PowerState": "On",
  "Status": {
    "State": "Enabled",
    "Health": "OK",
    "HealthRollup": "OK"
  },
  "Manufacturer": "Dell Inc.",
  "SerialNumber": "SVC1234567",
  "BiosVersion": "2.10.1",
  "ProcessorSummary": {
    "Count": 2,
    "Status": {
      "State": "Enabled",
      "Health": "OK"
    }
  }
}
```

### What the Compiler Proves (Server Side)

| # | Bug class | How it's prevented | Pattern (Section) |
|---|-----------|-------------------|-------------------|
| 1 | Missing required field in response | `.build()` requires all type-state markers to be `HasField` | Builder type-state (§1) |
| 2 | Calling into failed subsystem | Source-availability tokens gate data access | Capability tokens (§2) |
| 3 | No fallback for unavailable source | Both `match` arms (present/absent) must produce `HasField` | Type-state + exhaustive match (§2) |
| 4 | Wrong unit in JSON field | `reading_celsius: Celsius` ≠ `Rpm` ≠ `Watts` | Dimensional types (§3) |
| 5 | Incomplete health rollup | `compute_system_health` takes explicit source refs; SEL provides per-subsystem `HealthValue` via ch07's `TypedSelSummary` | Typed function signature + exhaustive matching (§4) |
| 6 | Schema version mismatch | `ComputerSystemResponse<V1_13>` has `last_reset_time()`; `V1_5` doesn't | Phantom types (§5) |
| 7 | Invalid action payload accepted | serde rejects unknown/missing fields before `execute()` | Typed action dispatch (§6) |
| 8 | Missing action variant handling | `match params.reset_type` is exhaustive | Enum exhaustiveness (§6) |
| 9 | Wrong action params to wrong handler | `RedfishAction::Params` is an associated type | Typed commands inverted (§6) |

**Total runtime overhead: zero.** The builder markers, availability tokens, phantom
version types, and dimensional newtypes all compile away. The JSON produced is
identical to the hand-rolled C version — minus nine classes of bugs.

---

## The Mirror: Client vs. Server Pattern Map

| Concern | Client (ch17) | Server (this chapter) |
|---------|---------------|----------------------|
| **Boundary direction** | Inbound: JSON → typed values | Outbound: typed values → JSON |
| **Core principle** | "Parse, don't validate" | "Construct, don't serialize" |
| **Field completeness** | `TryFrom` validates required fields are present | Builder type-state gates `.build()` on required fields |
| **Unit safety** | `Celsius` ≠ `Rpm` when reading | `Celsius` ≠ `Rpm` when writing |
| **Privilege / availability** | Capability tokens gate requests | Availability tokens gate data source access |
| **Data sources** | Single source (BMC) | Multiple sources (SMBIOS, sensors, SEL, PCIe, ...) |
| **Schema version** | Phantom types prevent accessing unsupported fields | Phantom types enforce providing version-required fields |
| **Actions** | Client sends typed action POST | Server validates + dispatches via `RedfishAction` trait |
| **Health** | Read and trust `Status.Health` | Compute `Status.Health` via typed rollup |
| **Failure propagation** | One bad parse → one client error | One bad serialization → every client sees wrong data |

The two chapters form a complete story. Ch17: *"Every response I consume is
type-checked."* This chapter: *"Every response I produce is type-checked."* The
same patterns flow in both directions — the type system doesn't know or care
which end of the wire you're on.

## Key Takeaways

1. **"Construct, don't serialize"** is the server-side mirror of "parse, don't
   validate" — use builder type-state so `.build()` only exists when all required
   fields are present.
2. **Source-availability tokens prove initialization** — the same capability token
   pattern from ch04, repurposed to prove a data source is ready.
3. **Dimensional types protect producers and consumers** — putting `Rpm` in a
   `ReadingCelsius` field is a compile error, not a customer-reported bug.
4. **Health rollup is a typed fold** — `Ord` on `HealthValue` plus explicit source
   references mean the compiler catches "forgot to include PSU status."
5. **Schema versioning at the type level** — phantom type parameters make
   version-specific fields appear and disappear at compile time.
6. **Action dispatch inverts ch02** — `serde` deserializes the payload into a
   typed `Params` struct, and exhaustive matching on enum variants means adding a
   new `ResetType` forces every handler to be updated.
7. **Server-side bugs propagate to every client** — that's why compile-time
   correctness on the producer side is even more critical than on the consumer side.

---
