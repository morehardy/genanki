mod error;
mod utils;
mod storage;
mod builders;

pub use error::AnkiError;

pub use builders::Note;
pub use builders::Field;
pub use builders::Template;

pub type Result<T> = std::result::Result<T, AnkiError>;

pub use utils::{generate_guid};

