//! Control command types for BVE train control
//!
//! This module defines type-safe representations of train control commands.

use std::fmt;

/// Brake notch command
///
/// Represents the brake notch position, from released to emergency brake.
/// Also supports extended brake (15-31 levels) for precise brake control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BrakeNotch {
    /// Brake released
    Released,
    /// Brake notch 1
    B1,
    /// Brake notch 2
    B2,
    /// Brake notch 3
    B3,
    /// Brake notch 4
    B4,
    /// Brake notch 5
    B5,
    /// Brake notch 6
    B6,
    /// Brake notch 7
    B7,
    /// Brake notch 8
    B8,
    /// Emergency brake
    Emergency,
    /// Extended brake (15-31 levels for fine control)
    Extended(u8),
}

impl BrakeNotch {
    /// Convert brake notch to integer value
    pub fn to_int(self) -> i32 {
        match self {
            BrakeNotch::Released => 0,
            BrakeNotch::B1 => 1,
            BrakeNotch::B2 => 2,
            BrakeNotch::B3 => 3,
            BrakeNotch::B4 => 4,
            BrakeNotch::B5 => 5,
            BrakeNotch::B6 => 6,
            BrakeNotch::B7 => 7,
            BrakeNotch::B8 => 8,
            BrakeNotch::Emergency => 9,
            BrakeNotch::Extended(level) => level as i32,
        }
    }

    /// Create brake notch from integer value
    pub fn from_int(value: i32) -> Self {
        match value {
            0 => BrakeNotch::Released,
            1 => BrakeNotch::B1,
            2 => BrakeNotch::B2,
            3 => BrakeNotch::B3,
            4 => BrakeNotch::B4,
            5 => BrakeNotch::B5,
            6 => BrakeNotch::B6,
            7 => BrakeNotch::B7,
            8 => BrakeNotch::B8,
            9 => BrakeNotch::Emergency,
            15..=31 => BrakeNotch::Extended(value as u8),
            _ => BrakeNotch::Emergency, // Safe fallback
        }
    }
}

impl fmt::Display for BrakeNotch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BrakeNotch::Released => write!(f, "Released"),
            BrakeNotch::B1 => write!(f, "B1"),
            BrakeNotch::B2 => write!(f, "B2"),
            BrakeNotch::B3 => write!(f, "B3"),
            BrakeNotch::B4 => write!(f, "B4"),
            BrakeNotch::B5 => write!(f, "B5"),
            BrakeNotch::B6 => write!(f, "B6"),
            BrakeNotch::B7 => write!(f, "B7"),
            BrakeNotch::B8 => write!(f, "B8"),
            BrakeNotch::Emergency => write!(f, "Emergency"),
            BrakeNotch::Extended(level) => write!(f, "Extended({})", level),
        }
    }
}

/// Power notch command
///
/// Represents the power notch position, from neutral to P5.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PowerNotch {
    /// Neutral (no power)
    N,
    /// Power notch 1
    P1,
    /// Power notch 2
    P2,
    /// Power notch 3
    P3,
    /// Power notch 4
    P4,
    /// Power notch 5
    P5,
}

impl PowerNotch {
    /// Convert power notch to integer value
    pub fn to_int(self) -> i32 {
        match self {
            PowerNotch::N => 0,
            PowerNotch::P1 => 1,
            PowerNotch::P2 => 2,
            PowerNotch::P3 => 3,
            PowerNotch::P4 => 4,
            PowerNotch::P5 => 5,
        }
    }

    /// Create power notch from integer value
    pub fn from_int(value: i32) -> Self {
        match value {
            0 => PowerNotch::N,
            1 => PowerNotch::P1,
            2 => PowerNotch::P2,
            3 => PowerNotch::P3,
            4 => PowerNotch::P4,
            5 => PowerNotch::P5,
            _ => PowerNotch::N, // Safe fallback
        }
    }
}

impl fmt::Display for PowerNotch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PowerNotch::N => write!(f, "N"),
            PowerNotch::P1 => write!(f, "P1"),
            PowerNotch::P2 => write!(f, "P2"),
            PowerNotch::P3 => write!(f, "P3"),
            PowerNotch::P4 => write!(f, "P4"),
            PowerNotch::P5 => write!(f, "P5"),
        }
    }
}

/// Reverser position
///
/// Represents the direction control.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reverser {
    /// Forward direction
    Forward,
    /// Neutral (no direction)
    Neutral,
    /// Backward direction
    Backward,
}

impl Reverser {
    /// Convert reverser to integer value
    pub fn to_int(self) -> i32 {
        match self {
            Reverser::Backward => -1,
            Reverser::Neutral => 0,
            Reverser::Forward => 1,
        }
    }

    /// Create reverser from integer value
    pub fn from_int(value: i32) -> Self {
        match value {
            -1 => Reverser::Backward,
            0 => Reverser::Neutral,
            1 => Reverser::Forward,
            _ => Reverser::Neutral, // Safe fallback
        }
    }
}

impl fmt::Display for Reverser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Reverser::Forward => write!(f, "Forward"),
            Reverser::Neutral => write!(f, "Neutral"),
            Reverser::Backward => write!(f, "Backward"),
        }
    }
}

/// Complete control command
///
/// Represents a complete set of train control commands.
#[derive(Debug, Clone, Copy)]
pub struct ControlCommand {
    /// Power notch position
    pub power: PowerNotch,
    /// Brake notch position
    pub brake: BrakeNotch,
    /// Reverser position
    pub reverser: Reverser,
}

impl ControlCommand {
    /// Create a new control command
    pub fn new(power: PowerNotch, brake: BrakeNotch, reverser: Reverser) -> Self {
        Self {
            power,
            brake,
            reverser,
        }
    }

    /// Create a neutral control command (all controls at neutral/released)
    pub fn neutral() -> Self {
        Self {
            power: PowerNotch::N,
            brake: BrakeNotch::Released,
            reverser: Reverser::Neutral,
        }
    }

    /// Create an emergency brake command
    pub fn emergency() -> Self {
        Self {
            power: PowerNotch::N,
            brake: BrakeNotch::Emergency,
            reverser: Reverser::Forward,
        }
    }
}

impl Default for ControlCommand {
    fn default() -> Self {
        Self::neutral()
    }
}

impl fmt::Display for ControlCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ControlCommand {{ power: {}, brake: {}, reverser: {} }}",
            self.power, self.brake, self.reverser
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brake_notch_conversion() {
        assert_eq!(BrakeNotch::from_int(0), BrakeNotch::Released);
        assert_eq!(BrakeNotch::from_int(5), BrakeNotch::B5);
        assert_eq!(BrakeNotch::from_int(9), BrakeNotch::Emergency);
        assert_eq!(BrakeNotch::from_int(20), BrakeNotch::Extended(20));
    }

    #[test]
    fn test_brake_notch_to_int() {
        assert_eq!(BrakeNotch::Released.to_int(), 0);
        assert_eq!(BrakeNotch::B8.to_int(), 8);
        assert_eq!(BrakeNotch::Emergency.to_int(), 9);
        assert_eq!(BrakeNotch::Extended(25).to_int(), 25);
    }

    #[test]
    fn test_power_notch_conversion() {
        assert_eq!(PowerNotch::from_int(0), PowerNotch::N);
        assert_eq!(PowerNotch::from_int(3), PowerNotch::P3);
        assert_eq!(PowerNotch::from_int(5), PowerNotch::P5);
    }

    #[test]
    fn test_reverser_conversion() {
        assert_eq!(Reverser::from_int(-1), Reverser::Backward);
        assert_eq!(Reverser::from_int(0), Reverser::Neutral);
        assert_eq!(Reverser::from_int(1), Reverser::Forward);
    }

    #[test]
    fn test_control_command() {
        let cmd = ControlCommand::neutral();
        assert_eq!(cmd.power, PowerNotch::N);
        assert_eq!(cmd.brake, BrakeNotch::Released);

        let emergency = ControlCommand::emergency();
        assert_eq!(emergency.brake, BrakeNotch::Emergency);
    }
}
