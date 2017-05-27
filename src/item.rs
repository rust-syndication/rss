// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use category::Category;
use chrono::DateTime;
use enclosure::Enclosure;
use error::Error;
use extension::ExtensionMap;
use extension::dublincore::DublinCoreExtension;
use extension::itunes::ITunesItemExtension;
use fromxml::{self, FromXml};
use guid::Guid;
use quick_xml::errors::Error as XmlError;
use quick_xml::events::{Event, BytesStart, BytesEnd};
use quick_xml::events::attributes::Attributes;
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use source::Source;
use toxml::{ToXml, WriterExt};
use url::Url;

/// A representation of the `<item>` element.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Item {
    /// The title of the item.
    title: Option<String>,
    /// The URL of the item.
    link: Option<String>,
    /// The item synopsis.
    description: Option<String>,
    /// The email address of author of the item.
    author: Option<String>,
    /// The categories the item belongs to.
    categories: Vec<Category>,
    /// The URL for the comments page of the item.
    comments: Option<String>,
    /// The description of a media object that is attached to the item.
    enclosure: Option<Enclosure>,
    /// A unique identifier for the item.
    guid: Option<Guid>,
    /// The date the item was published.
    pub_date: Option<String>,
    /// The RSS channel the item came from.
    source: Option<Source>,
    /// The HTML contents of the item.
    content: Option<String>,
    /// The extensions for the item.
    extensions: ExtensionMap,
    /// The iTunes extension for the item.
    itunes_ext: Option<ITunesItemExtension>,
    /// The Dublin Core extension for the item.
    dublin_core_ext: Option<DublinCoreExtension>,
}

impl Item {
    /// Get the optional title that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let title_string = "Making Music with Linux | LAS 408";
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some(title_string.to_string()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(Some(title_string), item.title());
    /// ```
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let item = ItemBuilder::new()
    ///     .title(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(item.title().is_none());
    /// ```
    pub fn title(&self) -> Option<&str> {
        self.title.as_ref().map(|s| s.as_str())
    }

    /// Get the optional link that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let link_string = "http://www.jupiterbroadcasting.com/";
    /// let item = ItemBuilder::new()
    ///     .link(Some(link_string.to_string()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(Some(link_string), item.link());
    /// ```
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let item = ItemBuilder::new()
    ///     .link(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(item.link().is_none());
    /// ```
    pub fn link(&self) -> Option<&str> {
        self.link.as_ref().map(|s| s.as_str())
    }

    /// Get the optional description that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let description_string = "This is a test description";
    ///
    /// let item = ItemBuilder::new()
    ///     .description(Some(description_string.to_string()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(Some(description_string), item.description());
    /// ```
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let item = ItemBuilder::new()
    ///     .description(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(item.description().is_none());
    /// ```
    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|s| s.as_str())
    }

    /// Get the optional author that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let author_string = "Chris Fisher";
    ///
    /// let item = ItemBuilder::new()
    ///     .author(Some(author_string.to_string()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(Some(author_string), item.author());
    /// ```
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let item = ItemBuilder::new()
    ///     .author(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(item.author().is_none());
    /// ```
    pub fn author(&self) -> Option<&str> {
        self.author.as_ref().map(|s| s.as_str())
    }

    /// Get the categories that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{CategoryBuilder, ItemBuilder, Item};
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
    /// let item = ItemBuilder::new()
    ///     .categories(categories_vec.clone())
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(categories_vec.clone().len(), item.categories().len());
    /// ```
    pub fn categories(&self) -> &[Category] {
        &self.categories
    }

    /// Get the optional comments that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let comments_string = "http://example.com/comments";
    ///
    /// let item = ItemBuilder::new()
    ///     .comments(Some(comments_string.to_string()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let comments_option =  item.comments();
    /// assert!(comments_option.is_some());
    ///
    /// assert_eq!(Some(comments_string), item.comments());
    /// ```
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let item = ItemBuilder::new()
    ///     .comments(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(item.comments().is_none());
    /// ```
    pub fn comments(&self) -> Option<&str> {
        self.comments.as_ref().map(|s| s.as_str())
    }

    /// Get the optional enclosure that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{EnclosureBuilder, ItemBuilder, Item};
    ///
    /// let url = "http://www.podtrac.com/pts/redirect.ogg/".to_string()
    /// + "traffic.libsyn.com/jnite/linuxactionshowep408.ogg";
    ///
    /// let enclosure = EnclosureBuilder::new()
    ///     .url(url.as_ref())
    ///     .length(70772893)
    ///     .mime_type("audio/ogg")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let item = ItemBuilder::new()
    ///     .enclosure(Some(enclosure))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(item.enclosure().is_some());
    /// ```
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let item = ItemBuilder::new()
    ///     .enclosure(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(item.enclosure().is_none());
    /// ```
    pub fn enclosure(&self) -> Option<&Enclosure> {
        self.enclosure.as_ref()
    }

    /// Get the optional guid that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{GuidBuilder, ItemBuilder, Item};
    ///
    /// let guid = GuidBuilder::new()
    ///     .value("9DE46946-2F90-4D5D-9047-7E9165C16E7C")
    ///     .is_permalink(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let item = ItemBuilder::new()
    ///     .guid(Some(guid))
    ///     .finalize()
    ///     .unwrap();
    /// assert!(item.guid().is_some())
    /// ```
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let item = ItemBuilder::new()
    ///     .guid(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(item.guid().is_none());
    /// ```
    pub fn guid(&self) -> Option<&Guid> {
        self.guid.as_ref()
    }

    /// Get the optional pub date that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let pub_date = "Sun, 13 Mar 2016 20:02:02 -0700";
    ///
    /// let item = ItemBuilder::new()
    ///     .pub_date(Some(pub_date.to_string()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let local = item.pub_date();
    /// assert!(local.is_some());
    ///
    /// assert_eq!(pub_date.to_string(), local.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let item = ItemBuilder::new()
    ///     .pub_date(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(item.pub_date().is_none());
    /// ```
    pub fn pub_date(&self) -> Option<&str> {
        self.pub_date.as_ref().map(|s| s.as_str())
    }

    /// Get the optional source that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ItemBuilder, Item, SourceBuilder};
    ///
    /// let source = SourceBuilder::new()
    ///     .url("http://www.tomalak.org/links2.xml")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let item = ItemBuilder::new()
    ///     .source(Some(source))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(item.source().is_some())
    /// ```
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let item = ItemBuilder::new()
    ///     .source(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(item.source().is_none());
    /// ```
    pub fn source(&self) -> Option<&Source> {
        self.source.as_ref()
    }

    /// Get the optional `ITunesItemExtension` under `Item`.
    /// # Examples
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let itunes_item = ITunesItemExtensionBuilder::new()
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
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_string()))
    ///     .itunes_ext(Some(itunes_item))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(item.itunes_ext().is_some());
    /// ```
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_string()))
    ///     .itunes_ext(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(item.itunes_ext().is_none());
    /// ```
    pub fn itunes_ext(&self) -> Option<&ITunesItemExtension> {
        self.itunes_ext.as_ref()
    }

    /// Get the optional `DublinCoreExtension` under `Item`.
    pub fn dublin_core_ext(&self) -> Option<&DublinCoreExtension> {
        self.dublin_core_ext.as_ref()
    }

    /// Get the `ExtensionMap` under `Item`.
    pub fn extensions(&self) -> &ExtensionMap {
        &self.extensions
    }

    /// Get the optional content under `Item`.
    pub fn content(&self) -> Option<&str> {
        self.content.as_ref().map(|s| s.as_str())
    }
}

impl FromXml for Item {
    fn from_xml<R: ::std::io::BufRead>(mut reader: Reader<R>,
                                       _: Attributes)
                                       -> Result<(Self, Reader<R>), Error> {
        let mut item = Item::default();
        let mut buf = Vec::new();

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(element)) => {
                    match element.name() {
                        b"category" => {
                            let (category, reader_) = Category::from_xml(reader,
                                                                         element.attributes())?;
                            reader = reader_;
                            item.categories.push(category);
                        }
                        b"guid" => {
                            let (guid, reader_) = Guid::from_xml(reader, element.attributes())?;
                            reader = reader_;
                            item.guid = Some(guid);
                        }
                        b"enclosure" => {
                            let (enclosure, reader_) = Enclosure::from_xml(reader,
                                                                           element.attributes())?;
                            reader = reader_;
                            item.enclosure = Some(enclosure);
                        }
                        b"source" => {
                            let (source, reader_) = Source::from_xml(reader, element.attributes())?;
                            reader = reader_;
                            item.source = Some(source);
                        }
                        b"title" => item.title = element_text!(reader),
                        b"link" => item.link = element_text!(reader),
                        b"description" => item.description = element_text!(reader),
                        b"author" => item.author = element_text!(reader),
                        b"comments" => item.comments = element_text!(reader),
                        b"pubDate" => item.pub_date = element_text!(reader),
                        b"content:encoded" => item.content = element_text!(reader),
                        n => {
                            if let Some((ns, name)) = fromxml::extension_name(n) {
                                parse_extension!(reader, element, ns, name, item.extensions);
                            } else {
                                try!(reader.read_to_end(n, &mut Vec::new()));
                            }
                        }
                    }
                }
                Ok(Event::End(_)) => {
                    if !item.extensions.is_empty() {
                        if let Some(map) = item.extensions.remove("itunes") {
                            item.itunes_ext = Some(ITunesItemExtension::from_map(map));
                        }

                        if let Some(map) = item.extensions.remove("dc") {
                            item.dublin_core_ext = Some(DublinCoreExtension::from_map(map));
                        }
                    }

                    return Ok((item, reader));
                }
                Ok(Event::Eof) => break,
                Err(err) => return Err(err.into()),
                _ => {}
            }
            buf.clear();
        }

        Err(Error::EOF)
    }
}

impl ToXml for Item {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = b"item";

        writer
            .write_event(Event::Start(BytesStart::borrowed(name, name.len())))?;

        if let Some(title) = self.title.as_ref() {
            writer.write_text_element(b"title", title)?;
        }

        if let Some(link) = self.link.as_ref() {
            writer.write_text_element(b"link", link)?;
        }

        if let Some(description) = self.description.as_ref() {
            writer.write_text_element(b"description", description)?;
        }

        if let Some(author) = self.author.as_ref() {
            writer.write_text_element(b"author", author)?;
        }

        writer.write_objects(&self.categories)?;

        if let Some(comments) = self.comments.as_ref() {
            writer.write_text_element(b"comments", comments)?;
        }

        if let Some(enclosure) = self.enclosure.as_ref() {
            writer.write_object(enclosure)?;
        }

        if let Some(guid) = self.guid.as_ref() {
            writer.write_object(guid)?;
        }

        if let Some(pub_date) = self.pub_date.as_ref() {
            writer.write_text_element(b"pubDate", pub_date)?;
        }

        if let Some(source) = self.source.as_ref() {
            writer.write_object(source)?;
        }

        if let Some(content) = self.content.as_ref() {
            writer.write_cdata_element(b"content:encoded", content)?;
        }

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

        try!(writer.write_event(Event::End(BytesEnd::borrowed(name))));
        Ok(())
    }
}

/// This `ItemBuilder` struct creates the `Item`.
#[derive(Debug, Clone, Default)]
pub struct ItemBuilder {
    title: Option<String>,
    link: Option<String>,
    description: Option<String>,
    author: Option<String>,
    categories: Vec<Category>,
    comments: Option<String>,
    enclosure: Option<Enclosure>,
    guid: Option<Guid>,
    pub_date: Option<String>,
    source: Option<Source>,
    extensions: ExtensionMap,
    itunes_ext: Option<ITunesItemExtension>,
    dublin_core_ext: Option<DublinCoreExtension>,
    content: Option<String>,
}

impl ItemBuilder {
    /// Construct a new `ItemBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let item_builder = ItemBuilder::new();
    /// ```
    pub fn new() -> ItemBuilder {
        ItemBuilder::default()
    }


    /// Set the optional title that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.title(Some("Making Music with Linux | LAS
    /// 408".to_string()));
    /// ```
    pub fn title(mut self, title: Option<String>) -> ItemBuilder {
        self.title = title;
        self
    }


    /// Set the optional link that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.link(Some("http://www.jupiterbroadcasting.com".
    /// to_owned()));
    /// ```
    pub fn link(mut self, link: Option<String>) -> ItemBuilder {
        self.link = link;
        self
    }


    /// Set the optional description that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.description(Some("This is a test description".to_string()));
    /// ```
    pub fn description(mut self, description: Option<String>) -> ItemBuilder {
        self.description = description;
        self
    }


    /// Set the optional author that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.author(Some("Chris Fisher".to_string()));
    /// ```
    pub fn author(mut self, author: Option<String>) -> ItemBuilder {
        self.author = author;
        self
    }


    /// Set the optional categories that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{CategoryBuilder, ItemBuilder};
    ///
    /// let category = CategoryBuilder::new()
    ///     .finalize()
    ///     .unwrap();;
    /// let categories = vec![category];
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.categories(categories);
    /// ```
    pub fn categories(mut self, categories: Vec<Category>) -> ItemBuilder {
        self.categories = categories;
        self
    }


    /// Set the optional comments that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.comments(Some("Test Comment".to_string()));
    /// ```
    pub fn comments(mut self, comments: Option<String>) -> ItemBuilder {
        self.comments = comments;
        self
    }


    /// Set the optional enclosure that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{EnclosureBuilder, ItemBuilder};
    ///
    /// let url = "http://www.podtrac.com/pts/redirect.ogg/".to_string()
    /// + "traffic.libsyn.com/jnite/linuxactionshowep408.ogg";
    ///
    /// let enclosure = EnclosureBuilder::new()
    ///     .url(url.as_str())
    ///     .mime_type("audio/ogg")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.enclosure(Some(enclosure));
    /// ```
    pub fn enclosure(mut self, enclosure: Option<Enclosure>) -> ItemBuilder {
        self.enclosure = enclosure;
        self
    }


    /// Set the optional guid that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{GuidBuilder, ItemBuilder};
    ///
    /// let guid = GuidBuilder::new()
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.guid(Some(guid));
    /// ```
    pub fn guid(mut self, guid: Option<Guid>) -> ItemBuilder {
        self.guid = guid;
        self
    }


    /// Set the optional pub date that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.pub_date(Some("Sun, 13 Mar 2016
    /// 20:02:02-0700".to_string()));
    /// ```
    pub fn pub_date(mut self, pub_date: Option<String>) -> ItemBuilder {
        self.pub_date = pub_date;
        self
    }


    /// Set the optional source that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ItemBuilder, SourceBuilder};
    ///
    /// let url = "http://www.tomalak.org/links2.xml";
    ///
    /// let source = SourceBuilder::new()
    ///     .url(url)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.source(Some(source));
    /// ```
    pub fn source(mut self, source: Option<Source>) -> ItemBuilder {
        self.source = source;
        self
    }


    /// Set the optional itunes_ext that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let url = "http://www.tomalak.org/links2.xml";
    ///
    /// let itunes_item = ITunesItemExtensionBuilder::new()
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
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.itunes_ext(Some(itunes_item));
    /// ```
    pub fn itunes_ext(mut self, itunes_ext: Option<ITunesItemExtension>) -> ItemBuilder {
        self.itunes_ext = itunes_ext;
        self
    }

    /// Set the optional dublin_core_ext that exists under `Item`.
    pub fn dublin_core_ext(mut self, dublin_core_ext: Option<DublinCoreExtension>) -> ItemBuilder {
        self.dublin_core_ext = dublin_core_ext;
        self
    }

    /// Set the extensions that exists under `Item`.
    pub fn extensions(mut self, extensions: ExtensionMap) -> ItemBuilder {
        self.extensions = extensions;
        self
    }

    /// Set the optional content that exists under `Item`.
    pub fn content(mut self, content: Option<String>) -> ItemBuilder {
        self.content = content;
        self
    }

    /// Validate the contents of `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_string()))
    ///     .link(Some("http://www.jupiterbroadcasting.com".to_string()))
    ///     .description(None)
    ///     .author(None)
    ///     .categories(Vec::new())
    ///     .comments(None)
    ///     .enclosure(None)
    ///     .guid(None)
    ///     .pub_date(None)
    ///     .source(None)
    ///     .validate().unwrap()
    ///     .finalize().unwrap();
    /// ```
    pub fn validate(self) -> Result<ItemBuilder, Error> {
        if self.title.is_none() && self.description.is_none() {
            return Err(Error::Validation("Either Title or Description must have a value."
                                             .to_string()));
        }

        if let Some(ref link) = self.link {
            Url::parse(link.as_str())?;
        }

        if let Some(ref comments) = self.comments {
            Url::parse(comments.as_str())?;
        }

        if let Some(ref pub_date) = self.pub_date {
            DateTime::parse_from_rfc2822(pub_date.as_str())?;
        }

        Ok(self)
    }


    /// Construct the `Item` from the `ItemBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_string()))
    ///     .link(Some("http://www.jupiterbroadcasting.com".to_string()))
    ///     .description(None)
    ///     .author(None)
    ///     .categories(Vec::new())
    ///     .comments(None)
    ///     .enclosure(None)
    ///     .guid(None)
    ///     .pub_date(None)
    ///     .source(None)
    ///     .finalize()
    ///     .unwrap();
    /// ```
    pub fn finalize(self) -> Result<Item, Error> {
        Ok(Item {
               title: self.title,
               link: self.link,
               description: self.description,
               author: self.author,
               categories: self.categories,
               comments: self.comments,
               enclosure: self.enclosure,
               guid: self.guid,
               pub_date: self.pub_date,
               source: self.source,
               extensions: self.extensions,
               itunes_ext: self.itunes_ext,
               dublin_core_ext: self.dublin_core_ext,
               content: self.content,
           })
    }
}
