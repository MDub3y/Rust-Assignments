/*
  Problem 55: Custom Error Enum (thiserror-style)

  Define an error enum DataError with variants InvalidLength { expected, actual },
  ChecksumMismatch, and Utf8Error(std::string::FromUtf8Error).
  Implement std::fmt::Display and std::error::Error manually.
  Write a function validate_packet that checks a &[u8] has exactly 10 bytes
  with a valid checksum (last byte = XOR of all previous bytes).

  Run the tests for this problem with:
    cargo test --test custom_error_enum_test
*/

use std::fmt::{self, write};

#[derive(Debug)]
pub enum DataError {
    InvalidLength { expected: usize, actual: usize },
    ChecksumMismatch,
    Utf8Error(std::string::FromUtf8Error),
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataError::InvalidLength { expected, actual } => {
                write!(f, "Invalid length: expected {}, got {}", expected, actual)
            }
            DataError::ChecksumMismatch => {
                write!(f, "Checksum verification failed")
            }
            DataError::Utf8Error(e) => {
                write!(f, "UTF-8 error: {}", e)
            }
        }
    }
}

impl std::error::Error for DataError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DataError::Utf8Error(e) => Some(e),
            _ => None
        }
    }
}

pub fn validate_packet(data: &[u8]) -> Result<(), DataError> {
    if data.len() != 10 {
        return Err(DataError::InvalidLength { expected: 10, actual: data.len() });
    }

    let calculated_checksum = data.iter().take(9).fold(0, |acc, &byte| acc ^ byte);

    let provided_checksum = data[9];

    if calculated_checksum != provided_checksum {
        return Err(DataError::ChecksumMismatch);
    }

    Ok(())
}
