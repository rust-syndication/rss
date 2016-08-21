use std::str::FromStr;

use fromxml::{FromXml, XmlName};
use error::Error;
use item::Item;

/// A representation of the `<channel>` element.
pub struct Channel {
    /// The name of the channel. This is the content of the `<title>` element inside the `<channel>`.
    pub title: String,
    /// The URL for the website corresponding to the channel. This is the content of the `<link>` element inside the `<channel>`.
    pub link: String,
    /// The description of the channel. This is the content of the `<description>` element inside the `<channel>`.
    pub description: String,
    /// The language of the channel. This is the content of the `<language>` element inside the `<channel>`.
    pub language: Option<String>,
    /// The publication date for the content of the channel. This is the content of the `<pubDate>` element inside the `<channel>`.
    pub pub_date: Option<String>,
    /// The date that the contents of the channel last changed. This is the content of the `<lastBuildDate>` element inside the `<channel>`.
    pub last_build_date: Option<String>,
    /// The categories the channel belongs to. This is the content of all of the `<category>` elements inside the `<channel>`.
    pub categories: Vec<String>,
    /// The items in the channel. This is the content of all of the `<item>` elements inside the `<channel>`.
    pub items: Vec<Item>,
}

impl Channel {
    #[cfg(feature = "quick-xml")]
    #[inline]
    /// Attempt to read the RSS channel from the speficied reader.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let reader: BufRead = ...;
    /// let channel = Channel::read_from(reader).unwrap();
    /// ```
    pub fn read_from<R: ::std::io::BufRead>(reader: R) -> Result<Channel, Error> {
        ::parse(::quick_xml::XmlReader::from_reader(reader))
    }

    #[cfg(feature = "xml-rs")]
    #[inline]
    /// Attempt to read the RSS channel from the speficied reader.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let reader: Read = ...;
    /// let channel = Channel::read_from(reader).unwrap();
    /// ```

    pub fn read_from<R: ::std::io::Read>(reader: R) -> Result<Channel, Error> {
        ::parse(::xml::reader::EventReader::new(reader))
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

#[derive(Default)]
pub struct ChannelBuilder {
    pub title: Option<String>,
    pub link: Option<String>,
    pub description: Option<String>,
    pub language: Option<String>,
    pub pub_date: Option<String>,
    pub last_build_date: Option<String>,
    pub categories: Vec<String>,
    pub items: Vec<Item>,
}

impl ChannelBuilder {
    #[inline]
    pub fn new() -> ChannelBuilder {
        Default::default()
    }

    pub fn build(self) -> Result<Channel, Error> {
        let title = match self.title {
            Some(value) => value,
            None => return Err(Error::MissingField("Channel", "title")),
        };

        let link = match self.link {
            Some(value) => value,
            None => return Err(Error::MissingField("Channel", "link")),
        };

        let description = match self.description {
            Some(value) => value,
            None => return Err(Error::MissingField("Channel", "description")),
        };

        Ok(Channel {
            title: title,
            link: link,
            description: description,
            language: self.language,
            pub_date: self.pub_date,
            last_build_date: self.last_build_date,
            categories: self.categories,
            items: self.items,
        })
    }
}

impl FromXml for ChannelBuilder {
    fn consume_named<T: XmlName>(&mut self, name: T, content: String) {
        match name.local_name() {
            b"title" => self.title = Some(content),
            b"link" => self.link = Some(content),
            b"description" => self.description = Some(content),
            b"language" => self.language = Some(content),
            b"pubDate" => self.pub_date = Some(content),
            b"lastBuildDate" => self.last_build_date = Some(content),
            b"category" => self.categories.push(content),
            _ => {}
        }
    }
}

