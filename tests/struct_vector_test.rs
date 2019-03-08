pub mod common;

use cfb::builder::Builder;
use proptest::prelude::*;

use common::struct_vector_builder::example as cfbe;
use common::struct_vector_generated::example as fbe;

fn _test_struct_vector_builder(stats: Vec<(u32, u8)>) {
    let buf = Builder::new(cfbe::Hero {
        stats: stats
            .iter()
            .map(|(hp, mp)| cfbe::Stat {
                hp: *hp,
                mp: *mp,
                ..Default::default()
            })
            .collect(),
    })
    .build();

    let root = flatbuffers::get_root::<fbe::Hero>(&buf[..]);

    assert_eq!(
        stats,
        root.stats()
            .map(|stats| stats.iter().map(|s| (s.hp(), s.mp())).collect::<Vec<_>>())
            .unwrap_or_default()
    );
}

#[test]
fn test_struct_vector_builder() {
    _test_struct_vector_builder(vec![]);
    _test_struct_vector_builder(vec![(1, 2)]);
}

proptest! {
    #[test]
    fn proptest_struct_vector_builder(stats: Vec<(u32, u8)>) {
        _test_struct_vector_builder(stats);
    }
}
