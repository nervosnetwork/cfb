use super::table_field_generated as reader;
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
            Error::NonNullTerminatedString => {
                write!(f, "string is not terminated with null")
            }
        }
    }
}

impl error::Error for Error {}

pub trait Verify {
    fn verify(&self) -> Result;
}

pub mod example {
    #![allow(unused_imports)]

    use super::reader::example as reader;
    use super::{Error, Result, Verify};

    impl<'a> Verify for reader::Hero<'a> {
        fn verify(&self) -> Result {
            Ok(())
        }
    }

    impl<'a> Verify for reader::Stat<'a> {
        fn verify(&self) -> Result {
            Ok(())
        }
    }
}