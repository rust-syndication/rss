// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::io::{BufRead, Write};

use quick_xml::errors::Error as XmlError;
use quick_xml::events::{Event, BytesStart, BytesEnd, BytesText};
use quick_xml::events::attributes::Attributes;
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;

use error::Error;
use fromxml::FromXml;
use toxml::ToXml;
use util::element_text;

/// Represents the GUID of an RSS item.
#[derive(Debug, Clone, PartialEq, Builder)]
#[builder(setter(into), default)]
pub struct Guid {
    /// The value of the GUID.
    value: String,
    /// Indicates if the GUID is a permalink.
    permalink: bool,
}

impl Guid {
    /// Return whether this GUID is a permalink.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Guid;
    ///
    /// let mut guid = Guid::default();
    /// guid.set_permalink(true);
    /// assert!(guid.is_permalink());
    /// ```
    pub fn is_permalink(&self) -> bool {
        self.permalink
    }

    /// Set whether this GUID is a permalink.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Guid;
    ///
    /// let mut guid = Guid::default();
    /// guid.set_permalink(true);
    /// ```
    pub fn set_permalink<V>(&mut self, permalink: V)
    where
        V: Into<bool>,
    {
        self.permalink = permalink.into()
    }

    /// Return the value of this GUID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Guid;
    ///
    /// let mut guid = Guid::default();
    /// guid.set_value("00000000-0000-0000-0000-00000000000");
    /// assert_eq!(guid.value(), "00000000-0000-0000-0000-00000000000");
    /// ```
    pub fn value(&self) -> &str {
        self.value.as_str()
    }

    /// Set the value of this GUID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Guid;
    ///
    /// let mut guid = Guid::default();
    /// guid.set_value("00000000-0000-0000-0000-00000000000");
    /// ```
    pub fn set_value<V>(&mut self, value: V)
    where
        V: Into<String>,
    {
        self.value = value.into();
    }
}

impl Default for Guid {
    #[inline]
    fn default() -> Self {
        Guid {
            value: Default::default(),
            permalink: true,
        }
    }
}

impl FromXml for Guid {
    fn from_xml<R: BufRead>(reader: &mut Reader<R>, mut atts: Attributes) -> Result<Self, Error> {
        let mut guid = Guid::default();

        for attr in atts.with_checks(false) {
            if let Ok(attr) = attr {
                if attr.key == b"isPermaLink" {
                    guid.permalink = &*attr.value != b"false";
                    break;
                }
            }
        }

        guid.value = element_text(reader)?.unwrap_or_default();
        Ok(guid)
    }
}

impl ToXml for Guid {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = b"guid";
        let mut element = BytesStart::borrowed(name, name.len());
        if !self.permalink {
            element.push_attribute(("isPermaLink", "false"));
        }

        writer.write_event(Event::Start(element))?;

        writer
            .write_event(Event::Text(BytesText::borrowed(self.value.as_bytes())))?;

        writer.write_event(Event::End(BytesEnd::borrowed(name)))?;
        Ok(())
    }
}
