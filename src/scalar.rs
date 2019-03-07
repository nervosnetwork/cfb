use std::intrinsics::copy_nonoverlapping;
use std::mem::{size_of, uninitialized};
use std::slice;

pub trait Scalar: Sized {
    fn to_le(self) -> Self;
    fn from_le(x: Self) -> Self;

    /// Gets bytes representing the scalar in native endian.
    fn as_bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self as *const Self as *const u8, size_of::<Self>()) }
    }

    /// Read scalar from bytes in native endian.
    fn from_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= size_of::<Self>());
        let mut x: Self = unsafe { uninitialized() };
        unsafe { copy_nonoverlapping(bytes.as_ptr() as *const Self, &mut x as *mut Self, 1) };
        x
    }
}

const FALSE_BYTES: &[u8] = &[0];
const TRUE_BYTES: &[u8] = &[1];

impl Scalar for bool {
    fn to_le(self) -> Self {
        self
    }
    fn from_le(x: Self) -> Self {
        x
    }

    fn as_bytes(&self) -> &[u8] {
        if *self {
            TRUE_BYTES
        } else {
            FALSE_BYTES
        }
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        assert!(!bytes.is_empty());
        bytes[0] != 0
    }
}

macro_rules! impl_scalar_no_op {
    ($ty:ident) => {
        impl Scalar for $ty {
            fn to_le(self) -> Self {
                self
            }
            fn from_le(x: Self) -> Self {
                x
            }
        }
    };
}

impl_scalar_no_op!(i8);
impl_scalar_no_op!(u8);

macro_rules! impl_scalar_for_int {
    ($ty:ident) => {
        impl Scalar for $ty {
            fn to_le(self) -> Self {
                self.to_le()
            }
            fn from_le(x: Self) -> Self {
                Self::from_le(x)
            }
        }
    };
}

impl_scalar_for_int!(i16);
impl_scalar_for_int!(u16);
impl_scalar_for_int!(i32);
impl_scalar_for_int!(u32);
impl_scalar_for_int!(i64);
impl_scalar_for_int!(u64);

macro_rules! impl_scalar_for_float {
    ($ty:ident) => {
        impl Scalar for $ty {
            fn to_le(self) -> Self {
                #[cfg(target_endian = "little")]
                {
                    self
                }
                #[cfg(not(target_endian = "little"))]
                {
                    Self::from_bits(self.to_bits().swap_bytes())
                }
            }
            fn from_le(x: Self) -> Self {
                #[cfg(target_endian = "little")]
                {
                    x
                }
                #[cfg(not(target_endian = "little"))]
                {
                    Self::from_bits(x.to_bits().swap_bytes())
                }
            }
        }
    };
}

impl_scalar_for_float!(f32);
impl_scalar_for_float!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalar() {
        assert_eq!(true, bool::from_le(true.to_le()));
        assert_eq!(false, bool::from_le(false.to_le()));
        assert_eq!(1u8, u8::from_le(1u8.to_le()));
        assert_eq!(1u16, u16::from_le(1u16.to_le()));
        #[allow(clippy::float_cmp)]
        {
            assert_eq!(1f32, f32::from_le(1f32.to_le()));
        }

        assert_eq!(1u8, 1u8.to_le());

        #[cfg(target_endian = "little")]
        {
            assert_eq!(1u16, 1u16.to_le());
            assert_eq!(1u32, 1u32.to_le());
        }
        #[cfg(not(target_endian = "little"))]
        {
            assert_eq!(1u16.swap_bytes(), 1u16.to_le());
            assert_eq!(1u32.swap_bytes(), 1u32.to_le());
        }
    }
}
