use quick_xml::{XmlReader, XmlWriter, Element, Event};
use quick_xml::error::Error as XmlError;

use fromxml::FromXml;
use toxml::ToXml;
use error::Error;

/// A representation of the `<source>` element.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Source {
    /// The URL of the source.
    pub url: String,
    /// The title of the source.
    pub title: Option<String>,
}

impl FromXml for Source {
    fn from_xml<R: ::std::io::BufRead>(mut reader: XmlReader<R>,
                                       element: Element)
                                       -> Result<(Self, XmlReader<R>), Error> {
        let mut url = None;

        for attr in element.attributes().with_checks(false).unescaped() {
            if let Ok(attr) = attr {
                if attr.0 == b"url" {
                    url = Some(try!(String::from_utf8(attr.1.into_owned())));
                    break;
                }
            }
        }

        let url = url.unwrap_or_default();
        let content = element_text!(reader);

        Ok((Source {
            url: url,
            title: content,
        }, reader))
    }
}

impl ToXml for Source {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut XmlWriter<W>) -> Result<(), XmlError> {
        let element = Element::new(b"source");

        try!(writer.write(Event::Start({
            let mut element = element.clone();
            element.extend_attributes(::std::iter::once((b"url", self.url.as_str())));
            element
        })));

        if let Some(text) = self.title.as_ref().map(|s| s.as_str()) {
            try!(writer.write(Event::Text(Element::new(text))));
        }

        writer.write(Event::End(element))
    }
}
