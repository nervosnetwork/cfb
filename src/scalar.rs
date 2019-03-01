pub trait Scalar {
    fn to_protocol_endian(self) -> Self;
    fn from_protocol_endian(x: Self) -> Self;
}

#[repr(transparent)]
#[derive(Copy, Clone, Default, Debug, PartialOrd, PartialEq)]
pub struct ProtocolEndian<T>(T);

impl<T> ProtocolEndian<T> {
    pub fn new(inner: T) -> Self {
        ProtocolEndian(inner)
    }

    pub fn into_inner(self) -> T {
        self.0
    }

    pub fn get_ref(&self) -> &T {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: Copy> ProtocolEndian<T> {
    pub fn get(&self) -> T {
        self.0
    }
}

impl<T> Scalar for ProtocolEndian<T> {
    fn to_protocol_endian(self) -> Self {
        self
    }

    fn from_protocol_endian(x: Self) -> Self {
        x
    }
}

/// The macro `impl_scalar_for_enum` implements trait `Scalar` for enum.
///
/// The enum must specify a integer type via repr.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate cfb;
/// use cfb::scalar::Scalar;
///
/// #[repr(u16)]
/// #[derive(Copy, Clone)]
/// enum Side {
///   Left = 1,
///   Right = 2,
/// }
/// impl_scalar_for_enum!(Side, u16);
///
/// assert_eq!(1u16, Side::from_protocol_endian(Side::Left.to_protocol_endian()) as u16);
/// assert_eq!(2u16, Side::from_protocol_endian(Side::Right.to_protocol_endian()) as u16);
/// ```
#[macro_export]
macro_rules! impl_scalar_for_enum {
    ($ty:ident, $repr:ident) => {{
        use cfb::scalar::Scalar;
        use std::mem::transmute;

        impl Scalar for $ty {
            fn to_protocol_endian(self) -> Self {
                #[cfg(target_endian = "little")]
                {
                    self
                }
                #[cfg(not(target_endian = "little"))]
                {
                    unsafe { transmute((self as $repr).swap_bytes()) }
                }
            }
            fn from_protocol_endian(x: Self) -> Self {
                #[cfg(target_endian = "little")]
                {
                    x
                }
                #[cfg(not(target_endian = "little"))]
                {
                    unsafe { transmute((x as $repr).swap_bytes()) }
                }
            }
        }
    }};
}

macro_rules! impl_scalar_no_op {
    ($ty:ident) => {
        impl Scalar for $ty {
            fn to_protocol_endian(self) -> Self {
                self
            }
            fn from_protocol_endian(x: Self) -> Self {
                x
            }
        }
    };
}

impl_scalar_no_op!(bool);
impl_scalar_no_op!(i8);
impl_scalar_no_op!(u8);

macro_rules! impl_scalar_for_int {
    ($ty:ident) => {
        impl Scalar for $ty {
            fn to_protocol_endian(self) -> Self {
                self.to_le()
            }
            fn from_protocol_endian(x: Self) -> Self {
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
            fn to_protocol_endian(self) -> Self {
                #[cfg(target_endian = "little")]
                {
                    self
                }
                #[cfg(not(target_endian = "little"))]
                {
                    Self::from_bits(self.to_bits().swap_bytes())
                }
            }
            fn from_protocol_endian(x: Self) -> Self {
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
        assert_eq!(true, bool::from_protocol_endian(true.to_protocol_endian()));
        assert_eq!(false, bool::from_protocol_endian(false.to_protocol_endian()));
        assert_eq!(1u8, u8::from_protocol_endian(1u8.to_protocol_endian()));
        assert_eq!(1u16, u16::from_protocol_endian(1u16.to_protocol_endian()));
        assert_eq!(1f32, f32::from_protocol_endian(1f32.to_protocol_endian()));

        assert_eq!(1u8, 1u8.to_protocol_endian());

        #[cfg(target_endian = "little")]
            {
                assert_eq!(1u16, 1u16.to_protocol_endian());
                assert_eq!(1u32, 1u32.to_protocol_endian());
            }
        #[cfg(not(target_endian = "little"))]
            {
                assert_eq!(1u16.swap_bytes(), 1u16.to_protocol_endian());
                assert_eq!(1u32.swap_bytes(), 1u32.to_protocol_endian());
            }
    }
}
