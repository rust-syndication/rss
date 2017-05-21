// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use category::{Category, CategoryBuilder};
use chrono::DateTime;
use cloud::{Cloud, CloudBuilder};
use enclosure::EnclosureBuilder;
use error::Error;
use extension::{self, ExtensionMap};
use extension::dublincore::DublinCoreExtension;
use extension::itunes::ITunesChannelExtension;
use fromxml::{self, FromXml};
use guid::GuidBuilder;
use image::{Image, ImageBuilder};
use item::{Item, ItemBuilder};
use quick_xml::{Element, Event, XmlReader, XmlWriter};
use quick_xml::error::Error as XmlError;
use source::SourceBuilder;
use std::collections::HashMap;
use std::i64;
use std::str::{self, FromStr};
use textinput::{TextInput, TextInputBuilder};
use toxml::{ToXml, XmlWriterExt};
use url::Url;

/// A representation of the `<channel>` element.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Channel {
    /// The name of the channel.
    title: String,
    /// The URL for the website corresponding to the channel.
    link: String,
    /// A description of the channel.
    description: String,
    /// The language of the channel.
    language: Option<String>,
    /// The copyright notice for the channel.
    copyright: Option<String>,
    /// The email address for the managing editor.
    managing_editor: Option<String>,
    /// The email address for the webmaster.
    webmaster: Option<String>,
    /// The publication date for the content of the channel.
    pub_date: Option<String>,
    /// The date that the contents of the channel last changed.
    last_build_date: Option<String>,
    /// The categories the channel belongs to.
    categories: Vec<Category>,
    /// The program used to generate the channel.
    generator: Option<String>,
    /// A URL that points to the documentation for the RSS format.
    docs: Option<String>,
    /// The cloud to register with to be notified of updates to the channel.
    cloud: Option<Cloud>,
    /// The PICS rating for the channel.
    rating: Option<String>,
    /// The number of minutes the channel can be cached before refreshing.
    ttl: Option<String>,
    /// An image that can be displayed with the channel.
    image: Option<Image>,
    /// A text input box that can be displayed with the channel.
    text_input: Option<TextInput>,
    /// A hint to tell the aggregator which hours it can skip.
    skip_hours: Vec<String>,
    /// A hint to tell the aggregator which days it can skip.
    skip_days: Vec<String>,
    /// The items in the channel.
    items: Vec<Item>,
    /// The extensions for the channel.
    extensions: ExtensionMap,
    /// The iTunes extension for the channel.
    itunes_ext: Option<ITunesChannelExtension>,
    /// The Dublin Core extension for the channel.
    dublin_core_ext: Option<DublinCoreExtension>,
    /// The namespaces present in the RSS tag.
    namespaces: HashMap<String, String>,
}

impl Channel {
    /// Get the title that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let title = "The Linux Action Show! OGG";
    ///
    /// let channels = ChannelBuilder::new()
    ///     .title(title)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(title.to_string(), channels.title());
    /// ```
    pub fn title(&self) -> &str {
        self.title
            .as_str()
    }


    /// Get the link that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let link = "http://www.jupiterbroadcasting.com/";
    ///
    /// let channels = ChannelBuilder::new()
    ///     .link(link)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(link.to_string(), channels.link());
    /// ```
    pub fn link(&self) -> &str {
        self.link
            .as_str()
    }


    /// Get the description that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let description = "Ogg Vorbis audio versions of The Linux ".to_string()
    /// + "Action Show! A show that covers everything geeks care about in the "
    /// + "computer industry. Get a solid dose of Linux, gadgets, news events "
    /// + "and much more!";
    ///
    /// let channels = ChannelBuilder::new()
    ///     .description(description.as_ref())
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(description.to_string(), channels.description());
    /// ```
    pub fn description(&self) -> &str {
        self.description
            .as_str()
    }


    /// Get the optional language that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let language_string = "en";
    ///
    /// let channels = ChannelBuilder::new()
    ///     .language(Some(language_string.to_string()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(Some(language_string), channels.language());
    /// ```
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .language(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.language().is_none());
    /// ```
    pub fn language(&self) -> Option<&str> {
        self.language
            .as_ref()
            .map(|s| s.as_str())
    }


    /// Get the optional copyright that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let copyright_string =
    ///     "Copyright 2002, Spartanburg Herald-Journal";
    ///
    /// let channels = ChannelBuilder::new()
    ///     .copyright(Some(copyright_string.to_string()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(Some(copyright_string), channels.copyright());
    /// ```
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .copyright(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.copyright().is_none());
    /// ```
    pub fn copyright(&self) -> Option<&str> {
        self.copyright
            .as_ref()
            .map(|s| s.as_str())
    }


    /// Get the optional managing editor that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let managing_editor_string =
    ///     "chris@jupiterbroadcasting.com (Chris Fisher)";
    ///
    /// let channels = ChannelBuilder::new()
    ///     .managing_editor(Some(managing_editor_string.to_string()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(Some(managing_editor_string), channels.managing_editor());
    /// ```
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .managing_editor(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.managing_editor().is_none());
    /// ```
    pub fn managing_editor(&self) -> Option<&str> {
        self.managing_editor
            .as_ref()
            .map(|s| s.as_str())
    }

    /// Get the optional web master that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let webmaster_string =
    ///     "chris@jupiterbroadcasting.com (Chris Fisher)";
    ///
    /// let channels = ChannelBuilder::new()
    ///     .webmaster(Some(webmaster_string.to_string()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(Some(webmaster_string), channels.webmaster());
    /// ```
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .webmaster(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.webmaster().is_none());
    /// ```
    pub fn webmaster(&self) -> Option<&str> {
        self.webmaster
            .as_ref()
            .map(|s| s.as_str())
    }


    /// Get the optional pub date that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let pub_date = "Sun, 13 Mar 2016 20:02:02 -0700";
    ///
    /// let channels = ChannelBuilder::new()
    ///     .pub_date(Some(pub_date.to_string()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(Some(pub_date), channels.pub_date());
    /// ```
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .pub_date(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.pub_date().is_none());
    /// ```
    pub fn pub_date(&self) -> Option<&str> {
        self.pub_date
            .as_ref()
            .map(|s| s.as_str())
    }


    /// Get the optional last build date that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let last_build_date = "Sun, 13 Mar 2016 20:02:02 -0700";
    ///
    /// let channels = ChannelBuilder::new()
    ///     .last_build_date(Some(last_build_date.to_string()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let local = channels.last_build_date();
    /// assert!(local.is_some());
    ///
    /// assert_eq!(Some(last_build_date), channels.last_build_date());
    /// ```
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .last_build_date(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.last_build_date().is_none());
    /// ```
    pub fn last_build_date(&self) -> Option<&str> {
        self.last_build_date
            .as_ref()
            .map(|s| s.as_str())
    }


    /// Get the categories that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel, CategoryBuilder};
    ///
    /// let category_1 = CategoryBuilder::new()
    ///     .domain(None)
    ///     .name("Media")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let category_2 = CategoryBuilder::new()
    ///     .domain(Some("http://jupiterbroadcasting.com".to_string()))
    ///     .name("Podcast")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let categories_vec = vec![category_1, category_2];
    ///
    /// let channels = ChannelBuilder::new()
    ///     .categories(categories_vec.clone())
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let categories = channels.categories();
    /// assert!(!categories.is_empty());
    ///
    /// assert_eq!(categories_vec.clone().len(), categories.len());
    /// ```
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .categories(Vec::new())
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.categories().is_empty());
    /// ```
    pub fn categories(&self) -> &[Category] {
        &self.categories
    }


    /// Get the optional generator that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let generator_string = "Feeder 2.5.12(2294); Mac OS X Version 10.9.5 (Build 13F34)
    /// http://reinventedsoftware.com/feeder/";
    ///
    /// let channels = ChannelBuilder::new()
    ///     .generator(Some(generator_string.to_string()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(Some(generator_string), channels.generator());
    /// ```
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .generator(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.generator().is_none());
    /// ```
    pub fn generator(&self) -> Option<&str> {
        self.generator
            .as_ref()
            .map(|s| s.as_str())
    }


    /// Get the optional docs that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let docs_string = "http://blogs.law.harvard.edu/tech/rss/";
    ///
    /// let channels = ChannelBuilder::new()
    ///     .docs(Some(docs_string.to_string()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let docs_option = channels.docs();
    /// assert!(docs_option.is_some());
    ///
    /// assert_eq!(Some(docs_string), channels.docs());
    /// ```
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .docs(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.docs().is_none());
    /// ```
    pub fn docs(&self) -> Option<&str> {
        self.docs
            .as_ref()
            .map(|s| s.as_str())
    }

    /// Get the optional cloud that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel, CloudBuilder};
    ///
    /// let cloud = CloudBuilder::new()
    ///     .domain("http://rpc.sys.com/")
    ///     .port(80)
    ///     .path("/RPC2")
    ///     .register_procedure("pingMe")
    ///     .protocol("soap")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let channels = ChannelBuilder::new()
    ///     .cloud(Some(cloud))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.cloud().is_some());
    /// ```
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .cloud(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.cloud().is_none());
    /// ```
    pub fn cloud(&self) -> Option<&Cloud> {
        self.cloud
            .as_ref()
    }


    /// Get the optional ttl that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let ttl_num = 60;
    ///
    /// let channels = ChannelBuilder::new()
    ///     .ttl(Some(ttl_num))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(Some(ttl_num.to_string().as_str()), channels.ttl());
    /// ```
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .ttl(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.ttl().is_none());
    /// ```
    pub fn ttl(&self) -> Option<&str> {
        self.ttl
            .as_ref()
            .map(|s| s.as_str())
    }


    /// Get the optional image that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel, ImageBuilder};
    ///
    /// let image = ImageBuilder::new()
    ///     .link("http://www.jupiterbroadcasting.com")
    ///     .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg")
    ///     .title("LAS 300 Logo")
    ///     .height(None)
    ///     .width(None)
    ///     .description(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let channels = ChannelBuilder::new()
    ///     .image(Some(image))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.image().is_some());
    /// ```
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .image(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.image().is_none());
    /// ```
    pub fn image(&self) -> Option<&Image> {
        self.image
            .as_ref()
    }


    /// Get the optional rating that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .rating(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.rating().is_none());
    /// ```
    pub fn rating(&self) -> Option<&str> {
        self.rating
            .as_ref()
            .map(|s| s.as_str())
    }


    /// Get the optional text input that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel, TextInputBuilder};
    ///
    /// let text_input = TextInputBuilder::new()
    ///     .title("Enter Comment")
    ///     .description("Provided Feedback")
    ///     .name("Comment")
    ///     .link("http://www.example.com/feedback")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let channels = ChannelBuilder::new()
    ///     .text_input(Some(text_input))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.text_input().is_some());
    /// ```
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .text_input(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.text_input().is_none());
    /// ```
    pub fn text_input(&self) -> Option<&TextInput> {
        self.text_input
            .as_ref()
    }

    /// Get the skip hours that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let skip_hours_vec: Vec<i64> = vec![6,7,8,14,22];
    ///
    /// let channels = ChannelBuilder::new()
    ///     .skip_hours(skip_hours_vec.clone())
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let skip_hours  = channels.skip_hours();
    /// assert!(!skip_hours.is_empty());
    ///
    /// let len = skip_hours_vec.clone().len();
    /// assert_eq!(len, skip_hours.len());
    ///
    /// for x in 0..len {
    ///     assert_eq!(skip_hours_vec[x].to_string(), skip_hours[x]);
    /// }
    /// ```
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .skip_hours(Vec::new())
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.skip_hours().is_empty());
    /// ```
    pub fn skip_hours(&self) -> &[String] {
        &self.skip_hours
    }


    /// Get the skip days that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let skip_days_vec: Vec<String> = vec!["Monday".to_string(),
    /// "Sunday".to_string(), "Thursday".to_owned(),
    ///     "Wednesday".to_string()];
    ///
    /// let channels = ChannelBuilder::new()
    ///     .skip_days(skip_days_vec.clone())
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let skip_days = channels.skip_days();
    /// assert!(!skip_days.is_empty());
    ///
    /// let len = skip_days_vec.clone().len();
    /// assert_eq!(len, skip_days.len());
    ///
    /// for x in 0..len {
    ///     assert_eq!(skip_days_vec[x], skip_days[x].clone());
    /// }
    /// ```
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .skip_days(Vec::new())
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.skip_days().is_empty());
    /// ```
    pub fn skip_days(&self) -> &[String] {
        &self.skip_days
    }


    /// Get the items that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel, ItemBuilder};
    ///
    /// let link = "http://www.jupiterbroadcasting.com/97561/".to_string()
    /// + "making-music-with-linux-las-408/";
    ///
    /// let description = "<![CDATA[<p>In special Rasberry Pi 3 ".to_string()
    /// + "edition of the show we look at the new hardware, review & chat with "
    /// + "Mycroft CTO Ryan Sipes on how important the Raspberry Pi is for "
    /// + "development of their open artificial intelligence platform & get "
    /// + "the latest news.</p><p>Plus replacing Spotify on Linux, the new "
    /// + "Microsoft lock-in, our hosts face a moral quandary & more!</p>]]>";
    ///
    /// let title = "Making Music with Linux | LAS 408".to_string();
    ///
    /// let item_1 = ItemBuilder::new()
    ///     .title(Some(title))
    ///     .link(Some(link))
    ///     .description(None)
    ///     .author(None)
    ///     .categories(Vec::new())
    ///     .enclosure(None)
    ///     .guid(None)
    ///     .pub_date(None)
    ///     .source(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let item_2 = ItemBuilder::new()
    ///     .title(None)
    ///     .link(None)
    ///     .description(Some(description))
    ///     .author(None)
    ///     .categories(Vec::new())
    ///     .enclosure(None)
    ///     .guid(None)
    ///     .pub_date(None)
    ///     .source(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let items_vec = vec![item_1, item_2];
    ///
    /// let channels = ChannelBuilder::new()
    ///     .items(items_vec.clone())
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let items = channels.items();
    /// assert!(!items.is_empty());
    ///
    /// assert_eq!(items_vec.clone().len(), items.len());
    /// ```
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .items(Vec::new())
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.items().is_empty());
    /// ```
    pub fn items(&self) -> &[Item] {
        &self.items
    }


    /// Get the optional `ITunesChannelExtension` under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesOwnerBuilder, ITunesCategoryBuilder};
    ///
    /// let owner = ITunesOwnerBuilder::new()
    ///     .email(Some("email@example.com".to_string()))
    ///     .name(Some("name".to_string()))
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
    /// let itunes_channel = ITunesChannelExtensionBuilder::new()
    ///     .author(Some("author".to_string()))
    ///     .block(Some("block".to_string()))
    ///     .image(Some("image".to_string()))
    ///     .explicit(Some("explicit".to_string()))
    ///     .subtitle(Some("subtitle".to_string()))
    ///     .summary(Some("summary".to_string()))
    ///     .keywords(Some("keywords".to_string()))
    ///     .new_feed_url(Some("new_feed_url".to_string()))
    ///     .complete(Some("complete".to_string()))
    ///     .owner(Some(owner))
    ///     .categories(categories)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let channel = ChannelBuilder::new()
    ///     .itunes_ext(Some(itunes_channel))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channel.itunes_ext().is_some());
    /// ```
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channel = ChannelBuilder::new()
    ///     .itunes_ext(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channel.itunes_ext().is_none());
    /// ```
    pub fn itunes_ext(&self) -> Option<&ITunesChannelExtension> {
        self.itunes_ext
            .as_ref()
    }

    /// Get the optional `DublinCoreExtension` under `Channel`.
    pub fn dublin_core_ext(&self) -> Option<&DublinCoreExtension> {
        self.dublin_core_ext
            .as_ref()
    }

    /// Get the `ExtensionMap` under `Channel`.
    pub fn extensions(&self) -> &ExtensionMap {
        &self.extensions
    }

    /// Get the namespaces under `Channel`.
    pub fn namespaces(&self) -> &HashMap<String, String> {
        &self.namespaces
    }
}

impl Channel {
    /// Construct a `Channel` from a url string.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate rss;
    ///
    /// use rss::Channel;
    ///
    /// fn main()
    /// {
    ///     let url = "https://feedpress.me/usererror.xml";
    ///
    ///     Channel::from_url(url).unwrap();
    /// }
    /// ```
    #[cfg(feature = "from_url")]
    pub fn from_url(url: &str) -> Result<Channel, Error> {
        use std::io::Read;

        let mut content = String::new();

        ::reqwest::get(url)?
            .read_to_string(&mut content)?;

        Ok(Channel::from_str(content.as_str())?)
    }

    /// Attempt to read the RSS channel from the speficied reader.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let reader: BufRead = ...;
    /// let channel = Channel::read_from(reader).unwrap();
    /// ```
    pub fn read_from<R: ::std::io::BufRead>(reader: R) -> Result<Channel, Error> {
        let mut reader = XmlReader::from_reader(reader).trim_text(true);
        let mut in_rss = false;
        let mut namespaces = HashMap::new();

        while let Some(e) = reader.next() {
            match e {
                Ok(Event::Start(element)) => {
                    match element.name() {
                        b"rss" if !in_rss => {
                            for attr in element.attributes()
                                               .with_checks(false) {
                                if let Ok(attr) = attr {
                                    let split = attr.0
                                                    .splitn(2,
                                                            |b| *b == b':')
                                                    .collect::<Vec<_>>();
                                    if split.len() != 2 {
                                        continue;
                                    }

                                    let ns = unsafe { split.get_unchecked(0) };
                                    if ns != b"xmlns" {
                                        continue;
                                    }

                                    let name = unsafe { split.get_unchecked(1) };
                                    if name == b"itunes" || name == b"dc" {
                                        continue;
                                    }

                                    let key = str::from_utf8(name)?
                                        .to_string();
                                    let value = str::from_utf8(attr.1)?
                                        .to_string();
                                    namespaces.insert(key,
                                                      value);
                                }
                            }

                            in_rss = true;
                        },
                        b"channel" if in_rss => {
                            let mut channel = Channel::from_xml(reader,
                                                                element)
                                .map(|v| v.0)?;
                            channel.namespaces = namespaces;
                            return Ok(channel);
                        },
                        _ => skip_element!(reader),
                    }
                },
                Ok(Event::End(_)) => in_rss = false,
                Err(err) => return Err(err.into()),
                _ => {},
            }
        }

        Err(Error::EOF)
    }

    /// Attempt to write the RSS channel as XML to the speficied writer.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let channel: Channel = ...;
    /// let writer: Write = ...;
    /// channel.write_to(writer).unwrap();
    /// ```
    pub fn write_to<W: ::std::io::Write>(&self,
                                         writer: W)
        -> Result<W, Error> {
        let mut writer = ::quick_xml::XmlWriter::new(writer);

        let element = Element::new(b"rss");

        writer.write(Event::Start({
                                      let mut element = element.clone();
                                      element.extend_attributes(::std::iter::once((b"version", b"2.0")));

                                      let mut itunes_ns = self.itunes_ext
                                                              .is_some();
                                      let mut dc_ns = self.dublin_core_ext
                                                          .is_some();

                                      if !itunes_ns || dc_ns {
                                          for item in &self.items {
                                              if !itunes_ns {
                                                  itunes_ns = item.itunes_ext()
                                                                  .is_some();
                                              }

                                              if !dc_ns {
                                                  dc_ns = item.dublin_core_ext()
                                                              .is_some();
                                              }

                                              if itunes_ns && dc_ns {
                                                  break;
                                              }
                                          }
                                      }

                                      if itunes_ns {
                                          element.extend_attributes(::std::iter::once((b"xmlns:itunes",
                                                                                       extension::itunes::NAMESPACE)));
                                      }

                                      if dc_ns {
                element.extend_attributes(::std::iter::once((b"xmlns:dc", extension::dublincore::NAMESPACE)));
            }

                                      element.extend_attributes(self.namespaces
                                                                    .iter()
                                                                    .map(|(name, url)| {
                                                                             (format!("xmlns:{}",
                                                                                      name),
                                                                              url)
                                                                         }));

                                      element
                                  }))?;

        self.to_xml(&mut writer)?;

        writer.write(Event::End(element))?;

        Ok(writer.into_inner())
    }

    /// Validate `Channel`
    ///
    /// ## Examples
    /// ```
    /// extern crate rss;
    ///
    /// use rss::Channel;
    ///
    /// fn main()
    /// {
    ///     let url = "https://feedpress.me/usererror.xml";
    ///
    ///     let channel = Channel::from_url(url).unwrap();
    ///     channel.validate().unwrap();
    /// }
    /// ```
    pub fn validate(&self) -> Result<Channel, Error> {
        let cloud = match self.cloud() {
            None => None,
            Some(val) => {
                Some(CloudBuilder::new()
                         .domain(val.domain())
                         .port(i64::from_str(val.port())?)
                         .path(val.path())
                         .register_procedure(val.register_procedure())
                         .protocol(val.protocol())
                         .validate()?
                         .finalize()?)
            },
        };

        let mut channel_cat: Vec<Category> = Vec::new();
        for cat in self.categories() {
            let domain = match cat.domain() {
                None => None,
                Some(val) => Some(val.to_string()),
            };

            channel_cat.push(CategoryBuilder::new()
                                 .name(cat.name())
                                 .domain(domain)
                                 .validate()?
                                 .finalize()?);
        }

        let mut skip_hours: Vec<i64> = Vec::new();
        for hour in self.skip_hours() {
            skip_hours.push(i64::from_str(hour.as_str())?);
        }

        let image = match self.image() {
            None => None,
            Some(val) => {
                let width = match val.width() {
                    None => None,
                    Some(wval) => Some(i64::from_str(wval)?),
                };

                let height = match val.height() {
                    None => None,
                    Some(hval) => Some(i64::from_str(hval)?),
                };

                let description = match val.description() {
                    None => None,
                    Some(dval) => Some(dval.to_string()),
                };

                Some(ImageBuilder::new()
                         .url(val.url())
                         .title(val.title())
                         .link(val.link())
                         .width(width)
                         .height(height)
                         .description(description)
                         .validate()?
                         .finalize()?)
            },
        };

        let text_input = match self.text_input() {
            None => None,
            Some(val) => {
                Some(TextInputBuilder::new()
                         .title(val.title())
                         .description(val.description())
                         .name(val.name())
                         .link(val.link())
                         .validate()?
                         .finalize()?)
            },
        };

        let mut items: Vec<Item> = Vec::new();
        for item in self.items() {
            let mut item_cat: Vec<Category> = Vec::new();
            for cat in item.categories() {
                let domain = match cat.domain() {
                    None => None,
                    Some(val) => Some(val.to_string()),
                };

                item_cat.push(CategoryBuilder::new()
                                  .name(cat.name())
                                  .domain(domain)
                                  .validate()?
                                  .finalize()?);
            }

            let enclosure = match item.enclosure() {
                None => None,
                Some(eval) => {
                    Some(EnclosureBuilder::new()
                             .url(eval.url())
                             .length(i64::from_str(eval.length())?)
                             .mime_type(eval.mime_type())
                             .validate()?
                             .finalize()?)
                },
            };

            let guid = match item.guid() {
                None => None,
                Some(gval) => {
                    Some(GuidBuilder::new()
                             .value(gval.value())
                             .is_permalink(Some(gval.is_permalink()))
                             .finalize()?)
                },
            };

            let source = match item.source() {
                None => None,
                Some(sval) => {
                    let title = match sval.title() {
                        None => None,
                        Some(tval) => Some(tval.to_string()),
                    };

                    Some(SourceBuilder::new()
                             .url(sval.url())
                             .title(title)
                             .validate()?
                             .finalize()?)
                },
            };

            let title = match item.title() {
                None => None,
                Some(val) => Some(val.to_string()),
            };

            let link = match item.link() {
                None => None,
                Some(val) => Some(val.to_string()),
            };

            let description = match item.description() {
                None => None,
                Some(val) => Some(val.to_string()),
            };

            let author = match item.author() {
                None => None,
                Some(val) => Some(val.to_string()),
            };

            let pub_date = match item.pub_date() {
                None => None,
                Some(val) => Some(val.to_string()),
            };

            let comments = match item.comments() {
                None => None,
                Some(val) => Some(val.to_string()),
            };

            items.push(ItemBuilder::new()
                           .title(title)
                           .link(link)
                           .description(description)
                           .author(author)
                           .pub_date(pub_date)
                           .comments(comments)
                           .categories(item_cat)
                           .enclosure(enclosure)
                           .guid(guid)
                           .source(source)
                           .validate()?
                           .finalize()?);
        }

        let ttl = match self.ttl() {
            None => None,
            Some(val) => Some(i64::from_str(val)?),
        };

        let language = match self.language() {
            None => None,
            Some(val) => Some(val.to_string()),
        };

        let copyright = match self.copyright() {
            None => None,
            Some(val) => Some(val.to_string()),
        };

        let managing_editor = match self.managing_editor() {
            None => None,
            Some(val) => Some(val.to_string()),
        };

        let webmaster = match self.webmaster() {
            None => None,
            Some(val) => Some(val.to_string()),
        };

        let pub_date = match self.pub_date() {
            None => None,
            Some(val) => Some(val.to_string()),
        };

        let last_build_date = match self.last_build_date() {
            None => None,
            Some(val) => Some(val.to_string()),
        };

        let generator = match self.generator() {
            None => None,
            Some(val) => Some(val.to_string()),
        };

        let docs = match self.docs() {
            None => None,
            Some(val) => Some(val.to_string()),
        };

        ChannelBuilder::new()
            .title(self.title())
            .link(self.link())
            .description(self.description())
            .language(language)
            .copyright(copyright)
            .managing_editor(managing_editor)
            .webmaster(webmaster)
            .pub_date(pub_date)
            .last_build_date(last_build_date)
            .generator(generator)
            .docs(docs)
            .rating(None)
            .ttl(ttl)
            .cloud(cloud)
            .categories(channel_cat)
            .image(image)
            .text_input(text_input)
            .skip_hours(skip_hours)
            .skip_days(self.skip_days()
                           .to_vec())
            .items(items)
            .validate()?
            .finalize()
    }
}

impl ToString for Channel {
    fn to_string(&self) -> String {
        let buf = self.write_to(Vec::new())
                      .unwrap_or(Vec::new());
        // this unwrap should be safe since the bytes written from the Channel are all valid utf8
        String::from_utf8(buf).unwrap()
    }
}

impl FromXml for Channel {
    fn from_xml<R: ::std::io::BufRead>(mut reader: XmlReader<R>,
                                       _: Element)
        -> Result<(Self, XmlReader<R>), Error> {
        let mut channel = Channel::default();

        while let Some(e) = reader.next() {
            match e {
                Ok(Event::Start(element)) => {
                    match element.name() {
                        b"category" => {
                            let (category, reader_) = Category::from_xml(reader,
                                                                         element)?;
                            reader = reader_;
                            channel.categories
                                   .push(category);
                        },
                        b"cloud" => {
                            let (cloud, reader_) = Cloud::from_xml(reader,
                                                                   element)?;
                            reader = reader_;
                            channel.cloud = Some(cloud);
                        },
                        b"image" => {
                            let (image, reader_) = Image::from_xml(reader,
                                                                   element)?;
                            reader = reader_;
                            channel.image = Some(image);
                        },
                        b"textInput" => {
                            let (text_input, reader_) = TextInput::from_xml(reader,
                                                                            element)?;
                            reader = reader_;
                            channel.text_input = Some(text_input);
                        },
                        b"item" => {
                            let (item, reader_) = Item::from_xml(reader,
                                                                 element)?;
                            reader = reader_;
                            channel.items
                                   .push(item);
                        },
                        b"title" => {
                            if let Some(content) = element_text!(reader) {
                                channel.title = content;
                            }
                        },
                        b"link" => {
                            if let Some(content) = element_text!(reader) {
                                channel.link = content;
                            }
                        },
                        b"description" => {
                            if let Some(content) = element_text!(reader) {
                                channel.description = content;
                            }
                        },
                        b"language" => channel.language = element_text!(reader),
                        b"copyright" => channel.copyright = element_text!(reader),
                        b"managingEditor" => {
                            channel.managing_editor = element_text!(reader);
                        },
                        b"webMaster" => channel.webmaster = element_text!(reader),
                        b"pubDate" => channel.pub_date = element_text!(reader),
                        b"lastBuildDate" => {
                            channel.last_build_date = element_text!(reader);
                        },
                        b"generator" => channel.generator = element_text!(reader),
                        b"docs" => channel.docs = element_text!(reader),
                        b"ttl" => channel.ttl = element_text!(reader),
                        b"skipHours" => {
                            while let Some(e) = reader.next() {
                                match e {
                                    Ok(Event::Start(element)) => {
                                        if element.name() == b"hour" {
                                            if let Some(content) = element_text!(reader) {
                                                channel.skip_hours
                                                       .push(content);
                                            }
                                        } else {
                                            skip_element!(reader);
                                        }
                                    },
                                    Ok(Event::End(_)) => {
                                        break;
                                    },
                                    Err(err) => return Err(err.into()),
                                    _ => {},
                                }
                            }
                        },
                        b"skipDays" => {
                            while let Some(e) = reader.next() {
                                match e {
                                    Ok(Event::Start(element)) => {
                                        if element.name() == b"day" {
                                            if let Some(content) = element_text!(reader) {
                                                channel.skip_days
                                                       .push(content);
                                            }
                                        } else {
                                            skip_element!(reader);
                                        }
                                    },
                                    Ok(Event::End(_)) => {
                                        break;
                                    },
                                    Err(err) => return Err(err.into()),
                                    _ => {},
                                }
                            }
                        },
                        _ => {
                            if let Some((ns, name)) = fromxml::extension_name(&element) {
                                parse_extension!(reader,
                                                 element,
                                                 ns,
                                                 name,
                                                 channel.extensions);
                            } else {
                                skip_element!(reader);
                            }
                        },
                    }
                },
                Ok(Event::End(_)) => {
                    if !channel.extensions
                               .is_empty() {
                        if let Some(map) = channel.extensions
                                                  .remove("itunes") {
                            channel.itunes_ext = Some(ITunesChannelExtension::from_map(map)?);
                        }

                        if let Some(map) = channel.extensions
                                                  .remove("dc") {
                            channel.dublin_core_ext = Some(DublinCoreExtension::from_map(map));
                        }
                    }

                    return Ok((channel, reader));
                },
                Err(err) => return Err(err.into()),
                _ => {},
            }
        }

        Err(Error::EOF)
    }
}

impl ToXml for Channel {
    fn to_xml<W: ::std::io::Write>(&self,
                                   writer: &mut XmlWriter<W>)
        -> Result<(), XmlError> {
        let element = Element::new(b"channel");

        writer.write(Event::Start(element.clone()))?;

        writer.write_text_element(b"title",
                                  &self.title)?;
        writer.write_text_element(b"link",
                                  &self.link)?;
        writer.write_text_element(b"description",
                                  &self.description)?;

        if let Some(language) = self.language
                                    .as_ref() {
            writer.write_text_element(b"language",
                                      language)?;
        }

        if let Some(copyright) = self.copyright
                                     .as_ref() {
            writer.write_text_element(b"copyright",
                                      copyright)?;
        }

        if let Some(managing_editor) =
            self.managing_editor
                .as_ref() {
            writer.write_text_element(b"managingEditor",
                                      managing_editor)?;
        }

        if let Some(webmaster) = self.webmaster
                                     .as_ref() {
            writer.write_text_element(b"webMaster",
                                      webmaster)?;
        }

        if let Some(pub_date) = self.pub_date
                                    .as_ref() {
            writer.write_text_element(b"pubDate",
                                      pub_date)?;
        }

        if let Some(last_build_date) =
            self.last_build_date
                .as_ref() {
            writer.write_text_element(b"lastBuildDate",
                                      last_build_date)?;
        }

        writer.write_objects(&self.categories)?;

        if let Some(generator) = self.generator
                                     .as_ref() {
            writer.write_text_element(b"generator",
                                      generator)?;
        }

        if let Some(docs) = self.docs
                                .as_ref() {
            writer.write_text_element(b"docs",
                                      docs)?;
        }

        if let Some(cloud) = self.cloud
                                 .as_ref() {
            writer.write_object(cloud)?;
        }

        if let Some(ttl) = self.ttl
                               .as_ref() {
            writer.write_text_element(b"ttl",
                                      ttl)?;
        }

        if let Some(image) = self.image
                                 .as_ref() {
            writer.write_object(image)?;
        }

        if let Some(text_input) = self.text_input
                                      .as_ref() {
            writer.write_object(text_input)?;
        }

        if !self.skip_hours
                .is_empty() {
            let element = Element::new(b"skipHours");
            writer.write(Event::Start(element.clone()))?;
            for hour in &self.skip_hours {
                writer.write_text_element(b"hour",
                                          hour)?;
            }
            writer.write(Event::End(element))?;
        }

        if !self.skip_days
                .is_empty() {
            let element = Element::new(b"skipDays");
            writer.write(Event::Start(element.clone()))?;
            for day in &self.skip_days {
                writer.write_text_element(b"day",
                                          day)?;
            }
            writer.write(Event::End(element))?;
        }

        writer.write_objects(&self.items)?;

        for map in self.extensions
                       .values() {
            for extensions in map.values() {
                for extension in extensions {
                    extension.to_xml(writer)?;
                }
            }
        }

        if let Some(ext) = self.itunes_ext
                               .as_ref() {
            ext.to_xml(writer)?;
        }

        if let Some(ext) = self.dublin_core_ext
                               .as_ref() {
            ext.to_xml(writer)?;
        }

        writer.write(Event::End(element))
    }
}

impl FromStr for Channel {
    type Err = Error;
    #[inline]
    /// Attempt to read the RSS channel from the speficied str.
    fn from_str(s: &str) -> Result<Channel, Error> {
        Channel::read_from(s.as_bytes())
    }
}

/// This `ChannelBuilder` struct creates the `Channel`.
#[derive(Debug, Clone, Default)]
pub struct ChannelBuilder {
    title: String,
    link: String,
    description: String,
    language: Option<String>,
    copyright: Option<String>,
    managing_editor: Option<String>,
    webmaster: Option<String>,
    pub_date: Option<String>,
    last_build_date: Option<String>,
    categories: Vec<Category>,
    generator: Option<String>,
    docs: Option<String>,
    cloud: Option<Cloud>,
    ttl: Option<i64>,
    image: Option<Image>,
    rating: Option<String>,
    text_input: Option<TextInput>,
    skip_hours: Vec<i64>,
    skip_days: Vec<String>,
    items: Vec<Item>,
    extensions: ExtensionMap,
    itunes_ext: Option<ITunesChannelExtension>,
    dublin_core_ext: Option<DublinCoreExtension>,
    namespaces: HashMap<String, String>,
}

impl ChannelBuilder {
    /// Construct a new `ChannelBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let channel_builder = ChannelBuilder::new();
    /// ```
    pub fn new() -> ChannelBuilder {
        ChannelBuilder::default()
    }


    /// Set the title that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.title("The Linux Action Show! OGG");
    /// ```
    pub fn title(mut self,
                 title: &str)
        -> ChannelBuilder {
        self.title = title.to_string();
        self
    }


    /// Set the link that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.link("http://www.jupiterbroadcasting.com");
    /// ```
    pub fn link(mut self,
                link: &str)
        -> ChannelBuilder {
        self.link = link.to_string();
        self
    }


    /// Set the description that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let description = "Ogg Vorbis audio versions of The Linux ".to_string()
    /// + "Action Show! A show that covers everything geeks care about in the "
    /// + "computer industry. Get a solid dose of Linux, gadgets, news events "
    /// + "and much more!";
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.description(description.as_ref());
    /// ```
    pub fn description(mut self,
                       description: &str)
        -> ChannelBuilder {
        self.description = description.to_string();
        self
    }


    /// Set the optional language that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.language(Some("en".to_string()));
    /// ```
    pub fn language(mut self,
                    language: Option<String>)
        -> ChannelBuilder {
        self.language = language;
        self
    }


    /// Set the optional copyright that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let copyright = "Copyright 2002, Spartanburg Herald-Journal".to_string();
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.copyright(Some(copyright));
    /// ```
    pub fn copyright(mut self,
                     copyright: Option<String>)
        -> ChannelBuilder {
        self.copyright = copyright;
        self
    }


    /// Set the optional managing editor that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let managing_editor =
    ///     "chris@jupiterbroadcasting.com (Chris Fisher)".to_string();
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.managing_editor(Some(managing_editor));
    /// ```
    pub fn managing_editor(mut self,
                           managing_editor: Option<String>)
        -> ChannelBuilder {
        self.managing_editor = managing_editor;
        self
    }


    /// Set the optional web master that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let webmaster =
    ///     "chris@jupiterbroadcasting.com (Chris Fisher)".to_string();
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.webmaster(Some(webmaster));
    /// ```
    pub fn webmaster(mut self,
                     webmaster: Option<String>)
        -> ChannelBuilder {
        self.webmaster = webmaster;
        self
    }


    /// Set the optional pub date that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.pub_date(Some("Sun, 13 Mar 2016 20:02:02
    /// -0700".to_string()));
    /// ```
    pub fn pub_date(mut self,
                    pub_date: Option<String>)
        -> ChannelBuilder {
        self.pub_date = pub_date;
        self
    }


    /// Set the optional last build date that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.last_build_date(Some("Sun, 13 Mar 2016 20:02:02
    /// -0700".to_string()));
    /// ```
    pub fn last_build_date(mut self,
                           last_build_date: Option<String>)
        -> ChannelBuilder {
        self.last_build_date = last_build_date;
        self
    }


    /// Set the optional categories that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, CategoryBuilder};
    ///
    /// let category = CategoryBuilder::new()
    ///     .finalize()
    ///     .unwrap();
    /// let categories = vec![category];
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.categories(categories);
    /// ```
    pub fn categories(mut self,
                      categories: Vec<Category>)
        -> ChannelBuilder {
        self.categories = categories;
        self
    }


    /// Set the optional generator that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let generator = "Feeder 2.5.12(2294); ".to_string()
    /// + "Mac OS X Version 10.9.5 (Build 13F34) "
    /// + "http://reinventedsoftware.com/feeder/";
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.generator(Some(generator));
    /// ```
    pub fn generator(mut self,
                     generator: Option<String>)
        -> ChannelBuilder {
        self.generator = generator;
        self
    }


    /// Set the optional docs that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.docs(Some("http://blogs.law.harvard.edu/tech/rss/".
    /// to_owned()));
    /// ```
    pub fn docs(mut self,
                docs: Option<String>)
        -> ChannelBuilder {
        self.docs = docs;
        self
    }


    /// Set the optional cloud that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, CloudBuilder};
    ///
    /// let cloud = CloudBuilder::new()
    ///     .domain("http://rpc.sys.com/")
    ///     .protocol("soap")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.cloud(Some(cloud));
    /// ```
    pub fn cloud(mut self,
                 cloud: Option<Cloud>)
        -> ChannelBuilder {
        self.cloud = cloud;
        self
    }


    /// Set the optional ttl that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.ttl(Some(60));
    /// ```
    pub fn ttl(mut self,
               ttl: Option<i64>)
        -> ChannelBuilder {
        self.ttl = ttl;
        self
    }


    /// Set the optional image that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, ImageBuilder};
    ///
    /// let url = "http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg";
    ///
    /// let link = "http://www.jupiterbroadcasting.com/";
    ///
    /// let image = ImageBuilder::new()
    ///     .url(url)
    ///     .link(link)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.image(Some(image));
    /// ```
    pub fn image(mut self,
                 image: Option<Image>)
        -> ChannelBuilder {
        self.image = image;
        self
    }

    /// Set the optional rating that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.rating(Some("PG-13".to_string()));
    /// ```
    pub fn rating(mut self,
                  rating: Option<String>)
        -> ChannelBuilder {
        self.rating = rating;
        self
    }


    /// Set the optional text input that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, TextInputBuilder};
    ///
    /// let text_input = TextInputBuilder::new()
    ///     .link("http://www.example.com/feedback")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.text_input(Some(text_input));
    /// ```
    pub fn text_input(mut self,
                      text_input: Option<TextInput>)
        -> ChannelBuilder {
        self.text_input = text_input;
        self
    }


    /// Set the optional skipdays that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let hours: Vec<i64> = vec![0, 12, 18];
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.skip_hours(hours);
    /// ```
    pub fn skip_hours(mut self,
                      skip_hours: Vec<i64>)
        -> ChannelBuilder {
        self.skip_hours = skip_hours;
        self
    }


    /// Set the optional skipdays that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let days = vec!["Monday".to_string(), "Tuesday".to_owned()];
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.skip_days(days);
    /// ```
    pub fn skip_days(mut self,
                     skip_days: Vec<String>)
        -> ChannelBuilder {
        self.skip_days = skip_days;
        self
    }


    /// Set the optional items that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, ItemBuilder};
    ///
    /// let title = "Making Music with Linux | LAS 408".to_string();
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some(title))
    ///     .finalize()
    ///     .unwrap();
    /// let items = vec![item];
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.items(items);
    /// ```
    pub fn items(mut self,
                 items: Vec<Item>)
        -> ChannelBuilder {
        self.items = items;
        self
    }


    /// Set the optional itunes channel extension that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder,
    /// ITunesOwnerBuilder, ITunesCategoryBuilder};
    ///
    /// let owner = ITunesOwnerBuilder::new()
    ///     .email(Some("email@example.com".to_string()))
    ///     .name(Some("name".to_string()))
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
    /// let itunes_channel = ITunesChannelExtensionBuilder::new()
    ///     .author(Some("author".to_string()))
    ///     .block(Some("block".to_string()))
    ///     .image(Some("image".to_string()))
    ///     .explicit(Some("explicit".to_string()))
    ///     .subtitle(Some("subtitle".to_string()))
    ///     .summary(Some("summary".to_string()))
    ///     .keywords(Some("keywords".to_string()))
    ///     .new_feed_url(Some("new_feed_url".to_string()))
    ///     .complete(Some("complete".to_string()))
    ///     .owner(Some(owner))
    ///     .categories(categories)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.itunes_ext(Some(itunes_channel));
    /// ```
    pub fn itunes_ext(mut self,
                      itunes_ext: Option<ITunesChannelExtension>)
        -> ChannelBuilder {
        self.itunes_ext = itunes_ext;
        self
    }

    /// Set the optional dublin_core_ext that exists under `Channel`.
    pub fn dublin_core_ext(mut self,
                           dublin_core_ext: Option<DublinCoreExtension>)
        -> ChannelBuilder {
        self.dublin_core_ext = dublin_core_ext;
        self
    }

    /// Set the extensions that exists under `Channel`.
    pub fn extensions(mut self,
                      extensions: ExtensionMap)
        -> ChannelBuilder {
        self.extensions = extensions;
        self
    }

    /// Set the namespaces that exists under `Channel`.
    pub fn namespaces(mut self,
                      namespaces: HashMap<String, String>)
        -> ChannelBuilder {
        self.namespaces = namespaces;
        self
    }


    /// Validate the contents of `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let description = "Ogg Vorbis audio versions of The Linux ".to_string()
    /// + "Action Show! A show that covers everything geeks care about in the "
    /// + "computer industry. Get a solid dose of Linux, gadgets, news events "
    /// + "and much more!";
    ///
    /// let channels = ChannelBuilder::new()
    ///         .title("The Linux Action Show! OGG")
    ///         .link("http://www.jupiterbroadcasting.com")
    ///         .description(description.as_ref())
    ///         .language(None)
    ///         .copyright(None)
    ///         .managing_editor(None)
    ///         .webmaster(None)
    ///         .pub_date(None)
    ///         .last_build_date(None)
    ///         .categories(Vec::new())
    ///         .generator(None)
    ///         .docs(None)
    ///         .cloud(None)
    ///         .ttl(None)
    ///         .image(None)
    ///         .rating(None)
    ///         .text_input(None)
    ///         .skip_hours(Vec::new())
    ///         .skip_days(Vec::new())
    ///         .items(Vec::new())
    ///         .validate().unwrap()
    ///         .finalize().unwrap();
    /// ```
    pub fn validate(self) -> Result<ChannelBuilder, Error> {
        Url::parse(self.link
                       .as_str())?;

        if let Some(ref pub_date) = self.pub_date {
            DateTime::parse_from_rfc2822(pub_date.as_str())?;
        }

        if let Some(ref last_build_date) = self.last_build_date {
            DateTime::parse_from_rfc2822(last_build_date.as_str())?;
        }

        if let Some(ref docs) = self.docs {
            Url::parse(docs.as_str())?;
        }

        for day in self.skip_days.as_slice() {
            match Day::from_str(day.as_str()) {
                Ok(_) => (),
                Err(err) => return Err(Error::Validation(err.to_string())),
            };
        }

        for hour in self.skip_hours.as_slice() {
            if *hour < 0 {
                return Err(Error::Validation("Channel Skip Hour cannot be a negative value.".to_string()));
            } else if *hour > 23 {
                return Err(Error::Validation("Channel Skip Hour cannot be greater than 23.".to_string()));
            }
        }

        if self.ttl
               .is_some() &&
           self.ttl
               .unwrap() < 0 {
            return Err(Error::Validation("Channel ttl cannot be a negative value.".to_string()));
        }

        Ok(self)
    }


    /// Construct the `Channel` from the `ChannelBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let description = "Ogg Vorbis audio versions of The Linux ".to_string()
    /// + "Action Show! A show that covers everything geeks care about in the "
    /// + "computer industry. Get a solid dose of Linux, gadgets, news events "
    /// + "and much more!";
    ///
    /// let channels = ChannelBuilder::new()
    ///         .title("The Linux Action Show! OGG")
    ///         .link("http://www.jupiterbroadcasting.com")
    ///         .description(description.as_ref())
    ///         .language(None)
    ///         .copyright(None)
    ///         .managing_editor(None)
    ///         .webmaster(None)
    ///         .pub_date(None)
    ///         .last_build_date(None)
    ///         .categories(Vec::new())
    ///         .generator(None)
    ///         .docs(None)
    ///         .cloud(None)
    ///         .ttl(None)
    ///         .image(None)
    ///         .rating(None)
    ///         .text_input(None)
    ///         .skip_hours(Vec::new())
    ///         .skip_days(Vec::new())
    ///         .items(Vec::new())
    ///         .finalize();
    /// ```
    pub fn finalize(self) -> Result<Channel, Error> {
        let mut skip_hours: Vec<String> = Vec::new();
        for hour in self.skip_hours.as_slice() {
            skip_hours.push(hour.to_string());
        }

        let ttl = match self.ttl {
            None => None,
            Some(val) => Some(val.to_string()),
        };

        Ok(Channel { title: self.title,
                     link: self.link,
                     description: self.description,
                     language: self.language,
                     copyright: self.copyright,
                     managing_editor: self.managing_editor,
                     webmaster: self.webmaster,
                     pub_date: self.pub_date,
                     last_build_date: self.last_build_date,
                     categories: self.categories,
                     generator: self.generator,
                     docs: self.docs,
                     cloud: self.cloud,
                     ttl: ttl,
                     image: self.image,
                     rating: self.rating,
                     text_input: self.text_input,
                     skip_hours: skip_hours,
                     skip_days: self.skip_days,
                     items: self.items,
                     itunes_ext: self.itunes_ext,
                     dublin_core_ext: self.dublin_core_ext,
                     extensions: self.extensions,
                     namespaces: self.namespaces, })
    }
}

/// Enumerations of protocols for `SkipDays`.
enum Day {
    /// Monday
    Monday,

    /// Tuesday
    Tuesday,

    /// Wednesday
    Wednesday,

    /// Thursday
    Thursday,

    /// Friday
    Friday,

    /// Saturday
    Saturday,

    /// Sunday
    Sunday,
}

impl FromStr for Day {
    type Err = &'static str;

    /// Convert `&str` to `Day`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Monday" => Ok(Day::Monday),
            "Tuesday" => Ok(Day::Tuesday),
            "Wednesday" => Ok(Day::Wednesday),
            "Thursday" => Ok(Day::Thursday),
            "Friday" => Ok(Day::Friday),
            "Saturday" => Ok(Day::Saturday),
            "Sunday" => Ok(Day::Sunday),
            _ => Err("not a valid value"),
        }
    }
}
