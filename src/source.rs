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

/// A representation of the `<source>` element.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Source {
    /// The URL of the source.
    url: String,
    /// The title of the source.
    title: Option<String>,
}

impl Source {
    /// Get the url that exists under `Source`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{SourceBuilder, Source};
    ///
    /// let url = "http://www.tomalak.org/links2.xml";
    ///
    /// let source = SourceBuilder::new()
    ///     .url(url)
    ///     .finalize();
    ///
    /// assert_eq!(url, source.url());
    /// ```
    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    /// Get the source that exists under `Source`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{SourceBuilder, Source};
    ///
    /// let title = "Tomalak's Realm";
    ///
    /// let url = "http://www.tomalak.org/links2.xml";
    ///
    /// let source_obj = SourceBuilder::new()
    ///     .title(Some(title.to_string()))
    ///     .url(url)
    ///     .finalize();
    ///
    /// assert_eq!(Some(title), source_obj.title());
    /// ```
    pub fn title(&self) -> Option<&str> {
        self.title.as_ref().map(|s| s.as_str())
    }
}

impl FromXml for Source {
    fn from_xml<R: ::std::io::BufRead>(reader: &mut Reader<R>,
                                       mut atts: Attributes)
                                       -> Result<Self, Error> {
        let mut url = None;

        for attr in atts.with_checks(false) {
            if let Ok(attr) = attr {
                if attr.key == b"url" {
                    url = Some(attr.unescape_and_decode_value(reader)?);
                    break;
                }
            }
        }

        let content = element_text(reader)?;

        Ok(Source {
               url: url.unwrap_or_default(),
               title: content,
           })
    }
}

impl ToXml for Source {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = b"source";
        let mut element = BytesStart::borrowed(name, name.len());
        element.push_attribute(("url", &*self.url));

        writer.write_event(Event::Start(element))?;

        if let Some(text) = self.title.as_ref().map(|s| s.as_bytes()) {
            writer.write_event(Event::Text(BytesText::borrowed(text)))?;
        }

        writer.write_event(Event::End(BytesEnd::borrowed(name)))?;
        Ok(())
    }
}

/// This `SourceBuilder` struct creates the `Source`.
#[derive(Debug, Clone, Default)]
pub struct SourceBuilder {
    url: String,
    title: Option<String>,
}

impl SourceBuilder {
    /// Construct a new `SourceBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::SourceBuilder;
    ///
    /// let source_builder = SourceBuilder::new();
    /// ```
    pub fn new() -> SourceBuilder {
        SourceBuilder::default()
    }


    /// Set the url that exists under `Source`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::SourceBuilder;
    ///
    /// let source_builder = SourceBuilder::new()
    ///     .url("http://www.example.com/source");
    /// ```
    pub fn url<S: Into<String>>(mut self, url: S) -> SourceBuilder {
        self.url = url.into();
        self
    }


    /// Set the source that exists under `Source`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::SourceBuilder;
    ///
    /// let source_builder = SourceBuilder::new()
    ///     .title(Some("Test".to_string()));
    /// ```
    pub fn title(mut self, title: Option<String>) -> SourceBuilder {
        self.title = title;
        self
    }


    /// Validate the contents of `Source`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::SourceBuilder;
    ///
    /// let source = SourceBuilder::new()
    ///     .url("http://www.example.com/source")
    ///     .title(None)
    ///     .validate()
    ///     .unwrap()
    ///     .finalize();
    /// ```
    pub fn validate(self) -> Result<SourceBuilder, Error> {
        Url::parse(self.url.as_str())?;

        Ok(self)
    }


    /// Construct the `Source` from the `SourceBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::SourceBuilder;
    ///
    /// let source = SourceBuilder::new()
    ///     .url("http://www.example.com/source")
    ///     .title(None)
    ///     .finalize();
    /// ```
    pub fn finalize(self) -> Source {
        Source {
            url: self.url,
            title: self.title,
        }
    }
}
