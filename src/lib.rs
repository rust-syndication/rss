#![feature(io)]

extern crate xml;

use std::ascii::AsciiExt;
use std::default::Default;
use std::iter::IteratorExt;
use std::io;
use xml::{Element, ElementBuilder, Parser, Xml};


trait ElementUtils {
    fn tag_with_text(&mut self, child_name: &'static str, child_body: &str);
    fn tag_with_text_opt(&mut self, child_name: &'static str, child_body: &Option<String>);
}


impl ElementUtils for Element {
    fn tag_with_text(&mut self, child_name: &'static str, child_body: &str) {
        self.tag(elem_with_text(child_name, child_body));
    }

    fn tag_with_text_opt(&mut self, child_name: &'static str, child_body: &Option<String>) {
        if let Some(ref c) = *child_body {
            self.tag_with_text(child_name, &c);
        }
    }
}


fn elem_with_text(tag_name: &'static str, chars: &str) -> Element {
    let mut elem = Element::new(tag_name, None, &[]);
    elem.text(chars);
    elem
}


trait ViaXml {
    fn to_xml(&self) -> Element;
    fn from_xml(element: Element) -> Result<Self, &'static str>;
}


/// RSS
///
/// "At the top level, a RSS document is a \<rss\> element, with a mandatory attribute called
/// version, that specifies the version of RSS that the document conforms to. If it conforms to
/// this specification, the version attribute must be 2.0."
///
/// [RSS 2.0 Specification § RSS]
/// (http://cyber.law.harvard.edu/rss/rss.html#whatIsRss)
#[derive(Default)]
pub struct Rss(pub Channel);

impl ViaXml for Rss {
    fn to_xml(&self) -> Element {
        let mut rss = Element::new("rss", None, &[("version", None, "2.0")]);

        let &Rss(ref channel) = self;
        rss.tag(channel.to_xml());

        rss
    }

    fn from_xml(rss_elem: Element) -> Result<Self, &'static str> {
        if rss_elem.name.to_ascii_lowercase() != "rss" {
            panic!("Expected <rss>, found <{}>", rss_elem.name);
        }

        let channel_elem = match rss_elem.get_child("channel", None) {
            Some(elem) => elem,
            None => return Err("No <channel> element found in <rss>"),
        };

        let channel = try!(ViaXml::from_xml(channel_elem.clone()));

        Ok(Rss(channel))
    }
}

impl Rss {
    fn to_string(&self) -> String {
        let mut ret = format!("{}", Xml::PINode("xml version='1.0' encoding='UTF-8'".to_string()));
        ret.push_str(&format!("{}", self.to_xml()));
        ret
    }

    fn from_read(reader: &mut io::Read) -> Result<Self, &'static str> {
        let mut rss_string = String::new();

        match reader.read_to_string(&mut rss_string) {
            Ok(..) => (),
            Err(..) => return Err("Error reading string from reader"),
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

        Err("RSS read error")
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
///     items: vec![],
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

        channel.tag_with_text("title", &self.title);
        channel.tag_with_text("link", &self.link);
        channel.tag_with_text("description", &self.description);

        for item in &self.items {
            channel.tag(item.to_xml());
        }

        channel
    }

    fn from_xml(element: Element) -> Result<Self, &'static str> {
        let mut channel: Channel = Default::default();

        match element.get_child("title", None) {
            Some(element) => channel.title = element.content_str(),
            None => return Err("<channel> is missing required <title> element"),
        }

        match element.get_child("link", None) {
            Some(element) => channel.link = element.content_str(),
            None => return Err("<channel> is missing required <link> element"),
        }

        match element.get_child("description", None) {
            Some(element) => channel.description = element.content_str(),
            None => return Err("<channel> is missing required <description> element"),
        }

        channel.items = element.get_children("item", None)
            .into_iter()
            .map(|e| ViaXml::from_xml(e.clone()).unwrap())
            .collect();

        Ok(channel)
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

        item.tag_with_text_opt("title", &self.title);
        item.tag_with_text_opt("link", &self.link);
        item.tag_with_text_opt("description", &self.description);

        item
    }

    fn from_xml(element: Element) -> Result<Self, &'static str> {
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

        Ok(item)
    }
}


#[cfg(test)]
mod test {
    use std::fs::File;
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
        let rss = Rss::from_read(&mut file).unwrap();
        assert!(rss.to_string().len() > 0);
    }

    #[test]
    #[should_fail]
    fn test_from_read_no_channels() {
        let mut rss_bytes = "<rss></rss>".as_bytes();
        let Rss(_) = Rss::from_read(&mut rss_bytes).unwrap();
    }

    #[test]
    #[should_fail]
    fn test_from_read_one_channel_no_properties() {
        let mut rss_bytes = "<rss><channel></channel></rss>".as_bytes();
        let Rss(_) = Rss::from_read(&mut rss_bytes).unwrap();
    }

    #[test]
    fn test_from_read_one_channel() {
        let mut rss_bytes = "<rss><channel><title>Hello world!</title><description></description><link></link></channel></rss>".as_bytes();
        let Rss(channel) = Rss::from_read(&mut rss_bytes).unwrap();
        assert_eq!("Hello world!", channel.title);
    }
}
