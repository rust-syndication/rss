use fromxml::{FromXml, XmlName, XmlAttribute};
use error::Error;

/// A representation of the `<guid>` element.
pub struct Guid {
    /// The value of the guid. This is the content of the `<guid>` tag.
    pub value: String,
    /// Indicates if the guid is a permalink. This is the `isPermaLink` attribute of the `<guid>` tag.
    pub is_permalink: Option<bool>,
}

#[derive(Default)]
pub struct GuidBuilder {
    pub value: Option<String>,
    pub is_permalink: Option<bool>,
}

impl GuidBuilder {
    #[inline]
    pub fn new() -> GuidBuilder {
        Default::default()
    }

    pub fn build(self) -> Result<Guid, Error> {
        let value = match self.value {
            Some(value) => value,
            None => return Err(Error::MissingField("Guid", "value")),
        };

        Ok(Guid {
            value: value,
            is_permalink: self.is_permalink,
        })
    }
}

impl FromXml for GuidBuilder {
    #[inline]
    fn consume_content(&mut self, content: String) {
        self.value = Some(content);
    }

    fn consume_attribute<T: XmlAttribute>(&mut self, attr: T) -> Result<(), Error> {
        if attr.name().local_name() == b"isPermaLink" {
            let value = try!(attr.borrowed_value());
            self.is_permalink = Some(value != "false");
        }

        Ok(())
    }
}

