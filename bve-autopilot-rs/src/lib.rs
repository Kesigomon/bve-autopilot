//! # BVE Autopilot - Rust Implementation
//!
//! This is a Rust implementation of the BVE Autopilot plugin, which provides
//! TASC (Target Station Stopping Control) and ATO (Automatic Train Operation)
//! functionality for BVE Trainsim 5/6.
//!
//! ## Features
//!
//! - **TASC**: Precise automatic stopping at designated station positions
//! - **ATO**: Automatic speed control with signal compliance
//! - **Type Safety**: Compile-time guarantees for physical quantities
//! - **Memory Safety**: Rust's ownership system prevents memory bugs
//!
//! ## Module Organization
//!
//! - `ffi`: Foreign Function Interface for BVE plugin API
//! - `types`: Type-safe physical quantities and control commands
//! - `utils`: Utility modules (section, observable values)
//!
//! ## License
//!
//! LGPL 2.1 - Copyright © 2019-2025 Watanabe, Yuki

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod ffi;
pub mod types;
pub mod utils;

// Re-export commonly used types
pub use types::{
    control_command::{BrakeNotch, ControlCommand, PowerNotch, Reverser},
    physical_quantity::{Acceleration, Distance, Velocity},
};
