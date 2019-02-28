use std::ffi::OsString;
use std::fmt::{self, Display};
use std::io;

pub enum Error {
    Io(io::Error),
    Utf8(OsString),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(err) => err.fmt(f),
            Error::Utf8(name) => write!(f, "unsupported non-utf8 file name: {}", name.to_string_lossy()),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}
