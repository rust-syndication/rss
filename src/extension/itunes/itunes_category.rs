// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::io::Write;

use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::Error as XmlError;
use quick_xml::Writer;

use crate::toxml::ToXml;

/// A category for an iTunes podcast.
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
pub struct ITunesCategory {
    /// The name of the category.
    pub text: String,
    // This is contained within a Box to ensure it gets allocated on the heap to prevent an
    // infinite size.
    /// An optional subcategory for the category.
    pub subcategory: Option<Box<ITunesCategory>>,
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
    pub fn subcategory(&self) -> Option<&ITunesCategory> {
        self.subcategory.as_deref()
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
        let name = "itunes:category";
        let mut element = BytesStart::new(name);
        element.push_attribute(("text", &*self.text));
        writer.write_event(Event::Start(element))?;

        if let Some(subcategory) = self.subcategory.as_ref() {
            subcategory.to_xml(writer)?;
        }

        writer.write_event(Event::End(BytesEnd::new(name)))?;
        Ok(())
    }
}

#[cfg(feature = "builders")]
impl ITunesCategoryBuilder {
    /// Builds a new `ITunesCategory`.
    pub fn build(&self) -> ITunesCategory {
        self.build_impl().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "builders")]
    fn test_builder() {
        assert_eq!(
            ITunesCategoryBuilder::default().text("music").build(),
            ITunesCategory {
                text: "music".to_string(),
                subcategory: None,
            }
        );
        assert_eq!(
            ITunesCategoryBuilder::default()
                .text("music")
                .subcategory(Some(Box::new(
                    ITunesCategoryBuilder::default().text("pop").build()
                )))
                .build(),
            ITunesCategory {
                text: "music".to_string(),
                subcategory: Some(Box::new(ITunesCategory {
                    text: "pop".to_string(),
                    subcategory: None,
                })),
            }
        );
    }
}
