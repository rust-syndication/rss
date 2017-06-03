// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

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
    /// Return the name of this category.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesCategoryBuilder;
    ///
    /// let text = "category";
    ///
    /// let category = ITunesCategoryBuilder::default()
    ///     .text(text)
    ///     .finalize();
    ///
    /// assert_eq!(text, category.text())
    /// ```
    pub fn text(&self) -> &str {
        self.text.as_str()
    }

    /// Return the subcategory for this category.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesCategoryBuilder;
    ///
    /// let subcategory = ITunesCategoryBuilder::default()
    ///     .text("subcategory")
    ///     .finalize();
    ///
    /// let category = ITunesCategoryBuilder::default()
    ///     .text("category")
    ///     .subcategory(Box::new(subcategory))
    ///     .finalize();
    ///
    /// assert!(category.subcategory().is_some());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::ITunesCategoryBuilder;
    ///
    /// let category = ITunesCategoryBuilder::default()
    ///     .subcategory(None)
    ///     .finalize();
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

/// A builder used to create an `ITunesCategory`.
#[derive(Debug, Clone, Default)]
pub struct ITunesCategoryBuilder {
    text: String,
    subcategory: Option<Box<ITunesCategory>>,
}

impl ITunesCategoryBuilder {
    /// Construct a new `ITunesCategoryBuilder` using the values from an existing `ITunesCategory`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    /// use rss::extension::itunes::ITunesCategoryBuilder;
    ///
    /// let input = include_str!("tests/data/itunes.xml");
    /// let channel = input.parse::<Channel>().unwrap();
    /// let category = channel.itunes_ext().unwrap().categories()[0].clone();
    /// let builder = ITunesCategoryBuilder::from_category(category);
    /// ```
    pub fn from_category(category: ITunesCategory) -> Self {
        ITunesCategoryBuilder {
            text: category.text,
            subcategory: category.subcategory,
        }
    }

    /// Set the name of the category.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesCategoryBuilder;
    ///
    /// let builder = ITunesCategoryBuilder::default()
    ///     .text("category");
    /// ```
    pub fn text<S>(mut self, text: S) -> ITunesCategoryBuilder
        where S: Into<String>
    {
        self.text = text.into();
        self
    }

    /// Set the subcategory for the category.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesCategoryBuilder;
    ///
    /// let subcategory = ITunesCategoryBuilder::default()
    ///     .text("subcategory")
    ///     .finalize();
    ///
    /// let builder = ITunesCategoryBuilder::default()
    ///     .text("category")
    ///     .subcategory(Box::new(subcategory));
    /// ```
    pub fn subcategory<V>(mut self, subcategory: V) -> ITunesCategoryBuilder
        where V: Into<Option<Box<ITunesCategory>>>
    {
        self.subcategory = subcategory.into();
        self
    }

    /// Construct the `ITunesCategory` from this `ITunesCategoryBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesCategoryBuilder;
    ///
    /// let category = ITunesCategoryBuilder::default()
    ///     .text("category")
    ///     .finalize();
    /// ```
    pub fn finalize(self) -> ITunesCategory {
        ITunesCategory {
            text: self.text,
            subcategory: self.subcategory,
        }
    }
}
