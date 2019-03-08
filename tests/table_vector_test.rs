pub mod common;

use cfb::builder::Builder;
use proptest::prelude::*;

use common::table_vector_builder::example as cfbe;
use common::table_vector_generated::example as fbe;

fn _test_table_vector_builder(hps: Vec<u32>) {
    let buf = Builder::new(cfbe::Hero {
        stats: hps.iter().map(|hp| cfbe::Stat { hp: *hp }).collect(),
    })
    .build();

    let root = flatbuffers::get_root::<fbe::Hero>(&buf[..]);

    assert_eq!(
        hps,
        root.stats()
            .as_ref()
            .map(common::collect_flatbuffers_vector)
            .unwrap_or_default()
            .into_iter()
            .map(|s| s.hp())
            .collect::<Vec<_>>()
    );
}

#[test]
fn test_table_vector_builder() {
    _test_table_vector_builder(vec![]);
    _test_table_vector_builder(vec![1]);
}

proptest! {
    #[test]
    fn proptest_table_vector_builder(hps: Vec<u32>) {
        _test_table_vector_builder(hps);
    }
}
