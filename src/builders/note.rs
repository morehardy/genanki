use serde::{ Serialize, Deserialize };
use crate::utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    id: i64,
    model_id: i64,
    fields: Vec<String>,
    tags: Vec<String>,
    guid: String,
}

impl Note {
    pub fn new(model_id: i64, fields: Vec<String>) -> Self {
        let guid = utils::generate_guid(&fields);
        Note {
            id: utils::generate_id(),
            model_id,
            fields,
            tags: vec![],
            guid,
        }
    }
}