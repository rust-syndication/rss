// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::collections::BTreeMap;
use std::io::Write;

use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::Error as XmlError;
use quick_xml::Writer;

use super::{parse_image, NAMESPACE};
use crate::extension::util::remove_extension_value;
use crate::extension::Extension;
use crate::toxml::{ToXml, WriterExt};

/// An iTunes item element extension.
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
pub struct ITunesItemExtension {
    /// The author of the podcast episode.
    pub author: Option<String>,
    /// Specifies if the podcast episode should be prevented from appearing in the iTunes Store. A
    /// value of `Yes` indicates that the episode should not show up in the iTunes Store. All other
    /// values are ignored.
    pub block: Option<String>,
    /// The artwork for the podcast episode.
    pub image: Option<String>,
    /// The podcast episode duration in one of the following formats: HH:MM:SS, H:MM:SS, MM:SS,
    /// M:SS.
    pub duration: Option<String>,
    /// Specifies whether the podcast episode contains explicit content. A value of `Yes`,
    /// `Explicit`, or `True` indicates that the episode contains explicit content. A value of
    /// `Clean`, `No`, `False` indicates that episode does not contain explicit content.
    pub explicit: Option<String>,
    /// Specifies whether the podcast episode contains embedded closed captioning. A value of `Yes`
    /// indicates that it does. Any other value indicates that it does not.
    pub closed_captioned: Option<String>,
    /// A value used to override the default sorting order for episodes.
    pub order: Option<String>,
    /// A description of the podcast episode.
    pub subtitle: Option<String>,
    /// A summary of the podcast episode.
    pub summary: Option<String>,
    /// Keywords for the podcast. The string contains a comma separated list of keywords.
    pub keywords: Option<String>,
    /// Episode number for this episode.
    pub episode: Option<String>,
    /// Season number for this episode.
    pub season: Option<String>,
    /// Type of episode. Usually `full`, but potentially also `trailer` or `bonus`
    pub episode_type: Option<String>,
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
        self.author.as_deref()
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
        self.block.as_deref()
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
        self.image.as_deref()
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
        self.duration.as_deref()
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
    /// content. A value of `Clean`, `No`, `False` indicates that episode does not contain
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
        self.explicit.as_deref()
    }

    /// Set whether this podcast episode contains explicit content.
    ///
    /// A value of `Yes`, `Explicit`, or `True` indicates that the episode contains explicit
    /// content. A value of `Clean`, `No`, `False` indicates that episode does not contain
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
        self.closed_captioned.as_deref()
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
        self.order.as_deref()
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
        self.subtitle.as_deref()
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
        self.summary.as_deref()
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
        self.keywords.as_deref()
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

    /// Return the episode number of this podcast episode
    ///
    /// The episode number will be a string although it is typically a number in practice
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtension;
    ///
    /// let mut extension = ITunesItemExtension::default();
    /// extension.set_episode("3".to_string());
    /// assert_eq!(extension.episode(), Some("3"));
    /// ```
    pub fn episode(&self) -> Option<&str> {
        self.episode.as_deref()
    }

    /// Set the the episode number for this episode.
    ///
    /// An string.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtension;
    ///
    /// let mut extension = ITunesItemExtension::default();
    /// extension.set_episode("2".to_string());
    /// assert_eq!(extension.episode(), Some("2"));
    /// ```
    pub fn set_episode<V>(&mut self, episode: V)
    where
        V: Into<Option<String>>,
    {
        self.episode = episode.into()
    }

    /// Return the season of this podcast episode
    ///
    /// The season will be a string although it is typically a number in practice
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtension;
    ///
    /// let mut extension = ITunesItemExtension::default();
    /// extension.set_season("3".to_string());
    /// assert_eq!(extension.season(), Some("3"));
    /// ```
    pub fn season(&self) -> Option<&str> {
        self.season.as_deref()
    }

    /// Set the the season number for this episode.
    ///
    /// An integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtension;
    ///
    /// let mut extension = ITunesItemExtension::default();
    /// extension.set_season("3".to_string());
    /// assert_eq!(extension.season(), Some("3"));
    /// ```
    pub fn set_season<V>(&mut self, season: V)
    where
        V: Into<Option<String>>,
    {
        self.season = season.into()
    }

    /// Return the episode_type of this podcast episode
    ///
    /// The episode type will be a string usually "full" "trailer" or "bonus"
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtension;
    ///
    /// let mut extension = ITunesItemExtension::default();
    /// extension.set_episode_type("trailer".to_string());
    /// assert_eq!(extension.episode_type(), Some("trailer"));
    /// ```
    pub fn episode_type(&self) -> Option<&str> {
        self.episode_type.as_deref()
    }

    /// Set the the episode type for this episode.
    ///
    /// A string, usually "full" but maybe "trailer" or "bonus"
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesItemExtension;
    ///
    /// let mut extension = ITunesItemExtension::default();
    /// extension.set_episode_type("full".to_string());
    /// assert_eq!(extension.episode_type(), Some("full"));
    /// ```
    pub fn set_episode_type<V>(&mut self, episode_type: V)
    where
        V: Into<Option<String>>,
    {
        self.episode_type = episode_type.into()
    }
}

impl ITunesItemExtension {
    /// Create an `ITunesChannelExtension` from a `BTreeMap`.
    pub fn from_map(mut map: BTreeMap<String, Vec<Extension>>) -> Self {
        Self {
            author: remove_extension_value(&mut map, "author"),
            block: remove_extension_value(&mut map, "block"),
            image: parse_image(&mut map),
            duration: remove_extension_value(&mut map, "duration"),
            explicit: remove_extension_value(&mut map, "explicit"),
            closed_captioned: remove_extension_value(&mut map, "isClosedCaptioned"),
            order: remove_extension_value(&mut map, "order"),
            subtitle: remove_extension_value(&mut map, "subtitle"),
            summary: remove_extension_value(&mut map, "summary"),
            keywords: remove_extension_value(&mut map, "keywords"),
            episode: remove_extension_value(&mut map, "episode"),
            season: remove_extension_value(&mut map, "season"),
            episode_type: remove_extension_value(&mut map, "episodeType"),
        }
    }
}

impl ToXml for ITunesItemExtension {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        if let Some(author) = self.author.as_ref() {
            writer.write_text_element("itunes:author", author)?;
        }

        if let Some(block) = self.block.as_ref() {
            writer.write_text_element("itunes:block", block)?;
        }

        if let Some(image) = self.image.as_ref() {
            let name = "itunes:image";
            let mut element = BytesStart::new(name);
            element.push_attribute(("href", &**image));
            writer.write_event(Event::Start(element))?;
            writer.write_event(Event::End(BytesEnd::new(name)))?;
        }

        if let Some(duration) = self.duration.as_ref() {
            writer.write_text_element("itunes:duration", duration)?;
        }

        if let Some(explicit) = self.explicit.as_ref() {
            writer.write_text_element("itunes:explicit", explicit)?;
        }

        if let Some(closed_captioned) = self.closed_captioned.as_ref() {
            writer.write_text_element("itunes:isClosedCaptioned", closed_captioned)?;
        }

        if let Some(order) = self.order.as_ref() {
            writer.write_text_element("itunes:order", order)?;
        }

        if let Some(subtitle) = self.subtitle.as_ref() {
            writer.write_text_element("itunes:subtitle", subtitle)?;
        }

        if let Some(summary) = self.summary.as_ref() {
            writer.write_text_element("itunes:summary", summary)?;
        }

        if let Some(keywords) = self.keywords.as_ref() {
            writer.write_text_element("itunes:keywords", keywords)?;
        }

        if let Some(episode) = self.episode.as_ref() {
            writer.write_text_element("itunes:episode", episode)?;
        }

        if let Some(season) = self.season.as_ref() {
            writer.write_text_element("itunes:season", season)?;
        }

        if let Some(episode_type) = self.episode_type.as_ref() {
            writer.write_text_element("itunes:episodeType", episode_type)?;
        }

        Ok(())
    }

    fn used_namespaces(&self) -> BTreeMap<String, String> {
        let mut namespaces = BTreeMap::new();
        namespaces.insert("itunes".to_owned(), NAMESPACE.to_owned());
        namespaces
    }
}

#[cfg(feature = "builders")]
impl ITunesItemExtensionBuilder {
    /// Builds a new `ITunesItemExtension`.
    pub fn build(&self) -> ITunesItemExtension {
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
            ITunesItemExtensionBuilder::default()
                .author("John Doe".to_string())
                .build(),
            ITunesItemExtension {
                author: Some("John Doe".to_string()),
                ..Default::default()
            }
        );
    }
}
