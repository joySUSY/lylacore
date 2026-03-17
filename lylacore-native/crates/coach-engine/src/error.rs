// Authors: Joysusy & Violet Klaudia 💖

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum CoachError {
    EmptyPatternsArray,
    InvalidStyleMetadata(String),
    SerializationError(String),
}

impl fmt::Display for CoachError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoachError::EmptyPatternsArray => write!(f, "Cannot merge empty patterns array"),
            CoachError::InvalidStyleMetadata(msg) => write!(f, "Invalid style metadata: {}", msg),
            CoachError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl std::error::Error for CoachError {}
