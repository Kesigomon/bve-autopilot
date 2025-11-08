//! Observable values with change detection
//!
//! Port of live.h - Provides a wrapper for values that can track changes.

use std::fmt;

/// An observable value that tracks changes
///
/// This is a port of the `live` template from the C++ codebase.
/// It wraps a value and provides change detection functionality.
///
/// # Type Parameters
/// * `T` - The type of value being observed
///
/// # Example
/// ```
/// use bve_autopilot::utils::Observable;
///
/// let mut speed = Observable::new(0.0);
/// assert!(!speed.has_changed());
///
/// speed.set(80.0);
/// assert!(speed.has_changed());
/// assert_eq!(*speed.get(), 80.0);
///
/// speed.clear_change();
/// assert!(!speed.has_changed());
/// ```
#[derive(Debug, Clone)]
pub struct Observable<T> {
    /// Current value
    value: T,
    /// Flag indicating if the value has changed
    changed: bool,
}

impl<T> Observable<T> {
    /// Create a new observable with an initial value
    ///
    /// # Arguments
    /// * `value` - Initial value
    pub fn new(value: T) -> Self {
        Self {
            value,
            changed: false,
        }
    }

    /// Get a reference to the current value
    pub fn get(&self) -> &T {
        &self.value
    }

    /// Get a mutable reference to the current value
    ///
    /// Note: This marks the value as changed.
    pub fn get_mut(&mut self) -> &mut T {
        self.changed = true;
        &mut self.value
    }

    /// Set a new value
    ///
    /// This marks the observable as changed.
    ///
    /// # Arguments
    /// * `value` - New value to set
    pub fn set(&mut self, value: T) {
        self.value = value;
        self.changed = true;
    }

    /// Check if the value has changed since the last clear
    pub fn has_changed(&self) -> bool {
        self.changed
    }

    /// Clear the changed flag
    pub fn clear_change(&mut self) {
        self.changed = false;
    }

    /// Consume the observable and return the inner value
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T: PartialEq> Observable<T> {
    /// Set a new value only if it's different from the current value
    ///
    /// # Arguments
    /// * `value` - New value to set
    ///
    /// # Returns
    /// `true` if the value was changed, `false` otherwise
    pub fn set_if_different(&mut self, value: T) -> bool {
        if self.value != value {
            self.value = value;
            self.changed = true;
            true
        } else {
            false
        }
    }
}

impl<T: Default> Default for Observable<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Clone> Observable<T> {
    /// Get a clone of the current value
    pub fn get_cloned(&self) -> T {
        self.value.clone()
    }
}

impl<T: fmt::Display> fmt::Display for Observable<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

// Implement Deref for convenient access
impl<T> std::ops::Deref for Observable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_observable_creation() {
        let obs = Observable::new(42);
        assert_eq!(*obs.get(), 42);
        assert!(!obs.has_changed());
    }

    #[test]
    fn test_observable_set() {
        let mut obs = Observable::new(10);
        assert!(!obs.has_changed());

        obs.set(20);
        assert!(obs.has_changed());
        assert_eq!(*obs.get(), 20);
    }

    #[test]
    fn test_observable_clear_change() {
        let mut obs = Observable::new(5);
        obs.set(10);
        assert!(obs.has_changed());

        obs.clear_change();
        assert!(!obs.has_changed());
        assert_eq!(*obs.get(), 10);
    }

    #[test]
    fn test_observable_get_mut() {
        let mut obs = Observable::new(100);
        assert!(!obs.has_changed());

        *obs.get_mut() = 200;
        assert!(obs.has_changed());
        assert_eq!(*obs.get(), 200);
    }

    #[test]
    fn test_observable_set_if_different() {
        let mut obs = Observable::new(50);

        let changed = obs.set_if_different(50);
        assert!(!changed);
        assert!(!obs.has_changed());

        let changed = obs.set_if_different(100);
        assert!(changed);
        assert!(obs.has_changed());
    }

    #[test]
    fn test_observable_into_inner() {
        let obs = Observable::new(42);
        let value = obs.into_inner();
        assert_eq!(value, 42);
    }

    #[test]
    fn test_observable_deref() {
        let obs = Observable::new(String::from("hello"));
        assert_eq!(obs.len(), 5); // Using String::len via Deref
    }

    #[test]
    fn test_observable_default() {
        let obs: Observable<i32> = Observable::default();
        assert_eq!(*obs.get(), 0);
        assert!(!obs.has_changed());
    }

    #[test]
    fn test_observable_clone() {
        let obs1 = Observable::new(42);
        let mut obs2 = obs1.clone();

        obs2.set(100);
        assert_eq!(*obs1.get(), 42);
        assert_eq!(*obs2.get(), 100);
    }
}
