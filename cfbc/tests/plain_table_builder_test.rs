pub mod common;

use cfb_runtime::builder::Builder;
use common::plain_table_builder::example as cfb_builder;
use common::plain_table_generated::example as flatc;
use flatbuffers::get_root;
use proptest::prelude::*;

fn _test_plain_table_builder(year: u32, balance: u64) {
    let buf = Builder::new().build(cfb_builder::AccountBuilder { year, balance });

    let root = get_root::<flatc::Account>(&buf[..]);
    assert_eq!(year, root.year());
    assert_eq!(balance, root.balance());
}

#[test]
fn test_plain_table_builder() {
    _test_plain_table_builder(1, 2)
}

proptest! {
    #[test]
    fn proptest_plain_table_builder(year: u32, balance: u64) {
        _test_plain_table_builder(year, balance);
    }
}
