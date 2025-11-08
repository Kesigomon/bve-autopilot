//! Distance section management
//!
//! Port of 区間.cpp/h - Handles distance-based sections along the track.

use crate::types::physical_quantity::Distance;
use uom::si::f64::Length;
use uom::si::length::meter;

/// Represents a section of track between two positions
///
/// This is a port of the 区間 (Section) class from the C++ codebase.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Section {
    /// Start position of the section
    pub start: Distance,
    /// End position of the section
    pub end: Distance,
}

impl Section {
    /// Create a new section
    ///
    /// # Arguments
    /// * `start` - Start position of the section
    /// * `end` - End position of the section
    ///
    /// # Panics
    /// Panics if start > end
    pub fn new(start: Distance, end: Distance) -> Self {
        assert!(
            start <= end,
            "Section start ({}) must be <= end ({})",
            start.get::<meter>(),
            end.get::<meter>()
        );
        Self { start, end }
    }

    /// Create a section with start and end values in meters
    pub fn from_meters(start: f64, end: f64) -> Self {
        Self::new(Length::new::<meter>(start), Length::new::<meter>(end))
    }

    /// Get the length of the section
    pub fn length(&self) -> Distance {
        self.end - self.start
    }

    /// Check if a position is contained within this section
    ///
    /// # Arguments
    /// * `position` - Position to check
    ///
    /// # Returns
    /// `true` if start <= position <= end
    pub fn contains(&self, position: Distance) -> bool {
        self.start <= position && position <= self.end
    }

    /// Check if this section intersects with another section
    ///
    /// # Arguments
    /// * `other` - Another section to check
    ///
    /// # Returns
    /// `true` if the sections overlap
    pub fn intersects(&self, other: &Section) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    /// Get the intersection of this section with another
    ///
    /// # Arguments
    /// * `other` - Another section
    ///
    /// # Returns
    /// `Some(Section)` if the sections intersect, `None` otherwise
    pub fn intersection(&self, other: &Section) -> Option<Section> {
        if !self.intersects(other) {
            return None;
        }

        let start = self.start.max(other.start);
        let end = self.end.min(other.end);

        Some(Section { start, end })
    }

    /// Get the union of this section with another
    ///
    /// # Arguments
    /// * `other` - Another section
    ///
    /// # Returns
    /// A section spanning from the minimum start to the maximum end
    pub fn union(&self, other: &Section) -> Section {
        Section {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    /// Check if a position is before this section
    pub fn is_before(&self, position: Distance) -> bool {
        position < self.start
    }

    /// Check if a position is after this section
    pub fn is_after(&self, position: Distance) -> bool {
        position > self.end
    }

    /// Get the distance from a position to the start of this section
    ///
    /// Returns negative if the position is before the section start.
    pub fn distance_to_start(&self, position: Distance) -> Distance {
        self.start - position
    }

    /// Get the distance from a position to the end of this section
    ///
    /// Returns negative if the position is after the section end.
    pub fn distance_to_end(&self, position: Distance) -> Distance {
        self.end - position
    }
}

impl Default for Section {
    fn default() -> Self {
        Self {
            start: Length::new::<meter>(0.0),
            end: Length::new::<meter>(0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_section_creation() {
        let section = Section::from_meters(100.0, 200.0);
        assert_relative_eq!(section.start.get::<meter>(), 100.0);
        assert_relative_eq!(section.end.get::<meter>(), 200.0);
    }

    #[test]
    #[should_panic]
    fn test_section_invalid() {
        Section::from_meters(200.0, 100.0); // Should panic
    }

    #[test]
    fn test_section_length() {
        let section = Section::from_meters(100.0, 200.0);
        assert_relative_eq!(section.length().get::<meter>(), 100.0);
    }

    #[test]
    fn test_section_contains() {
        let section = Section::from_meters(100.0, 200.0);
        assert!(section.contains(Length::new::<meter>(150.0)));
        assert!(section.contains(Length::new::<meter>(100.0)));
        assert!(section.contains(Length::new::<meter>(200.0)));
        assert!(!section.contains(Length::new::<meter>(50.0)));
        assert!(!section.contains(Length::new::<meter>(250.0)));
    }

    #[test]
    fn test_section_intersects() {
        let section1 = Section::from_meters(100.0, 200.0);
        let section2 = Section::from_meters(150.0, 250.0);
        let section3 = Section::from_meters(300.0, 400.0);

        assert!(section1.intersects(&section2));
        assert!(section2.intersects(&section1));
        assert!(!section1.intersects(&section3));
    }

    #[test]
    fn test_section_intersection() {
        let section1 = Section::from_meters(100.0, 200.0);
        let section2 = Section::from_meters(150.0, 250.0);
        let section3 = Section::from_meters(300.0, 400.0);

        let intersection = section1.intersection(&section2).unwrap();
        assert_relative_eq!(intersection.start.get::<meter>(), 150.0);
        assert_relative_eq!(intersection.end.get::<meter>(), 200.0);

        assert!(section1.intersection(&section3).is_none());
    }

    #[test]
    fn test_section_union() {
        let section1 = Section::from_meters(100.0, 200.0);
        let section2 = Section::from_meters(150.0, 250.0);

        let union = section1.union(&section2);
        assert_relative_eq!(union.start.get::<meter>(), 100.0);
        assert_relative_eq!(union.end.get::<meter>(), 250.0);
    }

    #[test]
    fn test_section_is_before_after() {
        let section = Section::from_meters(100.0, 200.0);

        assert!(section.is_before(Length::new::<meter>(50.0)));
        assert!(!section.is_before(Length::new::<meter>(150.0)));
        assert!(section.is_after(Length::new::<meter>(250.0)));
        assert!(!section.is_after(Length::new::<meter>(150.0)));
    }

    #[test]
    fn test_section_distance_to() {
        let section = Section::from_meters(100.0, 200.0);

        assert_relative_eq!(
            section
                .distance_to_start(Length::new::<meter>(50.0))
                .get::<meter>(),
            50.0
        );
        assert_relative_eq!(
            section
                .distance_to_end(Length::new::<meter>(150.0))
                .get::<meter>(),
            50.0
        );
    }
}
