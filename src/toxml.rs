// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::collections::BTreeMap;
use std::io::{self, Write};

use quick_xml::events::{BytesCData, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Error as XmlError;
use quick_xml::Writer;

/// Writes U+FFFD for each control character XML 1.0 forbids.
pub(crate) struct XmlSafeWriter<W>(pub(crate) W);

impl<W> XmlSafeWriter<W> {
    pub(crate) fn into_inner(self) -> W {
        self.0
    }
}

impl<W: Write> Write for XmlSafeWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let forbidden = |b: &u8| matches!(b, 0x00..=0x08 | 0x0B | 0x0C | 0x0E..=0x1F);
        let mut chunks = buf.split(forbidden);
        if let Some(first) = chunks.next() {
            self.0.write_all(first)?;
        }
        for chunk in chunks {
            self.0.write_all("\u{FFFD}".as_bytes())?;
            self.0.write_all(chunk)?;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

pub trait ToXml {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError>;

    fn used_namespaces(&self) -> BTreeMap<String, String> {
        BTreeMap::new()
    }
}

impl<T: ToXml> ToXml for &T {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        (*self).to_xml(writer)
    }
}

pub trait WriterExt {
    fn write_text_element<N, T>(&mut self, name: N, text: T) -> Result<(), XmlError>
    where
        N: AsRef<str>,
        T: AsRef<str>;

    fn write_text_elements<N, T, I>(&mut self, name: N, values: I) -> Result<(), XmlError>
    where
        N: AsRef<str>,
        T: AsRef<str>,
        I: IntoIterator<Item = T>;

    fn write_cdata_element<N, T>(&mut self, name: N, text: T) -> Result<(), XmlError>
    where
        N: AsRef<str>,
        T: AsRef<str>;

    fn write_object<T>(&mut self, object: T) -> Result<(), XmlError>
    where
        T: ToXml;

    fn write_objects<T, I>(&mut self, objects: I) -> Result<(), XmlError>
    where
        T: ToXml,
        I: IntoIterator<Item = T>;
}

impl<W: Write> WriterExt for Writer<W> {
    fn write_text_element<N, T>(&mut self, name: N, text: T) -> Result<(), XmlError>
    where
        N: AsRef<str>,
        T: AsRef<str>,
    {
        let name = name.as_ref();
        self.write_event(Event::Start(BytesStart::new(name)))?;
        self.write_event(Event::Text(BytesText::new(text.as_ref())))?;
        self.write_event(Event::End(BytesEnd::new(name)))?;
        Ok(())
    }

    fn write_text_elements<N, T, I>(&mut self, name: N, values: I) -> Result<(), XmlError>
    where
        N: AsRef<str>,
        T: AsRef<str>,
        I: IntoIterator<Item = T>,
    {
        for value in values {
            self.write_text_element(&name, value)?;
        }

        Ok(())
    }

    fn write_cdata_element<N, T>(&mut self, name: N, text: T) -> Result<(), XmlError>
    where
        N: AsRef<str>,
        T: AsRef<str>,
    {
        let name = name.as_ref();
        self.write_event(Event::Start(BytesStart::new(name)))?;
        BytesCData::escaped(text.as_ref())
            .try_for_each(|event| self.write_event(Event::CData(event)))?;
        self.write_event(Event::End(BytesEnd::new(name)))?;
        Ok(())
    }

    #[inline]
    fn write_object<T>(&mut self, object: T) -> Result<(), XmlError>
    where
        T: ToXml,
    {
        object.to_xml(self)
    }

    fn write_objects<T, I>(&mut self, objects: I) -> Result<(), XmlError>
    where
        T: ToXml,
        I: IntoIterator<Item = T>,
    {
        for object in objects {
            object.to_xml(self)?;
        }

        Ok(())
    }
}
