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
    /// Get the category that exists under `Category`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{CategoryBuilder, Category};
    ///
    /// let category = "podcast";
    ///
    /// let category_obj = CategoryBuilder::new()
    ///     .name(category)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(category, category_obj.name());
    /// ```
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Get the optional domain that exists under `Category`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{CategoryBuilder, Category};
    ///
    /// let domain_string = "http://jupiterbroadcasting.com/";
    ///
    /// let category = CategoryBuilder::new()
    ///     .domain(Some(domain_string.to_string()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(Some(domain_string), category.domain());
    /// ```
    ///
    /// ```
    /// use rss::{CategoryBuilder, Category};
    ///
    /// let category = CategoryBuilder::new()
    ///     .domain(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let domain_option = category.domain();
    /// assert!(domain_option.is_none());
    /// ```
    pub fn domain(&self) -> Option<&str> {
        self.domain.as_ref().map(|s| s.as_str())
    }
}

impl FromXml for Category {
    fn from_xml<R: ::std::io::BufRead>(mut reader: Reader<R>,
                                       mut atts: Attributes)
                                       -> Result<(Self, Reader<R>), Error> {
        let mut domain = None;

        for attr in atts.with_checks(false) {
            if let Ok(attr) = attr {
                if attr.key == b"domain" {
                    domain = Some(try!(attr.unescape_and_decode_value(&reader)));
                    break;
                }
            }
        }

        let content = element_text!(reader).unwrap_or_default();

        Ok((Category {
                name: content,
                domain: domain,
            },
            reader))
    }
}

impl ToXml for Category {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = b"category";

        try!(writer.write_event(Event::Start({
                                                 let mut element = BytesStart::borrowed(name,
                                                                                        name.len());
                                                 if let Some(ref domain) = self.domain {
                                                     element.push_attribute((b"domain".as_ref(),
                                                                             domain.as_bytes()));
                                                 }
                                                 element
                                             })));

        try!(writer.write_event(Event::Text(BytesText::borrowed(self.name.as_bytes()))));

        try!(writer.write_event(Event::End(BytesEnd::borrowed(name))));
        Ok(())
    }
}

/// This `CategoryBuilder` struct creates the `Category`.
#[derive(Debug, Clone, Default)]
pub struct CategoryBuilder {
    name: String,
    domain: Option<String>,
}

impl CategoryBuilder {
    /// Construct a new `CategoryBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::CategoryBuilder;
    ///
    /// let category_builder = CategoryBuilder::new();
    /// ```
    pub fn new() -> CategoryBuilder {
        CategoryBuilder::default()
    }

    /// Set the category that exists under `Category`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::CategoryBuilder;
    ///
    /// let mut category_builder = CategoryBuilder::new();
    /// category_builder.name("Podcast");
    /// ```
    pub fn name(mut self, name: &str) -> CategoryBuilder {
        self.name = name.to_string();
        self
    }

    /// Set the optional domain that exists under `Category`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::CategoryBuilder;
    ///
    /// let mut category_builder = CategoryBuilder::new();
    /// category_builder.domain(Some("http://www.example.com".to_string()));
    /// ```
    pub fn domain(mut self, domain: Option<String>) -> CategoryBuilder {
        self.domain = domain;
        self
    }

    /// Validate the contents of `Category`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::CategoryBuilder;
    ///
    /// let category_builder = CategoryBuilder::new()
    ///         .domain(Some("http://www.example.com".to_string()))
    ///         .name("Podcast")
    ///         .validate()
    ///         .unwrap()
    ///         .finalize()
    ///         .unwrap();
    /// ```

    pub fn validate(self) -> Result<CategoryBuilder, Error> {
        if let Some(ref domain) = self.domain {
            Url::parse(domain)?;
        }

        Ok(self)
    }

    /// Construct the `Category` from the `CategoryBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::CategoryBuilder;
    ///
    /// let category = CategoryBuilder::new()
    ///         .name("Title")
    ///         .domain(None)
    ///         .finalize()
    ///         .unwrap();
    /// ```
    pub fn finalize(self) -> Result<Category, Error> {
        Ok(Category {
               name: self.name,
               domain: self.domain,
           })
    }
}
