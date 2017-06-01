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
    /// Return the title of this `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let title = "Making Music with Linux | LAS 408";
    ///
    /// let item = ItemBuilder::default()
    ///     .title(title.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(title), item.title());
    /// ```
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let item = ItemBuilder::default()
    ///     .title(None)
    ///     .finalize();
    ///
    /// assert!(item.title().is_none());
    /// ```
    pub fn title(&self) -> Option<&str> {
        self.title.as_ref().map(|s| s.as_str())
    }

    /// Return the URL for this `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let link = "http://www.jupiterbroadcasting.com/";
    ///
    /// let item = ItemBuilder::default()
    ///     .link(link.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(link), item.link());
    /// ```
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let item = ItemBuilder::default()
    ///     .link(None)
    ///     .finalize();
    ///
    /// assert!(item.link().is_none());
    /// ```
    pub fn link(&self) -> Option<&str> {
        self.link.as_ref().map(|s| s.as_str())
    }

    /// Return the description of this `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let description = "This is a test description";
    ///
    /// let item = ItemBuilder::default()
    ///     .description(description.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(description), item.description());
    /// ```
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let item = ItemBuilder::default()
    ///     .description(None)
    ///     .finalize();
    ///
    /// assert!(item.description().is_none());
    /// ```
    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|s| s.as_str())
    }

    /// Return the author of this `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let author = "Chris Fisher";
    ///
    /// let item = ItemBuilder::default()
    ///     .author(author.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(author), item.author());
    /// ```
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let item = ItemBuilder::default()
    ///     .author(None)
    ///     .finalize();
    ///
    /// assert!(item.author().is_none());
    /// ```
    pub fn author(&self) -> Option<&str> {
        self.author.as_ref().map(|s| s.as_str())
    }

    /// Return the categories that this `Item` belongs to.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{CategoryBuilder, ItemBuilder};
    ///
    /// let category = CategoryBuilder::default()
    ///     .name("Media")
    ///     .finalize();
    ///
    /// let categories = vec![category];
    ///
    /// let item = ItemBuilder::default()
    ///     .categories(categories.clone())
    ///     .finalize();
    ///
    /// assert_eq!(categories, item.categories());
    /// ```
    pub fn categories(&self) -> &[Category] {
        &self.categories
    }

    /// Return the URL for comments about this `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let comments = "http://example.com/comments";
    ///
    /// let item = ItemBuilder::default()
    ///     .comments(comments.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(comments), item.comments());
    /// ```
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let item = ItemBuilder::default()
    ///     .comments(None)
    ///     .finalize();
    ///
    /// assert!(item.comments().is_none());
    /// ```
    pub fn comments(&self) -> Option<&str> {
        self.comments.as_ref().map(|s| s.as_str())
    }

    /// Return the enclosure information for this `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{EnclosureBuilder, ItemBuilder};
    ///
    /// let url = "http://www.podtrac.com/pts/redirect.ogg/traffic.libsyn.com/jnite/\
    ///     linuxactionshowep408.ogg";
    ///
    /// let enclosure = EnclosureBuilder::default()
    ///     .url(url)
    ///     .length(70772893)
    ///     .mime_type("audio/ogg")
    ///     .finalize();
    ///
    /// let item = ItemBuilder::default()
    ///     .enclosure(enclosure)
    ///     .finalize();
    ///
    /// assert!(item.enclosure().is_some());
    /// ```
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let item = ItemBuilder::default()
    ///     .enclosure(None)
    ///     .finalize();
    ///
    /// assert!(item.enclosure().is_none());
    /// ```
    pub fn enclosure(&self) -> Option<&Enclosure> {
        self.enclosure.as_ref()
    }

    /// Return the GUID for this `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{GuidBuilder, ItemBuilder};
    ///
    /// let guid = GuidBuilder::default()
    ///     .value("9DE46946-2F90-4D5D-9047-7E9165C16E7C")
    ///     .finalize();
    ///
    /// let item = ItemBuilder::default()
    ///     .guid(guid)
    ///     .finalize();
    ///
    /// assert!(item.guid().is_some())
    /// ```
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let item = ItemBuilder::default()
    ///     .guid(None)
    ///     .finalize();
    ///
    /// assert!(item.guid().is_none());
    /// ```
    pub fn guid(&self) -> Option<&Guid> {
        self.guid.as_ref()
    }

    /// Return the publication date for this `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let pub_date = "Sun, 13 Mar 2016 20:02:02 -0700";
    ///
    /// let item = ItemBuilder::default()
    ///     .pub_date(pub_date.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(pub_date), item.pub_date());
    /// ```
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let item = ItemBuilder::default()
    ///     .pub_date(None)
    ///     .finalize();
    ///
    /// assert!(item.pub_date().is_none());
    /// ```
    pub fn pub_date(&self) -> Option<&str> {
        self.pub_date.as_ref().map(|s| s.as_str())
    }

    /// Return the source URL for this `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ItemBuilder, SourceBuilder};
    ///
    /// let source = SourceBuilder::default()
    ///     .url("http://www.tomalak.org/links2.xml")
    ///     .finalize();
    ///
    /// let item = ItemBuilder::default()
    ///     .source(source)
    ///     .finalize();
    ///
    /// assert!(item.source().is_some())
    /// ```
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let item = ItemBuilder::default()
    ///     .source(None)
    ///     .finalize();
    ///
    /// assert!(item.source().is_none());
    /// ```
    pub fn source(&self) -> Option<&Source> {
        self.source.as_ref()
    }

    /// Return the content of this `Item`.
    pub fn content(&self) -> Option<&str> {
        self.content.as_ref().map(|s| s.as_str())
    }

    /// Return the `ITunesItemExtension` for this `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let itunes_item = ITunesItemExtensionBuilder::default()
    ///     .author("author".to_string())
    ///     .finalize();
    ///
    /// let item = ItemBuilder::default()
    ///     .itunes_ext(itunes_item)
    ///     .finalize();
    ///
    /// assert!(item.itunes_ext().is_some())
    /// ```
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let item = ItemBuilder::default()
    ///     .itunes_ext(None)
    ///     .finalize();
    ///
    /// assert!(item.itunes_ext().is_none());
    /// ```
    pub fn itunes_ext(&self) -> Option<&ITunesItemExtension> {
        self.itunes_ext.as_ref()
    }

    /// Return the `DublinCoreExtension` for this `Item`.
    pub fn dublin_core_ext(&self) -> Option<&DublinCoreExtension> {
        self.dublin_core_ext.as_ref()
    }

    /// Return the extensions for this `Item`.
    pub fn extensions(&self) -> &ExtensionMap {
        &self.extensions
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

/// A builder used to create an `Item`.
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
    /// Construct a new `ItemBuilder` using the values from an existing `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{Channel, ItemBuilder};
    ///
    /// let input = include_str!("tests/data/item.xml");
    /// let channel = input.parse::<Channel>().unwrap();
    /// let item = channel.items()[0].clone();
    /// let builder = ItemBuilder::from_item(item);
    /// ```
    pub fn from_item(item: Item) -> Self {
        ItemBuilder {
            title: item.title,
            link: item.link,
            description: item.description,
            author: item.author,
            categories: item.categories,
            comments: item.comments,
            enclosure: item.enclosure,
            guid: item.guid,
            pub_date: item.pub_date,
            source: item.source,
            extensions: item.extensions,
            itunes_ext: item.itunes_ext,
            dublin_core_ext: item.dublin_core_ext,
            content: item.content,
        }
    }

    /// Set the title of the `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let builder = ItemBuilder::default()
    ///     .title("Making Music with Linux | LAS 408".to_string());
    /// ```
    pub fn title<V>(mut self, title: V) -> ItemBuilder
        where V: Into<Option<String>>
    {
        self.title = title.into();
        self
    }

    /// Set the URL for the `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let builder = ItemBuilder::default()
    ///     .link("http://www.jupiterbroadcasting.com".to_string());
    /// ```
    pub fn link<V>(mut self, link: V) -> ItemBuilder
        where V: Into<Option<String>>
    {
        self.link = link.into();
        self
    }

    /// Set the description of this `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let builder = ItemBuilder::default()
    ///     .description("This is a test description".to_string());
    /// ```
    pub fn description<V>(mut self, description: V) -> ItemBuilder
        where V: Into<Option<String>>
    {
        self.description = description.into();
        self
    }

    /// Set the author of the `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let builder = ItemBuilder::default()
    ///     .author("Chris Fisher".to_string());
    /// ```
    pub fn author<V>(mut self, author: V) -> ItemBuilder
        where V: Into<Option<String>>
    {
        self.author = author.into();
        self
    }

    /// Set the categories that the `Item` belongs to.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{CategoryBuilder, ItemBuilder};
    ///
    /// let category = CategoryBuilder::default()
    ///     .finalize();
    ///
    /// let builder = ItemBuilder::default()
    ///     .categories(vec![category]);
    /// ```
    pub fn categories<V>(mut self, categories: V) -> ItemBuilder
        where V: Into<Vec<Category>>
    {
        self.categories = categories.into();
        self
    }

    /// Set the URL for comments about the `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let builder = ItemBuilder::default()
    ///     .comments("A comment".to_string());
    /// ```
    pub fn comments<V>(mut self, comments: V) -> ItemBuilder
        where V: Into<Option<String>>
    {
        self.comments = comments.into();
        self
    }

    /// Set the enclosure information for the `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{EnclosureBuilder, ItemBuilder};
    ///
    /// let url = "http://www.podtrac.com/pts/redirect.ogg/traffic.libsyn.com/jnite/\
    ///     linuxactionshowep408.ogg";
    ///
    /// let enclosure = EnclosureBuilder::default()
    ///     .url(url)
    ///     .mime_type("audio/ogg")
    ///     .finalize();
    ///
    /// let builder = ItemBuilder::default()
    ///     .enclosure(enclosure);
    /// ```
    pub fn enclosure<V>(mut self, enclosure: V) -> ItemBuilder
        where V: Into<Option<Enclosure>>
    {
        self.enclosure = enclosure.into();
        self
    }

    /// Set the GUID for the `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{GuidBuilder, ItemBuilder};
    ///
    /// let guid = GuidBuilder::default()
    ///     .finalize();
    ///
    /// let builder = ItemBuilder::default()
    ///     .guid(guid);
    /// ```
    pub fn guid<V>(mut self, guid: V) -> ItemBuilder
        where V: Into<Option<Guid>>
    {
        self.guid = guid.into();
        self
    }

    /// Set the publication date for the `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let builder = ItemBuilder::default()
    ///     .pub_date("Sun, 13 Mar 2016 20:02:02-0700".to_string());
    /// ```
    pub fn pub_date<V>(mut self, pub_date: V) -> ItemBuilder
        where V: Into<Option<String>>
    {
        self.pub_date = pub_date.into();
        self
    }

    /// Set the source URL for the `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ItemBuilder, SourceBuilder};
    ///
    /// let source = SourceBuilder::default()
    ///     .url("http://www.tomalak.org/links2.xml")
    ///     .finalize();
    ///
    /// let builder = ItemBuilder::default()
    ///     .source(source);
    /// ```
    pub fn source<V>(mut self, source: V) -> ItemBuilder
        where V: Into<Option<Source>>
    {
        self.source = source.into();
        self
    }

    /// Set the `ITunesItemExtension` for the `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let itunes_item = ITunesItemExtensionBuilder::default()
    ///     .author("author".to_string())
    ///     .finalize();
    ///
    /// let builder = ItemBuilder::default()
    ///     .itunes_ext(itunes_item);
    /// ```
    pub fn itunes_ext<V>(mut self, itunes_ext: V) -> ItemBuilder
        where V: Into<Option<ITunesItemExtension>>
    {
        self.itunes_ext = itunes_ext.into();
        self
    }

    /// Set the `DublinCoreExtension` for the `Item`.
    pub fn dublin_core_ext<V>(mut self, dublin_core_ext: V) -> ItemBuilder
        where V: Into<Option<DublinCoreExtension>>
    {
        self.dublin_core_ext = dublin_core_ext.into();
        self
    }

    /// Set the extensions for the `Item`.
    pub fn extensions<V>(mut self, extensions: V) -> ItemBuilder
        where V: Into<ExtensionMap>
    {
        self.extensions = extensions.into();
        self
    }

    /// Set the content of the `Item`.
    pub fn content<V>(mut self, content: V) -> ItemBuilder
        where V: Into<Option<String>>
    {
        self.content = content.into();
        self
    }

    /// Validate the contents of this `ItemBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let item = ItemBuilder::default()
    ///     .title("Making Music with Linux | LAS 408".to_string())
    ///     .link("http://www.jupiterbroadcasting.com".to_string())
    ///     .validate()
    ///     .unwrap();
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


    /// Construct the `Item` from this `ItemBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let item = ItemBuilder::default()
    ///     .title("Making Music with Linux | LAS 408".to_string())
    ///     .link("http://www.jupiterbroadcasting.com".to_string())
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
