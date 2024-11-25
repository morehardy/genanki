mod error;
mod utils;
mod storage;

pub use error::AnkiError;
pub type Result<T> = std::result::Result<T, AnkiError>;