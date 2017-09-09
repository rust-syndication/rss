// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::io::{BufRead, Write};

use quick_xml::errors::Error as XmlError;
use quick_xml::events::{Event, BytesStart, BytesEnd};
use quick_xml::events::attributes::Attributes;
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;

use category::Category;
use enclosure::Enclosure;
use error::Error;
use extension::ExtensionMap;
use extension::dublincore::DublinCoreExtension;
use extension::itunes::ITunesItemExtension;
use extension::util::{extension_name, parse_extension};
use fromxml::FromXml;
use guid::Guid;
use source::Source;
use toxml::{ToXml, WriterExt};
use util::element_text;

/// Represents an item in an RSS feed.
#[derive(Debug, Default, Clone, PartialEq, Builder)]
#[builder(setter(into), default)]
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
    /// The date the item was published as an RFC822 timestamp.
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
    /// Return the title of this item.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Item;
    ///
    /// let mut item = Item::default();
    /// item.set_title("Item Title".to_string());
    /// assert_eq!(item.title(), Some("Item Title"));
    /// ```
    pub fn title(&self) -> Option<&str> {
        self.title.as_ref().map(|s| s.as_str())
    }

    /// Set the title of this item.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Item;
    ///
    /// let mut item = Item::default();
    /// item.set_title("Item Title".to_string());
    /// ```
    pub fn set_title<V>(&mut self, title: V)
    where
        V: Into<Option<String>>,
    {
        self.title = title.into();
    }

    /// Return the URL of this item.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Item;
    ///
    /// let mut item = Item::default();
    /// item.set_link("http://example.com".to_string());
    /// assert_eq!(item.link(), Some("http://example.com"));
    /// ```
    pub fn link(&self) -> Option<&str> {
        self.link.as_ref().map(|s| s.as_str())
    }

    /// Set the URL of this item.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Item;
    ///
    /// let mut item = Item::default();
    /// item.set_link("http://example.com".to_string());
    /// ```
    pub fn set_link<V>(&mut self, link: V)
    where
        V: Into<Option<String>>,
    {
        self.link = link.into();
    }

    /// Return the description of this item.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Item;
    ///
    /// let mut item = Item::default();
    /// item.set_description("Item description".to_string());
    /// assert_eq!(item.description(), Some("Item description"));
    /// ```
    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|s| s.as_str())
    }

    /// Return the description of this item.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Item;
    ///
    /// let mut item = Item::default();
    /// item.set_description("Item description".to_string());
    /// ```
    pub fn set_description<V>(&mut self, description: V)
    where
        V: Into<Option<String>>,
    {
        self.description = description.into();
    }

    /// Return the email address for the author of this item.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Item;
    ///
    /// let mut item = Item::default();
    /// item.set_author("John Doe".to_string());
    /// assert_eq!(item.author(), Some("John Doe"));
    /// ```
    pub fn author(&self) -> Option<&str> {
        self.author.as_ref().map(|s| s.as_str())
    }

    /// Set the email address for the author of this item.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Item;
    ///
    /// let mut item = Item::default();
    /// item.set_author("John Doe".to_string());
    /// ```
    pub fn set_author<V>(&mut self, author: V)
    where
        V: Into<Option<String>>,
    {
        self.author = author.into();
    }

    /// Return the categories that this item belongs to.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{Category, Item};
    ///
    /// let mut item = Item::default();
    /// item.set_categories(vec![Category::default()]);
    /// assert_eq!(item.categories().len(), 1);
    /// ```
    pub fn categories(&self) -> &[Category] {
        &self.categories
    }

    /// Return a mutable slice of the categories that this item belongs to.
    pub fn categories_mut(&mut self) -> &mut [Category] {
        &mut self.categories
    }

    /// Set the categories that this item belongs to.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{Category, Item};
    ///
    /// let mut item = Item::default();
    /// item.set_categories(vec![Category::default()]);
    /// ```
    pub fn set_categories<V>(&mut self, categories: V)
    where
        V: Into<Vec<Category>>,
    {
        self.categories = categories.into();
    }

    /// Return the URL for comments about this item.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Item;
    ///
    /// let mut item = Item::default();
    /// item.set_comments("http://example.com".to_string());
    /// assert_eq!(item.comments(), Some("http://example.com"));
    /// ```
    pub fn comments(&self) -> Option<&str> {
        self.comments.as_ref().map(|s| s.as_str())
    }

    /// Set the URL for comments about this item.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Item;
    ///
    /// let mut item = Item::default();
    /// item.set_comments("http://example.com".to_string());
    /// ```
    pub fn set_comments<V>(&mut self, comments: V)
    where
        V: Into<Option<String>>,
    {
        self.comments = comments.into();
    }

    /// Return the enclosure information for this item.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{Enclosure, Item};
    ///
    /// let mut item = Item::default();
    /// item.set_enclosure(Enclosure::default());
    /// assert!(item.enclosure().is_some());
    /// ```
    pub fn enclosure(&self) -> Option<&Enclosure> {
        self.enclosure.as_ref()
    }

    /// Set the enclosure information for this item.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{Enclosure, Item};
    ///
    /// let mut item = Item::default();
    /// item.set_enclosure(Enclosure::default());
    /// ```
    pub fn set_enclosure<V>(&mut self, enclosure: V)
    where
        V: Into<Option<Enclosure>>,
    {
        self.enclosure = enclosure.into();
    }

    /// Return the GUID for this item.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{Guid, Item};
    ///
    /// let mut item = Item::default();
    /// item.set_guid(Guid::default());
    /// assert!(item.guid().is_some())
    /// ```
    pub fn guid(&self) -> Option<&Guid> {
        self.guid.as_ref()
    }

    /// Set the GUID for this item.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{Guid, Item};
    ///
    /// let mut item = Item::default();
    /// item.set_guid(Guid::default());
    /// ```
    pub fn set_guid<V>(&mut self, guid: V)
    where
        V: Into<Option<Guid>>,
    {
        self.guid = guid.into();
    }

    /// Return the publication date of this item as an RFC822 timestamp.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Item;
    ///
    /// let mut item = Item::default();
    /// item.set_pub_date("Mon, 01 Jan 2017 12:00:00 GMT".to_string());
    /// assert_eq!(item.pub_date(), Some("Mon, 01 Jan 2017 12:00:00 GMT"));
    /// ```
    pub fn pub_date(&self) -> Option<&str> {
        self.pub_date.as_ref().map(|s| s.as_str())
    }

    /// Set the publication date of this item as an RFC822 timestamp.
    ///
    /// # Examples
    ///
    ///
    /// ```
    /// use rss::Item;
    ///
    /// let mut item = Item::default();
    /// item.set_pub_date("Mon, 01 Jan 2017 12:00:00 GMT".to_string());
    /// ```
    pub fn set_pub_date<V>(&mut self, pub_date: V)
    where
        V: Into<Option<String>>,
    {
        self.pub_date = pub_date.into();
    }

    /// Return the source URL for this item.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{Item, Source};
    ///
    /// let mut item = Item::default();
    /// item.set_source(Source::default());
    /// assert!(item.source().is_some());
    /// ```
    pub fn source(&self) -> Option<&Source> {
        self.source.as_ref()
    }

    /// Set the source URL for this item.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{Item, Source};
    ///
    /// let mut item = Item::default();
    /// item.set_source(Source::default());
    /// ```
    pub fn set_source<V>(&mut self, source: V)
    where
        V: Into<Option<Source>>,
    {
        self.source = source.into();
    }

    /// Return the content of this item.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Item;
    ///
    /// let mut item = Item::default();
    /// item.set_content("Item content".to_string());
    /// assert_eq!(item.content(), Some("Item content"));
    /// ```
    pub fn content(&self) -> Option<&str> {
        self.content.as_ref().map(|s| s.as_str())
    }

    /// Set the content of this item.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Item;
    ///
    /// let mut item = Item::default();
    /// item.set_content("Item content".to_string());
    /// ```
    pub fn set_content<V>(&mut self, content: V)
    where
        V: Into<Option<String>>,
    {
        self.content = content.into();
    }

    /// Return the iTunes extension for this item.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Item;
    /// use rss::extension::itunes::ITunesItemExtension;
    ///
    /// let mut item = Item::default();
    /// item.set_itunes_ext(ITunesItemExtension::default());
    /// assert!(item.itunes_ext().is_some());
    /// ```
    pub fn itunes_ext(&self) -> Option<&ITunesItemExtension> {
        self.itunes_ext.as_ref()
    }

    /// Set the iTunes extension for this item.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Item;
    /// use rss::extension::itunes::ITunesItemExtension;
    ///
    /// let mut item = Item::default();
    /// item.set_itunes_ext(ITunesItemExtension::default());
    /// ```
    pub fn set_itunes_ext<V>(&mut self, itunes_ext: V)
    where
        V: Into<Option<ITunesItemExtension>>,
    {
        self.itunes_ext = itunes_ext.into();
    }

    /// Return the Dublin Core extension for this item.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Item;
    /// use rss::extension::dublincore::DublinCoreExtension;
    ///
    /// let mut item = Item::default();
    /// item.set_dublin_core_ext(DublinCoreExtension::default());
    /// assert!(item.dublin_core_ext().is_some());
    /// ```
    pub fn dublin_core_ext(&self) -> Option<&DublinCoreExtension> {
        self.dublin_core_ext.as_ref()
    }

    /// Set the Dublin Core extension for this item.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Item;
    /// use rss::extension::dublincore::DublinCoreExtension;
    ///
    /// let mut item = Item::default();
    /// item.set_dublin_core_ext(DublinCoreExtension::default());
    /// ```
    pub fn set_dublin_core_ext<V>(&mut self, dublin_core_ext: V)
    where
        V: Into<Option<DublinCoreExtension>>,
    {
        self.dublin_core_ext = dublin_core_ext.into();
    }

    /// Return the extensions for this item.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use rss::Item;
    /// use rss::extension::{ExtensionMap, Extension};
    ///
    /// let extension = Extension::default();
    ///
    /// let mut item_map = HashMap::<String, Vec<Extension>>::new();
    /// item_map.insert("ext:name".to_string(), vec![extension]);
    ///
    /// let mut extension_map = ExtensionMap::default();
    /// extension_map.insert("ext".to_string(), item_map);
    ///
    /// let mut item = Item::default();
    /// item.set_extensions(extension_map);
    /// assert_eq!(item.extensions()
    ///                .get("ext")
    ///                .and_then(|m| m.get("ext:name"))
    ///                .map(|v| v.len()),
    ///            Some(1));
    /// ```
    pub fn extensions(&self) -> &ExtensionMap {
        &self.extensions
    }

    /// Set the extensions for this item.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Item;
    /// use rss::extension::ExtensionMap;
    ///
    /// let mut item = Item::default();
    /// item.set_extensions(ExtensionMap::default());
    /// ```
    pub fn set_extensions<V>(&mut self, extensions: V)
    where
        V: Into<ExtensionMap>,
    {
        self.extensions = extensions.into();
    }
}

impl FromXml for Item {
    fn from_xml<R: BufRead>(reader: &mut Reader<R>, _: Attributes) -> Result<Self, Error> {
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
                            if let Some((ns, name)) = extension_name(n) {
                                parse_extension(
                                    reader,
                                    element.attributes(),
                                    ns,
                                    name,
                                    &mut item.extensions,
                                )?;
                            } else {
                                reader.read_to_end(n, &mut Vec::new())?;
                            }
                        }
                    }
                }
                Event::End(_) => break,
                Event::Eof => return Err(Error::Eof),
                _ => {}
            }
            buf.clear();
        }

        if !item.extensions.is_empty() {
            if let Some(map) = item.extensions.remove("itunes") {
                item.itunes_ext = Some(ITunesItemExtension::from_map(map));
            }

            if let Some(map) = item.extensions.remove("dc") {
                item.dublin_core_ext = Some(DublinCoreExtension::from_map(map));
            }
        }

        Ok(item)
    }
}

impl ToXml for Item {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
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
