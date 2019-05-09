use crate::templates::templates;
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
        for (name, mut source) in templates() {
            handlebars
                .register_template_source(name, &mut source)
                .expect("register template");
        }

        let context = Context { schema };

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
