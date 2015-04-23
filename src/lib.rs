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

extern crate xml;

use std::ascii::AsciiExt;
use std::io;
use xml::{Element, ElementBuilder, Parser, Xml};


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
            if let Some(Ok(element)) = builder.handle_event(event) {
                return ViaXml::from_xml(element);
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
/// use std::default::Default;
///
/// let channel = Channel {
///     title: "My Blog".to_string(),
///     link: "http://myblog.com".to_string(),
///     description: "Where I write stuff".to_string(),
///     items: vec![],
///     ..Default::default()
/// };
/// ```
#[derive(Default)]
pub struct Channel {
    pub title: String,
    pub link: String,
    pub description: String,
    pub items: Vec<Item>,
    pub language: Option<String>,
    pub copyright: Option<String>,
    pub managing_editor: Option<String>,
    pub web_master: Option<String>,
    pub pub_date: Option<String>,
    pub last_build_date: Option<String>,
    pub categories: Vec<Category>,
    pub generator: Option<String>,
    pub docs: Option<String>,
    // pub cloud:
    pub ttl: Option<String>,  // TODO: change this to Option<i32>?
    pub image: Option<String>,
    pub rating: Option<String>,
    // pub text_input:
    pub skip_hours: Option<String>,
    pub skip_days: Option<String>,
}

impl ViaXml for Channel {
    fn to_xml(&self) -> Element {
        let mut channel = Element::new("channel".to_string(), None, vec![]);

        channel.tag_with_text("title", &self.title);
        channel.tag_with_text("link", &self.link);
        channel.tag_with_text("description", &self.description);

        for item in &self.items {
            channel.tag(item.to_xml());
        }

        channel.tag_with_optional_text("language", &self.language);
        channel.tag_with_optional_text("copyright", &self.copyright);
        channel.tag_with_optional_text("managingEditor", &self.managing_editor);
        channel.tag_with_optional_text("webMaster", &self.web_master);
        channel.tag_with_optional_text("pubDate", &self.pub_date);
        channel.tag_with_optional_text("lastBuildDate", &self.last_build_date);
        channel.tag_with_optional_text("generator", &self.generator);
        channel.tag_with_optional_text("docs", &self.docs);
        channel.tag_with_optional_text("ttl", &self.ttl);
        channel.tag_with_optional_text("image", &self.image);
        channel.tag_with_optional_text("rating", &self.rating);
        channel.tag_with_optional_text("skipHours", &self.skip_hours);
        channel.tag_with_optional_text("skipDays", &self.skip_days);

        for category in &self.categories {
            channel.tag(category.to_xml());
        }

        channel
    }

    fn from_xml(element: Element) -> Result<Self, &'static str> {
        let title = match element.get_child("title", None) {
            Some(element) => element.content_str(),
            None => return Err("<channel> is missing required <title> element"),
        };

        let link = match element.get_child("link", None) {
            Some(element) => element.content_str(),
            None => return Err("<channel> is missing required <link> element"),
        };

        let description = match element.get_child("description", None) {
            Some(element) => element.content_str(),
            None => return Err("<channel> is missing required <description> element"),
        };

        let items = element.get_children("item", None)
            .map(|e| ViaXml::from_xml(e.clone()).unwrap())
            .collect();

        let language = element.get_child("language", None).map(Element::content_str);
        let copyright = element.get_child("copyright", None).map(Element::content_str);
        let managing_editor = element.get_child("managing_editor", None).map(Element::content_str);
        let web_master = element.get_child("managing_editor", None).map(Element::content_str);
        let pub_date = element.get_child("pub_date", None).map(Element::content_str);
        let last_build_date = element.get_child("last_build_date", None).map(Element::content_str);

        let categories = element.get_children("category", None)
            .map(|e| ViaXml::from_xml(e.clone()).unwrap())
            .collect();

        let generator = element.get_child("generator", None).map(Element::content_str);
        let docs = element.get_child("docs", None).map(Element::content_str);
        let ttl = element.get_child("ttl", None).map(Element::content_str);
        let image = element.get_child("image", None).map(Element::content_str);
        let rating = element.get_child("rating", None).map(Element::content_str);
        let skip_hours = element.get_child("skip_hours", None).map(Element::content_str);
        let skip_days = element.get_child("skip_days", None).map(Element::content_str);

        Ok(Channel {
            title: title,
            link: link,
            description: description,
            items: items,
            language: language,
            copyright: copyright,
            managing_editor: managing_editor,
            web_master: web_master,
            pub_date: pub_date,
            last_build_date: last_build_date,
            categories: categories,
            generator: generator,
            docs: docs,
            ttl: ttl,
            image: image,
            rating: rating,
            skip_hours: skip_hours,
            skip_days: skip_days,
        })
    }
}


/// Item
///
/// [RSS 2.0 Specification § Item]
/// (http://cyber.law.harvard.edu/rss/rss.html#hrelementsOfLtitemgt)
#[derive(Default)]
pub struct Item {
    pub title: Option<String>,
    pub link: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub categories: Vec<Category>,
    pub comments: Option<String>,
    // pub enclosure
    // pub guid
    // pub pubDate
    // pub source
}


impl ViaXml for Item {
    fn to_xml(&self) -> Element {
        let mut item = Element::new("item".to_string(), None, vec![]);

        item.tag_with_optional_text("title", &self.title);
        item.tag_with_optional_text("link", &self.link);
        item.tag_with_optional_text("description", &self.description);
        item.tag_with_optional_text("author", &self.author);
        item.tag_with_optional_text("comments", &self.comments);

        for category in &self.categories {
            item.tag(category.to_xml());
        }

        item
    }

    fn from_xml(element: Element) -> Result<Self, &'static str> {
        let title = element.get_child("title", None).map(Element::content_str);
        let link = element.get_child("link", None).map(Element::content_str);
        let description = element.get_child("description", None).map(Element::content_str);
        let author = element.get_child("author", None).map(Element::content_str);
        let comments = element.get_child("comments", None).map(Element::content_str);

        let categories = element.get_children("category", None)
            .map(|e| ViaXml::from_xml(e.clone()).unwrap())
            .collect();

        Ok(Item {
            title: title,
            link: link,
            description: description,
            categories: categories,
            author: author,
            comments: comments,
        })
    }
}


/// Category
///
/// [RSS 2.0 Specification § Category]
/// (http://cyber.law.harvard.edu/rss/rss.html#ltcategorygtSubelementOfLtitemgt)
#[derive(Default)]
pub struct Category {
    pub domain: Option<String>,
    pub value: String,
}

impl ViaXml for Category {
    fn to_xml(&self) -> Element {
        let mut category = match self.domain {
            Some(ref d) => Element::new("category".to_string(), None, vec![("domain".to_string(), None, d.clone())]),
            None => Element::new("category".to_string(), None, vec![]),
        };
        category.text(self.value.clone());
        category
    }

    fn from_xml(elem: Element) -> Result<Self, &'static str> {
        let domain = elem.get_attribute("domain", None).map(|s| s.to_string());
        let value = elem.content_str();

        Ok(Category {
            domain: domain,
            value: value,
        })
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
    fn test_from_read_no_channels() {
        let mut rss_bytes = "<rss></rss>".as_bytes();
        assert!(Rss::from_reader(&mut rss_bytes).is_err());
    }

    #[test]
    fn test_from_read_one_channel_no_properties() {
        let mut rss_bytes = "<rss><channel></channel></rss>".as_bytes();
        assert!(Rss::from_reader(&mut rss_bytes).is_err());
    }

    #[test]
    fn test_read_one_channel() {
        let rss_bytes = "\
            <rss>\
                <channel>\
                    <title>Hello world!</title>\
                    <description></description>\
                    <link></link>\
                </channel>\
            </rss>";
        let Rss(channel) = Rss::from_reader(&mut rss_bytes.as_bytes()).unwrap();
        assert_eq!("Hello world!", channel.title);
    }

    // Ensure reader ignores the PI XML node and continues to parse the RSS
    #[test]
    fn test_read_with_pinode() {
        let rss_bytes = "\
            <?xml version=\'1.0\' encoding=\'UTF-8\'?>\
            <rss>\
                <channel>\
                    <title>Title</title>\
                    <link></link>\
                    <description></description>\
                </channel>\
            </rss>";
        let Rss(channel) = Rss::from_reader(&mut rss_bytes.as_bytes()).unwrap();
        assert_eq!("Title", channel.title);
    }
}
