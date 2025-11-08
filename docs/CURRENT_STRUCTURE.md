# bve-autopilot Current Structure Documentation

## Overview / 概要

bve-autopilot is a BVE trainsim 5/6 plugin providing TASC (Target Station Stopping Control) and ATO (Automatic Train Operation) functionality. The codebase is written in C++20 with Japanese identifiers and consists of approximately 9,357 lines of code.

bve-autopilotは、Bve trainsim 5/6向けの保安装置プラグインで、TASC（定位置停止装置）とATO（自動列車運転装置）の機能を提供します。コードベースはC++20で記述され、日本語識別子を使用し、約9,357行のコードで構成されています。

## Directory Structure / ディレクトリ構造

```
bve-autopilot/
├── bve-autopilot/              # Main plugin project / メインプラグインプロジェクト
│   ├── *.cpp, *.h              # Source files (66 files) / ソースファイル（66ファイル）
│   ├── bve-autopilot.vcxproj   # Visual Studio project / Visual Studioプロジェクト
│   ├── autopilot.def           # DLL export definition / DLLエクスポート定義
│   └── stdafx.h                # Precompiled header / プリコンパイル済みヘッダー
├── bve-autopilot-test/         # Test project / テストプロジェクト
│   ├── *テスト.cpp             # Test files (4 files) / テストファイル（4ファイル）
│   └── bve-autopilot-test.vcxproj
├── README.md                   # Project documentation / プロジェクト説明
├── algorithm.md                # Algorithm documentation / アルゴリズム解説
├── LICENSE                     # LGPL 2.1 license / ライセンス
└── bve-autopilot.sln          # Visual Studio solution / ソリューション
```

## Technology Stack / 技術スタック

- **Language:** C++20 (Main project), C++17 (Test project)
- **Build System:** MSBuild (Visual Studio 2022)
- **Target Platform:** Windows (Win32 / x64)
- **Output:** DLL (Dynamic Link Library)
- **Test Framework:** Visual Studio Native Unit Test

## Module Architecture / モジュール構成

### 1. Entry Point & Integration / エントリーポイント・統合

| File | Description (EN) | Description (JP) |
|------|------------------|------------------|
| `bve-autopilot.cpp` | DLL export functions | DLLエクスポート関数 |
| `Main.h/cpp` | Plugin orchestration | プラグイン全体統括 |
| `atsplugin.h` | BVE ATS plugin API definition | BVE ATSプラグインAPI定義 |

### 2. TASC (Target Station Stopping Control) / 定位置停止制御

| File | Description (EN) | Description (JP) |
|------|------------------|------------------|
| `tasc.h/cpp` | TASC main module | TASCメインモジュール |
| - | Automatic stopping at designated station positions | 駅の所定位置への自動停止 |
| - | Inching function (forward only) | インチング機能（前進のみ）|

### 3. ATO (Automatic Train Operation) / 自動列車運転

| File | Description (EN) | Description (JP) |
|------|------------------|------------------|
| `ato.h/cpp` | ATO main module | ATOメインモジュール |
| `信号順守.h/cpp` | Signal compliance speed control | 信号に従った速度制御 |
| `早着防止.h/cpp` | Early arrival prevention | 目標時刻への速度調整 |
| `orp.h/cpp` | ORP (Over Run Protection) | ORP照査機能 |

### 4. Control Algorithms / 制御アルゴリズム

| File | Description (EN) | Description (JP) |
|------|------------------|------------------|
| `減速パターン.h/cpp` | Deceleration pattern calculation | 等減速度運動パターン計算 |
| `減速目標.h/cpp` | Deceleration target management | 減速目標管理 |
| `制動指令計算.h` | Braking command calculation | 制動指令計算 |
| `出力制御.h/cpp` | Notch output control | ノッチ出力制御 |
| `急動作抑制.h/cpp` | Sudden motion suppression | 急激な動作の抑制 |

### 5. Vehicle Characteristics / 車両特性管理

| File | Description (EN) | Description (JP) |
|------|------------------|------------------|
| `制動特性.h/cpp` | Brake characteristics calculation | ブレーキ特性計算 |
| `力行特性.h/cpp` | Power characteristics calculation | 力行特性計算 |
| `制動力推定.h/cpp` | Braking force estimation | ブレーキ力推定 |
| `加速度計.h/cpp` | Accelerometer | 加速度測定 |

### 6. Environment & Terrain / 環境・地形管理

| File | Description (EN) | Description (JP) |
|------|------------------|------------------|
| `勾配グラフ.h/cpp` | Gradient graph management | 勾配情報管理 |
| `制限グラフ.h/cpp` | Speed limit graph | 速度制限グラフ |
| `区間.h/cpp` | Distance section management | 距離区間管理 |

### 7. State Management / 状態管理

| File | Description (EN) | Description (JP) |
|------|------------------|------------------|
| `共通状態.h/cpp` | Shared plugin state | プラグイン全体の状態量 |
| `運動状態.h/cpp` | Train motion state | 列車の運動状態 |
| `稼働状態.h` | ATO/TASC active state | ATO/TASC有効状態 |
| `環境設定.h/cpp` | Configuration file loading | 設定ファイル読込 |

### 8. Type Safety System / 型安全システム

| File | Description (EN) | Description (JP) |
|------|------------------|------------------|
| `物理量.h` | Type-safe physical quantities | 型安全な物理量定義 |
| `制御指令.h` | Notch command types | ノッチ指令型定義 |

**Physical Quantity Types / 物理量の型:**
- Distance / 距離: `m` (meters), `cm` (centimeters)
- Velocity / 速度: `mps` (m/s), `kmph` (km/h)
- Acceleration / 加速度: `mps2` (m/s²), `kmphps` (km/h/s)
- Time / 時刻: `時刻` class, `s` (seconds)

### 9. Output / 出力機能

| File | Description (EN) | Description (JP) |
|------|------------------|------------------|
| `パネル出力.h/cpp` | Panel display output | 計器パネル表示 |
| `音声出力.h` | Sound control | 音声制御 |

### 10. Utilities / ユーティリティ

| File | Description (EN) | Description (JP) |
|------|------------------|------------------|
| `live.h` | Observable values | 監視可能な値 |
| `stdafx.h` | Precompiled headers | プリコンパイル済みヘッダー |

## Core Algorithm / コアアルゴリズム

### Deceleration Pattern Control / 減速パターン制御

The core formula for deceleration control is:

```
Output Deceleration = Expected Deceleration * (Current Velocity / Expected Velocity)
                    - (Expected Velocity - Current Velocity) / 2 seconds
```

減速制御の核心式：

```
出力減速度 = 期待減速度 * (現在速度 / 期待速度) - (期待速度 - 現在速度) / 2秒
```

This formula ensures:
- Calculation of deceleration to reach the deceleration pattern in 2 seconds
- Avoidance of sudden changes in deceleration
- Compensation for brake response delay

この式により：
- 2秒後に減速パターンに到達する減速度を計算
- 減速度の急激な変化を避ける
- ブレーキ応答の遅れに対応

## Dependencies / 依存関係

### External APIs / 外部API

**BVE Plugin API:**
- `ATS_VEHICLESPEC` - Vehicle specifications / 車両仕様
- `ATS_VEHICLESTATE` - Vehicle state / 車両状態
- `ATS_BEACONDATA` - Beacon data / 地上子データ
- `ATS_HANDLES` - Control handles / 制御ハンドル

**Windows API:**
- `GetModuleFileNameW` - DLL path retrieval / DLLパス取得
- File system operations / ファイルシステム操作

**C++ Standard Library:**
- `<chrono>` - Time calculations / 時間計算
- `<filesystem>` - File operations / ファイル操作
- `<map>`, `<vector>`, `<deque>` - Data structures / データ構造
- `<algorithm>` - Algorithms / アルゴリズム

## Configuration / 設定

**Configuration File:** `autopilot.ini`

| Setting | Description (EN) | Description (JP) |
|---------|------------------|------------------|
| Vehicle length | Train length configuration | 車両長 |
| Max deceleration | Maximum deceleration rate | 最大減速度 |
| Brake reaction time | Brake response time | ブレーキ反応時間 |
| Extended brake notches | Extended brake control (15-31 levels) | 拡張ブレーキノッチ |
| Pressure rates | Brake pressure characteristics | 空気圧特性 |
| Key assignments | Control key bindings | キー割り当て |
| Panel output | Panel display configuration | パネル出力設定 |

**Compatibility Modes / 互換モード:**
- Generic ATS / 汎用ATS
- Metro Integrated Plugin / メトロ総合プラグイン
- SWP2 Plugin
- Odakyu D-ATS-P / 小田急D-ATS-P
- Odakyu CS-ATC / 小田急CS-ATC
- Metro TASC / メトロTASC

## Module Dependencies / モジュール依存関係

```
Main
├── 共通状態 (Shared State)
│   ├── 環境設定 (Configuration)
│   ├── 車両仕様 (Vehicle Specs)
│   └── 運動状態 (Motion State)
├── TASC
│   ├── 減速パターン (Deceleration Pattern)
│   └── 出力制御 (Output Control)
└── ATO
    ├── 信号順守 (Signal Compliance)
    │   ├── 制限グラフ (Speed Limit Graph)
    │   └── ORP
    ├── 早着防止 (Early Arrival Prevention)
    └── 急動作抑制 (Sudden Motion Suppression)
```

## Test Structure / テスト構造

**Test Files (4 files, 59,682 total lines):**

| File | Lines | Description (EN) | Description (JP) |
|------|-------|------------------|------------------|
| `勾配グラフテスト.cpp` | 35,778 | Gradient graph tests | 勾配グラフのテスト |
| `tascテスト.cpp` | 13,932 | TASC operation tests | TASC動作のテスト |
| `力行特性テスト.cpp` | 7,808 | Power characteristics tests | 力行特性のテスト |
| `減速目標テスト.cpp` | 2,164 | Deceleration target tests | 減速目標のテスト |

## Build Configuration / ビルド設定

**Compiler Options / コンパイラオプション:**
- `/utf-8` - UTF-8 encoding for Japanese identifiers
- `Level4` - Warning level 4
- `ConformanceMode` - Standards conformance mode
- `MultiProcessorCompilation` - Multi-core compilation

**Optimization / 最適化:**
- Debug: Disabled / 最適化無効
- Release: MaxSpeed with function-level linking / 最速最適化、関数レベルリンク

## Features / 主要機能

### TASC Features / TASC機能
- Precise stopping at designated station positions / 駅の所定位置への停車（誤差最小化）
- Multiple beacon protocol support / 複数の地上子プロトコル対応
- Inching for position correction / インチング（停止位置修正）
- Panel display (brake notch, remaining distance) / 計器パネル表示（ブレーキノッチ、残距離）

### ATO Features / ATO機能
- Automatic speed control / 速度の自動制御
- Signal-based speed control (for non-ATC lines) / 信号による速度制御（非ATC路線対応）
- Advance deceleration based on advance signals / 前方予告による事前減速
- Speed adjustment to target time / 目標時刻への速度調整
- Panel display (notch, speed limit) / 計器パネル表示（ノッチ、制限速度）

### Advanced Control / 高度な制御機能
- Gradient compensation / 勾配補正
- Air brake and electric brake characteristic tracking / 空気ブレーキ・電気ブレーキの特性追従
- Extended brake commands (15-31 levels) / 拡張ブレーキ指令（15-31段階）
- Load-responsive control support / 応荷重制御対応

## License / ライセンス

**LGPL 2.1 (GNU Lesser General Public License)**
- Free to use as a plugin / プラグインとしての使用は自由
- Source code must be disclosed when distributing modified versions / 改変版の公開時はソースコード公開が必要
- Commercial use allowed / 商用利用可能
- Copyright © 2019-2020 Watanabe, Yuki

## Code Statistics / コード統計

- **Total source files:** 66 files (main project) + 4 files (test project)
- **Total lines of code:** ~9,357 lines (source) + ~59,682 lines (tests)
- **Primary language:** C++20
- **Character encoding:** UTF-8
- **Identifier language:** Japanese

## Key Design Principles / 主要な設計原則

1. **Type Safety / 型安全性:** Physical quantities represented in type system
2. **Modularity / モジュール性:** Clear separation of concerns between modules
3. **Extensibility / 拡張性:** Support for multiple compatibility modes
4. **Precision / 精密性:** Physics-based algorithms for accurate control
5. **Documentation / ドキュメント化:** Detailed algorithm documentation in algorithm.md
