// Shields clippy errors in generated bundled.rs
#![allow(clippy::unreadable_literal)]

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

use crate::lang::Lang;
use handlebars::Handlebars;
use std::io::Read;

fn templates(lang: Lang) -> impl Iterator<Item = (&'static str, Box<dyn Read + 'static>)> {
    let filter_key = match lang {
        Lang::Rust => "/rust/",
    };

    TEMPLATES
        .file_names()
        .filter(move |key| key.contains(filter_key))
        .map(|key| {
            (
                key.rsplitn(2, '/')
                    .next()
                    .and_then(|file_name| file_name.splitn(2, '.').next())
                    .expect("invalid bundled template file name"),
                TEMPLATES.read(key).expect("cannot read bundled template"),
            )
        })
}

pub fn register_templates(reg: &mut Handlebars, lang: Lang) {
    for (name, mut source) in templates(lang) {
        reg.register_template_source(name, &mut source)
            .expect("register template");
    }
}
