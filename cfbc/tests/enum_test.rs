pub mod common;

use cfb_runtime::Builder;
use proptest::prelude::*;
use std::mem::transmute;

use common::enum_builder as cfb_builder;
use common::enum_generated as flatc;

prop_compose! {
    fn arb_color()(color in 0i8..3i8) -> cfb_builder::Color {
        unsafe { transmute(color) }
    }
}

fn _test_enum_builder(color: cfb_builder::Color) {
    let buf = Builder::new().build(cfb_builder::BagBuilder { color });
    let root = flatbuffers::get_root::<flatc::Bag>(&buf[..]);
    assert_eq!(color as i8, root.color() as i8);
}

#[test]
fn test_enum_builder() {
    _test_enum_builder(cfb_builder::Color::Red)
}

proptest! {
    #[test]
    fn proptest_enum_builder(color in arb_color()) {
        _test_enum_builder(color);
    }
}
