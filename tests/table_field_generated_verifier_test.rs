pub mod common;

use common::table_field_generated::example as reader;
use flatbuffers_verifier::{get_root, Error};

#[test]
fn test_table_header_out_of_bounds() {
    let buf = [le!(6u32), le!(4u32)].concat();
    let hero = get_root::<reader::Hero>(&buf);
    assert_eq!(hero, Err(Error::OutOfBounds));
}

#[test]
fn test_vtable_header_out_of_bounds() {
    let buf = [le!(4u32), le!(-4i32), le!(0u16)].concat();
    let hero = get_root::<reader::Hero>(&buf);
    assert_eq!(hero, Err(Error::OutOfBounds));
}

#[test]
fn test_vtable_body_out_of_bounds() {
    let buf = [le!(4u32), le!(-4i32), le!(6u16), le!(4u16)].concat();
    let hero = get_root::<reader::Hero>(&buf);
    assert_eq!(hero, Err(Error::OutOfBounds));
}

#[test]
fn test_table_body_out_of_bounds() {
    let buf = [le!(8u32), le!(4u16), le!(6u16), le!(4i32)].concat();
    let hero = get_root::<reader::Hero>(&buf);
    assert_eq!(hero, Err(Error::OutOfBounds));
}

#[test]
fn test_table_fields_offset_out_of_bounds() {
    {
        let buf = [
            le!(10u32),
            // vtable
            le!(6u16),
            le!(6u16),
            le!(6u16),
            // tab
            le!(6i32),
            le!(0u16),
        ]
        .concat();
        let hero = get_root::<reader::Hero>(&buf);
        assert_eq!(hero, Err(Error::OutOfBounds));
    }

    {
        let buf = [
            le!(10u32),
            // vtable
            le!(6u16),
            le!(6u16),
            le!(4u16),
            // tab
            le!(6i32),
            le!(0u16),
        ]
        .concat();
        let hero = get_root::<reader::Hero>(&buf);
        assert_eq!(hero, Err(Error::OutOfBounds));
    }
}

#[test]
fn test_nested_table_field_out_of_bounds() {
    let buf = [
        le!(10u32),
        // vtable
        le!(6u16),
        le!(8u16),
        le!(4u16),
        // hero
        le!(6i32),
        le!(4u32),
        // vtable
        le!(6u16),
        le!(8u16),
        le!(6u16),
        // stat
        le!(6i32),
        le!(8u32),
    ]
    .concat();
    let hero = get_root::<reader::Hero>(&buf);
    assert_eq!(hero, Err(Error::OutOfBounds));
}
