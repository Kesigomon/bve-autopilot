# Instructions for Claude and AI Assistants

This document provides guidance for Claude Code and other AI assistants when working on the bve-autopilot Rust migration project.

## 📍 Documentation Location

All Rust migration documentation is located in the **`docs/`** directory:

```
bve-autopilot/
├── CLAUDE.md                          ← You are here
├── README.md
├── algorithm.md
├── LICENSE
│
└── docs/                              ← Migration documentation
    ├── RUST_MIGRATION_GUIDE.md        ← Start here for overview
    ├── CURRENT_STRUCTURE.md           ← C++ codebase reference
    ├── RUST_MIGRATION_PLAN.md         ← Migration strategy & naming
    └── MIGRATION_TODO.md              ← Implementation checklist
```

## 🎯 When Implementing Migration Tasks

### 1. Always Start Here

Before implementing any Rust migration task, you MUST:

1. **Read `docs/RUST_MIGRATION_GUIDE.md`** first
   - This is the entry point for all migration work
   - It explains what each document contains and when to use it

2. **Understand the current structure** via `docs/CURRENT_STRUCTURE.md`
   - Know what the C++ code does before porting
   - Understand module dependencies

3. **Follow the naming conventions** in `docs/RUST_MIGRATION_PLAN.md`
   - All Japanese names must be translated to English
   - Use the translation dictionary in Section 3
   - Follow Rust naming conventions (snake_case, PascalCase)

4. **Execute tasks systematically** using `docs/MIGRATION_TODO.md`
   - Follow the phase-by-phase checklist
   - Mark tasks as completed as you go
   - Don't skip validation steps

### 2. Reference Documents During Implementation

While coding, keep these documents open and reference them:

| Task | Primary Document | Secondary References |
|------|-----------------|---------------------|
| Understanding existing code | `docs/CURRENT_STRUCTURE.md` | Original C++ files, `algorithm.md` |
| Translating names | `docs/RUST_MIGRATION_PLAN.md` (Section 3) | `docs/CURRENT_STRUCTURE.md` |
| Implementing a module | `docs/MIGRATION_TODO.md` (current phase) | `docs/RUST_MIGRATION_PLAN.md` |
| Understanding algorithms | `algorithm.md` | `docs/CURRENT_STRUCTURE.md` (Section 5) |
| Checking progress | `docs/MIGRATION_TODO.md` (Progress Tracking) | - |

### 3. Translation Rules (CRITICAL)

When porting C++ code to Rust, you MUST translate all Japanese identifiers to English:

**❌ DO NOT:**
```rust
// Bad - keeping Japanese names
struct 制動特性 {
    現在速度: f64,
    目標減速度: f64,
}
```

**✅ DO:**
```rust
// Good - using English names from translation dictionary
struct BrakeCharacteristics {
    current_velocity: Velocity,
    target_deceleration: Acceleration,
}
```

**Translation process:**
1. Look up the Japanese term in `docs/RUST_MIGRATION_PLAN.md` Section 3
2. If not found, create a descriptive English name following Rust conventions
3. Document your translation choice in code comments
4. Consider adding it to the translation dictionary

### 4. Implementation Workflow

For each module you implement:

1. **Locate the module in TODO**
   - Find the current phase in `docs/MIGRATION_TODO.md`
   - Identify the specific module task

2. **Understand the C++ implementation**
   - Read `docs/CURRENT_STRUCTURE.md` to understand what it does
   - Review the original C++ source files
   - Check `algorithm.md` if it's an algorithm module

3. **Plan the Rust implementation**
   - Check `docs/RUST_MIGRATION_PLAN.md` for the target location
   - Look up all Japanese names in the translation dictionary
   - Identify dependencies on other modules

4. **Implement the module**
   - Create the Rust file in the correct location
   - Translate all identifiers to English
   - Port the logic maintaining the same behavior
   - Add unit tests

5. **Validate the implementation**
   - Run `cargo test` for the module
   - Compare behavior with C++ version if possible
   - Check off the task in `docs/MIGRATION_TODO.md`

6. **Update documentation**
   - Add doc comments to all public items
   - Document any deviations from C++ version
   - Update `docs/MIGRATION_TODO.md` with progress

## 📚 Quick Reference: What Each Document Contains

### `docs/RUST_MIGRATION_GUIDE.md`
- **Purpose:** Navigation hub for all documentation
- **Use when:** Starting the project, finding information
- **Key sections:** Document overview, usage scenarios, quick start

### `docs/CURRENT_STRUCTURE.md`
- **Purpose:** Complete documentation of C++ codebase
- **Use when:** Understanding what existing code does
- **Key sections:** Module architecture (Section 2), Dependencies (Section 10), Algorithm (Section 5)

### `docs/RUST_MIGRATION_PLAN.md`
- **Purpose:** Migration strategy and naming conventions
- **Use when:** Planning implementation, translating names
- **Key sections:** **Section 3 (Naming Convention)** ← MOST IMPORTANT, Module mapping (Section 4), Type system (Section 5)

### `docs/MIGRATION_TODO.md`
- **Purpose:** Detailed task checklist for all 9 phases
- **Use when:** Implementing tasks, tracking progress
- **Key sections:** Current phase tasks, Phase validation, Progress tracking

## 🔍 Common Scenarios

### Scenario: User asks to "implement Phase X"

1. Open `docs/MIGRATION_TODO.md`
2. Navigate to Phase X section
3. Read the phase objective and overview
4. For each subsection (X.1, X.2, etc.):
   - Review the task list
   - Reference `docs/RUST_MIGRATION_PLAN.md` for naming and architecture
   - Reference `docs/CURRENT_STRUCTURE.md` for C++ implementation details
   - Implement the module
   - Mark tasks as complete

### Scenario: User asks to "port module X"

1. Find module X in `docs/CURRENT_STRUCTURE.md`
   - Understand its purpose
   - Note its dependencies
2. Look up the English name in `docs/RUST_MIGRATION_PLAN.md` Section 3
3. Find the target location in `docs/RUST_MIGRATION_PLAN.md` Section 4
4. Find the implementation tasks in `docs/MIGRATION_TODO.md`
5. Implement following the workflow above

### Scenario: "What does 制動特性 mean?"

1. Open `docs/RUST_MIGRATION_PLAN.md`
2. Navigate to Section 3 (English Naming Convention)
3. Look in subsection "Vehicle & Environment"
4. Find: `制動特性` → `Brake Characteristics` → `brake_characteristics`

### Scenario: User asks about progress

1. Open `docs/MIGRATION_TODO.md`
2. Scroll to "Progress Tracking" section at the bottom
3. Review "Overall Progress" checklist
4. Check "Milestone Checklist"
5. Report completed phases and current phase

## ⚠️ Important Rules

### DO:
✅ Always translate Japanese names to English
✅ Follow the naming conventions in `docs/RUST_MIGRATION_PLAN.md`
✅ Reference the documentation before implementing
✅ Mark tasks as complete in `docs/MIGRATION_TODO.md`
✅ Add doc comments to all public items
✅ Write tests for all modules
✅ Validate each phase before moving to the next

### DO NOT:
❌ Keep Japanese identifiers in Rust code
❌ Skip reading the documentation
❌ Implement without checking the translation dictionary
❌ Move to next phase without completing validation
❌ Change the algorithm without documenting why
❌ Skip writing tests

## 🎓 Before You Start Coding

Make sure you have:
- [ ] Read `docs/RUST_MIGRATION_GUIDE.md` completely
- [ ] Skimmed `docs/CURRENT_STRUCTURE.md` to understand the project
- [ ] Read `docs/RUST_MIGRATION_PLAN.md` Section 3 (Naming Convention)
- [ ] Located your current task in `docs/MIGRATION_TODO.md`
- [ ] Understood the C++ implementation you're porting

## 📝 Updating Progress

As you work:
1. Check off completed tasks in `docs/MIGRATION_TODO.md`
2. Commit changes with clear messages
3. Update the "Progress Tracking" section when completing phases

## 🆘 If You're Stuck

1. Re-read the relevant documentation section
2. Check `algorithm.md` for algorithm details
3. Review the original C++ implementation
4. Look for similar examples in completed modules
5. Check if the issue is addressed in `docs/MIGRATION_TODO.md` notes

## 💡 Pro Tips

1. **Use the Quick Reference Card** in `docs/RUST_MIGRATION_GUIDE.md`
2. **Bookmark Section 3** of `docs/RUST_MIGRATION_PLAN.md` (naming conventions)
3. **Keep `docs/MIGRATION_TODO.md` open** while coding
4. **Search documents** with Ctrl+F / Cmd+F for quick lookups
5. **Follow the phase order** - don't jump ahead

## 🚀 Ready to Start?

1. Open `docs/RUST_MIGRATION_GUIDE.md`
2. Follow the "Quick Start Guide" section
3. Begin with Phase 1 in `docs/MIGRATION_TODO.md`

---

**Remember:** The documentation in `docs/` is your source of truth. Always reference it when implementing the Rust migration.

**Last Updated:** 2025-11-08
**For Questions:** See `docs/RUST_MIGRATION_GUIDE.md` → Support section
