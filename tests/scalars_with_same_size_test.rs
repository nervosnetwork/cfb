pub mod common;

use cfb::builder::Builder;
use proptest::prelude::*;

fn _test_scalars_with_same_size_builder(x: u64, y: u64) {
    let buf =
        Builder::new(common::scalars_with_same_size_builder::example::PointComponent { x, y })
            .build();

    let root =
        flatbuffers::get_root::<common::scalars_with_same_size_generated::example::Point>(&buf[..]);
    assert_eq!(x, root.x());
    assert_eq!(y, root.y());
}

#[test]
fn test_scalars_with_same_size_builder() {
    _test_scalars_with_same_size_builder(1, 2)
}

proptest! {
    #[test]
    fn proptest_scalars_with_same_size_builder(x: u64, y: u64) {
        _test_scalars_with_same_size_builder(x, y);
    }
}
