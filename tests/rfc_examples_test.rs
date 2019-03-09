pub mod common;

use cfb::builder::Builder;
use common::data_alignment_builder::{T1, T2};
use common::data_order_builder::{Item, Monster, Stat};
use common::hex;
use common::table_fields_order_builder::{Color, Complex, Ok, Result, T};

const DATA_ORDER_BIN: &[u8] = include_bytes!("common/data_order.bin");
const DATA_ALIGNMENT_BIN: &[u8] = include_bytes!("common/data_alignment.bin");
const TABLE_FIELDS_ORDER_BIN: &[u8] = include_bytes!("common/table_fields_order.bin");

#[test]
fn test_rfc_data_order() {
    let buf = Builder::new(Monster {
        name: "Slime".to_owned(),
        stat: Some(Stat { hp: 100, mp: 0 }),
        loots: vec![
            Item {
                name: "potion".to_owned(),
            },
            Item {
                name: "gold".to_owned(),
            },
        ],
    })
    .build();

    assert_eq!(hex(DATA_ORDER_BIN), hex(&buf));
}

#[test]
fn test_rfc_data_alignment() {
    let buf = Builder::new(T1 {
        f1: 100,
        s1: vec![80],
        f2: Some(T2 { f1: 2 }),
        s2: vec![1, 2, 3, 4, 5],
        f3: vec![101],
        s3: vec![96],
        f4: "a".to_owned(),
    })
    .build();

    assert_eq!(hex(DATA_ALIGNMENT_BIN), hex(&buf));
}

#[test]
fn test_rfc_table_fields_order() {
    let buf = Builder::new(T {
        a_ubyte: 5,
        complex: Complex { a: 1, b: 2 },
        a_uint32: 4,
        result: Some(Result::Ok(Ok { value: 6 })),
        a_uint64: 3,
        uint16_array: vec![7, 8],
        color: Color::Blue,
    })
    .build();

    assert_eq!(hex(TABLE_FIELDS_ORDER_BIN), hex(&buf));
}
