mod common_helpers;
mod rust_helpers;
use cfb_schema::Schema;
use handlebars::Handlebars;
use std::sync::Arc;

pub fn register_helpers(reg: &mut Handlebars, schema: Schema) {
    let schema = Arc::new(schema);

    common_helpers::register_helpers(reg);
    rust_helpers::register_helpers(reg, &schema);
}
