pub mod common;

use common::table_field_generated::example as reader;
use flatbuffers_verifier::{get_root, Error};

#[test]
fn test_soffset_overflow() {
    let buf = [
        le!(4u32),
        // hero
        le!(6i32),
    ]
    .concat();
    let hero = get_root::<reader::Hero>(&buf);
    assert_eq!(hero, Err(Error::OutOfBounds));
}

#[test]
fn test_zero_root_offset() {
    let buf = [le!(0u32)].concat();
    let hero = get_root::<reader::Hero>(&buf);
    assert_eq!(hero, Err(Error::OutOfBounds));
}
