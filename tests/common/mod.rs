#[rustfmt::skip]
pub mod scalars_with_different_size_builder;
#[rustfmt::skip]
#[allow(clippy::all)]
pub mod scalars_with_different_size_generated;
#[rustfmt::skip]
pub mod scalars_with_same_size_builder;
#[rustfmt::skip]
#[allow(clippy::all)]
pub mod scalars_with_same_size_generated;

pub fn hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join("")
}
