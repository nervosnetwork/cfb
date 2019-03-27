use super::table_fields_order_generated as reader;
use flatbuffers;
use std::error;
use std::fmt;
use std::result;

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    OutOfBounds,
    NonNullTerminatedString,
    UnmatchedUnion,
}

pub type Result = result::Result<(), Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::OutOfBounds => write!(f, "memory access is out of bounds"),
            Error::NonNullTerminatedString => write!(f, "string is not terminated with null"),
            Error::UnmatchedUnion => write!(f, "union type and value does not match"),
        }
    }
}

impl error::Error for Error {}

pub trait Verify {
    fn verify(&self) -> Result;
}

fn read_uoffset(buf: &[u8], offset_loc: usize) -> usize {
    flatbuffers::read_scalar::<flatbuffers::UOffsetT>(&buf[offset_loc..]) as usize
}

fn try_read_uoffset(buf: &[u8], offset_loc: usize) -> result::Result<usize, Error> {
    if offset_loc + flatbuffers::SIZE_UOFFSET <= buf.len() {
        Ok(read_uoffset(buf, offset_loc))
    } else {
        Err(Error::OutOfBounds)
    }
}

pub fn try_follow_uoffset(buf: &[u8], offset_loc: usize) -> result::Result<usize, Error> {
    try_read_uoffset(buf, offset_loc).map(|offset| offset_loc + offset)
}

pub struct StringVerifier<'a> {
    buf: &'a [u8],
    loc: usize,
}

impl<'a> flatbuffers::Follow<'a> for StringVerifier<'a> {
    type Inner = Self;
    fn follow(buf: &'a [u8], loc: usize) -> Self {
        Self { buf, loc }
    }
}

impl<'a> Verify for StringVerifier<'a> {
    fn verify(&self) -> Result {
        let buf_len = self.buf.len();

        let len = try_read_uoffset(self.buf, self.loc)?;
        if self.loc + flatbuffers::SIZE_UOFFSET + len + 1 > buf_len {
            return Err(Error::OutOfBounds);
        }

        if self.buf[self.loc + flatbuffers::SIZE_UOFFSET + len] != 0 {
            return Err(Error::NonNullTerminatedString);
        }

        Ok(())
    }
}

pub struct VectorVerifier<'a> {
    buf: &'a [u8],
    loc: usize,
}

impl<'a> flatbuffers::Follow<'a> for VectorVerifier<'a> {
    type Inner = Self;
    fn follow(buf: &'a [u8], loc: usize) -> Self {
        Self { buf, loc }
    }
}

impl<'a> VectorVerifier<'a> {
    pub fn verify_scalar_elements(&self, scalar_size: usize) -> Result {
        let len = try_read_uoffset(self.buf, self.loc)?;

        if self.loc + flatbuffers::SIZE_UOFFSET + len * scalar_size > self.buf.len() {
            return Err(Error::OutOfBounds);
        }

        Ok(())
    }

    pub fn verify_reference_elements<E>(&self) -> Result
    where
        E: flatbuffers::Follow<'a>,
        <E as flatbuffers::Follow<'a>>::Inner: Verify,
    {
        let len = try_read_uoffset(self.buf, self.loc)?;

        let mut offset_loc = self.loc + flatbuffers::SIZE_UOFFSET;
        let end_loc = offset_loc + len * flatbuffers::SIZE_UOFFSET;
        if end_loc > self.buf.len() {
            return Err(Error::OutOfBounds);
        }

        while offset_loc < end_loc {
            E::follow(self.buf, offset_loc + read_uoffset(self.buf, offset_loc)).verify()?;
            offset_loc += flatbuffers::SIZE_UOFFSET;
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


impl<'a> Verify for reader::Err<'a> {
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
            if (voffset > 0 && voffset < flatbuffers::SIZE_SOFFSET)
                || voffset >= object_inline_num_bytes
            {
                return Err(Error::OutOfBounds);
            }
        }

        if Self::VT_REASON as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
            let voffset = vtab.get(Self::VT_REASON) as usize;
            if voffset > 0 {
                if voffset + 4 > object_inline_num_bytes {
                    return Err(Error::OutOfBounds);
                }

                StringVerifier::follow(buf, try_follow_uoffset(buf, tab.loc + voffset)?).verify()?;
            }
        }

        Ok(())
    }
}

impl<'a> Verify for reader::Ok<'a> {
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
            if (voffset > 0 && voffset < flatbuffers::SIZE_SOFFSET)
                || voffset >= object_inline_num_bytes
            {
                return Err(Error::OutOfBounds);
            }
        }

        if Self::VT_VALUE as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
            let voffset = vtab.get(Self::VT_VALUE) as usize;
            if voffset > 0 && voffset + 4 > object_inline_num_bytes {
                return Err(Error::OutOfBounds);
            }
        }

        Ok(())
    }
}

impl<'a> Verify for reader::T<'a> {
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
            if (voffset > 0 && voffset < flatbuffers::SIZE_SOFFSET)
                || voffset >= object_inline_num_bytes
            {
                return Err(Error::OutOfBounds);
            }
        }

        if Self::VT_A_UBYTE as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
            let voffset = vtab.get(Self::VT_A_UBYTE) as usize;
            if voffset > 0 && voffset + 1 > object_inline_num_bytes {
                return Err(Error::OutOfBounds);
            }
        }

        if Self::VT_COMPLEX as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
            let voffset = vtab.get(Self::VT_COMPLEX) as usize;
            if voffset > 0 && voffset + 16 > object_inline_num_bytes {
                return Err(Error::OutOfBounds);
            }
        }

        if Self::VT_A_UINT32 as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
            let voffset = vtab.get(Self::VT_A_UINT32) as usize;
            if voffset > 0 && voffset + 4 > object_inline_num_bytes {
                return Err(Error::OutOfBounds);
            }
        }

        if Self::VT_RESULT_TYPE as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
            let voffset = vtab.get(Self::VT_RESULT_TYPE) as usize;
            if voffset > 0 && voffset + 1 > object_inline_num_bytes {
                return Err(Error::OutOfBounds);
            }
        }

        if Self::VT_RESULT as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
            let voffset = vtab.get(Self::VT_RESULT) as usize;
            if voffset > 0 {
                if voffset + 4 > object_inline_num_bytes {
                    return Err(Error::OutOfBounds);
                }

                match self.result_type() {
                    reader::Result::Ok => self
                        .result_as_ok()
                        .ok_or(Error::UnmatchedUnion)?
                        .verify()?,
                    reader::Result::Err => self
                        .result_as_err()
                        .ok_or(Error::UnmatchedUnion)?
                        .verify()?,
                    reader::Result::NONE => return Err(Error::UnmatchedUnion),
                }
            }
        }

        if Self::VT_A_UINT64 as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
            let voffset = vtab.get(Self::VT_A_UINT64) as usize;
            if voffset > 0 && voffset + 8 > object_inline_num_bytes {
                return Err(Error::OutOfBounds);
            }
        }

        if Self::VT_UINT16_ARRAY as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
            let voffset = vtab.get(Self::VT_UINT16_ARRAY) as usize;
            if voffset > 0 {
                if voffset + 4 > object_inline_num_bytes {
                    return Err(Error::OutOfBounds);
                }

                let uint16_array_verifier =
                    VectorVerifier::follow(buf, try_follow_uoffset(buf, tab.loc + voffset)?);
                uint16_array_verifier.verify_scalar_elements(2)?;
            }
        }

        if Self::VT_COLOR as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
            let voffset = vtab.get(Self::VT_COLOR) as usize;
            if voffset > 0 && voffset + 1 > object_inline_num_bytes {
                return Err(Error::OutOfBounds);
            }
        }

        Ok(())
    }
}