// Copyright 2015 Corey Farwell
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod category;
mod channel;
mod item;
mod text_input;

extern crate xml;

use std::ascii::AsciiExt;
use std::io;

use xml::{Element, ElementBuilder, Parser, Xml};

pub use ::category::Category;
pub use ::channel::Channel;
pub use ::item::Item;
pub use ::text_input::TextInput;


trait ElementUtils {
    fn tag_with_text(&mut self, child_name: &'static str, child_body: &str);
    fn tag_with_optional_text(&mut self, child_name: &'static str, child_body: &Option<String>);
}


impl ElementUtils for Element {
    fn tag_with_text(&mut self, child_name: &'static str, child_body: &str) {
        self.tag(elem_with_text(child_name, child_body));
    }

    fn tag_with_optional_text(&mut self, child_name: &'static str, child_body: &Option<String>) {
        if let Some(ref c) = *child_body {
            self.tag_with_text(child_name, &c);
        }
    }
}


fn elem_with_text(tag_name: &'static str, chars: &str) -> Element {
    let mut elem = Element::new(tag_name.to_string(), None, vec![]);
    elem.text(chars.to_string());
    elem
}


trait ViaXml {
    fn to_xml(&self) -> Element;
    fn from_xml(elem: Element) -> Result<Self, &'static str>;
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
        let mut rss = Element::new("rss".to_string(), None, vec![("version".to_string(), None, "2.0".to_string())]);

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
    pub fn to_string(&self) -> String {
        let mut ret = format!("{}", Xml::PINode("xml version='1.0' encoding='UTF-8'".to_string()));
        ret.push_str(&format!("{}", self.to_xml()));
        ret
    }

    pub fn from_reader(reader: &mut io::Read) -> Result<Self, &'static str> {
        let mut rss_string = String::new();

        if let Err(..) = reader.read_to_string(&mut rss_string) {
            return Err("Error reading string from reader");
        }

        let mut parser = Parser::new();
        parser.feed_str(&rss_string);

        let mut builder = ElementBuilder::new();

        for event in parser {
            if let Some(Ok(elem)) = builder.handle_event(event) {
                return ViaXml::from_xml(elem);
            }
        }

        Err("RSS read error")
    }
}


#[cfg(test)]
mod test {
    use std::default::Default;
    use std::fs::File;
    use super::{Rss, Item, Channel};

    #[test]
    fn test_basic_to_string() {
        let item = Item {
            title: Some("My first post!".to_string()),
            link: Some("http://myblog.com/post1".to_string()),
            description: Some("This is my first post".to_string()),
            ..Default::default()
        };

        let channel = Channel {
            title: "My Blog".to_string(),
            link: "http://myblog.com".to_string(),
            description: "Where I write stuff".to_string(),
            items: vec![item],
            ..Default::default()
        };

        let rss = Rss(channel);
        assert_eq!(rss.to_string(), "<?xml version=\'1.0\' encoding=\'UTF-8\'?><rss version=\'2.0\'><channel><title>My Blog</title><link>http://myblog.com</link><description>Where I write stuff</description><item><title>My first post!</title><link>http://myblog.com/post1</link><description>This is my first post</description></item></channel></rss>");
    }

    #[test]
    fn test_from_file() {
        let mut file = File::open("test-data/pinboard.xml").unwrap();
        let rss = Rss::from_reader(&mut file).unwrap();
        assert!(rss.to_string().len() > 0);
    }

    #[test]
    fn test_read_no_channels() {
        let rss_str = "<rss></rss>";
        assert!(Rss::from_reader(&mut rss_str.as_bytes()).is_err());
    }

    #[test]
    fn test_read_one_channel_no_properties() {
        let rss_str = "\
            <rss>\
                <channel>\
                </channel>\
            </rss>";
        assert!(Rss::from_reader(&mut rss_str.as_bytes()).is_err());
    }

    #[test]
    fn test_read_one_channel() {
        let rss_str = "\
            <rss>\
                <channel>\
                    <title>Hello world!</title>\
                    <description></description>\
                    <link></link>\
                </channel>\
            </rss>";
        let Rss(channel) = Rss::from_reader(&mut rss_str.as_bytes()).unwrap();
        assert_eq!("Hello world!", channel.title);
    }

    #[test]
    fn test_read_text_input() {
        let rss_str = "\
            <rss>\
                <channel>\
                    <title></title>\
                    <description></description>\
                    <link></link>\
                    <textInput>\
                        <title>Foobar</title>\
                        <description></description>\
                        <name></name>\
                        <link></link>\
                    </textInput>\
                </channel>\
            </rss>";
        let Rss(channel) = Rss::from_reader(&mut rss_str.as_bytes()).unwrap();
        assert_eq!("Foobar", channel.text_input.unwrap().title);
    }

    // Ensure reader ignores the PI XML node and continues to parse the RSS
    #[test]
    fn test_read_with_pinode() {
        let rss_str = "\
            <?xml version=\'1.0\' encoding=\'UTF-8\'?>\
            <rss>\
                <channel>\
                    <title>Title</title>\
                    <link></link>\
                    <description></description>\
                </channel>\
            </rss>";
        let Rss(channel) = Rss::from_reader(&mut rss_str.as_bytes()).unwrap();
        assert_eq!("Title", channel.title);
    }
}
