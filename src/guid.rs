use quick_xml::{XmlReader, XmlWriter, Element, Event};
use quick_xml::error::Error as XmlError;

use fromxml::FromXml;
use toxml::ToXml;
use error::Error;

/// A representation of the `<guid>` element.
#[derive(Debug, Clone, PartialEq)]
pub struct Guid {
    /// The value of the GUID.
    pub value: String,
    /// Indicates if the GUID is a permalink.
    pub is_permalink: bool,
}

impl Default for Guid {
    #[inline]
    fn default() -> Self {
        Guid {
            value: Default::default(),
            is_permalink: true,
        }
    }
}

impl FromXml for Guid {
    fn from_xml<R: ::std::io::BufRead>(mut reader: XmlReader<R>,
                                       element: Element)
                                       -> Result<(Self, XmlReader<R>), Error> {
        let mut is_permalink = true;

        for attr in element.attributes().with_checks(false).unescaped() {
            if let Ok(attr) = attr {
                if attr.0 == b"isPermaLink" {
                    is_permalink = &attr.1 as &[u8] != b"false";
                    break;
                }
            }
        }

        let content = element_text!(reader).unwrap_or_default();

        Ok((Guid {
            value: content,
            is_permalink: is_permalink,
        }, reader))
    }
}

impl ToXml for Guid {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut XmlWriter<W>) -> Result<(), XmlError> {
        let element = Element::new(b"guid");

        try!(writer.write(Event::Start({
            let mut element = element.clone();
            if !self.is_permalink {
                element.extend_attributes(::std::iter::once((b"isPermaLink", b"false")));
            }
            element
        })));

        try!(writer.write(Event::Text(Element::new(self.value.as_str()))));

        writer.write(Event::End(element))
    }
}
