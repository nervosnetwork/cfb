pub mod common;

use cfb::builder::Builder;
use proptest::prelude::*;

use common::union_builder::example as cfbe;
use common::union_generated::example as fbe;

fn arb_role() -> impl Strategy<Value = cfbe::Role> {
    prop_oneof![
        any::<u32>().prop_map(|hp| cfbe::Role::Hero(cfbe::Hero { hp })),
        any::<u32>().prop_map(|hp| cfbe::Role::Monster(cfbe::Monster { hp })),
    ]
}

fn _test_union_builder(role: Option<cfbe::Role>) {
    let buf = Builder::new(cfbe::Player { role: role.clone() }).build();
    let root = flatbuffers::get_root::<fbe::Player>(&buf[..]);

    assert_eq!(
        role.as_ref().map(|r| r.union_type()).unwrap_or_default(),
        root.role_type() as u8
    );

    match role {
        Some(cfbe::Role::Hero(cfbe::Hero { hp })) => {
            let hero = root.role_as_hero();
            assert!(hero.is_some());
            assert_eq!(hp, hero.unwrap().hp());
        }
        Some(cfbe::Role::Monster(cfbe::Monster { hp })) => {
            let monster = root.role_as_monster();
            assert!(monster.is_some());
            assert_eq!(hp, monster.unwrap().hp());
        }
        None => assert!(root.role().is_none()),
    }
}

#[test]
fn test_union_builder() {
    _test_union_builder(None);
}

proptest! {
    #[test]
    fn proptest_union_builder(role in prop::option::of(arb_role())) {
        _test_union_builder(role);
    }
}
