use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Field {
    pub(crate) name: String,
    pub(crate) ord: usize,
    pub(crate) sticky: Option<bool>,
    pub(crate) rtl: Option<bool>,
    pub(crate) font_name: Option<String>,
    pub(crate) font_size: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Fld {
    pub(crate) name: String,
    pub(crate) ord: usize,
    pub(crate) sticky: bool,
    pub(crate) rtl: bool,
    pub(crate) font_name: String,
    pub(crate) font_size: u32,
}

impl Field {
    pub fn new(name: &str) -> Self {
        Field {
            name: name.into(),
            ord: 0,
            sticky: None,
            rtl: None,
            font_name: None,
            font_size: None

        }
    }

    pub fn sticky(mut self, sticky: bool) -> Self {
        self.sticky = Some(sticky);
        self
    }

    pub  fn rtl(mut self, rtl: bool) -> Self {
        self.rtl = Some(rtl);
        self
    }

    pub fn font_name(mut self, font_name: &str) -> Self {
        self.font_name = Some(font_name.into());
        self
    }

    pub fn font_size(mut self, font_size: u32) -> Self {
        self.font_size = Some(font_size);
        self
    }

    pub fn get_sticky(&self) -> bool {
        self.sticky.unwrap_or(false)
    }
    
    pub fn get_rtl(&self) -> bool {
        self.rtl.unwrap_or(false)
    }
    
    pub fn get_font_name(&self) -> String {
        self.font_name.clone().unwrap_or_else(|| "Arial".into())
    }
    
    pub fn get_font_size(&self) -> u32 {
        self.font_size.unwrap_or(20)
    }
}

impl Into<Fld> for Field {
    fn into(self) -> Fld {
        Fld {
            name: self.name.into(),
            sticky: self.sticky.unwrap_or(false),
            rtl: self.rtl.unwrap_or(false),
            ord: 0,
            font_name: self.font_name.unwrap_or("Arial".into()),
            font_size: self.font_size.unwrap_or(20) as u32
        }
    }
}