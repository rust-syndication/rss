use std::str;
use std::io::BufRead;
use std::collections::HashMap;

use quick_xml::{XmlReader, Element, Event};

use error::Error;
use extension::{Extension, ExtensionMap};

pub trait FromXml: Sized {
    fn from_xml<R: ::std::io::BufRead>(mut reader: XmlReader<R>,
                                       element: Element)
                                       -> Result<(Self, XmlReader<R>), Error>;
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
        let result = ::fromxml::parse_extension($reader, &$element, $ns, $name, &mut $extensions);
        $reader = result.1;
        try!(result.0)
    })
}

pub fn element_text<R: BufRead>(mut reader: XmlReader<R>)
                                -> (Result<Option<String>, Error>, XmlReader<R>) {
    let mut content: Option<String> = None;

    while let Some(e) = reader.next() {
        match e {
            Ok(Event::Start(_)) => {
                let result = skip_element(reader);
                reader = result.1;
                try_reader!(result.0, reader);
            }
            Ok(Event::End(_)) => {
                break;
            }
            Ok(Event::CData(element)) => {
                let text = element.content();
                content = Some(try_reader!(str::from_utf8(text), reader).to_string());
            }
            Ok(Event::Text(element)) => {
                let text = try_reader!(element.unescaped_content(), reader);
                content = Some(try_reader!(String::from_utf8(text.into_owned()), reader));
            }
            Err(err) => return (Err(err.into()), reader),
            _ => {}
        }
    }

    (Ok(content), reader)
}

pub fn skip_element<R: BufRead>(mut reader: XmlReader<R>) -> (Result<(), Error>, XmlReader<R>) {
    while let Some(e) = reader.next() {
        match e {
            Ok(Event::Start(_)) => {
                let result = skip_element(reader);
                reader = result.1;
                try_reader!(result.0, reader);
            }
            Ok(Event::End(_)) => {
                return (Ok(()), reader);
            }
            Err(err) => return (Err(err.into()), reader),
            _ => {}
        }
    }

    (Err(Error::EOF), reader)
}

pub fn extension_name(element: &Element) -> Option<(&[u8], &[u8])> {
    let split = element.name().splitn(2, |b| *b == b':').collect::<Vec<_>>();

    if split.len() == 2 {
        let ns = unsafe { split.get_unchecked(0) };
        if ns != b"" && ns != b"rss" && ns != b"rdf" {
            return Some((ns, unsafe { split.get_unchecked(1) }));
        }
    }

    None
}

pub fn parse_extension<R: BufRead>(mut reader: XmlReader<R>,
                                   element: &Element,
                                   ns: &[u8],
                                   name: &[u8],
                                   extensions: &mut ExtensionMap)
                                   -> (Result<(), Error>, XmlReader<R>) {
    let ns = try_reader!(str::from_utf8(ns), reader);
    let name = try_reader!(str::from_utf8(name), reader);

    let ext = parse_extension_element(reader, element);
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

fn parse_extension_element<R: BufRead>(mut reader: XmlReader<R>,
                                       element: &Element)
                                       -> (Result<Extension, Error>, XmlReader<R>) {
    let mut children = HashMap::<String, Vec<Extension>>::new();
    let mut attrs = HashMap::<String, String>::new();
    let mut content = None;

    for attr in element.attributes().with_checks(false).unescaped() {
        if let Ok(attr) = attr {
            let key = try_reader!(str::from_utf8(attr.0), reader);
            let value = try_reader!(str::from_utf8(&attr.1), reader);
            attrs.insert(key.to_string(), value.to_string());
        }
    }

    while let Some(e) = reader.next() {
        match e {
            Ok(Event::Start(element)) => {
                let child = parse_extension_element(reader, &element);
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
                return (Ok(Extension {
                    name: try_reader!(str::from_utf8(element.name()), reader).to_string(),
                    value: content,
                    attrs: attrs,
                    children: children,
                }), reader)
            }
            Ok(Event::CData(element)) => {
                let text = element.content();
                content = Some(try_reader!(str::from_utf8(text), reader).to_string());
            }
            Ok(Event::Text(element)) => {
                let text = try_reader!(element.unescaped_content(), reader);
                content = Some(try_reader!(String::from_utf8(text.into_owned()), reader));
            }
            Err(err) => return (Err(err.into()), reader),
            _ => {}
        }
    }

    (Err(Error::EOF), reader)
}
