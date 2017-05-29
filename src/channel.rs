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
use fromxml::{self, FromXml, parse_extension, element_text};
use guid::GuidBuilder;
use image::{Image, ImageBuilder};
use item::{Item, ItemBuilder};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use quick_xml::events::{Event, BytesStart, BytesEnd};
use quick_xml::events::attributes::Attributes;
use quick_xml::errors::Error as XmlError;
use source::SourceBuilder;
use std::collections::HashMap;
use std::i64;
use std::str::{self, FromStr};
use textinput::{TextInput, TextInputBuilder};
use toxml::{ToXml, WriterExt};
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
    /// Return the title of this `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let title = "The Linux Action Show! OGG".to_string();
    ///
    /// let channel = ChannelBuilder::default()
    ///     .title(title.as_str())
    ///     .finalize();
    ///
    /// assert_eq!(title, channel.title());
    /// ```
    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    /// Return the web site URL for this `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let link = "http://www.jupiterbroadcasting.com/".to_string();
    ///
    /// let channel = ChannelBuilder::default()
    ///     .link(link.as_str())
    ///     .finalize();
    ///
    /// assert_eq!(link, channel.link());
    /// ```
    pub fn link(&self) -> &str {
        self.link.as_str()
    }

    /// Return the description of this `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let description = "Ogg Vorbis audio versions of The Linux \
    ///     Action Show! A show that covers everything geeks care about in the \
    ///     computer industry. Get a solid dose of Linux, gadgets, news events \
    ///     and much more!".to_string();
    ///
    /// let channel = ChannelBuilder::default()
    ///     .description(description.as_str())
    ///     .finalize();
    ///
    /// assert_eq!(description, channel.description());
    /// ```
    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    /// Return the langauge of this `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let language_string = "en-US";
    ///
    /// let channel = ChannelBuilder::default()
    ///     .language(language_string.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(language_string), channel.language());
    /// ```
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::default()
    ///     .language(None)
    ///     .finalize();
    ///
    /// assert!(channel.language().is_none());
    /// ```
    pub fn language(&self) -> Option<&str> {
        self.language.as_ref().map(|s| s.as_str())
    }

    /// Return the copyright notice for the content of this `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let copyright = "Copyright 2002, Spartanburg Herald-Journal";
    ///
    /// let channel = ChannelBuilder::default()
    ///     .copyright(copyright.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(copyright), channel.copyright());
    /// ```
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::default()
    ///     .copyright(None)
    ///     .finalize();
    ///
    /// assert!(channel.copyright().is_none());
    /// ```
    pub fn copyright(&self) -> Option<&str> {
        self.copyright.as_ref().map(|s| s.as_str())
    }

    /// Return the email address for the managing editor of this `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let managing_editor = "chris@jupiterbroadcasting.com (Chris Fisher)";
    ///
    /// let channel = ChannelBuilder::default()
    ///     .managing_editor(managing_editor.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(managing_editor), channel.managing_editor());
    /// ```
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::default()
    ///     .managing_editor(None)
    ///     .finalize();
    ///
    /// assert!(channel.managing_editor().is_none());
    /// ```
    pub fn managing_editor(&self) -> Option<&str> {
        self.managing_editor.as_ref().map(|s| s.as_str())
    }

    /// Return the email address for webmaster of this `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let webmaster = "chris@jupiterbroadcasting.com (Chris Fisher)";
    ///
    /// let channel = ChannelBuilder::default()
    ///     .webmaster(webmaster.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(webmaster), channel.webmaster());
    /// ```
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::default()
    ///     .webmaster(None)
    ///     .finalize();
    ///
    /// assert!(channel.webmaster().is_none());
    /// ```
    pub fn webmaster(&self) -> Option<&str> {
        self.webmaster.as_ref().map(|s| s.as_str())
    }

    /// Return the publication date for the content of this `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let pub_date = "Sun, 13 Mar 2016 20:02:02 -0700";
    ///
    /// let channel = ChannelBuilder::default()
    ///     .pub_date(pub_date.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(pub_date), channel.pub_date());
    /// ```
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::default()
    ///     .pub_date(None)
    ///     .finalize();
    ///
    /// assert!(channel.pub_date().is_none());
    /// ```
    pub fn pub_date(&self) -> Option<&str> {
        self.pub_date.as_ref().map(|s| s.as_str())
    }

    /// Return the time that the content of this `Channel` was last changed.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let last_build_date = "Sun, 13 Mar 2016 20:02:02 -0700";
    ///
    /// let channel = ChannelBuilder::default()
    ///     .last_build_date(last_build_date.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(last_build_date), channel.last_build_date());
    /// ```
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::default()
    ///     .last_build_date(None)
    ///     .finalize();
    ///
    /// assert!(channel.last_build_date().is_none());
    /// ```
    pub fn last_build_date(&self) -> Option<&str> {
        self.last_build_date.as_ref().map(|s| s.as_str())
    }

    /// Return the categories that this `Channel` belongs to.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, CategoryBuilder};
    ///
    /// let category = CategoryBuilder::default()
    ///     .name("Podcast")
    ///     .finalize();
    ///
    /// let categories = vec![category];
    ///
    /// let channel = ChannelBuilder::default()
    ///     .categories(categories.clone())
    ///     .finalize();
    ///
    /// assert_eq!(categories.as_slice(), channel.categories());
    /// ```
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::default()
    ///     .categories(Vec::new())
    ///     .finalize();
    ///
    /// assert!(channel.categories().is_empty());
    /// ```
    pub fn categories(&self) -> &[Category] {
        &self.categories
    }

    /// Return the name of the program used to generate the contents of this `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let generator = "Feeder 2.5.12(2294); Mac OS X Version 10.9.5 (Build 13F34) \
    ///     http://reinventedsoftware.com/feeder/";
    ///
    /// let channel = ChannelBuilder::default()
    ///     .generator(generator.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(generator), channel.generator());
    /// ```
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::default()
    ///     .generator(None)
    ///     .finalize();
    ///
    /// assert!(channel.generator().is_none());
    /// ```
    pub fn generator(&self) -> Option<&str> {
        self.generator.as_ref().map(|s| s.as_str())
    }

    /// Return a URL that points to the documentation of the RSS format used in this `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let docs = "http://blogs.law.harvard.edu/tech/rss/";
    ///
    /// let channel = ChannelBuilder::default()
    ///     .docs(docs.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(docs), channel.docs());
    /// ```
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::default()
    ///     .docs(None)
    ///     .finalize();
    ///
    /// assert!(channel.docs().is_none());
    /// ```
    pub fn docs(&self) -> Option<&str> {
        self.docs.as_ref().map(|s| s.as_str())
    }

    /// Return the information used to register with a cloud for notifications of updates to the
    /// `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, CloudBuilder};
    ///
    /// let cloud = CloudBuilder::default()
    ///     .domain("http://rpc.sys.com/")
    ///     .port(80)
    ///     .path("/RPC2")
    ///     .register_procedure("pingMe")
    ///     .protocol("soap")
    ///     .finalize();
    ///
    /// let channel = ChannelBuilder::default()
    ///     .cloud(cloud)
    ///     .finalize();
    ///
    /// assert!(channel.cloud().is_some());
    /// ```
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::default()
    ///     .cloud(None)
    ///     .finalize();
    ///
    /// assert!(channel.cloud().is_none());
    /// ```
    pub fn cloud(&self) -> Option<&Cloud> {
        self.cloud.as_ref()
    }

    /// Return the time to live of this `Channel`. This indicates the number of minutes the
    /// `Channel` can be cached before needing to be refreshed.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let ttl = 60;
    ///
    /// let channel = ChannelBuilder::default()
    ///     .ttl(ttl)
    ///     .finalize();
    ///
    /// assert_eq!(Some(ttl.to_string().as_str()), channel.ttl());
    /// ```
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channel = ChannelBuilder::default()
    ///     .ttl(None)
    ///     .finalize();
    ///
    /// assert!(channel.ttl().is_none());
    /// ```
    pub fn ttl(&self) -> Option<&str> {
        self.ttl.as_ref().map(|s| s.as_str())
    }

    /// Return the image to be displayed with this `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, ImageBuilder};
    ///
    /// let image = ImageBuilder::default()
    ///     .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg")
    ///     .finalize();
    ///
    /// let channel = ChannelBuilder::default()
    ///     .image(image)
    ///     .finalize();
    ///
    /// assert!(channel.image().is_some());
    /// ```
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::default()
    ///     .image(None)
    ///     .finalize();
    ///
    /// assert!(channel.image().is_none());
    /// ```
    pub fn image(&self) -> Option<&Image> {
        self.image.as_ref()
    }

    /// Return the [PICS](https://www.w3.org/PICS/) rating for this `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::default()
    ///     .rating(None)
    ///     .finalize();
    ///
    /// assert!(channel.rating().is_none());
    /// ```
    pub fn rating(&self) -> Option<&str> {
        self.rating.as_ref().map(|s| s.as_str())
    }

    /// Return the information for a text box to be displayed with this `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, TextInputBuilder};
    ///
    /// let text_input = TextInputBuilder::default()
    ///     .title("Enter Comment")
    ///     .description("Provided Feedback")
    ///     .name("Comment")
    ///     .link("http://www.example.com/feedback")
    ///     .finalize();
    ///
    /// let channel = ChannelBuilder::default()
    ///     .text_input(text_input)
    ///     .finalize();
    ///
    /// assert!(channel.text_input().is_some());
    /// ```
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::default()
    ///     .text_input(None)
    ///     .finalize();
    ///
    /// assert!(channel.text_input().is_none());
    /// ```
    pub fn text_input(&self) -> Option<&TextInput> {
        self.text_input.as_ref()
    }

    /// Return the hours that aggregators can skip for refreshing content.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let skip_hours = vec![6, 7, 8, 14, 22];
    ///
    /// let channel = ChannelBuilder::default()
    ///     .skip_hours(skip_hours.clone())
    ///     .finalize();
    ///
    /// let skip_hours_str = skip_hours.iter().map(|n| n.to_string()).collect::<Vec<_>>();
    /// assert_eq!(skip_hours_str.as_slice(), channel.skip_hours());
    /// ```
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::default()
    ///     .skip_hours(Vec::new())
    ///     .finalize();
    ///
    /// assert!(channel.skip_hours().is_empty());
    /// ```
    pub fn skip_hours(&self) -> &[String] {
        &self.skip_hours
    }

    /// Return the days that aggregators can skip for refreshing content.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let skip_days = vec!["Monday".to_string(), "Sunday".to_string()];
    ///
    /// let channel = ChannelBuilder::default()
    ///     .skip_days(skip_days.clone())
    ///     .finalize();
    ///
    /// assert_eq!(skip_days.as_slice(), channel.skip_days());
    /// ```
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channel = ChannelBuilder::default()
    ///     .skip_days(Vec::new())
    ///     .finalize();
    ///
    /// assert!(channel.skip_days().is_empty());
    /// ```
    pub fn skip_days(&self) -> &[String] {
        &self.skip_days
    }

    /// Return the `Item`s in this `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, ItemBuilder};
    ///
    /// let title = "Making Music with Linux | LAS 408".to_string();
    ///
    /// let item = ItemBuilder::default()
    ///     .title(title)
    ///     .finalize();
    ///
    /// let items = vec![item];
    ///
    /// let channel = ChannelBuilder::default()
    ///     .items(items.clone())
    ///     .finalize();
    ///
    /// assert_eq!(items.as_slice(), channel.items());
    /// ```
    ///
    /// ```
    /// use rss::{ChannelBuilder, Channel};
    ///
    /// let channel = ChannelBuilder::default()
    ///     .items(Vec::new())
    ///     .finalize();
    ///
    /// assert!(channel.items().is_empty());
    /// ```
    pub fn items(&self) -> &[Item] {
        &self.items
    }

    /// Return the `ITunesChannelExtension` for this `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder};
    ///
    /// let itunes_ext = ITunesChannelExtensionBuilder::default()
    ///     .author("author".to_string())
    ///     .finalize();
    ///
    /// let channel = ChannelBuilder::default()
    ///     .itunes_ext(itunes_ext)
    ///     .finalize();
    ///
    /// assert!(channel.itunes_ext().is_some());
    /// ```
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let channel = ChannelBuilder::default()
    ///     .itunes_ext(None)
    ///     .finalize();
    ///
    /// assert!(channel.itunes_ext().is_none());
    /// ```
    pub fn itunes_ext(&self) -> Option<&ITunesChannelExtension> {
        self.itunes_ext.as_ref()
    }

    /// Return the `DublinCoreExtension` for this `Channel`.
    pub fn dublin_core_ext(&self) -> Option<&DublinCoreExtension> {
        self.dublin_core_ext.as_ref()
    }

    /// Return the extensions for this `Channel`.
    pub fn extensions(&self) -> &ExtensionMap {
        &self.extensions
    }

    /// Return the namespaces for this `Channel`.
    pub fn namespaces(&self) -> &HashMap<String, String> {
        &self.namespaces
    }
}

impl Channel {
    /// Construct a `Channel` from a url string.
    ///
    /// Note: from_url can only be used by enabling the from_url feature in your
    /// Cargo.toml as follows:
    ///
    /// ```toml
    /// [dependencies]
    /// rss = { version = "*", features = ["from_url"] }
    /// ```
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
    ///     let channel = Channel::from_url(url).unwrap();
    /// }
    /// ```
    #[cfg(feature = "from_url")]
    pub fn from_url(url: &str) -> Result<Channel, Error> {
        use std::io::Read;

        let mut content = String::new();

        ::reqwest::get(url)?.read_to_string(&mut content)?;

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
        let mut reader = Reader::from_reader(reader);
        reader.trim_text(true).expand_empty_elements(true);
        let mut in_rss = false;
        let mut namespaces = HashMap::new();
        let mut buf = Vec::new();
        let mut skip_buf = Vec::new();

        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(element) => {
                    match element.name() {
                        b"rss" if !in_rss => {
                            for attr in element.attributes().with_checks(false) {
                                if let Ok(attr) = attr {

                                    if !attr.key.starts_with(b"xmlns:") ||
                                       attr.key == b"xmlns:itunes" ||
                                       attr.key == b"xmlns:dc" {
                                        continue;
                                    }

                                    let key = str::from_utf8(&attr.key[6..])?.to_string();
                                    let value = attr.unescape_and_decode_value(&reader)?;
                                    namespaces.insert(key, value);
                                }
                            }

                            in_rss = true;
                        }
                        b"channel" if in_rss => {
                            let mut channel = Channel::from_xml(&mut reader, element.attributes())?;
                            channel.namespaces = namespaces;
                            return Ok(channel);
                        }
                        name => reader.read_to_end(name, &mut skip_buf)?,
                    }
                }
                Event::End(_) => in_rss = false,
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
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
    pub fn write_to<W: ::std::io::Write>(&self, writer: W) -> Result<W, Error> {
        let mut writer = ::quick_xml::Writer::new(writer);

        let name = b"rss";
        let mut element = BytesStart::borrowed(name, name.len());
        element.push_attribute(("version", "2.0"));

        let mut itunes_ns = self.itunes_ext.is_some();
        let mut dc_ns = self.dublin_core_ext.is_some();

        if !itunes_ns || !dc_ns {
            for item in &self.items {
                if !itunes_ns {
                    itunes_ns = item.itunes_ext().is_some();
                }

                if !dc_ns {
                    dc_ns = item.dublin_core_ext().is_some();
                }

                if itunes_ns && dc_ns {
                    break;
                }
            }
        }

        if itunes_ns {
            element.push_attribute(("xmlns:itunes", extension::itunes::NAMESPACE));
        }

        if dc_ns {
            element.push_attribute(("xmlns:dc", extension::dublincore::NAMESPACE));
        }
        for (name, url) in &self.namespaces {
            element.push_attribute((format!("xmlns:{}", &**name).as_bytes(), url.as_bytes()));
        }

        writer.write_event(Event::Start(element))?;

        self.to_xml(&mut writer)?;

        writer.write_event(Event::End(BytesEnd::borrowed(name)))?;

        Ok(writer.into_inner())
    }

    /// Validate the contents of this `Channel`.
    ///
    /// ## Examples
    ///
    /// ```
    /// use rss::Channel;
    ///
    /// let input = include_str!("tests/data/rss2sample.xml");
    /// let channel = input.parse::<Channel>().unwrap();
    /// channel.validate().unwrap();
    /// ```
    pub fn validate(&self) -> Result<Channel, Error> {
        let cloud = match self.cloud() {
            None => None,
            Some(val) => {
                Some(CloudBuilder::default()
                         .domain(val.domain())
                         .port(i64::from_str(val.port())?)
                         .path(val.path())
                         .register_procedure(val.register_procedure())
                         .protocol(val.protocol())
                         .validate()?
                         .finalize())
            }
        };

        let mut categories = Vec::new();
        for cat in self.categories() {
            categories.push(CategoryBuilder::default()
                                .name(cat.name())
                                .domain(cat.domain().map(|s| s.into()))
                                .validate()?
                                .finalize());
        }

        let mut skip_hours = Vec::new();
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

                Some(ImageBuilder::default()
                         .url(val.url())
                         .title(val.title())
                         .link(val.link())
                         .width(width)
                         .height(height)
                         .description(description)
                         .validate()?
                         .finalize())
            }
        };

        let text_input = match self.text_input() {
            None => None,
            Some(val) => {
                Some(TextInputBuilder::default()
                         .title(val.title())
                         .description(val.description())
                         .name(val.name())
                         .link(val.link())
                         .validate()?
                         .finalize())
            }
        };

        let mut items = Vec::new();

        for item in self.items() {
            let mut categories = Vec::new();
            for cat in item.categories() {
                categories.push(CategoryBuilder::default()
                                    .name(cat.name())
                                    .domain(cat.domain().map(|s| s.into()))
                                    .validate()?
                                    .finalize());
            }

            let enclosure = match item.enclosure() {
                None => None,
                Some(eval) => {
                    Some(EnclosureBuilder::default()
                             .url(eval.url())
                             .length(i64::from_str(eval.length())?)
                             .mime_type(eval.mime_type())
                             .validate()?
                             .finalize())
                }
            };

            let guid = match item.guid() {
                None => None,
                Some(gval) => {
                    Some(GuidBuilder::default()
                             .value(gval.value())
                             .is_permalink(gval.is_permalink())
                             .finalize())
                }
            };

            let source = match item.source() {
                None => None,
                Some(sval) => {
                    let title = match sval.title() {
                        None => None,
                        Some(tval) => Some(tval.to_string()),
                    };

                    Some(SourceBuilder::default()
                             .url(sval.url())
                             .title(title)
                             .validate()?
                             .finalize())
                }
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

            items.push(ItemBuilder::default()
                           .title(title)
                           .link(link)
                           .description(description)
                           .author(author)
                           .pub_date(pub_date)
                           .comments(comments)
                           .categories(categories)
                           .enclosure(enclosure)
                           .guid(guid)
                           .source(source)
                           .validate()?
                           .finalize());
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

        Ok(ChannelBuilder::default()
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
               .categories(categories)
               .image(image)
               .text_input(text_input)
               .skip_hours(skip_hours)
               .skip_days(self.skip_days().to_vec())
               .items(items)
               .validate()?
               .finalize())
    }
}

impl ToString for Channel {
    fn to_string(&self) -> String {
        let buf = self.write_to(Vec::new()).unwrap_or_default();
        // this unwrap should be safe since the bytes written from the Channel are all valid utf8
        String::from_utf8(buf).unwrap()
    }
}

impl FromXml for Channel {
    fn from_xml<R: ::std::io::BufRead>(reader: &mut Reader<R>,
                                       _: Attributes)
                                       -> Result<Self, Error> {
        let mut channel = Channel::default();
        let mut buf = Vec::new();
        let mut skip_buf = Vec::new();

        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(element) => {
                    match element.name() {
                        b"category" => {
                            let category = Category::from_xml(reader, element.attributes())?;
                            channel.categories.push(category);
                        }
                        b"cloud" => {
                            let cloud = Cloud::from_xml(reader, element.attributes())?;
                            channel.cloud = Some(cloud);
                        }
                        b"image" => {
                            let image = Image::from_xml(reader, element.attributes())?;
                            channel.image = Some(image);
                        }
                        b"textInput" => {
                            let text_input = TextInput::from_xml(reader, element.attributes())?;
                            channel.text_input = Some(text_input);
                        }
                        b"item" => {
                            let item = Item::from_xml(reader, element.attributes())?;
                            channel.items.push(item);
                        }
                        b"title" => {
                            if let Some(content) = element_text(reader)? {
                                channel.title = content;
                            }
                        }
                        b"link" => {
                            if let Some(content) = element_text(reader)? {
                                channel.link = content;
                            }
                        }
                        b"description" => {
                            if let Some(content) = element_text(reader)? {
                                channel.description = content;
                            }
                        }
                        b"language" => channel.language = element_text(reader)?,
                        b"copyright" => channel.copyright = element_text(reader)?,
                        b"managingEditor" => {
                            channel.managing_editor = element_text(reader)?;
                        }
                        b"webMaster" => channel.webmaster = element_text(reader)?,
                        b"pubDate" => channel.pub_date = element_text(reader)?,
                        b"lastBuildDate" => {
                            channel.last_build_date = element_text(reader)?;
                        }
                        b"generator" => channel.generator = element_text(reader)?,
                        b"docs" => channel.docs = element_text(reader)?,
                        b"ttl" => channel.ttl = element_text(reader)?,
                        b"skipHours" => {
                            loop {
                                skip_buf.clear();
                                match reader.read_event(&mut skip_buf)? {
                                    Event::Start(element) => {
                                        if element.name() == b"hour" {
                                            if let Some(content) = element_text(reader)? {
                                                channel.skip_hours.push(content);
                                            }
                                        } else {
                                            reader.read_to_end(element.name(), &mut Vec::new())?;
                                        }
                                    }
                                    Event::End(_) | Event::Eof => break,
                                    _ => {}
                                }
                            }
                        }
                        b"skipDays" => {
                            loop {
                                skip_buf.clear();
                                match reader.read_event(&mut skip_buf)? {
                                    Event::Start(element) => {
                                        if element.name() == b"day" {
                                            if let Some(content) = element_text(reader)? {
                                                channel.skip_days.push(content);
                                            }
                                        } else {
                                            reader.read_to_end(element.name(), &mut Vec::new())?;
                                        }
                                    }
                                    Event::End(_) | Event::Eof => break,
                                    _ => {}
                                }
                            }
                        }
                        n => {
                            if let Some((ns, name)) = fromxml::extension_name(element.name()) {
                                parse_extension(reader,
                                                element.attributes(),
                                                ns,
                                                name,
                                                &mut channel.extensions)?;
                            } else {
                                reader.read_to_end(n, &mut skip_buf)?;
                            }
                        }
                    }
                }
                Event::End(_) => {
                    if !channel.extensions.is_empty() {
                        if let Some(map) = channel.extensions.remove("itunes") {
                            channel.itunes_ext = Some(ITunesChannelExtension::from_map(map)?);
                        }

                        if let Some(map) = channel.extensions.remove("dc") {
                            channel.dublin_core_ext = Some(DublinCoreExtension::from_map(map));
                        }
                    }

                    return Ok(channel);
                }
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }

        Err(Error::EOF)
    }
}

impl ToXml for Channel {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = b"channel";

        writer
            .write_event(Event::Start(BytesStart::borrowed(name, name.len())))?;

        writer.write_text_element(b"title", &self.title)?;
        writer.write_text_element(b"link", &self.link)?;
        writer
            .write_text_element(b"description", &self.description)?;

        if let Some(language) = self.language.as_ref() {
            writer.write_text_element(b"language", language)?;
        }

        if let Some(copyright) = self.copyright.as_ref() {
            writer.write_text_element(b"copyright", copyright)?;
        }

        if let Some(managing_editor) = self.managing_editor.as_ref() {
            writer
                .write_text_element(b"managingEditor", managing_editor)?;
        }

        if let Some(webmaster) = self.webmaster.as_ref() {
            writer.write_text_element(b"webMaster", webmaster)?;
        }

        if let Some(pub_date) = self.pub_date.as_ref() {
            writer.write_text_element(b"pubDate", pub_date)?;
        }

        if let Some(last_build_date) = self.last_build_date.as_ref() {
            writer
                .write_text_element(b"lastBuildDate", last_build_date)?;
        }

        writer.write_objects(&self.categories)?;

        if let Some(generator) = self.generator.as_ref() {
            writer.write_text_element(b"generator", generator)?;
        }

        if let Some(docs) = self.docs.as_ref() {
            writer.write_text_element(b"docs", docs)?;
        }

        if let Some(cloud) = self.cloud.as_ref() {
            writer.write_object(cloud)?;
        }

        if let Some(ttl) = self.ttl.as_ref() {
            writer.write_text_element(b"ttl", ttl)?;
        }

        if let Some(image) = self.image.as_ref() {
            writer.write_object(image)?;
        }

        if let Some(text_input) = self.text_input.as_ref() {
            writer.write_object(text_input)?;
        }

        if !self.skip_hours.is_empty() {
            let name = b"skipHours";
            writer
                .write_event(Event::Start(BytesStart::borrowed(name, name.len())))?;
            for hour in &self.skip_hours {
                writer.write_text_element(b"hour", hour)?;
            }
            writer.write_event(Event::End(BytesEnd::borrowed(name)))?;
        }

        if !self.skip_days.is_empty() {
            let name = b"skipDays";
            writer
                .write_event(Event::Start(BytesStart::borrowed(name, name.len())))?;
            for day in &self.skip_days {
                writer.write_text_element(b"day", day)?;
            }
            writer.write_event(Event::End(BytesEnd::borrowed(name)))?;
        }

        writer.write_objects(&self.items)?;

        for map in self.extensions.values() {
            for extensions in map.values() {
                for extension in extensions {
                    extension.to_xml(writer)?;
                }
            }
        }

        if let Some(ext) = self.itunes_ext.as_ref() {
            ext.to_xml(writer)?;
        }

        if let Some(ext) = self.dublin_core_ext.as_ref() {
            ext.to_xml(writer)?;
        }

        writer.write_event(Event::End(BytesEnd::borrowed(name)))?;
        Ok(())
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

/// A builder used to create a `Channel`.
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
    /// Set the title of the `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let builder = ChannelBuilder::default()
    ///     .title("The Linux Action Show! OGG");
    /// ```
    pub fn title<S>(mut self, title: S) -> ChannelBuilder
        where S: Into<String>
    {
        self.title = title.into();
        self
    }

    /// Set the web site URL for the `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let builder = ChannelBuilder::default()
    ///     .link("http://www.jupiterbroadcasting.com");
    /// ```
    pub fn link<S>(mut self, link: S) -> ChannelBuilder
        where S: Into<String>
    {
        self.link = link.into();
        self
    }

    /// Set the description of the `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let description = "Ogg Vorbis audio versions of The Linux \
    ///     Action Show! A show that covers everything geeks care about in the \
    ///     computer industry. Get a solid dose of Linux, gadgets, news events \
    ///     and much more!".to_string();
    ///
    /// let builder = ChannelBuilder::default()
    ///     .description(description);
    /// ```
    pub fn description<S>(mut self, description: S) -> ChannelBuilder
        where S: Into<String>
    {
        self.description = description.into();
        self
    }

    /// Set the language of the `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let builder = ChannelBuilder::default()
    ///     .language("en".to_string());
    /// ```
    pub fn language<V>(mut self, language: V) -> ChannelBuilder
        where V: Into<Option<String>>
    {
        self.language = language.into();
        self
    }

    /// Set the copyright notice for the content of the `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let builder = ChannelBuilder::default()
    ///     .copyright("Copyright 2002, Spartanburg Herald-Journal".to_string());
    /// ```
    pub fn copyright<V>(mut self, copyright: V) -> ChannelBuilder
        where V: Into<Option<String>>
    {
        self.copyright = copyright.into();
        self
    }

    /// Set the email address for the managing editor of the `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let builder = ChannelBuilder::default()
    ///     .managing_editor("chris@jupiterbroadcasting.com (Chris Fisher)".to_string());
    /// ```
    pub fn managing_editor<V>(mut self, managing_editor: V) -> ChannelBuilder
        where V: Into<Option<String>>
    {
        self.managing_editor = managing_editor.into();
        self
    }

    /// Set the email address for the webmaster of the `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let builder = ChannelBuilder::default()
    ///     .webmaster("chris@jupiterbroadcasting.com (Chris Fisher)".to_string());
    /// ```
    pub fn webmaster<V>(mut self, webmaster: V) -> ChannelBuilder
        where V: Into<Option<String>>
    {
        self.webmaster = webmaster.into();
        self
    }

    /// Set the publication date for the content of the `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let builder = ChannelBuilder::default()
    ///     .pub_date("Sun, 13 Mar 2016 20:02:02 -0700".to_string());
    /// ```
    pub fn pub_date<V>(mut self, pub_date: V) -> ChannelBuilder
        where V: Into<Option<String>>
    {
        self.pub_date = pub_date.into();
        self
    }

    /// Set the time that the content of the `Channel` was last changed.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let builder = ChannelBuilder::default()
    ///     .last_build_date("Sun, 13 Mar 2016 20:02:02 -0700".to_string());
    /// ```
    pub fn last_build_date<V>(mut self, last_build_date: V) -> ChannelBuilder
        where V: Into<Option<String>>
    {
        self.last_build_date = last_build_date.into();
        self
    }

    /// Set the categories that the `Channel` belongs to.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, CategoryBuilder};
    ///
    /// let category = CategoryBuilder::default()
    ///     .name("Podcast")
    ///     .finalize();
    ///
    /// let builder = ChannelBuilder::default()
    ///     .categories(vec![category]);
    /// ```
    pub fn categories<V>(mut self, categories: V) -> ChannelBuilder
        where V: Into<Vec<Category>>
    {
        self.categories = categories.into();
        self
    }

    /// Set the name of the program used to generate the contents of the `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let generator = "Feeder 2.5.12(2294); Mac OS X Version 10.9.5 (Build 13F34) \
    ///     http://reinventedsoftware.com/feeder/".to_string();
    ///
    /// let builder = ChannelBuilder::default()
    ///     .generator(generator);
    /// ```
    pub fn generator<V>(mut self, generator: V) -> ChannelBuilder
        where V: Into<Option<String>>
    {
        self.generator = generator.into();
        self
    }

    /// Set the URL that points to the documentation of the RSS format used in the `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let builder = ChannelBuilder::default()
    ///     .docs("http://blogs.law.harvard.edu/tech/rss/".to_string());
    /// ```
    pub fn docs<V>(mut self, docs: V) -> ChannelBuilder
        where V: Into<Option<String>>
    {
        self.docs = docs.into();
        self
    }

    /// Set the information used to register with a cloud for notifications of updates to the
    /// `Channel`
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, CloudBuilder};
    ///
    /// let cloud = CloudBuilder::default()
    ///     .domain("http://rpc.sys.com/")
    ///     .protocol("soap")
    ///     .finalize();
    ///
    /// let builder = ChannelBuilder::default()
    ///     .cloud(cloud);
    /// ```
    pub fn cloud<V>(mut self, cloud: V) -> ChannelBuilder
        where V: Into<Option<Cloud>>
    {
        self.cloud = cloud.into();
        self
    }

    /// Set the time to live of the `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let builder = ChannelBuilder::default()
    ///     .ttl(60);
    /// ```
    pub fn ttl<V>(mut self, ttl: V) -> ChannelBuilder
        where V: Into<Option<i64>>
    {
        self.ttl = ttl.into();
        self
    }

    /// Set the image to be display with the `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, ImageBuilder};
    ///
    /// let image = ImageBuilder::default()
    ///     .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg")
    ///     .link("http://www.jupiterbroadcasting.com/")
    ///     .finalize();
    ///
    /// let builder = ChannelBuilder::default()
    ///     .image(image);
    /// ```
    pub fn image<V>(mut self, image: V) -> ChannelBuilder
        where V: Into<Option<Image>>
    {
        self.image = image.into();
        self
    }

    /// Set the [PICS](https://www.w3.org/PICS/) rating for the `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let builder = ChannelBuilder::default()
    ///     .rating("PG-13".to_string());
    /// ```
    pub fn rating<V>(mut self, rating: V) -> ChannelBuilder
        where V: Into<Option<String>>
    {
        self.rating = rating.into();
        self
    }

    /// Set the information for a text box to be displayed with the `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, TextInputBuilder};
    ///
    /// let text_input = TextInputBuilder::default()
    ///     .link("http://www.example.com/feedback")
    ///     .finalize();
    ///
    /// let builder = ChannelBuilder::default()
    ///     .text_input(text_input);
    /// ```
    pub fn text_input<V>(mut self, text_input: V) -> ChannelBuilder
        where V: Into<Option<TextInput>>
    {
        self.text_input = text_input.into();
        self
    }

    /// Set the hours that aggregators can skip for refreshing content.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let builder = ChannelBuilder::default()
    ///     .skip_hours(vec![0, 12, 18]);
    /// ```
    pub fn skip_hours<V>(mut self, skip_hours: V) -> ChannelBuilder
        where V: Into<Vec<i64>>
    {
        self.skip_hours = skip_hours.into();
        self
    }

    /// Set the days that aggregators can skip for refreshing content.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let builder = ChannelBuilder::default()
    ///     .skip_days(vec!["Monday".to_string(), "Tuesday".to_string()]);
    /// ```
    pub fn skip_days<V>(mut self, skip_days: V) -> ChannelBuilder
        where V: Into<Vec<String>>
    {
        self.skip_days = skip_days.into();
        self
    }

    /// Set the `Item`s in this `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ChannelBuilder, ItemBuilder};
    ///
    /// let item = ItemBuilder::default()
    ///     .title("Making Music with Linux | LAS 408".to_string())
    ///     .finalize();
    ///
    /// let builder = ChannelBuilder::default()
    ///     .items(vec![item]);
    /// ```
    pub fn items<V>(mut self, items: V) -> ChannelBuilder
        where V: Into<Vec<Item>>
    {
        self.items = items.into();
        self
    }

    /// Set the `ITunesChannelExtension` for the `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    /// use rss::extension::itunes::{ITunesChannelExtensionBuilder, ITunesOwnerBuilder,
    ///     ITunesCategoryBuilder};
    ///
    /// let owner = ITunesOwnerBuilder::default()
    ///     .email("email@example.com".to_string())
    ///     .name("name".to_string())
    ///     .finalize();
    ///
    /// let subcategory = ITunesCategoryBuilder::default()
    ///     .text("text")
    ///     .finalize();
    ///
    /// let category = ITunesCategoryBuilder::default()
    ///     .text("text")
    ///     .subcategory(Box::new(subcategory))
    ///     .finalize();
    ///
    /// let itunes_channel = ITunesChannelExtensionBuilder::default()
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
    ///     .categories(vec![category])
    ///     .finalize();
    ///
    /// let builder = ChannelBuilder::default()
    ///     .itunes_ext(itunes_channel);
    /// ```
    pub fn itunes_ext<V>(mut self, itunes_ext: V) -> ChannelBuilder
        where V: Into<Option<ITunesChannelExtension>>
    {
        self.itunes_ext = itunes_ext.into();
        self
    }

    /// Set the `DublinCoreExtension` for the `Channel`.
    pub fn dublin_core_ext<V>(mut self, dublin_core_ext: V) -> ChannelBuilder
        where V: Into<Option<DublinCoreExtension>>
    {
        self.dublin_core_ext = dublin_core_ext.into();
        self
    }

    /// Set the extensions for the `Channel`.
    pub fn extensions<V>(mut self, extensions: V) -> ChannelBuilder
        where V: Into<ExtensionMap>
    {
        self.extensions = extensions.into();
        self
    }

    /// Set the namespaces for the `Channel`.
    pub fn namespaces<V>(mut self, namespaces: V) -> ChannelBuilder
        where V: Into<HashMap<String, String>>
    {
        self.namespaces = namespaces.into();
        self
    }

    /// Validate the contents of this `ChannelBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let description = "Ogg Vorbis audio versions of The Linux \
    ///     Action Show! A show that covers everything geeks care about in the \
    ///     computer industry. Get a solid dose of Linux, gadgets, news events \
    ///     and much more!";
    ///
    /// let builder = ChannelBuilder::default()
    ///         .title("The Linux Action Show! OGG")
    ///         .link("http://www.jupiterbroadcasting.com")
    ///         .description(description)
    ///         .validate()
    ///         .unwrap();
    /// ```
    pub fn validate(self) -> Result<ChannelBuilder, Error> {
        Url::parse(self.link.as_str())?;

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
            match SkipDay::from_str(day.as_str()) {
                Ok(_) => (),
                Err(err) => return Err(Error::Validation(err.to_string())),
            };
        }

        for hour in self.skip_hours.as_slice() {
            if *hour < 0 {
                return Err(Error::Validation("Channel Skip Hour cannot be a negative value."
                                                 .to_string()));
            } else if *hour > 23 {
                return Err(Error::Validation("Channel Skip Hour cannot be greater than 23."
                                                 .to_string()));
            }
        }

        if self.ttl.is_some() && self.ttl.unwrap() < 0 {
            return Err(Error::Validation("Channel TTL cannot be a negative value.".to_string()));
        }

        Ok(self)
    }

    /// Construct the `Channel` from this `ChannelBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ChannelBuilder;
    ///
    /// let description = "Ogg Vorbis audio versions of The Linux \
    ///     Action Show! A show that covers everything geeks care about in the \
    ///     computer industry. Get a solid dose of Linux, gadgets, news events \
    ///     and much more!";
    ///
    /// let channel = ChannelBuilder::default()
    ///         .title("The Linux Action Show! OGG")
    ///         .link("http://www.jupiterbroadcasting.com")
    ///         .description(description)
    ///         .finalize();
    /// ```
    pub fn finalize(self) -> Channel {
        Channel {
            title: self.title,
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
            ttl: self.ttl.map(|n| n.to_string()),
            image: self.image,
            rating: self.rating,
            text_input: self.text_input,
            skip_hours: self.skip_hours.into_iter().map(|n| n.to_string()).collect(),
            skip_days: self.skip_days,
            items: self.items,
            itunes_ext: self.itunes_ext,
            dublin_core_ext: self.dublin_core_ext,
            extensions: self.extensions,
            namespaces: self.namespaces,
        }
    }
}

/// Enumerations of values for `SkipDays`.
enum SkipDay {
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

impl FromStr for SkipDay {
    type Err = &'static str;

    /// Convert `&str` to `SkipDay`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Monday" => Ok(SkipDay::Monday),
            "Tuesday" => Ok(SkipDay::Tuesday),
            "Wednesday" => Ok(SkipDay::Wednesday),
            "Thursday" => Ok(SkipDay::Thursday),
            "Friday" => Ok(SkipDay::Friday),
            "Saturday" => Ok(SkipDay::Saturday),
            "Sunday" => Ok(SkipDay::Sunday),
            _ => Err("Skip Day is not not a valid value"),
        }
    }
}
