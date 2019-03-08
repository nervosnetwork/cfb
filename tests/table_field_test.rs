pub mod common;

use cfb::builder::Builder;
use proptest::prelude::*;

use common::table_field_builder::example as cfbe;
use common::table_field_generated::example as fbe;

fn _test_table_field_builder(hp: Option<u32>) {
    let buf = Builder::new(cfbe::Hero {
        stat: hp.map(|hp| cfbe::Stat { hp }),
    })
    .build();

    let root = flatbuffers::get_root::<fbe::Hero>(&buf[..]);

    assert_eq!(hp, root.stat().map(|s| s.hp()));
}

#[test]
fn test_table_field_builder() {
    _test_table_field_builder(None);
    _test_table_field_builder(Some(1));
}

proptest! {
    #[test]
    fn proptest_table_field_builder(hp: Option<u32>) {
        _test_table_field_builder(hp);
    }
}
