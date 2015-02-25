#![feature(old_io)]

extern crate xml;

use xml::attribute::Attribute;
use xml::common::XmlVersion;
use xml::name::Name;
use xml::namespace::Namespace;
use xml::writer::EventWriter;
use xml::writer::events::XmlEvent;


fn add_block<'a>(events: &mut Vec<XmlEvent<'a>>, namespace: &'a Namespace, tag_name: &'static str, chars: &'a String) {
    // <(tag_name)>
    events.push(XmlEvent::StartElement {
        name: Name::local(tag_name),
        attributes: vec![],
        namespace: namespace,
    });

    events.push(XmlEvent::Characters(chars));
    
    // </(tag_name)>
    events.push(XmlEvent::EndElement {
        name: Name::local(tag_name),
    });
}


trait Write {
    fn write<W: Writer>(&self, writer: W) -> Result<(), ()>;
}


trait ToXml<'a> {
    // todo: get rid of namespace parameter
    fn to_xml(&'a self, namespace: &'a Namespace) -> Vec<XmlEvent>;
}


/// RSS version 2.0
pub struct Rss(pub Vec<Channel>);

impl<'a> ToXml<'a> for Rss {
    fn to_xml(&'a self, namespace: &'a Namespace) -> Vec<XmlEvent> {
        let mut events = vec![];

        // <?xml version="1.0" encoding="UTF-8"?>
        events.push(XmlEvent::StartDocument{ 
            version: XmlVersion::Version10,
            encoding: Some("UTF-8"),
            standalone: None,
        });

        // <rss version="2.0">
        events.push(XmlEvent::StartElement {
            name: Name::local("rss"),
            attributes: vec![
                Attribute::new(Name::local("version"), "2.0"),
            ],
            namespace: namespace,
        });

        let &Rss(ref channels) = self;
        for channel in channels {
            for event in channel.to_xml(namespace) {
                events.push(event);
            }
        }

        // </rss>
        events.push(XmlEvent::EndElement {
            name: Name::local("rss"),
        });
        
        events
    }
}

impl Write for Rss {
    fn write<W: Writer>(&self, writer: W) -> Result<(), ()> {
        let mut event_writer = EventWriter::new(writer);

        let namespace = Namespace::empty();
        let events = self.to_xml(&namespace);

        for event in events {
            event_writer.write(event);
        };

        Ok(())
    }
}


pub struct Channel {
    pub title: String,
    pub link: String,
    pub description: String,
    pub items: Vec<Item>,
}

impl<'a> ToXml<'a> for Channel {
    fn to_xml(&'a self, namespace: &'a Namespace) -> Vec<XmlEvent> {
        let mut events = vec![];

        // <channel>
        events.push(XmlEvent::StartElement {
            name: Name::local("channel"),
            attributes: vec![],
            namespace: namespace,
        });

        add_block(&mut events, namespace, "title", &self.title);
        add_block(&mut events, namespace, "link", &self.link);
        add_block(&mut events, namespace, "description", &self.description);

        // </channel>
        events.push(XmlEvent::EndElement {
            name: Name::local("channel"),
        });

        events
    }
}

pub struct Item {
    pub title: Option<String>,
    pub link: Option<String>,
    pub description: Option<String>,
}


#[test]
fn test_consruct() {
    use std::old_io;

    let channel = Channel {
        title: String::from_str("My Blog"),
        link: String::from_str("http://myblog.com"),
        description: String::from_str("Where I write stuff"),
        items: vec![],
    };

    let rss = Rss(vec![channel]);
    rss.write(old_io::stdout());
}
