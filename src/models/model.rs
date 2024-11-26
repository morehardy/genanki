use serde::{ Serialize, Deserialize };
use crate::utils;
#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) fields: Vec<Field>,
    pub(crate) templates: Vec<Template>,
    pub(crate) css: String,
}

impl Model {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Model {
            id: utils::gen_id(),
            name: name.into(),
            fields: Vec::new(),
            templates: Vec::new(),
            css: String::new(),
        }
    }
}