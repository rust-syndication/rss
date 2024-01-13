// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::collections::BTreeMap;
use std::io::{BufRead, Write};

use quick_xml::events::attributes::Attributes;
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::Error as XmlError;
use quick_xml::Reader;
use quick_xml::Writer;

use crate::category::Category;
use crate::enclosure::Enclosure;
use crate::error::Error;
#[cfg(feature = "atom")]
use crate::extension::atom;
use crate::extension::dublincore;
use crate::extension::itunes::{self, is_itunes_namespace};
use crate::extension::util::{
    extension_entry, extension_name, parse_extension_element, read_namespace_declarations,
};
use crate::extension::ExtensionMap;
use crate::guid::Guid;
use crate::source::Source;
use crate::toxml::{ToXml, WriterExt};
use crate::util::{decode, element_text, skip};

/// Represents an item in an RSS feed.
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
pub struct Item {
    /// The title of the item.
    pub title: Option<String>,
    /// The URL of the item.
    pub link: Option<String>,
    /// The item synopsis.
    pub description: Option<String>,
    /// The email address of author of the item.
    pub author: Option<String>,
    /// The categories the item belongs to.
    #[cfg_attr(feature = "builders", builder(setter(each = "category")))]
    pub categories: Vec<Category>,
    /// The URL for the comments page of the item.
    pub comments: Option<String>,
    /// The description of a media object that is attached to the item.
    pub enclosure: Option<Enclosure>,
    /// A unique identifier for the item.
    pub guid: Option<Guid>,
    /// The date the item was published as an RFC 2822 timestamp.
    pub pub_date: Option<String>,
    /// The RSS channel the item came from.
    pub source: Option<Source>,
    /// The HTML contents of the item.
    pub content: Option<String>,
    /// The extensions for the item.
    #[cfg_attr(feature = "builders", builder(setter(each = "extension")))]
    pub extensions: ExtensionMap,
    /// The Atom extension for the channel.
    #[cfg(feature = "atom")]
    pub atom_ext: Option<atom::AtomExtension>,
    /// The iTunes extension for the item.
    pub itunes_ext: Option<itunes::ITunesItemExtension>,
    /// The Dublin Core extension for the item.
    pub dublin_core_ext: Option<dublincore::DublinCoreExtension>,
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
        self.title.as_deref()
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
        self.link.as_deref()
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
        self.description.as_deref()
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
        self.author.as_deref()
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
        self.comments.as_deref()
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

    /// Return the publication date of this item as an RFC 2822 timestamp.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Item;
    ///
    /// let mut item = Item::default();
    /// item.set_pub_date("Sun, 01 Jan 2017 12:00:00 GMT".to_string());
    /// assert_eq!(item.pub_date(), Some("Sun, 01 Jan 2017 12:00:00 GMT"));
    /// ```
    pub fn pub_date(&self) -> Option<&str> {
        self.pub_date.as_deref()
    }

    /// Set the publication date of this item as an RFC 2822 timestamp.
    ///
    /// # Examples
    ///
    ///
    /// ```
    /// use rss::Item;
    ///
    /// let mut item = Item::default();
    /// item.set_pub_date("Sun, 01 Jan 2017 12:00:00 GMT".to_string());
    /// assert_eq!(item.pub_date(), Some("Sun, 01 Jan 2017 12:00:00 GMT"));
    /// ```
    ///
    /// ## Using chrono::DateTime
    /// ```
    /// # #[cfg(feature = "validation")]
    /// # {
    /// use rss::Item;
    /// use chrono::{FixedOffset, TimeZone, Utc};
    ///
    /// let mut item = Item::default();
    /// item.set_pub_date(Utc.with_ymd_and_hms(2017, 1, 1, 12, 0, 0).unwrap().to_rfc2822());
    /// assert_eq!(item.pub_date(), Some("Sun, 1 Jan 2017 12:00:00 +0000"));
    ///
    /// item.set_pub_date(FixedOffset::east_opt(2 * 3600).unwrap().with_ymd_and_hms(2017, 1, 1, 12, 0, 0).unwrap().to_rfc2822());
    /// assert_eq!(item.pub_date(), Some("Sun, 1 Jan 2017 12:00:00 +0200"));
    /// # }
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
        self.content.as_deref()
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

    /// Return the Atom extension for this item.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Item;
    /// use rss::extension::atom::AtomExtension;
    ///
    /// let mut item = Item::default();
    /// item.set_atom_ext(AtomExtension::default());
    /// assert!(item.atom_ext().is_some());
    /// ```
    #[cfg(feature = "atom")]
    pub fn atom_ext(&self) -> Option<&atom::AtomExtension> {
        self.atom_ext.as_ref()
    }

    /// Set the Atom extension for this item.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Item;
    /// use rss::extension::atom::AtomExtension;
    ///
    /// let mut item = Item::default();
    /// item.set_atom_ext(AtomExtension::default());
    /// ```
    #[cfg(feature = "atom")]
    pub fn set_atom_ext<V>(&mut self, atom_ext: V)
    where
        V: Into<Option<atom::AtomExtension>>,
    {
        self.atom_ext = atom_ext.into();
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
    pub fn itunes_ext(&self) -> Option<&itunes::ITunesItemExtension> {
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
        V: Into<Option<itunes::ITunesItemExtension>>,
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
    pub fn dublin_core_ext(&self) -> Option<&dublincore::DublinCoreExtension> {
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
        V: Into<Option<dublincore::DublinCoreExtension>>,
    {
        self.dublin_core_ext = dublin_core_ext.into();
    }

    /// Return the extensions for this item.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeMap;
    /// use rss::Item;
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

impl Item {
    /// Builds an Item from source XML
    pub fn from_xml<R: BufRead>(
        namespaces: &BTreeMap<String, String>,
        reader: &mut Reader<R>,
        atts: Attributes,
    ) -> Result<Self, Error> {
        let mut item = Item::default();
        let mut extensions = ExtensionMap::new();
        let mut buf = Vec::new();

        let namespaces = read_namespace_declarations(reader, atts, namespaces)?;

        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(element) => match decode(element.name().as_ref(), reader)?.as_ref() {
                    "category" => {
                        let category = Category::from_xml(reader, element.attributes())?;
                        item.categories.push(category);
                    }
                    "guid" => {
                        let guid = Guid::from_xml(reader, element.attributes())?;
                        item.guid = Some(guid);
                    }
                    "enclosure" => item.enclosure = Some(Enclosure::from_xml(reader, &element)?),
                    "source" => {
                        let source = Source::from_xml(reader, element.attributes())?;
                        item.source = Some(source);
                    }
                    "title" => item.title = element_text(reader)?,
                    "link" => {
                        if let Some(link) = element_text(reader)?.filter(|text| !text.is_empty()) {
                            item.link = Some(link);
                        }
                    }
                    "description" => item.description = element_text(reader)?,
                    "author" => item.author = element_text(reader)?,
                    "comments" => item.comments = element_text(reader)?,
                    "pubDate" => item.pub_date = element_text(reader)?,
                    "content:encoded" => item.content = element_text(reader)?,
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
                                Some(ns @ dublincore::NAMESPACE) => {
                                    extension_entry(&mut extensions, ns, name).push(ext);
                                }
                                _ => extension_entry(&mut item.extensions, prefix, name).push(ext),
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
            item.atom_ext = Some(atom::AtomExtension::from_map(v));
        }
        if let Some(v) = extensions.remove(itunes::NAMESPACE) {
            item.itunes_ext = Some(itunes::ITunesItemExtension::from_map(v))
        }
        if let Some(v) = extensions.remove(dublincore::NAMESPACE) {
            item.dublin_core_ext = Some(dublincore::DublinCoreExtension::from_map(v))
        }

        Ok(item)
    }
}

impl ToXml for Item {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = "item";

        writer.write_event(Event::Start(BytesStart::new(name)))?;

        if let Some(title) = self.title.as_ref() {
            writer.write_text_element("title", title)?;
        }

        if let Some(link) = self.link.as_ref() {
            writer.write_text_element("link", link)?;
        }

        if let Some(description) = self.description.as_ref() {
            writer.write_cdata_element("description", description)?;
        }

        if let Some(author) = self.author.as_ref() {
            writer.write_text_element("author", author)?;
        }

        writer.write_objects(&self.categories)?;

        if let Some(comments) = self.comments.as_ref() {
            writer.write_text_element("comments", comments)?;
        }

        if let Some(enclosure) = self.enclosure.as_ref() {
            writer.write_object(enclosure)?;
        }

        if let Some(guid) = self.guid.as_ref() {
            writer.write_object(guid)?;
        }

        if let Some(pub_date) = self.pub_date.as_ref() {
            writer.write_text_element("pubDate", pub_date)?;
        }

        if let Some(source) = self.source.as_ref() {
            writer.write_object(source)?;
        }

        if let Some(content) = self.content.as_ref() {
            writer.write_cdata_element("content:encoded", content)?;
        }

        for map in self.extensions.values() {
            for extensions in map.values() {
                for extension in extensions {
                    extension.to_xml(writer)?;
                }
            }
        }

        #[cfg(feature = "atom")]
        if let Some(ext) = self.atom_ext.as_ref() {
            ext.to_xml(writer)?;
        }

        if let Some(ext) = self.itunes_ext.as_ref() {
            ext.to_xml(writer)?;
        }

        if let Some(ext) = self.dublin_core_ext.as_ref() {
            ext.to_xml(writer)?;
        }

        writer.write_event(Event::End(BytesEnd::new(name)))?;
        Ok(())
    }

    fn used_namespaces(&self) -> BTreeMap<String, String> {
        let mut namespaces = BTreeMap::new();
        if self.content.is_some() {
            namespaces.insert(
                "content".to_owned(),
                "http://purl.org/rss/1.0/modules/content/".to_owned(),
            );
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

#[cfg(feature = "builders")]
impl ItemBuilder {
    /// Builds a new `Item`.
    pub fn build(&self) -> Item {
        self.build_impl().unwrap()
    }
}
