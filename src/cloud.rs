use quick_xml::{XmlReader, XmlWriter, Element, Event};
use quick_xml::error::Error as XmlError;

use fromxml::FromXml;
use toxml::ToXml;
use error::Error;

/// A representation of the `<cloud>` element.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Cloud {
    /// The domain to register with.
    pub domain: String,
    /// The port to register with.
    pub port: String,
    /// The path to register with.
    pub path: String,
    /// The procedure to register with.
    pub register_procedure: String,
    /// The protocol to register with.
    pub protocol: String,
}

impl FromXml for Cloud {
    fn from_xml<R: ::std::io::BufRead>(mut reader: XmlReader<R>,
                                       element: Element)
                                       -> Result<(Self, XmlReader<R>), Error> {
        let mut domain = None;
        let mut port = None;
        let mut path = None;
        let mut register_procedure = None;
        let mut protocol = None;

        for attr in element.attributes().with_checks(false).unescaped() {
            if let Ok(attr) = attr {
                match attr.0 {
                    b"domain" if domain.is_none() => {
                        domain = Some(try!(String::from_utf8(attr.1.into_owned())));
                    }
                    b"port" if port.is_none() => {
                        port = Some(try!(String::from_utf8(attr.1.into_owned())));
                    }
                    b"path" if path.is_none() => {
                        path = Some(try!(String::from_utf8(attr.1.into_owned())));
                    }
                    b"registerProcedure" if register_procedure.is_none() => {
                        register_procedure = Some(try!(String::from_utf8(attr.1.into_owned())));
                    }
                    b"protocol" if protocol.is_none() => {
                        protocol = Some(try!(String::from_utf8(attr.1.into_owned())));
                    }
                    _ => {}
                }
            }
        }

        skip_element!(reader);

        let domain = domain.unwrap_or_default();
        let port = port.unwrap_or_default();
        let path = path.unwrap_or_default();
        let register_procedure = register_procedure.unwrap_or_default();
        let protocol = protocol.unwrap_or_default();

        Ok((Cloud {
            domain: domain,
            port: port,
            path: path,
            register_procedure: register_procedure,
            protocol: protocol,
        }, reader))

    }
}

impl ToXml for Cloud {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut XmlWriter<W>) -> Result<(), XmlError> {
        let element = Element::new(b"cloud");

        try!(writer.write(Event::Start({
            let mut element = element.clone();

            let attrs = &[(b"domain" as &[u8], &self.domain),
                          (b"port", &self.port),
                          (b"path", &self.path),
                          (b"registerProcedure", &self.register_procedure),
                          (b"protocol", &self.protocol)];
            element.extend_attributes(attrs.into_iter().map(|v| *v));

            element
        })));

        writer.write(Event::End(element))
    }
}
