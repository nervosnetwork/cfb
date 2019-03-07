pub mod common;

use cfb::builder::Builder;
use proptest::prelude::*;

fn _test_string_vector_builder(lines: Vec<String>) {
    let buf = Builder::new(common::string_vector_builder::example::Text {
        lines: lines.clone(),
    })
    .build();

    let root = flatbuffers::get_root::<common::string_vector_generated::example::Text>(&buf[..]);

    if lines.is_empty() {
        assert!(root.lines().is_none());
    } else {
        assert_eq!(
            lines,
            common::collect_flatbuffers_vector(&root.lines().unwrap())
        );
    }
}

#[test]
fn test_empty_string_vector_builder() {
    _test_string_vector_builder(vec![]);
    _test_string_vector_builder(vec!["".to_owned()]);
}

#[test]
fn test_string_vector_builder() {
    _test_string_vector_builder(vec!["hello".to_owned()]);
}

proptest! {
    #[test]
    fn proptest_string_vector_builder(lines: Vec<String>) {
        _test_string_vector_builder(lines);
    }
}
