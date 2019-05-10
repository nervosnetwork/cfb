pub mod common;

use cfb_runtime::Builder;
use proptest::prelude::*;
use std::mem::transmute;

use common::struct_nesting_builder as cfb_builder;
use common::struct_nesting_generated as flatc;

prop_compose! {
    fn arb_color()(color in 0i8..3i8) -> cfb_builder::Color {
        unsafe { transmute(color) }
    }
}

fn _test_struct_nesting_builder(x: u64, y: cfb_builder::Color, z: u64) {
    let buf = Builder::new().build(cfb_builder::WrapperBuilder {
        outter: cfb_builder::Outter {
            x: cfb_builder::Inner { x },
            y,
            z,
            ..Default::default()
        },
    });
    let root = flatbuffers::get_root::<flatc::Wrapper>(&buf[..]);

    if x == 0 && y == cfb_builder::Color::Red && z == 0 {
        assert!(root.outter().is_none());
    } else {
        let outter = root.outter().unwrap();
        assert_eq!(x, outter.x().x());
        assert_eq!(y as i8, outter.y() as i8);
        assert_eq!(z, outter.z());
    }
}

#[test]
fn test_struct_nesting_default_builder() {
    _test_struct_nesting_builder(0, cfb_builder::Color::Red, 0)
}

#[test]
fn test_struct_nesting_builder() {
    _test_struct_nesting_builder(1, cfb_builder::Color::Blue, 3)
}

proptest! {
    #[test]
    fn proptest_struct_nesting_builder(x: u64, y in arb_color(), z: u64) {
        _test_struct_nesting_builder(x, y, z);
    }
}
