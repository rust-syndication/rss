// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::error::Error as StdError;
use std::fmt;
use std::str::Utf8Error;

use quick_xml::errors::Error as XmlError;

#[derive(Debug)]
/// Errors that occur during parsing.
pub enum Error {
    /// An error while converting bytes to UTF8.
    Utf8(Utf8Error),
    /// An XML parsing error.
    Xml(XmlError),
    /// The input didn't begin with an opening `<rss>` tag.
    InvalidStartTag,
    /// The end of the input was reached without finding a complete channel element.
    Eof,
    /// An error during the web request.
    #[cfg(feature = "from_url")]
    UrlRequest(::reqwest::Error),
    /// An IO error.
    #[cfg(feature = "from_url")]
    Io(::std::io::Error),
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Utf8(ref err) => err.description(),
            Error::Xml(ref err) => err.description(),
            Error::InvalidStartTag => "the input did not begin with an rss tag",
            Error::Eof => "reached end of input without finding a complete channel",
            #[cfg(feature = "from_url")]
            Error::UrlRequest(ref err) => err.description(),
            #[cfg(feature = "from_url")]
            Error::Io(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Utf8(ref err) => Some(err),
            Error::Xml(ref err) => Some(err),
            #[cfg(feature = "from_url")]
            Error::UrlRequest(ref err) => Some(err),
            #[cfg(feature = "from_url")]
            Error::Io(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Utf8(ref err) => err.fmt(f),
            Error::Xml(ref err) => err.fmt(f),
            Error::InvalidStartTag => write!(f, "the input did not begin with an rss tag"),
            Error::Eof => write!(f, "reached end of input without finding a complete channel"),
            #[cfg(feature = "from_url")]
            Error::UrlRequest(ref err) => err.fmt(f),
            #[cfg(feature = "from_url")]
            Error::Io(ref err) => err.fmt(f),
        }
    }
}

impl From<XmlError> for Error {
    fn from(err: XmlError) -> Error {
        Error::Xml(err)
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Error {
        Error::Utf8(err)
    }
}

#[cfg(feature = "from_url")]
impl From<::reqwest::Error> for Error {
    fn from(err: ::reqwest::Error) -> Error {
        Error::UrlRequest(err)
    }
}

#[cfg(feature = "from_url")]
impl From<::std::io::Error> for Error {
    fn from(err: ::std::io::Error) -> Error {
        Error::Io(err)
    }
}
