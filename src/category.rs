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
use url::Url;

/// A representation of the `<category>` element.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Category {
    /// The name of the category.
    name: String,
    /// The domain for the category.
    domain: Option<String>,
}

impl Category {
    /// Return the name of this `Category`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::CategoryBuilder;
    ///
    /// let name = "Podcast";
    ///
    /// let category = CategoryBuilder::default()
    ///     .name(name)
    ///     .finalize();
    ///
    /// assert_eq!(name, category.name());
    /// ```
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Return the domain of this `Category`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::CategoryBuilder;
    ///
    /// let domain = "http://jupiterbroadcasting.com/";
    ///
    /// let category = CategoryBuilder::default()
    ///     .domain(domain.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(domain), category.domain());
    /// ```
    ///
    /// ```
    /// use rss::CategoryBuilder;
    ///
    /// let category = CategoryBuilder::default()
    ///     .domain(None)
    ///     .finalize();
    ///
    /// assert!(category.domain().is_none());
    /// ```
    pub fn domain(&self) -> Option<&str> {
        self.domain.as_ref().map(|s| s.as_str())
    }
}

impl FromXml for Category {
    fn from_xml<R: ::std::io::BufRead>(reader: &mut Reader<R>,
                                       mut atts: Attributes)
                                       -> Result<Self, Error> {
        let mut domain = None;

        for attr in atts.with_checks(false) {
            if let Ok(attr) = attr {
                if attr.key == b"domain" {
                    domain = Some(attr.unescape_and_decode_value(reader)?);
                    break;
                }
            }
        }

        let content = element_text(reader)?.unwrap_or_default();

        Ok(Category {
               name: content,
               domain: domain,
           })
    }
}

impl ToXml for Category {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
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

/// A builder used to create a `Category`.
#[derive(Debug, Clone, Default)]
pub struct CategoryBuilder {
    name: String,
    domain: Option<String>,
}

impl CategoryBuilder {
    /// Construct a new `CategoryBuilder` using the values from an existing `Category`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{Channel, CategoryBuilder};
    ///
    /// let input = include_str!("tests/data/category.xml");
    /// let channel = input.parse::<Channel>().unwrap();
    /// let category = channel.categories()[0].clone();
    /// let builder = CategoryBuilder::from_category(category);
    /// ```
    pub fn from_category(category: Category) -> Self {
        CategoryBuilder {
            name: category.name,
            domain: category.domain,
        }
    }

    /// Set the name of the `Category`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::CategoryBuilder;
    ///
    /// let builder = CategoryBuilder::default()
    ///     .name("Podcast");
    /// ```
    pub fn name<S>(mut self, name: S) -> CategoryBuilder
        where S: Into<String>
    {
        self.name = name.into();
        self
    }

    /// Set the domain for the `Category`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::CategoryBuilder;
    ///
    /// let builder = CategoryBuilder::default()
    ///     .domain("http://www.example.com".to_string());
    /// ```
    pub fn domain<V>(mut self, domain: V) -> CategoryBuilder
        where V: Into<Option<String>>
    {
        self.domain = domain.into();
        self
    }

    /// Validate the contents of this `CategoryBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::CategoryBuilder;
    ///
    /// let builder = CategoryBuilder::default()
    ///     .name("Podcast")
    ///     .domain("http://www.example.com".to_string())
    ///     .validate()
    ///     .unwrap();
    /// ```
    pub fn validate(self) -> Result<CategoryBuilder, Error> {
        if let Some(ref domain) = self.domain {
            Url::parse(domain)?;
        }

        Ok(self)
    }

    /// Construct the `Category` from this `CategoryBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::CategoryBuilder;
    ///
    /// let category = CategoryBuilder::default()
    ///     .name("Podcast")
    ///     .domain("http://www.example.com".to_string())
    ///     .finalize();
    /// ```
    pub fn finalize(self) -> Category {
        Category {
            name: self.name,
            domain: self.domain,
        }
    }
}
