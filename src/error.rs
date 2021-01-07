// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::error::Error as StdError;
use std::fmt;
use std::str::Utf8Error;

use quick_xml::Error as XmlError;

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
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            Error::Utf8(ref err) => Some(err),
            Error::Xml(ref err) => Some(err),
            Error::InvalidStartTag | Error::Eof => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Utf8(ref err) => fmt::Display::fmt(err, f),
            Error::Xml(ref err) => fmt::Display::fmt(err, f),
            Error::InvalidStartTag => write!(f, "the input did not begin with an rss tag"),
            Error::Eof => write!(f, "reached end of input without finding a complete channel"),
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
