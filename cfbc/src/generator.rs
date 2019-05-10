use crate::helpers::{common_helpers, rust_helpers};
use crate::lang::Lang;
use crate::templates::register_templates;
use cfb_schema::Schema;
use handlebars::{Handlebars, RenderError};
use serde::Serialize;
use std::io;
use std::sync::Arc;

#[derive(Serialize, Debug)]
struct Context {
    schema: Schema,
}

pub struct Generator {
    context: Context,
    reg: Handlebars,
}

impl Generator {
    pub fn new(lang: Lang, schema: Schema) -> Self {
        let context = Context {
            schema: schema.clone(),
        };

        let mut reg = Handlebars::new();
        register_templates(&mut reg, lang);
        common_helpers::register_helpers(&mut reg);

        let schema = Arc::new(schema);
        match lang {
            Lang::Rust => rust_helpers::register_helpers(&mut reg, &schema),
        }

        Generator { context, reg }
    }

    pub fn generate_builder<W: io::Write>(&self, writer: W) -> Result<(), RenderError> {
        self.reg.render_to_write("builder", &self.context, writer)
    }
}
