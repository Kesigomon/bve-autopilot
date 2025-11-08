# Rust Migration Documentation Guide / Rust移行ドキュメントガイド

Welcome to the bve-autopilot Rust migration project! This guide will help you navigate the migration documentation and understand where to find information.

bve-autopilot Rust移行プロジェクトへようこそ！このガイドは、移行ドキュメントのナビゲーションと、情報の場所を理解するのに役立ちます。

---

## 📁 Document Structure / ドキュメント構造

```
bve-autopilot/
├── README.md                      # Original project documentation
├── algorithm.md                   # Algorithm documentation (Japanese)
├── LICENSE                        # LGPL 2.1 License
├── CLAUDE.md                      # Instructions for Claude/AI assistants
│
└── docs/                          # Migration documentation
    ├── 📘 RUST_MIGRATION_GUIDE.md     # ← START HERE / ここから始める
    │   └── This document (you are here)
    │
    ├── 📗 CURRENT_STRUCTURE.md        # Current C++ codebase documentation
    │   └── Reference for understanding existing code
    │
    ├── 📕 RUST_MIGRATION_PLAN.md      # Overall migration strategy
    │   └── High-level plan and architecture
    │
    └── 📙 MIGRATION_TODO.md           # Detailed task checklist
        └── Phase-by-phase implementation tasks
```

---

## 📚 Document Overview / ドキュメント概要

### 1. 📘 RUST_MIGRATION_GUIDE.md (This Document)

**Location:** `/home/user/bve-autopilot/docs/RUST_MIGRATION_GUIDE.md`

**Purpose / 目的:**
- Entry point for the Rust migration project
- Explains the documentation structure
- Provides quick start instructions
- Links to all other documents

**When to use / 使用タイミング:**
- 📍 **First time** exploring the migration project
- When you need to find specific documentation
- To understand the overall documentation structure

---

### 2. 📗 CURRENT_STRUCTURE.md

**Location:** `/home/user/bve-autopilot/docs/CURRENT_STRUCTURE.md`

**Purpose / 目的:**
- Comprehensive documentation of the existing C++20 codebase
- Module architecture and dependencies
- File-by-file breakdown (66 source files)
- Current features and capabilities
- Bilingual (Japanese/English)

**Contents / 内容:**
- Directory structure
- Technology stack (C++20, MSBuild, Visual Studio)
- Module classification (TASC, ATO, Control, Vehicle, Environment)
- Core algorithms
- Dependencies (BVE API, Windows API, STL)
- Configuration system
- Test structure
- Build configuration

**When to use / 使用タイミング:**
- 📍 **Understanding** the existing codebase structure
- When porting a specific module (to see what it does)
- To understand module dependencies
- When comparing Rust implementation with C++ original
- Looking up Japanese → English module name mappings

**Key Sections / 重要セクション:**
- Module Architecture (Section 2) - Shows all 66 files organized by function
- Module Dependencies (Section 10) - Shows how modules relate
- Core Algorithm (Section 5) - Explains deceleration control formula

---

### 3. 📕 RUST_MIGRATION_PLAN.md

**Location:** `/home/user/bve-autopilot/docs/RUST_MIGRATION_PLAN.md`

**Purpose / 目的:**
- Overall migration strategy and approach
- English naming conventions (Japanese → English)
- Proposed Rust architecture
- Type system design
- FFI strategy
- 9-phase migration roadmap

**Contents / 内容:**
1. Migration Objectives - Why migrate to Rust?
2. Why Rust? - Advantages for this project
3. **English Naming Convention** - Complete translation dictionary
4. Module Migration Mapping - C++ → Rust module structure
5. Type System Migration - Physical quantities with `uom` crate
6. **Migration Phases** - 20-week timeline (9 phases)
7. Build System - Cargo configuration
8. FFI Strategy - Windows DLL exports
9. Testing Strategy - Unit tests, integration tests, benchmarks
10. Timeline & Milestones

**When to use / 使用タイミング:**
- 📍 **Planning** the migration approach
- When deciding how to translate Japanese names to English
- To understand the proposed Rust architecture
- When making architectural decisions
- To see the overall timeline and phases
- Looking up naming conventions

**Key Sections / 重要セクション:**
- **English Naming Convention (Section 3)** - Essential translation dictionary
  - Core module names: `共通状態` → `shared_state`
  - Physical quantities: `速度` → `Velocity`
  - Variables: `現在速度` → `current_velocity`
  - Railway terms: `保安装置` → `Safety Device`
- Migration Phases (Section 6) - High-level phase breakdown
- Type System Migration (Section 5) - How to use `uom` crate
- FFI Strategy (Section 8) - Example FFI code

---

### 4. 📙 MIGRATION_TODO.md

**Location:** `/home/user/bve-autopilot/docs/MIGRATION_TODO.md`

**Purpose / 目的:**
- Detailed task breakdown for each phase
- Actionable checklist with 300+ tasks
- Step-by-step implementation guide
- Progress tracking

**Contents / 内容:**
- **Phase 1:** Foundation (47 tasks) - Project setup, FFI, types
- **Phase 2:** State Management (36 tasks) - State modules
- **Phase 3:** Vehicle & Environment (40 tasks) - Characteristics, graphs
- **Phase 4:** Control Algorithms (38 tasks) - Deceleration, braking
- **Phase 5:** TASC Implementation (31 tasks) - TASC features
- **Phase 6:** ATO Implementation (42 tasks) - ATO features
- **Phase 7:** Integration & Output (33 tasks) - Panel, sound, main
- **Phase 8:** Testing & Optimization (40 tasks) - Tests, benchmarks
- **Phase 9:** Documentation & Release (36 tasks) - Docs, release
- Progress Tracking - Overall checklist and milestones

**When to use / 使用タイミング:**
- 📍 **During implementation** of each phase
- When you need detailed step-by-step tasks
- To track progress through checkboxes
- When stuck and need to see what's next
- Daily work planning

**How to use / 使い方:**
1. Navigate to the current phase section
2. Read through the subsections (e.g., "2.1 Module Structure")
3. Check off tasks as you complete them
4. Complete "Phase X Validation" before moving to next phase
5. Update "Overall Progress" checklist

**Key Sections / 重要セクション:**
- Each Phase's validation section - Ensures phase completion
- Progress Tracking (Section 10) - Overall milestone checklist
- Notes section - Critical success factors and risk management

---

## 🗺️ Document Relationships / ドキュメントの関係

```
┌─────────────────────────────────────────────────────────┐
│  RUST_MIGRATION_GUIDE.md (Start Here)                  │
│  ├─ Navigation hub for all documents                   │
│  └─ Quick start instructions                           │
└─────────────────────────────────────────────────────────┘
                           │
        ┌──────────────────┼──────────────────┐
        ↓                  ↓                  ↓
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│   CURRENT    │  │     RUST     │  │  MIGRATION   │
│  STRUCTURE   │  │  MIGRATION   │  │     TODO     │
│              │  │     PLAN     │  │              │
│  What exists │  │  How to do   │  │  Step-by-    │
│  now (C++)   │  │  it (Rust)   │  │  step tasks  │
│              │  │              │  │              │
│  Reference   │←→│  Strategy    │←→│  Execution   │
│  during      │  │  Naming      │  │  Checklist   │
│  porting     │  │  conventions │  │              │
└──────────────┘  └──────────────┘  └──────────────┘

Legend:
←→ : Reference between documents
│  : Hierarchical relationship
```

---

## 🚀 Quick Start Guide / クイックスタートガイド

### For Beginners / 初心者向け

If you're new to the Rust migration project, follow these steps:

Rust移行プロジェクトが初めての場合は、以下の手順に従ってください：

1. **Read this document** (RUST_MIGRATION_GUIDE.md) completely
   - Understand the documentation structure
   - このドキュメントを最後まで読む

2. **Skim CURRENT_STRUCTURE.md**
   - Get familiar with the existing codebase
   - Understand what TASC and ATO do
   - 既存のコードベースの概要を把握する

3. **Read RUST_MIGRATION_PLAN.md** carefully
   - Understand the migration strategy
   - **Study Section 3 (English Naming Convention)** thoroughly
   - Review the proposed Rust architecture
   - 移行戦略を理解する、特に命名規則を熟読

4. **Start with Phase 1 in MIGRATION_TODO.md**
   - Begin implementing tasks
   - Check off completed items
   - Phase 1から実装を開始する

### For Experienced Developers / 経験豊富な開発者向け

If you're experienced with Rust and C++:

RustとC++に精通している場合：

1. **Review RUST_MIGRATION_PLAN.md** (30 minutes)
   - Focus on architecture and naming conventions
   - アーキテクチャと命名規則に注目

2. **Reference CURRENT_STRUCTURE.md** as needed
   - Look up specific modules when porting
   - 必要に応じて特定のモジュールを参照

3. **Use MIGRATION_TODO.md** as your daily checklist
   - Track your progress
   - 日々のチェックリストとして使用

---

## 📖 How to Use This Documentation / ドキュメントの使い方

### Scenario 1: Starting Phase 1 / フェーズ1を開始

**Goal:** Set up Rust project infrastructure

**Documents to use:**
1. **MIGRATION_TODO.md** → Phase 1 section
   - Follow checklist items 1.1 through 1.7
2. **RUST_MIGRATION_PLAN.md** → Section 7 (Build System)
   - Reference Cargo.toml configuration
3. **RUST_MIGRATION_PLAN.md** → Section 8 (FFI Strategy)
   - Reference FFI code examples

**順序:**
1. TODOのPhase 1セクションを開く
2. 1.1から順番にタスクを実行
3. わからない場合はMIGRATION_PLANを参照

---

### Scenario 2: Porting a Specific Module (e.g., 制動特性) / 特定モジュールの移行

**Goal:** Port brake characteristics module from C++ to Rust

**Documents to use:**
1. **CURRENT_STRUCTURE.md** → Section 2.5 (Vehicle Characteristics)
   - Understand what `制動特性.cpp/h` does
   - See module dependencies
2. **RUST_MIGRATION_PLAN.md** → Section 3 (Naming Convention)
   - Look up translation: `制動特性` → `brake_characteristics`
   - Look up related terms: `制動力` → `braking_force`
3. **RUST_MIGRATION_PLAN.md** → Section 4 (Module Mapping)
   - See target location: `src/vehicle/brake_characteristics.rs`
4. **MIGRATION_TODO.md** → Phase 3, Section 3.2
   - Follow implementation checklist
5. **Original C++ files:** `bve-autopilot/制動特性.cpp` and `.h`
   - Reference the original implementation

**順序:**
1. CURRENT_STRUCTUREで現在の実装を理解
2. MIGRATION_PLANで英語名と配置場所を確認
3. MIGRATION_TODOでタスクリストに従う
4. 元のC++コードを参照しながら実装

---

### Scenario 3: Understanding the Deceleration Algorithm / 減速アルゴリズムの理解

**Goal:** Understand how the deceleration control works

**Documents to use:**
1. **algorithm.md** (existing document in repository)
   - Read the detailed algorithm explanation in Japanese
2. **CURRENT_STRUCTURE.md** → Section 5 (Core Algorithm)
   - See the formula in English with explanation
3. **RUST_MIGRATION_PLAN.md** → Section 3 (Naming Convention)
   - Look up term translations
4. **MIGRATION_TODO.md** → Phase 4, Section 4.2
   - See implementation tasks for deceleration pattern

**Formula / 式:**
```
出力減速度 = 期待減速度 * (現在速度 / 期待速度) - (期待速度 - 現在速度) / 2秒

Output Deceleration = Expected Deceleration * (Current Velocity / Expected Velocity)
                    - (Expected Velocity - Current Velocity) / 2 seconds
```

---

### Scenario 4: Naming a Variable / 変数の命名

**Goal:** Translate a Japanese variable name to English

**Documents to use:**
1. **RUST_MIGRATION_PLAN.md** → Section 3 (English Naming Convention)
   - Check translation tables:
     - Section 3.2: Core Modules
     - Section 3.3: TASC & ATO Modules
     - Section 3.4: Vehicle & Environment
     - Section 3.5: Physical Quantities
     - Section 3.6: Common Variables
     - Section 3.7: Control & State
     - Section 3.8: Railway-Specific Terms

**Example lookups / 例:**
- `現在位置` → `current_position`
- `目標速度` → `target_velocity`
- `制動距離` → `braking_distance`
- `有効` → `is_active` or `enabled`

**If not found / 見つからない場合:**
1. Use descriptive English following Rust conventions
2. Prefer clarity over brevity
3. Document your choice in code comments
4. Consider adding to the dictionary for future reference

---

### Scenario 5: Tracking Progress / 進捗の追跡

**Goal:** See how much is completed and what's next

**Documents to use:**
1. **MIGRATION_TODO.md** → Progress Tracking section (bottom)
   - Check overall phase completion
   - Review milestone checklist
2. **MIGRATION_TODO.md** → Current phase validation section
   - Ensure current phase is complete before moving on

**Milestones / マイルストーン:**
- **M1:** DLL builds and loads in BVE (Phase 1)
- **M2:** Configuration loads successfully (Phase 2)
- **M3:** Vehicle characteristics calculated (Phase 3)
- **M4:** Deceleration pattern works (Phase 4)
- **M5:** TASC stops train at station (Phase 5)
- **M6:** ATO controls speed automatically (Phase 6)
- **M7:** Full plugin functionality (Phase 7)
- **M8:** Performance optimized (Phase 8)
- **M9:** v2.0.0 released (Phase 9)

---

## 🎯 Common Tasks / よくあるタスク

### Task: Looking up a Japanese term / 日本語用語の検索

**Method 1: Use RUST_MIGRATION_PLAN.md**
- Navigate to Section 3 (English Naming Convention)
- Find the appropriate subsection
- Look up the term in the table

**Method 2: Search in your editor**
- Open RUST_MIGRATION_PLAN.md
- Use Ctrl+F (or Cmd+F) to search for the Japanese term
- Review the translation and suggested English name

---

### Task: Understanding module dependencies / モジュール依存関係の理解

**Use CURRENT_STRUCTURE.md**
- Navigate to Section 10 (Module Dependencies)
- View the dependency tree diagram
- Understand which modules depend on which

**Example:**
```
Main
├── 共通状態 (Shared State)
├── TASC (uses 減速パターン, 出力制御)
└── ATO
    ├── 信号順守 (Signal Compliance)
    ├── 早着防止 (Early Arrival Prevention)
    └── 急動作抑制 (Sudden Motion Suppression)
```

---

### Task: Finding the original C++ code / 元のC++コードの検索

**Location:** `/home/user/bve-autopilot/bve-autopilot/`

**Reference CURRENT_STRUCTURE.md** to find file paths:
- Section 2 contains file paths for all modules
- Example: `制動特性.cpp/h` is at `/home/user/bve-autopilot/bve-autopilot/制動特性.cpp`

**Or use file search:**
```bash
cd /home/user/bve-autopilot
find . -name "*.cpp" -o -name "*.h" | grep [keyword]
```

---

## 🔧 Tips for Success / 成功のためのヒント

### 1. Read Documents in Order / ドキュメントを順番に読む

Follow this reading order for best understanding:
1. RUST_MIGRATION_GUIDE.md (this document)
2. CURRENT_STRUCTURE.md (skim)
3. RUST_MIGRATION_PLAN.md (detailed read)
4. MIGRATION_TODO.md (when starting implementation)

### 2. Bookmark Key Sections / 重要セクションをブックマーク

Key sections to bookmark:
- RUST_MIGRATION_PLAN.md → Section 3 (Naming Convention)
- MIGRATION_TODO.md → Current phase section
- CURRENT_STRUCTURE.md → Section 2 (Module Architecture)

### 3. Keep Documents Open / ドキュメントを開いたまま

While coding, keep these documents open in separate tabs:
- MIGRATION_TODO.md (for current task)
- RUST_MIGRATION_PLAN.md (for naming reference)
- CURRENT_STRUCTURE.md (for C++ reference)

### 4. Update TODO Checkboxes / TODOチェックボックスを更新

As you complete tasks:
- Check off completed items in MIGRATION_TODO.md
- Commit changes: `git add MIGRATION_TODO.md && git commit -m "Complete task X"`
- This helps track progress

### 5. Validate Each Phase / 各フェーズを検証

Before moving to the next phase:
- Complete all tasks in current phase
- Run all tests
- Complete the "Phase X Validation" section
- Commit all code

### 6. Ask Questions / 質問する

If something is unclear:
- Check if it's explained in another document
- Consult algorithm.md for algorithm details
- Review the original C++ code
- Document your decisions in code comments

---

## 📦 Additional Resources / 追加リソース

### Original Project Documentation / 元プロジェクトのドキュメント

- **README.md** - Original project README (Japanese)
  - Project overview
  - Features
  - Usage instructions
  - License information

- **algorithm.md** - Detailed algorithm documentation (Japanese)
  - TASC algorithm explanation
  - ATO algorithm explanation
  - Mathematical formulas
  - Implementation considerations

### External Resources / 外部リソース

Referenced in RUST_MIGRATION_PLAN.md:
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [uom - Units of Measurement](https://docs.rs/uom/)
- [Rust FFI Omnibus](http://jakegoulding.com/rust-ffi-omnibus/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)

### Source Code / ソースコード

- **C++ Source:** `/home/user/bve-autopilot/bve-autopilot/`
  - 66 source files (`.cpp` and `.h`)
  - ~9,357 lines of code

- **C++ Tests:** `/home/user/bve-autopilot/bve-autopilot-test/`
  - 4 test files
  - ~59,682 lines of test code

---

## 📝 Document Maintenance / ドキュメントのメンテナンス

### Updating Documents / ドキュメントの更新

As the migration progresses, these documents may need updates:

1. **MIGRATION_TODO.md** - Most frequently updated
   - Check off completed tasks
   - Add new tasks if discovered
   - Update with actual challenges encountered

2. **RUST_MIGRATION_PLAN.md** - Occasionally updated
   - Adjust timeline if needed
   - Add new naming conventions
   - Update architecture if design changes

3. **CURRENT_STRUCTURE.md** - Rarely updated
   - Update only if C++ codebase changes
   - This is primarily a reference document

4. **RUST_MIGRATION_GUIDE.md** (this document) - Rarely updated
   - Update if documentation structure changes
   - Add new scenarios as needed

### Suggesting Improvements / 改善提案

If you find issues or have suggestions:
- Create an issue in the repository
- Propose changes via pull request
- Document improvements in commit messages

---

## 🎓 Learning Path / 学習パス

### If you're new to Rust / Rustが初めての場合

Before starting the migration:
1. Complete [The Rust Book](https://doc.rust-lang.org/book/)
2. Learn about FFI: [Rust FFI Omnibus](http://jakegoulding.com/rust-ffi-omnibus/)
3. Understand Cargo: [Cargo Book](https://doc.rust-lang.org/cargo/)
4. Learn `uom` crate: [uom documentation](https://docs.rs/uom/)

### If you're new to BVE plugins / BVEプラグインが初めての場合

Before starting the migration:
1. Read the original README.md
2. Study algorithm.md to understand TASC/ATO
3. Review the C++ source code in `bve-autopilot/`
4. Try running the existing C++ plugin in BVE

---

## 🏁 Next Steps / 次のステップ

Now that you understand the documentation structure, you're ready to begin!

ドキュメント構造を理解したので、始める準備が整いました！

### Recommended path / 推奨パス:

1. ✅ You are here: RUST_MIGRATION_GUIDE.md
2. 📗 **Next:** Skim [CURRENT_STRUCTURE.md](CURRENT_STRUCTURE.md)
   - Goal: Understand what the codebase does
   - Time: 20-30 minutes
3. 📕 **Then:** Read [RUST_MIGRATION_PLAN.md](RUST_MIGRATION_PLAN.md)
   - Goal: Understand the migration strategy
   - Focus on Section 3 (Naming Convention)
   - Time: 1-2 hours
4. 📙 **Finally:** Start [MIGRATION_TODO.md](MIGRATION_TODO.md) Phase 1
   - Goal: Begin implementation
   - Follow checklist systematically
   - Time: 2 weeks

### Ready to start coding? / コーディングを始める準備はできましたか？

Jump to **MIGRATION_TODO.md → Phase 1: Foundation** and begin with:
- [ ] 1.1 Project Setup
- [ ] Install Rust toolchain
- [ ] Create Cargo workspace

**Good luck with the migration! / 移行頑張ってください！** 🚀

---

## 📞 Support / サポート

If you need help:
- Review the documentation again
- Check the original C++ implementation
- Consult algorithm.md for algorithm questions
- Create an issue in the repository

---

**Document Version:** 1.0
**Last Updated:** 2025-11-08
**Author:** Migration team
**Status:** Ready for use

---

## Quick Reference Card / クイックリファレンスカード

```
┌─────────────────────────────────────────────────────────────┐
│  Quick Reference: Which Document to Use?                   │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Need to...                          Use...                │
│  ───────────────────────────────────────────────────────   │
│  Understand project structure        RUST_MIGRATION_GUIDE  │
│  Look up C++ module details          CURRENT_STRUCTURE     │
│  Translate Japanese → English        RUST_MIGRATION_PLAN   │
│  See Rust architecture               RUST_MIGRATION_PLAN   │
│  Get implementation tasks            MIGRATION_TODO        │
│  Track progress                      MIGRATION_TODO        │
│  Understand algorithms               algorithm.md          │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│  Phase in Progress: Phase 1 - Foundation                   │
│  Next Milestone: M1 - DLL builds and loads in BVE          │
│  Estimated Completion: Week 20                             │
└─────────────────────────────────────────────────────────────┘
```
