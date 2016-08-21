use fromxml::{FromXml, XmlName, XmlAttribute};
use error::Error;

/// A representation of the `<enclosure>` element.
pub struct Enclosure {
    /// The url of the enclosure. This is the `url` attribute of the `<enclosure>`.
    pub url: String,
    /// The length of the enclosure. This is the `length` attribute of the `<enclosure>`.
    pub length: u64,
    /// The mime type of the enclosure. This is the `type` attribute of the `<enclosure>`.
    pub mime_type: String,
}

#[derive(Default)]
pub struct EnclosureBuilder {
    pub url: Option<String>,
    pub length: Option<u64>,
    pub mime_type: Option<String>,
}

impl EnclosureBuilder {
    pub fn new() -> EnclosureBuilder {
        Default::default()
    }

    pub fn build(self) -> Result<Enclosure, Error> {
        let url = match self.url {
            Some(value) => value,
            None => return Err(Error::MissingField("Enclosure", "url")),
        };

        let length = match self.length {
            Some(value) => value,
            None => return Err(Error::MissingField("Enclosure", "length")),
        };

        let mime_type = match self.mime_type {
            Some(value) => value,
            None => return Err(Error::MissingField("Enclosure", "type")),
        };

        Ok(Enclosure {
            url: url,
            length: length,
            mime_type: mime_type,
        })
    }
}

impl FromXml for EnclosureBuilder {
    fn consume_attribute<T: XmlAttribute>(&mut self, attr: T) -> Result<(), Error> {
        match attr.name().local_name() {
            b"url" => self.url = Some(try!(attr.owned_value())),
            b"length" => {
                self.length = Some({
                    let value = try!(attr.borrowed_value());
                    match value.parse::<u64>() {
                        Ok(value) => value,
                        Err(_) => return Err(Error::InvalidField("Enclosure", "length")),
                    }
                })
            }
            b"type" => self.mime_type = Some(try!(attr.owned_value())),
            _ => {}
        }

        Ok(())
    }
}

