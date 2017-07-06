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

/// Represents a category in an RSS feed
#[derive(Debug, Default, Clone, PartialEq, Builder)]
#[builder(setter(into), default)]
pub struct Category {
    /// The name of the category.
    name: String,
    /// The domain for the category.
    domain: Option<String>,
}

impl Category {
    /// Return the name of this category.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Category;
    ///
    /// let mut category = Category::default();
    /// category.set_name("Technology");
    /// assert_eq!(category.name(), "Technology");
    /// ```
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Set the name of this category.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Category;
    ///
    /// let mut category = Category::default();
    /// category.set_name("Technology");
    /// ```
    pub fn set_name<V>(&mut self, name: V)
    where
        V: Into<String>,
    {
        self.name = name.into();
    }

    /// Return the domain of this category.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Category;
    ///
    /// let mut category = Category::default();
    /// category.set_domain("http://example.com".to_string());
    /// assert_eq!(category.domain(), Some("http://example.com"));
    /// ```
    pub fn domain(&self) -> Option<&str> {
        self.domain.as_ref().map(|s| s.as_str())
    }

    /// Set the domain of this category.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Category;
    ///
    /// let mut category = Category::default();
    /// category.set_domain("http://example.com".to_string());
    /// ```
    pub fn set_domain<V>(&mut self, domain: V)
    where
        V: Into<Option<String>>,
    {
        self.domain = domain.into();
    }
}

impl FromXml for Category {
    fn from_xml<R: BufRead>(reader: &mut Reader<R>, mut atts: Attributes) -> Result<Self, Error> {
        let mut category = Category::default();

        for attr in atts.with_checks(false) {
            if let Ok(attr) = attr {
                if attr.key == b"domain" {
                    category.domain = Some(attr.unescape_and_decode_value(reader)?);
                    break;
                }
            }
        }

        category.name = element_text(reader)?.unwrap_or_default();
        Ok(category)
    }
}

impl ToXml for Category {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = b"category";
        let mut element = BytesStart::borrowed(name, name.len());
        if let Some(ref domain) = self.domain {
            element.push_attribute(("domain", &**domain));
        }
        writer.write_event(Event::Start(element))?;
        writer
            .write_event(Event::Text(BytesText::borrowed(self.name.as_bytes())))?;
        writer.write_event(Event::End(BytesEnd::borrowed(name)))?;
        Ok(())
    }
}
