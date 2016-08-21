use std::fmt;
use std::error::Error as StdError;
use std::str::Utf8Error;

#[cfg(feature = "quick-xml")]
use quick_xml::error::Error as XmlError;
#[cfg(not(feature = "quick-xml"))]
use xml::reader::Error as XmlError;

#[derive(Debug)]
/// An enumration containing the types of errors that could occur while parsing an RSS feed.
pub enum Error {
    /// A required field was missing.
    MissingField(&'static str, &'static str),
    /// A field was the wrong type.
    InvalidField(&'static str, &'static str),
    /// An error occurred while converting bytes to UTF8"
    Utf8(Utf8Error),
    /// An XML parser error occurred.
    XmlParsing(XmlError),
    /// No <channel> element was found.
    NotFound,
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::MissingField(_, _) => "a required field is missing",
            Error::InvalidField(_, _) => "a field was the wrong type",
            Error::Utf8(ref err) => err.description(),
            Error::XmlParsing(ref err) => err.description(),
            Error::NotFound => "no channel element was found",
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
            Error::MissingField(ty, field) => {
                write!(f, "`{}` is missing the required field `{}`", ty, field)
            }
            Error::InvalidField(ty, field) => {
                write!(f,
                       "`{}` contains an invalid type for the field `{}`",
                       ty,
                       field)
            }
            Error::Utf8(ref err) => fmt::Display::fmt(err, f),
            Error::XmlParsing(ref err) => fmt::Display::fmt(err, f),
            Error::NotFound => write!(f, "no channel element was found"),
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

