mod schema_context;

mod common_helpers;
mod rust_helpers;

use handlebars::Handlebars;

pub fn register_helpers(reg: &mut Handlebars) {
    common_helpers::register_helpers(reg);
    rust_helpers::register_helpers(reg);
}
