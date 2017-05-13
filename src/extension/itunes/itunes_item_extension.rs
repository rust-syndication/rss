// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use super::parse_image;
use error::Error;
use extension::Extension;
use extension::remove_extension_value;
use quick_xml::{Element, Event, XmlWriter};
use quick_xml::error::Error as XmlError;
use std::collections::HashMap;
use toxml::{ToXml, XmlWriterExt};

/// An iTunes item element extension.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ITunesItemExtension
{
    /// The author of the podcast episode.
    author: Option<String>,
    /// Specifies if the podcast episode should be prevented from appearing in the iTunes Store. A
    /// value of `Yes` indicates that the episode should not show up in the iTunes Store. All other
    /// values are ignored.
    block: Option<String>,
    /// The artwork for the podcast episode.
    image: Option<String>,
    /// The podcast episode duration in one of the following formats: HH:MM:SS, H:MM:SS, MM:SS,
    /// M:SS.
    duration: Option<String>,
    /// Specifies whether the podcast episode contains explicit content. A value of `Yes`,
    /// `Explicit`, or `True` indicates that the episode contains explicit content. A value of
    /// `Clean`, `No`, `False` inidicates that episode does not contain explicit content.
    explicit: Option<String>,
    /// Specifies whether the podcast episode contains embedded closed captioning. A value of `Yes`
    /// indicates that it does. Any other value indicates that it does not.
    closed_captioned: Option<String>,
    /// A value used to override the default sorting order for episodes.
    order: Option<String>,
    /// A description of the podcast episode.
    subtitle: Option<String>,
    /// A summary of the podcast episode.
    summary: Option<String>,
    /// Keywords for the podcast. The string contains a comma separated list of keywords.
    keywords: Option<String>,
}

impl ITunesItemExtension
{
    /// Get the optional author that exists under `ITunesItemExtension`.
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let author = "author".to_owned();
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .author(Some(author.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let author_opt = item.author();
    /// assert!(author_opt.is_some());
    ///
    /// assert_eq!(author.clone(), author_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .author(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let author_opt = item.author();
    /// assert!(author_opt.is_none());
    /// ```
    pub fn author(&self) -> Option<String>
    {
        self.author
            .clone()
    }


    /// Get the optional block that exists under `ITunesItemExtension`.
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let block = "block".to_owned();
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .block(Some(block.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let block_opt = item.block();
    /// assert!(block_opt.is_some());
    ///
    /// assert_eq!(block.clone(), block_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .block(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let block_opt = item.block();
    /// assert!(block_opt.is_none());
    /// ```
    pub fn block(&self) -> Option<String>
    {
        self.block
            .clone()
    }


    /// Get the optional image that exists under `ITunesItemExtension`.
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let image = "image".to_owned();
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .image(Some(image.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let image_opt = item.image();
    /// assert!(image_opt.is_some());
    ///
    /// assert_eq!(image.clone(), image_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .image(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let image_opt = item.image();
    /// assert!(image_opt.is_none());
    /// ```
    pub fn image(&self) -> Option<String>
    {
        self.image
            .clone()
    }


    /// Get the optional duration that exists under `ITunesItemExtension`.
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let duration = "duration".to_owned();
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .duration(Some(duration.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let duration_opt = item.duration();
    /// assert!(duration_opt.is_some());
    ///
    /// assert_eq!(duration.clone(), duration_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .duration(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let duration_opt = item.duration();
    /// assert!(duration_opt.is_none());
    /// ```
    pub fn duration(&self) -> Option<String>
    {
        self.duration
            .clone()
    }


    /// Get the optional explicit that exists under `ITunesItemExtension`.
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let explicit = "explicit".to_owned();
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .explicit(Some(explicit.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let explicit_opt = item.explicit();
    /// assert!(explicit_opt.is_some());
    ///
    /// assert_eq!(explicit.clone(), explicit_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .explicit(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let explicit_opt = item.explicit();
    /// assert!(explicit_opt.is_none());
    /// ```
    pub fn explicit(&self) -> Option<String>
    {
        self.explicit
            .clone()
    }


    /// Get the optional closed_captioned that exists under
    /// `ITunesItemExtension`.
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let closed_captioned = "closed_captioned".to_owned();
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .closed_captioned(Some(closed_captioned.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let closed_captioned_opt = item.closed_captioned();
    /// assert!(closed_captioned_opt.is_some());
    ///
    /// assert_eq!(closed_captioned.clone(), closed_captioned_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .closed_captioned(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let closed_captioned_opt = item.closed_captioned();
    /// assert!(closed_captioned_opt.is_none());
    /// ```
    pub fn closed_captioned(&self) -> Option<String>
    {
        self.closed_captioned
            .clone()
    }


    /// Get the optional order that exists under `ITunesItemExtension`.
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let order = "order".to_owned();
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .order(Some(order.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let order_opt = item.order();
    /// assert!(order_opt.is_some());
    ///
    /// assert_eq!(order.clone(), order_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .order(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let order_opt = item.order();
    /// assert!(order_opt.is_none());
    /// ```
    pub fn order(&self) -> Option<String>
    {
        self.order
            .clone()
    }


    /// Get the optional subtitle that exists under `ITunesItemExtension`.
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let subtitle = "subtitle".to_owned();
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .subtitle(Some(subtitle.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let subtitle_opt = item.subtitle();
    /// assert!(subtitle_opt.is_some());
    ///
    /// assert_eq!(subtitle.clone(), subtitle_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .subtitle(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let subtitle_opt = item.subtitle();
    /// assert!(subtitle_opt.is_none());
    /// ```
    pub fn subtitle(&self) -> Option<String>
    {
        self.subtitle
            .clone()
    }


    /// Get the optional summary that exists under `ITunesItemExtension`.
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let summary = "summary".to_owned();
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .summary(Some(summary.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let summary_opt = item.summary();
    /// assert!(summary_opt.is_some());
    ///
    /// assert_eq!(summary.clone(), summary_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .summary(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let summary_opt = item.summary();
    /// assert!(summary_opt.is_none());
    /// ```
    pub fn summary(&self) -> Option<String>
    {
        self.summary
            .clone()
    }


    /// Get the optional keywords that exists under `ITunesItemExtension`.
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let keywords = "keywords".to_owned();
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .keywords(Some(keywords.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let keywords_opt = item.keywords();
    /// assert!(keywords_opt.is_some());
    ///
    /// assert_eq!(keywords.clone(), keywords_opt.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .keywords(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let keywords_opt = item.keywords();
    /// assert!(keywords_opt.is_none());
    /// ```
    pub fn keywords(&self) -> Option<String>
    {
        self.keywords
            .clone()
    }
}

impl ITunesItemExtension
{
    /// Creates an ITunesChannelExtension using the specified hashmap.
    pub fn from_map(mut map: HashMap<String, Vec<Extension>>) -> Self
    {
        let mut ext = ITunesItemExtension::default();
        ext.author = remove_extension_value(&mut map,
                                            "author");
        ext.block = remove_extension_value(&mut map,
                                           "block");
        ext.image = parse_image(&mut map);
        ext.duration = remove_extension_value(&mut map,
                                              "duration");
        ext.explicit = remove_extension_value(&mut map,
                                              "explicit");
        ext.closed_captioned = remove_extension_value(&mut map,
                                                      "isClosedCaptioned");
        ext.order = remove_extension_value(&mut map,
                                           "order");
        ext.subtitle = remove_extension_value(&mut map,
                                              "subtitle");
        ext.summary = remove_extension_value(&mut map,
                                             "summary");
        ext.keywords = remove_extension_value(&mut map,
                                              "keywords");
        ext
    }
}

impl ToXml for ITunesItemExtension
{
    fn to_xml<W: ::std::io::Write>(&self,
                                   writer: &mut XmlWriter<W>)
        -> Result<(), XmlError>
    {
        if let Some(author) = self.author
                                  .as_ref() {
            writer.write_text_element(b"itunes:author",
                                      author)?;
        }

        if let Some(block) = self.block
                                 .as_ref() {
            writer.write_text_element(b"itunes:block",
                                      block)?;
        }

        if let Some(image) = self.image
                                 .as_ref() {
            let element = Element::new(b"itunes:image");
            writer.write(Event::Start({
                                          let mut element = element.clone();
                                          element.extend_attributes(::std::iter::once((b"href", image)));
                                          element
                                      }))?;
            writer.write(Event::End(element))?;
        }

        if let Some(duration) = self.duration
                                    .as_ref() {
            writer.write_text_element(b"itunes:duration",
                                      duration)?;
        }

        if let Some(explicit) = self.explicit
                                    .as_ref() {
            writer.write_text_element(b"itunes:explicit",
                                      explicit)?;
        }

        if let Some(closed_captioned) =
            self.closed_captioned
                .as_ref() {
            writer.write_text_element(b"itunes:isClosedCaptioned",
                                      closed_captioned)?;
        }

        if let Some(order) = self.order
                                 .as_ref() {
            writer.write_text_element(b"itunes:order",
                                      order)?;
        }

        if let Some(subtitle) = self.subtitle
                                    .as_ref() {
            writer.write_text_element(b"itunes:subtitle",
                                      subtitle)?;
        }

        if let Some(summary) = self.summary
                                   .as_ref() {
            writer.write_text_element(b"itunes:summary",
                                      summary)?;
        }

        if let Some(keywords) = self.keywords
                                    .as_ref() {
            writer.write_text_element(b"itunes:keywords",
                                      keywords)?;
        }

        Ok(())
    }
}

/// This `ITunesItemExtensionBuilder` struct creates the
/// `ITunesChannelExtension`.
#[derive(Debug, Clone, Default)]
pub struct ITunesItemExtensionBuilder
{
    author: Option<String>,
    block: Option<String>,
    image: Option<String>,
    duration: Option<String>,
    explicit: Option<String>,
    closed_captioned: Option<String>,
    order: Option<String>,
    subtitle: Option<String>,
    summary: Option<String>,
    keywords: Option<String>,
}

impl ITunesItemExtensionBuilder
{
    /// Construct a new `ITunesItemExtensionBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let item_builder = ITunesItemExtensionBuilder::new();
    /// ```
    pub fn new() -> ITunesItemExtensionBuilder
    {
        ITunesItemExtensionBuilder::default()
    }


    /// Set the optional author that exists under `ITunesItemExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let mut item_builder = ITunesItemExtensionBuilder::new();
    /// item_builder.author(Some("author".to_owned()));
    /// ```
    pub fn author(&mut self,
                  author: Option<String>)
        -> &mut ITunesItemExtensionBuilder
    {
        self.author = author;
        self
    }


    /// Set the optional block that exists under `ITunesItemExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let mut item_builder = ITunesItemExtensionBuilder::new();
    /// item_builder.block(Some("block".to_owned()));
    /// ```
    pub fn block(&mut self,
                 block: Option<String>)
        -> &mut ITunesItemExtensionBuilder
    {
        self.block = block;
        self
    }


    /// Set the optional image that exists under `ITunesItemExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let mut item_builder = ITunesItemExtensionBuilder::new();
    /// item_builder.image(Some("image".to_owned()));
    /// ```
    pub fn image(&mut self,
                 image: Option<String>)
        -> &mut ITunesItemExtensionBuilder
    {
        self.image = image;
        self
    }


    /// Set the optional duration that exists under `ITunesItemExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let mut item_builder = ITunesItemExtensionBuilder::new();
    /// item_builder.duration(Some("duration".to_owned()));
    /// ```
    pub fn duration(&mut self,
                    duration: Option<String>)
        -> &mut ITunesItemExtensionBuilder
    {
        self.duration = duration;
        self
    }


    /// Set the optional explicit that exists under `ITunesItemExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let mut item_builder = ITunesItemExtensionBuilder::new();
    /// item_builder.explicit(Some("explicit".to_owned()));
    /// ```
    pub fn explicit(&mut self,
                    explicit: Option<String>)
        -> &mut ITunesItemExtensionBuilder
    {
        self.explicit = explicit;
        self
    }


    /// Set the optional closed_captioned that exists under
    /// `ITunesItemExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let mut item_builder = ITunesItemExtensionBuilder::new();
    /// item_builder.closed_captioned(Some("closed_captioned".to_owned()));
    /// ```
    pub fn closed_captioned(&mut self,
                            closed_captioned: Option<String>)
        -> &mut ITunesItemExtensionBuilder
    {
        self.closed_captioned = closed_captioned;
        self
    }


    /// Set the optional order that exists under `ITunesItemExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let mut item_builder = ITunesItemExtensionBuilder::new();
    /// item_builder.order(Some("order".to_owned()));
    /// ```
    pub fn order(&mut self,
                 order: Option<String>)
        -> &mut ITunesItemExtensionBuilder
    {
        self.order = order;
        self
    }


    /// Set the optional subtitle that exists under `ITunesItemExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let mut item_builder = ITunesItemExtensionBuilder::new();
    /// item_builder.subtitle(Some("subtitle".to_owned()));
    /// ```
    pub fn subtitle(&mut self,
                    subtitle: Option<String>)
        -> &mut ITunesItemExtensionBuilder
    {
        self.subtitle = subtitle;
        self
    }


    /// Set the optional summary that exists under `ITunesItemExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let mut item_builder = ITunesItemExtensionBuilder::new();
    /// item_builder.summary(Some("summary".to_owned()));
    /// ```
    pub fn summary(&mut self,
                   summary: Option<String>)
        -> &mut ITunesItemExtensionBuilder
    {
        self.summary = summary;
        self
    }


    /// Set the optional keywords that exists under `ITunesItemExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let mut item_builder = ITunesItemExtensionBuilder::new();
    /// item_builder.keywords(Some("keywords".to_owned()));
    /// ```
    pub fn keywords(&mut self,
                    keywords: Option<String>)
        -> &mut ITunesItemExtensionBuilder
    {
        self.keywords = keywords;
        self
    }


    /// Construct the `ITunesItemExtension` from the
    /// `ITunesItemExtensionBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .author(Some("author".to_owned()))
    ///     .block(Some("block".to_owned()))
    ///     .image(Some("image".to_owned()))
    ///     .duration(Some("duration".to_owned()))
    ///     .explicit(Some("explicit".to_owned()))
    ///     .closed_captioned(Some("closed_captioned".to_owned()))
    ///     .order(Some("order".to_owned()))
    ///     .subtitle(Some("subtitle".to_owned()))
    ///     .summary(Some("summary".to_owned()))
    ///     .keywords(Some("keywords".to_owned()))
    ///     .finalize()
    ///     .unwrap();
    /// ```
    pub fn finalize(&self) -> Result<ITunesItemExtension, Error>
    {
        Ok(ITunesItemExtension { author: self.author
                                             .clone(),
                                 block: self.block
                                            .clone(),
                                 image: self.image
                                            .clone(),
                                 duration: self.duration
                                               .clone(),
                                 explicit: self.explicit
                                               .clone(),
                                 closed_captioned: self.closed_captioned
                                                       .clone(),
                                 order: self.order
                                            .clone(),
                                 subtitle: self.subtitle
                                               .clone(),
                                 summary: self.summary
                                              .clone(),
                                 keywords: self.keywords
                                               .clone(), })
    }
}
