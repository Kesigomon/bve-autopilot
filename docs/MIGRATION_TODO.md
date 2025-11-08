# Rust Migration Todo List / Rust移行Todoリスト

This document provides a detailed task breakdown for each phase of the Rust migration. Use this as a checklist to track progress through the migration process.

このドキュメントは、Rust移行の各フェーズの詳細なタスク分解を提供します。移行プロセスの進捗を追跡するためのチェックリストとして使用してください。

**Related Documents / 関連ドキュメント:**
- [RUST_MIGRATION_PLAN.md](RUST_MIGRATION_PLAN.md) - Overall migration strategy
- [CURRENT_STRUCTURE.md](CURRENT_STRUCTURE.md) - Current codebase documentation

---

## Phase 1: Foundation / 基盤構築 (Weeks 1-2)

**Objective:** Set up Rust project infrastructure and core type system

### 1.1 Project Setup / プロジェクトセットアップ

- [ ] Install Rust toolchain (rustup)
  - [ ] Install stable Rust
  - [ ] Install Windows target: `x86_64-pc-windows-msvc`
  - [ ] Install i686 target: `i686-pc-windows-msvc`
  - [ ] Verify installation: `rustc --version` and `cargo --version`

- [ ] Create Cargo workspace
  - [ ] Run `cargo new --lib bve-autopilot-rs`
  - [ ] Configure `Cargo.toml` with project metadata
  - [ ] Set crate-type to `["cdylib"]` for DLL output
  - [ ] Add license and author information

- [ ] Set up development environment
  - [ ] Install Visual Studio Code / Visual Studio
  - [ ] Install rust-analyzer extension
  - [ ] Configure `.vscode/settings.json` for Rust
  - [ ] Set up `.gitignore` for Rust (target/, Cargo.lock for libraries)

- [ ] Configure build targets
  - [ ] Add `[profile.dev]` configuration
  - [ ] Add `[profile.release]` with optimizations (opt-level = 3, lto = true)
  - [ ] Test debug build: `cargo build`
  - [ ] Test release build: `cargo build --release`

### 1.2 Dependencies / 依存関係

- [ ] Add core dependencies to `Cargo.toml`
  - [ ] Add `uom = "0.36"` for units of measurement
  - [ ] Add `serde = { version = "1.0", features = ["derive"] }`
  - [ ] Add `serde_ini = "0.2"` for configuration parsing
  - [ ] Add `windows = { version = "0.52", features = ["Win32_Foundation"] }`

- [ ] Add development dependencies
  - [ ] Add `approx = "0.5"` for float comparisons
  - [ ] Add `criterion = "0.5"` for benchmarking
  - [ ] Configure criterion benchmark harness

- [ ] Verify dependencies
  - [ ] Run `cargo build` to download and compile dependencies
  - [ ] Check for version conflicts
  - [ ] Review dependency licenses for LGPL compatibility

### 1.3 Type System / 型システム

- [ ] Create `src/types/` module structure
  - [ ] Create `src/types/mod.rs`
  - [ ] Create `src/types/physical_quantity.rs`
  - [ ] Create `src/types/control_command.rs`
  - [ ] Create `src/types/beacon.rs`

- [ ] Implement physical quantity types (in `physical_quantity.rs`)
  - [ ] Import uom types (Length, Velocity, Acceleration, Time)
  - [ ] Create type aliases: `Distance`, `Speed`, `Accel`
  - [ ] Define unit constants (meter, kilometer, second, etc.)
  - [ ] Add conversion helper functions
  - [ ] Write unit tests for conversions

- [ ] Implement control command types (in `control_command.rs`)
  - [ ] Define `BrakeNotch` enum (Released, B1-B8, Emergency, Extended)
  - [ ] Define `PowerNotch` enum (N, P1-P5)
  - [ ] Define `Reverser` enum (Forward, Neutral, Backward)
  - [ ] Define `ControlCommand` struct
  - [ ] Implement Display trait for debugging
  - [ ] Write unit tests

- [ ] Implement beacon types (in `beacon.rs`)
  - [ ] Define `BeaconType` enum
  - [ ] Define `BeaconData` struct
  - [ ] Add parsing functions for beacon signals
  - [ ] Write unit tests

### 1.4 FFI Layer / FFI層

- [ ] Create `src/ffi.rs` module
  - [ ] Define FFI-safe types with `#[repr(C)]`
  - [ ] Define `AtsVehicleSpec` struct
  - [ ] Define `AtsVehicleState` struct
  - [ ] Define `AtsHandles` struct
  - [ ] Define `AtsBeaconData` struct

- [ ] Implement plugin API exports
  - [ ] Implement `Load()` function
  - [ ] Implement `Dispose()` function
  - [ ] Implement `GetPluginVersion()` function
  - [ ] Implement `SetVehicleSpec()` function
  - [ ] Implement `Initialize()` function
  - [ ] Implement `Elapse()` function (stub)
  - [ ] Implement `SetPower()` function
  - [ ] Implement `SetBrake()` function
  - [ ] Implement `SetReverser()` function
  - [ ] Implement `KeyDown()` function
  - [ ] Implement `KeyUp()` function
  - [ ] Implement `HornBlow()` function
  - [ ] Implement `DoorOpen()` function
  - [ ] Implement `DoorClose()` function
  - [ ] Implement `SetSignal()` function
  - [ ] Implement `SetBeaconData()` function

- [ ] Add FFI utilities
  - [ ] Create conversion functions: C types ↔ Rust types
  - [ ] Add safety wrappers for raw pointer handling
  - [ ] Implement panic catching for FFI boundaries

### 1.5 Utility Modules / ユーティリティモジュール

- [ ] Port `区間.cpp/h` → `src/utils/section.rs`
  - [ ] Define `Section` struct with start/end positions
  - [ ] Implement `contains()` method
  - [ ] Implement `intersects()` method
  - [ ] Implement `length()` method
  - [ ] Write unit tests

- [ ] Port `live.h` → `src/utils/observable.rs`
  - [ ] Define `Observable<T>` type
  - [ ] Implement value tracking with change detection
  - [ ] Add getter/setter methods
  - [ ] Write unit tests

### 1.6 Project Infrastructure / プロジェクトインフラ

- [ ] Create documentation structure
  - [ ] Add module-level doc comments to `lib.rs`
  - [ ] Configure `cargo doc` settings
  - [ ] Test documentation generation: `cargo doc --open`

- [ ] Set up testing infrastructure
  - [ ] Create `tests/` directory for integration tests
  - [ ] Create `benches/` directory for benchmarks
  - [ ] Add `#[cfg(test)]` module template
  - [ ] Test that `cargo test` runs successfully

- [ ] Set up CI/CD (optional but recommended)
  - [ ] Create `.github/workflows/rust.yml`
  - [ ] Configure build matrix (stable, beta, nightly)
  - [ ] Add test, clippy, and fmt checks
  - [ ] Configure Windows build targets

### 1.7 Phase 1 Validation / フェーズ1検証

- [ ] Build verification
  - [ ] Debug build succeeds: `cargo build`
  - [ ] Release build succeeds: `cargo build --release`
  - [ ] All tests pass: `cargo test`
  - [ ] No clippy warnings: `cargo clippy`
  - [ ] Code is formatted: `cargo fmt --check`

- [ ] DLL verification
  - [ ] Locate output DLL in `target/release/`
  - [ ] Verify DLL exports with `dumpbin` or similar tool
  - [ ] Test DLL loads in BVE (basic smoke test)

- [ ] Documentation
  - [ ] Document Phase 1 completion in migration log
  - [ ] Update this checklist with actual issues encountered
  - [ ] Commit all Phase 1 code

---

## Phase 2: State Management / 状態管理 (Weeks 3-4)

**Objective:** Port all state management modules with configuration support

### 2.1 Module Structure / モジュール構造

- [ ] Create `src/state/` module
  - [ ] Create `src/state/mod.rs`
  - [ ] Create `src/state/shared_state.rs`
  - [ ] Create `src/state/motion_state.rs`
  - [ ] Create `src/state/operating_state.rs`
  - [ ] Create `src/state/configuration.rs`

### 2.2 Shared State / 共通状態

- [ ] Port `共通状態.h` → `state/shared_state.rs`
  - [ ] Define `SharedState` struct
  - [ ] Add vehicle specification fields
  - [ ] Add environment configuration fields
  - [ ] Add motion state reference
  - [ ] Add operating state reference

- [ ] Implement state initialization
  - [ ] Create `new()` constructor
  - [ ] Create `from_vehicle_spec()` method
  - [ ] Implement default values

- [ ] Port `共通状態.cpp` logic
  - [ ] Port state update methods
  - [ ] Port state query methods
  - [ ] Add state validation

- [ ] Write tests
  - [ ] Test initialization
  - [ ] Test state updates
  - [ ] Test state queries

### 2.3 Motion State / 運動状態

- [ ] Port `運動状態.h` → `state/motion_state.rs`
  - [ ] Define `MotionState` struct with physical quantities
  - [ ] Add position field (Distance)
  - [ ] Add velocity field (Velocity)
  - [ ] Add acceleration field (Acceleration)
  - [ ] Add time field (Time)

- [ ] Port `運動状態.cpp` logic
  - [ ] Implement state update from BVE data
  - [ ] Implement velocity calculation
  - [ ] Implement acceleration estimation
  - [ ] Add state history tracking (if needed)

- [ ] Implement motion calculations
  - [ ] Add kinematic equations
  - [ ] Add prediction methods
  - [ ] Add interpolation methods

- [ ] Write tests
  - [ ] Test state updates
  - [ ] Test kinematic calculations
  - [ ] Test edge cases (zero velocity, negative values)

### 2.4 Operating State / 稼働状態

- [ ] Port `稼働状態.h` → `state/operating_state.rs`
  - [ ] Define `OperatingState` struct
  - [ ] Add TASC active flag
  - [ ] Add ATO active flag
  - [ ] Add ORP active flag
  - [ ] Add compatibility mode enum

- [ ] Implement state transitions
  - [ ] Define `OperatingMode` enum
  - [ ] Implement mode switching logic
  - [ ] Add validation for mode transitions
  - [ ] Implement mode persistence

- [ ] Add compatibility mode support
  - [ ] Define `CompatibilityMode` enum (Generic ATS, Metro, etc.)
  - [ ] Implement mode detection
  - [ ] Add mode-specific behavior flags

- [ ] Write tests
  - [ ] Test mode transitions
  - [ ] Test flag combinations
  - [ ] Test invalid state handling

### 2.5 Configuration / 環境設定

- [ ] Port `環境設定.h` → `state/configuration.rs`
  - [ ] Define `Configuration` struct
  - [ ] Add vehicle length setting
  - [ ] Add max deceleration setting
  - [ ] Add brake reaction time setting
  - [ ] Add extended brake notch settings
  - [ ] Add key binding settings
  - [ ] Add panel output settings

- [ ] Implement INI file parsing (port `環境設定.cpp`)
  - [ ] Use `serde_ini` crate
  - [ ] Define `#[derive(Deserialize)]` for config struct
  - [ ] Implement `load_from_file()` method
  - [ ] Add default configuration
  - [ ] Add configuration validation

- [ ] Add configuration utilities
  - [ ] Implement `get_config_path()` to find autopilot.ini
  - [ ] Add error handling for missing/invalid config
  - [ ] Add config migration for version updates
  - [ ] Implement `save_to_file()` for future use

- [ ] Write tests
  - [ ] Test parsing valid INI file
  - [ ] Test parsing with missing fields (use defaults)
  - [ ] Test parsing with invalid values
  - [ ] Test configuration validation

### 2.6 State Integration / 状態統合

- [ ] Create state manager
  - [ ] Define `StateManager` struct holding all state
  - [ ] Implement initialization from FFI data
  - [ ] Implement update from FFI data
  - [ ] Add state query methods

- [ ] Integrate with FFI layer
  - [ ] Update `ffi.rs` to use state modules
  - [ ] Implement state conversion in `SetVehicleSpec()`
  - [ ] Implement state update in `Elapse()`
  - [ ] Add state persistence between calls

- [ ] Add state serialization (optional)
  - [ ] Implement `serde::Serialize` for state types
  - [ ] Add state snapshot functionality
  - [ ] Add state debugging output

### 2.7 Phase 2 Validation / フェーズ2検証

- [ ] Unit test verification
  - [ ] All state module tests pass
  - [ ] Configuration parsing tests pass
  - [ ] State transition tests pass

- [ ] Integration test
  - [ ] Create integration test loading real autopilot.ini
  - [ ] Test state initialization from FFI
  - [ ] Test state updates through multiple frames

- [ ] Documentation
  - [ ] Add doc comments to all public APIs
  - [ ] Generate and review documentation: `cargo doc`
  - [ ] Document Phase 2 completion

---

## Phase 3: Vehicle & Environment / 車両・環境 (Weeks 5-6)

**Objective:** Port vehicle characteristics and environment graph modules

### 3.1 Module Structure / モジュール構造

- [ ] Create `src/vehicle/` module
  - [ ] Create `src/vehicle/mod.rs`
  - [ ] Create `src/vehicle/brake_characteristics.rs`
  - [ ] Create `src/vehicle/power_characteristics.rs`
  - [ ] Create `src/vehicle/braking_force_estimation.rs`
  - [ ] Create `src/vehicle/accelerometer.rs`

- [ ] Create `src/environment/` module
  - [ ] Create `src/environment/mod.rs`
  - [ ] Create `src/environment/gradient_graph.rs`
  - [ ] Create `src/environment/speed_limit_graph.rs`
  - [ ] Create `src/environment/section.rs`

### 3.2 Brake Characteristics / 制動特性

- [ ] Port `制動特性.h` → `vehicle/brake_characteristics.rs`
  - [ ] Define `BrakeCharacteristics` struct
  - [ ] Add air brake parameters
  - [ ] Add electric brake parameters
  - [ ] Add load-responsive parameters

- [ ] Port `制動特性.cpp` logic
  - [ ] Implement brake force calculation
  - [ ] Implement air brake pressure dynamics
  - [ ] Implement electric brake characteristics
  - [ ] Add gradient compensation

- [ ] Implement brake models
  - [ ] Add linear brake model
  - [ ] Add exponential brake model
  - [ ] Add combined brake model (air + electric)
  - [ ] Add brake delay modeling

- [ ] Write tests (refer to C++ test suite)
  - [ ] Test brake force at various notches
  - [ ] Test pressure dynamics
  - [ ] Test gradient compensation
  - [ ] Validate against expected values

### 3.3 Power Characteristics / 力行特性

- [ ] Port `力行特性.h` → `vehicle/power_characteristics.rs`
  - [ ] Define `PowerCharacteristics` struct
  - [ ] Add acceleration parameters
  - [ ] Add speed-dependent characteristics
  - [ ] Add notch-dependent characteristics

- [ ] Port `力行特性.cpp` logic
  - [ ] Implement acceleration calculation
  - [ ] Implement speed-force curve
  - [ ] Implement notch interpolation
  - [ ] Add gradient effects

- [ ] Implement power models
  - [ ] Add constant acceleration model
  - [ ] Add speed-dependent model
  - [ ] Add combined model
  - [ ] Add motor characteristics

- [ ] Port tests from `力行特性テスト.cpp`
  - [ ] Test acceleration at various speeds
  - [ ] Test notch transitions
  - [ ] Test gradient effects
  - [ ] Validate against C++ test data

### 3.4 Braking Force Estimation / 制動力推定

- [ ] Port `制動力推定.h` → `vehicle/braking_force_estimation.rs`
  - [ ] Define `BrakingForceEstimator` struct
  - [ ] Add measurement history
  - [ ] Add estimation parameters

- [ ] Port `制動力推定.cpp` logic
  - [ ] Implement real-time force estimation
  - [ ] Implement Kalman filter or similar
  - [ ] Add measurement noise handling
  - [ ] Implement adaptive estimation

- [ ] Add estimation utilities
  - [ ] Add moving average filter
  - [ ] Add outlier rejection
  - [ ] Add confidence estimation

- [ ] Write tests
  - [ ] Test estimation accuracy
  - [ ] Test noise handling
  - [ ] Test convergence time

### 3.5 Accelerometer / 加速度計

- [ ] Port `加速度計.h` → `vehicle/accelerometer.rs`
  - [ ] Define `Accelerometer` struct
  - [ ] Add velocity history
  - [ ] Add acceleration smoothing

- [ ] Port `加速度計.cpp` logic
  - [ ] Implement acceleration measurement from velocity
  - [ ] Implement smoothing filter
  - [ ] Add derivative calculation
  - [ ] Handle numerical stability

- [ ] Write tests
  - [ ] Test acceleration calculation
  - [ ] Test filtering
  - [ ] Test edge cases

### 3.6 Gradient Graph / 勾配グラフ

- [ ] Port `勾配グラフ.h` → `environment/gradient_graph.rs`
  - [ ] Define `GradientGraph` struct
  - [ ] Define `GradientPoint` struct
  - [ ] Add gradient storage (position → gradient map)

- [ ] Port `勾配グラフ.cpp` logic
  - [ ] Implement gradient lookup by position
  - [ ] Implement gradient interpolation
  - [ ] Add gradient update from beacons
  - [ ] Implement range queries

- [ ] Optimize data structure
  - [ ] Use `BTreeMap` for sorted storage
  - [ ] Implement efficient range lookups
  - [ ] Add caching for frequent queries

- [ ] Port tests from `勾配グラフテスト.cpp`
  - [ ] Test gradient insertion
  - [ ] Test gradient lookup
  - [ ] Test interpolation
  - [ ] Test edge cases (empty, single point)
  - [ ] **Note:** This is the largest test file (35,778 lines)

### 3.7 Speed Limit Graph / 制限グラフ

- [ ] Port `制限グラフ.h` → `environment/speed_limit_graph.rs`
  - [ ] Define `SpeedLimitGraph` struct
  - [ ] Define `SpeedLimit` struct
  - [ ] Add limit storage structure

- [ ] Port `制限グラフ.cpp` logic
  - [ ] Implement limit lookup by position
  - [ ] Implement limit range queries
  - [ ] Add limit update from signals/beacons
  - [ ] Implement limit merging logic

- [ ] Add query methods
  - [ ] Get current limit at position
  - [ ] Get next limit change
  - [ ] Get strictest limit in range
  - [ ] Get limit lookahead

- [ ] Write tests
  - [ ] Test limit insertion and update
  - [ ] Test limit queries
  - [ ] Test overlapping limits
  - [ ] Test limit merging

### 3.8 Section Utilities / 区間ユーティリティ

- [ ] Enhance `environment/section.rs`
  - [ ] Add distance-based section operations
  - [ ] Add section intersection logic
  - [ ] Add section merging
  - [ ] Add section validation

- [ ] Write tests
  - [ ] Test section operations
  - [ ] Test boundary conditions

### 3.9 Phase 3 Validation / フェーズ3検証

- [ ] Test compilation
  - [ ] All modules compile without warnings
  - [ ] Run `cargo clippy` and fix issues

- [ ] Test suite
  - [ ] All vehicle tests pass
  - [ ] All environment tests pass
  - [ ] Gradient graph tests match C++ results
  - [ ] Power characteristics tests match C++ results

- [ ] Integration test
  - [ ] Test vehicle characteristics with real parameters
  - [ ] Test gradient graph with real track data
  - [ ] Validate calculations against C++ version

- [ ] Documentation
  - [ ] Document all public APIs
  - [ ] Add usage examples
  - [ ] Document Phase 3 completion

---

## Phase 4: Control Algorithms / 制御アルゴリズム (Weeks 7-9)

**Objective:** Port core control algorithm modules

### 4.1 Module Structure / モジュール構造

- [ ] Create `src/control/` module
  - [ ] Create `src/control/mod.rs`
  - [ ] Create `src/control/deceleration_pattern.rs`
  - [ ] Create `src/control/deceleration_target.rs`
  - [ ] Create `src/control/braking_command.rs`
  - [ ] Create `src/control/output_control.rs`
  - [ ] Create `src/control/sudden_motion_suppression.rs`

### 4.2 Deceleration Pattern / 減速パターン

- [ ] Port `減速パターン.h` → `control/deceleration_pattern.rs`
  - [ ] Define `DecelerationPattern` struct
  - [ ] Add target position
  - [ ] Add target velocity
  - [ ] Add deceleration rate

- [ ] Port `減速パターン.cpp` core algorithm
  - [ ] Implement pattern calculation
  - [ ] Implement expected velocity at position
  - [ ] Implement expected deceleration calculation
  - [ ] **Port core formula:** `出力減速度 = 期待減速度 * (現在速度 / 期待速度) - (期待速度 - 現在速度) / 2秒`

- [ ] Add pattern utilities
  - [ ] Implement pattern validation
  - [ ] Add margin calculations
  - [ ] Implement pattern adjustment
  - [ ] Add safety checks

- [ ] Write tests
  - [ ] Test pattern generation
  - [ ] Test velocity calculation at various positions
  - [ ] Test deceleration output calculation
  - [ ] Validate formula implementation
  - [ ] Test boundary conditions

### 4.3 Deceleration Target / 減速目標

- [ ] Port `減速目標.h` → `control/deceleration_target.rs`
  - [ ] Define `DecelerationTarget` struct
  - [ ] Define `TargetType` enum
  - [ ] Add target priority system

- [ ] Port `減速目標.cpp` logic
  - [ ] Implement target management
  - [ ] Implement target selection (by priority)
  - [ ] Add target update logic
  - [ ] Implement target expiration

- [ ] Add target sources
  - [ ] Station stop targets
  - [ ] Speed limit targets
  - [ ] Signal targets
  - [ ] ORP targets

- [ ] Port tests from `減速目標テスト.cpp`
  - [ ] Test target addition
  - [ ] Test target selection
  - [ ] Test priority handling
  - [ ] Test target updates

### 4.4 Braking Command / 制動指令計算

- [ ] Port `制動指令計算.h` → `control/braking_command.rs`
  - [ ] Define `BrakingCommandCalculator` struct
  - [ ] Add brake characteristics reference
  - [ ] Add configuration parameters

- [ ] Implement braking command calculation
  - [ ] Implement deceleration to notch conversion
  - [ ] Add extended brake support (15-31 levels)
  - [ ] Implement brake interpolation
  - [ ] Add emergency brake logic

- [ ] Add command utilities
  - [ ] Implement notch rounding
  - [ ] Add hysteresis for stability
  - [ ] Implement minimum command filtering

- [ ] Write tests
  - [ ] Test deceleration to notch conversion
  - [ ] Test extended brake mapping
  - [ ] Test edge cases (zero, emergency)

### 4.5 Output Control / 出力制御

- [ ] Port `出力制御.h` → `control/output_control.rs`
  - [ ] Define `OutputController` struct
  - [ ] Add current command state
  - [ ] Add command history

- [ ] Port `出力制御.cpp` logic
  - [ ] Implement notch output logic
  - [ ] Implement power/brake coordination
  - [ ] Add reverser control
  - [ ] Implement command smoothing

- [ ] Add control modes
  - [ ] Manual mode passthrough
  - [ ] TASC mode control
  - [ ] ATO mode control
  - [ ] Emergency override

- [ ] Write tests
  - [ ] Test command output
  - [ ] Test mode switching
  - [ ] Test smoothing behavior

### 4.6 Sudden Motion Suppression / 急動作抑制

- [ ] Port `急動作抑制.h` → `control/sudden_motion_suppression.rs`
  - [ ] Define `SuddenMotionSuppressor` struct
  - [ ] Add rate limiters
  - [ ] Add acceleration limits

- [ ] Port `急動作抑制.cpp` logic
  - [ ] Implement notch rate limiting
  - [ ] Implement jerk limiting
  - [ ] Add smooth transitions
  - [ ] Implement acceleration limiting

- [ ] Add suppression methods
  - [ ] Power notch rate limiter
  - [ ] Brake notch rate limiter
  - [ ] Combined limiter
  - [ ] Adaptive limiting

- [ ] Write tests
  - [ ] Test rate limiting
  - [ ] Test smooth transitions
  - [ ] Test acceleration bounds

### 4.7 Control Integration / 制御統合

- [ ] Create control coordinator
  - [ ] Define `ControlCoordinator` struct
  - [ ] Integrate all control modules
  - [ ] Implement control pipeline

- [ ] Implement control flow
  - [ ] Input: current state
  - [ ] Process: pattern → target → command → output
  - [ ] Output: control handles
  - [ ] Add feedback loop

- [ ] Add diagnostics
  - [ ] Log control decisions
  - [ ] Add performance metrics
  - [ ] Implement debug output

### 4.8 Phase 4 Validation / フェーズ4検証

- [ ] Algorithm validation
  - [ ] Verify deceleration formula matches algorithm.md
  - [ ] Compare outputs with C++ version
  - [ ] Test with real-world scenarios

- [ ] Test suite
  - [ ] All control tests pass
  - [ ] Deceleration target tests pass
  - [ ] Integration tests pass

- [ ] Performance check
  - [ ] Benchmark critical paths
  - [ ] Ensure real-time performance (< 16ms per frame)
  - [ ] Profile for optimization opportunities

- [ ] Documentation
  - [ ] Document control algorithms
  - [ ] Add diagrams for control flow
  - [ ] Document Phase 4 completion

---

## Phase 5: TASC Implementation / TASC実装 (Weeks 10-11)

**Objective:** Implement TASC functionality using control modules

### 5.1 TASC Module Structure / TASCモジュール構造

- [ ] Create `src/tasc/` module
  - [ ] Create `src/tasc/mod.rs`
  - [ ] Plan module organization

### 5.2 TASC Core / TASCコア

- [ ] Port `tasc.h` → `tasc/mod.rs`
  - [ ] Define `Tasc` struct
  - [ ] Add stop position target
  - [ ] Add operating state
  - [ ] Add configuration

- [ ] Port `tasc.cpp` main logic
  - [ ] Implement TASC initialization
  - [ ] Implement beacon processing
  - [ ] Implement stop position calculation
  - [ ] Implement state machine

- [ ] Implement TASC states
  - [ ] Define `TascState` enum (Standby, Armed, Active, Stopped)
  - [ ] Implement state transitions
  - [ ] Add state-specific behavior

### 5.3 Stop Position Control / 停止位置制御

- [ ] Implement precise stopping
  - [ ] Use deceleration pattern module
  - [ ] Calculate brake timing
  - [ ] Implement final approach control
  - [ ] Add position error correction

- [ ] Add stopping accuracy features
  - [ ] Implement adaptive braking
  - [ ] Add predictive control
  - [ ] Implement error compensation
  - [ ] Target: ±10cm accuracy

### 5.4 Beacon Protocol Support / 地上子プロトコル対応

- [ ] Implement generic TASC protocol
  - [ ] Parse TASC beacon data
  - [ ] Extract stop position
  - [ ] Extract speed limit
  - [ ] Validate beacon data

- [ ] Implement compatibility modes
  - [ ] Generic ATS mode
  - [ ] Metro plugin compatibility
  - [ ] SWP2 compatibility
  - [ ] Odakyu D-ATS-P compatibility
  - [ ] Odakyu CS-ATC compatibility
  - [ ] Metro TASC compatibility

- [ ] Add beacon validation
  - [ ] Check beacon consistency
  - [ ] Handle invalid beacons
  - [ ] Implement fallback behavior

### 5.5 Inching / インチング

- [ ] Implement inching mode
  - [ ] Detect overshoot
  - [ ] Enable forward creep
  - [ ] Implement position correction
  - [ ] Add safety limits

- [ ] Add inching control
  - [ ] Low-speed power control
  - [ ] Position monitoring
  - [ ] Automatic exit on target reached

- [ ] Write tests
  - [ ] Test inching activation
  - [ ] Test position correction
  - [ ] Test safety limits

### 5.6 TASC Output / TASC出力

- [ ] Implement control output
  - [ ] Generate brake commands
  - [ ] Handle manual override
  - [ ] Implement emergency brake
  - [ ] Add mode indicator

- [ ] Implement panel output
  - [ ] Output brake notch indicator
  - [ ] Output remaining distance
  - [ ] Output TASC state
  - [ ] Output target position

### 5.7 TASC Testing / TASCテスト

- [ ] Port tests from `tascテスト.cpp`
  - [ ] Test beacon processing
  - [ ] Test stop position calculation
  - [ ] Test deceleration control
  - [ ] Test stopping accuracy
  - [ ] **Note:** 13,932 lines of tests

- [ ] Add integration tests
  - [ ] Test full stop sequence
  - [ ] Test with various vehicle types
  - [ ] Test with various gradients
  - [ ] Test error cases

- [ ] Validation tests
  - [ ] Compare with C++ version
  - [ ] Test stopping accuracy < ±10cm
  - [ ] Test all compatibility modes

### 5.8 Phase 5 Validation / フェーズ5検証

- [ ] Functional verification
  - [ ] TASC activates correctly
  - [ ] Stops at target position
  - [ ] Inching works correctly
  - [ ] All beacons parsed correctly

- [ ] Accuracy verification
  - [ ] Stopping accuracy within ±10cm
  - [ ] Smooth deceleration
  - [ ] No emergency braking in normal operation

- [ ] Test suite
  - [ ] All TASC tests pass
  - [ ] Integration tests pass
  - [ ] Regression tests pass

- [ ] Documentation
  - [ ] Document TASC usage
  - [ ] Document beacon protocols
  - [ ] Document Phase 5 completion

---

## Phase 6: ATO Implementation / ATO実装 (Weeks 12-14)

**Objective:** Implement ATO functionality with signal compliance and speed control

### 6.1 ATO Module Structure / ATOモジュール構造

- [ ] Create `src/ato/` module
  - [ ] Create `src/ato/mod.rs`
  - [ ] Create `src/ato/signal_compliance.rs`
  - [ ] Create `src/ato/early_arrival_prevention.rs`
  - [ ] Create `src/ato/orp.rs`
  - [ ] Create `src/ato/sudden_motion_suppression.rs`

### 6.2 ATO Core / ATOコア

- [ ] Port `ato.h` → `ato/mod.rs`
  - [ ] Define `Ato` struct
  - [ ] Add operating mode
  - [ ] Add target speed
  - [ ] Add sub-module references

- [ ] Port `ato.cpp` main logic
  - [ ] Implement ATO initialization
  - [ ] Implement main update loop
  - [ ] Integrate sub-modules
  - [ ] Implement state management

- [ ] Implement ATO states
  - [ ] Define `AtoState` enum
  - [ ] Implement state transitions
  - [ ] Add activation/deactivation logic

### 6.3 Signal Compliance / 信号順守

- [ ] Port `信号順守.h` → `ato/signal_compliance.rs`
  - [ ] Define `SignalCompliance` struct
  - [ ] Add signal state tracking
  - [ ] Add speed limit management

- [ ] Port `信号順守.cpp` logic
  - [ ] Implement signal processing
  - [ ] Implement speed limit enforcement
  - [ ] Add advance signal handling
  - [ ] Implement deceleration for signals

- [ ] Add signal features
  - [ ] Parse signal aspects
  - [ ] Calculate required deceleration
  - [ ] Implement predictive braking
  - [ ] Add signal lookahead

- [ ] Integrate with speed limit graph
  - [ ] Update limits from signals
  - [ ] Query next speed change
  - [ ] Calculate brake points

- [ ] Write tests
  - [ ] Test signal parsing
  - [ ] Test speed enforcement
  - [ ] Test advance braking
  - [ ] Test signal transitions

### 6.4 ORP (Over Run Protection)

- [ ] Port `orp.h` → `ato/orp.rs`
  - [ ] Define `Orp` struct
  - [ ] Add protection state
  - [ ] Add brake intervention

- [ ] Port `orp.cpp` logic
  - [ ] Implement pattern照査 (pattern checking)
  - [ ] Implement speed monitoring
  - [ ] Add intervention logic
  - [ ] Implement penalty brake

- [ ] Add ORP features
  - [ ] Set protection patterns from beacons
  - [ ] Monitor speed vs. pattern
  - [ ] Trigger warning before intervention
  - [ ] Apply emergency brake on violation

- [ ] Write tests
  - [ ] Test pattern setting
  - [ ] Test speed monitoring
  - [ ] Test intervention timing
  - [ ] Test recovery

### 6.5 Early Arrival Prevention / 早着防止

- [ ] Port `早着防止.h` → `ato/early_arrival_prevention.rs`
  - [ ] Define `EarlyArrivalPrevention` struct
  - [ ] Add target time
  - [ ] Add speed adjustment

- [ ] Port `早着防止.cpp` logic
  - [ ] Calculate time to target
  - [ ] Calculate required speed
  - [ ] Implement speed reduction
  - [ ] Add margin management

- [ ] Add timing features
  - [ ] Parse timing beacons
  - [ ] Calculate optimal speed profile
  - [ ] Adjust for early/late arrival
  - [ ] Implement smooth transitions

- [ ] Write tests
  - [ ] Test time calculation
  - [ ] Test speed adjustment
  - [ ] Test edge cases

### 6.6 Sudden Motion Suppression (ATO) / 急動作抑制

- [ ] Port `急動作抑制.cpp` ATO-specific logic
  - [ ] Reuse control module
  - [ ] Add ATO-specific parameters
  - [ ] Implement smooth acceleration
  - [ ] Implement smooth braking

- [ ] Add comfort features
  - [ ] Limit jerk (acceleration change rate)
  - [ ] Smooth power application
  - [ ] Gradual brake release
  - [ ] Passenger comfort optimization

- [ ] Write tests
  - [ ] Test acceleration smoothing
  - [ ] Test jerk limits
  - [ ] Test comfort metrics

### 6.7 ATO Speed Control / ATO速度制御

- [ ] Implement automatic acceleration
  - [ ] Calculate power notch for target speed
  - [ ] Implement speed tracking
  - [ ] Add gradient compensation
  - [ ] Implement cruise control

- [ ] Implement automatic braking
  - [ ] Calculate brake notch for target speed
  - [ ] Implement smooth deceleration
  - [ ] Add speed overshoot prevention
  - [ ] Integrate with TASC for stations

- [ ] Add control modes
  - [ ] Acceleration mode
  - [ ] Cruising mode
  - [ ] Deceleration mode
  - [ ] Coast mode

### 6.8 ATO Integration / ATO統合

- [ ] Integrate all ATO modules
  - [ ] Connect signal compliance
  - [ ] Connect early arrival prevention
  - [ ] Connect ORP
  - [ ] Connect motion suppression

- [ ] Implement decision logic
  - [ ] Prioritize speed limits
  - [ ] Resolve conflicts (signal vs. time)
  - [ ] Handle mode transitions
  - [ ] Add safety overrides

- [ ] Add ATO output
  - [ ] Generate power/brake commands
  - [ ] Implement manual override
  - [ ] Add panel indicators
  - [ ] Add debug output

### 6.9 Phase 6 Validation / フェーズ6検証

- [ ] Functional verification
  - [ ] ATO activates correctly
  - [ ] Speed control works
  - [ ] Signal compliance works
  - [ ] ORP functions correctly
  - [ ] Early arrival prevention works

- [ ] Integration test
  - [ ] Test full route with ATO
  - [ ] Test all control modes
  - [ ] Test transitions between modes
  - [ ] Test edge cases

- [ ] Performance test
  - [ ] Smooth acceleration/deceleration
  - [ ] Passenger comfort
  - [ ] Accurate timing

- [ ] Documentation
  - [ ] Document ATO usage
  - [ ] Document control logic
  - [ ] Document Phase 6 completion

---

## Phase 7: Integration & Output / 統合・出力 (Weeks 15-16)

**Objective:** Complete plugin integration, output systems, and main orchestration

### 7.1 Output Module Structure / 出力モジュール構造

- [ ] Create `src/output/` module
  - [ ] Create `src/output/mod.rs`
  - [ ] Create `src/output/panel.rs`
  - [ ] Create `src/output/sound.rs`

### 7.2 Panel Output / パネル出力

- [ ] Port `パネル出力.h` → `output/panel.rs`
  - [ ] Define `PanelOutput` struct
  - [ ] Define panel index constants
  - [ ] Add output array

- [ ] Port `パネル出力.cpp` logic
  - [ ] Implement panel value setting
  - [ ] Add TASC indicators
  - [ ] Add ATO indicators
  - [ ] Add status indicators

- [ ] Implement panel features
  - [ ] Brake notch indicator
  - [ ] Power notch indicator
  - [ ] Remaining distance
  - [ ] Target speed
  - [ ] Mode indicators (TASC/ATO/ORP active)
  - [ ] State indicators

- [ ] Add panel configuration
  - [ ] Read panel mappings from config
  - [ ] Support custom panel layouts
  - [ ] Add default panel layout

- [ ] Write tests
  - [ ] Test panel value updates
  - [ ] Test indicator logic
  - [ ] Test configuration

### 7.3 Sound Output / 音声出力

- [ ] Port `音声出力.h` → `output/sound.rs`
  - [ ] Define `SoundOutput` struct
  - [ ] Define sound index constants
  - [ ] Add output array

- [ ] Implement sound triggers
  - [ ] TASC activation sound
  - [ ] ATO activation sound
  - [ ] Warning sounds
  - [ ] Confirmation sounds

- [ ] Add sound configuration
  - [ ] Read sound mappings from config
  - [ ] Support custom sound indices
  - [ ] Add default sound layout

- [ ] Write tests
  - [ ] Test sound triggers
  - [ ] Test configuration

### 7.4 Main Plugin Logic / メインプラグインロジック

- [ ] Port `Main.h` → `src/lib.rs` integration
  - [ ] Define `Plugin` struct
  - [ ] Add all module references
  - [ ] Add state management

- [ ] Port `Main.cpp` orchestration logic
  - [ ] Implement plugin initialization
  - [ ] Implement main update loop (`Elapse`)
  - [ ] Coordinate TASC and ATO
  - [ ] Handle mode switching

- [ ] Implement plugin lifecycle
  - [ ] Load: Initialize plugin
  - [ ] SetVehicleSpec: Configure vehicle
  - [ ] Initialize: Set initial state
  - [ ] Elapse: Main update (called every frame)
  - [ ] Dispose: Cleanup

### 7.5 FFI Integration / FFI統合

- [ ] Complete FFI layer in `src/ffi.rs`
  - [ ] Connect all plugin functions to Rust logic
  - [ ] Implement state persistence between calls
  - [ ] Add error handling
  - [ ] Add panic catching at FFI boundary

- [ ] Implement event handlers
  - [ ] `SetPower`: Handle power notch change
  - [ ] `SetBrake`: Handle brake notch change
  - [ ] `SetReverser`: Handle reverser change
  - [ ] `KeyDown`/`KeyUp`: Handle key events
  - [ ] `SetSignal`: Handle signal updates
  - [ ] `SetBeaconData`: Handle beacon data

- [ ] Add FFI safety
  - [ ] Validate all input data
  - [ ] Handle null pointers safely
  - [ ] Catch and log panics
  - [ ] Return safe defaults on error

### 7.6 Configuration Integration / 設定統合

- [ ] Load configuration on plugin load
  - [ ] Find autopilot.ini in plugin directory
  - [ ] Parse configuration
  - [ ] Validate settings
  - [ ] Apply to all modules

- [ ] Support configuration hot-reload (optional)
  - [ ] Detect config file changes
  - [ ] Reload settings
  - [ ] Apply without restart

### 7.7 Plugin Coordination / プラグイン協調

- [ ] Implement TASC/ATO coordination
  - [ ] Handle mode switching
  - [ ] Prioritize commands (TASC overrides ATO at stations)
  - [ ] Smooth transitions
  - [ ] Add mutual exclusion where needed

- [ ] Implement manual override
  - [ ] Detect driver input
  - [ ] Disable automatic control
  - [ ] Resume automatic control
  - [ ] Add safety checks

- [ ] Add emergency handling
  - [ ] Detect emergency situations
  - [ ] Override normal control
  - [ ] Apply emergency brake
  - [ ] Log emergency events

### 7.8 Logging and Diagnostics / ログと診断

- [ ] Add logging system (optional but recommended)
  - [ ] Use `log` crate
  - [ ] Add logging levels (error, warn, info, debug)
  - [ ] Log to file
  - [ ] Add log rotation

- [ ] Add diagnostic output
  - [ ] Log state transitions
  - [ ] Log control decisions
  - [ ] Log errors and warnings
  - [ ] Add performance metrics

### 7.9 Phase 7 Validation / フェーズ7検証

- [ ] Build verification
  - [ ] Build succeeds for both targets (x86, x64)
  - [ ] DLL exports all required functions
  - [ ] No warnings or errors

- [ ] Integration test
  - [ ] Load plugin in BVE
  - [ ] Test basic functionality
  - [ ] Test TASC operation
  - [ ] Test ATO operation
  - [ ] Test mode switching

- [ ] Output test
  - [ ] Verify panel output
  - [ ] Verify sound output
  - [ ] Test all indicators

- [ ] Documentation
  - [ ] Document plugin usage
  - [ ] Document configuration
  - [ ] Document Phase 7 completion

---

## Phase 8: Testing & Optimization / テスト・最適化 (Weeks 17-18)

**Objective:** Comprehensive testing, performance optimization, and bug fixing

### 8.1 Test Coverage / テストカバレッジ

- [ ] Measure current test coverage
  - [ ] Install `cargo-tarpaulin` or `cargo-llvm-cov`
  - [ ] Run coverage report
  - [ ] Identify untested code

- [ ] Improve test coverage
  - [ ] Add tests for uncovered modules
  - [ ] Add edge case tests
  - [ ] Add error path tests
  - [ ] Target: >80% coverage

- [ ] Add missing unit tests
  - [ ] Review all modules
  - [ ] Add tests for public APIs
  - [ ] Test internal logic

### 8.2 Integration Testing / 統合テスト

- [ ] Create comprehensive integration tests
  - [ ] Test full TASC workflow
  - [ ] Test full ATO workflow
  - [ ] Test TASC + ATO coordination
  - [ ] Test all compatibility modes

- [ ] Create scenario tests
  - [ ] Typical station stop
  - [ ] Signal approach
  - [ ] Speed limit change
  - [ ] Gradient traversal
  - [ ] Emergency stop

- [ ] Add regression tests
  - [ ] Test against known C++ behaviors
  - [ ] Add tests for fixed bugs
  - [ ] Prevent future regressions

### 8.3 Validation Against C++ Version / C++版との検証

- [ ] Create comparison tests
  - [ ] Use same input data for both versions
  - [ ] Compare outputs
  - [ ] Document differences
  - [ ] Ensure differences are acceptable or fix

- [ ] Validate calculations
  - [ ] Compare deceleration patterns
  - [ ] Compare brake commands
  - [ ] Compare stopping distances
  - [ ] Validate within tolerance (< 1%)

- [ ] Test with real route data
  - [ ] Use BVE route files
  - [ ] Compare behavior side-by-side
  - [ ] Document any behavioral changes

### 8.4 Performance Benchmarking / パフォーマンスベンチマーク

- [ ] Create benchmarks with Criterion
  - [ ] Benchmark `Elapse` function (main loop)
  - [ ] Benchmark deceleration pattern calculation
  - [ ] Benchmark gradient graph lookup
  - [ ] Benchmark brake command calculation

- [ ] Measure performance
  - [ ] Run benchmarks: `cargo bench`
  - [ ] Record baseline metrics
  - [ ] Compare with C++ version
  - [ ] Identify bottlenecks

- [ ] Performance targets
  - [ ] `Elapse` < 1ms (ideally < 0.5ms)
  - [ ] 60 FPS support (16.67ms frame time)
  - [ ] Memory usage < C++ version or similar

### 8.5 Optimization / 最適化

- [ ] Profile code
  - [ ] Use `cargo flamegraph` or `perf`
  - [ ] Identify hot paths
  - [ ] Find allocation hotspots
  - [ ] Locate unnecessary work

- [ ] Optimize hot paths
  - [ ] Optimize deceleration calculations
  - [ ] Optimize graph lookups
  - [ ] Reduce allocations
  - [ ] Use iterators efficiently

- [ ] Memory optimization
  - [ ] Reduce clone operations
  - [ ] Use references where possible
  - [ ] Optimize data structures
  - [ ] Add object pooling if needed

- [ ] Algorithm optimization
  - [ ] Replace O(n) with O(log n) where possible
  - [ ] Cache frequently used calculations
  - [ ] Precompute static data
  - [ ] Use SIMD if beneficial (optional)

### 8.6 Bug Fixing / バグ修正

- [ ] Test all edge cases
  - [ ] Zero velocity
  - [ ] Zero distance
  - [ ] Negative values (invalid input)
  - [ ] Very large values
  - [ ] Extreme gradients

- [ ] Fix identified bugs
  - [ ] Create issue for each bug
  - [ ] Write failing test
  - [ ] Fix bug
  - [ ] Verify test passes
  - [ ] Add regression test

- [ ] Stress testing
  - [ ] Long-duration tests
  - [ ] Rapid state changes
  - [ ] Invalid beacon data
  - [ ] Configuration errors

### 8.7 Code Quality / コード品質

- [ ] Run Clippy with strict settings
  - [ ] `cargo clippy -- -W clippy::all -W clippy::pedantic`
  - [ ] Fix all warnings
  - [ ] Add `#[allow]` only with justification

- [ ] Format all code
  - [ ] Run `cargo fmt`
  - [ ] Configure rustfmt (rustfmt.toml)
  - [ ] Ensure consistent style

- [ ] Review code
  - [ ] Self-review all code
  - [ ] Check for unwraps (replace with proper error handling)
  - [ ] Verify panic-free in FFI boundaries
  - [ ] Add comments for complex logic

### 8.8 Memory Safety Verification / メモリ安全性検証

- [ ] Run Miri (Rust's interpreter for detecting undefined behavior)
  - [ ] `cargo +nightly miri test`
  - [ ] Fix any undefined behavior
  - [ ] Verify unsafe code

- [ ] Memory leak check
  - [ ] Use Valgrind or AddressSanitizer
  - [ ] Test long-running scenarios
  - [ ] Verify no leaks

- [ ] Audit unsafe code
  - [ ] Review all `unsafe` blocks
  - [ ] Document safety invariants
  - [ ] Minimize unsafe usage

### 8.9 Phase 8 Validation / フェーズ8検証

- [ ] All tests pass
  - [ ] `cargo test` passes all tests
  - [ ] No ignored tests
  - [ ] Coverage > 80%

- [ ] Performance acceptable
  - [ ] Benchmarks meet targets
  - [ ] Performance within 5% of C++
  - [ ] No performance regressions

- [ ] Quality checks
  - [ ] No Clippy warnings
  - [ ] Code formatted
  - [ ] Documentation complete

- [ ] Ready for release
  - [ ] All critical bugs fixed
  - [ ] Known issues documented
  - [ ] Release notes drafted

---

## Phase 9: Documentation & Release / ドキュメント・リリース (Weeks 19-20)

**Objective:** Complete documentation, prepare release, and publish

### 9.1 API Documentation / APIドキュメント

- [ ] Complete doc comments
  - [ ] All public items have doc comments
  - [ ] All modules have module-level docs
  - [ ] Examples in doc comments
  - [ ] Link related items

- [ ] Generate documentation
  - [ ] Run `cargo doc --no-deps`
  - [ ] Review generated docs
  - [ ] Fix formatting issues
  - [ ] Add missing documentation

- [ ] Add usage examples
  - [ ] Add examples to doc comments
  - [ ] Create `examples/` directory
  - [ ] Add standalone example programs

### 9.2 User Documentation / ユーザードキュメント

- [ ] Update README.md
  - [ ] Add Rust version information
  - [ ] Update build instructions
  - [ ] Update usage instructions
  - [ ] Add migration guide for C++ users

- [ ] Create installation guide
  - [ ] Document how to install plugin
  - [ ] Document configuration
  - [ ] Document compatibility

- [ ] Create user manual
  - [ ] Document TASC usage
  - [ ] Document ATO usage
  - [ ] Document key bindings
  - [ ] Document panel indicators
  - [ ] Add troubleshooting section

### 9.3 Developer Documentation / 開発者ドキュメント

- [ ] Create developer guide
  - [ ] Document project structure
  - [ ] Explain architecture
  - [ ] Document build process
  - [ ] Add contribution guidelines

- [ ] Create migration guide
  - [ ] Document migration from C++
  - [ ] Explain design decisions
  - [ ] Document differences from C++ version
  - [ ] Add lessons learned

- [ ] Document algorithms
  - [ ] Update algorithm.md if needed
  - [ ] Add Rust-specific implementation notes
  - [ ] Add performance notes

### 9.4 Configuration Documentation / 設定ドキュメント

- [ ] Document autopilot.ini format
  - [ ] Document all settings
  - [ ] Add examples
  - [ ] Document defaults
  - [ ] Add validation rules

- [ ] Create configuration examples
  - [ ] Example for different train types
  - [ ] Example for different routes
  - [ ] Example for compatibility modes

### 9.5 Release Preparation / リリース準備

- [ ] Update version numbers
  - [ ] Update Cargo.toml version to 2.0.0
  - [ ] Update documentation versions
  - [ ] Update FFI version

- [ ] Create CHANGELOG.md
  - [ ] Document all changes from C++ version
  - [ ] List new features
  - [ ] List breaking changes
  - [ ] List bug fixes

- [ ] Prepare release notes
  - [ ] Highlight major features
  - [ ] Explain migration from v1.x
  - [ ] Add known issues
  - [ ] Add upgrade instructions

### 9.6 Build Release Artifacts / リリース成果物のビルド

- [ ] Build release DLLs
  - [ ] Build x86 (32-bit): `cargo build --release --target i686-pc-windows-msvc`
  - [ ] Build x64 (64-bit): `cargo build --release --target x86_64-pc-windows-msvc`
  - [ ] Verify DLL exports
  - [ ] Test DLLs in BVE

- [ ] Create release package
  - [ ] Include DLLs (x86 and x64)
  - [ ] Include default autopilot.ini
  - [ ] Include README
  - [ ] Include LICENSE
  - [ ] Include CHANGELOG

- [ ] Create source archive
  - [ ] `cargo package`
  - [ ] Verify package contents
  - [ ] Test building from package

### 9.7 Testing Release / リリーステスト

- [ ] Test installation
  - [ ] Test on clean BVE installation
  - [ ] Test with various routes
  - [ ] Test all features
  - [ ] Verify compatibility

- [ ] Beta testing (if applicable)
  - [ ] Distribute to beta testers
  - [ ] Collect feedback
  - [ ] Fix reported issues
  - [ ] Update documentation

### 9.8 Publication / 公開

- [ ] Prepare GitHub release
  - [ ] Create release tag: `v2.0.0`
  - [ ] Upload release artifacts
  - [ ] Add release notes
  - [ ] Mark as pre-release or stable

- [ ] Update repository
  - [ ] Merge to main branch
  - [ ] Update README badges
  - [ ] Update documentation links
  - [ ] Archive C++ version branch

- [ ] Publish crate (optional)
  - [ ] Publish to crates.io: `cargo publish`
  - [ ] Verify crate page
  - [ ] Update documentation links

### 9.9 Announcement / アナウンス

- [ ] Announce release
  - [ ] Post on project website/forum
  - [ ] Update BVE community sites
  - [ ] Share on relevant channels

- [ ] Create demo materials
  - [ ] Record demo video (like C++ version)
  - [ ] Create screenshots
  - [ ] Write blog post (optional)

### 9.10 Post-Release / リリース後

- [ ] Monitor issues
  - [ ] Watch for bug reports
  - [ ] Respond to questions
  - [ ] Plan patch releases if needed

- [ ] Create roadmap for future versions
  - [ ] Plan v2.1 features
  - [ ] Collect feature requests
  - [ ] Prioritize improvements

- [ ] Document lessons learned
  - [ ] What went well
  - [ ] What could be improved
  - [ ] Update migration guide

---

## Progress Tracking / 進捗管理

### Overall Progress / 全体進捗

- [ ] Phase 1: Foundation (Weeks 1-2)
- [ ] Phase 2: State Management (Weeks 3-4)
- [ ] Phase 3: Vehicle & Environment (Weeks 5-6)
- [ ] Phase 4: Control Algorithms (Weeks 7-9)
- [ ] Phase 5: TASC Implementation (Weeks 10-11)
- [ ] Phase 6: ATO Implementation (Weeks 12-14)
- [ ] Phase 7: Integration & Output (Weeks 15-16)
- [ ] Phase 8: Testing & Optimization (Weeks 17-18)
- [ ] Phase 9: Documentation & Release (Weeks 19-20)

### Milestone Checklist / マイルストーンチェックリスト

- [ ] **M1:** DLL builds and loads in BVE (Phase 1)
- [ ] **M2:** Configuration loads successfully (Phase 2)
- [ ] **M3:** Vehicle characteristics calculated (Phase 3)
- [ ] **M4:** Deceleration pattern works (Phase 4)
- [ ] **M5:** TASC stops train at station (Phase 5)
- [ ] **M6:** ATO controls speed automatically (Phase 6)
- [ ] **M7:** Full plugin functionality (Phase 7)
- [ ] **M8:** Performance optimized (Phase 8)
- [ ] **M9:** v2.0.0 released (Phase 9)

---

## Notes / 注意事項

### Critical Success Factors / 重要成功要因

1. **Continuous Testing:** Test after each module to catch issues early
2. **Benchmarking:** Compare with C++ version throughout
3. **Documentation:** Document as you go, not at the end
4. **Version Control:** Commit frequently with clear messages
5. **Validation:** Validate outputs against C++ version

### Risk Management / リスク管理

1. **Performance Issues:** Benchmark early and often
2. **FFI Problems:** Test FFI boundary thoroughly
3. **Behavior Differences:** Compare with C++ version continuously
4. **Time Overruns:** Each phase can be extended if needed
5. **Scope Creep:** Stick to feature parity first, enhancements later

### Resources / リソース

- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [uom Documentation](https://docs.rs/uom/)
- [Windows-rs Documentation](https://docs.rs/windows/)
- Original C++ codebase
- `algorithm.md` for algorithms
- `CURRENT_STRUCTURE.md` for architecture reference
- `RUST_MIGRATION_PLAN.md` for strategy

---

**Last Updated:** 2025-11-08
**Status:** Ready for Phase 1
**Estimated Completion:** Week 20 (約5ヶ月後)
