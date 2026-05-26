// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::borrow::Cow;
use std::io::BufRead;

use quick_xml::escape::resolve_predefined_entity;
use quick_xml::events::attributes::Attribute;
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::Reader;

use crate::error::Error;

pub(crate) fn decode<'s, B: BufRead>(
    bytes: &'s [u8],
    reader: &Reader<B>,
) -> Result<Cow<'s, str>, Error> {
    let text = reader.decoder().decode(bytes)?;
    Ok(text)
}

pub(crate) fn attr_value<'s, B: BufRead>(
    attr: &'s Attribute<'s>,
    reader: &Reader<B>,
) -> Result<Cow<'s, str>, Error> {
    let value = attr.decode_and_unescape_value(reader.decoder())?;
    Ok(value)
}

pub(crate) fn skip<B: BufRead>(end: QName<'_>, reader: &mut Reader<B>) -> Result<(), Error> {
    reader.read_to_end_into(end, &mut Vec::new())?;
    Ok(())
}

pub fn element_text<R: BufRead>(reader: &mut Reader<R>) -> Result<Option<String>, Error> {
    let mut content = String::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(element) => {
                skip(element.name(), reader)?;
            }
            Event::Text(element) => {
                let decoded = element.decode()?;
                content.push_str(decoded.as_ref());
            }
            Event::GeneralRef(gref) => {
                let entity = gref.decode()?;
                if let Some(resolved_entity) = resolve_predefined_entity(&entity) {
                    content.push_str(resolved_entity);
                } else if let Some(ch) = gref.resolve_char_ref()? {
                    content.push(ch);
                } else {
                    content.push('&');
                    content.push_str(&entity);
                    content.push(';');
                }
            }
            Event::CData(element) => {
                content.push_str(decode(&element, reader)?.as_ref());
            }
            Event::End(_) | Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(Some(content.trim().to_owned()).filter(|c| !c.is_empty()))
}
