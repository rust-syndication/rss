// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use error::Error;
use fromxml::FromXml;
use quick_xml::errors::Error as XmlError;
use quick_xml::events::{Event, BytesStart, BytesEnd, BytesText};
use quick_xml::events::attributes::Attributes;
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use toxml::ToXml;

/// A representation of the `<guid>` element.
#[derive(Debug, Clone, PartialEq)]
pub struct Guid {
    /// The value of the GUID.
    value: String,
    /// Indicates if the GUID is a permalink.
    is_permalink: bool,
}

impl Guid {
    /// Get the permalink that exists under `Guid`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{GuidBuilder, Guid};
    ///
    /// let guid = GuidBuilder::new()
    ///     .is_permalink(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(guid.is_permalink());
    /// ```
    ///
    /// ```
    /// use rss::{GuidBuilder, Guid};
    ///
    /// let permalink = true;
    ///
    /// let guid = GuidBuilder::new()
    ///     .is_permalink(Some(permalink))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(permalink, guid.is_permalink());
    /// ```
    ///
    /// ```
    /// use rss::{GuidBuilder, Guid};
    ///
    /// let permalink = false;
    ///
    /// let guid = GuidBuilder::new()
    ///     .is_permalink(Some(permalink))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(permalink, guid.is_permalink());
    /// ```
    pub fn is_permalink(&self) -> bool {
        self.is_permalink
    }

    /// Get the guid that exists under `Guid`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{GuidBuilder, Guid};
    ///
    /// let guid = "9DE46946-2F90-4D5D-9047-7E9165C16E7C";
    /// let guid_obj = GuidBuilder::new()
    ///     .value(guid)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(guid, guid_obj.value());
    /// ```
    pub fn value(&self) -> &str {
        self.value.as_str()
    }
}

impl Default for Guid {
    #[inline]
    fn default() -> Self {
        Guid {
            value: Default::default(),
            is_permalink: true,
        }
    }
}

impl FromXml for Guid {
    fn from_xml<R: ::std::io::BufRead>(mut reader: Reader<R>,
                                       mut atts: Attributes)
                                       -> Result<(Self, Reader<R>), Error> {
        let mut is_permalink = true;

        for attr in atts.with_checks(false) {
            if let Ok(attr) = attr {
                if attr.key == b"isPermaLink" {
                    is_permalink = attr.value != b"false";
                    break;
                }
            }
        }

        let content = element_text!(reader).unwrap_or_default();

        Ok((Guid {
                value: content,
                is_permalink: is_permalink,
            },
            reader))
    }
}

impl ToXml for Guid {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = b"guid";

        writer.write_event(Event::Start({
            let mut element = BytesStart::borrowed(name, name.len());
            if !self.is_permalink {
                element.push_attribute((b"isPermaLink".as_ref(), b"false".as_ref()));
            }
            element
        }))?;

        writer.write_event(Event::Text(BytesText::borrowed(self.value.as_bytes())))?;

        writer.write_event(Event::End(BytesEnd::borrowed(name)))?;
        Ok(())
    }
}

/// This `GuidBuilder` struct creates the `Guid`.
#[derive(Debug, Clone, Default)]
pub struct GuidBuilder {
    is_permalink: Option<bool>,
    value: String,
}

impl GuidBuilder {
    /// Construct a new `GuidBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::GuidBuilder;
    ///
    /// let guid_builder = GuidBuilder::new();
    /// ```
    pub fn new() -> GuidBuilder {
        GuidBuilder::default()
    }

    /// Set the optional permalink that exists under `Guid`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::GuidBuilder;
    ///
    /// let mut guid_builder = GuidBuilder::new();
    /// guid_builder.is_permalink(Some(false));
    /// ```
    pub fn is_permalink(mut self, is_permalink: Option<bool>) -> GuidBuilder {
        self.is_permalink = is_permalink;
        self
    }

    /// Set the guid that exists under `Guid`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::GuidBuilder;
    ///
    /// let mut guid_builder = GuidBuilder::new();
    /// guid_builder.value("9DE46946-2F90-4D5D-9047-7E9165C16E7C");
    /// ```
    pub fn value(mut self, value: &str) -> GuidBuilder {
        self.value = value.to_string();
        self
    }

    /// Construct the `Guid` from the `GuidBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::GuidBuilder;
    ///
    /// let guid = GuidBuilder::new()
    ///         .value("9DE46946-2F90-4D5D-9047-7E9165C16E7C")
    ///         .is_permalink(Some(true))
    ///         .finalize();
    /// ```
    pub fn finalize(self) -> Result<Guid, Error> {
        let is_permalink = match self.is_permalink {
            Some(val) => val,
            None => true,
        };

        Ok(Guid {
               is_permalink: is_permalink,
               value: self.value,
           })
    }
}
