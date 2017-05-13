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
use quick_xml::{Element, Event, XmlReader, XmlWriter};
use quick_xml::error::Error as XmlError;
use source::Source;
use toxml::{ToXml, XmlWriterExt};
use url::Url;

/// A representation of the `<item>` element.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Item
{
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

impl Item
{
    /// Get the optional title that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let title_string = "Making Music with Linux | LAS 408".to_owned();
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some(title_string.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let title_option = item.title();
    /// assert!(title_option.is_some());
    ///
    /// assert_eq!(title_string.clone(), title_option.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let item = ItemBuilder::new()
    ///     .title(None)
    ///     .description(Some("A Test Description".to_owned()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(item.title().is_none());
    /// ```
    pub fn title(&self) -> Option<String>
    {
        self.title
            .clone()
    }

    /// Get the optional link that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let link_string = "http://www.jupiterbroadcasting.com/".to_owned();
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .link(Some(link_string.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let link_option = item.link();
    /// assert!(link_option.is_some());
    ///
    /// assert_eq!(link_string.clone(), link_option.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .link(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(item.link().is_none());
    /// ```
    pub fn link(&self) -> Option<String>
    {
        self.link
            .clone()
    }

    /// Get the optional description that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let description_string = "This is a test description".to_owned();
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .description(Some(description_string.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let description_option = item.description();
    /// assert!(description_option.is_some());
    ///
    /// assert_eq!(description_string.clone(), description_option.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .description(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(item.description().is_none());
    /// ```
    pub fn description(&self) -> Option<String>
    {
        self.description
            .clone()
    }

    /// Get the optional author that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let author_string = "Chris Fisher".to_owned();
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .author(Some(author_string.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let author_option = item.author();
    /// assert!(author_option.is_some());
    ///
    /// assert_eq!(author_string.clone(), author_option.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .author(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(item.author().is_none());
    /// ```
    pub fn author(&self) -> Option<String>
    {
        self.author
            .clone()
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
    ///     .domain(Some("http://jupiterbroadcasting.com".to_owned()))
    ///     .name("Podcast")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let categories_vec = vec![category_1, category_2];
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .categories(categories_vec.clone())
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(categories_vec.clone().len(), item.categories().len());
    /// ```
    pub fn categories(&self) -> Vec<Category>
    {
        self.categories
            .clone()
    }

    /// Get the optional comments that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let comments_string = "http://example.com/comments".to_owned();
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .comments(Some(comments_string.clone()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let comments_option =  item.comments();
    /// assert!(comments_option.is_some());
    ///
    /// assert_eq!(comments_string.clone(), comments_option.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .comments(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(item.comments().is_none());
    /// ```
    pub fn comments(&self) -> Option<String>
    {
        self.comments
            .clone()
    }

    /// Get the optional enclosure that exists under `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{EnclosureBuilder, ItemBuilder, Item};
    ///
    /// let url = "http://www.podtrac.com/pts/redirect.ogg/".to_owned()
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
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
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
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .enclosure(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(item.enclosure().is_none());
    /// ```
    pub fn enclosure(&self) -> Option<Enclosure>
    {
        self.enclosure
            .clone()
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
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
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
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .guid(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(item.guid().is_none());
    /// ```
    pub fn guid(&self) -> Option<Guid>
    {
        self.guid
            .clone()
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
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .pub_date(Some(pub_date.to_owned()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let local = item.pub_date();
    /// assert!(local.is_some());
    ///
    /// assert_eq!(pub_date.to_owned(), local.unwrap());
    /// ```
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .pub_date(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(item.pub_date().is_none());
    /// ```
    pub fn pub_date(&self) -> Option<String>
    {
        self.pub_date
            .clone()
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
    ///     .title(Some("Tomalak's Realm".to_owned()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
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
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .source(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(item.source().is_none());
    /// ```
    pub fn source(&self) -> Option<Source>
    {
        self.source
            .clone()
    }

    /// Get the optional `ITunesItemExtension` under `Item`.
    /// # Examples
    ///
    /// ```
    /// use rss::{ItemBuilder, Item};
    /// use rss::extension::itunes::ITunesItemExtensionBuilder;
    ///
    /// let itunes_item = ITunesItemExtensionBuilder::new()
    ///     .author(Some("author".to_owned()))
    ///     .block(Some("block".to_owned()))
    ///     .image(Some("image".to_owned()))
    ///     .duration(Some("duration".to_owned()))
    ///     .explicit(Some("explicit".to_owned()))
    ///     .closed_captioned(Some("closed_captioned".to_owned()))
    ///     .order(Some("order".to_owned()))
    ///     .subtitle(Some("subtitle".to_owned()))
    ///     .summary(Some("summary".to_owned()))
    ///     .keywords(Some("keywords".to_owned()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
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
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .itunes_ext(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert!(item.itunes_ext().is_none());
    /// ```
    pub fn itunes_ext(&self) -> Option<ITunesItemExtension>
    {
        self.itunes_ext
            .clone()
    }

    /// TODO: document dublincore getter
    pub fn dublin_core_ext(&self) -> Option<DublinCoreExtension>
    {
        self.dublin_core_ext
            .clone()
    }

    /// TODO: document extensions getter
    pub fn extensions(&self) -> ExtensionMap
    {
        self.extensions
            .clone()
    }

    /// TODO: document content getter
    pub fn content(&self) -> Option<String>
    {
        self.content
            .clone()
    }
}

impl FromXml for Item
{
    /// TODO: document from xml
    fn from_xml<R: ::std::io::BufRead>(mut reader: XmlReader<R>,
                                       _: Element)
        -> Result<(Self, XmlReader<R>), Error>
    {
        let mut item = Item::default();

        while let Some(e) = reader.next() {
            match e {
                Ok(Event::Start(element)) => {
                    match element.name() {
                        b"category" => {
                            let (category, reader_) = Category::from_xml(reader,
                                                                         element)?;
                            reader = reader_;
                            item.categories
                                .push(category);
                        },
                        b"guid" => {
                            let (guid, reader_) = Guid::from_xml(reader,
                                                                 element)?;
                            reader = reader_;
                            item.guid = Some(guid);
                        },
                        b"enclosure" => {
                            let (enclosure, reader_) = Enclosure::from_xml(reader,
                                                                           element)?;
                            reader = reader_;
                            item.enclosure = Some(enclosure);
                        },
                        b"source" => {
                            let (source, reader_) = Source::from_xml(reader,
                                                                     element)?;
                            reader = reader_;
                            item.source = Some(source);
                        },
                        b"title" => item.title = element_text!(reader),
                        b"link" => item.link = element_text!(reader),
                        b"description" => item.description = element_text!(reader),
                        b"author" => item.author = element_text!(reader),
                        b"comments" => item.comments = element_text!(reader),
                        b"pubDate" => item.pub_date = element_text!(reader),
                        b"content:encoded" => item.content = element_text!(reader),
                        _ => {
                            if let Some((ns, name)) = fromxml::extension_name(&element) {
                                parse_extension!(reader,
                                                 element,
                                                 ns,
                                                 name,
                                                 item.extensions);
                            } else {
                                skip_element!(reader);
                            }
                        },
                    }
                },
                Ok(Event::End(_)) => {
                    if !item.extensions
                            .is_empty() {
                        if let Some(map) = item.extensions
                                               .remove("itunes") {
                            item.itunes_ext = Some(ITunesItemExtension::from_map(map));
                        }

                        if let Some(map) = item.extensions
                                               .remove("dc") {
                            item.dublin_core_ext = Some(DublinCoreExtension::from_map(map));
                        }
                    }

                    return Ok((item, reader));
                },
                Err(err) => return Err(err.into()),
                _ => {},
            }
        }

        Err(Error::EOF)
    }
}

impl ToXml for Item
{
    /// TODO: document to xml
    fn to_xml<W: ::std::io::Write>(&self,
                                   writer: &mut XmlWriter<W>)
        -> Result<(), XmlError>
    {
        let element = Element::new(b"item");

        writer.write(Event::Start(element.clone()))?;

        if let Some(title) = self.title
                                 .as_ref() {
            writer.write_text_element(b"title",
                                      title)?;
        }

        if let Some(link) = self.link
                                .as_ref() {
            writer.write_text_element(b"link",
                                      link)?;
        }

        if let Some(description) =
            self.description
                .as_ref() {
            writer.write_text_element(b"description",
                                      description)?;
        }

        if let Some(author) = self.author
                                  .as_ref() {
            writer.write_text_element(b"author",
                                      author)?;
        }

        writer.write_objects(&self.categories)?;

        if let Some(comments) = self.comments
                                    .as_ref() {
            writer.write_text_element(b"comments",
                                      comments)?;
        }

        if let Some(enclosure) = self.enclosure
                                     .as_ref() {
            writer.write_object(enclosure)?;
        }

        if let Some(guid) = self.guid
                                .as_ref() {
            writer.write_object(guid)?;
        }

        if let Some(pub_date) = self.pub_date
                                    .as_ref() {
            writer.write_text_element(b"pubDate",
                                      pub_date)?;
        }

        if let Some(source) = self.source
                                  .as_ref() {
            writer.write_object(source)?;
        }

        if let Some(content) = self.content
                                   .as_ref() {
            writer.write_cdata_element(b"content:encoded",
                                       content)?;
        }

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

/// This `ItemBuilder` struct creates the `Item`.
#[derive(Debug, Clone, Default)]
pub struct ItemBuilder
{
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

impl ItemBuilder
{
    /// Construct a new `ItemBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let item_builder = ItemBuilder::new();
    /// ```
    pub fn new() -> ItemBuilder
    {
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
    /// 408".to_owned()));
    /// ```
    pub fn title(&mut self,
                 title: Option<String>)
        -> &mut ItemBuilder
    {
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
    pub fn link(&mut self,
                link: Option<String>)
        -> &mut ItemBuilder
    {
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
    /// item_builder.description(Some("This is a test description".to_owned()));
    /// ```
    pub fn description(&mut self,
                       description: Option<String>)
        -> &mut ItemBuilder
    {
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
    /// item_builder.author(Some("Chris Fisher".to_owned()));
    /// ```
    pub fn author(&mut self,
                  author: Option<String>)
        -> &mut ItemBuilder
    {
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
    pub fn categories(&mut self,
                      categories: Vec<Category>)
        -> &mut ItemBuilder
    {
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
    /// item_builder.comments(Some("Test Comment".to_owned()));
    /// ```
    pub fn comments(&mut self,
                    comments: Option<String>)
        -> &mut ItemBuilder
    {
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
    /// let url = "http://www.podtrac.com/pts/redirect.ogg/".to_owned()
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
    pub fn enclosure(&mut self,
                     enclosure: Option<Enclosure>)
        -> &mut ItemBuilder
    {
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
    pub fn guid(&mut self,
                guid: Option<Guid>)
        -> &mut ItemBuilder
    {
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
    /// 20:02:02-0700".to_owned()));
    /// ```
    pub fn pub_date(&mut self,
                    pub_date: Option<String>)
        -> &mut ItemBuilder
    {
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
    pub fn source(&mut self,
                  source: Option<Source>)
        -> &mut ItemBuilder
    {
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
    ///     .author(Some("author".to_owned()))
    ///     .block(Some("block".to_owned()))
    ///     .image(Some("image".to_owned()))
    ///     .duration(Some("duration".to_owned()))
    ///     .explicit(Some("explicit".to_owned()))
    ///     .closed_captioned(Some("closed_captioned".to_owned()))
    ///     .order(Some("order".to_owned()))
    ///     .subtitle(Some("subtitle".to_owned()))
    ///     .summary(Some("summary".to_owned()))
    ///     .keywords(Some("keywords".to_owned()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let mut item_builder = ItemBuilder::new();
    /// item_builder.itunes_ext(Some(itunes_item));
    /// ```
    pub fn itunes_ext(&mut self,
                      itunes_ext: Option<ITunesItemExtension>)
        -> &mut ItemBuilder
    {
        self.itunes_ext = itunes_ext;
        self
    }

    /// Set the optional dublin_core_ext that exists under `Item`.
    /// TODO: Add Example
    pub fn dublin_core_ext(&mut self,
                           dublin_core_ext: Option<DublinCoreExtension>)
        -> &mut ItemBuilder
    {
        self.dublin_core_ext = dublin_core_ext;
        self
    }

    /// Set the extensions that exists under `Item`.
    /// TODO: Add Example
    pub fn extensions(&mut self,
                      extensions: ExtensionMap)
        -> &mut ItemBuilder
    {
        self.extensions = extensions;
        self
    }

    /// Set the optional content that exists under `Item`.
    /// TODO: Add Example
    pub fn content(&mut self,
                   content: Option<String>)
        -> &mut ItemBuilder
    {
        self.content = content;
        self
    }

    // TODO: add dublincore, extensions, content to builder

    /// Validate the contents of `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ItemBuilder;
    ///
    /// let item = ItemBuilder::new()
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .link(Some("http://www.jupiterbroadcasting.com".to_owned()))
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
    pub fn validate(&mut self) -> Result<&mut ItemBuilder, Error>
    {
        if self.title
               .is_none() &&
           self.description
               .is_none() {
            return Err(Error::Validation(String::from("Either Title or Description must have a value.")));
        }

        let link = self.link
                       .clone();
        if link.is_some() {
            Url::parse(link.unwrap()
                           .as_str())?;
        }

        let comments = self.comments
                           .clone();
        if comments.is_some() {
            Url::parse(comments.unwrap()
                               .as_str())?;
        }

        let pub_date = self.pub_date
                           .clone();
        if pub_date.is_some() {
            DateTime::parse_from_rfc2822(pub_date.unwrap()
                                                 .as_str())?;
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
    ///     .title(Some("Making Music with Linux | LAS 408".to_owned()))
    ///     .link(Some("http://www.jupiterbroadcasting.com".to_owned()))
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
    pub fn finalize(&self) -> Result<Item, Error>
    {
        Ok(Item { title: self.title
                             .clone(),
                  link: self.link
                            .clone(),
                  description: self.description
                                   .clone(),
                  author: self.author
                              .clone(),
                  categories: self.categories
                                  .clone(),
                  comments: self.comments
                                .clone(),
                  enclosure: self.enclosure
                                 .clone(),
                  guid: self.guid
                            .clone(),
                  pub_date: self.pub_date
                                .clone(),
                  source: self.source
                              .clone(),
                  extensions: self.extensions
                                  .clone(),
                  itunes_ext: self.itunes_ext
                                  .clone(),
                  dublin_core_ext: self.dublin_core_ext
                                       .clone(),
                  content: self.content
                               .clone(), })
    }
}
