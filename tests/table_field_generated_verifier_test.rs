pub mod common;

use common::table_field_generated::example as reader;
use common::table_field_generated_verifier::example as verifier;
use flatbuffers;
use verifier::Verify;

#[test]
fn test_table_offset_out_of_bounds() {
    let buf = [6u32.to_le_bytes(), 0u32.to_le_bytes()].concat();
    let hero = flatbuffers::get_root::<reader::Hero>(&buf);
    assert_eq!(hero.verify(), Err(verifier::Error::OutOfBounds));
}
