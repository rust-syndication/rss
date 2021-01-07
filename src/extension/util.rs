// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::collections::HashMap;
use std::io::BufRead;
use std::str;

use quick_xml::events::attributes::Attributes;
use quick_xml::events::Event;
use quick_xml::Reader;

use crate::error::Error;
use crate::extension::{Extension, ExtensionMap};

pub fn extension_name(element_name: &[u8]) -> Option<(&[u8], &[u8])> {
    let mut split = element_name.splitn(2, |b| *b == b':');
    match split.next() {
        Some(b"") | None => None,
        Some(ns) => split.next().map(|name| (ns, name)),
    }
}

pub fn parse_extension<R>(
    reader: &mut Reader<R>,
    atts: Attributes,
    ns: &[u8],
    name: &[u8],
    extensions: &mut ExtensionMap,
) -> Result<(), Error>
where
    R: BufRead,
{
    let ns = str::from_utf8(ns)?;
    let name = str::from_utf8(name)?;
    let ext = parse_extension_element(reader, atts)?;

    let map = extensions
        .entry(ns.to_string())
        .or_insert_with(HashMap::new);

    let items = map.entry(name.to_string()).or_insert_with(Vec::new);
    items.push(ext);

    Ok(())
}

fn parse_extension_element<R: BufRead>(
    reader: &mut Reader<R>,
    mut atts: Attributes,
) -> Result<Extension, Error> {
    let mut extension = Extension::default();
    let mut buf = Vec::new();

    for attr in atts.with_checks(false) {
        if let Ok(attr) = attr {
            let key = str::from_utf8(attr.key)?;
            let value = attr.unescape_and_decode_value(reader)?;
            extension.attrs.insert(key.to_string(), value);
        }
    }

    loop {
        match reader.read_event(&mut buf)? {
            Event::Start(element) => {
                let ext = parse_extension_element(reader, element.attributes())?;
                let name = str::from_utf8(element.local_name())?;
                let items = extension
                    .children
                    .entry(name.to_string())
                    .or_insert_with(Vec::new);

                items.push(ext);
            }
            Event::CData(element) => {
                extension.value = Some(reader.decode(&element).into());
            }
            Event::Text(element) => {
                extension.value = Some(element.unescape_and_decode(reader)?);
            }
            Event::End(element) => {
                extension.name = reader.decode(element.name()).into();
                break;
            }
            Event::Eof => return Err(Error::Eof),
            _ => {}
        }

        buf.clear();
    }

    Ok(extension)
}

pub fn get_extension_values(v: Vec<Extension>) -> Vec<String> {
    v.into_iter()
        .filter_map(|ext| ext.value)
        .collect::<Vec<_>>()
}

pub fn remove_extension_value(
    map: &mut HashMap<String, Vec<Extension>>,
    key: &str,
) -> Option<String> {
    map.remove(key)
        .map(|mut v| v.remove(0))
        .and_then(|ext| ext.value)
}
