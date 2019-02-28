use std::ffi::OsString;
use std::fmt::{self, Display};
use std::io;

pub enum Error {
    Io(io::Error),
    Utf8(OsString),
    Empty,
}

pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        match self {
            Io(err) => err.fmt(f),
            Utf8(name) => write!(
                f,
                "unsupported non-utf8 file name: {}",
                name.to_string_lossy(),
            ),
            Empty => f.write_str("no source files found"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}
