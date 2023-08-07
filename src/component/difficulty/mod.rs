mod dc;

#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RollStatus {
    /// Rolls with advantage roll twice and take the highest.
    Advantage,
    /// Rolls with disadvantage roll twice and take the lowest.
    Disadvantage,
    /// Rolls only once
    Normal,
}

pub use dc::{DifficultyClass, DifficultyClassResult};
