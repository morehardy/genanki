use rusqlite::Connection;
use crate::error::AnkiError;

#[derive(Debug)]
pub struct Database {
    pub(crate) db: Connection,
}

impl Database {
    pub fn get_db(&self) -> &Connection {
        &self.db
    }

    pub fn new() -> Result<Self, AnkiError> {
        let db = Connection::open_in_memory()?;
        Ok(Database {
            db,
        })
    }
}
