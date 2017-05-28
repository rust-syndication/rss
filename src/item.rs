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
use fromxml::{self, FromXml, parse_extension, element_text};
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
    ///     .title(title_string.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(title_string), item.title());
    /// ```
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let item = ItemBuilder::new()
    ///     .title(None)
    ///     .finalize();
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
    ///     .link(link_string.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(link_string), item.link());
    /// ```
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let item = ItemBuilder::new()
    ///     .link(None)
    ///     .finalize();
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
    ///     .description(description_string.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(description_string), item.description());
    /// ```
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let item = ItemBuilder::new()
    ///     .description(None)
    ///     .finalize();
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
    ///     .author(author_string.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(author_string), item.author());
    /// ```
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let item = ItemBuilder::new()
    ///     .author(None)
    ///     .finalize();
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
    ///     .finalize();
    ///
    /// let category_2 = CategoryBuilder::new()
    ///     .domain("http://jupiterbroadcasting.com".to_string())
    ///     .name("Podcast")
    ///     .finalize();
    ///
    /// let categories_vec = vec![category_1, category_2];
    ///
    /// let item = ItemBuilder::new()
    ///     .categories(categories_vec.clone())
    ///     .finalize();
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
    ///     .comments(comments_string.to_string())
    ///     .finalize();
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
    ///     .finalize();
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
    ///     .finalize();
    ///
    /// let item = ItemBuilder::new()
    ///     .enclosure(enclosure)
    ///     .finalize();
    ///
    /// assert!(item.enclosure().is_some());
    /// ```
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let item = ItemBuilder::new()
    ///     .enclosure(None)
    ///     .finalize();
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
    ///     .finalize();
    ///
    /// let item = ItemBuilder::new()
    ///     .guid(guid)
    ///     .finalize();
    /// assert!(item.guid().is_some())
    /// ```
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let item = ItemBuilder::new()
    ///     .guid(None)
    ///     .finalize();
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
    ///     .pub_date(pub_date.to_string())
    ///     .finalize();
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
    ///     .finalize();
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
    ///     .finalize();
    ///
    /// let item = ItemBuilder::new()
    ///     .source(source)
    ///     .finalize();
    ///
    /// assert!(item.source().is_some())
    /// ```
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let item = ItemBuilder::new()
    ///     .source(None)
    ///     .finalize();
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
    ///     .author("author".to_string())
    ///     .block("block".to_string())
    ///     .image("image".to_string())
    ///     .duration("duration".to_string())
    ///     .explicit("explicit".to_string())
    ///     .closed_captioned("closed_captioned".to_string())
    ///     .order("order".to_string())
    ///     .subtitle("subtitle".to_string())
    ///     .summary("summary".to_string())
    ///     .keywords("keywords".to_string())
    ///     .finalize();
    ///
    /// let item = ItemBuilder::new()
    ///     .title("Making Music with Linux | LAS 408".to_string())
    ///     .itunes_ext(itunes_item)
    ///     .finalize();
    ///
    /// assert!(item.itunes_ext().is_some());
    /// ```
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let item = ItemBuilder::new()
    ///     .title("Making Music with Linux | LAS 408".to_string())
    ///     .itunes_ext(None)
    ///     .finalize();
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
    fn from_xml<R: ::std::io::BufRead>(reader: &mut Reader<R>,
                                       _: Attributes)
                                       -> Result<Self, Error> {
        let mut item = Item::default();
        let mut buf = Vec::new();

        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(element) => {
                    match element.name() {
                        b"category" => {
                            let category = Category::from_xml(reader, element.attributes())?;
                            item.categories.push(category);
                        }
                        b"guid" => {
                            let guid = Guid::from_xml(reader, element.attributes())?;
                            item.guid = Some(guid);
                        }
                        b"enclosure" => {
                            let enclosure = Enclosure::from_xml(reader, element.attributes())?;
                            item.enclosure = Some(enclosure);
                        }
                        b"source" => {
                            let source = Source::from_xml(reader, element.attributes())?;
                            item.source = Some(source);
                        }
                        b"title" => item.title = element_text(reader)?,
                        b"link" => item.link = element_text(reader)?,
                        b"description" => item.description = element_text(reader)?,
                        b"author" => item.author = element_text(reader)?,
                        b"comments" => item.comments = element_text(reader)?,
                        b"pubDate" => item.pub_date = element_text(reader)?,
                        b"content:encoded" => item.content = element_text(reader)?,
                        n => {
                            if let Some((ns, name)) = fromxml::extension_name(n) {
                                parse_extension(reader,
                                                element.attributes(),
                                                ns,
                                                name,
                                                &mut item.extensions)?;
                            } else {
                                reader.read_to_end(n, &mut Vec::new())?;
                            }
                        }
                    }
                }
                Event::End(_) => {
                    if !item.extensions.is_empty() {
                        if let Some(map) = item.extensions.remove("itunes") {
                            item.itunes_ext = Some(ITunesItemExtension::from_map(map));
                        }

                        if let Some(map) = item.extensions.remove("dc") {
                            item.dublin_core_ext = Some(DublinCoreExtension::from_map(map));
                        }
                    }

                    return Ok(item);
                }
                Event::Eof => break,
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

        writer.write_event(Event::End(BytesEnd::borrowed(name)))?;
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
    /// let item_builder = ItemBuilder::new()
    ///     .title("Making Music with Linux | LAS 408".to_string());
    /// ```
    pub fn title<V: Into<Option<String>>>(mut self, title: V) -> ItemBuilder {
        self.title = title.into();
        self
    }


    /// Set the optional link that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let item_builder = ItemBuilder::new()
    ///     .link("http://www.jupiterbroadcasting.com".to_string());
    /// ```
    pub fn link<V: Into<Option<String>>>(mut self, link: V) -> ItemBuilder {
        self.link = link.into();
        self
    }


    /// Set the optional description that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let item_builder = ItemBuilder::new()
    ///     .description("This is a test description".to_string());
    /// ```
    pub fn description<V: Into<Option<String>>>(mut self, description: V) -> ItemBuilder {
        self.description = description.into();
        self
    }


    /// Set the optional author that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let item_builder = ItemBuilder::new()
    ///     .author("Chris Fisher".to_string());
    /// ```
    pub fn author<V: Into<Option<String>>>(mut self, author: V) -> ItemBuilder {
        self.author = author.into();
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
    ///     .finalize();
    ///
    /// let item_builder = ItemBuilder::new()
    ///     .categories(vec![category]);
    /// ```
    pub fn categories<V: Into<Vec<Category>>>(mut self, categories: V) -> ItemBuilder {
        self.categories = categories.into();
        self
    }


    /// Set the optional comments that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let item_builder = ItemBuilder::new()
    ///     .comments("A comment".to_string());
    /// ```
    pub fn comments<V: Into<Option<String>>>(mut self, comments: V) -> ItemBuilder {
        self.comments = comments.into();
        self
    }


    /// Set the optional enclosure that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{EnclosureBuilder, ItemBuilder};
    ///
    /// let url = "http://www.podtrac.com/pts/redirect.ogg/traffic.libsyn.com/jnite/\
    ///     linuxactionshowep408.ogg";
    ///
    /// let enclosure = EnclosureBuilder::new()
    ///     .url(url)
    ///     .mime_type("audio/ogg")
    ///     .finalize();
    ///
    /// let item_builder = ItemBuilder::new()
    ///     .enclosure(enclosure);
    /// ```
    pub fn enclosure<V: Into<Option<Enclosure>>>(mut self, enclosure: V) -> ItemBuilder {
        self.enclosure = enclosure.into();
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
    ///     .finalize();
    ///
    /// let item_builder = ItemBuilder::new()
    ///     .guid(guid);
    /// ```
    pub fn guid<V: Into<Option<Guid>>>(mut self, guid: V) -> ItemBuilder {
        self.guid = guid.into();
        self
    }


    /// Set the optional pub date that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let item_builder = ItemBuilder::new()
    ///     .pub_date("Sun, 13 Mar 2016 20:02:02-0700".to_string());
    /// ```
    pub fn pub_date<V: Into<Option<String>>>(mut self, pub_date: V) -> ItemBuilder {
        self.pub_date = pub_date.into();
        self
    }


    /// Set the optional source that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ItemBuilder, SourceBuilder};
    ///
    /// let source = SourceBuilder::new()
    ///     .url("http://www.tomalak.org/links2.xml")
    ///     .finalize();
    ///
    /// let item_builder = ItemBuilder::new()
    ///     .source(source);
    /// ```
    pub fn source<V: Into<Option<Source>>>(mut self, source: V) -> ItemBuilder {
        self.source = source.into();
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
    /// let itunes_item = ITunesItemExtensionBuilder::new()
    ///     .author("author".to_string())
    ///     .block("block".to_string())
    ///     .image("image".to_string())
    ///     .duration("duration".to_string())
    ///     .explicit("explicit".to_string())
    ///     .closed_captioned("closed_captioned".to_string())
    ///     .order("order".to_string())
    ///     .subtitle("subtitle".to_string())
    ///     .summary("summary".to_string())
    ///     .keywords("keywords".to_string())
    ///     .finalize();
    ///
    /// let item_builder = ItemBuilder::new()
    ///     .itunes_ext(itunes_item);
    /// ```
    pub fn itunes_ext<V>(mut self, itunes_ext: V) -> ItemBuilder
        where V: Into<Option<ITunesItemExtension>>
    {
        self.itunes_ext = itunes_ext.into();
        self
    }

    /// Set the optional dublin_core_ext that exists under `Item`.
    pub fn dublin_core_ext<V>(mut self, dublin_core_ext: V) -> ItemBuilder
        where V: Into<Option<DublinCoreExtension>>
    {
        self.dublin_core_ext = dublin_core_ext.into();
        self
    }

    /// Set the extensions that exists under `Item`.
    pub fn extensions<V: Into<ExtensionMap>>(mut self, extensions: V) -> ItemBuilder {
        self.extensions = extensions.into();
        self
    }

    /// Set the optional content that exists under `Item`.
    pub fn content<V: Into<Option<String>>>(mut self, content: V) -> ItemBuilder {
        self.content = content.into();
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
    ///     .title("Making Music with Linux | LAS 408".to_string())
    ///     .link("http://www.jupiterbroadcasting.com".to_string())
    ///     .description(None)
    ///     .author(None)
    ///     .categories(Vec::new())
    ///     .comments(None)
    ///     .enclosure(None)
    ///     .guid(None)
    ///     .pub_date(None)
    ///     .source(None)
    ///     .validate()
    ///     .unwrap()
    ///     .finalize();
    /// ```
    pub fn validate(self) -> Result<ItemBuilder, Error> {
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
    ///     .title("Making Music with Linux | LAS 408".to_string())
    ///     .link("http://www.jupiterbroadcasting.com".to_string())
    ///     .description(None)
    ///     .author(None)
    ///     .categories(Vec::new())
    ///     .comments(None)
    ///     .enclosure(None)
    ///     .guid(None)
    ///     .pub_date(None)
    ///     .source(None)
    ///     .finalize();
    /// ```
    pub fn finalize(self) -> Item {
        Item {
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
        }
    }
}
