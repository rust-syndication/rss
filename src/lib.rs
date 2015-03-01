extern crate xml;

use std::ascii::AsciiExt;
use std::default::Default;
use std::iter::IteratorExt;
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
pub struct Rss(pub Vec<Channel>);

impl ViaXml for Rss {
    fn to_xml(&self) -> Element {
        let mut rss = Element::new("rss", None, &[("version", None, "2.0")]);

        let &Rss(ref channels) = self;
        for channel in channels {
            rss.tag(channel.to_xml());
        }

        rss
    }

    fn from_xml(element: Element) -> Self {
        if element.name.to_ascii_lowercase() != "rss" {
            panic!("Expected <rss>, found <{}>", element.name);
        }

        let channels = element.get_children("channel", None)
            .into_iter()
            .map(|e| ViaXml::from_xml(e.clone()))
            .collect();

        Rss(channels)
    }
}

impl Rss {
    fn to_string(&self) -> String {
        let mut ret = format!("{}", Xml::PINode("xml version='1.0' encoding='UTF-8'".to_string()));
        ret.push_str(&format!("{}", self.to_xml()));
        ret
    }

    fn from_str(input: &str) -> Self {
        let mut parser = Parser::new();
        parser.feed_str(input);

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


#[derive(Default)]
pub struct Channel {
    pub title: String,
    pub link: String,
    pub description: String,
    pub items: Vec<Item>,
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


#[test]
fn test_basic_to_string() {
    let item = Item {
        title: Some(String::from_str("My first post!")),
        link: Some(String::from_str("http://myblog.com/post1")),
        description: Some(String::from_str("This is my first post")),
    };

    let channel = Channel {
        title: String::from_str("My Blog"),
        link: String::from_str("http://myblog.com"),
        description: String::from_str("Where I write stuff"),
        items: vec![item],
    };

    let rss = Rss(vec![channel]);
    assert_eq!(rss.to_string(), "<?xml version=\'1.0\' encoding=\'UTF-8\'?><rss version=\'2.0\'><channel><title>My Blog</title><link>http://myblog.com</link><description>Where I write stuff</description><item><title>My first post!</title><link>http://myblog.com/post1</link><description>This is my first post</description></item></channel></rss>");
}

#[test]
fn test_from_str_no_channels() {
    let rss_str = "<rss></rss>";
    let Rss(channels) = Rss::from_str(rss_str);
    assert_eq!(0, channels.len());
}

#[test]
fn test_from_str_one_channel() {
    let rss_str = "<rss><channel></channel></rss>";
    let Rss(channels) = Rss::from_str(rss_str);
    assert_eq!(1, channels.len());
}

#[test]
fn test_from_str_one_channel_with_title() {
    let rss_str = "<rss><channel><title>Hello world!</title></channel></rss>";
    let Rss(channels) = Rss::from_str(rss_str);
    assert_eq!(1, channels.len());
    assert_eq!("Hello world!", channels[0].title);
}
