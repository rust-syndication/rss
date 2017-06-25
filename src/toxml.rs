// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use quick_xml::errors::Error as XmlError;
use quick_xml::events::{Event, BytesStart, BytesEnd, BytesText};
use quick_xml::writer::Writer;

pub trait ToXml {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError>;
}

impl<'a, T: ToXml> ToXml for &'a T {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        (*self).to_xml(writer)
    }
}

pub trait WriterExt {
    fn write_text_element<N: AsRef<[u8]>, T: AsRef<[u8]>>(
        &mut self,
        name: N,
        text: T,
    ) -> Result<(), XmlError>;

    fn write_text_elements<N: AsRef<[u8]>, T: AsRef<[u8]>, I: IntoIterator<Item = T>>(
        &mut self,
        name: N,
        values: I,
    ) -> Result<(), XmlError>;

    fn write_cdata_element<N: AsRef<[u8]>, T: AsRef<[u8]>>(
        &mut self,
        name: N,
        text: T,
    ) -> Result<(), XmlError>;

    fn write_object<T: ToXml>(&mut self, object: T) -> Result<(), XmlError>;

    fn write_objects<T: ToXml, I: IntoIterator<Item = T>>(
        &mut self,
        objects: I,
    ) -> Result<(), XmlError>;
}

impl<W: ::std::io::Write> WriterExt for Writer<W> {
    fn write_text_element<N: AsRef<[u8]>, T: AsRef<[u8]>>(
        &mut self,
        name: N,
        text: T,
    ) -> Result<(), XmlError> {
        let name = name.as_ref();
        self.write_event(Event::Start(BytesStart::borrowed(name, name.len())))?;
        self.write_event(Event::Text(BytesText::borrowed(text.as_ref())))?;
        self.write_event(Event::End(BytesEnd::borrowed(name)))?;
        Ok(())
    }

    fn write_text_elements<N: AsRef<[u8]>, T: AsRef<[u8]>, I: IntoIterator<Item = T>>(
        &mut self,
        name: N,
        values: I,
    ) -> Result<(), XmlError> {
        for value in values {
            self.write_text_element(&name, value)?;
        }

        Ok(())
    }

    fn write_cdata_element<N: AsRef<[u8]>, T: AsRef<[u8]>>(
        &mut self,
        name: N,
        text: T,
    ) -> Result<(), XmlError> {
        let name = name.as_ref();
        self.write_event(Event::Start(BytesStart::borrowed(name, name.len())))?;
        self.write_event(Event::CData(BytesText::borrowed(text.as_ref())))?;
        self.write_event(Event::End(BytesEnd::borrowed(name)))?;
        Ok(())
    }

    #[inline]
    fn write_object<T: ToXml>(&mut self, object: T) -> Result<(), XmlError> {
        object.to_xml(self)
    }

    fn write_objects<T: ToXml, I: IntoIterator<Item = T>>(
        &mut self,
        objects: I,
    ) -> Result<(), XmlError> {
        for object in objects {
            object.to_xml(self)?;
        }

        Ok(())
    }
}
