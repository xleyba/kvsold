extern crate failure;
use failure::Fail;

use std::fmt;
use std::io;

#[derive(Fail, Debug)]
pub enum KvsError {
    #[fail(display = "{} is not a valid version.", _0)]
    InvalidVersion(u32),
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
     /// Serialization or deserialization error
     #[fail(display = "{}", _0)]
     Serde(#[cause] serde_json::Error),
     /// Removing non-existent key error
    #[fail(display = "Key not found")]
    KeyNotFound,
    #[fail(display = "An unknown error has occurred.")]
    UnknownError,
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> KvsError {
        KvsError::Io(err)
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(err: serde_json::Error) -> KvsError {
        KvsError::Serde(err)
    }
}


/// Result type for kvs
pub type Result<T> = std::result::Result<T, KvsError>;