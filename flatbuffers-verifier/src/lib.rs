use flatbuffers::{read_scalar, Follow, UOffsetT, SIZE_SIZEPREFIX, SIZE_UOFFSET};
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

pub const MAX_OFFSET_LOC: usize = usize::max_value() - SIZE_UOFFSET;

fn read_uoffset(buf: &[u8], offset_loc: usize) -> usize {
    read_scalar::<UOffsetT>(&buf[offset_loc..]) as usize
}

fn try_read_uoffset(buf: &[u8], offset_loc: usize) -> result::Result<usize, Error> {
    if offset_loc <= MAX_OFFSET_LOC && offset_loc + SIZE_UOFFSET <= buf.len() {
        Ok(read_uoffset(buf, offset_loc))
    } else {
        Err(Error::OutOfBounds)
    }
}

pub fn try_follow_uoffset(buf: &[u8], offset_loc: usize) -> result::Result<usize, Error> {
    try_read_uoffset(buf, offset_loc)
        .and_then(|offset| offset_loc.checked_add(offset).ok_or(Error::OutOfBounds))
}

#[allow(dead_code)]
pub struct StringVerifier<'a> {
    buf: &'a [u8],
    loc: usize,
}

impl<'a> Follow<'a> for StringVerifier<'a> {
    type Inner = Self;
    fn follow(buf: &'a [u8], loc: usize) -> Self {
        Self { buf, loc }
    }
}

impl<'a> Verify for StringVerifier<'a> {
    fn verify(&self) -> Result {
        let buf_len = self.buf.len();

        let len = try_read_uoffset(self.buf, self.loc)?;
        let null_loc = (self.loc + SIZE_UOFFSET)
            .checked_add(len)
            .ok_or(Error::OutOfBounds)?;

        if null_loc >= buf_len {
            return Err(Error::OutOfBounds);
        }
        if self.buf[null_loc] != 0 {
            return Err(Error::NonNullTerminatedString);
        }

        Ok(())
    }
}

#[allow(dead_code)]
pub struct VectorVerifier<'a> {
    buf: &'a [u8],
    loc: usize,
}

impl<'a> Follow<'a> for VectorVerifier<'a> {
    type Inner = Self;
    fn follow(buf: &'a [u8], loc: usize) -> Self {
        Self { buf, loc }
    }
}

impl<'a> VectorVerifier<'a> {
    pub fn verify_scalar_elements(&self, scalar_size: usize) -> Result {
        let len = try_read_uoffset(self.buf, self.loc)?;

        match (self.loc + SIZE_UOFFSET)
            .checked_add(len * scalar_size)
            .filter(|loc| *loc <= self.buf.len())
        {
            Some(_) => Ok(()),
            _ => Err(Error::OutOfBounds),
        }
    }

    pub fn verify_reference_elements<E>(&self) -> Result
    where
        E: Follow<'a>,
        <E as Follow<'a>>::Inner: Verify,
    {
        let len = try_read_uoffset(self.buf, self.loc)?;

        let mut offset_loc = self.loc + SIZE_UOFFSET;
        let end_loc = offset_loc
            .checked_add(len * SIZE_UOFFSET)
            .ok_or(Error::OutOfBounds)?;
        if end_loc > self.buf.len() {
            return Err(Error::OutOfBounds);
        }

        while offset_loc < end_loc {
            E::follow(
                self.buf,
                offset_loc
                    .checked_add(read_uoffset(self.buf, offset_loc))
                    .ok_or(Error::OutOfBounds)?,
            )
            .verify()?;
            offset_loc += SIZE_UOFFSET;
        }

        Ok(())
    }
}

pub fn get_root<'a, T>(data: &'a [u8]) -> result::Result<T::Inner, Error>
where
    T: Follow<'a> + 'a,
    T::Inner: Verify,
{
    if data.len() < SIZE_UOFFSET {
        return Err(Error::OutOfBounds);
    }

    let root = flatbuffers::get_root::<T>(data);
    root.verify()?;
    Ok(root)
}

pub fn get_size_prefixed_root<'a, T>(data: &'a [u8]) -> result::Result<T::Inner, Error>
where
    T: Follow<'a> + 'a,
    T::Inner: Verify,
{
    if data.len() < SIZE_SIZEPREFIX + SIZE_UOFFSET {
        return Err(Error::OutOfBounds);
    }

    let root = flatbuffers::get_size_prefixed_root::<T>(data);
    root.verify()?;
    Ok(root)
}
