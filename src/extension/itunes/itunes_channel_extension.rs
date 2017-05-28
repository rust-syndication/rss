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
    /// Get the optional author that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder, ITunesChannelExtension};
    ///
    /// let author = "author".to_string();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .author(author.clone())
    ///     .finalize();
    ///
    /// let author_opt = channel.author();
    /// assert!(author_opt.is_some());
    ///
    /// assert_eq!(author.clone(), author_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder, ITunesChannelExtension};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .author(None)
    ///     .finalize();
    ///
    /// let author_opt = channel.author();
    /// assert!(author_opt.is_none());
    /// ```
    pub fn author(&self) -> Option<&str> {
        self.author.as_ref().map(|s| s.as_str())
    }


    /// Get the optional block that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder, ITunesChannelExtension};
    ///
    /// let block = "block".to_string();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .block(block.clone())
    ///     .finalize();
    ///
    /// let block_opt = channel.block();
    /// assert!(block_opt.is_some());
    ///
    /// assert_eq!(block.clone(), block_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder, ITunesChannelExtension};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .block(None)
    ///     .finalize();
    ///
    /// let block_opt = channel.block();
    /// assert!(block_opt.is_none());
    /// ```
    pub fn block(&self) -> Option<&str> {
        self.block.as_ref().map(|s| s.as_str())
    }


    /// Get the categories that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder, ITunesChannelExtension,
    ///     ITunesCategoryBuilder};
    ///
    /// let subcategory = ITunesCategoryBuilder::new()
    ///     .text("text")
    ///     .finalize();
    ///
    /// let category = ITunesCategoryBuilder::new()
    ///     .text("text")
    ///     .subcategory(Box::new(subcategory))
    ///     .finalize();
    ///
    /// let categories_vec = vec![category];
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .categories(categories_vec)
    ///     .finalize();
    ///
    /// let categories = channel.categories();
    /// assert!(!categories.is_empty());
    /// ```
    pub fn categories(&self) -> &[ITunesCategory] {
        &self.categories
    }


    /// Get the optional image that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder, ITunesChannelExtension};
    ///
    /// let image = "image".to_string();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .image(image.clone())
    ///     .finalize();
    ///
    /// let image_opt = channel.image();
    /// assert!(image_opt.is_some());
    ///
    /// assert_eq!(image, image_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder, ITunesChannelExtension};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .image(None)
    ///     .finalize();
    ///
    /// let image_opt = channel.image();
    /// assert!(image_opt.is_none());
    /// ```
    pub fn image(&self) -> Option<&str> {
        self.image.as_ref().map(|s| s.as_str())
    }


    /// Get the optional explicit that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder, ITunesChannelExtension};
    ///
    /// let explicit = "explicit".to_string();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .explicit(explicit.clone())
    ///     .finalize();
    ///
    /// let explicit_opt = channel.explicit();
    /// assert!(explicit_opt.is_some());
    ///
    /// assert_eq!(explicit.clone(), explicit_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder, ITunesChannelExtension};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .explicit(None)
    ///     .finalize();
    ///
    /// let explicit_opt = channel.explicit();
    /// assert!(explicit_opt.is_none());
    /// ```
    pub fn explicit(&self) -> Option<&str> {
        self.explicit.as_ref().map(|s| s.as_str())
    }


    /// Get the optional complete that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder, ITunesChannelExtension};
    ///
    /// let complete = "complete".to_string();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .complete(complete.clone())
    ///     .finalize();
    ///
    /// let complete_opt = channel.complete();
    /// assert!(complete_opt.is_some());
    ///
    /// assert_eq!(complete.clone(), complete_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder, ITunesChannelExtension};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .complete(None)
    ///     .finalize();
    ///
    /// let complete_opt = channel.complete();
    /// assert!(complete_opt.is_none());
    /// ```
    pub fn complete(&self) -> Option<&str> {
        self.complete.as_ref().map(|s| s.as_str())
    }


    /// Get the optional new_feed_url that exists under
    /// `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder, ITunesChannelExtension};
    ///
    /// let new_feed_url = "new_feed_url".to_string();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .new_feed_url(new_feed_url.clone())
    ///     .finalize();
    ///
    /// let new_feed_url_opt = channel.new_feed_url();
    /// assert!(new_feed_url_opt.is_some());
    ///
    /// assert_eq!(new_feed_url.clone(), new_feed_url_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder, ITunesChannelExtension};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .new_feed_url(None)
    ///     .finalize();
    ///
    /// let new_feed_url_opt = channel.new_feed_url();
    /// assert!(new_feed_url_opt.is_none());
    /// ```
    pub fn new_feed_url(&self) -> Option<&str> {
        self.new_feed_url.as_ref().map(|s| s.as_str())
    }


    /// Get the optional owner that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder, ITunesChannelExtension,
    ///     ITunesOwnerBuilder};
    ///
    /// let owner = ITunesOwnerBuilder::new()
    ///     .email("email@example.com".to_string())
    ///     .name("name".to_string())
    ///     .finalize();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .owner(owner)
    ///     .finalize();
    ///
    /// let owner_opt = channel.owner();
    /// assert!(owner_opt.is_some());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder, ITunesChannelExtension};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .owner(None)
    ///     .finalize();
    ///
    /// let owner_opt = channel.owner();
    /// assert!(owner_opt.is_none());
    /// ```
    pub fn owner(&self) -> Option<&ITunesOwner> {
        self.owner.as_ref()
    }


    /// Get the optional subtitle that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder, ITunesChannelExtension};
    ///
    /// let subtitle = "subtitle".to_string();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .subtitle(subtitle.clone())
    ///     .finalize();
    ///
    /// let subtitle_opt = channel.subtitle();
    /// assert!(subtitle_opt.is_some());
    ///
    /// assert_eq!(subtitle.clone(), subtitle_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder, ITunesChannelExtension};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .subtitle(None)
    ///     .finalize();
    ///
    /// let subtitle_opt = channel.subtitle();
    /// assert!(subtitle_opt.is_none());
    /// ```
    pub fn subtitle(&self) -> Option<&str> {
        self.subtitle.as_ref().map(|s| s.as_str())
    }


    /// Get the optional summary that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder, ITunesChannelExtension};
    ///
    /// let summary = "summary".to_string();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .summary(summary.clone())
    ///     .finalize();
    ///
    /// let summary_opt = channel.summary();
    /// assert!(summary_opt.is_some());
    ///
    /// assert_eq!(summary.clone(), summary_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder, ITunesChannelExtension};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .summary(None)
    ///     .finalize();
    ///
    /// let summary_opt = channel.summary();
    /// assert!(summary_opt.is_none());
    /// ```
    pub fn summary(&self) -> Option<&str> {
        self.summary.as_ref().map(|s| s.as_str())
    }


    /// Get the optional keywords that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder, ITunesChannelExtension};
    ///
    /// let keywords = "keywords".to_string();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .keywords(keywords.clone())
    ///     .finalize();
    ///
    /// let keywords_opt = channel.keywords();
    /// assert!(keywords_opt.is_some());
    ///
    /// assert_eq!(keywords.clone(), keywords_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder, ITunesChannelExtension};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .keywords(None)
    ///     .finalize();
    ///
    /// let keywords_opt = channel.keywords();
    /// assert!(keywords_opt.is_none());
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

/// This `ITunesChannelExtensionBuilder` struct creates the
/// `ITunesChannelExtension`.
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
    /// Construct a new `ITunesChannelExtension` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let channel_builder = ITunesChannelExtensionBuilder::new();
    /// ```
    pub fn new() -> ITunesChannelExtensionBuilder {
        ITunesChannelExtensionBuilder::default()
    }


    /// Set the optional author that exists under `ITunesChannelExtension`.
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let channel_builder = ITunesChannelExtensionBuilder::new()
    ///     .author("author".to_string());
    /// ```
    pub fn author<V: Into<Option<String>>>(mut self, author: V) -> ITunesChannelExtensionBuilder {
        self.author = author.into();
        self
    }


    /// Set the optional block that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let channel_builder = ITunesChannelExtensionBuilder::new()
    ///     .block("block".to_string());
    /// ```
    pub fn block<V: Into<Option<String>>>(mut self, block: V) -> ITunesChannelExtensionBuilder {
        self.block = block.into();
        self
    }


    /// Set the categories that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesCategoryBuilder, ITunesChannelExtensionBuilder};
    ///
    /// let subcategory = ITunesCategoryBuilder::new()
    ///     .text("text")
    ///     .finalize();
    ///
    /// let category = ITunesCategoryBuilder::new()
    ///     .text("text")
    ///     .subcategory(Box::new(subcategory))
    ///     .finalize();
    ///
    /// let channel_builder = ITunesChannelExtensionBuilder::new()
    ///     .categories(vec![category]);
    /// ```
    pub fn categories<V>(mut self, categories: V) -> ITunesChannelExtensionBuilder
        where V: Into<Vec<ITunesCategory>>
    {
        self.categories = categories.into();
        self
    }


    /// Set the optional image that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let channel_builder = ITunesChannelExtensionBuilder::new()
    ///     .image("image".to_string());
    /// ```
    pub fn image<V: Into<Option<String>>>(mut self, image: V) -> ITunesChannelExtensionBuilder {
        self.image = image.into();
        self
    }


    /// Set the optional explicit that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let channel_builder = ITunesChannelExtensionBuilder::new()
    ///     .explicit("explicit".to_string());
    /// ```
    pub fn explicit<V>(mut self, explicit: V) -> ITunesChannelExtensionBuilder
        where V: Into<Option<String>>
    {
        self.explicit = explicit.into();
        self
    }


    /// Set the optional complete that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let channel_builder = ITunesChannelExtensionBuilder::new()
    ///     .complete("complete".to_string());
    /// ```
    pub fn complete<V>(mut self, complete: V) -> ITunesChannelExtensionBuilder
        where V: Into<Option<String>>
    {
        self.complete = complete.into();
        self
    }


    /// Set the optional new_feed_url that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let channel_builder = ITunesChannelExtensionBuilder::new()
    ///     .new_feed_url("new_feed_url".to_string());
    /// ```
    pub fn new_feed_url<V>(mut self, new_feed_url: V) -> ITunesChannelExtensionBuilder
        where V: Into<Option<String>>
    {
        self.new_feed_url = new_feed_url.into();
        self
    }


    /// Set the optional owner that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder, ITunesOwnerBuilder};
    ///
    /// let owner = ITunesOwnerBuilder::new()
    ///     .email("email@example.com".to_string())
    ///     .name("name".to_string())
    ///     .finalize();
    ///
    /// let channel_builder = ITunesChannelExtensionBuilder::new()
    ///     .owner(owner);
    /// ```
    pub fn owner<V>(mut self, owner: V) -> ITunesChannelExtensionBuilder
        where V: Into<Option<ITunesOwner>>
    {
        self.owner = owner.into();
        self
    }


    /// Set the optional subtitle that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let channel_builder = ITunesChannelExtensionBuilder::new()
    ///     .subtitle("subtitle".to_string());
    /// ```
    pub fn subtitle<V>(mut self, subtitle: V) -> ITunesChannelExtensionBuilder
        where V: Into<Option<String>>
    {
        self.subtitle = subtitle.into();
        self
    }


    /// Set the optional summary that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let channel_builder = ITunesChannelExtensionBuilder::new()
    ///     .summary("summary".to_string());
    /// ```
    pub fn summary<V: Into<Option<String>>>(mut self, summary: V) -> ITunesChannelExtensionBuilder {
        self.summary = summary.into();
        self
    }


    /// Set the optional keywords that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let channel_builder = ITunesChannelExtensionBuilder::new()
    ///     .keywords("keywords".to_string());
    /// ```
    pub fn keywords<V>(mut self, keywords: V) -> ITunesChannelExtensionBuilder
        where V: Into<Option<String>>
    {
        self.keywords = keywords.into();
        self
    }


    /// Construct the `ITunesChannelExtension` from the `ITunesChannelExtensionBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesCategoryBuilder, ITunesChannelExtensionBuilder,
    ///     ITunesOwnerBuilder};
    ///
    /// let owner = ITunesOwnerBuilder::new()
    ///     .email("email@example.com".to_string())
    ///     .name("name".to_string())
    ///     .finalize();
    ///
    /// let subcategory = ITunesCategoryBuilder::new()
    ///     .text("text")
    ///     .finalize();
    ///
    /// let category = ITunesCategoryBuilder::new()
    ///     .text("text")
    ///     .subcategory(Box::new(subcategory))
    ///     .finalize();
    ///
    /// let categories = vec![category];
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .author("author".to_string())
    ///     .block("block".to_string())
    ///     .image("image".to_string())
    ///     .explicit("explicit".to_string())
    ///     .subtitle("subtitle".to_string())
    ///     .summary("summary".to_string())
    ///     .keywords("keywords".to_string())
    ///     .new_feed_url("new_feed_url".to_string())
    ///     .complete("complete".to_string())
    ///     .owner(owner)
    ///     .categories(categories)
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
