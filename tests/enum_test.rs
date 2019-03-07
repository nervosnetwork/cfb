pub mod common;

use cfb::builder::Builder;
use proptest::prelude::*;
use std::mem::transmute;

fn _test_enum_builder(color: i8) {
    let buf = Builder::new(common::enum_builder::example::Bag {
        color: unsafe { transmute(color) },
    })
    .build();

    let root = flatbuffers::get_root::<common::enum_generated::example::Bag>(&buf[..]);
    assert_eq!(color, root.color() as i8);
}

#[test]
fn test_enum_builder() {
    _test_enum_builder(0)
}

proptest! {
    #[test]
    fn proptest_enum_builder(color in 0i8..3i8) {
        _test_enum_builder(color);
    }
}
