use quick_xml::{XmlReader, Event, Element};

use fromxml::FromXml;
use error::Error;

/// A representation of the `<image>` element.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Image {
    /// The URL of the channel image.
    pub url: String,
    /// A description of the image. This is used in the HTML `alt` attribute.
    pub title: String,
    /// The URL that the image links to.
    pub link: String,
    /// The width of the image.
    pub width: Option<String>,
    /// The height of the image.
    pub height: Option<String>,
    /// The text for the HTML `title` attribute.
    pub description: Option<String>,
}

impl FromXml for Image {
    fn from_xml<R: ::std::io::BufRead>(mut reader: XmlReader<R>,
                                       _: Element)
                                       -> Result<(Self, XmlReader<R>), Error> {
        let mut url = None;
        let mut title = None;
        let mut link = None;
        let mut width = None;
        let mut height = None;
        let mut description = None;

        while let Some(e) = reader.next() {
            match e {
                Ok(Event::Start(element)) => {
                    match element.name() {
                        b"url" => url = element_text!(reader),
                        b"title" => title = element_text!(reader),
                        b"link" => link = element_text!(reader),
                        b"width" => width = element_text!(reader),
                        b"height" => height = element_text!(reader),
                        b"description" => description = element_text!(reader),
                        _ => skip_element!(reader),
                    }
                }
                Ok(Event::End(_)) => {
                    let url = url.unwrap_or_default();
                    let title = title.unwrap_or_default();
                    let link = link.unwrap_or_default();

                    return Ok((Image {
                        url: url,
                        title: title,
                        link: link,
                        width: width,
                        height: height,
                        description: description,
                    }, reader))
                }
                Err(err) => return Err(err.into()),
                _ => {}
            }
        }

        Err(Error::EOF)
    }
}
