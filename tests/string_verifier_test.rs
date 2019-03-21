pub mod common;

use common::string_generated::example as reader;
use common::string_generated_verifier::example as verifier;
use common::string_generated_verifier::get_root;

#[test]
fn test_string_uoffset_out_of_bounds() {
    let buf = [
        le!(10u32),
        // vtable
        le!(6u16),
        le!(8u16),
        le!(4u16),
        // author
        le!(6i32),
        le!(4u32),
    ]
    .concat();
    let author = get_root::<reader::Author>(&buf);
    assert_eq!(author, Err(verifier::Error::OutOfBounds));
}

#[test]
fn test_string_len_out_of_bounds() {
    let buf = [
        le!(10u32),
        // vtable
        le!(6u16),
        le!(8u16),
        le!(4u16),
        // author
        le!(6i32),
        le!(4u32),
        // name
        le!(0u16),
    ]
    .concat();
    let author = get_root::<reader::Author>(&buf);
    assert_eq!(author, Err(verifier::Error::OutOfBounds));
}

#[test]
fn test_string_content_out_of_bounds() {
    let buf = [
        le!(10u32),
        // vtable
        le!(6u16),
        le!(8u16),
        le!(4u16),
        // author
        le!(6i32),
        le!(4u32),
        // name
        le!(4u32),
        b"1234",
    ]
    .concat();
    let author = get_root::<reader::Author>(&buf);
    assert_eq!(author, Err(verifier::Error::OutOfBounds));
}

#[test]
fn test_string_not_terminated_with_null() {
    let buf = [
        le!(10u32),
        // vtable
        le!(6u16),
        le!(8u16),
        le!(4u16),
        // author
        le!(6i32),
        le!(4u32),
        // name
        le!(4u32),
        b"12345",
    ]
    .concat();
    let author = get_root::<reader::Author>(&buf);
    assert_eq!(author, Err(verifier::Error::NonNullTerminatedString));
}
