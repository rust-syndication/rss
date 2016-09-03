use quick_xml::{XmlReader, XmlWriter, Element, Event};
use quick_xml::error::Error as XmlError;

use fromxml::FromXml;
use toxml::ToXml;
use error::Error;

/// A representation of the `<category>` element.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Category {
    /// The name of the category.
    pub name: String,
    /// The domain for the category.
    pub domain: Option<String>,
}

impl FromXml for Category {
    fn from_xml<R: ::std::io::BufRead>(mut reader: XmlReader<R>,
                                       element: Element)
                                       -> Result<(Self, XmlReader<R>), Error> {
        let mut domain = None;

        for attr in element.attributes().with_checks(false).unescaped() {
            if let Ok(attr) = attr {
                if attr.0 == b"domain" {
                    domain = Some(try!(String::from_utf8(attr.1.into_owned())));
                    break;
                }
            }
        }

        let content = element_text!(reader).unwrap_or_default();

        Ok((Category {
            name: content,
            domain: domain,
        }, reader))
    }
}

impl ToXml for Category {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut XmlWriter<W>) -> Result<(), XmlError> {
        let element = Element::new(b"category");

        try!(writer.write(Event::Start({
            let mut element = element.clone();
            if let Some(ref domain) = self.domain {
                element.extend_attributes(::std::iter::once((b"domain", domain)));
            }
            element
        })));

        try!(writer.write(Event::Text(Element::new(self.name.as_str()))));

        writer.write(Event::End(element))
    }
}
