// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use super::parse_image;
use extension::Extension;
use extension::remove_extension_value;
use quick_xml::errors::Error as XmlError;
use quick_xml::events::{Event, BytesStart, BytesEnd};
use quick_xml::writer::Writer;
use std::collections::HashMap;
use toxml::{ToXml, WriterExt};

/// An iTunes item element extension.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ITunesItemExtension {
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

impl ITunesItemExtension {
    /// Get the optional author that exists under `ITunesItemExtension`.
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let author = "author";
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .author(Some(author.to_string()))
    ///     .finalize();
    ///
    /// assert_eq!(Some(author), item.author());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .author(None)
    ///     .finalize();
    ///
    /// let author_opt = item.author();
    /// assert!(author_opt.is_none());
    /// ```
    pub fn author(&self) -> Option<&str> {
        self.author.as_ref().map(|s| s.as_str())
    }


    /// Get the optional block that exists under `ITunesItemExtension`.
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let block = "block";
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .block(Some(block.to_string()))
    ///     .finalize();
    ///
    ///
    /// assert_eq!(Some(block), item.block());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .block(None)
    ///     .finalize();
    ///
    /// let block_opt = item.block();
    /// assert!(block_opt.is_none());
    /// ```
    pub fn block(&self) -> Option<&str> {
        self.block.as_ref().map(|s| s.as_str())
    }


    /// Get the optional image that exists under `ITunesItemExtension`.
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let image = "image";
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .image(Some(image.to_string()))
    ///     .finalize();
    ///
    /// assert_eq!(Some(image), item.image());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .image(None)
    ///     .finalize();
    ///
    /// let image_opt = item.image();
    /// assert!(image_opt.is_none());
    /// ```
    pub fn image(&self) -> Option<&str> {
        self.image.as_ref().map(|s| s.as_str())
    }


    /// Get the optional duration that exists under `ITunesItemExtension`.
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let duration = "duration";
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .duration(Some(duration.to_string()))
    ///     .finalize();
    ///
    /// assert_eq!(Some(duration), item.duration());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .duration(None)
    ///     .finalize();
    ///
    /// let duration_opt = item.duration();
    /// assert!(duration_opt.is_none());
    /// ```
    pub fn duration(&self) -> Option<&str> {
        self.duration.as_ref().map(|s| s.as_str())
    }


    /// Get the optional explicit that exists under `ITunesItemExtension`.
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let explicit = "explicit";
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .explicit(Some(explicit.to_string()))
    ///     .finalize();
    ///
    /// assert_eq!(Some(explicit), item.explicit());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .explicit(None)
    ///     .finalize();
    ///
    /// let explicit_opt = item.explicit();
    /// assert!(explicit_opt.is_none());
    /// ```
    pub fn explicit(&self) -> Option<&str> {
        self.explicit.as_ref().map(|s| s.as_str())
    }


    /// Get the optional closed_captioned that exists under
    /// `ITunesItemExtension`.
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let closed_captioned = "closed_captioned";
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .closed_captioned(Some(closed_captioned.to_string()))
    ///     .finalize();
    ///
    /// assert_eq!(Some(closed_captioned), item.closed_captioned());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .closed_captioned(None)
    ///     .finalize();
    ///
    /// let closed_captioned_opt = item.closed_captioned();
    /// assert!(closed_captioned_opt.is_none());
    /// ```
    pub fn closed_captioned(&self) -> Option<&str> {
        self.closed_captioned.as_ref().map(|s| s.as_str())
    }


    /// Get the optional order that exists under `ITunesItemExtension`.
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let order = "order";
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .order(Some(order.to_string()))
    ///     .finalize();
    ///
    /// assert_eq!(Some(order), item.order());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .order(None)
    ///     .finalize();
    ///
    /// let order_opt = item.order();
    /// assert!(order_opt.is_none());
    /// ```
    pub fn order(&self) -> Option<&str> {
        self.order.as_ref().map(|s| s.as_str())
    }


    /// Get the optional subtitle that exists under `ITunesItemExtension`.
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let subtitle = "subtitle";
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .subtitle(Some(subtitle.to_string()))
    ///     .finalize();
    ///
    /// assert_eq!(Some(subtitle), item.subtitle());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .subtitle(None)
    ///     .finalize();
    ///
    /// let subtitle_opt = item.subtitle();
    /// assert!(subtitle_opt.is_none());
    /// ```
    pub fn subtitle(&self) -> Option<&str> {
        self.subtitle.as_ref().map(|s| s.as_str())
    }


    /// Get the optional summary that exists under `ITunesItemExtension`.
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let summary = "summary";
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .summary(Some(summary.to_string()))
    ///     .finalize();
    ///
    /// assert_eq!(Some(summary), item.summary());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .summary(None)
    ///     .finalize();
    ///
    /// let summary_opt = item.summary();
    /// assert!(summary_opt.is_none());
    /// ```
    pub fn summary(&self) -> Option<&str> {
        self.summary.as_ref().map(|s| s.as_str())
    }


    /// Get the optional keywords that exists under `ITunesItemExtension`.
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let keywords = "keywords";
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .keywords(Some(keywords.to_string()))
    ///     .finalize();
    ///
    /// assert_eq!(Some(keywords), item.keywords());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesItemExtensionBuilder,
    /// ITunesItemExtension};
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .keywords(None)
    ///     .finalize();
    ///
    /// let keywords_opt = item.keywords();
    /// assert!(keywords_opt.is_none());
    /// ```
    pub fn keywords(&self) -> Option<&str> {
        self.keywords.as_ref().map(|s| s.as_str())
    }
}

impl ITunesItemExtension {
    /// Creates an ITunesChannelExtension using the specified hashmap.
    pub fn from_map(mut map: HashMap<String, Vec<Extension>>) -> Self {
        let mut ext = ITunesItemExtension::default();
        ext.author = remove_extension_value(&mut map, "author");
        ext.block = remove_extension_value(&mut map, "block");
        ext.image = parse_image(&mut map);
        ext.duration = remove_extension_value(&mut map, "duration");
        ext.explicit = remove_extension_value(&mut map, "explicit");
        ext.closed_captioned = remove_extension_value(&mut map, "isClosedCaptioned");
        ext.order = remove_extension_value(&mut map, "order");
        ext.subtitle = remove_extension_value(&mut map, "subtitle");
        ext.summary = remove_extension_value(&mut map, "summary");
        ext.keywords = remove_extension_value(&mut map, "keywords");
        ext
    }
}

impl ToXml for ITunesItemExtension {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        if let Some(author) = self.author.as_ref() {
            writer.write_text_element(b"itunes:author", author)?;
        }

        if let Some(block) = self.block.as_ref() {
            writer.write_text_element(b"itunes:block", block)?;
        }

        if let Some(image) = self.image.as_ref() {
            let name = b"itunes:image";
            let mut element = BytesStart::borrowed(name, name.len());
            element.push_attribute(("href", &**image));
            writer.write_event(Event::Start(element))?;
            writer.write_event(Event::End(BytesEnd::borrowed(name)))?;
        }

        if let Some(duration) = self.duration.as_ref() {
            writer.write_text_element(b"itunes:duration", duration)?;
        }

        if let Some(explicit) = self.explicit.as_ref() {
            writer.write_text_element(b"itunes:explicit", explicit)?;
        }

        if let Some(closed_captioned) = self.closed_captioned.as_ref() {
            writer
                .write_text_element(b"itunes:isClosedCaptioned", closed_captioned)?;
        }

        if let Some(order) = self.order.as_ref() {
            writer.write_text_element(b"itunes:order", order)?;
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

/// This `ITunesItemExtensionBuilder` struct creates the
/// `ITunesChannelExtension`.
#[derive(Debug, Clone, Default)]
pub struct ITunesItemExtensionBuilder {
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

impl ITunesItemExtensionBuilder {
    /// Construct a new `ITunesItemExtensionBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let item_builder = ITunesItemExtensionBuilder::new();
    /// ```
    pub fn new() -> ITunesItemExtensionBuilder {
        ITunesItemExtensionBuilder::default()
    }


    /// Set the optional author that exists under `ITunesItemExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let item_builder = ITunesItemExtensionBuilder::new()
    ///     .author(Some("author".to_string()));
    /// ```
    pub fn author(mut self, author: Option<String>) -> ITunesItemExtensionBuilder {
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
    /// let item_builder = ITunesItemExtensionBuilder::new()
    ///     .block(Some("block".to_string()));
    /// ```
    pub fn block(mut self, block: Option<String>) -> ITunesItemExtensionBuilder {
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
    /// let item_builder = ITunesItemExtensionBuilder::new()
    ///     .image(Some("image".to_string()));
    /// ```
    pub fn image(mut self, image: Option<String>) -> ITunesItemExtensionBuilder {
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
    /// let item_builder = ITunesItemExtensionBuilder::new()
    ///     .duration(Some("duration".to_string()));
    /// ```
    pub fn duration(mut self, duration: Option<String>) -> ITunesItemExtensionBuilder {
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
    /// let item_builder = ITunesItemExtensionBuilder::new()
    ///     .explicit(Some("explicit".to_string()));
    /// ```
    pub fn explicit(mut self, explicit: Option<String>) -> ITunesItemExtensionBuilder {
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
    /// let item_builder = ITunesItemExtensionBuilder::new()
    ///     .closed_captioned(Some("closed_captioned".to_string()));
    /// ```
    pub fn closed_captioned(mut self,
                            closed_captioned: Option<String>)
                            -> ITunesItemExtensionBuilder {
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
    /// let item_builder = ITunesItemExtensionBuilder::new()
    ///     .order(Some("order".to_string()));
    /// ```
    pub fn order(mut self, order: Option<String>) -> ITunesItemExtensionBuilder {
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
    /// let item_builder = ITunesItemExtensionBuilder::new()
    ///     .subtitle(Some("subtitle".to_string()));
    /// ```
    pub fn subtitle(mut self, subtitle: Option<String>) -> ITunesItemExtensionBuilder {
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
    /// let item_builder = ITunesItemExtensionBuilder::new()
    ///     .summary(Some("summary".to_string()));
    /// ```
    pub fn summary(mut self, summary: Option<String>) -> ITunesItemExtensionBuilder {
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
    /// let item_builder = ITunesItemExtensionBuilder::new()
    ///     .keywords(Some("keywords".to_string()));
    /// ```
    pub fn keywords(mut self, keywords: Option<String>) -> ITunesItemExtensionBuilder {
        self.keywords = keywords;
        self
    }


    /// Construct the `ITunesItemExtension` from the `ITunesItemExtensionBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let item = ITunesItemExtensionBuilder::new()
    ///     .author(Some("author".to_string()))
    ///     .block(Some("block".to_string()))
    ///     .image(Some("image".to_string()))
    ///     .duration(Some("duration".to_string()))
    ///     .explicit(Some("explicit".to_string()))
    ///     .closed_captioned(Some("closed_captioned".to_string()))
    ///     .order(Some("order".to_string()))
    ///     .subtitle(Some("subtitle".to_string()))
    ///     .summary(Some("summary".to_string()))
    ///     .keywords(Some("keywords".to_string()))
    ///     .finalize();
    /// ```
    pub fn finalize(self) -> ITunesItemExtension {
        ITunesItemExtension {
            author: self.author,
            block: self.block,
            image: self.image,
            duration: self.duration,
            explicit: self.explicit,
            closed_captioned: self.closed_captioned,
            order: self.order,
            subtitle: self.subtitle,
            summary: self.summary,
            keywords: self.keywords,
        }
    }
}
