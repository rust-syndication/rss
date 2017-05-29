// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use error::Error;
use fromxml::{FromXml, element_text};
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
    /// Return whether this `Guid` is a permalink.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::GuidBuilder;
    ///
    /// let guid = GuidBuilder::default()
    ///     .finalize();
    ///
    /// assert!(guid.is_permalink());
    /// ```
    ///
    /// ```
    /// use rss::GuidBuilder;
    ///
    /// let permalink = true;
    ///
    /// let guid = GuidBuilder::default()
    ///     .is_permalink(permalink)
    ///     .finalize();
    ///
    /// assert_eq!(permalink, guid.is_permalink());
    /// ```
    ///
    /// ```
    /// use rss::GuidBuilder;
    ///
    /// let permalink = false;
    ///
    /// let guid = GuidBuilder::default()
    ///     .is_permalink(permalink)
    ///     .finalize();
    ///
    /// assert_eq!(permalink, guid.is_permalink());
    /// ```
    pub fn is_permalink(&self) -> bool {
        self.is_permalink
    }

    /// Return the value of this `Guid`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::GuidBuilder;
    ///
    /// let value = "9DE46946-2F90-4D5D-9047-7E9165C16E7C";
    ///
    /// let guid = GuidBuilder::default()
    ///     .value(value)
    ///     .finalize();
    ///
    /// assert_eq!(value, guid.value());
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
    fn from_xml<R: ::std::io::BufRead>(reader: &mut Reader<R>,
                                       mut atts: Attributes)
                                       -> Result<Self, Error> {
        let mut is_permalink = true;

        for attr in atts.with_checks(false) {
            if let Ok(attr) = attr {
                if attr.key == b"isPermaLink" {
                    is_permalink = attr.value != b"false";
                    break;
                }
            }
        }

        let content = element_text(reader)?.unwrap_or_default();

        Ok(Guid {
               value: content,
               is_permalink: is_permalink,
           })
    }
}

impl ToXml for Guid {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = b"guid";
        let mut element = BytesStart::borrowed(name, name.len());
        if !self.is_permalink {
            element.push_attribute(("isPermaLink", "false"));
        }

        writer.write_event(Event::Start(element))?;

        writer
            .write_event(Event::Text(BytesText::borrowed(self.value.as_bytes())))?;

        writer.write_event(Event::End(BytesEnd::borrowed(name)))?;
        Ok(())
    }
}

/// A builder used to create an `Guid`.
#[derive(Debug, Clone, Default)]
pub struct GuidBuilder {
    is_permalink: Option<bool>,
    value: String,
}

impl GuidBuilder {
    /// Set whether this `Guid` is a permalink.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::GuidBuilder;
    ///
    /// let builder = GuidBuilder::default()
    ///     .is_permalink(false);
    /// ```
    pub fn is_permalink(mut self, is_permalink: bool) -> GuidBuilder {
        self.is_permalink = Some(is_permalink);
        self
    }

    /// Set the value of this `Guid`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::GuidBuilder;
    ///
    /// let builder = GuidBuilder::default()
    ///     .value("9DE46946-2F90-4D5D-9047-7E9165C16E7C");
    /// ```
    pub fn value<S>(mut self, value: S) -> GuidBuilder
        where S: Into<String>
    {
        self.value = value.into();
        self
    }

    /// Construct the `Guid` from this `GuidBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::GuidBuilder;
    ///
    /// let guid = GuidBuilder::default()
    ///         .is_permalink(true)
    ///         .value("9DE46946-2F90-4D5D-9047-7E9165C16E7C")
    ///         .finalize();
    /// ```
    pub fn finalize(self) -> Guid {
        Guid {
            is_permalink: self.is_permalink.unwrap_or(true),
            value: self.value,
        }
    }
}
