#![feature(io)]

extern crate xml;

use std::ascii::AsciiExt;
use std::default::Default;
use std::iter::IteratorExt;
use std::io;
use xml::{Element, ElementBuilder, Parser, Xml};


fn elem_with_text(tag_name: &'static str, chars: &str) -> Element {
    let mut elem = Element::new(tag_name, None, &[]);
    elem.text(chars);
    elem
}


trait ViaXml {
    fn to_xml(&self) -> Element;
    fn from_xml(element: Element) -> Self;
}


/// RSS version 2.0
#[derive(Default)]
pub struct Rss(pub Channel);

impl ViaXml for Rss {
    fn to_xml(&self) -> Element {
        let mut rss = Element::new("rss", None, &[("version", None, "2.0")]);

        let &Rss(ref channel) = self;
        rss.tag(channel.to_xml());

        rss
    }

    fn from_xml(element: Element) -> Self {
        if element.name.to_ascii_lowercase() != "rss" {
            panic!("Expected <rss>, found <{}>", element.name);
        }

        let channel_element = element.get_child("channel", None).unwrap();
        let channel = ViaXml::from_xml(channel_element.clone());

        Rss(channel)
    }
}

impl Rss {
    fn to_string(&self) -> String {
        let mut ret = format!("{}", Xml::PINode("xml version='1.0' encoding='UTF-8'".to_string()));
        ret.push_str(&format!("{}", self.to_xml()));
        ret
    }

    fn from_read(reader: &mut io::Read) -> Self {
        let mut rss_string = String::new();

        match reader.read_to_string(&mut rss_string) {
            Ok(..) => (),
            Err(..) => panic!("Error reading string from reader"),
        }

        let mut parser = Parser::new();
        parser.feed_str(&rss_string);

        let mut builder = ElementBuilder::new();

        for event in parser {
            match builder.push_event(event) {
                Ok(Some(element)) => return ViaXml::from_xml(element),
                _ => (),
            }
        }

        panic!("RSS read error")
    }
}


/// Channel
///
/// "Subordinate to the \<rss\> element is a single \<channel\> element, which contains information
/// about the channel (metadata) and its contents."
///
/// [RSS 2.0 Specification § Channel]
/// (http://cyber.law.harvard.edu/rss/rss.html#requiredChannelElements)
///
/// ## Examples
///
/// ```
/// use rss::Channel;
///
/// let channel = Channel {
///     title: "My Blog".to_string(),
///     link: "http://myblog.com".to_string(),
///     description: "Where I write stuff".to_string(),
/// };
/// ```
#[derive(Default)]
pub struct Channel {
    pub title: String,
    pub link: String,
    pub description: String,
    pub items: Vec<Item>,
    // pub language: Option<String>,
    // pub copyright: Option<String>,
    // pub managing_editor: Option<String>,
    // pub web_master: Option<String>,
    // pub pub_date: Option<String>,
    // pub last_build_date: Option<String>,
    // pub category:
    // pub generator: Option<String>,
    // pub docs: Option<String>,
    // pub cloud:
    // pub ttl:
    // pub image: Option<String>,
    // pub rating: Option<String>,
    // pub text_input:
    // pub skip_hours: Option<String>,
    // pub skip_days: Option<String>,
}

impl ViaXml for Channel {
    fn to_xml(&self) -> Element {
        let mut channel = Element::new("channel", None, &[]);

        channel.tag(elem_with_text("title", &self.title));
        channel.tag(elem_with_text("link", &self.link));
        channel.tag(elem_with_text("description", &self.description));

        for item in &self.items {
            channel.tag(item.to_xml());
        }

        channel
    }

    fn from_xml(element: Element) -> Self {
        let mut channel: Channel = Default::default();

        match element.get_child("title", None) {
            Some(element) => channel.title = element.content_str(),
            None => (),
        }

        match element.get_child("link", None) {
            Some(element) => channel.link = element.content_str(),
            None => (),
        }

        match element.get_child("description", None) {
            Some(element) => channel.description = element.content_str(),
            None => (),
        }

        channel.items = element.get_children("item", None)
            .into_iter()
            .map(|e| ViaXml::from_xml(e.clone()))
            .collect();

        channel
    }
}


#[derive(Default)]
pub struct Item {
    pub title: Option<String>,
    pub link: Option<String>,
    pub description: Option<String>,
    // pub author
    // pub category
    // pub comments
    // pub enclosure
    // pub guid
    // pub pubDate
    // pub source
}


impl ViaXml for Item {
    fn to_xml(&self) -> Element {
        let mut item = Element::new("item", None, &[]);

        if let Some(ref s) = self.title {
            item.tag(elem_with_text("title", s));
        }

        if let Some(ref s) = self.link {
            item.tag(elem_with_text("link", s));
        }

        if let Some(ref s) = self.description {
            item.tag(elem_with_text("description", s));
        }

        item
    }

    fn from_xml(element: Element) -> Self {
        let mut item: Item = Default::default();

        match element.get_child("title", None) {
            Some(element) => item.title = Some(element.content_str()),
            None => (),
        }

        match element.get_child("link", None) {
            Some(element) => item.link = Some(element.content_str()),
            None => (),
        }

        match element.get_child("description", None) {
            Some(element) => item.description = Some(element.content_str()),
            None => (),
        }

        item
    }
}


#[cfg(test)]
mod test {
    use super::{Rss, Item, Channel};

    #[test]
    fn test_basic_to_string() {
        let item = Item {
            title: Some("My first post!".to_string()),
            link: Some("http://myblog.com/post1".to_string()),
            description: Some("This is my first post".to_string()),
        };

        let channel = Channel {
            title: "My Blog".to_string(),
            link: "http://myblog.com".to_string(),
            description: "Where I write stuff".to_string(),
            items: vec![item],
        };

        let rss = Rss(channel);
        assert_eq!(rss.to_string(), "<?xml version=\'1.0\' encoding=\'UTF-8\'?><rss version=\'2.0\'><channel><title>My Blog</title><link>http://myblog.com</link><description>Where I write stuff</description><item><title>My first post!</title><link>http://myblog.com/post1</link><description>This is my first post</description></item></channel></rss>");
    }

    #[test]
    fn test_from_file() {
        let mut file = File::open("test-data/pinboard.xml").unwrap();
        let Rss(channel) = Rss::from_read(&mut file);
    }

    #[test]
    #[should_fail]
    fn test_from_read_no_channels() {
        let mut rss_bytes = "<rss></rss>".as_bytes();
        let Rss(channel) = Rss::from_read(&mut rss_bytes);
    }

    #[test]
    fn test_from_read_one_channel() {
        let mut rss_bytes = "<rss><channel></channel></rss>".as_bytes();
        let Rss(channel) = Rss::from_read(&mut rss_bytes);
    }

    #[test]
    fn test_from_read_one_channel_with_title() {
        let mut rss_bytes = "<rss><channel><title>Hello world!</title></channel></rss>".as_bytes();
        let Rss(channel) = Rss::from_read(&mut rss_bytes);
        assert_eq!("Hello world!", channel.title);
    }
}
