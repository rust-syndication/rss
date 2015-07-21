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

//! Library for serializing the RSS web content syndication format
//!
//! # Examples
//!
//! ## Writing
//!
//! ```
//! use rss::{Channel, Item, Rss};
//!
//! let item = Item {
//!     title: Some(String::from("Ford hires Elon Musk as CEO")),
//!     pub_date: Some(String::from("01 Apr 2019 07:30:00 GMT")),
//!     description: Some(String::from("In an unprecedented move, Ford hires Elon Musk.")),
//!     ..Default::default()
//! };
//!
//! let channel = Channel {
//!     title: String::from("TechCrunch"),
//!     link: String::from("http://techcrunch.com"),
//!     description: String::from("The latest technology news and information on startups"),
//!     items: vec![item],
//!     ..Default::default()
//! };
//!
//! let rss = Rss(channel);
//!
//! let rss_string = rss.to_string();
//! ```
//!
//! ## Reading
//!
//! ```
//! use rss::Rss;
//!
//! let rss_str = r#"
//! <?xml version="1.0" encoding="UTF-8"?>
//! <rss version="2.0">
//!   <channel>
//!     <title>TechCrunch</title>
//!     <link>http://techcrunch.com</link>
//!     <description>The latest technology news and information on startups</description>
//!     <item>
//!       <title>Ford hires Elon Musk as CEO</title>
//!       <pubDate>01 Apr 2019 07:30:00 GMT</pubDate>
//!       <description>In an unprecedented move, Ford hires Elon Musk.</description>
//!     </item>
//!   </channel>
//! </rss>
//! "#;
//!
//! let rss = rss_str.parse::<Rss>().unwrap();
//! ```

mod category;
mod channel;
mod item;
mod text_input;

extern crate xml;

use std::ascii::AsciiExt;
use std::str::FromStr;

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


/// [RSS 2.0 Specification § What is RSS]
/// (http://cyber.law.harvard.edu/rss/rss.html#whatIsRss)
#[derive(Default, Debug, Clone)]
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
            return Err("Top element is not <rss>, most likely not an RSS feed");
        }

        let channel_elem = match rss_elem.get_child("channel", None) {
            Some(elem) => elem,
            None => return Err("No <channel> element found in <rss>"),
        };

        let channel = try!(ViaXml::from_xml(channel_elem.clone()));

        Ok(Rss(channel))
    }
}

impl FromStr for Rss {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new();
        parser.feed_str(&s);

        let mut builder = ElementBuilder::new();

        for event in parser {
            if let Some(Ok(elem)) = builder.handle_event(event) {
                return ViaXml::from_xml(elem);
            }
        }

        Err("RSS read error")
    }
}

impl ToString for Rss {
    fn to_string(&self) -> String {
        let mut ret = format!("{}", Xml::PINode("xml version='1.0' encoding='UTF-8'".to_string()));
        ret.push_str(&format!("{}", self.to_xml()));
        ret
    }
}


#[cfg(test)]
mod test {
    use std::default::Default;
    use std::fs::File;
    use std::io::Read;
    use std::str::FromStr;
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
        let mut rss_string = String::new();
        file.read_to_string(&mut rss_string).unwrap();
        let rss = Rss::from_str(&rss_string).unwrap();
        assert!(rss.to_string().len() > 0);
    }

    #[test]
    fn test_read_no_channels() {
        let rss_str = "<rss></rss>";
        assert!(Rss::from_str(rss_str).is_err());
    }

    #[test]
    fn test_read_one_channel_no_properties() {
        let rss_str = "\
            <rss>\
                <channel>\
                </channel>\
            </rss>";
        assert!(Rss::from_str(rss_str).is_err());
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
        let Rss(channel) = Rss::from_str(rss_str).unwrap();
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
        let Rss(channel) = Rss::from_str(rss_str).unwrap();
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
        let Rss(channel) = Rss::from_str(rss_str).unwrap();
        assert_eq!("Title", channel.title);
    }
}
