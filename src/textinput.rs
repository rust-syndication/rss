use quick_xml::{XmlReader, XmlWriter, Element, Event};
use quick_xml::error::Error as XmlError;

use fromxml::FromXml;
use toxml::{ToXml, XmlWriterExt};
use error::Error;

/// A representation of the `<textInput>` element.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TextInput {
    /// The label of the Submit button for the text input.
    pub title: String,
    /// A description of the text input.
    pub description: String,
    /// The name of the text object.
    pub name: String,
    /// The URL of the CGI script that processes the text input request.
    pub link: String,
}

impl FromXml for TextInput {
    fn from_xml<R: ::std::io::BufRead>(mut reader: XmlReader<R>,
                                       _: Element)
                                       -> Result<(Self, XmlReader<R>), Error> {
        let mut title = None;
        let mut description = None;
        let mut name = None;
        let mut link = None;

        while let Some(e) = reader.next() {
            match e {
                Ok(Event::Start(element)) => {
                    match element.name() {
                        b"title" => title = element_text!(reader),
                        b"description" => description = element_text!(reader),
                        b"name" => name = element_text!(reader),
                        b"link" => link = element_text!(reader),
                        _ => skip_element!(reader),
                    }
                }
                Ok(Event::End(_)) => {
                    let title = title.unwrap_or_default();
                    let description = description.unwrap_or_default();
                    let name = name.unwrap_or_default();
                    let link = link.unwrap_or_default();

                    return Ok((TextInput {
                        title: title,
                        description: description,
                        name: name,
                        link: link,
                    }, reader))
                }
                Err(err) => return Err(err.into()),
                _ => {}
            }
        }

        Err(Error::EOF)
    }
}

impl ToXml for TextInput {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut XmlWriter<W>) -> Result<(), XmlError> {
        let element = Element::new("textInput");

        try!(writer.write(Event::Start(element.clone())));

        try!(writer.write_text_element(b"title", &self.title));
        try!(writer.write_text_element(b"description", &self.description));
        try!(writer.write_text_element(b"name", &self.name));
        try!(writer.write_text_element(b"link", &self.link));

        writer.write(Event::End(element))
    }
}
