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
    /// Return the author of this podcast episode.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let author = "author";
    ///
    /// let ext = ITunesItemExtensionBuilder::default()
    ///     .author(author.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(author), ext.author());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let ext = ITunesItemExtensionBuilder::default()
    ///     .author(None)
    ///     .finalize();
    ///
    /// assert!(ext.author().is_none());
    /// ```
    pub fn author(&self) -> Option<&str> {
        self.author.as_ref().map(|s| s.as_str())
    }

    /// Return whether this podcast episode should be blocked from appearing in the iTunes Store.
    ///
    /// A value of `Yes` indicates that the podcast should not show up in the iTunes Store. All
    /// other values are ignored.
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let block = "Yes";
    ///
    /// let ext = ITunesItemExtensionBuilder::default()
    ///     .block(block.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(block), ext.block());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let ext = ITunesItemExtensionBuilder::default()
    ///     .block(None)
    ///     .finalize();
    ///
    /// assert!(ext.block().is_none());
    /// ```
    pub fn block(&self) -> Option<&str> {
        self.block.as_ref().map(|s| s.as_str())
    }

    /// Return the artwork URL for this podcast episode.
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let image = "http://example.com/image.png";
    ///
    /// let ext = ITunesItemExtensionBuilder::default()
    ///     .image(image.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(image), ext.image());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let ext = ITunesItemExtensionBuilder::default()
    ///     .image(None)
    ///     .finalize();
    ///
    /// assert!(ext.image().is_none());
    /// ```
    pub fn image(&self) -> Option<&str> {
        self.image.as_ref().map(|s| s.as_str())
    }

    /// Return the duration of this podcast episode.
    ///
    /// The duration should be in one of the following formats: HH:MM:SS, H:MM:SS, MM:SS, M:SS.
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let duration = "50:03";
    ///
    /// let ext = ITunesItemExtensionBuilder::default()
    ///     .duration(duration.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(duration), ext.duration());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let ext = ITunesItemExtensionBuilder::default()
    ///     .duration(None)
    ///     .finalize();
    ///
    /// assert!(ext.duration().is_none());
    /// ```
    pub fn duration(&self) -> Option<&str> {
        self.duration.as_ref().map(|s| s.as_str())
    }

    /// Return whether this podcast episode contains explicit content.
    ///
    /// A value of `Yes`, `Explicit`, or `True` indicates that the episode contains explicit
    /// content. A value of `Clean`, `No`, `False` inidicates that episode does not contain
    /// explicit content.
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let explicit = "Yes";
    ///
    /// let ext = ITunesItemExtensionBuilder::default()
    ///     .explicit(explicit.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(explicit), ext.explicit());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let ext = ITunesItemExtensionBuilder::default()
    ///     .explicit(None)
    ///     .finalize();
    ///
    /// assert!(ext.explicit().is_none());
    /// ```
    pub fn explicit(&self) -> Option<&str> {
        self.explicit.as_ref().map(|s| s.as_str())
    }

    /// Return whether this podcast episode contains embedded closed captioning.
    ///
    /// A value of `Yes` indicates that it does. Any other value indicates that it does not.
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let closed_captioned = "Yes";
    ///
    /// let ext = ITunesItemExtensionBuilder::default()
    ///     .closed_captioned(closed_captioned.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(closed_captioned), ext.closed_captioned());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let ext = ITunesItemExtensionBuilder::default()
    ///     .closed_captioned(None)
    ///     .finalize();
    ///
    /// assert!(ext.closed_captioned().is_none());
    /// ```
    pub fn closed_captioned(&self) -> Option<&str> {
        self.closed_captioned.as_ref().map(|s| s.as_str())
    }

    /// Return the value used to override the default sorting order for episodes.
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let order = "0";
    ///
    /// let ext = ITunesItemExtensionBuilder::default()
    ///     .order(order.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(order), ext.order());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let ext = ITunesItemExtensionBuilder::default()
    ///     .order(None)
    ///     .finalize();
    ///
    /// assert!(ext.order().is_none());
    /// ```
    pub fn order(&self) -> Option<&str> {
        self.order.as_ref().map(|s| s.as_str())
    }

    /// Return the description of this podcast episode.
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let subtitle = "description";
    ///
    /// let ext = ITunesItemExtensionBuilder::default()
    ///     .subtitle(subtitle.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(subtitle), ext.subtitle());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let ext = ITunesItemExtensionBuilder::default()
    ///     .subtitle(None)
    ///     .finalize();
    ///
    /// assert!(ext.subtitle().is_none());
    /// ```
    pub fn subtitle(&self) -> Option<&str> {
        self.subtitle.as_ref().map(|s| s.as_str())
    }

    /// Return the summary for this podcast episode.
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let summary = "summary";
    ///
    /// let ext = ITunesItemExtensionBuilder::default()
    ///     .summary(summary.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(summary), ext.summary());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let ext = ITunesItemExtensionBuilder::default()
    ///     .summary(None)
    ///     .finalize();
    ///
    /// assert!(ext.summary().is_none());
    /// ```
    pub fn summary(&self) -> Option<&str> {
        self.summary.as_ref().map(|s| s.as_str())
    }

    /// Return the keywords for this podcast episode.
    ///
    /// The string contains a comma separated list of keywords.
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let keywords = "keyword1,keyword2";
    ///
    /// let ext = ITunesItemExtensionBuilder::default()
    ///     .keywords(keywords.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(keywords), ext.keywords());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let ext = ITunesItemExtensionBuilder::default()
    ///     .keywords(None)
    ///     .finalize();
    ///
    /// assert!(ext.keywords().is_none());
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

/// A builder used to create an `ITunesItemExtension`.
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
    /// Set the author of the podcast episode.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let builder = ITunesItemExtensionBuilder::default()
    ///     .author("author".to_string());
    /// ```
    pub fn author<V>(mut self, author: V) -> ITunesItemExtensionBuilder
        where V: Into<Option<String>>
    {
        self.author = author.into();
        self
    }

    /// Set whether the podcast episode should be blocked from appearing in the iTunes Store.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let builder = ITunesItemExtensionBuilder::default()
    ///     .block("Yes".to_string());
    /// ```
    pub fn block<V>(mut self, block: V) -> ITunesItemExtensionBuilder
        where V: Into<Option<String>>
    {
        self.block = block.into();
        self
    }

    /// Set the artwork URL for the podcast episode.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let builder = ITunesItemExtensionBuilder::default()
    ///     .image("http://example.com/image.png".to_string());
    /// ```
    pub fn image<V>(mut self, image: V) -> ITunesItemExtensionBuilder
        where V: Into<Option<String>>
    {
        self.image = image.into();
        self
    }

    /// Set the duration of the podcast episode.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let builder = ITunesItemExtensionBuilder::default()
    ///     .duration("50:03".to_string());
    /// ```
    pub fn duration<V>(mut self, duration: V) -> ITunesItemExtensionBuilder
        where V: Into<Option<String>>
    {
        self.duration = duration.into();
        self
    }

    /// Set whether the podcast episode contains explicit content.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let builder = ITunesItemExtensionBuilder::default()
    ///     .explicit("Yes".to_string());
    /// ```
    pub fn explicit<V>(mut self, explicit: V) -> ITunesItemExtensionBuilder
        where V: Into<Option<String>>
    {
        self.explicit = explicit.into();
        self
    }

    /// Set whether the podcast episode contains embedded closed captioning.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let builder = ITunesItemExtensionBuilder::default()
    ///     .closed_captioned("Yes".to_string());
    /// ```
    pub fn closed_captioned<V>(mut self, closed_captioned: V) -> ITunesItemExtensionBuilder
        where V: Into<Option<String>>
    {
        self.closed_captioned = closed_captioned.into();
        self
    }

    /// Set the value used to override the default sorting order for episodes.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let builder = ITunesItemExtensionBuilder::default()
    ///     .order("0".to_string());
    /// ```
    pub fn order<V>(mut self, order: V) -> ITunesItemExtensionBuilder
        where V: Into<Option<String>>
    {
        self.order = order.into();
        self
    }

    /// Set the description of the podcast episode.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let builder = ITunesItemExtensionBuilder::default()
    ///     .subtitle("description".to_string());
    /// ```
    pub fn subtitle<V>(mut self, subtitle: V) -> ITunesItemExtensionBuilder
        where V: Into<Option<String>>
    {
        self.subtitle = subtitle.into();
        self
    }

    /// Set the summary for the podcast episode.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let builder = ITunesItemExtensionBuilder::default()
    ///     .summary("summary".to_string());
    /// ```
    pub fn summary<V>(mut self, summary: V) -> ITunesItemExtensionBuilder
        where V: Into<Option<String>>
    {
        self.summary = summary.into();
        self
    }

    /// Set the keywords for the podcast episode.
    ///
    /// The string should be a comma separated list of keywords.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let builder = ITunesItemExtensionBuilder::default()
    ///     .keywords("keyword1,keyword2".to_string());
    /// ```
    pub fn keywords<V>(mut self, keywords: V) -> ITunesItemExtensionBuilder
        where V: Into<Option<String>>
    {
        self.keywords = keywords.into();
        self
    }

    /// Construct the `ITunesItemExtension` from this `ITunesItemExtensionBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let ext = ITunesItemExtensionBuilder::default()
    ///     .author("author".to_string())
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
