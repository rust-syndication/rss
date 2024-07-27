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
use crate::util::{attr_value, decode, element_text};

/// Represents a category in an RSS feed.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[cfg_attr(
    feature = "builders",
    builder(
        setter(into),
        default,
        build_fn(name = "build_impl", private, error = "never::Never")
    )
)]
pub struct Category {
    /// The name of the category.
    pub name: String,
    /// The domain for the category.
    pub domain: Option<String>,
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
        self.domain.as_deref()
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

impl Category {
    /// Builds a Category from source XML
    pub fn from_xml<R: BufRead>(
        reader: &mut Reader<R>,
        mut atts: Attributes,
    ) -> Result<Self, Error> {
        let mut category = Category::default();

        for attr in atts.with_checks(false).flatten() {
            if decode(attr.key.as_ref(), reader)?.as_ref() == "domain" {
                category.domain = Some(attr_value(&attr, reader)?.to_string());
                break;
            }
        }

        category.name = element_text(reader)?.unwrap_or_default();
        Ok(category)
    }
}

impl ToXml for Category {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = "category";
        let mut element = BytesStart::new(name);
        if let Some(ref domain) = self.domain {
            element.push_attribute(("domain", &**domain));
        }
        writer.write_event(Event::Start(element))?;
        writer.write_event(Event::Text(BytesText::new(&self.name)))?;
        writer.write_event(Event::End(BytesEnd::new(name)))?;
        Ok(())
    }
}

impl From<String> for Category {
    fn from(name: String) -> Self {
        Self { name, domain: None }
    }
}

impl From<&str> for Category {
    fn from(name: &str) -> Self {
        Self {
            name: name.to_string(),
            domain: None,
        }
    }
}

#[cfg(feature = "builders")]
impl CategoryBuilder {
    /// Builds a new `Category`.
    pub fn build(&self) -> Category {
        self.build_impl().unwrap()
    }
}
