
use crate::base::fmt;

/// An error indicating that a message does not have the correct parity
#[derive(Debug)]
pub struct ParityError {
    /// Expected parity bit (based on other bits), 0 or 1
    expected: u8,
    /// Actual parity bit (in the message), 0 or 1
    actual: u8,
}

impl ParityError {
    pub(crate) fn new(expected: u8, actual: u8) -> Self {
        ParityError {
            expected,
            actual,
        }
    }
}


impl fmt::Display for ParityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "incorrect parity: expected {}, actual {}", self.expected, self.actual)
    }
}

#[cfg(feature = "std")]
mod std_impls {
    use std::error::Error;
    use super::ParityError;

    impl Error for ParityError {
    }
}
