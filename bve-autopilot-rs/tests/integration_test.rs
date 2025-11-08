//! Integration tests for BVE Autopilot
//!
//! These tests will be expanded as more functionality is implemented.

use bve_autopilot::types::*;
use bve_autopilot::utils::*;

#[test]
fn test_control_command_creation() {
    let cmd = ControlCommand::neutral();
    assert_eq!(cmd.power, PowerNotch::N);
    assert_eq!(cmd.brake, BrakeNotch::Released);
}

#[test]
fn test_section_basic_operations() {
    let section = Section::from_meters(0.0, 100.0);
    use uom::si::f64::Length;
    use uom::si::length::meter;

    assert!(section.contains(Length::new::<meter>(50.0)));
    assert!(!section.contains(Length::new::<meter>(150.0)));
}

#[test]
fn test_observable_tracking() {
    let mut value = Observable::new(0);

    assert!(!value.has_changed());

    value.set(10);
    assert!(value.has_changed());
    assert_eq!(*value.get(), 10);

    value.clear_change();
    assert!(!value.has_changed());
}
