use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::{Field, Temp};
use handlebars::Handlebars;
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct Requirement {
    template_ord: usize,
    requirements_type: String,
    required_fields: Vec<usize>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub(crate) id: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) fields: Vec<Field>,
    pub(crate) templates: Vec<Temp>,
    pub(crate) css: String,
    // pub(crate) model_type: Vec<Requirement>,
    pub latex_pre: String,
    pub latex_post: String,
    pub sort_field_index: usize,
}

impl Model {
    pub fn new(id: Option<String>, name: Option<String>, fields: Option<Vec<Field>>, templates: Vec<Temp>) -> Self {
        const DEFAULT_LATEX_PRE: &str = "\\documentclass[12pt]{article}\n\\special{papersize=3in,5in}\n\\usepackage[utf8]{inputenc}\n\\usepackage{amssymb,amsmath}\n\\pagestyle{empty}\n\\setlength{\\parindent}{0in}\n\\begin{document}\n";
        const DEFAULT_LATEX_POST: &str = "\\end{document}";
        Model {
            id,
            name: name.into(),
            fields: fields.unwrap_or_default(),
            templates: templates,
            css: String::new(),
            latex_pre: DEFAULT_LATEX_PRE.to_string(),
            latex_post: DEFAULT_LATEX_POST.to_string(),
            sort_field_index: 0,
        }
    }

    // pub fn set_fields(&mut self, fields: impl Into<Fields>) {
    //     self.fields = fields.into()
    // }

    pub fn compute_requirements(&self) -> Vec<Requirement> {
        let sentinel = "SeNtInEl";
        let field_names: Vec<String> = self.fields.iter().map(|f| f.name.clone()).collect();
        let mut requirements: Vec<Requirement> = Vec::new();
        let req = Handlebars::new();
        for (template_ord, template) in self.templates.iter().enumerate() {
            let mut required_fields: Vec<usize> = Vec::new();
            // all
            for field_ord in 0..field_names.len() {
                let mut field_values = HashMap::new();
                for name in &field_names {
                    field_values.insert(name.clone(), sentinel.to_string());
                }
                field_values.insert(field_names[field_ord].clone(), String::new());
                let rendered = req.render_template(&template.qfmt, &json!(field_values)).unwrap_or_else(|e| panic!("Template error: {:?}", e));
                if rendered.contains(sentinel) {
                    required_fields.push(field_ord);
                }
            }
            if !requirements.is_empty() {
                requirements.push(Requirement {
                    template_ord,
                    requirements_type: "any".into(),
                    required_fields,
                });
                continue;
            }
            required_fields.clear();
            for field_ord in 0..field_names.len() {
                let mut field_values = HashMap::new();
                for name in &field_names {
                    field_values.insert(name.clone(), String::new());
                }
                field_values.insert(field_names[field_ord].clone(), sentinel.to_string());
                let rendered = req.render_template(&template.qfmt, &json!(field_values)).unwrap_or_else(|e| panic!("Template error: {:?}", e));
                if rendered.contains(sentinel) {
                    required_fields.push(field_ord);
                }

                if required_fields.is_empty() {
                    panic!("Template needs at least one field in template.");
                }
            }
            requirements.push(Requirement {
                template_ord,
                requirements_type: "any".into(),
                required_fields,
            });
        }

        requirements
    }

    pub fn to_json(&self, timestamp: f64, deck_id: usize) -> serde_json::Value {
        let templates: Vec<Temp> = self.templates.iter().enumerate().map(|(ord, tmpl)| {
            let mut t = tmpl.clone();
            t.ord = ord;
            t.bafmt = t.bafmt;
            t.bqfmt = t.bqfmt;
            t.bfont = t.bfont;
            t.bsize = t.bsize;
            t.did = deck_id;
            t
        }).collect();
        json!({
            "css": self.css,
            "did": deck_id,
            "flds": self.fields,
            "id": self.id,
            "latexPost": self.latex_post,
            "latexPre": self.latex_pre,
            "latexsvg": false,
            "mod": timestamp as i64,
            "name": self.name,
            "req": self.compute_requirements(),
            "sortf": self.sort_field_index,
            "tags": Vec::<String>::new(),
            "tmpls": templates,
            // "type": self.model_type as i32,
            "usn": -1,
            "vers": Vec::<String>::new()
        })
    }
}