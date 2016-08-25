use std::str;

use quick_xml::{XmlReader, Element};

use fromxml::FromXml;
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

        for attr in element.attributes().with_checks(false) {
            if let Ok(attr) = attr {
                if attr.0 == b"url" {
                    url = str::from_utf8(attr.1).map(|s| s.to_string()).ok();
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
