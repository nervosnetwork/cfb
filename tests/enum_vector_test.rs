pub mod common;

use cfb::builder::Builder;
use proptest::prelude::*;
use std::mem::transmute;

use common::enum_vector_builder::example as cfbe;
use common::enum_vector_generated::example as fbe;

prop_compose! {
    fn arb_color()(color in 0i8..3i8) -> cfbe::Color {
        unsafe { transmute(color) }
    }
}

fn _test_enum_vector_builder(colors: Vec<cfbe::Color>) {
    let buf = Builder::new(cfbe::Bag {
        colors: colors.clone(),
    })
    .build();

    let root = flatbuffers::get_root::<fbe::Bag>(&buf[..]);
    assert_eq!(
        colors.iter().map(|c| *c as i8).collect::<Vec<_>>(),
        root.colors()
            .as_ref()
            .map(common::collect_flatbuffers_vector)
            .unwrap_or_default()
            .iter()
            .map(|c| *c as i8)
            .collect::<Vec<_>>()
    );
}

#[test]
fn test_enum_vector_builder() {
    _test_enum_vector_builder(vec![cfbe::Color::Red])
}

proptest! {
    #[test]
    fn proptest_enum_vector_builder(colors in prop::collection::vec(arb_color(), 0..10)) {
        _test_enum_vector_builder(colors);
    }
}
