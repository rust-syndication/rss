// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::collections::BTreeMap;
use std::fmt::Display;
use std::io::{BufRead, Write};
use std::str::{self, FromStr};

use quick_xml::events::attributes::Attributes;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, Event};
use quick_xml::Error as XmlError;
use quick_xml::Reader;
use quick_xml::Writer;

use crate::category::Category;
use crate::cloud::Cloud;
use crate::error::Error;
#[cfg(feature = "atom")]
use crate::extension::atom;
use crate::extension::dublincore;
use crate::extension::itunes::{self, is_itunes_namespace};
use crate::extension::syndication;
use crate::extension::util::{
    extension_entry, extension_name, parse_extension_element, read_namespace_declarations,
};
use crate::extension::ExtensionMap;
use crate::image::Image;
use crate::item::Item;
use crate::textinput::TextInput;
use crate::toxml::{ToXml, WriterExt};
use crate::util::{decode, element_text, skip};

/// Represents the channel of an RSS feed.
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
pub struct Channel {
    /// The name of the channel.
    pub title: String,
    /// The URL for the website corresponding to the channel.
    pub link: String,
    /// A description of the channel.
    pub description: String,
    /// The language of the channel.
    pub language: Option<String>,
    /// The copyright notice for the channel.
    pub copyright: Option<String>,
    /// The email address for the managing editor.
    pub managing_editor: Option<String>,
    /// The email address for the webmaster.
    pub webmaster: Option<String>,
    /// The publication date for the content of the channel as an RFC822 timestamp.
    pub pub_date: Option<String>,
    /// The date that the contents of the channel last changed as an RFC822 timestamp.
    pub last_build_date: Option<String>,
    /// The categories the channel belongs to.
    #[cfg_attr(feature = "builders", builder(setter(each = "category")))]
    pub categories: Vec<Category>,
    /// A string indicating the program used to generate the channel.
    pub generator: Option<String>,
    /// A URL that points to the documentation for the RSS format.
    pub docs: Option<String>,
    /// The cloud to register with to be notified of updates to the channel.
    pub cloud: Option<Cloud>,
    /// The PICS rating for the channel.
    pub rating: Option<String>,
    /// The number of minutes the channel can be cached before refreshing.
    pub ttl: Option<String>,
    /// An image that can be displayed with the channel.
    pub image: Option<Image>,
    /// A text input box that can be displayed with the channel.
    pub text_input: Option<TextInput>,
    /// A hint to tell the aggregator which hours it can skip.
    #[cfg_attr(feature = "builders", builder(setter(each = "skip_hour")))]
    pub skip_hours: Vec<String>,
    /// A hint to tell the aggregator which days it can skip.
    #[cfg_attr(feature = "builders", builder(setter(each = "skip_day")))]
    pub skip_days: Vec<String>,
    /// The items in the channel.
    #[cfg_attr(feature = "builders", builder(setter(each = "item")))]
    pub items: Vec<Item>,
    /// The extensions for the channel.
    #[cfg_attr(feature = "builders", builder(setter(each = "extension")))]
    pub extensions: ExtensionMap,
    /// The Atom extension for the channel.
    #[cfg(feature = "atom")]
    pub atom_ext: Option<atom::AtomExtension>,
    /// The iTunes extension for the channel.
    pub itunes_ext: Option<itunes::ITunesChannelExtension>,
    /// The Dublin Core extension for the channel.
    pub dublin_core_ext: Option<dublincore::DublinCoreExtension>,
    /// The Syndication extension for the channel.
    pub syndication_ext: Option<syndication::SyndicationExtension>,
    /// The namespaces present in the RSS tag.
    #[cfg_attr(feature = "builders", builder(setter(each = "namespace")))]
    pub namespaces: BTreeMap<String, String>,
}

impl Channel {
    /// Return the title of this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_title("Channel Title");
    /// assert_eq!(channel.title(), "Channel Title");
    /// ```
    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    /// Set the title of this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_title("Channel Title");
    /// ```
    pub fn set_title<V>(&mut self, title: V)
    where
        V: Into<String>,
    {
        self.title = title.into();
    }

    /// Return the URL for the website corresponding to this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_link("http://example.com");
    /// assert_eq!(channel.link(), "http://example.com");
    /// ```
    pub fn link(&self) -> &str {
        self.link.as_str()
    }

    /// Set the URL for the website corresponding to this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_link("http://example.com");
    /// ```
    pub fn set_link<V>(&mut self, link: V)
    where
        V: Into<String>,
    {
        self.link = link.into();
    }

    /// Return the description of this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_description("Channel description");
    /// assert_eq!(channel.description(), "Channel description");
    /// ```
    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    /// Set the description of this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_description("Channel description");
    /// ```
    pub fn set_description<V>(&mut self, description: V)
    where
        V: Into<String>,
    {
        self.description = description.into();
    }

    /// Return the language of this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_language("en-US".to_string());
    /// assert_eq!(channel.language(), Some("en-US"));
    /// ```
    pub fn language(&self) -> Option<&str> {
        self.language.as_ref().map(String::as_ref)
    }

    /// Set the language of this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_language("en-US".to_string());
    /// ```
    pub fn set_language<V>(&mut self, language: V)
    where
        V: Into<Option<String>>,
    {
        self.language = language.into();
    }

    /// Return the copyright notice for this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_copyright("© 2017 John Doe".to_string());
    /// assert_eq!(channel.copyright(), Some("© 2017 John Doe"));
    /// ```
    pub fn copyright(&self) -> Option<&str> {
        self.copyright.as_deref()
    }

    /// Set the copyright notice for this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_copyright("© 2017 John Doe".to_string());
    /// ```
    pub fn set_copyright<V>(&mut self, copyright: V)
    where
        V: Into<Option<String>>,
    {
        self.copyright = copyright.into();
    }

    /// Return the email address for the managing editor of this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_managing_editor("johndoe@example.com".to_string());
    /// assert_eq!(channel.managing_editor(), Some("johndoe@example.com"));
    /// ```
    pub fn managing_editor(&self) -> Option<&str> {
        self.managing_editor.as_deref()
    }

    /// Set the email address for the managing editor of this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_managing_editor("johndoe@example.com".to_string());
    /// assert_eq!(channel.managing_editor(), Some("johndoe@example.com"));
    /// ```
    pub fn set_managing_editor<V>(&mut self, managing_editor: V)
    where
        V: Into<Option<String>>,
    {
        self.managing_editor = managing_editor.into();
    }

    /// Return the email address for webmaster of this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_webmaster("johndoe@example.com".to_string());
    /// assert_eq!(channel.webmaster(), Some("johndoe@example.com"));
    /// ```
    pub fn webmaster(&self) -> Option<&str> {
        self.webmaster.as_deref()
    }

    /// Set the email address for webmaster of this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_webmaster("johndoe@example.com".to_string());
    /// ```
    pub fn set_webmaster<V>(&mut self, webmaster: V)
    where
        V: Into<Option<String>>,
    {
        self.webmaster = webmaster.into();
    }

    /// Return the publication date for the content of this channel as an RFC822 timestamp.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_pub_date("Sun, 1 Jan 2017 12:00:00 GMT".to_string());
    /// assert_eq!(channel.pub_date(), Some("Sun, 1 Jan 2017 12:00:00 GMT"));
    /// ```
    pub fn pub_date(&self) -> Option<&str> {
        self.pub_date.as_deref()
    }

    /// Set the publication date for the content of this channel as an RFC822 timestamp.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_pub_date("Sun, 1 Jan 2017 12:00:00 GMT".to_string());
    /// assert_eq!(channel.pub_date(), Some("Sun, 1 Jan 2017 12:00:00 GMT"));
    /// ```
    ///
    /// ## Using chrono::DateTime
    /// ```
    /// # #[cfg(feature = "validation")]
    /// # {
    /// use rss::Channel;
    /// use chrono::{TimeZone, Utc};
    ///
    /// let mut channel = Channel::default();
    /// channel.set_pub_date(Utc.with_ymd_and_hms(2017, 1, 1, 12, 0, 0).unwrap().to_rfc2822());
    /// assert_eq!(channel.pub_date(), Some("Sun, 1 Jan 2017 12:00:00 +0000"));
    /// # }
    /// ```
    pub fn set_pub_date<V>(&mut self, pub_date: V)
    where
        V: Into<Option<String>>,
    {
        self.pub_date = pub_date.into();
    }

    /// Return the time that the content of this channel was last changed as an RFC822 timestamp.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_last_build_date("Sun, 1 Jan 2017 12:00:00 GMT".to_string());
    /// assert_eq!(channel.last_build_date(), Some("Sun, 1 Jan 2017 12:00:00 GMT"));
    /// ```
    pub fn last_build_date(&self) -> Option<&str> {
        self.last_build_date.as_deref()
    }

    /// Set the time that the content of this channel was last changed as an RFC822 timestamp.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_last_build_date("Sun, 1 Jan 2017 12:00:00 GMT".to_string());
    /// assert_eq!(channel.last_build_date(), Some("Sun, 1 Jan 2017 12:00:00 GMT"));
    /// ```
    ///
    /// ## Using chrono::DateTime
    /// ```
    /// # #[cfg(feature = "validation")]
    /// # {
    /// use rss::Channel;
    /// use chrono::{TimeZone, Utc};
    ///
    /// let mut channel = Channel::default();
    /// channel.set_last_build_date(Utc.with_ymd_and_hms(2017, 1, 1, 12, 0, 0).unwrap().to_rfc2822());
    /// assert_eq!(channel.last_build_date(), Some("Sun, 1 Jan 2017 12:00:00 +0000"));
    /// # }
    /// ```
    pub fn set_last_build_date<V>(&mut self, last_build_date: V)
    where
        V: Into<Option<String>>,
    {
        self.last_build_date = last_build_date.into();
    }

    /// Return the categories that this channel belongs to.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{Channel, Category};
    ///
    /// let mut channel = Channel::default();
    /// channel.set_categories(vec![Category::default()]);
    /// assert_eq!(channel.categories().len(), 1);
    /// ```
    pub fn categories(&self) -> &[Category] {
        &self.categories
    }

    /// Return a mutable slice of the categories that this channel belongs to.
    pub fn categories_mut(&mut self) -> &mut [Category] {
        &mut self.categories
    }

    /// Set the categories that this channel belongs to.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{Channel, Category};
    ///
    /// let mut channel = Channel::default();
    /// channel.set_categories(vec![Category::default()]);
    /// ```
    pub fn set_categories<V>(&mut self, categories: V)
    where
        V: Into<Vec<Category>>,
    {
        self.categories = categories.into();
    }

    /// Return a string indicating the program used to generate the channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_generator("Program Name".to_string());
    /// assert_eq!(channel.generator(), Some("Program Name"));
    /// ```
    pub fn generator(&self) -> Option<&str> {
        self.generator.as_deref()
    }

    /// Set a string indicating the program used to generate the channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_generator("Program Name".to_string());
    /// ```
    pub fn set_generator<V>(&mut self, generator: V)
    where
        V: Into<Option<String>>,
    {
        self.generator = generator.into();
    }

    /// Return a URL that points to the documentation of the RSS format used in this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_docs("https://cyber.harvard.edu/rss/rss.html".to_string());
    /// assert_eq!(channel.docs(), Some("https://cyber.harvard.edu/rss/rss.html"));
    /// ```
    pub fn docs(&self) -> Option<&str> {
        self.docs.as_deref()
    }

    /// Set a URL that points to the documentation of the RSS format used in this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_docs("https://cyber.harvard.edu/rss/rss.html".to_string());
    /// ```
    pub fn set_docs<V>(&mut self, docs: V)
    where
        V: Into<Option<String>>,
    {
        self.docs = docs.into();
    }

    /// Return the information used to register with a cloud for notifications of updates to the
    /// channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{Channel, Cloud};
    ///
    /// let mut channel = Channel::default();
    /// channel.set_cloud(Cloud::default());
    /// assert!(channel.cloud().is_some());
    /// ```
    pub fn cloud(&self) -> Option<&Cloud> {
        self.cloud.as_ref()
    }

    /// Set the information used to register with a cloud for notifications of updates to the
    /// channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{Channel, Cloud};
    ///
    /// let mut channel = Channel::default();
    /// channel.set_cloud(Cloud::default());
    /// ```
    pub fn set_cloud<V>(&mut self, cloud: V)
    where
        V: Into<Option<Cloud>>,
    {
        self.cloud = cloud.into();
    }

    /// Return the time to live of this channel. This indicates the number of minutes the
    /// channel can be cached before needing to be refreshed.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_ttl("60".to_string());
    /// assert_eq!(channel.ttl(), Some("60"));
    /// ```
    pub fn ttl(&self) -> Option<&str> {
        self.ttl.as_deref()
    }

    /// Set the time to live of this channel. This indicates the number of minutes the
    /// channel can be cached before needing to be refreshed.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_ttl("60".to_string());
    /// ```
    pub fn set_ttl<V>(&mut self, ttl: V)
    where
        V: Into<Option<String>>,
    {
        self.ttl = ttl.into();
    }

    /// Return the image to be displayed with this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{Channel, Image};
    ///
    /// let mut channel = Channel::default();
    /// channel.set_image(Image::default());
    /// assert!(channel.image().is_some());
    /// ```
    pub fn image(&self) -> Option<&Image> {
        self.image.as_ref()
    }

    /// Set the image to be displayed with this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{Channel, Image};
    ///
    /// let mut channel = Channel::default();
    /// channel.set_image(Image::default());
    /// ```
    pub fn set_image<V>(&mut self, image: V)
    where
        V: Into<Option<Image>>,
    {
        self.image = image.into();
    }

    /// Return the [PICS](https://www.w3.org/PICS/) rating for this channel.
    pub fn rating(&self) -> Option<&str> {
        self.rating.as_deref()
    }

    /// Set the [PICS](https://www.w3.org/PICS/) rating for this channel.
    pub fn set_rating<V>(&mut self, rating: V)
    where
        V: Into<Option<String>>,
    {
        self.rating = rating.into();
    }

    /// Return the information for a text box to be displayed with this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{Channel, TextInput};
    ///
    /// let mut channel = Channel::default();
    /// channel.set_text_input(TextInput::default());
    /// assert!(channel.text_input().is_some());
    /// ```
    pub fn text_input(&self) -> Option<&TextInput> {
        self.text_input.as_ref()
    }

    /// Set the information for a text box to be displayed with this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{Channel, TextInput};
    ///
    /// let mut channel = Channel::default();
    /// channel.set_text_input(TextInput::default());
    /// ```
    pub fn set_text_input<V>(&mut self, text_input: V)
    where
        V: Into<Option<TextInput>>,
    {
        self.text_input = text_input.into();
    }

    /// Return the hours that aggregators can skip for refreshing content.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    ///
    /// let skip_hours = vec![6, 7, 8, 14, 22];
    ///
    /// let mut channel = Channel::default();
    /// channel.set_skip_hours(vec!["1".to_string()]);
    /// assert_eq!(channel.skip_hours().len(), 1);
    /// ```
    pub fn skip_hours(&self) -> &[String] {
        &self.skip_hours
    }

    /// Return a mutable slice of the hours that aggregators can skip for refreshing content.
    pub fn skip_hours_mut(&mut self) -> &mut [String] {
        &mut self.skip_hours
    }

    /// Set the hours that aggregators can skip for refreshing content.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_skip_hours(vec!["1".to_string()]);
    /// ```
    pub fn set_skip_hours<V>(&mut self, skip_hours: V)
    where
        V: Into<Vec<String>>,
    {
        self.skip_hours = skip_hours.into();
    }

    /// Return the days that aggregators can skip for refreshing content.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_skip_days(vec!["Monday".to_string()]);
    /// assert_eq!(channel.skip_days().len(), 1);
    /// ```
    pub fn skip_days(&self) -> &[String] {
        &self.skip_days
    }

    /// Return a mutable slice of the days that aggregators can skip for refreshing content.
    pub fn skip_days_mut(&mut self) -> &mut [String] {
        &mut self.skip_days
    }

    /// Set the days that aggregators can skip for refreshing content.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_skip_days(vec!["Monday".to_string()]);
    /// ```
    pub fn set_skip_days<V>(&mut self, skip_days: V)
    where
        V: Into<Vec<String>>,
    {
        self.skip_days = skip_days.into();
    }

    /// Return the items in this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{Channel, Item};
    ///
    /// let mut channel = Channel::default();
    /// channel.set_items(vec![Item::default()]);
    /// assert_eq!(channel.items().len(), 1);
    /// ```
    pub fn items(&self) -> &[Item] {
        &self.items
    }

    /// Return a mutable slice of the items in this channel.
    pub fn items_mut(&mut self) -> &mut [Item] {
        &mut self.items
    }

    /// Consume the `Channel` and return a vector of `Item`s.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{Channel, Item};
    ///
    /// let mut channel = Channel::default();
    /// channel.set_items(vec![Item::default()]);
    /// assert_eq!(channel.into_items().len(), 1);
    /// ```
    pub fn into_items(self) -> Vec<Item> {
        self.items
    }

    /// Set the items in this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{Channel, Item};
    ///
    /// let mut channel = Channel::default();
    /// channel.set_items(vec![Item::default()]);
    /// ```
    pub fn set_items<V>(&mut self, items: V)
    where
        V: Into<Vec<Item>>,
    {
        self.items = items.into();
    }

    /// Return the Atom extension for this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    /// use rss::extension::atom::AtomExtension;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_atom_ext(AtomExtension::default());
    /// assert!(channel.atom_ext().is_some());
    /// ```
    #[cfg(feature = "atom")]
    pub fn atom_ext(&self) -> Option<&atom::AtomExtension> {
        self.atom_ext.as_ref()
    }

    /// Set the Atom extension for this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    /// use rss::extension::atom::AtomExtension;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_atom_ext(AtomExtension::default());
    /// ```
    #[cfg(feature = "atom")]
    pub fn set_atom_ext<V>(&mut self, atom_ext: V)
    where
        V: Into<Option<atom::AtomExtension>>,
    {
        self.atom_ext = atom_ext.into();
    }

    /// Return the iTunes extension for this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    /// use rss::extension::itunes::ITunesChannelExtension;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_itunes_ext(ITunesChannelExtension::default());
    /// assert!(channel.itunes_ext().is_some());
    /// ```
    pub fn itunes_ext(&self) -> Option<&itunes::ITunesChannelExtension> {
        self.itunes_ext.as_ref()
    }

    /// Set the iTunes extension for this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    /// use rss::extension::itunes::ITunesChannelExtension;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_itunes_ext(ITunesChannelExtension::default());
    /// ```
    pub fn set_itunes_ext<V>(&mut self, itunes_ext: V)
    where
        V: Into<Option<itunes::ITunesChannelExtension>>,
    {
        self.itunes_ext = itunes_ext.into();
    }

    /// Return the Dublin Core extension for this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    /// use rss::extension::dublincore::DublinCoreExtension;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_dublin_core_ext(DublinCoreExtension::default());
    /// assert!(channel.dublin_core_ext().is_some());
    /// ```
    pub fn dublin_core_ext(&self) -> Option<&dublincore::DublinCoreExtension> {
        self.dublin_core_ext.as_ref()
    }

    /// Set the Dublin Core extension for this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    /// use rss::extension::dublincore::DublinCoreExtension;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_dublin_core_ext(DublinCoreExtension::default());
    /// ```
    pub fn set_dublin_core_ext<V>(&mut self, dublin_core_ext: V)
    where
        V: Into<Option<dublincore::DublinCoreExtension>>,
    {
        self.dublin_core_ext = dublin_core_ext.into();
    }

    /// Return the Syndication extension for this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    /// use rss::extension::syndication::SyndicationExtension;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_syndication_ext(SyndicationExtension::default());
    /// assert!(channel.syndication_ext().is_some());
    /// ```
    pub fn syndication_ext(&self) -> Option<&syndication::SyndicationExtension> {
        self.syndication_ext.as_ref()
    }

    /// Set the Syndication extension for this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    /// use rss::extension::syndication::SyndicationExtension;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_syndication_ext(SyndicationExtension::default());
    /// ```
    pub fn set_syndication_ext<V>(&mut self, syndication_ext: V)
    where
        V: Into<Option<syndication::SyndicationExtension>>,
    {
        self.syndication_ext = syndication_ext.into();
    }

    /// Return the extensions for this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeMap;
    /// use rss::Channel;
    /// use rss::extension::{ExtensionMap, Extension};
    ///
    /// let extension = Extension::default();
    ///
    /// let mut item_map = BTreeMap::<String, Vec<Extension>>::new();
    /// item_map.insert("ext:name".to_string(), vec![extension]);
    ///
    /// let mut extension_map = ExtensionMap::default();
    /// extension_map.insert("ext".to_string(), item_map);
    ///
    /// let mut channel = Channel::default();
    /// channel.set_extensions(extension_map);
    /// assert_eq!(channel.extensions()
    ///                   .get("ext")
    ///                   .and_then(|m| m.get("ext:name"))
    ///                   .map(|v| v.len()),
    ///            Some(1));
    /// ```
    pub fn extensions(&self) -> &ExtensionMap {
        &self.extensions
    }

    /// Set the extensions for this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    /// use rss::extension::ExtensionMap;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_extensions(ExtensionMap::default());
    /// ```
    pub fn set_extensions<V>(&mut self, extensions: V)
    where
        V: Into<ExtensionMap>,
    {
        self.extensions = extensions.into()
    }

    /// Return the namespaces for this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeMap;
    /// use rss::Channel;
    ///
    /// let mut namespaces = BTreeMap::new();
    /// namespaces.insert("ext".to_string(), "http://example.com".to_string());
    ///
    /// let mut channel = Channel::default();
    /// channel.set_namespaces(namespaces);
    /// assert_eq!(channel.namespaces().get("ext").map(String::as_str), Some("http://example.com"));
    /// ```
    pub fn namespaces(&self) -> &BTreeMap<String, String> {
        &self.namespaces
    }

    /// Set the namespaces for this channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeMap;
    /// use rss::Channel;
    ///
    /// let mut channel = Channel::default();
    /// channel.set_namespaces(BTreeMap::new());
    /// ```
    pub fn set_namespaces<V>(&mut self, namespaces: V)
    where
        V: Into<BTreeMap<String, String>>,
    {
        self.namespaces = namespaces.into()
    }
}

impl Channel {
    /// Attempt to read an RSS channel from a reader.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let reader: BufRead = ...;
    /// let channel = Channel::read_from(reader).unwrap();
    /// ```
    pub fn read_from<R: BufRead>(reader: R) -> Result<Channel, Error> {
        let mut reader = Reader::from_reader(reader);
        reader.config_mut().expand_empty_elements = true;
        let namespaces;
        let mut buf = Vec::new();

        let mut channel: Option<Channel> = None;

        // for parsing RSS 0.9, 1.0 feeds
        let mut items: Option<Vec<Item>> = None;
        let mut image: Option<Image> = None;
        let mut text_input: Option<TextInput> = None;

        // find opening element
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(element) => match decode(element.name().as_ref(), &reader)?.as_ref() {
                    "rss" | "rdf:RDF" => {
                        namespaces = read_namespace_declarations(
                            &mut reader,
                            element.attributes(),
                            &BTreeMap::new(),
                        )?
                        .into_owned();
                        break;
                    }
                    _ => {
                        return Err(Error::InvalidStartTag);
                    }
                },
                Event::Eof => return Err(Error::Eof),
                _ => continue,
            }
        }

        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(element) => match decode(element.name().as_ref(), &reader)?.as_ref() {
                    "channel" => {
                        let inner =
                            Channel::from_xml(&namespaces, &mut reader, element.attributes())?;
                        channel = Some(inner);
                    }
                    "item" => {
                        let item = Item::from_xml(&namespaces, &mut reader, element.attributes())?;
                        if items.is_none() {
                            items = Some(Vec::new());
                        }
                        items.as_mut().unwrap().push(item);
                    }
                    "image" => {
                        let inner = Image::from_xml(&mut reader, element.attributes())?;
                        image = Some(inner);
                    }
                    "textinput" => {
                        let inner = TextInput::from_xml(&mut reader, element.attributes())?;
                        text_input = Some(inner);
                    }
                    _ => skip(element.name(), &mut reader)?,
                },
                Event::End(_) | Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }

        if let Some(mut channel) = channel {
            if let Some(mut items) = items {
                channel.items.append(&mut items);
            }

            if image.is_some() {
                channel.image = image;
            }

            if text_input.is_some() {
                channel.text_input = text_input;
            }

            channel.namespaces = namespaces;

            Ok(channel)
        } else {
            Err(Error::Eof)
        }
    }

    fn write<W: Write>(&self, mut writer: Writer<W>) -> Result<W, Error> {
        writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("utf-8"), None)))?;

        let name = "rss";
        let mut element = BytesStart::new(name);
        element.push_attribute(("version", "2.0"));

        let used_namespaces = self.used_namespaces();
        let mut namespaces: BTreeMap<&String, &String> = BTreeMap::new();
        namespaces.extend(&used_namespaces);
        namespaces.extend(&self.namespaces);
        for (name, url) in namespaces {
            element.push_attribute((format!("xmlns:{}", name).as_bytes(), url.as_bytes()));
        }

        writer.write_event(Event::Start(element))?;

        self.to_xml(&mut writer)?;

        writer.write_event(Event::End(BytesEnd::new(name)))?;

        Ok(writer.into_inner())
    }

    /// Attempt to write the RSS channel as XML to a writer.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let channel: Channel = ...;
    /// let writer: Write = ...;
    /// channel.write_to(writer).unwrap();
    /// ```
    pub fn write_to<W: Write>(&self, writer: W) -> Result<W, Error> {
        self.write(::quick_xml::Writer::new(writer))
    }

    /// Attempt to write the RSS channel as pretty XML to a writer.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let channel: Channel = ...;
    /// let writer: Write = ...;
    /// channel.pretty_write_to(writer, b' ', 2).unwrap();
    /// ```
    pub fn pretty_write_to<W: Write>(
        &self,
        writer: W,
        indent_char: u8,
        indent_size: usize,
    ) -> Result<W, Error> {
        self.write(::quick_xml::Writer::new_with_indent(
            writer,
            indent_char,
            indent_size,
        ))
    }
}

impl Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buf = self.write_to(Vec::new()).unwrap_or_default();
        // this unwrap should be safe since the bytes written from the Channel are all valid utf8
        f.write_str(String::from_utf8(buf).unwrap().as_str())
    }
}

impl Channel {
    /// Builds a Channel from source XML
    pub fn from_xml<R: BufRead>(
        namespaces: &BTreeMap<String, String>,
        reader: &mut Reader<R>,
        atts: Attributes,
    ) -> Result<Self, Error> {
        let mut channel = Channel::default();
        let mut extensions = ExtensionMap::new();
        let mut buf = Vec::new();
        let mut skip_buf = Vec::new();

        let namespaces = read_namespace_declarations(reader, atts, namespaces)?;

        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(element) => match decode(element.name().as_ref(), reader)?.as_ref() {
                    "category" => {
                        let category = Category::from_xml(reader, element.attributes())?;
                        channel.categories.push(category);
                    }
                    "cloud" => {
                        let cloud = Cloud::from_xml(reader, &element)?;
                        channel.cloud = Some(cloud);
                    }
                    "image" => {
                        let image = Image::from_xml(reader, element.attributes())?;
                        channel.image = Some(image);
                    }
                    "textInput" => {
                        let text_input = TextInput::from_xml(reader, element.attributes())?;
                        channel.text_input = Some(text_input);
                    }
                    "item" => {
                        let item =
                            Item::from_xml(namespaces.as_ref(), reader, element.attributes())?;
                        channel.items.push(item);
                    }
                    "title" => {
                        if let Some(content) = element_text(reader)? {
                            channel.title = content;
                        }
                    }
                    "link" => {
                        if let Some(content) = element_text(reader)? {
                            channel.link = content;
                        }
                    }
                    "description" => {
                        if let Some(content) = element_text(reader)? {
                            channel.description = content;
                        }
                    }
                    "language" => channel.language = element_text(reader)?,
                    "copyright" => channel.copyright = element_text(reader)?,
                    "managingEditor" => {
                        channel.managing_editor = element_text(reader)?;
                    }
                    "webMaster" => channel.webmaster = element_text(reader)?,
                    "pubDate" => channel.pub_date = element_text(reader)?,
                    "lastBuildDate" => {
                        channel.last_build_date = element_text(reader)?;
                    }
                    "generator" => channel.generator = element_text(reader)?,
                    "rating" => channel.rating = element_text(reader)?,
                    "docs" => channel.docs = element_text(reader)?,
                    "ttl" => channel.ttl = element_text(reader)?,
                    "skipHours" => loop {
                        skip_buf.clear();
                        match reader.read_event_into(&mut skip_buf)? {
                            Event::Start(element) => {
                                if decode(element.name().as_ref(), reader)?.as_ref() == "hour" {
                                    if let Some(content) = element_text(reader)? {
                                        channel.skip_hours.push(content);
                                    }
                                } else {
                                    skip(element.name(), reader)?;
                                }
                            }
                            Event::End(_) | Event::Eof => break,
                            _ => {}
                        }
                    },
                    "skipDays" => loop {
                        skip_buf.clear();
                        match reader.read_event_into(&mut skip_buf)? {
                            Event::Start(element) => {
                                if decode(element.name().as_ref(), reader)?.as_ref() == "day" {
                                    if let Some(content) = element_text(reader)? {
                                        channel.skip_days.push(content);
                                    }
                                } else {
                                    skip(element.name(), reader)?;
                                }
                            }
                            Event::End(_) | Event::Eof => break,
                            _ => {}
                        }
                    },
                    n => {
                        if let Some((prefix, name)) = extension_name(n) {
                            let scope_namespases = read_namespace_declarations(
                                reader,
                                element.attributes(),
                                namespaces.as_ref(),
                            )?;
                            let ext_ns = scope_namespases.get(prefix).map(|s| s.as_str());
                            let ext = parse_extension_element(reader, element.attributes())?;
                            match ext_ns {
                                #[cfg(feature = "atom")]
                                Some(ns @ atom::NAMESPACE) => {
                                    extension_entry(&mut extensions, ns, name).push(ext);
                                }
                                Some(ns) if is_itunes_namespace(ns) => {
                                    extension_entry(&mut extensions, itunes::NAMESPACE, name)
                                        .push(ext);
                                }
                                Some(ns @ dublincore::NAMESPACE)
                                | Some(ns @ syndication::NAMESPACE) => {
                                    extension_entry(&mut extensions, ns, name).push(ext);
                                }
                                _ => {
                                    extension_entry(&mut channel.extensions, prefix, name).push(ext)
                                }
                            }
                        } else {
                            skip(element.name(), reader)?;
                        }
                    }
                },
                Event::End(_) => break,
                Event::Eof => return Err(Error::Eof),
                _ => {}
            }

            buf.clear();
        }

        // Process each of the namespaces we know
        #[cfg(feature = "atom")]
        if let Some(v) = extensions.remove(atom::NAMESPACE) {
            channel.atom_ext = Some(atom::AtomExtension::from_map(v));
        }
        if let Some(v) = extensions.remove(itunes::NAMESPACE) {
            channel.itunes_ext = Some(itunes::ITunesChannelExtension::from_map(v));
        }
        if let Some(v) = extensions.remove(dublincore::NAMESPACE) {
            channel.dublin_core_ext = Some(dublincore::DublinCoreExtension::from_map(v));
        }
        if let Some(v) = extensions.remove(syndication::NAMESPACE) {
            channel.syndication_ext = Some(syndication::SyndicationExtension::from_map(v));
        }

        Ok(channel)
    }
}

impl ToXml for Channel {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = "channel";

        writer.write_event(Event::Start(BytesStart::new(name)))?;

        writer.write_text_element("title", &self.title)?;
        writer.write_text_element("link", &self.link)?;
        writer.write_text_element("description", &self.description)?;

        if let Some(language) = self.language.as_ref() {
            writer.write_text_element("language", language)?;
        }

        if let Some(copyright) = self.copyright.as_ref() {
            writer.write_text_element("copyright", copyright)?;
        }

        if let Some(managing_editor) = self.managing_editor.as_ref() {
            writer.write_text_element("managingEditor", managing_editor)?;
        }

        if let Some(webmaster) = self.webmaster.as_ref() {
            writer.write_text_element("webMaster", webmaster)?;
        }

        if let Some(pub_date) = self.pub_date.as_ref() {
            writer.write_text_element("pubDate", pub_date)?;
        }

        if let Some(last_build_date) = self.last_build_date.as_ref() {
            writer.write_text_element("lastBuildDate", last_build_date)?;
        }

        writer.write_objects(&self.categories)?;

        if let Some(generator) = self.generator.as_ref() {
            writer.write_text_element("generator", generator)?;
        }

        if let Some(rating) = self.rating.as_ref() {
            writer.write_text_element("rating", rating)?;
        }

        if let Some(docs) = self.docs.as_ref() {
            writer.write_text_element("docs", docs)?;
        }

        if let Some(cloud) = self.cloud.as_ref() {
            writer.write_object(cloud)?;
        }

        if let Some(ttl) = self.ttl.as_ref() {
            writer.write_text_element("ttl", ttl)?;
        }

        if let Some(image) = self.image.as_ref() {
            writer.write_object(image)?;
        }

        if let Some(text_input) = self.text_input.as_ref() {
            writer.write_object(text_input)?;
        }

        if !self.skip_hours.is_empty() {
            let name = "skipHours";
            writer.write_event(Event::Start(BytesStart::new(name)))?;
            for hour in &self.skip_hours {
                writer.write_text_element("hour", hour)?;
            }
            writer.write_event(Event::End(BytesEnd::new(name)))?;
        }

        if !self.skip_days.is_empty() {
            let name = "skipDays";
            writer.write_event(Event::Start(BytesStart::new(name)))?;
            for day in &self.skip_days {
                writer.write_text_element("day", day)?;
            }
            writer.write_event(Event::End(BytesEnd::new(name)))?;
        }

        for map in self.extensions.values() {
            for extensions in map.values() {
                for extension in extensions {
                    extension.to_xml(writer)?;
                }
            }
        }

        #[cfg(feature = "atom")]
        if let Some(ext) = &self.atom_ext {
            ext.to_xml(writer)?;
        }

        if let Some(ext) = &self.itunes_ext {
            ext.to_xml(writer)?;
        }

        if let Some(ext) = &self.dublin_core_ext {
            ext.to_xml(writer)?;
        }

        if let Some(ext) = &self.syndication_ext {
            ext.to_xml(&self.namespaces, writer)?;
        }

        writer.write_objects(&self.items)?;

        writer.write_event(Event::End(BytesEnd::new(name)))?;
        Ok(())
    }

    fn used_namespaces(&self) -> BTreeMap<String, String> {
        let mut namespaces = BTreeMap::new();
        for item in &self.items {
            namespaces.extend(item.used_namespaces());
        }
        if let Some(ext) = self.itunes_ext() {
            namespaces.extend(ext.used_namespaces());
        }
        if let Some(ext) = self.dublin_core_ext() {
            namespaces.extend(ext.used_namespaces());
        }
        #[cfg(feature = "atom")]
        if let Some(ext) = self.atom_ext() {
            namespaces.extend(ext.used_namespaces());
        }
        namespaces
    }
}

impl FromStr for Channel {
    type Err = Error;

    #[inline]
    fn from_str(s: &str) -> Result<Channel, Error> {
        Channel::read_from(s.as_bytes())
    }
}

#[cfg(feature = "builders")]
impl ChannelBuilder {
    /// Builds a new `Channel`.
    pub fn build(&self) -> Channel {
        self.build_impl().unwrap()
    }
}
