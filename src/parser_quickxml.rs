use std::io::BufRead;

use quick_xml::{XmlReader, Event};
use quick_xml::AsStr;

use fromxml::FromXml;
use channel::{Channel, ChannelBuilder};
use item::ItemBuilder;
use guid::GuidBuilder;
use enclosure::EnclosureBuilder;
use source::SourceBuilder;
use error::Error;

/// Attempt to parse the `XmlReader` for an RSS channel.
///
/// # Example
///
/// ```rust,ignore
/// let reader: BufRead = ...;
/// let xml_reader = XmlReader::from_reader(reader);
/// let channel = parse(xml_reader).unwrap();
/// ```
pub fn parse<R: BufRead>(reader: XmlReader<R>) -> Result<Channel, Error> {
    let reader = reader.trim_text(true);
    let mut channel = ChannelBuilder::new();
    let mut item: Option<ItemBuilder> = None;

    let mut context_stack = Vec::<Vec<u8>>::new();
    let mut buffer: Option<String> = None;

    for e in reader.into_iter() {
        match e {
            Ok(Event::Start(element)) => {
                match element.name() {
                    b"channel" => {
                        context_stack.push(element.name().to_vec());
                    }
                    b"item" => {
                        context_stack.push(element.name().to_vec());
                        item = Some(ItemBuilder::new())
                    }
                    b"guid" => {
                        if let Some(item) = item.as_mut() {
                            let mut builder = GuidBuilder::new();
                            for attr in element.attributes() {
                                try!(attr.map_err(|e| e.0.into())
                                    .and_then(|attr| builder.consume_attribute(attr)));
                            }
                            item.guid = Some(builder);
                        }
                    }
                    b"enclosure" => {
                        if let Some(item) = item.as_mut() {
                            let mut builder = EnclosureBuilder::new();
                            for attr in element.attributes() {
                                try!(attr.map_err(|e| e.0.into())
                                    .and_then(|attr| builder.consume_attribute(attr)));
                            }
                            item.enclosure = Some(builder);
                        }
                    }
                    b"source" => {
                        if let Some(item) = item.as_mut() {
                            let mut builder = SourceBuilder::new();
                            for attr in element.attributes() {
                                try!(attr.map_err(|e| e.0.into())
                                    .and_then(|attr| builder.consume_attribute(attr)));
                            }
                            item.source = Some(builder);
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::End(element)) => {
                match element.name() {
                    b"channel" => return channel.build(),
                    b"item" => {
                        if let Some(inner) = item {
                            channel.items.push(try!(inner.build()));
                            item = None;
                        }
                        context_stack.pop();
                    }
                    b"guid" => {
                        if let Some(mut inner) = item.as_mut().and_then(|v| v.guid.as_mut()) {
                            if let Some(buffer) = buffer {
                                inner.consume_content(buffer);
                            }
                            buffer = None;
                        }
                    }
                    b"source" => {
                        if let Some(mut inner) = item.as_mut().and_then(|v| v.source.as_mut()) {
                            if let Some(buffer) = buffer {
                                inner.consume_content(buffer);
                            }
                            buffer = None;
                        }
                    }
                    _ => {}
                }

                match context_stack.last().map(|v| v.as_slice()) {
                    Some(b"channel") => {
                        if let Some(buffer) = buffer {
                            channel.consume_named(element.name(), buffer);
                        }
                    }
                    Some(b"item") => {
                        if let Some(mut inner) = item.as_mut() {
                            if let Some(buffer) = buffer {
                                inner.consume_named(element.name(), buffer);
                            }
                        }
                    }
                    _ => {}
                }

                buffer = None;
            }
            Ok(Event::CData(element)) => {
                let content = element.content();
                let text = try!(content.as_str());

                if buffer.is_some() {
                    buffer.as_mut().unwrap().push_str(text);
                } else {
                    buffer = Some(text.to_string());
                }

            }
            Ok(Event::Text(element)) => {
                let content = try!(element.unescaped_content().map_err(|e| e.0));
                let text = try!(content.as_str());

                if buffer.is_some() {
                    buffer.as_mut().unwrap().push_str(text);
                } else {
                    buffer = Some(text.to_string());
                }
            }
            Err(err) => return Err(Error::XmlParsing(err.0)),
            _ => {}
        }
    }

    Err(Error::NotFound)
}

