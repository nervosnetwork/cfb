use flatbuffers;
use super::string_generated as reader;
use std::error;
use std::fmt;
use std::result;

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    OutOfBounds,
    NonNullTerminatedString,
}

pub type Result = result::Result<(), Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::OutOfBounds => write!(f, "memory access is out of bounds"),
            Error::NonNullTerminatedString => write!(f, "string is not terminated with null"),
        }
    }
}

impl error::Error for Error {}

pub trait Verify {
    fn verify(&self) -> Result;
}

pub struct StringVerifier<'a> {
    pub buf: &'a [u8],
    pub offset_loc: usize,
}

impl<'a> StringVerifier<'a> {
    pub fn new(buf: &'a [u8], offset_loc: usize) -> Self {
        Self { buf, offset_loc }
    }
}

impl<'a> Verify for StringVerifier<'a> {
    fn verify(&self) -> Result {
        let buf_len = self.buf.len();

        if self.offset_loc + flatbuffers::SIZE_UOFFSET > buf_len {
            return Err(Error::OutOfBounds);
        }

        let loc = self.offset_loc
            + flatbuffers::read_scalar::<flatbuffers::UOffsetT>(&self.buf[self.offset_loc..])
                as usize;
        if loc + flatbuffers::SIZE_UOFFSET > buf_len {
            return Err(Error::OutOfBounds);
        }
        let len = flatbuffers::read_scalar::<flatbuffers::UOffsetT>(&self.buf[loc..]) as usize;
        if loc + flatbuffers::SIZE_UOFFSET + len + 1 > buf_len {
            return Err(Error::OutOfBounds);
        }

        if self.buf[loc + flatbuffers::SIZE_UOFFSET + len] != 0 {
            return Err(Error::NonNullTerminatedString);
        }

        Ok(())
    }
}

pub fn get_root<'a, T>(data: &'a [u8]) -> result::Result<T::Inner, Error>
where
    T: flatbuffers::Follow<'a> + 'a,
    T::Inner: Verify,
{
    if data.len() < flatbuffers::SIZE_UOFFSET {
        return Err(Error::OutOfBounds);
    }

    let root = flatbuffers::get_root::<T>(data);
    root.verify()?;
    Ok(root)
}

pub mod example {
    #![allow(unused_imports)]

    use super::reader::example as reader;
    pub use super::{Error, Result, Verify, StringVerifier};
    use flatbuffers;

    impl<'a> Verify for reader::Author<'a> {
        fn verify(&self) -> Result {
            let tab = self._tab;
            let buf = tab.buf;
            let buf_len = buf.len();

            if tab.loc + flatbuffers::SIZE_SOFFSET > buf_len {
                return Err(Error::OutOfBounds);
            }

            let vtab_loc = {
                let soffset_slice = &buf[tab.loc..tab.loc + flatbuffers::SIZE_SOFFSET];
                let soffset = flatbuffers::read_scalar::<flatbuffers::SOffsetT>(soffset_slice);
                (tab.loc as flatbuffers::SOffsetT - soffset) as usize
            };
            if vtab_loc + flatbuffers::SIZE_VOFFSET + flatbuffers::SIZE_VOFFSET > buf_len {
                return Err(Error::OutOfBounds);
            }

            let vtab = tab.vtable();
            let vtab_num_bytes = vtab.num_bytes();
            if vtab_loc + vtab_num_bytes > buf_len {
                return Err(Error::OutOfBounds);
            }
            let object_inline_num_bytes = vtab.object_inline_num_bytes();
            if tab.loc + object_inline_num_bytes > buf_len {
                return Err(Error::OutOfBounds);
            }
            for i in 0..vtab.num_fields() {
                let voffset = vtab.get_field(i) as usize;
                if voffset < flatbuffers::SIZE_SOFFSET || voffset >= object_inline_num_bytes {
                    return Err(Error::OutOfBounds);
                }
            }
            if Self::VT_NAME as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                let voffset = vtab.get(Self::VT_NAME) as usize;
                if voffset > 0 {
                    if voffset + 4 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }

                    {
                        let offset_loc = tab.loc + voffset;
                        StringVerifier::new(buf, offset_loc).verify()?;
                    }
                }
            }

            Ok(())
        }
    }
}
