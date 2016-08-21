use fromxml::{FromXml, XmlName, XmlAttribute};
use error::Error;

/// A representation of the `<source>` element.
pub struct Source {
    /// The url of the source. This is the `url` attribute of the `<source>`.
    pub url: String,
    /// The title of the source. This is the content of the `<source>`.
    pub title: Option<String>,
}

#[derive(Default)]
pub struct SourceBuilder {
    pub url: Option<String>,
    pub title: Option<String>,
}

impl SourceBuilder {
    #[inline]
    pub fn new() -> SourceBuilder {
        Default::default()
    }

    pub fn build(self) -> Result<Source, Error> {
        let url = match self.url {
            Some(value) => value,
            None => return Err(Error::MissingField("Source", "url")),
        };

        Ok(Source {
            url: url,
            title: self.title,
        })
    }
}

impl FromXml for SourceBuilder {
    #[inline]
    fn consume_content(&mut self, content: String) {
        self.title = Some(content);
    }

    fn consume_attribute<T: XmlAttribute>(&mut self, attr: T) -> Result<(), Error> {
        if attr.name().local_name() == b"url" {
            self.url = Some(try!(attr.owned_value()));
        }

        Ok(())
    }
}

