pub mod common;

use common::union_generated::example::Player;
use common::union_generated_verifier::{get_root, Error};

#[test]
fn test_union_element_verification_error() {
    let buf = [
        le!(12u32),
        // vtable
        le!(8u16),
        le!(12u16),
        le!(4u16),
        le!(8u16),
        // tab
        le!(8i32),
        le!(1u32),
        le!(12u32),
        // vtable
        le!(6u16),
        le!(8u16),
        le!(4u16),
        le!(0u16),
        // tab
        le!(8i32),
    ]
    .concat();
    let hero = get_root::<Player>(&buf);
    assert_eq!(hero, Err(Error::OutOfBounds));
}
