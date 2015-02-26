extern crate xml;

use xml::{Element, Xml};


fn elem_with_text(tag_name: &'static str, chars: &str) -> Element {
    let mut elem = Element::new(tag_name, None, &[]);
    elem.text(chars);
    elem
}


trait ToXml<> {
    fn to_xml(&self) -> Element;
}


/// RSS version 2.0
pub struct Rss(pub Vec<Channel>);

impl ToXml for Rss {
    fn to_xml(&self) -> Element {
        let mut rss = Element::new("rss", None, &[("version", None, "2.0")]);

        let &Rss(ref channels) = self;
        for channel in channels {
            rss.tag(channel.to_xml());
        }

        rss
    }
}

impl Rss {
    fn to_string(&self) -> String {
        let mut ret = format!("{}", Xml::PINode("xml version='1.0' encoding='UTF-8'".to_string()));
        ret.push_str(&format!("{}", self.to_xml()));
        ret
    }
}


pub struct Channel {
    pub title: String,
    pub link: String,
    pub description: String,
    pub items: Vec<Item>,
}

impl ToXml for Channel {
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
}

pub struct Item {
    pub title: Option<String>,
    pub link: Option<String>,
    pub description: Option<String>,
}


impl ToXml for Item {
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
}


#[test]
fn test_consruct() {
    #![feature(collections)]

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
