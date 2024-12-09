use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub(crate) ord: usize,
    pub(crate) bfont: Option<u32>,
    pub(crate) name: String,
    pub(crate) qfmt: Option<String>,
    pub(crate) afmt: Option<String>,
    pub(crate) bqfmt: Option<String>,
    pub(crate) bafmt: Option<String>,
    pub(crate) did: Option<usize>,
    pub(crate) bsize: Option<usize>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Temp {
    pub(crate) ord: usize,
    pub(crate) bfont: u32,
    pub(crate) name: String,
    pub(crate) qfmt: String,
    pub(crate) afmt: String,
    pub(crate) bqfmt: String,
    pub(crate) bafmt: String,
    pub(crate) did: usize,
    pub(crate) bsize: usize,
}

impl Template {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Template {
            name: name.into(),
            qfmt: None,
            afmt: None,
            bqfmt: None,
            bafmt: None,
            did: None,
            bfont: None,
            ord: 0,
            bsize: None,
        }
    }

    pub fn qfmt<S: Into<String>>(mut self, qfmt: S) -> Self {
        self.qfmt = Some(qfmt.into());
        self
    }

    pub fn afmt<S: Into<String>>(mut self, afmt: S) -> Self {
        self.afmt = Some(afmt.into());
        self
    }

    pub fn bqfmt<S: Into<String>>(mut self, bqfmt: S) -> Self {
        self.bqfmt = Some(bqfmt.into());
        self
    }

    pub fn bafmt<S: Into<String>>(mut self, bafmt: S) -> Self {
        self.bafmt = Some(bafmt.into());
        self
    }

    pub fn did(mut self, id: usize) -> Self {
        self.did = Some(id);
        self
    }
}

impl Into<Temp> for Template {
    fn into(self) -> Temp {
        Temp {
            bfont: self.bfont.unwrap(),
            ord: self.ord,
            name: self.name,
            qfmt: self.qfmt.unwrap_or("".into()),
            afmt: self.afmt.unwrap_or("".into()),
            bafmt: self.bafmt.unwrap_or("".into()),
            bqfmt: self.bqfmt.unwrap_or("".into()),
            did: self.did.unwrap_or(0),
            bsize: self.bsize.unwrap_or(0),
        }
    }
}
