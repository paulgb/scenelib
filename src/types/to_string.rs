//! Debug helper trait.

/// Struct is convertable into a string.
pub trait ToString {
    /// Returns a string representation of self.
    fn to_string(&self) -> String;
}
