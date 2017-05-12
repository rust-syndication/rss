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
use quick_xml::{Element, Event, XmlWriter};
use quick_xml::error::Error as XmlError;
use std::collections::HashMap;
use toxml::{ToXml, XmlWriterExt};

/// An iTunes channel element extension.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ITunesChannelExtension
{
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

impl ITunesChannelExtension
{
    /// Get the optional author that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtension};
    ///
    /// let author = "author".to_owned();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .author(Some(author.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let author_opt = channel.author();
    /// assert!(author_opt.is_some());
    ///
    /// assert_eq!(author.clone(), author_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtension};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .author(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let author_opt = channel.author();
    /// assert!(author_opt.is_none());
    /// ```
    pub fn author(&self) -> Option<String>
    {
        self.author.clone()
    }


    /// Get the optional block that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtension};
    ///
    /// let block = "block".to_owned();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .block(Some(block.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let block_opt = channel.block();
    /// assert!(block_opt.is_some());
    ///
    /// assert_eq!(block.clone(), block_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtension};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .block(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let block_opt = channel.block();
    /// assert!(block_opt.is_none());
    /// ```
    pub fn block(&self) -> Option<String>
    {
        self.block.clone()
    }


    /// Get the categories that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtension, ITunesCategoryBuilder};
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
    ///
    /// let categories_vec = vec![category];
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .categories(categories_vec)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let categories = channel.categories();
    /// assert!(!categories.is_empty());
    /// ```
    pub fn categories(&self) -> Vec<ITunesCategory>
    {
        self.categories.clone()
    }


    /// Get the optional image that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtension};
    ///
    /// let image = "image".to_owned();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .image(Some(image.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let image_opt = channel.image();
    /// assert!(image_opt.is_some());
    ///
    /// assert_eq!(image.clone(), image_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtension};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .image(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let image_opt = channel.image();
    /// assert!(image_opt.is_none());
    /// ```
    pub fn image(&self) -> Option<String>
    {
        self.image.clone()
    }


    /// Get the optional explicit that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtension};
    ///
    /// let explicit = "explicit".to_owned();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .explicit(Some(explicit.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let explicit_opt = channel.explicit();
    /// assert!(explicit_opt.is_some());
    ///
    /// assert_eq!(explicit.clone(), explicit_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtension};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .explicit(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let explicit_opt = channel.explicit();
    /// assert!(explicit_opt.is_none());
    /// ```
    pub fn explicit(&self) -> Option<String>
    {
        self.explicit.clone()
    }


    /// Get the optional complete that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtension};
    ///
    /// let complete = "complete".to_owned();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .complete(Some(complete.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let complete_opt = channel.complete();
    /// assert!(complete_opt.is_some());
    ///
    /// assert_eq!(complete.clone(), complete_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtension};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .complete(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let complete_opt = channel.complete();
    /// assert!(complete_opt.is_none());
    /// ```
    pub fn complete(&self) -> Option<String>
    {
        self.complete.clone()
    }


    /// Get the optional new_feed_url that exists under
    /// `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtension};
    ///
    /// let new_feed_url = "new_feed_url".to_owned();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .new_feed_url(Some(new_feed_url.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let new_feed_url_opt = channel.new_feed_url();
    /// assert!(new_feed_url_opt.is_some());
    ///
    /// assert_eq!(new_feed_url.clone(), new_feed_url_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtension};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .new_feed_url(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let new_feed_url_opt = channel.new_feed_url();
    /// assert!(new_feed_url_opt.is_none());
    /// ```
    pub fn new_feed_url(&self) -> Option<String>
    {
        self.new_feed_url.clone()
    }


    /// Get the optional owner that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtension, ITunesOwnerBuilder};
    ///
    /// let owner = ITunesOwnerBuilder::new()
    ///     .email(Some("email@example.com".to_owned()))
    ///     .name(Some("name".to_owned()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .owner(Some(owner))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let owner_opt = channel.owner();
    /// assert!(owner_opt.is_some());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtension};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .owner(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let owner_opt = channel.owner();
    /// assert!(owner_opt.is_none());
    /// ```
    pub fn owner(&self) -> Option<ITunesOwner>
    {
        self.owner.clone()
    }


    /// Get the optional subtitle that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtension};
    ///
    /// let subtitle = "subtitle".to_owned();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .subtitle(Some(subtitle.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let subtitle_opt = channel.subtitle();
    /// assert!(subtitle_opt.is_some());
    ///
    /// assert_eq!(subtitle.clone(), subtitle_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtension};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .subtitle(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let subtitle_opt = channel.subtitle();
    /// assert!(subtitle_opt.is_none());
    /// ```
    pub fn subtitle(&self) -> Option<String>
    {
        self.subtitle.clone()
    }


    /// Get the optional summary that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtension};
    ///
    /// let summary = "summary".to_owned();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .summary(Some(summary.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let summary_opt = channel.summary();
    /// assert!(summary_opt.is_some());
    ///
    /// assert_eq!(summary.clone(), summary_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtension};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .summary(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let summary_opt = channel.summary();
    /// assert!(summary_opt.is_none());
    /// ```
    pub fn summary(&self) -> Option<String>
    {
        self.summary.clone()
    }


    /// Get the optional keywords that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtension};
    ///
    /// let keywords = "keywords".to_owned();
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .keywords(Some(keywords.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let keywords_opt = channel.keywords();
    /// assert!(keywords_opt.is_some());
    ///
    /// assert_eq!(keywords.clone(), keywords_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesChannelExtension};
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .keywords(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let keywords_opt = channel.keywords();
    /// assert!(keywords_opt.is_none());
    /// ```
    pub fn keywords(&self) -> Option<String>
    {
        self.keywords.clone()
    }
}

impl ITunesChannelExtension
{
    /// Creates an ITunesChannelExtension using the specified hashmap.
    pub fn from_map(mut map: HashMap<String, Vec<Extension>>) -> Result<Self, Error>
    {
        let mut ext = ITunesChannelExtension::default();
        ext.author = remove_extension_value(&mut map,
                                            "author");
        ext.block = remove_extension_value(&mut map,
                                           "block");
        ext.categories = parse_categories(&mut map)?;
        ext.image = parse_image(&mut map);
        ext.explicit = remove_extension_value(&mut map,
                                              "explicit");
        ext.complete = remove_extension_value(&mut map,
                                              "complete");
        ext.new_feed_url = remove_extension_value(&mut map,
                                                  "new-feed-url");
        ext.owner = parse_owner(&mut map)?;
        ext.subtitle = remove_extension_value(&mut map,
                                              "subtitle");
        ext.summary = remove_extension_value(&mut map,
                                             "summary");
        ext.keywords = remove_extension_value(&mut map,
                                              "keywords");
        Ok(ext)
    }
}

impl ToXml for ITunesChannelExtension
{
    /// TODO: document from xml
    fn to_xml<W: ::std::io::Write>(&self,
                                   writer: &mut XmlWriter<W>)
        -> Result<(), XmlError>
    {
        if let Some(author) = self.author.as_ref() {
            writer.write_text_element(b"itunes:author",
                                      author)?;
        }

        if let Some(block) = self.block.as_ref() {
            writer.write_text_element(b"itunes:block",
                                      block)?;
        }

        writer.write_objects(&self.categories)?;

        if let Some(image) = self.image.as_ref() {
            let element = Element::new(b"itunes:image");
            writer.write(Event::Start({
                                          let mut element = element.clone();
                                          element.extend_attributes(::std::iter::once((b"href", image)));
                                          element
                                      }))?;
            writer.write(Event::End(element))?;
        }

        if let Some(explicit) = self.explicit.as_ref() {
            writer.write_text_element(b"itunes:explicit",
                                      explicit)?;
        }

        if let Some(complete) = self.complete.as_ref() {
            writer.write_text_element(b"itunes:complete",
                                      complete)?;
        }

        if let Some(new_feed_url) = self.new_feed_url.as_ref() {
            writer.write_text_element(b"itunes:new-feed-url",
                                      new_feed_url)?;
        }

        if let Some(owner) = self.owner.as_ref() {
            writer.write_object(owner)?;
        }

        if let Some(subtitle) = self.subtitle.as_ref() {
            writer.write_text_element(b"itunes:subtitle",
                                      subtitle)?;
        }

        if let Some(summary) = self.summary.as_ref() {
            writer.write_text_element(b"itunes:summary",
                                      summary)?;
        }

        if let Some(keywords) = self.keywords.as_ref() {
            writer.write_text_element(b"itunes:keywords",
                                      keywords)?;
        }

        Ok(())
    }
}

/// This `ITunesChannelExtensionBuilder` struct creates the
/// `ITunesChannelExtension`.
#[derive(Debug, Clone, Default)]
pub struct ITunesChannelExtensionBuilder
{
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

impl ITunesChannelExtensionBuilder
{
    /// Construct a new `ITunesChannelExtension` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let channel_builder = ITunesChannelExtensionBuilder::new();
    /// ```
    pub fn new() -> ITunesChannelExtensionBuilder
    {
        ITunesChannelExtensionBuilder::default()
    }


    /// Set the optional author that exists under `ITunesChannelExtension`.
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let mut channel_builder = ITunesChannelExtensionBuilder::new();
    /// channel_builder.author(Some("author".to_owned()));
    /// ```
    pub fn author(&mut self,
                  author: Option<String>)
        -> &mut ITunesChannelExtensionBuilder
    {
        self.author = author;
        self
    }


    /// Set the optional block that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let mut channel_builder = ITunesChannelExtensionBuilder::new();
    /// channel_builder.block(Some("block".to_owned()));
    /// ```
    pub fn block(&mut self,
                 block: Option<String>)
        -> &mut ITunesChannelExtensionBuilder
    {
        self.block = block;
        self
    }


    /// Set the categories that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesCategoryBuilder,
    /// ITunesChannelExtensionBuilder};
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
    ///
    /// let categories = vec![category];
    ///
    /// let mut channel_builder = ITunesChannelExtensionBuilder::new();
    /// channel_builder.categories(categories);
    /// ```
    pub fn categories(&mut self,
                      categories: Vec<ITunesCategory>)
        -> &mut ITunesChannelExtensionBuilder
    {
        self.categories = categories;
        self
    }


    /// Set the optional image that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let mut channel_builder = ITunesChannelExtensionBuilder::new();
    /// channel_builder.image(Some("image".to_owned()));
    /// ```
    pub fn image(&mut self,
                 image: Option<String>)
        -> &mut ITunesChannelExtensionBuilder
    {
        self.image = image;
        self
    }


    /// Set the optional explicit that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let mut channel_builder = ITunesChannelExtensionBuilder::new();
    /// channel_builder.explicit(Some("explicit".to_owned()));
    /// ```
    pub fn explicit(&mut self,
                    explicit: Option<String>)
        -> &mut ITunesChannelExtensionBuilder
    {
        self.explicit = explicit;
        self
    }


    /// Set the optional complete that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let mut channel_builder = ITunesChannelExtensionBuilder::new();
    /// channel_builder.complete(Some("complete".to_owned()));
    /// ```
    pub fn complete(&mut self,
                    complete: Option<String>)
        -> &mut ITunesChannelExtensionBuilder
    {
        self.complete = complete;
        self
    }


    /// Set the optional new_feed_url that exists under
    /// `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let mut channel_builder = ITunesChannelExtensionBuilder::new();
    /// channel_builder.new_feed_url(Some("new_feed_url".to_owned()));
    /// ```
    pub fn new_feed_url(&mut self,
                        new_feed_url: Option<String>)
        -> &mut ITunesChannelExtensionBuilder
    {
        self.new_feed_url = new_feed_url;
        self
    }


    /// Set the optional owner that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesOwnerBuilder};
    ///
    /// let owner = ITunesOwnerBuilder::new()
    ///     .email(Some("email@example.com".to_owned()))
    ///     .name(Some("name".to_owned()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let mut channel_builder = ITunesChannelExtensionBuilder::new();
    /// channel_builder.owner(Some(owner));
    /// ```
    pub fn owner(&mut self,
                 owner: Option<ITunesOwner>)
        -> &mut ITunesChannelExtensionBuilder
    {
        self.owner = owner;
        self
    }


    /// Set the optional subtitle that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let mut channel_builder = ITunesChannelExtensionBuilder::new();
    /// channel_builder.subtitle(Some("subtitle".to_owned()));
    /// ```
    pub fn subtitle(&mut self,
                    subtitle: Option<String>)
        -> &mut ITunesChannelExtensionBuilder
    {
        self.subtitle = subtitle;
        self
    }


    /// Set the optional summary that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let mut channel_builder = ITunesChannelExtensionBuilder::new();
    /// channel_builder.summary(Some("summary".to_owned()));
    /// ```
    pub fn summary(&mut self,
                   summary: Option<String>)
        -> &mut ITunesChannelExtensionBuilder
    {
        self.summary = summary;
        self
    }


    /// Set the optional keywords that exists under `ITunesChannelExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesChannelExtensionBuilder;
    ///
    /// let mut channel_builder = ITunesChannelExtensionBuilder::new();
    /// channel_builder.keywords(Some("keywords".to_owned()));
    /// ```
    pub fn keywords(&mut self,
                    keywords: Option<String>)
        -> &mut ITunesChannelExtensionBuilder
    {
        self.keywords = keywords;
        self
    }


    /// Construct the `ITunesChannelExtension` from the
    /// `ITunesChannelExtensionBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesCategoryBuilder,
    /// ITunesChannelExtensionBuilder, ITunesOwnerBuilder};
    ///
    /// let owner = ITunesOwnerBuilder::new()
    ///     .email(Some("email@example.com".to_owned()))
    ///     .name(Some("name".to_owned()))
    ///     .finalize()
    ///     .unwrap();
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
    ///
    /// let categories = vec![category];
    ///
    /// let channel = ITunesChannelExtensionBuilder::new()
    ///     .author(Some("author".to_owned()))
    ///     .block(Some("block".to_owned()))
    ///     .image(Some("image".to_owned()))
    ///     .explicit(Some("explicit".to_owned()))
    ///     .subtitle(Some("subtitle".to_owned()))
    ///     .summary(Some("summary".to_owned()))
    ///     .keywords(Some("keywords".to_owned()))
    ///     .new_feed_url(Some("new_feed_url".to_owned()))
    ///     .complete(Some("complete".to_owned()))
    ///     .owner(Some(owner))
    ///     .categories(categories)
    ///     .finalize()
    ///     .unwrap();
    /// ```
    pub fn finalize(&self) -> Result<ITunesChannelExtension, Error>
    {
        Ok(ITunesChannelExtension { author: self.author.clone(),
                                    block: self.block.clone(),
                                    categories: self.categories.clone(),
                                    image: self.image.clone(),
                                    explicit: self.explicit.clone(),
                                    complete: self.complete.clone(),
                                    new_feed_url: self.new_feed_url.clone(),
                                    owner: self.owner.clone(),
                                    subtitle: self.subtitle.clone(),
                                    summary: self.summary.clone(),
                                    keywords: self.keywords.clone(), })
    }
}
