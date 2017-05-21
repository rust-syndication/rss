// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use chrono::ParseError as DateParseError;
use quick_xml::error::Error as XmlError;
use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IOError;
use std::num::ParseIntError;
use std::str::Utf8Error;
use std::string::FromUtf8Error;
use url::ParseError as UrlParseError;

#[derive(Debug)]
/// Types of errors that could occur while parsing an RSS feed.
pub enum Error {
    /// An error occured during validation
    Validation(String),
    /// An error occured while reading channel from url.
    FromUrl(String),
    /// An error occured while reading url to string.
    IO(IOError),
    /// An error occurred during the web request.
    #[cfg(feature = "from_url")]
    ReqParsing(::reqwest::Error),
    /// An error occured while parsing a str to i64.
    IntParsing(ParseIntError),
    /// An error occured during parsing dates from str.
    DateParsing(DateParseError),
    /// An error occurred while parsing a str to Url.
    UrlParsing(UrlParseError),
    /// An error occurred while converting bytes to UTF8.
    Utf8(Utf8Error),
    /// An XML parser error occurred at the specified byte offset.
    XmlParsing(XmlError, usize),
    /// An XML error occurred.
    Xml(XmlError),
    /// The end of the input was reached without finding a complete channel element.
    EOF,
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Validation(ref err) => err,
            Error::FromUrl(ref err) => err,
            Error::IO(ref err) => err.description(),
            #[cfg(feature = "from_url")]
            Error::ReqParsing(ref err) => err.description(),
            Error::IntParsing(ref err) => err.description(),
            Error::DateParsing(ref err) => err.description(),
            Error::UrlParsing(ref err) => err.description(),
            Error::Utf8(ref err) => err.description(),
            Error::XmlParsing(ref err, _) => err.description(),
            Error::Xml(ref err) => err.description(),
            Error::EOF => "reached end of input without finding a complete channel",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::IO(ref err) => Some(err),
            #[cfg(feature = "from_url")]
            Error::ReqParsing(ref err) => Some(err),
            Error::IntParsing(ref err) => Some(err),
            Error::DateParsing(ref err) => Some(err),
            Error::UrlParsing(ref err) => Some(err),
            Error::Utf8(ref err) => Some(err),
            Error::XmlParsing(ref err, _) => Some(err),
            Error::Xml(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Validation(ref err) => fmt::Display::fmt(err, f),
            Error::FromUrl(ref err) => fmt::Display::fmt(err, f),
            Error::IO(ref err) => fmt::Display::fmt(err, f),
            #[cfg(feature = "from_url")]
            Error::ReqParsing(ref err) => fmt::Display::fmt(err, f),
            Error::IntParsing(ref err) => fmt::Display::fmt(err, f),
            Error::DateParsing(ref err) => fmt::Display::fmt(err, f),
            Error::UrlParsing(ref err) => fmt::Display::fmt(err, f),
            Error::Utf8(ref err) => fmt::Display::fmt(err, f),
            Error::XmlParsing(ref err, _) => fmt::Display::fmt(err, f),
            Error::Xml(ref err) => fmt::Display::fmt(err, f),
            Error::EOF => write!(f, "reached end of input without finding a complete channel"),
        }
    }
}

impl From<(XmlError, usize)> for Error {
    fn from(err: (XmlError, usize)) -> Error {
        Error::XmlParsing(err.0, err.1)
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

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        Error::Utf8(err.utf8_error())
    }
}

impl From<UrlParseError> for Error {
    fn from(err: UrlParseError) -> Error {
        Error::UrlParsing(err)
    }
}

impl From<DateParseError> for Error {
    fn from(err: DateParseError) -> Error {
        Error::DateParsing(err)
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Error {
        Error::IntParsing(err)
    }
}

#[cfg(feature = "from_url")]
impl From<::reqwest::Error> for Error {
    fn from(err: ::reqwest::Error) -> Error {
        Error::ReqParsing(err)
    }
}


impl From<IOError> for Error {
    fn from(err: IOError) -> Error {
        Error::IO(err)
    }
}
