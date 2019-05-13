use std::mem::size_of;

/// Unsigned offset used for reference to table, vector and string.
pub type UOffset = u32;
/// Signed offset used for vtable.
pub type SOffset = i32;
/// Unsigned offset used for field offset stored in vtable.
pub type VOffset = u16;
/// Length of vector and string.
pub type Len = u32;

pub const SIZE_OF_UOFFSET: usize = size_of::<UOffset>();
pub const SIZE_OF_SOFFSET: usize = size_of::<SOffset>();
pub const SIZE_OF_VOFFSET: usize = size_of::<VOffset>();
pub const SIZE_OF_LEN: usize = size_of::<Len>();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_of() {
        assert_eq!(4, SIZE_OF_UOFFSET);
        assert_eq!(4, SIZE_OF_SOFFSET);
        assert_eq!(2, SIZE_OF_VOFFSET);
        assert_eq!(4, SIZE_OF_LEN);
    }
}
