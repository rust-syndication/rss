// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use error::Error;
use quick_xml::errors::Error as XmlError;
use quick_xml::events::{Event, BytesStart, BytesEnd};
use quick_xml::writer::Writer;
use toxml::ToXml;

/// A category for an iTunes podcast.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ITunesCategory {
    /// The name of the category.
    text: String,
    // This is contained within a Box to ensure it gets allocated on the heap to prevent an
    // infinite size.
    /// An optional subcategory for the cagetory.
    subcategory: Option<Box<ITunesCategory>>,
}

impl ITunesCategory {
    /// Get the text that exists under `ITunesCategory`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesCategoryBuilder,
    /// ITunesCategory};
    ///
    /// let text = "text";
    ///
    /// let category = ITunesCategoryBuilder::new()
    ///     .text(text)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(text, category.text())
    /// ```
    pub fn text(&self) -> &str {
        self.text.as_str()
    }


    /// Get the optional subcategory that exists under `ITunesCategory`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesCategoryBuilder,
    /// ITunesCategory};
    ///
    /// let subcategory = ITunesCategoryBuilder::new()
    ///     .text("text")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let category = ITunesCategoryBuilder::new()
    ///     .text("text")
    ///     .subcategory(Some(Box::new(subcategory)))
    ///     .finalize()
    ///     .unwrap();;
    ///
    /// assert!(category.subcategory().is_some());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesCategoryBuilder,
    /// ITunesCategory};
    ///
    /// let category = ITunesCategoryBuilder::new()
    ///     .text("text")
    ///     .subcategory(None)
    ///     .finalize()
    ///     .unwrap();;
    ///
    /// assert!(category.subcategory().is_none());
    /// ```
    pub fn subcategory(&self) -> Option<&Box<ITunesCategory>> {
        self.subcategory.as_ref()
    }
}

impl ToXml for ITunesCategory {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = b"itunes:category";
        let mut element = BytesStart::borrowed(name, name.len());
        element.push_attribute(("text", &*self.text));
        writer.write_event(Event::Start(element))?;

        if let Some(subcategory) = self.subcategory.as_ref() {
            subcategory.to_xml(writer)?;
        }

        writer.write_event(Event::End(BytesEnd::borrowed(name)))?;
        Ok(())
    }
}

/// This `ITunesCategoryBuilder` struct creates the `ITunesCategory`.
#[derive(Debug, Clone, Default)]
pub struct ITunesCategoryBuilder {
    text: String,
    subcategory: Option<Box<ITunesCategory>>,
}

impl ITunesCategoryBuilder {
    /// Construct a new `ITunesCategoryBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesCategoryBuilder;
    ///
    /// let category_builder = ITunesCategoryBuilder::new();
    /// ```
    pub fn new() -> ITunesCategoryBuilder {
        ITunesCategoryBuilder::default()
    }

    /// Set the text that exists uner `ITunesCategory`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesCategoryBuilder;
    ///
    /// let mut category_builder = ITunesCategoryBuilder::new();
    /// category_builder.text("text");
    /// ```
    pub fn text(mut self, text: &str) -> ITunesCategoryBuilder {
        self.text = text.to_string();
        self
    }

    /// Set the optional subcategory that exists uner `ITunesCategory`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesCategoryBuilder;
    ///
    /// let subcategory = ITunesCategoryBuilder::new()
    ///     .text("text")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let mut category_builder = ITunesCategoryBuilder::new();
    /// category_builder.subcategory(Some(Box::new(subcategory)));
    /// ```
    pub fn subcategory(mut self,
                       subcategory: Option<Box<ITunesCategory>>)
                       -> ITunesCategoryBuilder {
        self.subcategory = subcategory;
        self
    }

    /// Construct the `ITunesCategory` from the `ITunesCategoryBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesCategoryBuilder;
    ///
    /// let subcategory = ITunesCategoryBuilder::new()
    ///     .text("text")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let category = ITunesCategoryBuilder::new()
    ///     .text("text")
    ///     .subcategory(Some(Box::new(subcategory)))
    ///     .finalize()
    ///     .unwrap();
    /// ```
    pub fn finalize(self) -> Result<ITunesCategory, Error> {
        Ok(ITunesCategory {
               text: self.text,
               subcategory: self.subcategory,
           })
    }
}
