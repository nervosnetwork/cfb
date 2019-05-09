// Shields clippy errors in generated bundled.rs
#![allow(clippy::unreadable_literal)]

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

use std::io::Read;

pub fn templates() -> impl Iterator<Item = (&'static str, Box<dyn Read + 'static>)> {
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
