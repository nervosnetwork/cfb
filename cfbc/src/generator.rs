use crate::helpers::register_helpers;
use crate::templates::register_templates;
use cfb_schema::Schema;
use handlebars::{Handlebars, RenderError};
use serde::Serialize;
use std::io;

#[derive(Serialize, Debug)]
struct Context {
    schema: Schema,
}

pub struct Generator {
    context: Context,
    handlebars: Handlebars,
}

impl Generator {
    pub fn new(schema: Schema) -> Self {
        let mut handlebars = Handlebars::new();
        register_templates(&mut handlebars);
        register_helpers(&mut handlebars, schema.clone());

        let context = Context { schema };
        // println!("{}", handlebars::to_json(&context).to_string());

        Generator {
            context,
            handlebars,
        }
    }

    pub fn generate_builder<W: io::Write>(&self, writer: W) -> Result<(), RenderError> {
        self.handlebars
            .render_to_write("builder.rs", &self.context, writer)
    }
}
