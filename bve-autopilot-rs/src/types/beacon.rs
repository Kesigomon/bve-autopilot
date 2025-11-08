//! Beacon data types
//!
//! This module defines types for handling beacon (地上子) data from the track.

use std::fmt;

/// Beacon type identifier
///
/// Different beacon types provide different information to the train control system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeaconType {
    /// TASC stop position beacon
    TascStop,
    /// Speed limit beacon
    SpeedLimit,
    /// Gradient information beacon
    Gradient,
    /// Signal beacon
    Signal,
    /// ATO timing beacon
    AtoTiming,
    /// ORP pattern beacon
    OrpPattern,
    /// Unknown or custom beacon
    Unknown(i32),
}

impl BeaconType {
    /// Create beacon type from integer identifier
    pub fn from_int(value: i32) -> Self {
        match value {
            // These values will be determined from the actual BVE beacon protocol
            // Placeholder values for now
            0 => BeaconType::TascStop,
            1 => BeaconType::SpeedLimit,
            2 => BeaconType::Gradient,
            3 => BeaconType::Signal,
            4 => BeaconType::AtoTiming,
            5 => BeaconType::OrpPattern,
            _ => BeaconType::Unknown(value),
        }
    }

    /// Convert beacon type to integer identifier
    pub fn to_int(self) -> i32 {
        match self {
            BeaconType::TascStop => 0,
            BeaconType::SpeedLimit => 1,
            BeaconType::Gradient => 2,
            BeaconType::Signal => 3,
            BeaconType::AtoTiming => 4,
            BeaconType::OrpPattern => 5,
            BeaconType::Unknown(value) => value,
        }
    }
}

impl fmt::Display for BeaconType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BeaconType::TascStop => write!(f, "TASC Stop"),
            BeaconType::SpeedLimit => write!(f, "Speed Limit"),
            BeaconType::Gradient => write!(f, "Gradient"),
            BeaconType::Signal => write!(f, "Signal"),
            BeaconType::AtoTiming => write!(f, "ATO Timing"),
            BeaconType::OrpPattern => write!(f, "ORP Pattern"),
            BeaconType::Unknown(value) => write!(f, "Unknown({})", value),
        }
    }
}

/// Beacon data received from the track
///
/// Contains all information from a beacon encounter.
#[derive(Debug, Clone, Copy)]
pub struct BeaconData {
    /// Beacon type identifier
    pub beacon_type: i32,
    /// Signal index (for signal beacons)
    pub signal: i32,
    /// Distance to the beacon target (meters)
    pub distance: f32,
    /// Optional additional data
    pub optional: i32,
}

impl BeaconData {
    /// Create new beacon data
    pub fn new(beacon_type: i32, signal: i32, distance: f32, optional: i32) -> Self {
        Self {
            beacon_type,
            signal,
            distance,
            optional,
        }
    }

    /// Get the beacon type as an enum
    pub fn get_type(&self) -> BeaconType {
        BeaconType::from_int(self.beacon_type)
    }
}

impl Default for BeaconData {
    fn default() -> Self {
        Self {
            beacon_type: 0,
            signal: 0,
            distance: 0.0,
            optional: 0,
        }
    }
}

impl fmt::Display for BeaconData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BeaconData {{ type: {}, signal: {}, distance: {:.1}m, optional: {} }}",
            self.get_type(),
            self.signal,
            self.distance,
            self.optional
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beacon_type_conversion() {
        assert_eq!(BeaconType::from_int(0), BeaconType::TascStop);
        assert_eq!(BeaconType::from_int(3), BeaconType::Signal);
        assert_eq!(BeaconType::from_int(999), BeaconType::Unknown(999));
    }

    #[test]
    fn test_beacon_data() {
        let beacon = BeaconData::new(0, 3, 150.0, 80);
        assert_eq!(beacon.beacon_type, 0);
        assert_eq!(beacon.get_type(), BeaconType::TascStop);
        assert_eq!(beacon.distance, 150.0);
    }
}
