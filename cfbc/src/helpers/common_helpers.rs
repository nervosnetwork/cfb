#![allow(clippy::redundant_closure)]

use handlebars::Handlebars;
use inflector::Inflector;

handlebars_helper!(screaming_snake_case_helper: |s: str| s.to_screaming_snake_case());

pub fn register_helpers(reg: &mut Handlebars) {
    reg.register_helper(
        "screaming_snake_case",
        Box::new(screaming_snake_case_helper),
    );
}
