// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use category::{Category, CategoryBuilder};
use chrono::DateTime;
use cloud::{Cloud, CloudBuilder};
use curl::easy::Easy;
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
pub struct Channel
{
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

impl Channel
{
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
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(title.to_owned(), channels.title());
    /// ```
    pub fn title(&self) -> String
    {
        self.title
            .clone()
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
    /// assert_eq!(link.to_owned(), channels.link());
    /// ```
    pub fn link(&self) -> String
    {
        self.link
            .clone()
    }


    /// Get the description that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let description = "Ogg Vorbis audio versions of The Linux ".to_owned()
    /// + "Action Show! A show that covers everything geeks care about in the "
    /// + "computer industry. Get a solid dose of Linux, gadgets, news events "
    /// + "and much more!";
    ///
    /// let channels = ChannelBuilder::new()
    ///     .description(description.as_ref())
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(description.to_owned(), channels.description());
    /// ```
    pub fn description(&self) -> String
    {
        self.description
            .clone()
    }


    /// Get the optional language that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let language_string = "en".to_owned();
    ///
    /// let channels = ChannelBuilder::new()
    ///     .language(Some(language_string.clone()))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let language_option = channels.language();
    /// assert!(language_option.is_some());
    ///
    /// assert_eq!(language_string.clone(), language_option.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .language(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.language().is_none());
    /// ```
    pub fn language(&self) -> Option<String>
    {
        self.language
            .clone()
    }


    /// Get the optional copyright that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let copyright_string =
    ///     "Copyright 2002, Spartanburg Herald-Journal".to_owned();
    ///
    /// let channels = ChannelBuilder::new()
    ///     .copyright(Some(copyright_string.clone()))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let copyright_option = channels.copyright();
    /// assert!(copyright_option.is_some());
    ///
    /// assert_eq!(copyright_string.clone(), copyright_option.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .copyright(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.copyright().is_none());
    /// ```
    pub fn copyright(&self) -> Option<String>
    {
        self.copyright
            .clone()
    }


    /// Get the optional managing editor that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let managing_editor_string =
    ///     "chris@jupiterbroadcasting.com (Chris Fisher)".to_owned();
    ///
    /// let channels = ChannelBuilder::new()
    ///     .managing_editor(Some(managing_editor_string.clone()))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let managing_editor_option = channels.managing_editor();
    /// assert!(managing_editor_option.is_some());
    ///
    /// assert_eq!(managing_editor_string.clone(),
    /// managing_editor_option.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .managing_editor(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.managing_editor().is_none());
    /// ```
    pub fn managing_editor(&self) -> Option<String>
    {
        self.managing_editor
            .clone()
    }

    /// Get the optional web master that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let webmaster_string =
    ///     "chris@jupiterbroadcasting.com (Chris Fisher)".to_owned();
    ///
    /// let channels = ChannelBuilder::new()
    ///     .webmaster(Some(webmaster_string.clone()))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let webmaster_option = channels.webmaster();
    /// assert!(webmaster_option.is_some());
    ///
    /// assert_eq!(webmaster_string.clone(), webmaster_option.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .webmaster(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.webmaster().is_none());
    /// ```
    pub fn webmaster(&self) -> Option<String>
    {
        self.webmaster
            .clone()
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
    ///     .pub_date(Some(pub_date.to_owned()))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let local = channels.pub_date();
    /// assert!(local.is_some());
    ///
    /// assert_eq!(pub_date.to_owned(), local.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .pub_date(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.pub_date().is_none());
    /// ```
    pub fn pub_date(&self) -> Option<String>
    {
        self.pub_date
            .clone()
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
    ///     .last_build_date(Some(last_build_date.to_owned()))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let local = channels.last_build_date();
    /// assert!(local.is_some());
    ///
    /// assert_eq!(last_build_date.to_owned(), local.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .last_build_date(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.last_build_date().is_none());
    /// ```
    pub fn last_build_date(&self) -> Option<String>
    {
        self.last_build_date
            .clone()
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
    ///     .domain(Some("http://jupiterbroadcasting.com".to_owned()))
    ///     .name("Podcast")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let categories_vec = vec![category_1, category_2];
    ///
    /// let channels = ChannelBuilder::new()
    ///     .categories(categories_vec.clone())
    ///     .link("http://www.jupiterbroadcasting.com/")
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
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.categories().is_empty());
    /// ```
    pub fn categories(&self) -> Vec<Category>
    {
        self.categories
            .clone()
    }


    /// Get the optional generator that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let generator_string = "Feeder 2.5.12(2294); ".to_owned()
    /// + "Mac OS X Version 10.9.5 (Build 13F34) "
    /// + "http://reinventedsoftware.com/feeder/";
    ///
    /// let channels = ChannelBuilder::new()
    ///     .generator(Some(generator_string.clone()))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let generator_option = channels.generator();
    /// assert!(generator_option.is_some());
    ///
    /// assert_eq!(generator_string.clone(), generator_option.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .generator(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.generator().is_none());
    /// ```
    pub fn generator(&self) -> Option<String>
    {
        self.generator
            .clone()
    }


    /// Get the optional docs that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let docs_string = "http://blogs.law.harvard.edu/tech/rss/".to_owned();
    ///
    /// let channels = ChannelBuilder::new()
    ///     .docs(Some(docs_string.clone()))
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let docs_option = channels.docs();
    /// assert!(docs_option.is_some());
    ///
    /// assert_eq!(docs_string.clone(), docs_option.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .docs(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.docs().is_none());
    /// ```
    pub fn docs(&self) -> Option<String>
    {
        self.docs
            .clone()
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
    ///     .link("http://www.jupiterbroadcasting.com/")
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
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.cloud().is_none());
    /// ```
    pub fn cloud(&self) -> Option<Cloud>
    {
        self.cloud
            .clone()
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
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let ttl_option = channels.ttl();
    /// assert!(ttl_option.is_some());
    ///
    /// assert_eq!(ttl_num.to_string(), ttl_option.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channels = ChannelBuilder::new()
    ///     .ttl(None)
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(channels.ttl().is_none());
    /// ```
    pub fn ttl(&self) -> Option<String>
    {
        self.ttl
            .clone()
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
    pub fn image(&self) -> Option<Image>
    {
        self.image
            .clone()
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
    pub fn rating(&self) -> Option<String>
    {
        None
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
    pub fn text_input(&self) -> Option<TextInput>
    {
        self.text_input
            .clone()
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
    pub fn skip_hours(&self) -> Vec<String>
    {
        self.skip_hours
            .clone()
    }


    /// Get the skip days that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let skip_days_vec: Vec<String> = vec!["Monday".to_owned(),
    /// "Sunday".to_owned(), "Thursday".to_owned(),
    ///     "Wednesday".to_owned()];
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
    pub fn skip_days(&self) -> Vec<String>
    {
        self.skip_days
            .clone()
    }


    /// Get the items that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel, ItemBuilder};
    ///
    /// let link = "http://www.jupiterbroadcasting.com/97561/".to_owned()
    /// + "making-music-with-linux-las-408/";
    ///
    /// let description = "<![CDATA[<p>In special Rasberry Pi 3 ".to_owned()
    /// + "edition of the show we look at the new hardware, review & chat with "
    /// + "Mycroft CTO Ryan Sipes on how important the Raspberry Pi is for "
    /// + "development of their open artificial intelligence platform & get "
    /// + "the latest news.</p><p>Plus replacing Spotify on Linux, the new "
    /// + "Microsoft lock-in, our hosts face a moral quandary & more!</p>]]>";
    ///
    /// let title = "Making Music with Linux | LAS 408".to_owned();
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
    pub fn items(&self) -> Vec<Item>
    {
        self.items
            .clone()
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
    /// let itunes_channel = ITunesChannelExtensionBuilder::new()
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
    pub fn itunes_ext(&self) -> Option<ITunesChannelExtension>
    {
        self.itunes_ext
            .clone()
    }

    /// Get the optional `DublinCoreExtension` under `Channel`.
    pub fn dublin_core_ext(&self) -> Option<DublinCoreExtension>
    {
        self.dublin_core_ext
            .clone()
    }

    /// Get the `ExtensionMap` under `Channel`.
    pub fn extensions(&self) -> ExtensionMap
    {
        self.extensions
            .clone()
    }

    /// Get the namespaces under `Channel`.
    pub fn namespaces(&self) -> HashMap<String, String>
    {
        self.namespaces
            .clone()
    }
}

impl Channel
{
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
    pub fn from_url(url: &str) -> Result<Channel, Error>
    {
        let feed_url = Url::parse(url)?;
        let mut xml = Vec::new();
        let mut handle = Easy::new();

        handle.url(feed_url.into_string()
                           .as_str())?;
        {
            let mut transfer = handle.transfer();
            transfer.write_function(|data| {
                                        xml.extend_from_slice(data);
                                        Ok(data.len())
                                    })
                    .unwrap();
            transfer.perform()
                    .unwrap();
        }

        let content_type = match handle.content_type()? {
            Some(val) => val,
            None => return Err(Error::FromUrl(String::from("Unable to unwrap() content_type"))),
        };

        if !content_type.contains("xml") {
            return Err(Error::FromUrl(String::from("Url must end with .xml")));
        }

        Ok(Channel::from_str(String::from_utf8(xml)?
                                 .as_str())?)
    }

    /// Attempt to read the RSS channel from the speficied reader.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let reader: BufRead = ...;
    /// let channel = Channel::read_from(reader).unwrap();
    /// ```
    pub fn read_from<R: ::std::io::BufRead>(reader: R) -> Result<Channel, Error>
    {
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
        -> Result<W, Error>
    {
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
    pub fn validate(&self) -> Result<Channel, Error>
    {
        let cloud = match self.cloud() {
            None => None,
            Some(val) => {
                Some(CloudBuilder::new()
                         .domain(val.domain()
                                    .as_str())
                         .port(i64::from_str(val.port()
                                                .as_str())?)
                         .path(val.path()
                                  .as_str())
                         .register_procedure(val.register_procedure()
                                                .as_str())
                         .protocol(val.protocol()
                                      .as_str())
                         .validate()?
                         .finalize()?)
            },
        };

        let mut channel_cat: Vec<Category> = Vec::new();
        for cat in self.categories() {
            channel_cat.push(CategoryBuilder::new()
                                 .name(cat.name()
                                          .as_str())
                                 .domain(cat.domain())
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
                    Some(wval) => Some(i64::from_str(wval.as_str())?),
                };

                let height = match val.height() {
                    None => None,
                    Some(hval) => Some(i64::from_str(hval.as_str())?),
                };

                Some(ImageBuilder::new()
                         .url(val.url()
                                 .as_str())
                         .title(val.title()
                                   .as_str())
                         .link(val.link()
                                  .as_str())
                         .width(width)
                         .height(height)
                         .description(val.description())
                         .validate()?
                         .finalize()?)
            },
        };

        let text_input = match self.text_input() {
            None => None,
            Some(val) => {
                Some(TextInputBuilder::new()
                         .title(val.title()
                                   .as_str())
                         .description(val.description()
                                         .as_str())
                         .name(val.name()
                                  .as_str())
                         .link(val.link()
                                  .as_str())
                         .validate()?
                         .finalize()?)
            },
        };

        let mut items: Vec<Item> = Vec::new();
        for item in self.items() {
            let mut item_cat: Vec<Category> = Vec::new();
            for cat in item.categories() {
                item_cat.push(CategoryBuilder::new()
                                  .name(cat.name()
                                           .as_str())
                                  .domain(cat.domain())
                                  .validate()?
                                  .finalize()?);
            }

            let enclosure = match item.enclosure() {
                None => None,
                Some(eval) => {
                    Some(EnclosureBuilder::new()
                             .url(eval.url()
                                      .as_str())
                             .length(i64::from_str(eval.length()
                                                       .as_str())?)
                             .mime_type(eval.mime_type()
                                            .as_str())
                             .validate()?
                             .finalize()?)
                },
            };

            let guid = match item.guid() {
                None => None,
                Some(gval) => {
                    Some(GuidBuilder::new()
                             .value(gval.value()
                                        .as_str())
                             .is_permalink(Some(gval.is_permalink()))
                             .finalize()?)
                },
            };

            let source = match item.source() {
                None => None,
                Some(sval) => {
                    Some(SourceBuilder::new()
                             .url(sval.url()
                                      .as_str())
                             .title(sval.title())
                             .validate()?
                             .finalize()?)
                },
            };

            items.push(ItemBuilder::new()
                           .title(item.title())
                           .link(item.link())
                           .description(item.description())
                           .author(item.author())
                           .pub_date(item.pub_date())
                           .comments(item.comments())
                           .categories(item_cat)
                           .enclosure(enclosure)
                           .guid(guid)
                           .source(source)
                           .validate()?
                           .finalize()?);
        }

        let ttl = match self.ttl() {
            None => None,
            Some(val) => Some(i64::from_str(val.as_str())?),
        };

        ChannelBuilder::new()
            .title(self.title()
                       .as_str())
            .link(self.link()
                      .as_str())
            .description(self.description()
                             .as_str())
            .language(self.language())
            .copyright(self.copyright())
            .managing_editor(self.managing_editor())
            .webmaster(self.webmaster())
            .pub_date(self.pub_date())
            .last_build_date(self.last_build_date())
            .generator(self.generator())
            .docs(self.docs())
            .rating(None)
            .ttl(ttl)
            .cloud(cloud)
            .categories(channel_cat)
            .image(image)
            .text_input(text_input)
            .skip_hours(skip_hours)
            .skip_days(self.skip_days())
            .items(items)
            .validate()?
            .finalize()
    }
}

impl ToString for Channel
{
    fn to_string(&self) -> String
    {
        let buf = self.write_to(Vec::new())
                      .unwrap_or(Vec::new());
        // this unwrap should be safe since the bytes written from the Channel are all valid utf8
        String::from_utf8(buf).unwrap()
    }
}

impl FromXml for Channel
{
    fn from_xml<R: ::std::io::BufRead>(mut reader: XmlReader<R>,
                                       _: Element)
        -> Result<(Self, XmlReader<R>), Error>
    {
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

impl ToXml for Channel
{
    fn to_xml<W: ::std::io::Write>(&self,
                                   writer: &mut XmlWriter<W>)
        -> Result<(), XmlError>
    {
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

impl FromStr for Channel
{
    type Err = Error;
    #[inline]
    /// Attempt to read the RSS channel from the speficied str.
    fn from_str(s: &str) -> Result<Channel, Error>
    {
        Channel::read_from(s.as_bytes())
    }
}

/// This `ChannelBuilder` struct creates the `Channel`.
#[derive(Debug, Clone, Default)]
pub struct ChannelBuilder
{
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

impl ChannelBuilder
{
    /// Construct a new `ChannelBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let channel_builder = ChannelBuilder::new();
    /// ```
    pub fn new() -> ChannelBuilder
    {
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
    pub fn title(&mut self,
                 title: &str)
        -> &mut ChannelBuilder
    {
        self.title = title.to_owned();
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
    pub fn link(&mut self,
                link: &str)
        -> &mut ChannelBuilder
    {
        self.link = link.to_owned();
        self
    }


    /// Set the description that exists under `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let description = "Ogg Vorbis audio versions of The Linux ".to_owned()
    /// + "Action Show! A show that covers everything geeks care about in the "
    /// + "computer industry. Get a solid dose of Linux, gadgets, news events "
    /// + "and much more!";
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.description(description.as_ref());
    /// ```
    pub fn description(&mut self,
                       description: &str)
        -> &mut ChannelBuilder
    {
        self.description = description.to_owned();
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
    /// channel_builder.language(Some("en".to_owned()));
    /// ```
    pub fn language(&mut self,
                    language: Option<String>)
        -> &mut ChannelBuilder
    {
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
    /// let copyright = "Copyright 2002, Spartanburg Herald-Journal".to_owned();
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.copyright(Some(copyright));
    /// ```
    pub fn copyright(&mut self,
                     copyright: Option<String>)
        -> &mut ChannelBuilder
    {
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
    ///     "chris@jupiterbroadcasting.com (Chris Fisher)".to_owned();
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.managing_editor(Some(managing_editor));
    /// ```
    pub fn managing_editor(&mut self,
                           managing_editor: Option<String>)
        -> &mut ChannelBuilder
    {
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
    ///     "chris@jupiterbroadcasting.com (Chris Fisher)".to_owned();
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.webmaster(Some(webmaster));
    /// ```
    pub fn webmaster(&mut self,
                     webmaster: Option<String>)
        -> &mut ChannelBuilder
    {
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
    /// -0700".to_owned()));
    /// ```
    pub fn pub_date(&mut self,
                    pub_date: Option<String>)
        -> &mut ChannelBuilder
    {
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
    /// -0700".to_owned()));
    /// ```
    pub fn last_build_date(&mut self,
                           last_build_date: Option<String>)
        -> &mut ChannelBuilder
    {
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
    pub fn categories(&mut self,
                      categories: Vec<Category>)
        -> &mut ChannelBuilder
    {
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
    /// let generator = "Feeder 2.5.12(2294); ".to_owned()
    /// + "Mac OS X Version 10.9.5 (Build 13F34) "
    /// + "http://reinventedsoftware.com/feeder/";
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.generator(Some(generator));
    /// ```
    pub fn generator(&mut self,
                     generator: Option<String>)
        -> &mut ChannelBuilder
    {
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
    pub fn docs(&mut self,
                docs: Option<String>)
        -> &mut ChannelBuilder
    {
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
    pub fn cloud(&mut self,
                 cloud: Option<Cloud>)
        -> &mut ChannelBuilder
    {
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
    pub fn ttl(&mut self,
               ttl: Option<i64>)
        -> &mut ChannelBuilder
    {
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
    pub fn image(&mut self,
                 image: Option<Image>)
        -> &mut ChannelBuilder
    {
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
    /// channel_builder.rating(Some("PG-13".to_owned()));
    /// ```
    pub fn rating(&mut self,
                  rating: Option<String>)
        -> &mut ChannelBuilder
    {
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
    pub fn text_input(&mut self,
                      text_input: Option<TextInput>)
        -> &mut ChannelBuilder
    {
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
    pub fn skip_hours(&mut self,
                      skip_hours: Vec<i64>)
        -> &mut ChannelBuilder
    {
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
    /// let days = vec!["Monday".to_owned(), "Tuesday".to_owned()];
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.skip_days(days);
    /// ```
    pub fn skip_days(&mut self,
                     skip_days: Vec<String>)
        -> &mut ChannelBuilder
    {
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
    /// let title = "Making Music with Linux | LAS 408".to_owned();
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
    pub fn items(&mut self,
                 items: Vec<Item>)
        -> &mut ChannelBuilder
    {
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
    /// let itunes_channel = ITunesChannelExtensionBuilder::new()
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
    ///
    /// let mut channel_builder = ChannelBuilder::new();
    /// channel_builder.itunes_ext(Some(itunes_channel));
    /// ```
    pub fn itunes_ext(&mut self,
                      itunes_ext: Option<ITunesChannelExtension>)
        -> &mut ChannelBuilder
    {
        self.itunes_ext = itunes_ext;
        self
    }

    /// Set the optional dublin_core_ext that exists under `Channel`.
    pub fn dublin_core_ext(&mut self,
                           dublin_core_ext: Option<DublinCoreExtension>)
        -> &mut ChannelBuilder
    {
        self.dublin_core_ext = dublin_core_ext;
        self
    }

    /// Set the extensions that exists under `Channel`.
    pub fn extensions(&mut self,
                      extensions: ExtensionMap)
        -> &mut ChannelBuilder
    {
        self.extensions = extensions;
        self
    }

    /// Set the onamespaces that exists under `Channel`.
    pub fn namespaces(&mut self,
                      namespaces: HashMap<String, String>)
        -> &mut ChannelBuilder
    {
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
    /// let description = "Ogg Vorbis audio versions of The Linux ".to_owned()
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
    pub fn validate(&mut self) -> Result<&mut ChannelBuilder, Error>
    {
        Url::parse(self.link
                       .as_str())?;

        let pub_date = self.pub_date
                           .clone();
        if pub_date.is_some() {
            DateTime::parse_from_rfc2822(pub_date.unwrap()
                                                 .as_str())?;
        }

        let last_build_date = self.last_build_date
                                  .clone();
        if last_build_date.is_some() {
            DateTime::parse_from_rfc2822(last_build_date.unwrap()
                                                        .as_str())?;
        }

        let docs = self.docs
                       .clone();
        if docs.is_some() {
            Url::parse(docs.unwrap()
                           .as_str())?;
        }

        let mut skip_days = self.skip_days
                                .clone();
        skip_days.sort();
        skip_days.dedup();

        for day in skip_days {
            Day::value_of(day.as_str())?;
        }

        let mut skip_hours = self.skip_hours
                                 .clone();
        skip_hours.sort();
        skip_hours.dedup();

        for hour in skip_hours {
            if hour < 0 {
                return Err(Error::Validation(String::from("Channel Skip Hour cannot be a negative value.")));
            } else if hour > 23 {
                return Err(Error::Validation(String::from("Channel Skip Hour cannot be greater than 23.")));
            }
        }

        if self.ttl
               .is_some() &&
           self.ttl
               .unwrap() < 0 {
            return Err(Error::Validation(String::from("Channel ttl cannot be a negative value.")));
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
    /// let description = "Ogg Vorbis audio versions of The Linux ".to_owned()
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
    pub fn finalize(&self) -> Result<Channel, Error>
    {
        let mut skip_hours: Vec<String> = Vec::new();
        for hour in self.skip_hours
                        .clone() {
            skip_hours.push(hour.to_string());
        }

        let ttl = match self.ttl {
            None => None,
            Some(val) => Some(val.to_string()),
        };

        Ok(Channel { title: self.title
                                .clone(),
                     link: self.link
                               .clone(),
                     description: self.description
                                      .clone(),
                     language: self.language
                                   .clone(),
                     copyright: self.copyright
                                    .clone(),
                     managing_editor: self.managing_editor
                                          .clone(),
                     webmaster: self.webmaster
                                    .clone(),
                     pub_date: self.pub_date
                                   .clone(),
                     last_build_date: self.last_build_date
                                          .clone(),
                     categories: self.categories
                                     .clone(),
                     generator: self.generator
                                    .clone(),
                     docs: self.docs
                               .clone(),
                     cloud: self.cloud
                                .clone(),
                     ttl: ttl,
                     image: self.image
                                .clone(),
                     text_input: self.text_input
                                     .clone(),
                     skip_hours: skip_hours,
                     skip_days: self.skip_days
                                    .clone(),
                     items: self.items
                                .clone(),
                     itunes_ext: self.itunes_ext
                                     .clone(),
                     dublin_core_ext: self.dublin_core_ext
                                          .clone(),
                     extensions: self.extensions
                                     .clone(),
                     namespaces: self.namespaces
                                     .clone(), })
    }
}

/// Enumerations of protocols for `SkipDays`.
enum Day
{
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

impl Day
{
    /// Convert `&str` to `Day`.
    pub fn value_of(s: &str) -> Result<Day, Error>
    {
        match s {
            "Monday" => Ok(Day::Monday),
            "Tuesday" => Ok(Day::Tuesday),
            "Wednesday" => Ok(Day::Wednesday),
            "Thursday" => Ok(Day::Thursday),
            "Friday" => Ok(Day::Friday),
            "Saturday" => Ok(Day::Saturday),
            "Sunday" => Ok(Day::Sunday),
            _ => {
                Err(Error::Validation(String::from(format!("Invalid value: {}",
                                                           s))))
            },
        }
    }
}
