pub mod common;

use common::table_field_generated::example as reader;
use common::table_field_generated_verifier::example as verifier;
use common::table_field_generated_verifier::get_root;

#[test]
fn test_soffset_overflow() {
    let buf = [
        le!(4u32),
        // hero
        le!(6i32),
    ]
    .concat();
    let hero = get_root::<reader::Hero>(&buf);
    assert_eq!(hero, Err(verifier::Error::OutOfBounds));
}
