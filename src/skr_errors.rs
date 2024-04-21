use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    InvalidFloat(String, u16),
    // Add other kinds of errors as needed
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::InvalidFloat(message, line) => write!(f, "Invalid float: {} at line {}", message, line),
            // Handle other kinds of errors as needed
        }
    }
}

impl std::error::Error for CustomError {}