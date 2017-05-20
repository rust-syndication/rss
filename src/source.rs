// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use error::Error;
use fromxml::FromXml;
use quick_xml::{Element, Event, XmlReader, XmlWriter};
use quick_xml::error::Error as XmlError;
use reqwest::Url;
use toxml::ToXml;

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
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(url, source.url());
    /// ```
    pub fn url(&self) -> &str {
        self.url
            .as_str()
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
    ///     .title(Some(title.to_owned()))
    ///     .url(url)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(Some(title), source_obj.title());
    /// ```
    pub fn title(&self) -> Option<&str> {
        self.title
            .as_ref()
            .map(|s| s.as_str())
    }
}

impl FromXml for Source {
    fn from_xml<R: ::std::io::BufRead>(mut reader: XmlReader<R>,
                                       element: Element)
        -> Result<(Self, XmlReader<R>), Error> {
        let mut url = None;

        for attr in element.attributes()
                           .with_checks(false)
                           .unescaped() {
            if let Ok(attr) = attr {
                if attr.0 == b"url" {
                    url = Some(String::from_utf8(attr.1
                                                     .into_owned())?);
                    break;
                }
            }
        }

        let url = url.unwrap_or_default();
        let content = element_text!(reader);

        Ok((Source { url: url,
                     title: content, },
            reader))
    }
}

impl ToXml for Source {
    fn to_xml<W: ::std::io::Write>(&self,
                                   writer: &mut XmlWriter<W>)
        -> Result<(), XmlError> {
        let element = Element::new(b"source");

        writer.write(Event::Start({
                                      let mut element = element.clone();
                                      element.extend_attributes(::std::iter::once((b"url",
                                                                                   self.url
                                                                                       .as_str())));
                                      element
                                  }))?;

        if let Some(text) = self.title
                                .as_ref()
                                .map(|s| s.as_str()) {
            writer.write(Event::Text(Element::new(text)))?;
        }

        writer.write(Event::End(element))
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
    /// let mut source_builder = SourceBuilder::new();
    /// source_builder.url("http://www.example.com/source");
    /// ```
    pub fn url(mut self,
               url: &str)
        -> SourceBuilder {
        self.url = String::from(url);
        self
    }


    /// Set the source that exists under `Source`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::SourceBuilder;
    ///
    /// let mut source_builder = SourceBuilder::new();
    /// source_builder.title(Some("Test".to_owned()));
    /// ```
    pub fn title(mut self,
                 title: Option<String>)
        -> SourceBuilder {
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
    ///     .finalize()
    ///     .unwrap();
    /// ```
    pub fn validate(self) -> Result<SourceBuilder, Error> {
        Url::parse(self.url
                       .as_str())?;

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
    ///     .finalize()
    ///     .unwrap();
    /// ```
    pub fn finalize(self) -> Result<Source, Error> {
        Ok(Source { url: self.url,
                    title: self.title, })
    }
}
