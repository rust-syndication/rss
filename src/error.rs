use std::fmt;
use std::error::Error as StdError;
use std::str::Utf8Error;
use std::string::FromUtf8Error;

use quick_xml::error::Error as XmlError;

#[derive(Debug)]
/// An enumration containing the types of errors that could occur while parsing an RSS feed.
pub enum Error {
    /// An error occurred while converting bytes to UTF8"
    Utf8(Utf8Error),
    /// An XML parser error occurred.
    XmlParsing(XmlError),
    /// The end of the input was reached without finding a complete channel element.
    EOF,
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Utf8(ref err) => err.description(),
            Error::XmlParsing(ref err) => err.description(),
            Error::EOF => "reached end of input without finding a complete channel",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Utf8(ref err) => Some(err),
            Error::XmlParsing(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Utf8(ref err) => fmt::Display::fmt(err, f),
            Error::XmlParsing(ref err) => fmt::Display::fmt(err, f),
            Error::EOF => write!(f, "reached end of input without finding a complete channel"),
        }
    }
}

impl From<XmlError> for Error {
    fn from(err: XmlError) -> Error {
        Error::XmlParsing(err)
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Error {
        Error::Utf8(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        Error::Utf8(err.utf8_error())
    }
}
