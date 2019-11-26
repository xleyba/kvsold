extern crate failure;
#[macro_use] extern crate failure_derive;

use std::fmt;

#[derive(Fail, Debug)]
enum MyError {
    #[fail(display = "{} is not a valid version.", _0)]
    InvalidVersion(u32),
    #[fail(display = "IO error: {}", error)]
    IoError { error: io::Error },
    #[fail(display = "An unknown error has occurred.")]
    UnknownError,
}

/// Result type for kvs
pub type Result<T> = std::result::Result<T, KvsError>;