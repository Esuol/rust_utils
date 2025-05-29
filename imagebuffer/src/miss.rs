use std::error::Error;
use std::fmt;

/// Represents a missing or error case in the image buffer operations
#[derive(Debug)]
pub enum ImageBufferMiss {
    /// The requested pixel position is out of bounds
    OutOfBounds {
        /// The x coordinate that was out of bounds
        x: u32,
        /// The y coordinate that was out of bounds
        y: u32,
        /// The maximum width of the buffer
        max_width: u32,
        /// The maximum height of the buffer
        max_height: u32,
    },
    /// The requested buffer operation failed
    OperationFailed {
        /// Description of what operation failed
        operation: String,
        /// The underlying error if available
        source: Option<Box<dyn Error + Send + Sync>>,
    },
    /// The buffer is empty or uninitialized
    EmptyBuffer,
    /// The buffer format is invalid or unsupported
    InvalidFormat {
        /// Description of the format error
        details: String,
    },
}

impl fmt::Display for ImageBufferMiss {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfBounds {
                x,
                y,
                max_width,
                max_height,
            } => write!(
                f,
                "Pixel position ({}, {}) is out of bounds. Buffer dimensions are {}x{}",
                x, y, max_width, max_height
            ),
            Self::OperationFailed { operation, source } => {
                if let Some(err) = source {
                    write!(f, "Buffer operation failed: {}: {}", operation, err)
                } else {
                    write!(f, "Buffer operation failed: {}", operation)
                }
            }
            Self::EmptyBuffer => write!(f, "The image buffer is empty or uninitialized"),
            Self::InvalidFormat { details } => write!(f, "Invalid buffer format: {}", details),
        }
    }
}

impl Error for ImageBufferMiss {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::OperationFailed { source, .. } => source
                .as_ref()
                .map(|e| e.as_ref() as &(dyn Error + 'static)),
            _ => None,
        }
    }
}

impl From<String> for ImageBufferMiss {
    fn from(operation: String) -> Self {
        Self::OperationFailed {
            operation,
            source: None,
        }
    }
}

impl<E: Error + Send + Sync + 'static> From<(String, E)> for ImageBufferMiss {
    fn from((operation, error): (String, E)) -> Self {
        Self::OperationFailed {
            operation,
            source: Some(Box::new(error)),
        }
    }
}
