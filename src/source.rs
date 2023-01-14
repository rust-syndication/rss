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

/// Represents the source of an RSS item.
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
pub struct Source {
    /// The URL of the source.
    pub url: String,
    /// The title of the source.
    pub title: Option<String>,
}

impl Source {
    /// Return the URL of this source.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Source;
    ///
    /// let mut source = Source::default();
    /// source.set_url("http://example.com");
    /// assert_eq!(source.url(), "http://example.com");
    /// ```
    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    /// Set the URL of this source.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Source;
    ///
    /// let mut source = Source::default();
    /// source.set_url("http://example.com");
    /// ```
    pub fn set_url<V>(&mut self, url: V)
    where
        V: Into<String>,
    {
        self.url = url.into();
    }

    /// Return the title of this source.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Source;
    ///
    /// let mut source = Source::default();
    /// source.set_title("Source Title".to_string());
    /// assert_eq!(source.title(), Some("Source Title"));
    /// ```
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Set the title of this source.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Source;
    ///
    /// let mut source = Source::default();
    /// source.set_title("Source Title".to_string());
    /// ```
    pub fn set_title<V>(&mut self, title: V)
    where
        V: Into<Option<String>>,
    {
        self.title = title.into();
    }
}

impl Source {
    /// Builds a Source from source XML
    pub fn from_xml<R: BufRead>(
        reader: &mut Reader<R>,
        mut atts: Attributes,
    ) -> Result<Self, Error> {
        let mut source = Source::default();

        for attr in atts.with_checks(false).flatten() {
            if decode(attr.key.as_ref(), reader)?.as_ref() == "url" {
                source.url = attr_value(&attr, reader)?.to_string();
                break;
            }
        }

        source.title = element_text(reader)?;
        Ok(source)
    }
}

impl ToXml for Source {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = "source";
        let mut element = BytesStart::new(name);
        element.push_attribute(("url", &*self.url));

        writer.write_event(Event::Start(element))?;

        if let Some(ref text) = self.title {
            writer.write_event(Event::Text(BytesText::new(text)))?;
        }

        writer.write_event(Event::End(BytesEnd::new(name)))?;
        Ok(())
    }
}

#[cfg(feature = "builders")]
impl SourceBuilder {
    /// Builds a new `Source`.
    pub fn build(&self) -> Source {
        self.build_impl().unwrap()
    }
}
