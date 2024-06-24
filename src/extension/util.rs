// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::io::BufRead;
use std::str;

use quick_xml::events::attributes::Attributes;
use quick_xml::events::Event;
use quick_xml::Reader;

use crate::error::Error;
use crate::extension::{Extension, ExtensionMap};
use crate::util::{attr_value, decode};

pub(crate) fn read_namespace_declarations<'m, R>(
    reader: &mut Reader<R>,
    mut atts: Attributes,
    base: &'m BTreeMap<String, String>,
) -> Result<Cow<'m, BTreeMap<String, String>>, Error>
where
    R: BufRead,
{
    let mut namespaces = Cow::Borrowed(base);
    for attr in atts.with_checks(false).flatten() {
        let key = decode(attr.key.as_ref(), reader)?;
        if let Some(ns) = key.strip_prefix("xmlns:") {
            namespaces
                .to_mut()
                .insert(ns.to_string(), attr_value(&attr, reader)?.to_string());
        }
    }
    Ok(namespaces)
}

pub(crate) fn extension_name(element_name: &str) -> Option<(&str, &str)> {
    let mut split = element_name.splitn(2, ':');
    let ns = split.next().filter(|ns| !ns.is_empty())?;
    let name = split.next()?;
    Some((ns, name))
}

pub(crate) fn extension_entry<'e>(
    extensions: &'e mut ExtensionMap,
    ns: &str,
    name: &str,
) -> &'e mut Vec<Extension> {
    let map = extensions.entry(ns.to_string()).or_default();

    map.entry(name.to_string()).or_default()
}

pub(crate) fn parse_extension_element<R: BufRead>(
    reader: &mut Reader<R>,
    mut atts: Attributes,
) -> Result<Extension, Error> {
    let mut extension = Extension::default();
    let mut buf = Vec::new();

    for attr in atts.with_checks(false).flatten() {
        let key = decode(attr.key.as_ref(), reader)?.to_string();
        let value = attr_value(&attr, reader)?.to_string();
        extension.attrs.insert(key.to_string(), value);
    }

    let mut text = String::new();
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(element) => {
                let ext = parse_extension_element(reader, element.attributes())?;
                let element_local_name = element.local_name();
                let name = decode(element_local_name.as_ref(), reader)?;

                let items = extension
                    .children
                    .entry(name.to_string())
                    .or_insert_with(Vec::new);

                items.push(ext);
            }
            Event::CData(element) => {
                text.push_str(decode(&element, reader)?.as_ref());
            }
            Event::Text(element) => {
                text.push_str(element.unescape()?.as_ref());
            }
            Event::End(element) => {
                extension.name = decode(element.name().as_ref(), reader)?.into();
                break;
            }
            Event::Eof => return Err(Error::Eof),
            _ => {}
        }

        buf.clear();
    }
    extension.value = Some(text.trim())
        .filter(|t| !t.is_empty())
        .map(ToString::to_string);

    Ok(extension)
}

pub fn get_extension_values(v: Vec<Extension>) -> Vec<String> {
    v.into_iter()
        .filter_map(|ext| ext.value)
        .collect::<Vec<_>>()
}

pub fn remove_extension_value(
    map: &mut BTreeMap<String, Vec<Extension>>,
    key: &str,
) -> Option<String> {
    map.remove(key)
        .map(|mut v| v.remove(0))
        .and_then(|ext| ext.value)
}
