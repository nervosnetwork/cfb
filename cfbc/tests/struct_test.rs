pub mod common;

use cfb_runtime::Builder;
use proptest::prelude::*;

use common::struct_builder as cfb_builder;
use common::struct_generated as flatc;

fn _test_struct_builder(x: u64, y: u8, z: u64) {
    let buf = Builder::new().build(cfb_builder::PointBuilder {
        position: cfb_builder::Vec3 {
            x,
            y,
            z,
            ..Default::default()
        },
    });
    let root = flatbuffers::get_root::<flatc::Point>(&buf[..]);

    if x == 0 && y == 0 && z == 0 {
        assert!(root.position().is_none());
    } else {
        let position = root.position().unwrap();
        assert_eq!(x, position.x());
        assert_eq!(y, position.y());
        assert_eq!(z, position.z());
    }
}

#[test]
fn test_struct_default_builder() {
    _test_struct_builder(0, 0, 0)
}

#[test]
fn test_struct_builder() {
    _test_struct_builder(1, 2, 3)
}

proptest! {
    #[test]
    fn proptest_struct_builder(x: u64, y: u8, z: u64) {
        _test_struct_builder(x, y, z);
    }
}
