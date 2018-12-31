use std::error;
use std::fmt;
use std::io;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// A connectivity error raised while making a request to the DNS upstream.
    Request(reqwest::Error),
    /// An IO error raised when reading the response from the DNS upstream.
    Read(reqwest::Error),
    /// An IO error raised when sending the response back to the client.
    Response(io::Error),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Request(ref err) => err.description(),
            Error::Read(ref err) => err.description(),
            Error::Response(ref err) => err.description(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Request(ref err) => err.fmt(f),
            Error::Read(ref err) => err.fmt(f),
            Error::Response(ref err) => err.fmt(f),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(original: reqwest::Error) -> Error {
        Error::Request(original)
    }
}
