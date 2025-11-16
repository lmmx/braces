use std::fmt;

pub type Result<T> = std::result::Result<T, BraceError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BraceError {
    EmptyInput,
    MixedSeparators {
        found: Vec<String>,
        expected: String,
    },
    InvalidBraceInput {
        path: String,
        reason: String,
    },
    DepthLimitExceeded {
        limit: usize,
    },
}

impl fmt::Display for BraceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BraceError::EmptyInput => write!(f, "No paths provided"),
            BraceError::MixedSeparators { found, expected } => {
                write!(
                    f,
                    "Mixed path separators found: {:?}, expected: {}",
                    found, expected
                )
            }
            BraceError::InvalidBraceInput { path, reason } => {
                write!(f, "Invalid brace input '{}': {}", path, reason)
            }
            BraceError::DepthLimitExceeded { limit } => {
                write!(f, "Brace depth limit of {} exceeded", limit)
            }
        }
    }
}

impl std::error::Error for BraceError {}
