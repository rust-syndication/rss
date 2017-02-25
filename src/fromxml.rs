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
    fn from_xml<R: ::std::io::BufRead>(reader: Reader<R>,
                                       atts: Attributes)
                                       -> Result<(Self, Reader<R>), Error>;
}

macro_rules! try_reader {
    ($e:expr, $reader:ident) => (match $e {
        Ok(v) => v,
        Err(err) => return (Err(err.into()), $reader),
    })
}

macro_rules! element_text {
    ($reader:ident) => ({
        let text = ::fromxml::element_text($reader);
        $reader = text.1;
        try!(text.0)
    })
}

macro_rules! skip_element {
    ($reader:ident) => ({
        let result = ::fromxml::skip_element($reader);
        $reader = result.1;
        try!(result.0)
    })
}

macro_rules! parse_extension {
    ($reader:ident, $element:ident, $ns:ident, $name:ident, $extensions:expr) => ({
        let result = ::fromxml::parse_extension($reader, $element.attributes(), $ns, $name, &mut $extensions);
        $reader = result.1;
        try!(result.0)
    })
}

pub fn element_text<R: BufRead>(mut reader: Reader<R>)
                                -> (Result<Option<String>, Error>, Reader<R>) {
    let mut content: Option<String> = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(_)) => {
                let result = skip_element(reader);
                reader = result.1;
                try_reader!(result.0, reader);
            }
            Ok(Event::End(_)) => {
                break;
            }
            Ok(Event::CData(element)) => {
                let text = &*element;
                content = Some(try_reader!(str::from_utf8(text), reader).to_string());
            }
            Ok(Event::Text(element)) => {
                let text = try_reader!(element.unescaped(), reader);
                content = Some(try_reader!(String::from_utf8(text.into_owned()), reader));
            }
            Ok(Event::Eof) => break,
            Err(err) => return (Err(err.into()), reader),
            _ => {}
        }
        buf.clear();
    }

    (Ok(content), reader)
}

pub fn skip_element<R: BufRead>(mut reader: Reader<R>) -> (Result<(), Error>, Reader<R>) {
    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(_)) => {
                let result = skip_element(reader);
                reader = result.1;
                try_reader!(result.0, reader);
            }
            Ok(Event::End(_)) => {
                return (Ok(()), reader);
            }
            Ok(Event::Eof) => break,
            Err(err) => return (Err(err.into()), reader),
            _ => {}
        }
        buf.clear();
    }

    (Err(Error::EOF), reader)
}

pub fn extension_name(element_name: &[u8]) -> Option<(&[u8], &[u8])> {
    let split = element_name.splitn(2, |b| *b == b':').collect::<Vec<_>>();

    if split.len() == 2 {
        let ns = unsafe { split.get_unchecked(0) };
        if ns != b"" && ns != b"rss" && ns != b"rdf" {
            return Some((ns, unsafe { split.get_unchecked(1) }));
        }
    }

    None
}

pub fn parse_extension<R: BufRead>(mut reader: Reader<R>,
                                   atts: Attributes,
                                   ns: &[u8],
                                   name: &[u8],
                                   extensions: &mut ExtensionMap)
                                   -> (Result<(), Error>, Reader<R>) {
    let ns = try_reader!(str::from_utf8(ns), reader);
    let name = try_reader!(str::from_utf8(name), reader);

    let ext = parse_extension_element(reader, atts);
    reader = ext.1;
    let ext = try_reader!(ext.0, reader);

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

    (Ok(()), reader)
}

fn parse_extension_element<R: BufRead>(mut reader: Reader<R>,
                                       mut atts: Attributes)
                                       -> (Result<Extension, Error>, Reader<R>) {
    let mut children = HashMap::<String, Vec<Extension>>::new();
    let mut attrs = HashMap::<String, String>::new();
    let mut content = None;
    let mut buf = Vec::new();

    for attr in atts.with_checks(false) {
        if let Ok(attr) = attr {
            let key = try_reader!(str::from_utf8(attr.key), reader);
            let value = try_reader!(attr.unescape_and_decode_value(&reader), reader);
            attrs.insert(key.to_string(), value.to_string());
        }
    }

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(element)) => {
                let child = parse_extension_element(reader, element.attributes());
                reader = child.1;
                let ext = try_reader!(child.0, reader);

                let name = {
                    let split = element.name().splitn(2, |b| *b == b':').collect::<Vec<_>>();
                    if split.len() == 2 {
                        try_reader!(str::from_utf8(unsafe { split.get_unchecked(1) }), reader)
                    } else {
                        try_reader!(str::from_utf8(unsafe { split.get_unchecked(0) }), reader)
                    }
                };

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
            Ok(Event::End(element)) => {
                return (ExtensionBuilder::new()
                            .name(&*reader.decode(element.name()))
                            .value(content)
                            .attrs(attrs)
                            .children(children)
                            .finalize(),
                        reader);
            }
            Ok(Event::CData(element)) => {
                content = Some(reader.decode(&element).into_owned());
            }
            Ok(Event::Text(element)) => {
                content = Some(try_reader!(element.unescape_and_decode(&reader), reader));
            }
            Ok(Event::Eof) => break,
            Err(err) => return (Err(err.into()), reader),
            _ => {}
        }
        buf.clear();
    }

    (Err(Error::EOF), reader)
}
