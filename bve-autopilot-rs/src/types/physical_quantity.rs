//! Type-safe physical quantities using the `uom` crate
//!
//! This module provides type aliases and helper functions for working with
//! physical quantities in a type-safe manner.

use uom::si::acceleration::meter_per_second_squared;
use uom::si::f64::*;
use uom::si::length::{centimeter, meter};
use uom::si::time::second;
use uom::si::velocity::{kilometer_per_hour, meter_per_second};

// Type aliases for convenience
/// Distance in meters (m)
pub type Distance = Length;

/// Velocity in meters per second (m/s)
pub type Velocity = uom::si::f64::Velocity;

/// Acceleration in meters per second squared (m/s²)
pub type Acceleration = uom::si::f64::Acceleration;

/// Time duration in seconds
pub type Duration = Time;

/// Extension trait for Distance construction
pub trait DistanceExt {
    /// Create a Distance from meters
    fn from_meters(value: f64) -> Distance;

    /// Create a Distance from centimeters
    fn from_centimeters(value: f64) -> Distance;
}

impl DistanceExt for Distance {
    fn from_meters(value: f64) -> Distance {
        Length::new::<meter>(value)
    }

    fn from_centimeters(value: f64) -> Distance {
        Length::new::<centimeter>(value)
    }
}

/// Extension trait for Velocity construction
pub trait VelocityExt {
    /// Create a Velocity from meters per second
    fn from_mps(value: f64) -> Velocity;

    /// Create a Velocity from kilometers per hour
    fn from_kmph(value: f64) -> Velocity;
}

impl VelocityExt for Velocity {
    fn from_mps(value: f64) -> Velocity {
        uom::si::f64::Velocity::new::<meter_per_second>(value)
    }

    fn from_kmph(value: f64) -> Velocity {
        uom::si::f64::Velocity::new::<kilometer_per_hour>(value)
    }
}

/// Extension trait for Acceleration construction
pub trait AccelerationExt {
    /// Create an Acceleration from meters per second squared
    fn from_mps2(value: f64) -> Acceleration;
}

impl AccelerationExt for Acceleration {
    fn from_mps2(value: f64) -> Acceleration {
        uom::si::f64::Acceleration::new::<meter_per_second_squared>(value)
    }
}

/// Extension trait for Duration construction
pub trait DurationExt {
    /// Create a Duration from seconds
    fn from_seconds(value: f64) -> Duration;
}

impl DurationExt for Duration {
    fn from_seconds(value: f64) -> Duration {
        Time::new::<second>(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_distance_conversion() {
        let d1 = Distance::from_meters(100.0);
        let d2 = Distance::from_centimeters(10000.0);
        assert_relative_eq!(d1.get::<meter>(), d2.get::<meter>(), epsilon = 1e-10);
    }

    #[test]
    fn test_velocity_conversion() {
        let v1 = Velocity::from_mps(27.778);
        let v2 = Velocity::from_kmph(100.0);
        assert_relative_eq!(
            v1.get::<meter_per_second>(),
            v2.get::<meter_per_second>(),
            epsilon = 0.001
        );
    }

    #[test]
    fn test_acceleration() {
        let a = Acceleration::from_mps2(1.5);
        assert_relative_eq!(a.get::<meter_per_second_squared>(), 1.5, epsilon = 1e-10);
    }

    #[test]
    fn test_duration() {
        let t = Duration::from_seconds(10.0);
        assert_relative_eq!(t.get::<second>(), 10.0, epsilon = 1e-10);
    }
}
