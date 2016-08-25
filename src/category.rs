use std::str;

use quick_xml::{XmlReader, Element};

use fromxml::FromXml;
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

        for attr in element.attributes().with_checks(false) {
            if let Ok(attr) = attr {
                if attr.0 == b"domain" {
                    domain = str::from_utf8(attr.1).map(|s| s.to_string()).ok();
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
