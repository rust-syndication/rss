use std::io::Read;

use xml::reader::{EventReader, XmlEvent};

use fromxml::FromXml;
use channel::{Channel, ChannelBuilder};
use item::ItemBuilder;
use guid::GuidBuilder;
use enclosure::EnclosureBuilder;
use source::SourceBuilder;
use error::Error;

/// Attempt to parse the `EventReader` for an RSS channel.
/// # Example
///
/// ```rust,ignore
/// let reader: Read = ...;
/// let xml_reader = EventReader::new(reader);
/// let channel = parse(xml_reader).unwrap();
/// ```
pub fn parse<R: Read>(reader: EventReader<R>) -> Result<Channel, Error> {
    let mut channel = ChannelBuilder::new();
    let mut item: Option<ItemBuilder> = None;

    let mut element_stack = Vec::<String>::new();
    let mut buffer: Option<String> = None;

    for e in reader {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                match name.local_name.as_str() {
                    "item" => item = Some(ItemBuilder::new()),
                    "guid" => {
                        if let Some(item) = item.as_mut() {
                            let mut builder = GuidBuilder::new();
                            for attr in attributes {
                                try!(builder.consume_attribute(attr));
                            }
                            item.guid = Some(builder);
                        }
                    }
                    "enclosure" => {
                        if let Some(item) = item.as_mut() {
                            let mut builder = EnclosureBuilder::new();
                            for attr in attributes {
                                try!(builder.consume_attribute(attr));
                            }
                            item.enclosure = Some(builder);
                        }
                    }
                    "source" => {
                        if let Some(item) = item.as_mut() {
                            let mut builder = SourceBuilder::new();
                            for attr in attributes {
                                try!(builder.consume_attribute(attr));
                            }
                            item.source = Some(builder);
                        }
                    }
                    _ => {}
                }

                element_stack.push(name.local_name);
            }
            Ok(XmlEvent::EndElement { name }) => {
                element_stack.pop();

                match name.local_name.as_str() {
                    "channel" => return channel.build(),
                    "item" => {
                        if let Some(inner) = item {
                            channel.items.push(try!(inner.build()));
                            item = None;
                        }
                    }
                    "guid" => {
                        if let Some(mut inner) = item.as_mut().and_then(|v| v.guid.as_mut()) {
                            if let Some(buffer) = buffer {
                                inner.consume_content(buffer);
                            }
                            buffer = None;
                        }
                    }
                    "source" => {
                        if let Some(mut inner) = item.as_mut().and_then(|v| v.source.as_mut()) {
                            if let Some(buffer) = buffer {
                                inner.consume_content(buffer);
                            }
                            buffer = None;
                        }
                    }
                    _ => {}
                }

                match element_stack.last().map(|s| s.as_str()) {
                    Some("channel") => {
                        if let Some(buffer) = buffer {
                            channel.consume_named(name, buffer);
                        }
                    }
                    Some("item") => {
                        if let Some(mut inner) = item.as_mut() {
                            if let Some(buffer) = buffer {
                                inner.consume_named(name, buffer);
                            }
                        }
                    }
                    _ => {}
                }

                buffer = None;
            }
            Ok(XmlEvent::Characters(chars)) |
            Ok(XmlEvent::CData(chars)) => {
                if buffer.is_some() {
                    buffer.as_mut().unwrap().push_str(&chars);
                } else {
                    buffer = Some(chars);
                }
            }
            Err(err) => return Err(err.into()),
            _ => {}
        }
    }

    Err(Error::NotFound)
}

