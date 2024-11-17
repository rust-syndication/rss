// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::collections::BTreeMap;
use std::io::Write;

use quick_xml::events::{BytesCData, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Error as XmlError;
use quick_xml::Writer;

pub trait ToXml {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError>;

    fn used_namespaces(&self) -> BTreeMap<String, String> {
        BTreeMap::new()
    }
}

impl<'a, T: ToXml> ToXml for &'a T {
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
