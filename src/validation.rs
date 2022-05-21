// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::collections::HashSet;
use std::error::Error as StdError;
use std::fmt;
use std::num::ParseIntError;

use chrono::DateTime;
use chrono::ParseError as DateParseError;
use mime::FromStrError as MimeParseError;
use mime::Mime;
use url::ParseError as UrlParseError;
use url::Url;

use crate::{Category, Channel, Cloud, Enclosure, Image, Item, Source, TextInput};

#[derive(Debug)]
/// Errors that occur during validation.
pub enum ValidationError {
    /// An error while parsing a string to a date.
    DateParsing(DateParseError),
    /// An error while parsing a string to an integer.
    IntParsing(ParseIntError),
    /// An error while parsing a string to a URL.
    UrlParsing(UrlParseError),
    /// An error while parsing a string to a MIME type.
    MimeParsing(MimeParseError),
    /// A different validation error.
    Validation(String),
}

impl StdError for ValidationError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            ValidationError::DateParsing(ref err) => Some(err),
            ValidationError::IntParsing(ref err) => Some(err),
            ValidationError::UrlParsing(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ValidationError::DateParsing(ref err) => err.fmt(f),
            ValidationError::IntParsing(ref err) => err.fmt(f),
            ValidationError::UrlParsing(ref err) => err.fmt(f),
            ValidationError::MimeParsing(_) => write!(f, "Unable to parse MIME type"),
            ValidationError::Validation(ref s) => write!(f, "{}", s),
        }
    }
}

impl From<DateParseError> for ValidationError {
    fn from(err: DateParseError) -> Self {
        ValidationError::DateParsing(err)
    }
}

impl From<ParseIntError> for ValidationError {
    fn from(err: ParseIntError) -> Self {
        ValidationError::IntParsing(err)
    }
}

impl From<UrlParseError> for ValidationError {
    fn from(err: UrlParseError) -> Self {
        ValidationError::UrlParsing(err)
    }
}

impl From<MimeParseError> for ValidationError {
    fn from(err: MimeParseError) -> Self {
        ValidationError::MimeParsing(err)
    }
}

/// A trait to support data validation.
pub trait Validate {
    /// Validate the data against the RSS specification.
    fn validate(&self) -> Result<(), ValidationError>;
}

macro_rules! validate {
    ($e: expr, $msg: expr) => {{
        if !($e) {
            return Err(ValidationError::Validation(String::from($msg)));
        }
    }};
}

impl Validate for Channel {
    fn validate(&self) -> Result<(), ValidationError> {
        Url::parse(self.link())?;

        for category in self.categories() {
            category.validate()?;
        }

        if let Some(cloud) = self.cloud() {
            cloud.validate()?;
        }

        if let Some(docs) = self.docs() {
            Url::parse(docs)?;
        }

        if let Some(image) = self.image() {
            image.validate()?;
        }

        for item in self.items() {
            item.validate()?;
        }

        if let Some(last_build_date) = self.last_build_date() {
            DateTime::parse_from_rfc2822(last_build_date)?;
        }

        if let Some(pub_date) = self.pub_date() {
            DateTime::parse_from_rfc2822(pub_date)?;
        }

        for hour in self.skip_hours() {
            let hour = hour.parse::<i64>()?;
            validate!(
                (0..=23).contains(&hour),
                "Channel skip hour is not between 0 and 23"
            );
        }

        let valid_days = {
            let mut set = HashSet::with_capacity(7);
            set.insert("Monday");
            set.insert("Tuesday");
            set.insert("Wednesday");
            set.insert("Thursday");
            set.insert("Friday");
            set.insert("Saturday");
            set.insert("Sunday");
            set
        };

        for day in self.skip_days() {
            validate!(
                valid_days.contains(day.as_str()),
                format!("Unknown skip day: {}", day)
            );
        }

        if let Some(text_input) = self.text_input() {
            text_input.validate()?;
        }

        if let Some(ttl) = self.ttl() {
            let ttl = ttl.parse::<i64>()?;
            validate!(ttl > 0, "Channel TTL is not greater than 0");
        }

        Ok(())
    }
}

impl Validate for Category {
    fn validate(&self) -> Result<(), ValidationError> {
        if let Some(domain) = self.domain() {
            Url::parse(domain)?;
        }
        Ok(())
    }
}

impl Validate for Cloud {
    fn validate(&self) -> Result<(), ValidationError> {
        let port = self.port().parse::<i64>()?;
        validate!(port > 0, "Cloud port must be greater than 0");
        Url::parse(self.domain())?;
        validate!(
            vec!["xml-rpc", "soap", "http-post"].contains(&self.protocol()),
            format!("Unknown cloud protocol: {}", self.protocol())
        );
        Ok(())
    }
}

impl Validate for Enclosure {
    fn validate(&self) -> Result<(), ValidationError> {
        Url::parse(self.url())?;
        self.mime_type().parse::<Mime>()?;
        let length = self.length().parse::<i64>()?;
        validate!(length > 0, "Enclosure length is not greater than 0");
        Ok(())
    }
}

impl Validate for TextInput {
    fn validate(&self) -> Result<(), ValidationError> {
        Url::parse(self.link())?;
        Ok(())
    }
}

impl Validate for Image {
    fn validate(&self) -> Result<(), ValidationError> {
        Url::parse(self.link())?;
        Url::parse(self.url())?;

        if let Some(width) = self.width() {
            let width = width.parse::<i64>()?;
            validate!(
                (0..=144).contains(&width),
                "Image width is not between 0 and 144"
            );
        }

        if let Some(height) = self.height() {
            let height = height.parse::<i64>()?;
            validate!(
                (0..=144).contains(&height),
                "Image height is not between 0 and 144"
            );
        }

        Ok(())
    }
}

impl Validate for Item {
    fn validate(&self) -> Result<(), ValidationError> {
        if let Some(link) = self.link() {
            Url::parse(link)?;
        }

        if let Some(comments) = self.comments() {
            Url::parse(comments)?;
        }

        if let Some(enclosure) = self.enclosure() {
            enclosure.validate()?;
        }

        if let Some(pub_date) = self.pub_date() {
            DateTime::parse_from_rfc2822(pub_date)?;
        }

        if let Some(source) = self.source() {
            source.validate()?;
        }

        Ok(())
    }
}

impl Validate for Source {
    fn validate(&self) -> Result<(), ValidationError> {
        Url::parse(self.url())?;
        Ok(())
    }
}
