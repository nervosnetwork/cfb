pub mod common;

use cfb::builder::Builder;
use proptest::prelude::*;

use common::nested_buffer_builder as cfbe;
use common::nested_buffer_generated as fbe;

fn _test_nested_buffer_builder(number: Option<u64>) {
    let buf = Builder::new(cfbe::Block {
        header: number.map(|number| cfbe::Header { number }),
    })
    .build();

    let root = flatbuffers::get_root::<fbe::Block>(&buf[..]);
    if number.is_some() {
        let header = flatbuffers::get_root::<fbe::Header>(root.header().unwrap());
        assert_eq!(number.unwrap(), header.number());
    } else {
        assert!(root.header().is_none());
    }
}

#[test]
fn test_nested_buffer_builder() {
    _test_nested_buffer_builder(None);
    _test_nested_buffer_builder(Some(1));
}

proptest! {
    #[test]
    fn proptest_nested_buffer_builder(hp: Option<u64>) {
        _test_nested_buffer_builder(hp);
    }
}
