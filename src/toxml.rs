use quick_xml::{XmlWriter, Element, Event};
use quick_xml::error::Error as XmlError;

pub trait ToXml {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut XmlWriter<W>) -> Result<(), XmlError>;
}

impl<'a, T: ToXml> ToXml for &'a T {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut XmlWriter<W>) -> Result<(), XmlError> {
        (*self).to_xml(writer)
    }
}

pub trait XmlWriterExt {
    fn write_text_element<N: AsRef<[u8]>, T: AsRef<[u8]>>(&mut self,
                                                          name: N,
                                                          text: T)
                                                          -> Result<(), XmlError>;

    fn write_text_elements<N: AsRef<[u8]>, T: AsRef<[u8]>, I: IntoIterator<Item = T>>
        (&mut self,
         name: N,
         values: I)
         -> Result<(), XmlError>;

    fn write_cdata_element<N: AsRef<[u8]>, T: AsRef<[u8]>>(&mut self,
                                                           name: N,
                                                           text: T)
                                                           -> Result<(), XmlError>;

    fn write_object<T: ToXml>(&mut self, object: T) -> Result<(), XmlError>;

    fn write_objects<T: ToXml, I: IntoIterator<Item = T>>(&mut self,
                                                          objects: I)
                                                          -> Result<(), XmlError>;
}

impl<W: ::std::io::Write> XmlWriterExt for XmlWriter<W> {
    fn write_text_element<N: AsRef<[u8]>, T: AsRef<[u8]>>(&mut self,
                                                          name: N,
                                                          text: T)
                                                          -> Result<(), XmlError> {
        let elem = Element::new(name);
        try!(self.write(Event::Start(elem.clone())));
        try!(self.write(Event::Text(Element::new(text))));
        self.write(Event::End(elem))
    }

    fn write_text_elements<N: AsRef<[u8]>, T: AsRef<[u8]>, I: IntoIterator<Item = T>>
        (&mut self,
         name: N,
         values: I)
         -> Result<(), XmlError> {
        for value in values {
            try!(self.write_text_element(&name, value));
        }

        Ok(())
    }

    fn write_cdata_element<N: AsRef<[u8]>, T: AsRef<[u8]>>(&mut self,
                                                           name: N,
                                                           text: T)
                                                           -> Result<(), XmlError> {
        let elem = Element::new(name);
        try!(self.write(Event::Start(elem.clone())));
        try!(self.write(Event::CData(Element::new(text))));
        self.write(Event::End(elem))
    }

    #[inline]
    fn write_object<T: ToXml>(&mut self, object: T) -> Result<(), XmlError> {
        object.to_xml(self)
    }

    fn write_objects<T: ToXml, I: IntoIterator<Item = T>>(&mut self,
                                                          objects: I)
                                                          -> Result<(), XmlError> {
        for object in objects {
            try!(object.to_xml(self));
        }

        Ok(())
    }
}
