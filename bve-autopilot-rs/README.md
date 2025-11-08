# BVE Autopilot - Rust Implementation

This is a Rust implementation of the BVE Autopilot plugin (Phase 1: Foundation).

## Phase 1 Status

✅ **Completed Components:**

- [x] Cargo workspace setup
- [x] Type system (physical quantities, control commands, beacons)
- [x] FFI layer (BVE plugin API)
- [x] Utility modules (Section, Observable)
- [x] Testing infrastructure
- [x] Benchmark infrastructure

## Project Structure

```
bve-autopilot-rs/
├── src/
│   ├── lib.rs              # Library root
│   ├── ffi.rs              # FFI exports for BVE plugin API
│   ├── types/
│   │   ├── mod.rs
│   │   ├── physical_quantity.rs    # Type-safe physical quantities (using uom)
│   │   ├── control_command.rs      # Notch command types
│   │   └── beacon.rs               # Beacon data types
│   └── utils/
│       ├── mod.rs
│       ├── section.rs              # Distance section management (区間)
│       └── observable.rs           # Observable values (live.h)
├── tests/
│   └── integration_test.rs         # Integration tests
└── benches/
    └── control_benchmarks.rs       # Performance benchmarks
```

## Building

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

# Check code with clippy
cargo clippy

# Format code
cargo fmt
```

## Dependencies

- **uom** (v0.36): Units of measurement for type-safe physical quantities
- **serde** (v1.0): Serialization framework
- **serde_ini** (v0.2): INI file parsing for configuration
- **windows** (v0.52): Windows API bindings

## Testing

The project includes comprehensive unit tests and integration tests:

```bash
cargo test --all
```

## Next Steps (Phase 2)

The next phase will implement state management modules:

- [ ] Shared state (`共通状態` → `shared_state`)
- [ ] Motion state (`運動状態` → `motion_state`)
- [ ] Operating state (`稼働状態` → `operating_state`)
- [ ] Configuration (`環境設定` → `configuration`)

## License

LGPL 2.1 - Copyright © 2019-2025 Watanabe, Yuki

See [LICENSE](../LICENSE) for details.

## Documentation

- [Migration Plan](../docs/RUST_MIGRATION_PLAN.md)
- [Current C++ Structure](../docs/CURRENT_STRUCTURE.md)
- [Migration TODO](../docs/MIGRATION_TODO.md)
- [Algorithm Documentation](../algorithm.md)
