use super::ckb_generated as reader;
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

pub mod ckb {
    #![allow(unused_imports)]

    use super::reader::ckb as reader;
    pub use super::{try_follow_uoffset, Error, Result, StringVerifier, VectorVerifier, Verify};
    use flatbuffers::{self, Follow};
    pub mod protocol {
        #![allow(unused_imports)]

        use super::reader::protocol as reader;
        pub use super::{try_follow_uoffset, Error, Result, StringVerifier, VectorVerifier, Verify};
        use flatbuffers::{self, Follow};

        impl<'a> Verify for reader::AddFilter<'a> {
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

                if Self::VT_FILTER as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_FILTER) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        let filter_verifier =
                            VectorVerifier::follow(buf, try_follow_uoffset(buf, tab.loc + voffset)?);
                        filter_verifier.verify_scalar_elements(1)?;
                    }
                }

                Ok(())
            }
        }

        impl<'a> Verify for reader::Block<'a> {
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

                if Self::VT_HEADER as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_HEADER) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        if let Some(f) = self.header() {
                            f.verify()?;
                        }
                    }
                }

                if Self::VT_UNCLES as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_UNCLES) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        let uncles_verifier =
                            VectorVerifier::follow(buf, try_follow_uoffset(buf, tab.loc + voffset)?);
                        uncles_verifier.verify_reference_elements::<reader::UncleBlock>()?;
                    }
                }

                if Self::VT_COMMIT_TRANSACTIONS as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_COMMIT_TRANSACTIONS) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        let commit_transactions_verifier =
                            VectorVerifier::follow(buf, try_follow_uoffset(buf, tab.loc + voffset)?);
                        commit_transactions_verifier.verify_reference_elements::<reader::Transaction>()?;
                    }
                }

                if Self::VT_PROPOSAL_TRANSACTIONS as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_PROPOSAL_TRANSACTIONS) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        let proposal_transactions_verifier =
                            VectorVerifier::follow(buf, try_follow_uoffset(buf, tab.loc + voffset)?);
                        proposal_transactions_verifier.verify_scalar_elements(10)?;
                    }
                }

                Ok(())
            }
        }

        impl<'a> Verify for reader::BlockProposal<'a> {
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

                if Self::VT_TRANSACTIONS as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_TRANSACTIONS) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        let transactions_verifier =
                            VectorVerifier::follow(buf, try_follow_uoffset(buf, tab.loc + voffset)?);
                        transactions_verifier.verify_reference_elements::<reader::Transaction>()?;
                    }
                }

                Ok(())
            }
        }

        impl<'a> Verify for reader::BlockTransactions<'a> {
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

                if Self::VT_HASH as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_HASH) as usize;
                    if voffset > 0 && voffset + 32 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                if Self::VT_TRANSACTIONS as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_TRANSACTIONS) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        let transactions_verifier =
                            VectorVerifier::follow(buf, try_follow_uoffset(buf, tab.loc + voffset)?);
                        transactions_verifier.verify_reference_elements::<reader::Transaction>()?;
                    }
                }

                Ok(())
            }
        }

        impl<'a> Verify for reader::Bytes<'a> {
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

                if Self::VT_SEQ as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_SEQ) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        let seq_verifier =
                            VectorVerifier::follow(buf, try_follow_uoffset(buf, tab.loc + voffset)?);
                        seq_verifier.verify_scalar_elements(1)?;
                    }
                }

                Ok(())
            }
        }

        impl<'a> Verify for reader::CellInput<'a> {
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

                if Self::VT_HASH as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_HASH) as usize;
                    if voffset > 0 && voffset + 32 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                if Self::VT_INDEX as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_INDEX) as usize;
                    if voffset > 0 && voffset + 4 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                if Self::VT_ARGS as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_ARGS) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        let args_verifier =
                            VectorVerifier::follow(buf, try_follow_uoffset(buf, tab.loc + voffset)?);
                        args_verifier.verify_reference_elements::<reader::Bytes>()?;
                    }
                }

                Ok(())
            }
        }

        impl<'a> Verify for reader::CellOutput<'a> {
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

                if Self::VT_CAPACITY as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_CAPACITY) as usize;
                    if voffset > 0 && voffset + 8 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                if Self::VT_DATA as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_DATA) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        if let Some(f) = self.data() {
                            f.verify()?;
                        }
                    }
                }

                if Self::VT_LOCK as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_LOCK) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        if let Some(f) = self.lock() {
                            f.verify()?;
                        }
                    }
                }

                if Self::VT_TYPE_ as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_TYPE_) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        if let Some(f) = self.type_() {
                            f.verify()?;
                        }
                    }
                }

                Ok(())
            }
        }

        impl<'a> Verify for reader::ClearFilter<'a> {
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

                Ok(())
            }
        }

        impl<'a> Verify for reader::CompactBlock<'a> {
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

                if Self::VT_HEADER as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_HEADER) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        if let Some(f) = self.header() {
                            f.verify()?;
                        }
                    }
                }

                if Self::VT_NONCE as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_NONCE) as usize;
                    if voffset > 0 && voffset + 8 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                if Self::VT_SHORT_IDS as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_SHORT_IDS) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        let short_ids_verifier =
                            VectorVerifier::follow(buf, try_follow_uoffset(buf, tab.loc + voffset)?);
                        short_ids_verifier.verify_reference_elements::<reader::Bytes>()?;
                    }
                }

                if Self::VT_PREFILLED_TRANSACTIONS as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_PREFILLED_TRANSACTIONS) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        let prefilled_transactions_verifier =
                            VectorVerifier::follow(buf, try_follow_uoffset(buf, tab.loc + voffset)?);
                        prefilled_transactions_verifier.verify_reference_elements::<reader::IndexTransaction>()?;
                    }
                }

                if Self::VT_UNCLES as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_UNCLES) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        let uncles_verifier =
                            VectorVerifier::follow(buf, try_follow_uoffset(buf, tab.loc + voffset)?);
                        uncles_verifier.verify_reference_elements::<reader::UncleBlock>()?;
                    }
                }

                if Self::VT_PROPOSAL_TRANSACTIONS as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_PROPOSAL_TRANSACTIONS) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        let proposal_transactions_verifier =
                            VectorVerifier::follow(buf, try_follow_uoffset(buf, tab.loc + voffset)?);
                        proposal_transactions_verifier.verify_scalar_elements(10)?;
                    }
                }

                Ok(())
            }
        }

        impl<'a> Verify for reader::FilteredBlock<'a> {
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

                if Self::VT_HEADER as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_HEADER) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        if let Some(f) = self.header() {
                            f.verify()?;
                        }
                    }
                }

                if Self::VT_TRANSACTIONS as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_TRANSACTIONS) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        let transactions_verifier =
                            VectorVerifier::follow(buf, try_follow_uoffset(buf, tab.loc + voffset)?);
                        transactions_verifier.verify_reference_elements::<reader::Transaction>()?;
                    }
                }

                if Self::VT_PROOF as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_PROOF) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        if let Some(f) = self.proof() {
                            f.verify()?;
                        }
                    }
                }

                Ok(())
            }
        }

        impl<'a> Verify for reader::GetBlockProposal<'a> {
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

                if Self::VT_BLOCK_NUMBER as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_BLOCK_NUMBER) as usize;
                    if voffset > 0 && voffset + 8 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                if Self::VT_PROPOSAL_TRANSACTIONS as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_PROPOSAL_TRANSACTIONS) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        let proposal_transactions_verifier =
                            VectorVerifier::follow(buf, try_follow_uoffset(buf, tab.loc + voffset)?);
                        proposal_transactions_verifier.verify_scalar_elements(10)?;
                    }
                }

                Ok(())
            }
        }

        impl<'a> Verify for reader::GetBlockTransactions<'a> {
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

                if Self::VT_HASH as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_HASH) as usize;
                    if voffset > 0 && voffset + 32 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                if Self::VT_INDEXES as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_INDEXES) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        let indexes_verifier =
                            VectorVerifier::follow(buf, try_follow_uoffset(buf, tab.loc + voffset)?);
                        indexes_verifier.verify_scalar_elements(4)?;
                    }
                }

                Ok(())
            }
        }

        impl<'a> Verify for reader::GetBlocks<'a> {
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

                if Self::VT_BLOCK_HASHES as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_BLOCK_HASHES) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        let block_hashes_verifier =
                            VectorVerifier::follow(buf, try_follow_uoffset(buf, tab.loc + voffset)?);
                        block_hashes_verifier.verify_scalar_elements(32)?;
                    }
                }

                Ok(())
            }
        }

        impl<'a> Verify for reader::GetHeaders<'a> {
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

                if Self::VT_VERSION as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_VERSION) as usize;
                    if voffset > 0 && voffset + 4 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                if Self::VT_BLOCK_LOCATOR_HASHES as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_BLOCK_LOCATOR_HASHES) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        let block_locator_hashes_verifier =
                            VectorVerifier::follow(buf, try_follow_uoffset(buf, tab.loc + voffset)?);
                        block_locator_hashes_verifier.verify_scalar_elements(32)?;
                    }
                }

                if Self::VT_HASH_STOP as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_HASH_STOP) as usize;
                    if voffset > 0 && voffset + 32 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                Ok(())
            }
        }

        impl<'a> Verify for reader::Header<'a> {
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

                if Self::VT_VERSION as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_VERSION) as usize;
                    if voffset > 0 && voffset + 4 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                if Self::VT_PARENT_HASH as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_PARENT_HASH) as usize;
                    if voffset > 0 && voffset + 32 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                if Self::VT_TIMESTAMP as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_TIMESTAMP) as usize;
                    if voffset > 0 && voffset + 8 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                if Self::VT_NUMBER as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_NUMBER) as usize;
                    if voffset > 0 && voffset + 8 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                if Self::VT_TXS_COMMIT as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_TXS_COMMIT) as usize;
                    if voffset > 0 && voffset + 32 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                if Self::VT_TXS_PROPOSAL as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_TXS_PROPOSAL) as usize;
                    if voffset > 0 && voffset + 32 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                if Self::VT_DIFFICULTY as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_DIFFICULTY) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        if let Some(f) = self.difficulty() {
                            f.verify()?;
                        }
                    }
                }

                if Self::VT_NONCE as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_NONCE) as usize;
                    if voffset > 0 && voffset + 8 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                if Self::VT_PROOF as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_PROOF) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        if let Some(f) = self.proof() {
                            f.verify()?;
                        }
                    }
                }

                if Self::VT_CELLBASE_ID as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_CELLBASE_ID) as usize;
                    if voffset > 0 && voffset + 32 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                if Self::VT_UNCLES_HASH as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_UNCLES_HASH) as usize;
                    if voffset > 0 && voffset + 32 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                if Self::VT_UNCLES_COUNT as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_UNCLES_COUNT) as usize;
                    if voffset > 0 && voffset + 4 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                Ok(())
            }
        }

        impl<'a> Verify for reader::Headers<'a> {
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

                if Self::VT_HEADERS as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_HEADERS) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        let headers_verifier =
                            VectorVerifier::follow(buf, try_follow_uoffset(buf, tab.loc + voffset)?);
                        headers_verifier.verify_reference_elements::<reader::Header>()?;
                    }
                }

                Ok(())
            }
        }

        impl<'a> Verify for reader::IndexTransaction<'a> {
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

                if Self::VT_INDEX as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_INDEX) as usize;
                    if voffset > 0 && voffset + 4 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                if Self::VT_TRANSACTION as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_TRANSACTION) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        if let Some(f) = self.transaction() {
                            f.verify()?;
                        }
                    }
                }

                Ok(())
            }
        }

        impl<'a> Verify for reader::MerkleProof<'a> {
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

                if Self::VT_INDICES as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_INDICES) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        let indices_verifier =
                            VectorVerifier::follow(buf, try_follow_uoffset(buf, tab.loc + voffset)?);
                        indices_verifier.verify_scalar_elements(4)?;
                    }
                }

                if Self::VT_LEMMAS as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_LEMMAS) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        let lemmas_verifier =
                            VectorVerifier::follow(buf, try_follow_uoffset(buf, tab.loc + voffset)?);
                        lemmas_verifier.verify_scalar_elements(32)?;
                    }
                }

                Ok(())
            }
        }

        impl<'a> Verify for reader::OutPoint<'a> {
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

                if Self::VT_HASH as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_HASH) as usize;
                    if voffset > 0 && voffset + 32 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                if Self::VT_INDEX as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_INDEX) as usize;
                    if voffset > 0 && voffset + 4 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                Ok(())
            }
        }

        impl<'a> Verify for reader::RelayMessage<'a> {
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

                if Self::VT_PAYLOAD_TYPE as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_PAYLOAD_TYPE) as usize;
                    if voffset > 0 && voffset + 1 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                if Self::VT_PAYLOAD as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_PAYLOAD) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        match self.payload_type() {
                            reader::RelayPayload::CompactBlock => self
                                .payload_as_compact_block()
                                .ok_or(Error::UnmatchedUnion)?
                                .verify()?,
                            reader::RelayPayload::ValidTransaction => self
                                .payload_as_valid_transaction()
                                .ok_or(Error::UnmatchedUnion)?
                                .verify()?,
                            reader::RelayPayload::GetBlockTransactions => self
                                .payload_as_get_block_transactions()
                                .ok_or(Error::UnmatchedUnion)?
                                .verify()?,
                            reader::RelayPayload::BlockTransactions => self
                                .payload_as_block_transactions()
                                .ok_or(Error::UnmatchedUnion)?
                                .verify()?,
                            reader::RelayPayload::GetBlockProposal => self
                                .payload_as_get_block_proposal()
                                .ok_or(Error::UnmatchedUnion)?
                                .verify()?,
                            reader::RelayPayload::BlockProposal => self
                                .payload_as_block_proposal()
                                .ok_or(Error::UnmatchedUnion)?
                                .verify()?,
                            reader::RelayPayload::NONE => return Err(Error::UnmatchedUnion),
                        }
                    }
                }

                Ok(())
            }
        }

        impl<'a> Verify for reader::Script<'a> {
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

                if Self::VT_VERSION as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_VERSION) as usize;
                    if voffset > 0 && voffset + 1 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                if Self::VT_ARGS as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_ARGS) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        let args_verifier =
                            VectorVerifier::follow(buf, try_follow_uoffset(buf, tab.loc + voffset)?);
                        args_verifier.verify_reference_elements::<reader::Bytes>()?;
                    }
                }

                if Self::VT_BINARY_HASH as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_BINARY_HASH) as usize;
                    if voffset > 0 && voffset + 32 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                Ok(())
            }
        }

        impl<'a> Verify for reader::SetFilter<'a> {
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

                if Self::VT_FILTER as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_FILTER) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        let filter_verifier =
                            VectorVerifier::follow(buf, try_follow_uoffset(buf, tab.loc + voffset)?);
                        filter_verifier.verify_scalar_elements(1)?;
                    }
                }

                if Self::VT_NUM_HASHES as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_NUM_HASHES) as usize;
                    if voffset > 0 && voffset + 1 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                if Self::VT_HASH_SEED as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_HASH_SEED) as usize;
                    if voffset > 0 && voffset + 4 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                Ok(())
            }
        }

        impl<'a> Verify for reader::SyncMessage<'a> {
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

                if Self::VT_PAYLOAD_TYPE as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_PAYLOAD_TYPE) as usize;
                    if voffset > 0 && voffset + 1 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                if Self::VT_PAYLOAD as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_PAYLOAD) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        match self.payload_type() {
                            reader::SyncPayload::GetHeaders => self
                                .payload_as_get_headers()
                                .ok_or(Error::UnmatchedUnion)?
                                .verify()?,
                            reader::SyncPayload::Headers => self
                                .payload_as_headers()
                                .ok_or(Error::UnmatchedUnion)?
                                .verify()?,
                            reader::SyncPayload::GetBlocks => self
                                .payload_as_get_blocks()
                                .ok_or(Error::UnmatchedUnion)?
                                .verify()?,
                            reader::SyncPayload::Block => self
                                .payload_as_block()
                                .ok_or(Error::UnmatchedUnion)?
                                .verify()?,
                            reader::SyncPayload::SetFilter => self
                                .payload_as_set_filter()
                                .ok_or(Error::UnmatchedUnion)?
                                .verify()?,
                            reader::SyncPayload::AddFilter => self
                                .payload_as_add_filter()
                                .ok_or(Error::UnmatchedUnion)?
                                .verify()?,
                            reader::SyncPayload::ClearFilter => self
                                .payload_as_clear_filter()
                                .ok_or(Error::UnmatchedUnion)?
                                .verify()?,
                            reader::SyncPayload::FilteredBlock => self
                                .payload_as_filtered_block()
                                .ok_or(Error::UnmatchedUnion)?
                                .verify()?,
                            reader::SyncPayload::NONE => return Err(Error::UnmatchedUnion),
                        }
                    }
                }

                Ok(())
            }
        }

        impl<'a> Verify for reader::Time<'a> {
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

                if Self::VT_TIMESTAMP as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_TIMESTAMP) as usize;
                    if voffset > 0 && voffset + 8 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                Ok(())
            }
        }

        impl<'a> Verify for reader::TimeMessage<'a> {
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

                if Self::VT_PAYLOAD as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_PAYLOAD) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        if let Some(f) = self.payload() {
                            f.verify()?;
                        }
                    }
                }

                Ok(())
            }
        }

        impl<'a> Verify for reader::Transaction<'a> {
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

                if Self::VT_VERSION as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_VERSION) as usize;
                    if voffset > 0 && voffset + 4 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                if Self::VT_DEPS as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_DEPS) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        let deps_verifier =
                            VectorVerifier::follow(buf, try_follow_uoffset(buf, tab.loc + voffset)?);
                        deps_verifier.verify_reference_elements::<reader::OutPoint>()?;
                    }
                }

                if Self::VT_INPUTS as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_INPUTS) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        let inputs_verifier =
                            VectorVerifier::follow(buf, try_follow_uoffset(buf, tab.loc + voffset)?);
                        inputs_verifier.verify_reference_elements::<reader::CellInput>()?;
                    }
                }

                if Self::VT_OUTPUTS as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_OUTPUTS) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        let outputs_verifier =
                            VectorVerifier::follow(buf, try_follow_uoffset(buf, tab.loc + voffset)?);
                        outputs_verifier.verify_reference_elements::<reader::CellOutput>()?;
                    }
                }

                if Self::VT_EMBEDS as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_EMBEDS) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        let embeds_verifier =
                            VectorVerifier::follow(buf, try_follow_uoffset(buf, tab.loc + voffset)?);
                        embeds_verifier.verify_reference_elements::<reader::Bytes>()?;
                    }
                }

                Ok(())
            }
        }

        impl<'a> Verify for reader::UncleBlock<'a> {
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

                if Self::VT_HEADER as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_HEADER) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        if let Some(f) = self.header() {
                            f.verify()?;
                        }
                    }
                }

                if Self::VT_CELLBASE as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_CELLBASE) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        if let Some(f) = self.cellbase() {
                            f.verify()?;
                        }
                    }
                }

                if Self::VT_PROPOSAL_TRANSACTIONS as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_PROPOSAL_TRANSACTIONS) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        let proposal_transactions_verifier =
                            VectorVerifier::follow(buf, try_follow_uoffset(buf, tab.loc + voffset)?);
                        proposal_transactions_verifier.verify_scalar_elements(10)?;
                    }
                }

                Ok(())
            }
        }

        impl<'a> Verify for reader::ValidTransaction<'a> {
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

                if Self::VT_CYCLES as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_CYCLES) as usize;
                    if voffset > 0 && voffset + 8 > object_inline_num_bytes {
                        return Err(Error::OutOfBounds);
                    }
                }

                if Self::VT_TRANSACTION as usize + flatbuffers::SIZE_VOFFSET <= vtab_num_bytes {
                    let voffset = vtab.get(Self::VT_TRANSACTION) as usize;
                    if voffset > 0 {
                        if voffset + 4 > object_inline_num_bytes {
                            return Err(Error::OutOfBounds);
                        }

                        if let Some(f) = self.transaction() {
                            f.verify()?;
                        }
                    }
                }

                Ok(())
            }
        }
    }

}