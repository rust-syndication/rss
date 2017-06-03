// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use super::{parse_categories, parse_image, parse_owner};
use error::Error;
use extension::Extension;
use extension::itunes::ITunesCategory;
use extension::itunes::ITunesOwner;
use extension::remove_extension_value;
use quick_xml::errors::Error as XmlError;
use quick_xml::events::{Event, BytesStart};
use quick_xml::writer::Writer;
use std::collections::HashMap;
use toxml::{ToXml, WriterExt};

/// An iTunes channel element extension.
#[derive(Debug, Default, Clone, PartialEq)]
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
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let author = "author";
    ///
    /// let ext = ITunesChannelExtensionBuilder::default()
    ///     .author(author.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(author), ext.author());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let ext = ITunesChannelExtensionBuilder::default()
    ///     .author(None)
    ///     .finalize();
    ///
    /// assert!(ext.author().is_none());
    /// ```
    pub fn author(&self) -> Option<&str> {
        self.author.as_ref().map(|s| s.as_str())
    }

    /// Return whether the podcast should be blocked from appearing in the iTunes Store.
    ///
    /// A value of `Yes` indicates that the podcast should not show up in the iTunes Store. All
    /// other values are ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let block = "Yes";
    ///
    /// let ext = ITunesChannelExtensionBuilder::default()
    ///     .block(block.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(block), ext.block());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let ext = ITunesChannelExtensionBuilder::default()
    ///     .block(None)
    ///     .finalize();
    ///
    /// assert!(ext.block().is_none());
    /// ```
    pub fn block(&self) -> Option<&str> {
        self.block.as_ref().map(|s| s.as_str())
    }

    /// Return the iTunes categories that the podcast belongs to.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder, ITunesCategoryBuilder};
    ///
    /// let category = ITunesCategoryBuilder::default()
    ///     .text("category")
    ///     .finalize();
    ///
    /// let categories = vec![category];
    ///
    /// let ext = ITunesChannelExtensionBuilder::default()
    ///     .categories(categories.clone())
    ///     .finalize();
    ///
    /// assert_eq!(categories, ext.categories());
    /// ```
    pub fn categories(&self) -> &[ITunesCategory] {
        &self.categories
    }

    /// Return the artwork URL for the podcast.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let image = "http://example.com/image.png";
    ///
    /// let ext = ITunesChannelExtensionBuilder::default()
    ///     .image(image.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(image), ext.image());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let ext = ITunesChannelExtensionBuilder::default()
    ///     .image(None)
    ///     .finalize();
    ///
    /// assert!(ext.image().is_none());
    /// ```
    pub fn image(&self) -> Option<&str> {
        self.image.as_ref().map(|s| s.as_str())
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
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let explicit = "Yes";
    ///
    /// let ext = ITunesChannelExtensionBuilder::default()
    ///     .explicit(explicit.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(explicit), ext.explicit());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let ext = ITunesChannelExtensionBuilder::default()
    ///     .explicit(None)
    ///     .finalize();
    ///
    /// assert!(ext.explicit().is_none());
    /// ```
    pub fn explicit(&self) -> Option<&str> {
        self.explicit.as_ref().map(|s| s.as_str())
    }

    /// Return whether the podcast is complete and no new episodes will be posted.
    ///
    /// A value of `Yes` indicates that the podcast is complete.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let complete = "Yes";
    ///
    /// let ext = ITunesChannelExtensionBuilder::default()
    ///     .complete(complete.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(complete), ext.complete());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let ext = ITunesChannelExtensionBuilder::default()
    ///     .complete(None)
    ///     .finalize();
    ///
    /// assert!(ext.complete().is_none());
    /// ```
    pub fn complete(&self) -> Option<&str> {
        self.complete.as_ref().map(|s| s.as_str())
    }

    /// Return the new feed URL for this podcast.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let url = "http://example.com/feed";
    ///
    /// let ext = ITunesChannelExtensionBuilder::default()
    ///     .new_feed_url(url.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(url), ext.new_feed_url());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let ext = ITunesChannelExtensionBuilder::default()
    ///     .new_feed_url(None)
    ///     .finalize();
    ///
    /// assert!(ext.new_feed_url().is_none());
    /// ```
    pub fn new_feed_url(&self) -> Option<&str> {
        self.new_feed_url.as_ref().map(|s| s.as_str())
    }

    /// Return the contact information for the owner of this podcast.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder, ITunesOwnerBuilder};
    ///
    /// let owner = ITunesOwnerBuilder::default()
    ///     .email("email@example.com".to_string())
    ///     .name("name".to_string())
    ///     .finalize();
    ///
    /// let ext = ITunesChannelExtensionBuilder::default()
    ///     .owner(owner)
    ///     .finalize();
    ///
    /// assert!(ext.owner().is_some());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let ext = ITunesChannelExtensionBuilder::default()
    ///     .owner(None)
    ///     .finalize();
    ///
    /// assert!(ext.owner().is_none());
    /// ```
    pub fn owner(&self) -> Option<&ITunesOwner> {
        self.owner.as_ref()
    }

    /// Return the description of this podcast.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let subtitle = "description";
    ///
    /// let ext = ITunesChannelExtensionBuilder::default()
    ///     .subtitle(subtitle.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(subtitle), ext.subtitle())
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let ext = ITunesChannelExtensionBuilder::default()
    ///     .subtitle(None)
    ///     .finalize();
    ///
    /// assert!(ext.subtitle().is_none());
    /// ```
    pub fn subtitle(&self) -> Option<&str> {
        self.subtitle.as_ref().map(|s| s.as_str())
    }

    /// Return the summary for this podcast.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let summary = "summary";
    ///
    /// let ext = ITunesChannelExtensionBuilder::default()
    ///     .summary(summary.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(summary), ext.summary());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let ext = ITunesChannelExtensionBuilder::default()
    ///     .summary(None)
    ///     .finalize();
    ///
    /// assert!(ext.summary().is_none());
    /// ```
    pub fn summary(&self) -> Option<&str> {
        self.summary.as_ref().map(|s| s.as_str())
    }

    /// Return the keywords for this podcast.
    ///
    /// The string contains a comma separated list of keywords.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let keywords = "keyword1,keyword2";
    ///
    /// let ext = ITunesChannelExtensionBuilder::default()
    ///     .keywords(keywords.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(keywords), ext.keywords());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let ext = ITunesChannelExtensionBuilder::default()
    ///     .keywords(None)
    ///     .finalize();
    ///
    /// assert!(ext.keywords().is_none());
    /// ```
    pub fn keywords(&self) -> Option<&str> {
        self.keywords.as_ref().map(|s| s.as_str())
    }
}

impl ITunesChannelExtension {
    /// Creates an ITunesChannelExtension using the specified hashmap.
    pub fn from_map(mut map: HashMap<String, Vec<Extension>>) -> Result<Self, Error> {
        let mut ext = ITunesChannelExtension::default();
        ext.author = remove_extension_value(&mut map, "author");
        ext.block = remove_extension_value(&mut map, "block");
        ext.categories = parse_categories(&mut map)?;
        ext.image = parse_image(&mut map);
        ext.explicit = remove_extension_value(&mut map, "explicit");
        ext.complete = remove_extension_value(&mut map, "complete");
        ext.new_feed_url = remove_extension_value(&mut map, "new-feed-url");
        ext.owner = parse_owner(&mut map)?;
        ext.subtitle = remove_extension_value(&mut map, "subtitle");
        ext.summary = remove_extension_value(&mut map, "summary");
        ext.keywords = remove_extension_value(&mut map, "keywords");
        Ok(ext)
    }
}

impl ToXml for ITunesChannelExtension {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
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
            writer
                .write_text_element(b"itunes:new-feed-url", new_feed_url)?;
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

/// A builder used to create an `ITunesChannelExtension`.
#[derive(Debug, Clone, Default)]
pub struct ITunesChannelExtensionBuilder {
    author: Option<String>,
    block: Option<String>,
    categories: Vec<ITunesCategory>,
    image: Option<String>,
    explicit: Option<String>,
    complete: Option<String>,
    new_feed_url: Option<String>,
    owner: Option<ITunesOwner>,
    subtitle: Option<String>,
    summary: Option<String>,
    keywords: Option<String>,
}

impl ITunesChannelExtensionBuilder {
    /// Construct a new `ITunesChannelExtensionBuilder` using the values from an existing
    /// `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let input = include_str!("tests/data/itunes.xml");
    /// let channel = input.parse::<Channel>().unwrap();
    /// let extension = channel.itunes_ext().unwrap().clone();
    /// let builder = ITunesChannelExtensionBuilder::from_extension(extension);
    /// ```
    pub fn from_extension(extension: ITunesChannelExtension) -> Self {
        ITunesChannelExtensionBuilder {
            author: extension.author,
            block: extension.block,
            categories: extension.categories,
            image: extension.image,
            explicit: extension.explicit,
            complete: extension.complete,
            new_feed_url: extension.new_feed_url,
            owner: extension.owner,
            subtitle: extension.subtitle,
            summary: extension.summary,
            keywords: extension.keywords,
        }
    }

    /// Set the author of the podcast.
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let builder = ITunesChannelExtensionBuilder::default()
    ///     .author("author".to_string());
    /// ```
    pub fn author<V>(mut self, author: V) -> ITunesChannelExtensionBuilder
        where V: Into<Option<String>>
    {
        self.author = author.into();
        self
    }

    /// Set whether the podcast should be blocked from appearing in the iTunes Store.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let builder = ITunesChannelExtensionBuilder::default()
    ///     .block("Yes".to_string());
    /// ```
    pub fn block<V>(mut self, block: V) -> ITunesChannelExtensionBuilder
        where V: Into<Option<String>>
    {
        self.block = block.into();
        self
    }

    /// Set the iTunes categories that the podcast belongs to.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesCategoryBuilder, ITunesChannelExtensionBuilder};
    ///
    /// let category = ITunesCategoryBuilder::default()
    ///     .text("category")
    ///     .finalize();
    ///
    /// let builder = ITunesChannelExtensionBuilder::default()
    ///     .categories(vec![category]);
    /// ```
    pub fn categories<V>(mut self, categories: V) -> ITunesChannelExtensionBuilder
        where V: Into<Vec<ITunesCategory>>
    {
        self.categories = categories.into();
        self
    }

    /// Set the artwork URL for the podcast.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let builder = ITunesChannelExtensionBuilder::default()
    ///     .image("http://example.com/image.png".to_string());
    /// ```
    pub fn image<V>(mut self, image: V) -> ITunesChannelExtensionBuilder
        where V: Into<Option<String>>
    {
        self.image = image.into();
        self
    }

    /// Set whether the podcast contains explicit content.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let builder = ITunesChannelExtensionBuilder::default()
    ///     .explicit("Yes".to_string());
    /// ```
    pub fn explicit<V>(mut self, explicit: V) -> ITunesChannelExtensionBuilder
        where V: Into<Option<String>>
    {
        self.explicit = explicit.into();
        self
    }

    /// Set whether the podcast is complete and no new episodes will be posted.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let builder = ITunesChannelExtensionBuilder::default()
    ///     .complete("Yes".to_string());
    /// ```
    pub fn complete<V>(mut self, complete: V) -> ITunesChannelExtensionBuilder
        where V: Into<Option<String>>
    {
        self.complete = complete.into();
        self
    }

    /// Set the new feed URL for the podcast.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let builder = ITunesChannelExtensionBuilder::default()
    ///     .new_feed_url("http://example.com/feed".to_string());
    /// ```
    pub fn new_feed_url<V>(mut self, new_feed_url: V) -> ITunesChannelExtensionBuilder
        where V: Into<Option<String>>
    {
        self.new_feed_url = new_feed_url.into();
        self
    }

    /// Set the contact information for the owner of the podcast.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder, ITunesOwnerBuilder};
    ///
    /// let owner = ITunesOwnerBuilder::default()
    ///     .email("email@example.com".to_string())
    ///     .name("name".to_string())
    ///     .finalize();
    ///
    /// let builder = ITunesChannelExtensionBuilder::default()
    ///     .owner(owner);
    /// ```
    pub fn owner<V>(mut self, owner: V) -> ITunesChannelExtensionBuilder
        where V: Into<Option<ITunesOwner>>
    {
        self.owner = owner.into();
        self
    }

    /// Set the description of the podcast.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let builder = ITunesChannelExtensionBuilder::default()
    ///     .subtitle("description".to_string());
    /// ```
    pub fn subtitle<V>(mut self, subtitle: V) -> ITunesChannelExtensionBuilder
        where V: Into<Option<String>>
    {
        self.subtitle = subtitle.into();
        self
    }

    /// Set the summary for the podcast.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let builder = ITunesChannelExtensionBuilder::default()
    ///     .summary("summary".to_string());
    /// ```
    pub fn summary<V>(mut self, summary: V) -> ITunesChannelExtensionBuilder
        where V: Into<Option<String>>
    {
        self.summary = summary.into();
        self
    }

    /// Set the keywords for the podcast.
    ///
    /// The string should be a comma separated list of keywords.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let builder = ITunesChannelExtensionBuilder::default()
    ///     .keywords("keyword1,keyword2".to_string());
    /// ```
    pub fn keywords<V>(mut self, keywords: V) -> ITunesChannelExtensionBuilder
        where V: Into<Option<String>>
    {
        self.keywords = keywords.into();
        self
    }

    /// Construct the `ITunesChannelExtension` from this `ITunesChannelExtensionBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesCategoryBuilder, ITunesChannelExtensionBuilder,
    ///     ITunesOwnerBuilder};
    ///
    /// let owner = ITunesOwnerBuilder::default()
    ///     .email("email@example.com".to_string())
    ///     .name("name".to_string())
    ///     .finalize();
    ///
    /// let ext = ITunesChannelExtensionBuilder::default()
    ///     .owner(owner)
    ///     .finalize();
    /// ```
    pub fn finalize(self) -> ITunesChannelExtension {
        ITunesChannelExtension {
            author: self.author,
            block: self.block,
            categories: self.categories,
            image: self.image,
            explicit: self.explicit,
            complete: self.complete,
            new_feed_url: self.new_feed_url,
            owner: self.owner,
            subtitle: self.subtitle,
            summary: self.summary,
            keywords: self.keywords,
        }
    }
}
