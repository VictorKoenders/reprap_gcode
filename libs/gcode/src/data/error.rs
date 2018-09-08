use core::num::ParseIntError;
use core::str::Utf8Error;

/// An error has occured
pub enum Error {
    NotEnoughBytes,
    ParseIntError(ParseIntError),
    Utf8Error(Utf8Error),
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Error {
        Error::ParseIntError(e)
    }
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Error {
        Error::Utf8Error(e)
    }
}
