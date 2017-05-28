// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use error::Error;
use extension::{Extension, ExtensionBuilder, ExtensionMap};
use quick_xml::events::Event;
use quick_xml::events::attributes::Attributes;
use quick_xml::reader::Reader;
use std::collections::HashMap;
use std::io::BufRead;
use std::str;

pub trait FromXml: Sized {
    fn from_xml<R: BufRead>(reader: &mut Reader<R>, atts: Attributes) -> Result<Self, Error>;
}

pub fn element_text<R: BufRead>(reader: &mut Reader<R>) -> Result<Option<String>, Error> {
    let mut content: Option<String> = None;
    let mut buf = Vec::new();
    let mut skip_buf = Vec::new();

    loop {
        match reader.read_event(&mut buf)? {
            Event::Start(element) => {
                reader.read_to_end(element.name(), &mut skip_buf)?;
            }
            Event::CData(element) => {
                let text = reader.decode(&*element).into_owned();
                content = Some(text);
            }
            Event::Text(element) => {
                let text = element.unescape_and_decode(&reader)?;
                content = Some(text);
            }
            Event::End(_) | Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(content)
}

pub fn extension_name(element_name: &[u8]) -> Option<(&[u8], &[u8])> {
    let mut split = element_name.splitn(2, |b| *b == b':');
    match split.next() {
        Some(b"") | Some(b"rss") | Some(b"rdf") | None => None,
        Some(ns) => split.next().map(|name| (ns, name)),
    }
}

pub fn parse_extension<R: BufRead>(reader: &mut Reader<R>,
                                   atts: Attributes,
                                   ns: &[u8],
                                   name: &[u8],
                                   extensions: &mut ExtensionMap)
                                   -> Result<(), Error> {
    let ns = str::from_utf8(ns)?;
    let name = str::from_utf8(name)?;
    let ext = parse_extension_element(reader, atts)?;

    if !extensions.contains_key(ns) {
        extensions.insert(ns.to_string(), HashMap::new());
    }

    let map = match extensions.get_mut(ns) {
        Some(map) => map,
        None => unreachable!(),
    };

    let ext = {
        if let Some(list) = map.get_mut(name) {
            list.push(ext);
            None
        } else {
            Some(ext)
        }
    };

    if let Some(ext) = ext {
        map.insert(name.to_string(), vec![ext]);
    }

    Ok(())
}

fn parse_extension_element<R: BufRead>(reader: &mut Reader<R>,
                                       mut atts: Attributes)
                                       -> Result<Extension, Error> {
    let mut children = HashMap::<String, Vec<Extension>>::new();
    let mut attrs = HashMap::<String, String>::new();
    let mut content = None;
    let mut buf = Vec::new();

    for attr in atts.with_checks(false) {
        if let Ok(attr) = attr {
            let key = str::from_utf8(attr.key)?;
            let value = attr.unescape_and_decode_value(&reader)?;
            attrs.insert(key.to_string(), value);
        }
    }

    loop {
        match reader.read_event(&mut buf)? {
            Event::Start(element) => {
                let ext = parse_extension_element(reader, element.attributes())?;
                let name = str::from_utf8(element.local_name())?;

                let ext = {
                    if let Some(list) = children.get_mut(name) {
                        list.push(ext);
                        None
                    } else {
                        Some(ext)
                    }
                };

                if let Some(ext) = ext {
                    children.insert(name.to_string(), vec![ext]);
                }
            }
            Event::End(element) => {
                return ExtensionBuilder::new()
                           .name(&*reader.decode(element.name()))
                           .value(content)
                           .attrs(attrs)
                           .children(children)
                           .finalize();
            }
            Event::CData(element) => {
                content = Some(reader.decode(&element).into_owned());
            }
            Event::Text(element) => {
                content = Some(element.unescape_and_decode(&reader)?);
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Err(Error::EOF)
}
