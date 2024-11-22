use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AnkiError {
    Io(std::io::Error),
    Database(rusqlite::Error),
    Json(serde_json::Error),
    Validation(String),
    Media(String),
}

impl fmt::Display for AnkiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnkiError::Io(error) => write!(f, "IO error: {}", error),
            AnkiError::Database(error) => write!(f, "Database error: {}", error),
            AnkiError::Validation(msg) => write!(f, "Validation error: {}", msg),
            AnkiError::Media(msg) => write!(f, "Media error: {}", msg),
            AnkiError::Json(error) => write!(f, "Json error: {}", error),
        }
    }
}

impl Error for AnkiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AnkiError::Io(error) => Some(error),
            AnkiError::Database(error) => Some(error),
            AnkiError::Json(error) => Some(error),
            _ => None,
        }
    }
}

impl From<std::io::Error> for AnkiError {
    fn from(err: std::io::Error) -> Self {
        AnkiError::Io(err)
    }
}

impl From<rusqlite::Error> for AnkiError {
    fn from(err: rusqlite::Error) -> Self {
        AnkiError::Database(err)
    }
}

impl From<serde_json::Error> for AnkiError {
    fn from(err: serde_json::Error) -> Self {
        AnkiError::Json(err)
    }
}
