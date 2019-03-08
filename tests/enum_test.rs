pub mod common;

use cfb::builder::Builder;
use proptest::prelude::*;
use std::mem::transmute;

use common::enum_builder::example as cfbe;
use common::enum_generated::example as fbe;

prop_compose! {
    fn arb_color()(color in 0i8..3i8) -> cfbe::Color {
        unsafe { transmute(color) }
    }
}

fn _test_enum_builder(color: cfbe::Color) {
    let buf = Builder::new(cfbe::Bag { color }).build();

    let root = flatbuffers::get_root::<fbe::Bag>(&buf[..]);
    assert_eq!(color as i8, root.color() as i8);
}

#[test]
fn test_enum_builder() {
    _test_enum_builder(cfbe::Color::Red)
}

proptest! {
    #[test]
    fn proptest_enum_builder(color in arb_color()) {
        _test_enum_builder(color);
    }
}
