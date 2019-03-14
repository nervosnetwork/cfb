pub mod common;

use cfb::builder::Builder;
use proptest::prelude::*;

fn _test_string_builder(name: String) {
    let buf = Builder::new(common::string_builder::example::Author { name: name.clone() }).build();

    let root = flatbuffers::get_root::<common::string_generated::example::Author>(&buf[..]);

    if name.is_empty() {
        assert!(root.name().is_none());
    } else {
        assert_eq!(name, root.name().unwrap());
    }
}

#[test]
fn test_empty_string_builder() {
    _test_string_builder("".to_owned())
}

#[test]
fn test_string_builder() {
    _test_string_builder("hello".to_owned())
}

proptest! {
    #[test]
    fn proptest_string_builder(name: String) {
        _test_string_builder(name);
    }
}
