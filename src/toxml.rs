// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use quick_xml::{Element, Event, XmlWriter};
use quick_xml::error::Error as XmlError;

pub trait ToXml
{
    fn to_xml<W: ::std::io::Write>(&self,
                                   writer: &mut XmlWriter<W>)
        -> Result<(), XmlError>;
}

impl<'a, T: ToXml> ToXml for &'a T
{
    fn to_xml<W: ::std::io::Write>(&self,
                                   writer: &mut XmlWriter<W>)
        -> Result<(), XmlError>
    {
        (*self).to_xml(writer)
    }
}

pub trait XmlWriterExt
{
    fn write_text_element<N: AsRef<[u8]>, T: AsRef<[u8]>>(&mut self,
                                                          name: N,
                                                          text: T)
        -> Result<(), XmlError>;

    fn write_text_elements<N: AsRef<[u8]>, T: AsRef<[u8]>, I: IntoIterator<Item = T>>(&mut self,
                                                                                      name: N,
                                                                                      values: I)
        -> Result<(), XmlError>;

    fn write_cdata_element<N: AsRef<[u8]>, T: AsRef<[u8]>>(&mut self,
                                                           name: N,
                                                           text: T)
        -> Result<(), XmlError>;

    fn write_object<T: ToXml>(&mut self,
                              object: T)
        -> Result<(), XmlError>;

    fn write_objects<T: ToXml, I: IntoIterator<Item = T>>(&mut self,
                                                          objects: I)
        -> Result<(), XmlError>;
}

impl<W: ::std::io::Write> XmlWriterExt for XmlWriter<W>
{
    fn write_text_element<N: AsRef<[u8]>, T: AsRef<[u8]>>(&mut self,
                                                          name: N,
                                                          text: T)
        -> Result<(), XmlError>
    {
        let elem = Element::new(name);
        self.write(Event::Start(elem.clone()))?;
        self.write(Event::Text(Element::new(text)))?;
        self.write(Event::End(elem))
    }

    fn write_text_elements<N: AsRef<[u8]>, T: AsRef<[u8]>, I: IntoIterator<Item = T>>(&mut self,
                                                                                      name: N,
                                                                                      values: I)
        -> Result<(), XmlError>
    {
        for value in values {
            self.write_text_element(&name,
                                    value)?;
        }

        Ok(())
    }

    fn write_cdata_element<N: AsRef<[u8]>, T: AsRef<[u8]>>(&mut self,
                                                           name: N,
                                                           text: T)
        -> Result<(), XmlError>
    {
        let elem = Element::new(name);
        self.write(Event::Start(elem.clone()))?;
        self.write(Event::CData(Element::new(text)))?;
        self.write(Event::End(elem))
    }

    #[inline]
    fn write_object<T: ToXml>(&mut self,
                              object: T)
        -> Result<(), XmlError>
    {
        object.to_xml(self)
    }

    fn write_objects<T: ToXml, I: IntoIterator<Item = T>>(&mut self,
                                                          objects: I)
        -> Result<(), XmlError>
    {
        for object in objects {
            object.to_xml(self)?;
        }

        Ok(())
    }
}
