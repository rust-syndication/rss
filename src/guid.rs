// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::io::{BufRead, Write};

use quick_xml::events::attributes::Attributes;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Error as XmlError;
use quick_xml::Reader;
use quick_xml::Writer;

use crate::error::Error;
use crate::toxml::ToXml;
use crate::util::{decode, element_text};

/// Represents the GUID of an RSS item.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[cfg_attr(
    feature = "builders",
    builder(
        setter(into),
        default,
        build_fn(name = "build_impl", private, error = "never::Never")
    )
)]
pub struct Guid {
    /// The value of the GUID.
    pub value: String,
    /// Indicates if the GUID is a permalink.
    pub permalink: bool,
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

impl Guid {
    /// Builds a Guid from source XML
    pub fn from_xml<R: BufRead>(
        reader: &mut Reader<R>,
        mut atts: Attributes,
    ) -> Result<Self, Error> {
        let mut guid = Guid::default();

        for attr in atts.with_checks(false).flatten() {
            if decode(attr.key.as_ref(), reader)?.as_ref() == "isPermaLink" {
                guid.permalink = &*attr.value != b"false";
                break;
            }
        }

        guid.value = element_text(reader)?.unwrap_or_default();
        Ok(guid)
    }
}

impl ToXml for Guid {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = "guid";
        let mut element = BytesStart::new(name);
        if !self.permalink {
            element.push_attribute(("isPermaLink", "false"));
        }
        writer.write_event(Event::Start(element))?;
        writer.write_event(Event::Text(BytesText::new(&self.value)))?;
        writer.write_event(Event::End(BytesEnd::new(name)))?;
        Ok(())
    }
}

#[cfg(feature = "builders")]
impl GuidBuilder {
    /// Builds a new `Guid`.
    pub fn build(&self) -> Guid {
        self.build_impl().unwrap()
    }
}
