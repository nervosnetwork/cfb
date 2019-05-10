pub mod common;

use cfb_runtime::Builder;
use proptest::prelude::*;

use common::string_builder as cfb_builder;
use common::string_generated as flatc;

fn _test_string_builder(name: String) {
    let buf = Builder::new().build(cfb_builder::AuthorBuilder { name: Some(&name) });
    let root = flatbuffers::get_root::<flatc::Author>(&buf[..]);

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
