# Rust Migration Plan for bve-autopilot

## Table of Contents / 目次

1. [Migration Objectives / 移行の目的](#migration-objectives--移行の目的)
2. [Why Rust? / なぜRust？](#why-rust--なぜrust)
3. [English Naming Convention / 英語命名規則](#english-naming-convention--英語命名規則)
4. [Module Migration Mapping / モジュール移行マッピング](#module-migration-mapping--モジュール移行マッピング)
5. [Type System Migration / 型システム移行](#type-system-migration--型システム移行)
6. [Migration Phases / 移行フェーズ](#migration-phases--移行フェーズ)
7. [Build System / ビルドシステム](#build-system--ビルドシステム)
8. [FFI Strategy / FFI戦略](#ffi-strategy--ffi戦略)
9. [Testing Strategy / テスト戦略](#testing-strategy--テスト戦略)
10. [Timeline & Milestones / スケジュール](#timeline--milestones--スケジュール)

---

## Migration Objectives / 移行の目的

### Primary Goals / 主要目標

1. **Memory Safety / メモリ安全性:** Eliminate potential memory bugs through Rust's ownership system
2. **Performance / パフォーマンス:** Maintain or improve runtime performance with zero-cost abstractions
3. **Type Safety / 型安全性:** Enhance the existing type-safe physical quantity system
4. **Maintainability / 保守性:** Improve code maintainability with English identifiers and modern tooling
5. **Cross-platform / クロスプラットフォーム:** Potential for easier cross-platform support in the future

### Secondary Goals / 副次的目標

1. **Modern Tooling / モダンなツール:** Leverage Cargo and the Rust ecosystem
2. **Documentation / ドキュメント:** Generate documentation automatically with `cargo doc`
3. **Testing / テスト:** Improve test coverage with Rust's built-in test framework
4. **Community / コミュニティ:** Make the codebase more accessible to international contributors

---

## Why Rust? / なぜRust？

### Advantages for This Project / このプロジェクトにおける利点

| Aspect | C++20 | Rust | Benefit |
|--------|-------|------|---------|
| Memory Safety | Manual management | Compile-time guarantees | Fewer runtime bugs |
| Type Safety | Strong, but verbose | Strong with inference | Less boilerplate |
| Build System | MSBuild (Windows-only) | Cargo (cross-platform) | Better portability |
| Dependencies | Manual or vcpkg | Cargo.toml | Easier management |
| Documentation | Separate Markdown | Integrated doc comments | Auto-generated docs |
| Testing | Separate project | Built-in `#[test]` | Simpler test workflow |
| Package Management | Limited | crates.io | Rich ecosystem |

### Rust Features for This Domain / 鉄道制御に有用なRust機能

1. **Type-safe Units / 型安全な単位:** Use crates like `uom` (units of measurement)
2. **Real-time / リアルタイム:** No garbage collection pauses
3. **Concurrency / 並行性:** Safe concurrent programming (for future enhancements)
4. **Error Handling / エラー処理:** `Result<T, E>` for explicit error paths
5. **Pattern Matching / パターンマッチング:** Clean state machine implementation

---

## English Naming Convention / 英語命名規則

### General Principles / 一般原則

1. **Use snake_case:** Rust convention for variables, functions, modules
2. **Use PascalCase:** For types, structs, enums, traits
3. **Use SCREAMING_SNAKE_CASE:** For constants
4. **Be descriptive:** Prefer clarity over brevity
5. **Use domain terms:** Retain railway-specific terminology

### Translation Dictionary / 翻訳辞書

#### Core Modules / コアモジュール

| Japanese (日本語) | English | Module/Type Name |
|------------------|---------|------------------|
| 共通状態 | Shared State | `shared_state` |
| 運動状態 | Motion State | `motion_state` |
| 稼働状態 | Operating State | `operating_state` |
| 環境設定 | Configuration | `configuration` |
| 物理量 | Physical Quantity | `physical_quantity` |
| 制御指令 | Control Command | `control_command` |

#### TASC & ATO Modules

| Japanese (日本語) | English | Module/Type Name |
|------------------|---------|------------------|
| 定位置停止制御 / TASC | Target Station Stopping Control | `tasc` |
| 自動列車運転 / ATO | Automatic Train Operation | `ato` |
| 信号順守 | Signal Compliance | `signal_compliance` |
| 早着防止 | Early Arrival Prevention | `early_arrival_prevention` |
| 減速パターン | Deceleration Pattern | `deceleration_pattern` |
| 減速目標 | Deceleration Target | `deceleration_target` |
| 制動指令計算 | Braking Command Calculation | `braking_command_calc` |
| 出力制御 | Output Control | `output_control` |
| 急動作抑制 | Sudden Motion Suppression | `sudden_motion_suppression` |

#### Vehicle & Environment

| Japanese (日本語) | English | Module/Type Name |
|------------------|---------|------------------|
| 制動特性 | Brake Characteristics | `brake_characteristics` |
| 力行特性 | Power Characteristics | `power_characteristics` |
| 制動力推定 | Braking Force Estimation | `braking_force_estimation` |
| 加速度計 | Accelerometer | `accelerometer` |
| 勾配グラフ | Gradient Graph | `gradient_graph` |
| 制限グラフ | Speed Limit Graph | `speed_limit_graph` |
| 区間 | Section | `section` |

#### Physical Quantities / 物理量

| Japanese (日本語) | English | Type Name |
|------------------|---------|-----------|
| 距離 | Distance | `Distance` |
| 速度 | Velocity | `Velocity` |
| 加速度 | Acceleration | `Acceleration` |
| 時刻 | Time | `Time` |
| 時間 | Duration | `Duration` |
| メートル (m) | Meter | `Meter` |
| センチメートル (cm) | Centimeter | `Centimeter` |
| メートル毎秒 (mps) | Meters per Second | `MeterPerSecond` |
| キロメートル毎時 (kmph) | Kilometers per Hour | `KilometerPerHour` |
| メートル毎秒毎秒 (mps2) | Meters per Second Squared | `MeterPerSecondSquared` |

#### Common Variables / 共通変数

| Japanese (日本語) | English | Variable Name |
|------------------|---------|---------------|
| 現在位置 | Current Position | `current_position` |
| 現在速度 | Current Velocity | `current_velocity` |
| 目標位置 | Target Position | `target_position` |
| 目標速度 | Target Velocity | `target_velocity` |
| 制動距離 | Braking Distance | `braking_distance` |
| 減速度 | Deceleration | `deceleration` |
| 加速度 | Acceleration | `acceleration` |
| ノッチ | Notch | `notch` |
| ブレーキノッチ | Brake Notch | `brake_notch` |
| 力行ノッチ | Power Notch | `power_notch` |
| 勾配 | Gradient | `gradient` |
| 制限速度 | Speed Limit | `speed_limit` |
| 停止位置 | Stop Position | `stop_position` |
| 残距離 | Remaining Distance | `remaining_distance` |

#### Control & State / 制御・状態

| Japanese (日本語) | English | Variable Name |
|------------------|---------|---------------|
| 有効 | Active/Enabled | `is_active` / `enabled` |
| 無効 | Inactive/Disabled | `is_inactive` / `disabled` |
| 作動中 | Operating | `is_operating` |
| 待機中 | Standby | `is_standby` |
| 地上子 | Beacon | `beacon` |
| 出力 | Output | `output` |
| 指令 | Command | `command` |
| 応答 | Response | `response` |
| 遅延 | Delay | `delay` |
| 反応時間 | Reaction Time | `reaction_time` |

#### Railway-Specific Terms / 鉄道専門用語

| Japanese (日本語) | English | Notes |
|------------------|---------|-------|
| 保安装置 | Safety Device | Or "Protection System" |
| 車両長 | Vehicle Length | |
| 編成 | Train Formation | Or "Consist" |
| 空気ブレーキ | Air Brake | |
| 電気ブレーキ | Electric Brake | |
| 応荷重 | Load-responsive | |
| インチング | Inching | Keep as-is (technical term) |
| ORP | Over Run Protection | Keep acronym |

### Naming Examples / 命名例

#### Before (C++/Japanese)

```cpp
namespace 制動特性 {
    double 計算(double 現在速度, double 目標減速度) {
        // ...
    }
}

class 減速パターン {
    m 現在位置;
    mps 現在速度;
    mps2 期待減速度;
};
```

#### After (Rust/English)

```rust
mod brake_characteristics {
    pub fn calculate(current_velocity: f64, target_deceleration: f64) -> f64 {
        // ...
    }
}

struct DecelerationPattern {
    current_position: Meter,
    current_velocity: MeterPerSecond,
    expected_deceleration: MeterPerSecondSquared,
}
```

---

## Module Migration Mapping / モジュール移行マッピング

### Proposed Rust Project Structure / 提案されるRust構造

```
bve-autopilot-rs/
├── Cargo.toml                  # Project manifest
├── src/
│   ├── lib.rs                  # Library root
│   ├── ffi.rs                  # FFI exports for BVE plugin API
│   ├── types/
│   │   ├── mod.rs
│   │   ├── physical_quantity.rs    # 物理量
│   │   ├── control_command.rs      # 制御指令
│   │   └── beacon.rs               # 地上子
│   ├── state/
│   │   ├── mod.rs
│   │   ├── shared_state.rs         # 共通状態
│   │   ├── motion_state.rs         # 運動状態
│   │   ├── operating_state.rs      # 稼働状態
│   │   └── configuration.rs        # 環境設定
│   ├── tasc/
│   │   ├── mod.rs                  # TASC main
│   │   ├── deceleration_pattern.rs # 減速パターン
│   │   └── output_control.rs       # 出力制御
│   ├── ato/
│   │   ├── mod.rs                  # ATO main
│   │   ├── signal_compliance.rs    # 信号順守
│   │   ├── early_arrival_prevention.rs  # 早着防止
│   │   ├── orp.rs                  # ORP
│   │   └── sudden_motion_suppression.rs # 急動作抑制
│   ├── vehicle/
│   │   ├── mod.rs
│   │   ├── brake_characteristics.rs      # 制動特性
│   │   ├── power_characteristics.rs      # 力行特性
│   │   ├── braking_force_estimation.rs   # 制動力推定
│   │   └── accelerometer.rs              # 加速度計
│   ├── environment/
│   │   ├── mod.rs
│   │   ├── gradient_graph.rs       # 勾配グラフ
│   │   ├── speed_limit_graph.rs    # 制限グラフ
│   │   └── section.rs              # 区間
│   ├── control/
│   │   ├── mod.rs
│   │   ├── deceleration_target.rs  # 減速目標
│   │   ├── braking_command.rs      # 制動指令計算
│   │   └── output_control.rs       # 出力制御
│   └── output/
│       ├── mod.rs
│       ├── panel.rs                # パネル出力
│       └── sound.rs                # 音声出力
├── tests/
│   ├── tasc_test.rs                # TASCテスト
│   ├── gradient_graph_test.rs      # 勾配グラフテスト
│   ├── power_characteristics_test.rs   # 力行特性テスト
│   └── deceleration_target_test.rs     # 減速目標テスト
└── benches/                        # Performance benchmarks
    └── control_benchmarks.rs
```

---

## Type System Migration / 型システム移行

### Physical Quantities with uom Crate / uomクレートによる物理量

Instead of custom types, use the `uom` (units of measurement) crate for type safety:

```rust
use uom::si::f64::*;
use uom::si::length::{meter, centimeter};
use uom::si::velocity::{meter_per_second, kilometer_per_hour};
use uom::si::acceleration::meter_per_second_squared;

// Type aliases for convenience
pub type Distance = Length;
pub type Velocity = uom::si::f64::Velocity;
pub type Acceleration = uom::si::f64::Acceleration;

// Example usage
let distance = Length::new::<meter>(100.0);
let velocity = Velocity::new::<kilometer_per_hour>(80.0);
let deceleration = Acceleration::new::<meter_per_second_squared>(1.5);
```

### Control Command Types / 制御指令型

```rust
/// Brake notch command
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrakeNotch {
    Released,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    Emergency,
    Extended(u8), // 15-31 for extended brake
}

/// Power notch command
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerNotch {
    N,
    P1,
    P2,
    P3,
    P4,
    P5,
}

/// Complete control command
#[derive(Debug, Clone, Copy)]
pub struct ControlCommand {
    pub power: PowerNotch,
    pub brake: BrakeNotch,
    pub reverser: Reverser,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reverser {
    Forward,
    Neutral,
    Backward,
}
```

### State Types / 状態型

```rust
use uom::si::f64::*;

/// Train motion state
#[derive(Debug, Clone)]
pub struct MotionState {
    pub position: Length,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub time: Time,
}

/// Operating state for TASC/ATO
#[derive(Debug, Clone, Copy)]
pub struct OperatingState {
    pub tasc_active: bool,
    pub ato_active: bool,
    pub orp_active: bool,
}
```

---

## Migration Phases / 移行フェーズ

### Phase 1: Foundation / 基盤構築 (Weeks 1-2)

**Deliverables:**
- [ ] Create Rust project structure with Cargo
- [ ] Set up FFI layer for BVE plugin API
- [ ] Implement type system (physical quantities, control commands)
- [ ] Port utility modules (section, live)
- [ ] Set up CI/CD pipeline

**Files to create:**
- `Cargo.toml`
- `src/lib.rs`
- `src/ffi.rs`
- `src/types/physical_quantity.rs`
- `src/types/control_command.rs`

### Phase 2: State Management / 状態管理 (Weeks 3-4)

**Deliverables:**
- [ ] Port `共通状態` → `shared_state`
- [ ] Port `運動状態` → `motion_state`
- [ ] Port `稼働状態` → `operating_state`
- [ ] Port `環境設定` → `configuration`
- [ ] Implement configuration file parsing

**Files to port:**
- `共通状態.cpp/h` → `state/shared_state.rs`
- `運動状態.cpp/h` → `state/motion_state.rs`
- `稼働状態.h` → `state/operating_state.rs`
- `環境設定.cpp/h` → `state/configuration.rs`

### Phase 3: Vehicle & Environment / 車両・環境 (Weeks 5-6)

**Deliverables:**
- [ ] Port brake characteristics module
- [ ] Port power characteristics module
- [ ] Port braking force estimation
- [ ] Port accelerometer
- [ ] Port gradient graph
- [ ] Port speed limit graph
- [ ] Port section module

**Priority order:**
1. `制動特性` → `vehicle/brake_characteristics.rs`
2. `力行特性` → `vehicle/power_characteristics.rs`
3. `勾配グラフ` → `environment/gradient_graph.rs`
4. `制限グラフ` → `environment/speed_limit_graph.rs`

### Phase 4: Control Algorithms / 制御アルゴリズム (Weeks 7-9)

**Deliverables:**
- [ ] Port deceleration pattern calculation
- [ ] Port deceleration target management
- [ ] Port braking command calculation
- [ ] Port output control
- [ ] Port sudden motion suppression

**Critical modules:**
- `減速パターン.cpp/h` → `control/deceleration_pattern.rs`
- `減速目標.cpp/h` → `control/deceleration_target.rs`
- `制動指令計算.h` → `control/braking_command.rs`
- `出力制御.cpp/h` → `control/output_control.rs`

### Phase 5: TASC Implementation / TASC実装 (Weeks 10-11)

**Deliverables:**
- [ ] Port TASC main module
- [ ] Integrate deceleration pattern with TASC
- [ ] Implement inching functionality
- [ ] Port TASC tests

**Files to port:**
- `tasc.cpp/h` → `tasc/mod.rs`
- `tascテスト.cpp` → `tests/tasc_test.rs`

### Phase 6: ATO Implementation / ATO実装 (Weeks 12-14)

**Deliverables:**
- [ ] Port ATO main module
- [ ] Port signal compliance
- [ ] Port early arrival prevention
- [ ] Port ORP module
- [ ] Integrate all ATO components

**Files to port:**
- `ato.cpp/h` → `ato/mod.rs`
- `信号順守.cpp/h` → `ato/signal_compliance.rs`
- `早着防止.cpp/h` → `ato/early_arrival_prevention.rs`
- `orp.cpp/h` → `ato/orp.rs`

### Phase 7: Integration & Output / 統合・出力 (Weeks 15-16)

**Deliverables:**
- [ ] Port panel output
- [ ] Port sound output
- [ ] Port main plugin orchestration
- [ ] Complete FFI layer
- [ ] Integration testing

**Files to port:**
- `Main.cpp/h` → `lib.rs` (main logic)
- `パネル出力.cpp/h` → `output/panel.rs`
- `音声出力.h` → `output/sound.rs`

### Phase 8: Testing & Optimization / テスト・最適化 (Weeks 17-18)

**Deliverables:**
- [ ] Port all remaining tests
- [ ] Achieve feature parity with C++ version
- [ ] Performance benchmarking
- [ ] Memory profiling
- [ ] Bug fixes and optimization

### Phase 9: Documentation & Release / ドキュメント・リリース (Week 19-20)

**Deliverables:**
- [ ] Complete API documentation with `cargo doc`
- [ ] Update README for Rust version
- [ ] Create migration guide
- [ ] Release v2.0.0-beta

---

## Build System / ビルドシステム

### Cargo.toml Configuration

```toml
[package]
name = "bve-autopilot"
version = "2.0.0"
edition = "2021"
authors = ["Watanabe, Yuki"]
license = "LGPL-2.1"
description = "TASC/ATO plugin for BVE trainsim"

[lib]
name = "bve_autopilot"
crate-type = ["cdylib"]  # For Windows DLL

[dependencies]
uom = "0.36"  # Units of measurement
serde = { version = "1.0", features = ["derive"] }
serde_ini = "0.2"  # For .ini file parsing
windows = { version = "0.52", features = ["Win32_Foundation"] }

[dev-dependencies]
approx = "0.5"  # For floating-point comparisons in tests
criterion = "0.5"  # For benchmarking

[[bench]]
name = "control_benchmarks"
harness = false

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

### Build Commands

```bash
# Build debug version
cargo build

# Build release version (optimized)
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench

# Generate documentation
cargo doc --open

# Check without building
cargo check
```

---

## FFI Strategy / FFI戦略

### Windows DLL Export / Windows DLLエクスポート

```rust
use std::os::raw::{c_int, c_float};

// BVE Plugin API structures
#[repr(C)]
pub struct AtsVehicleSpec {
    pub brake_notches: c_int,
    pub power_notches: c_int,
    pub ats_notch: c_int,
    pub b67_notch: c_int,
    pub cars: c_int,
}

#[repr(C)]
pub struct AtsVehicleState {
    pub location: c_float,
    pub speed: c_float,
    pub time: c_int,
    pub bc_pressure: c_float,
    pub mr_pressure: c_float,
    pub er_pressure: c_float,
    pub bp_pressure: c_float,
    pub sap_pressure: c_float,
    pub current: c_float,
}

#[repr(C)]
pub struct AtsHandles {
    pub brake: c_int,
    pub power: c_int,
    pub reverser: c_int,
    pub constant_speed: c_int,
}

// Plugin API exports
#[no_mangle]
pub extern "stdcall" fn Load() {
    // Initialize plugin
}

#[no_mangle]
pub extern "stdcall" fn Dispose() {
    // Cleanup
}

#[no_mangle]
pub extern "stdcall" fn GetPluginVersion() -> c_int {
    0x00020000  // Version 2.0.0
}

#[no_mangle]
pub extern "stdcall" fn SetVehicleSpec(spec: AtsVehicleSpec) {
    // Set vehicle specifications
}

#[no_mangle]
pub extern "stdcall" fn Initialize(brake: c_int) {
    // Initialize train state
}

#[no_mangle]
pub extern "stdcall" fn Elapse(
    state: AtsVehicleState,
    panel: *mut c_int,
    sound: *mut c_int,
) -> AtsHandles {
    // Main update loop
    AtsHandles {
        brake: 0,
        power: 0,
        reverser: 0,
        constant_speed: 0,
    }
}

#[no_mangle]
pub extern "stdcall" fn SetPower(notch: c_int) {
    // Handle power notch change
}

#[no_mangle]
pub extern "stdcall" fn SetBrake(notch: c_int) {
    // Handle brake notch change
}

#[no_mangle]
pub extern "stdcall" fn SetReverser(pos: c_int) {
    // Handle reverser change
}

#[no_mangle]
pub extern "stdcall" fn KeyDown(key: c_int) {
    // Handle key press
}

#[no_mangle]
pub extern "stdcall" fn KeyUp(key: c_int) {
    // Handle key release
}

#[no_mangle]
pub extern "stdcall" fn HornBlow(horn_type: c_int) {
    // Handle horn
}

#[no_mangle]
pub extern "stdcall" fn DoorOpen() {
    // Handle door open
}

#[no_mangle]
pub extern "stdcall" fn DoorClose() {
    // Handle door close
}

#[no_mangle]
pub extern "stdcall" fn SetSignal(signal: c_int) {
    // Handle signal change
}

#[no_mangle]
pub extern "stdcall" fn SetBeaconData(beacon: AtsBeaconData) {
    // Handle beacon data
}
```

---

## Testing Strategy / テスト戦略

### Unit Tests / ユニットテスト

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_deceleration_pattern() {
        let pattern = DecelerationPattern::new(
            Length::new::<meter>(0.0),
            Velocity::new::<kilometer_per_hour>(80.0),
            Acceleration::new::<meter_per_second_squared>(1.5),
        );

        let expected_velocity = pattern.expected_velocity_at(
            Length::new::<meter>(100.0)
        );

        assert_relative_eq!(
            expected_velocity.get::<kilometer_per_hour>(),
            70.5,
            epsilon = 0.1
        );
    }
}
```

### Integration Tests / 統合テスト

```rust
// tests/tasc_test.rs
use bve_autopilot::tasc::Tasc;
use bve_autopilot::types::*;

#[test]
fn test_tasc_stopping_accuracy() {
    let mut tasc = Tasc::new(/* ... */);

    // Simulate train approaching station
    // Verify stopping accuracy within ±10cm
}
```

### Benchmarks / ベンチマーク

```rust
// benches/control_benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn deceleration_pattern_benchmark(c: &mut Criterion) {
    c.bench_function("deceleration pattern", |b| {
        b.iter(|| {
            // Benchmark critical path
        });
    });
}

criterion_group!(benches, deceleration_pattern_benchmark);
criterion_main!(benches);
```

---

## Timeline & Milestones / スケジュール

| Phase | Duration | Completion Date | Key Deliverables |
|-------|----------|-----------------|------------------|
| 1. Foundation | 2 weeks | Week 2 | Project setup, FFI, types |
| 2. State Management | 2 weeks | Week 4 | State modules |
| 3. Vehicle & Environment | 2 weeks | Week 6 | Characteristics, graphs |
| 4. Control Algorithms | 3 weeks | Week 9 | Core algorithms |
| 5. TASC | 2 weeks | Week 11 | TASC implementation |
| 6. ATO | 3 weeks | Week 14 | ATO implementation |
| 7. Integration | 2 weeks | Week 16 | Output, main logic |
| 8. Testing | 2 weeks | Week 18 | Tests, optimization |
| 9. Documentation | 2 weeks | Week 20 | Docs, release |

**Total Estimated Time:** 20 weeks (~5 months)

---

## Risk Mitigation / リスク軽減

### Potential Risks / 潜在的リスク

1. **Performance Regression / パフォーマンス低下**
   - Mitigation: Continuous benchmarking against C++ version
   - Target: Match or exceed C++ performance

2. **FFI Overhead / FFIオーバーヘッド**
   - Mitigation: Minimize boundary crossings, batch operations
   - Profiling at each phase

3. **Breaking Changes / 破壊的変更**
   - Mitigation: Maintain compatibility with existing `.ini` files
   - Versioned configuration format

4. **Translation Errors / 翻訳エラー**
   - Mitigation: Parallel testing with C++ version
   - Code review with domain experts

5. **Team Capacity / チーム容量**
   - Mitigation: Phased approach allows for flexibility
   - Can pause/resume at phase boundaries

---

## Success Criteria / 成功基準

### Must Have / 必須条件

- [ ] Feature parity with C++ version
- [ ] All tests passing
- [ ] Performance within 5% of C++ version
- [ ] No memory leaks or undefined behavior
- [ ] Compatible with existing BVE routes and trains

### Should Have / 推奨条件

- [ ] Improved test coverage (>80%)
- [ ] Complete API documentation
- [ ] Performance improvements in critical paths
- [ ] Cross-platform build capability

### Nice to Have / あれば良い条件

- [ ] Performance improvements (>10% faster)
- [ ] Additional features (e.g., telemetry, diagnostics)
- [ ] GUI configuration tool
- [ ] Multi-language support in UI

---

## Next Steps / 次のステップ

1. **Review this plan** with stakeholders / このプランをステークホルダーとレビュー
2. **Set up development environment** / 開発環境のセットアップ
   - Install Rust toolchain / Rustツールチェーンのインストール
   - Configure IDE (VS Code with rust-analyzer) / IDE設定
3. **Create initial Cargo project** / 初期Cargoプロジェクトの作成
4. **Begin Phase 1: Foundation** / フェーズ1開始

---

## References / 参考資料

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [uom - Units of Measurement](https://docs.rs/uom/)
- [Rust FFI Omnibus](http://jakegoulding.com/rust-ffi-omnibus/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- Current codebase: `CURRENT_STRUCTURE.md`
- Algorithm documentation: `algorithm.md`

---

**Document Version:** 1.0
**Last Updated:** 2025-11-07
**Author:** Claude (with Watanabe, Yuki)
**Status:** Draft for Review
