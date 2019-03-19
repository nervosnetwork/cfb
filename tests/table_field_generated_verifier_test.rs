pub mod common;

use common::table_field_generated::example as reader;
use common::table_field_generated_verifier::example as verifier;
use common::table_field_generated_verifier::get_root;

macro_rules! le {
    ($e:expr) => {
        &(($e).to_le_bytes())[..]
    };
}

#[test]
fn test_table_header_out_of_bounds() {
    let buf = [le!(6u32), le!(4u32)].concat();
    let hero = get_root::<reader::Hero>(&buf);
    assert_eq!(hero, Err(verifier::Error::OutOfBounds));
}

#[test]
fn test_vtable_out_of_bounds() {
    let buf = [le!(4u32), le!(-4i32), le!(0u16)].concat();
    let hero = get_root::<reader::Hero>(&buf);
    assert_eq!(hero, Err(verifier::Error::OutOfBounds));
}
