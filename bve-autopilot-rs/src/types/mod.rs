//! Type system for BVE Autopilot
//!
//! This module provides type-safe representations of:
//! - Physical quantities (distance, velocity, acceleration)
//! - Control commands (power, brake, reverser)
//! - Beacon data

pub mod beacon;
pub mod control_command;
pub mod physical_quantity;

pub use beacon::{BeaconData, BeaconType};
pub use control_command::{BrakeNotch, ControlCommand, PowerNotch, Reverser};
pub use physical_quantity::{Acceleration, Distance, Velocity};
