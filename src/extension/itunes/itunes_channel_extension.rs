// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::collections::HashMap;
use std::io::Write;

use quick_xml::Error as XmlError;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Writer;

use super::{parse_categories, parse_image, parse_owner};
use crate::extension::Extension;
use crate::extension::itunes::{ITunesCategory, ITunesOwner};
use crate::extension::util::remove_extension_value;
use crate::toxml::{ToXml, WriterExt};

/// An iTunes channel element extension.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[cfg_attr(feature = "builders", builder(setter(into), default))]
pub struct ITunesChannelExtension {
    /// The author of the podcast.
    author: Option<String>,
    /// Specifies if the podcast should be prevented from appearing in the iTunes Store. A value of
    /// `Yes` indicates that the podcast should not show up in the iTunes Store. All other values
    /// are ignored.
    block: Option<String>,
    /// The iTunes categories the podcast belongs to.
    categories: Vec<ITunesCategory>,
    /// The artwork for the podcast.
    image: Option<String>,
    /// Specifies whether the podcast contains explicit content. A value of `Yes`, `Explicit`, or
    /// `True` indicates that the podcast contains explicit content. A value of `Clean`, `No`,
    /// `False` inidicates that none of the episodes contain explicit content.
    explicit: Option<String>,
    /// Specifies whether the podcast is complete and no new episodes will be posted. A value of
    /// `Yes` indicates that the podcast is complete.
    complete: Option<String>,
    /// The new URL where the podcast is located.
    new_feed_url: Option<String>,
    /// The contact information for the owner of the podcast.
    owner: Option<ITunesOwner>,
    /// A description of the podcast.
    subtitle: Option<String>,
    /// A summary of the podcast.
    summary: Option<String>,
    /// Keywords for the podcast. The string contains a comma separated list of keywords.
    keywords: Option<String>,
}

impl ITunesChannelExtension {
    /// Return the author of this podcast.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtension;
    ///
    /// let mut extension = ITunesChannelExtension::default();
    /// extension.set_author("John Doe".to_string());
    /// assert_eq!(extension.author(), Some("John Doe"));
    /// ```
    pub fn author(&self) -> Option<&str> {
        self.author.as_ref().map(String::as_str)
    }

    /// Set the author of this podcast.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtension;
    ///
    /// let mut extension = ITunesChannelExtension::default();
    /// extension.set_author("John Doe".to_string());
    /// ```
    pub fn set_author<V>(&mut self, author: V)
    where
        V: Into<Option<String>>,
    {
        self.author = author.into();
    }

    /// Return whether the podcast should be blocked from appearing in the iTunes Store.
    ///
    /// A value of `Yes` indicates that the podcast should not show up in the iTunes Store. All
    /// other values are ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtension;
    ///
    /// let mut extension = ITunesChannelExtension::default();
    /// extension.set_block("Yes".to_string());
    /// assert_eq!(extension.block(), Some("Yes"));
    /// ```
    pub fn block(&self) -> Option<&str> {
        self.block.as_ref().map(String::as_str)
    }

    /// Set whether the podcast should be blocked from appearing in the iTunes Store.
    ///
    /// A value of `Yes` indicates that the podcast should not show up in the iTunes Store. All
    /// other values are ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtension;
    ///
    /// let mut extension = ITunesChannelExtension::default();
    /// extension.set_block("Yes".to_string());
    /// ```
    pub fn set_block<V>(&mut self, block: V)
    where
        V: Into<Option<String>>,
    {
        self.block = block.into();
    }

    /// Return the iTunes categories that the podcast belongs to.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesCategory, ITunesChannelExtension};
    ///
    /// let mut extension = ITunesChannelExtension::default();
    /// extension.set_categories(vec![ITunesCategory::default()]);
    /// assert_eq!(extension.categories().len(), 1);
    /// ```
    pub fn categories(&self) -> &[ITunesCategory] {
        &self.categories
    }

    /// Return a mutable slice of the iTunes categories that the podcast belongs to.
    pub fn categories_mut(&mut self) -> &mut [ITunesCategory] {
        &mut self.categories
    }

    /// Set the iTunes categories that the podcast belongs to.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesCategory, ITunesChannelExtension};
    ///
    /// let mut extension = ITunesChannelExtension::default();
    /// extension.set_categories(vec![ITunesCategory::default()]);
    /// ```
    pub fn set_categories<V>(&mut self, categories: V)
    where
        V: Into<Vec<ITunesCategory>>,
    {
        self.categories = categories.into();
    }

    /// Return the artwork URL for the podcast.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtension;
    ///
    /// let mut extension = ITunesChannelExtension::default();
    /// extension.set_image("http://example.com/artwork.png".to_string());
    /// assert_eq!(extension.image(), Some("http://example.com/artwork.png"));
    /// ```
    pub fn image(&self) -> Option<&str> {
        self.image.as_ref().map(String::as_str)
    }

    /// Set the artwork URL for the podcast.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtension;
    ///
    /// let mut extension = ITunesChannelExtension::default();
    /// extension.set_image("http://example.com/artwork.png".to_string());
    /// ```
    pub fn set_image<V>(&mut self, image: V)
    where
        V: Into<Option<String>>,
    {
        self.image = image.into();
    }

    /// Return whether the podcast contains explicit content.
    ///
    /// A value of `Yes`, `Explicit`, or `True` indicates that the podcast contains explicit
    /// content. A value of `Clean`, `No`, `False` inidicates that none of the episodes contain
    /// explicit content.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtension;
    ///
    /// let mut extension = ITunesChannelExtension::default();
    /// extension.set_explicit("Yes".to_string());
    /// assert_eq!(extension.explicit(), Some("Yes"));
    /// ```
    pub fn explicit(&self) -> Option<&str> {
        self.explicit.as_ref().map(String::as_str)
    }

    /// Set whether the podcast contains explicit content.
    ///
    /// A value of `Yes`, `Explicit`, or `True` indicates that the podcast contains explicit
    /// content. A value of `Clean`, `No`, `False` inidicates that none of the episodes contain
    /// explicit content.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtension;
    ///
    /// let mut extension = ITunesChannelExtension::default();
    /// extension.set_explicit("Yes".to_string());
    /// ```
    pub fn set_explicit<V>(&mut self, explicit: V)
    where
        V: Into<Option<String>>,
    {
        self.explicit = explicit.into();
    }

    /// Return whether the podcast is complete and no new episodes will be posted.
    ///
    /// A value of `Yes` indicates that the podcast is complete.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtension;
    ///
    /// let mut extension = ITunesChannelExtension::default();
    /// extension.set_complete("Yes".to_string());
    /// assert_eq!(extension.complete(), Some("Yes"));
    /// ```
    pub fn complete(&self) -> Option<&str> {
        self.complete.as_ref().map(String::as_str)
    }

    /// Set whether the podcast is complete and no new episodes will be posted.
    ///
    /// A value of `Yes` indicates that the podcast is complete.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtension;
    ///
    /// let mut extension = ITunesChannelExtension::default();
    /// extension.set_complete("Yes".to_string());
    /// ```
    pub fn set_complete<V>(&mut self, complete: V)
    where
        V: Into<Option<String>>,
    {
        self.complete = complete.into();
    }

    /// Return the new feed URL for this podcast.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtension;
    ///
    /// let mut extension = ITunesChannelExtension::default();
    /// extension.set_new_feed_url("http://example.com/feed".to_string());
    /// assert_eq!(extension.new_feed_url(), Some("http://example.com/feed"));
    /// ```
    pub fn new_feed_url(&self) -> Option<&str> {
        self.new_feed_url.as_ref().map(String::as_str)
    }

    /// Set the new feed URL for this podcast.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtension;
    ///
    /// let mut extension = ITunesChannelExtension::default();
    /// extension.set_new_feed_url("http://example.com/feed".to_string());
    /// ```
    pub fn set_new_feed_url<V>(&mut self, new_feed_url: V)
    where
        V: Into<Option<String>>,
    {
        self.new_feed_url = new_feed_url.into();
    }

    /// Return the contact information for the owner of this podcast.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtension, ITunesOwner};
    ///
    /// let mut extension = ITunesChannelExtension::default();
    /// extension.set_owner(ITunesOwner::default());
    /// assert!(extension.owner().is_some());
    /// ```
    pub fn owner(&self) -> Option<&ITunesOwner> {
        self.owner.as_ref()
    }

    /// Set the contact information for the owner of this podcast.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtension, ITunesOwner};
    ///
    /// let mut extension = ITunesChannelExtension::default();
    /// extension.set_owner(ITunesOwner::default());
    /// ```
    pub fn set_owner<V>(&mut self, owner: V)
    where
        V: Into<Option<ITunesOwner>>,
    {
        self.owner = owner.into();
    }

    /// Return the description of this podcast.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtension;
    ///
    /// let mut extension = ITunesChannelExtension::default();
    /// extension.set_subtitle("A podcast".to_string());
    /// assert_eq!(extension.subtitle(), Some("A podcast"));
    /// ```
    pub fn subtitle(&self) -> Option<&str> {
        self.subtitle.as_ref().map(String::as_str)
    }

    /// Set the description of this podcast.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtension;
    ///
    /// let mut extension = ITunesChannelExtension::default();
    /// extension.set_subtitle("A podcast".to_string());
    /// ```
    pub fn set_subtitle<V>(&mut self, subtitle: V)
    where
        V: Into<Option<String>>,
    {
        self.subtitle = subtitle.into();
    }

    /// Return the summary for this podcast.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtension;
    ///
    /// let mut extension = ITunesChannelExtension::default();
    /// extension.set_summary("A podcast".to_string());
    /// assert_eq!(extension.summary(), Some("A podcast"));
    /// ```
    pub fn summary(&self) -> Option<&str> {
        self.summary.as_ref().map(String::as_str)
    }

    /// Set the summary for this podcast.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtension;
    ///
    /// let mut extension = ITunesChannelExtension::default();
    /// extension.set_summary("A podcast about technology".to_string());
    /// ```
    pub fn set_summary<V>(&mut self, summary: V)
    where
        V: Into<Option<String>>,
    {
        self.summary = summary.into();
    }

    /// Return the keywords for this podcast.
    ///
    /// A comma separated list of keywords.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtension;
    ///
    /// let mut extension = ITunesChannelExtension::default();
    /// extension.set_keywords("technology".to_string());
    /// assert_eq!(extension.keywords(), Some("technology"));
    /// ```
    pub fn keywords(&self) -> Option<&str> {
        self.keywords.as_ref().map(String::as_str)
    }

    /// Set the keywords for this podcast.
    ///
    /// A comma separated list of keywords.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtension;
    ///
    /// let mut extension = ITunesChannelExtension::default();
    /// extension.set_keywords("technology".to_string());
    /// ```
    pub fn set_keywords<V>(&mut self, keywords: V)
    where
        V: Into<Option<String>>,
    {
        self.keywords = keywords.into();
    }
}

impl ITunesChannelExtension {
    /// Create an `ITunesChannelExtension` from a `HashMap`.
    pub fn from_map(mut map: HashMap<String, Vec<Extension>>) -> Self {
        let mut ext = ITunesChannelExtension::default();
        ext.author = remove_extension_value(&mut map, "author");
        ext.block = remove_extension_value(&mut map, "block");
        ext.categories = parse_categories(&mut map);
        ext.image = parse_image(&mut map);
        ext.explicit = remove_extension_value(&mut map, "explicit");
        ext.complete = remove_extension_value(&mut map, "complete");
        ext.new_feed_url = remove_extension_value(&mut map, "new-feed-url");
        ext.owner = parse_owner(&mut map);
        ext.subtitle = remove_extension_value(&mut map, "subtitle");
        ext.summary = remove_extension_value(&mut map, "summary");
        ext.keywords = remove_extension_value(&mut map, "keywords");
        ext
    }
}

impl ToXml for ITunesChannelExtension {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        if let Some(author) = self.author.as_ref() {
            writer.write_text_element(b"itunes:author", author)?;
        }

        if let Some(block) = self.block.as_ref() {
            writer.write_text_element(b"itunes:block", block)?;
        }

        writer.write_objects(&self.categories)?;

        if let Some(image) = self.image.as_ref() {
            let name = b"itunes:image";
            let mut element = BytesStart::borrowed(name, name.len());
            element.push_attribute(("href", &**image));
            writer.write_event(Event::Empty(element))?;
        }

        if let Some(explicit) = self.explicit.as_ref() {
            writer.write_text_element(b"itunes:explicit", explicit)?;
        }

        if let Some(complete) = self.complete.as_ref() {
            writer.write_text_element(b"itunes:complete", complete)?;
        }

        if let Some(new_feed_url) = self.new_feed_url.as_ref() {
            writer.write_text_element(b"itunes:new-feed-url", new_feed_url)?;
        }

        if let Some(owner) = self.owner.as_ref() {
            writer.write_object(owner)?;
        }

        if let Some(subtitle) = self.subtitle.as_ref() {
            writer.write_text_element(b"itunes:subtitle", subtitle)?;
        }

        if let Some(summary) = self.summary.as_ref() {
            writer.write_text_element(b"itunes:summary", summary)?;
        }

        if let Some(keywords) = self.keywords.as_ref() {
            writer.write_text_element(b"itunes:keywords", keywords)?;
        }

        Ok(())
    }
}
