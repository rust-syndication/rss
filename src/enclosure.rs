use std::str;

use quick_xml::{XmlReader, Event, Element};

use fromxml::FromXml;
use error::Error;

/// A representation of the `<enclosure>` element.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Enclosure {
    /// The url of the enclosure.
    pub url: String,
    /// The length of the enclosure in bytes.
    pub length: String,
    /// The mime type of the enclosure.
    pub mime_type: String,
}

impl FromXml for Enclosure {
    fn from_xml<R: ::std::io::BufRead>(mut reader: XmlReader<R>,
                                       element: Element)
                                       -> Result<(Self, XmlReader<R>), Error> {
        let mut url = None;
        let mut length = None;
        let mut mime_type = None;

        for attr in element.attributes() {
            if let Ok(attr) = attr {
                match attr.0 {
                    b"url" => url = str::from_utf8(attr.1).map(|s| s.to_string()).ok(),
                    b"length" => length = str::from_utf8(attr.1).map(|s| s.to_string()).ok(),
                    b"type" => mime_type = str::from_utf8(attr.1).map(|s| s.to_string()).ok(),
                    _ => {}
                }
            }
        }

        close_element!(reader);

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
