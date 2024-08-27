/// Implements a generator for generating unique identifiers for entities
#[derive(Clone, Copy, Debug)]
pub struct IdGenerator {
    id: i32,
}

impl IdGenerator {
    /// Creates a new `IdGenerator` with an initial identifier of 0
    pub fn new() -> IdGenerator {
        IdGenerator { id: 0 }
    }

    /// Generates a new unique identifier
    /// Increments the identifier by 1 and returns the new identifier
    /// # Returns
    /// The new unique identifier
    pub fn get_id(&mut self) -> i32 {
        self.id += 1;
        log::debug!("New id: {}", self.id);
        self.id
    }
}