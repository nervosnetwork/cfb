pub mod common;

use cfb::builder::Builder;
use proptest::prelude::*;

fn _test_scalar_vector_builder(readings: Vec<u32>) {
    let buf = Builder::new(common::scalar_vector_builder::example::Sensor {
        readings: readings.clone(),
    })
    .build();

    let root = flatbuffers::get_root::<common::scalar_vector_generated::example::Sensor>(&buf[..]);

    if readings.is_empty() {
        assert!(root.readings().is_none());
    } else {
        assert_eq!(
            readings,
            common::collect_flatbuffers_vector(&root.readings().unwrap())
        );
    }
}

#[test]
fn test_empty_scalar_vector_builder() {
    _test_scalar_vector_builder(vec![])
}

#[test]
fn test_scalar_vector_builder() {
    _test_scalar_vector_builder(vec![1u32])
}

proptest! {
    #[test]
    fn proptest_scalar_vector_builder(readings: Vec<u32>) {
        _test_scalar_vector_builder(readings);
    }
}
