// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::io::Write;

use quick_xml::errors::Error as XmlError;
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::writer::Writer;

use toxml::ToXml;

/// A category for an iTunes podcast.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, Builder)]
#[builder(setter(into), default)]
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
    /// use rss::extension::itunes::ITunesCategory;
    ///
    /// let mut category = ITunesCategory::default();
    /// category.set_text("Technology");
    /// assert_eq!(category.text(), "Technology")
    /// ```
    pub fn text(&self) -> &str {
        self.text.as_str()
    }
    /// Set the name of this category.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesCategory;
    ///
    /// let mut category = ITunesCategory::default();
    /// category.set_text("Technology");
    /// ```
    pub fn set_text<V>(&mut self, text: V)
    where
        V: Into<String>,
    {
        self.text = text.into();
    }

    /// Return the subcategory for this category.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesCategory;
    ///
    /// let mut category = ITunesCategory::default();
    /// category.set_subcategory(Box::new(ITunesCategory::default()));
    /// assert!(category.subcategory().is_some());
    /// ```
    pub fn subcategory(&self) -> Option<&Box<ITunesCategory>> {
        self.subcategory.as_ref()
    }

    /// Set the subcategory for this category.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesCategory;
    ///
    /// let mut category = ITunesCategory::default();
    /// category.set_subcategory(Box::new(ITunesCategory::default()));
    /// ```
    pub fn set_subcategory<V>(&mut self, subcategory: V)
    where
        V: Into<Option<Box<ITunesCategory>>>,
    {
        self.subcategory = subcategory.into();
    }
}

impl ToXml for ITunesCategory {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
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
