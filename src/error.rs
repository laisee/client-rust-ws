use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum AppError {
    Config(String),
    Connection(String),
    Authentication(String),
    WebSocket(tungstenite::Error),
    Io(std::io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Config(msg) => write!(f, "Configuration error: {}", msg),
            AppError::Connection(msg) => write!(f, "Connection error: {}", msg),
            AppError::Authentication(msg) => write!(f, "Authentication error: {}", msg),
            AppError::WebSocket(e) => write!(f, "WebSocket error: {}", e),
            AppError::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl Error for AppError {}

impl From<tungstenite::Error> for AppError {
    fn from(error: tungstenite::Error) -> Self {
        AppError::WebSocket(error)
    }
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::Io(error)
    }
}

impl From<String> for AppError {
    fn from(error: String) -> Self {
        AppError::Config(error)
    }
}
