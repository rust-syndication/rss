use fromxml::{FromXml, XmlName};
use error::Error;
use guid::{Guid, GuidBuilder};
use enclosure::{Enclosure, EnclosureBuilder};
use source::{Source, SourceBuilder};

/// A representation of the `<item>` element.
pub struct Item {
    /// The title of the item. This is the content of the `<title>` element inside the `<item>`.
    pub title: Option<String>,
    /// The URL of the item. This is the content of the `<link>` element inside the `<item>`.
    pub link: Option<String>,
    /// The item synopsis. This is the content of the `<description>` element inside the `<item>`.
    pub description: Option<String>,
    /// The email address of author of the item. This is the content of the `<author>` element inside the `<item>`.
    pub author: Option<String>,
    /// The categories the item belongs to. This is the content of all of the `<category>` elements inside the `<item>`.
    pub categories: Vec<String>,
    /// The URL for the comments page of the item. This is the content of the `<comments>` element inside the `<item>`.
    pub comments: Option<String>,
    /// The description of a media object that is attached to the item. This is the content of the `<enclosure>` element inside the `<item>`.
    pub enclosure: Option<Enclosure>,
    /// A string that uniquely identifies the item. This is the content of the `<guid>` element inside the `<item>`.
    pub guid: Option<Guid>,
    /// The date the item was published. This is the content of the `<pubDate>` element inside the `<item>`.
    pub pub_date: Option<String>,
    /// The RSS channel the item came from. This is the content of the `<source>` element inside the `<item>`.
    pub source: Option<Source>,
    /// The HTML contents of the item. This is the content of the `<content:encoded>` element inside the `<item>`.
    pub content: Option<String>,
}

#[derive(Default)]
pub struct ItemBuilder {
    pub title: Option<String>,
    pub link: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub categories: Vec<String>,
    pub comments: Option<String>,
    pub enclosure: Option<EnclosureBuilder>,
    pub guid: Option<GuidBuilder>,
    pub pub_date: Option<String>,
    pub source: Option<SourceBuilder>,
    pub content: Option<String>,
}

impl ItemBuilder {
    #[inline]
    pub fn new() -> ItemBuilder {
        Default::default()
    }

    pub fn build(self) -> Result<Item, Error> {
        let enclosure = match self.enclosure {
            Some(value) => value.build().ok(),
            None => None,
        };

        let guid = match self.guid {
            Some(value) => Some(try!(value.build())),
            None => None,
        };

        let source = match self.source {
            Some(value) => Some(try!(value.build())),
            None => None,
        };


        Ok(Item {
            title: self.title,
            link: self.link,
            description: self.description,
            author: self.author,
            categories: self.categories,
            comments: self.comments,
            enclosure: enclosure,
            guid: guid,
            pub_date: self.pub_date,
            source: source,
            content: self.content,
        })
    }
}

impl FromXml for ItemBuilder {
    fn consume_named<T: XmlName>(&mut self, name: T, content: String) {
        match name.local_name() {
            b"title" => self.title = Some(content),
            b"link" => self.link = Some(content),
            b"description" => self.description = Some(content),
            b"author" => self.author = Some(content),
            b"category" => self.categories.push(content),
            b"comments" => self.comments = Some(content),
            b"pubDate" => self.pub_date = Some(content),
            #[cfg(feature = "quick-xml")]
            b"content:encoded" => {
                self.content = Some(content);
            }
            #[cfg(feature = "xml-rs")]
            b"encoded" if name.prefix() == Some(b"content") => {
                self.content = Some(content);
            }
            _ => {}
        }
    }
}

