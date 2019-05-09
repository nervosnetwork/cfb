// Shields clippy errors in generated bundled.rs
#![allow(clippy::unreadable_literal)]

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

use handlebars::Handlebars;
use std::io::Read;

fn templates() -> impl Iterator<Item = (&'static str, Box<dyn Read + 'static>)> {
    TEMPLATES.file_names().map(|key| {
        (
            key.rsplitn(2, '/')
                .next()
                .and_then(|file_name| file_name.rsplitn(2, '.').nth(1))
                .expect("invalid bundled template file name"),
            TEMPLATES.read(key).expect("cannot read bundled template"),
        )
    })
}

pub fn register_templates(reg: &mut Handlebars) {
    for (name, mut source) in templates() {
        reg.register_template_source(name, &mut source)
            .expect("register template");
    }
}
