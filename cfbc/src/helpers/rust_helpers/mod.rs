#![allow(clippy::redundant_closure)]

mod field_default_helper;
mod is_field_present_helper;
mod rust_type_helper;

use cfb_schema::Schema;
use handlebars::Handlebars;
use std::sync::Arc;

handlebars_helper!(lifetime_if_helper: |c: bool| if c { "<'a>" } else { "" });

pub fn register_helpers(reg: &mut Handlebars, schema: &Arc<Schema>) {
    reg.register_helper(
        "rust_type",
        Box::new(rust_type_helper::RustTypeHelper(Arc::clone(schema))),
    );
    reg.register_helper(
        "is_field_present",
        Box::new(is_field_present_helper::IsFieldPresentHelper(Arc::clone(
            schema,
        ))),
    );
    reg.register_helper(
        "field_default",
        Box::new(field_default_helper::FieldDefaultHelper(Arc::clone(schema))),
    );
    reg.register_helper("lifetime_if", Box::new(lifetime_if_helper));
}
