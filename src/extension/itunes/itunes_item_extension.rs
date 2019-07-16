// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::collections::HashMap;
use std::io::Write;

use quick_xml::Error as XmlError;
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::Writer;

use super::parse_image;
use crate::extension::Extension;
use crate::extension::util::remove_extension_value;
use crate::toxml::{ToXml, WriterExt};

/// An iTunes item element extension.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, Builder)]
#[builder(setter(into), default)]
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
    /// use rss::extension::itunes::ITunesItemExtension;
    ///
    /// let mut extension = ITunesItemExtension::default();
    /// extension.set_author("John Doe".to_string());
    /// assert_eq!(extension.author(), Some("John Doe"));
    /// ```
    pub fn author(&self) -> Option<&str> {
        self.author.as_ref().map(String::as_str)
    }

    /// Set the author of this podcast episode.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtension;
    ///
    /// let mut extension = ITunesItemExtension::default();
    /// extension.set_author("John Doe".to_string());
    /// ```
    pub fn set_author<V>(&mut self, author: V)
    where
        V: Into<Option<String>>,
    {
        self.author = author.into();
    }

    /// Return whether this podcast episode should be blocked from appearing in the iTunes Store.
    ///
    /// A value of `Yes` indicates that the podcast should not show up in the iTunes Store. All
    /// other values are ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtension;
    ///
    /// let mut extension = ITunesItemExtension::default();
    /// extension.set_block("Yes".to_string());
    /// assert_eq!(extension.block(), Some("Yes"));
    /// ```
    pub fn block(&self) -> Option<&str> {
        self.block.as_ref().map(String::as_str)
    }

    /// Set whether this podcast episode should be blocked from appearing in the iTunes Store.
    ///
    /// A value of `Yes` indicates that the podcast should not show up in the iTunes Store. All
    /// other values are ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtension;
    ///
    /// let mut extension = ITunesItemExtension::default();
    /// extension.set_block("Yes".to_string());
    /// ```
    pub fn set_block<V>(&mut self, block: V)
    where
        V: Into<Option<String>>,
    {
        self.block = block.into();
    }

    /// Return the artwork URL for this podcast episode.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtension;
    ///
    /// let mut extension = ITunesItemExtension::default();
    /// extension.set_image("http://example.com/artwork.png".to_string());
    /// assert_eq!(extension.image(), Some("http://example.com/artwork.png"));
    /// ```
    pub fn image(&self) -> Option<&str> {
        self.image.as_ref().map(String::as_str)
    }

    /// Set the artwork URL for this podcast episode.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtension;
    ///
    /// let mut extension = ITunesItemExtension::default();
    /// extension.set_image("http://example.com/artwork.png".to_string());
    /// ```
    pub fn set_image<V>(&mut self, image: V)
    where
        V: Into<Option<String>>,
    {
        self.image = image.into();
    }

    /// Return the duration of this podcast episode.
    ///
    /// The duration should be in one of the following formats: HH:MM:SS, H:MM:SS, MM:SS, M:SS.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtension;
    ///
    /// let mut extension = ITunesItemExtension::default();
    /// extension.set_duration("1:00".to_string());
    /// assert_eq!(extension.duration(), Some("1:00"));
    /// ```
    pub fn duration(&self) -> Option<&str> {
        self.duration.as_ref().map(String::as_str)
    }

    /// Set the duration of this podcast episode.
    ///
    /// The duration should be in one of the following formats: HH:MM:SS, H:MM:SS, MM:SS, M:SS.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtension;
    ///
    /// let mut extension = ITunesItemExtension::default();
    /// extension.set_duration("1:00".to_string());
    /// ```
    pub fn set_duration<V>(&mut self, duration: V)
    where
        V: Into<Option<String>>,
    {
        self.duration = duration.into();
    }

    /// Return whether this podcast episode contains explicit content.
    ///
    /// A value of `Yes`, `Explicit`, or `True` indicates that the episode contains explicit
    /// content. A value of `Clean`, `No`, `False` inidicates that episode does not contain
    /// explicit content.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtension;
    ///
    /// let mut extension = ITunesItemExtension::default();
    /// extension.set_explicit("Yes".to_string());
    /// assert_eq!(extension.explicit(), Some("Yes"));
    /// ```
    pub fn explicit(&self) -> Option<&str> {
        self.explicit.as_ref().map(String::as_str)
    }

    /// Set whether this podcast episode contains explicit content.
    ///
    /// A value of `Yes`, `Explicit`, or `True` indicates that the episode contains explicit
    /// content. A value of `Clean`, `No`, `False` inidicates that episode does not contain
    /// explicit content.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtension;
    ///
    /// let mut extension = ITunesItemExtension::default();
    /// extension.set_explicit("Yes".to_string());
    /// ```
    pub fn set_explicit<V>(&mut self, explicit: V)
    where
        V: Into<Option<String>>,
    {
        self.explicit = explicit.into();
    }

    /// Return whether this podcast episode contains embedded closed captioning.
    ///
    /// A value of `Yes` indicates that it does. Any other value indicates that it does not.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtension;
    ///
    /// let mut extension = ITunesItemExtension::default();
    /// extension.set_closed_captioned("Yes".to_string());
    /// assert_eq!(extension.closed_captioned(), Some("Yes"));
    /// ```
    pub fn closed_captioned(&self) -> Option<&str> {
        self.closed_captioned.as_ref().map(String::as_str)
    }

    /// Set whether this podcast episode contains embedded closed captioning.
    ///
    /// A value of `Yes` indicates that it does. Any other value indicates that it does not.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtension;
    ///
    /// let mut extension = ITunesItemExtension::default();
    /// extension.set_closed_captioned("Yes".to_string());
    /// ```
    pub fn set_closed_captioned<V>(&mut self, closed_captioned: V)
    where
        V: Into<Option<String>>,
    {
        self.closed_captioned = closed_captioned.into();
    }

    /// Return the value used to override the default sorting order for episodes.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtension;
    ///
    /// let mut extension = ITunesItemExtension::default();
    /// extension.set_order("1".to_string());
    /// assert_eq!(extension.order(), Some("1"));
    /// ```
    pub fn order(&self) -> Option<&str> {
        self.order.as_ref().map(String::as_str)
    }

    /// Set the value used to override the default sorting order for episodes.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtension;
    ///
    /// let mut extension = ITunesItemExtension::default();
    /// extension.set_order("1".to_string());
    /// ```
    pub fn set_order<V>(&mut self, order: V)
    where
        V: Into<Option<String>>,
    {
        self.order = order.into();
    }

    /// Return the description of this podcast episode.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtension;
    ///
    /// let mut extension = ITunesItemExtension::default();
    /// extension.set_subtitle("An episode".to_string());
    /// assert_eq!(extension.subtitle(), Some("An episode"));
    /// ```
    pub fn subtitle(&self) -> Option<&str> {
        self.subtitle.as_ref().map(String::as_str)
    }

    /// Set the description of this podcast episode.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtension;
    ///
    /// let mut extension = ITunesItemExtension::default();
    /// extension.set_subtitle("An episode".to_string());
    /// ```
    pub fn set_subtitle<V>(&mut self, subtitle: V)
    where
        V: Into<Option<String>>,
    {
        self.subtitle = subtitle.into();
    }

    /// Return the summary for this podcast episode.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtension;
    ///
    /// let mut extension = ITunesItemExtension::default();
    /// extension.set_summary("An episode".to_string());
    /// assert_eq!(extension.summary(), Some("An episode"));
    /// ```
    pub fn summary(&self) -> Option<&str> {
        self.summary.as_ref().map(String::as_str)
    }

    /// Set the summary for this podcast episode.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtension;
    ///
    /// let mut extension = ITunesItemExtension::default();
    /// extension.set_summary("An episode".to_string());
    /// ```
    pub fn set_summary<V>(&mut self, summary: V)
    where
        V: Into<Option<String>>,
    {
        self.summary = summary.into();
    }

    /// Return the keywords for this podcast episode.
    ///
    /// A comma separated list of keywords.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtension;
    ///
    /// let mut extension = ITunesItemExtension::default();
    /// extension.set_keywords("technology".to_string());
    /// assert_eq!(extension.keywords(), Some("technology"));
    /// ```
    pub fn keywords(&self) -> Option<&str> {
        self.keywords.as_ref().map(String::as_str)
    }

    /// Set the keywords for this podcast episode.
    ///
    /// A comma separated list of keywords.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtension;
    ///
    /// let mut extension = ITunesItemExtension::default();
    /// extension.set_keywords("technology".to_string());
    /// ```
    pub fn set_keywords<V>(&mut self, keywords: V)
    where
        V: Into<Option<String>>,
    {
        self.keywords = keywords.into();
    }
}

impl ITunesItemExtension {
    /// Create an `ITunesChannelExtension` from a `HashMap`.
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
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
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
            writer.write_text_element(b"itunes:isClosedCaptioned", closed_captioned)?;
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
