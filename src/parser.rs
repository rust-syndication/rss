use std::io::BufRead;

use quick_xml::{XmlReader, Event};

use fromxml::FromXml;
use channel::Channel;
use error::Error;

pub fn parse<R: BufRead>(reader: XmlReader<R>) -> Result<Channel, Error> {
    let mut reader = reader.trim_text(true);

    while let Some(e) = reader.next() {
        match e {
            Ok(Event::Start(element)) => {
                if element.name() == b"channel" {
                    return Channel::from_xml(reader, element).map(|v| v.0);
                }
            }
            Err(err) => return Err(err.into()),
            _ => {}
        }
    }

    Err(Error::EOF)
}
