use quick_xml::{XmlReader, XmlWriter, Element, Event};
use quick_xml::error::Error as XmlError;

use fromxml::FromXml;
use toxml::ToXml;
use error::Error;

/// A representation of the `<enclosure>` element.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Enclosure {
    /// The URL of the enclosure.
    pub url: String,
    /// The length of the enclosure in bytes.
    pub length: String,
    /// The MIME type of the enclosure.
    pub mime_type: String,
}

impl FromXml for Enclosure {
    fn from_xml<R: ::std::io::BufRead>(mut reader: XmlReader<R>,
                                       element: Element)
                                       -> Result<(Self, XmlReader<R>), Error> {
        let mut url = None;
        let mut length = None;
        let mut mime_type = None;

        for attr in element.attributes().with_checks(false).unescaped() {
            if let Ok(attr) = attr {
                match attr.0 {
                    b"url" if url.is_none() => {
                        url = Some(try!(String::from_utf8(attr.1.into_owned())));
                    }
                    b"length" if length.is_none() => {
                        length = Some(try!(String::from_utf8(attr.1.into_owned())));
                    }
                    b"type" if mime_type.is_none() => {
                        mime_type = Some(try!(String::from_utf8(attr.1.into_owned())));
                    }
                    _ => {}
                }
            }
        }

        skip_element!(reader);

        let url = url.unwrap_or_default();
        let length = length.unwrap_or_default();
        let mime_type = mime_type.unwrap_or_default();

        Ok((Enclosure {
            url: url,
            length: length,
            mime_type: mime_type,
        }, reader))
    }
}

impl ToXml for Enclosure {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut XmlWriter<W>) -> Result<(), XmlError> {
        let element = Element::new(b"enclosure");

        try!(writer.write(Event::Start({
            let mut element = element.clone();

            let attrs = &[(b"url" as &[u8], &self.url),
                          (b"length", &self.length),
                          (b"type", &self.mime_type)];
            element.extend_attributes(attrs.into_iter().map(|v| *v));

            element
        })));

        writer.write(Event::End(element))
    }
}
