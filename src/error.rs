use std::{error::Error, fmt, io};

/// Main error type for the Pollen application
#[derive(Debug)]
pub enum PollenError {
    /// I/O operation failed
    Io(io::Error),
    /// YAML parsing failed
    Yaml(serde_yaml::Error),
    /// Invalid endpoint configuration
    InvalidEndpoint(String),
    /// Invalid configuration option
    InvalidOption(String),
    /// HOME environment variable is not set
    HomeDirectoryNotSet,
    /// Invalid mapping structure in YAML
    InvalidMapping(String),
    /// Circular dependency detected
    CircularDependency(String),
    /// Missing dependency
    MissingDependency(String),
}

impl fmt::Display for PollenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PollenError::Io(err) => write!(f, "File I/O error: {}", err),
            PollenError::Yaml(err) => write!(f, "YAML parsing error: {}", err),
            PollenError::InvalidEndpoint(msg) => write!(f, "Invalid endpoint configuration: {}", msg),
            PollenError::InvalidOption(msg) => write!(f, "Invalid option: {}", msg),
            PollenError::HomeDirectoryNotSet => write!(f, "HOME environment variable is not set"),
            PollenError::InvalidMapping(msg) => write!(f, "Invalid mapping structure: {}", msg),
            PollenError::CircularDependency(msg) => write!(f, "Circular dependency detected: {}", msg),
            PollenError::MissingDependency(msg) => write!(f, "Missing dependency: {}", msg),
        }
    }
}

impl Error for PollenError {}

impl From<io::Error> for PollenError {
    fn from(err: io::Error) -> Self {
        PollenError::Io(err)
    }
}

impl From<serde_yaml::Error> for PollenError {
    fn from(err: serde_yaml::Error) -> Self {
        PollenError::Yaml(err)
    }
}
