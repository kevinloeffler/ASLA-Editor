use std::error;
use std::fmt;
use serde::{Serialize, Deserialize};

// Define a custom error type
#[derive(Debug, Serialize, Deserialize)]
pub struct BackendError {
    pub message: String,
}

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "General Backend Error: {}", self.message)
    }
}

impl error::Error for BackendError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None // You can provide the underlying cause here if needed
    }
}
