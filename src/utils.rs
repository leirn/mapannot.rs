//! Utilities module
//!
//! This module provides utility functions and structures that can be used across the application.
//! Currently, it includes an `IdGenerator` for generating unique identifiers.

/// Implements a generator for generating unique identifiers for entities
///
/// The `IdGenerator` struct provides a simple mechanism for generating unique
/// identifiers. Each time a new identifier is requested, the internal counter
/// is incremented, ensuring that each identifier is unique.
///
/// # Examples
///
/// ```
/// use utils::IdGenerator;
///
/// let mut generator = IdGenerator::new();
/// let id1 = generator.get_id();
/// let id2 = generator.get_id();
///
/// assert_eq!(id1, 1);
/// assert_eq!(id2, 2);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct IdGenerator {
    id: i32,
}

impl IdGenerator {
    /// Creates a new `IdGenerator` with an initial identifier of 0
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::IdGenerator;
    ///
    /// let generator = IdGenerator::new();
    /// assert_eq!(generator.get_id(), 1);
    /// ```
    pub fn new() -> IdGenerator {
        IdGenerator { id: 0 }
    }

    /// Generates a new unique identifier
    ///
    /// Increments the identifier by 1 and returns the new identifier.
    ///
    /// # Returns
    ///
    /// The new unique identifier.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::IdGenerator;
    ///
    /// let mut generator = IdGenerator::new();
    /// let id = generator.get_id();
    ///
    /// assert_eq!(id, 1);
    /// ```
    pub fn get_id(&mut self) -> i32 {
        self.id += 1;
        log::debug!("New id: {}", self.id);
        self.id
    }
}