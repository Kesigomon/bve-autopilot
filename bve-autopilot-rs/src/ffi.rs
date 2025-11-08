//! Foreign Function Interface (FFI) for BVE Plugin API
//!
//! This module provides the C-compatible interface required by BVE Trainsim.
//! All functions are marked with `#[no_mangle]` and use `extern "system"` calling convention.

use std::os::raw::{c_float, c_int};
use std::panic;

/// FFI-safe vehicle specification structure
///
/// Matches the BVE `ATS_VEHICLESPEC` structure
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AtsVehicleSpec {
    /// Number of brake notches
    pub brake_notches: c_int,
    /// Number of power notches
    pub power_notches: c_int,
    /// ATS check position (notch number)
    pub ats_notch: c_int,
    /// B67 notch position
    pub b67_notch: c_int,
    /// Number of cars in the train
    pub cars: c_int,
}

/// FFI-safe vehicle state structure
///
/// Matches the BVE `ATS_VEHICLESTATE` structure
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AtsVehicleState {
    /// Current location (meters)
    pub location: c_float,
    /// Current speed (km/h)
    pub speed: c_float,
    /// Current time (milliseconds)
    pub time: c_int,
    /// Brake cylinder pressure (kPa)
    pub bc_pressure: c_float,
    /// Main reservoir pressure (kPa)
    pub mr_pressure: c_float,
    /// Equalizing reservoir pressure (kPa)
    pub er_pressure: c_float,
    /// Brake pipe pressure (kPa)
    pub bp_pressure: c_float,
    /// Straight air pipe pressure (kPa)
    pub sap_pressure: c_float,
    /// Electric current (A)
    pub current: c_float,
}

/// FFI-safe handles structure
///
/// Matches the BVE `ATS_HANDLES` structure.
/// This is what the plugin returns to BVE to control the train.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct AtsHandles {
    /// Brake notch command
    pub brake: c_int,
    /// Power notch command
    pub power: c_int,
    /// Reverser position
    pub reverser: c_int,
    /// Constant speed mode
    pub constant_speed: c_int,
}

/// FFI-safe beacon data structure
///
/// Matches the BVE `ATS_BEACONDATA` structure
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AtsBeaconData {
    /// Beacon type number
    pub beacon_type: c_int,
    /// Signal index
    pub signal: c_int,
    /// Distance to beacon target (meters)
    pub distance: c_float,
    /// Optional data
    pub optional: c_int,
}

// Plugin state (will be properly implemented in later phases)
static mut PLUGIN_INITIALIZED: bool = false;

/// Helper macro to catch panics at FFI boundary
macro_rules! ffi_guard {
    ($body:expr) => {
        match panic::catch_unwind(panic::AssertUnwindSafe(|| $body)) {
            Ok(result) => result,
            Err(_) => {
                // Log panic (in the future, log to file)
                // For now, just return safe defaults
                Default::default()
            }
        }
    };
}

/// Load function - Called when plugin is loaded
///
/// # Safety
/// This is called by BVE with stdcall convention.
#[no_mangle]
pub extern "system" fn Load() {
    ffi_guard!({
        unsafe {
            PLUGIN_INITIALIZED = true;
        }
        // TODO: Initialize logging
        // TODO: Load configuration
    });
}

/// Dispose function - Called when plugin is unloaded
///
/// # Safety
/// This is called by BVE with stdcall convention.
#[no_mangle]
pub extern "system" fn Dispose() {
    ffi_guard!({
        unsafe {
            PLUGIN_INITIALIZED = false;
        }
        // TODO: Cleanup resources
    });
}

/// Get plugin version
///
/// Returns version in format 0xMMmmpppp where:
/// - MM = major version
/// - mm = minor version
/// - pppp = patch version
///
/// # Safety
/// This is called by BVE with stdcall convention.
#[no_mangle]
pub extern "system" fn GetPluginVersion() -> c_int {
    0x00020000 // Version 2.0.0
}

/// Set vehicle specification
///
/// Called by BVE to provide vehicle characteristics.
///
/// # Safety
/// This is called by BVE with stdcall convention.
#[no_mangle]
pub extern "system" fn SetVehicleSpec(spec: AtsVehicleSpec) {
    ffi_guard!({
        // TODO: Store vehicle specification
        let _ = spec;
    });
}

/// Initialize train state
///
/// Called by BVE when starting or loading a save.
///
/// # Safety
/// This is called by BVE with stdcall convention.
#[no_mangle]
pub extern "system" fn Initialize(_brake: c_int) {
    ffi_guard!({
        // TODO: Initialize train state
    });
}

/// Main update loop
///
/// Called by BVE every frame to update the plugin state.
/// Returns the control handles to apply.
///
/// # Safety
/// This is called by BVE with stdcall convention.
/// The panel and sound arrays must be valid pointers with sufficient capacity.
#[no_mangle]
pub extern "system" fn Elapse(
    _state: AtsVehicleState,
    _panel: *mut c_int,
    _sound: *mut c_int,
) -> AtsHandles {
    ffi_guard!({
        // TODO: Implement main control logic
        AtsHandles::default()
    })
}

/// Set power notch
///
/// Called by BVE when the driver changes the power notch.
///
/// # Safety
/// This is called by BVE with stdcall convention.
#[no_mangle]
pub extern "system" fn SetPower(_notch: c_int) {
    ffi_guard!({
        // TODO: Handle power notch change
    });
}

/// Set brake notch
///
/// Called by BVE when the driver changes the brake notch.
///
/// # Safety
/// This is called by BVE with stdcall convention.
#[no_mangle]
pub extern "system" fn SetBrake(_notch: c_int) {
    ffi_guard!({
        // TODO: Handle brake notch change
    });
}

/// Set reverser
///
/// Called by BVE when the driver changes the reverser position.
///
/// # Safety
/// This is called by BVE with stdcall convention.
#[no_mangle]
pub extern "system" fn SetReverser(_pos: c_int) {
    ffi_guard!({
        // TODO: Handle reverser change
    });
}

/// Key down event
///
/// Called by BVE when a key is pressed.
///
/// # Safety
/// This is called by BVE with stdcall convention.
#[no_mangle]
pub extern "system" fn KeyDown(_key: c_int) {
    ffi_guard!({
        // TODO: Handle key press
    });
}

/// Key up event
///
/// Called by BVE when a key is released.
///
/// # Safety
/// This is called by BVE with stdcall convention.
#[no_mangle]
pub extern "system" fn KeyUp(_key: c_int) {
    ffi_guard!({
        // TODO: Handle key release
    });
}

/// Horn blow event
///
/// Called by BVE when the horn is used.
///
/// # Safety
/// This is called by BVE with stdcall convention.
#[no_mangle]
pub extern "system" fn HornBlow(_horn_type: c_int) {
    ffi_guard!({
        // TODO: Handle horn event
    });
}

/// Door open event
///
/// Called by BVE when doors are opened.
///
/// # Safety
/// This is called by BVE with stdcall convention.
#[no_mangle]
pub extern "system" fn DoorOpen() {
    ffi_guard!({
        // TODO: Handle door open
    });
}

/// Door close event
///
/// Called by BVE when doors are closed.
///
/// # Safety
/// This is called by BVE with stdcall convention.
#[no_mangle]
pub extern "system" fn DoorClose() {
    ffi_guard!({
        // TODO: Handle door close
    });
}

/// Set signal
///
/// Called by BVE when the signal aspect changes.
///
/// # Safety
/// This is called by BVE with stdcall convention.
#[no_mangle]
pub extern "system" fn SetSignal(_signal: c_int) {
    ffi_guard!({
        // TODO: Handle signal change
    });
}

/// Set beacon data
///
/// Called by BVE when the train passes over a beacon.
///
/// # Safety
/// This is called by BVE with stdcall convention.
#[no_mangle]
pub extern "system" fn SetBeaconData(_beacon: AtsBeaconData) {
    ffi_guard!({
        // TODO: Handle beacon data
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ats_handles_default() {
        let handles = AtsHandles::default();
        assert_eq!(handles.brake, 0);
        assert_eq!(handles.power, 0);
        assert_eq!(handles.reverser, 0);
        assert_eq!(handles.constant_speed, 0);
    }

    #[test]
    fn test_plugin_version() {
        assert_eq!(GetPluginVersion(), 0x00020000);
    }
}
