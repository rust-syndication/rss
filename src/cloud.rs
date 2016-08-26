use std::str;

use quick_xml::{XmlReader, Element};

use fromxml::FromXml;
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

        for attr in element.attributes().with_checks(false) {
            if let Ok(attr) = attr {
                match attr.0 {
                    b"domain" if domain.is_none() => {
                        domain = str::from_utf8(attr.1).map(|s| s.to_string()).ok();
                    }
                    b"port" if port.is_none() => {
                        port = str::from_utf8(attr.1).map(|s| s.to_string()).ok();
                    }
                    b"path" if path.is_none() => {
                        path = str::from_utf8(attr.1).map(|s| s.to_string()).ok();
                    }
                    b"registerProcedure" if register_procedure.is_none() => {
                        register_procedure = str::from_utf8(attr.1).map(|s| s.to_string()).ok();
                    }
                    b"protocol" if protocol.is_none() => {
                        protocol = str::from_utf8(attr.1).map(|s| s.to_string()).ok();
                    }
                    _ => {}
                }
            }
        }

        close_element!(reader);

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
