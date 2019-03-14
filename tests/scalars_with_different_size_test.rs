pub mod common;

use cfb::builder::Builder;
use proptest::prelude::*;

fn _test_scalars_with_different_size_builder(year: u32, balance: u64) {
    let buf = Builder::new(
        common::scalars_with_different_size_builder::example::Account { year, balance },
    )
    .build();

    let root = flatbuffers::get_root::<
        common::scalars_with_different_size_generated::example::Account,
    >(&buf[..]);
    assert_eq!(year, root.year());
    assert_eq!(balance, root.balance());
}

#[test]
fn test_scalars_with_different_size_builder() {
    _test_scalars_with_different_size_builder(1, 2)
}

proptest! {
    #[test]
    fn proptest_scalars_with_different_size_builder(year: u32, balance: u64) {
        _test_scalars_with_different_size_builder(year, balance);
    }
}
