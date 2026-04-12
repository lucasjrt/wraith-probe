use std::fmt;

use esp_idf_svc::sys::EspError;

#[derive(Debug)]
pub enum WraithError {
    Esp(EspError),
    Io(std::io::Error),
    Display(String),
    Wifi(String),
    Lock(String),
}

impl fmt::Display for WraithError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WraithError::Esp(e) => write!(f, "ESP: {}", e),
            WraithError::Io(e) => write!(f, "IO: {}", e),
            WraithError::Display(msg) => write!(f, "Display: {}", msg),
            WraithError::Wifi(msg) => write!(f, "WiFi: {}", msg),
            WraithError::Lock(msg) => write!(f, "Lock: {}", msg),
        }
    }
}

impl std::error::Error for WraithError {}

impl From<EspError> for WraithError {
    fn from(e: EspError) -> Self {
        WraithError::Esp(e)
    }
}

impl From<std::io::Error> for WraithError {
    fn from(e: std::io::Error) -> Self {
        WraithError::Io(e)
    }
}

pub type WraithResult<T> = Result<T, WraithError>;
