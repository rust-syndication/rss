use error::Error;

pub trait XmlName {
    fn local_name(&self) -> &[u8];
    #[cfg(not(feature = "quick-xml"))]
    fn prefix(&self) -> Option<&[u8]>;
}

pub trait XmlAttribute {
    type Name: XmlName;
    fn name(&self) -> &Self::Name;
    fn borrowed_value(&self) -> Result<&str, ::std::str::Utf8Error>;
    fn owned_value(self) -> Result<String, ::std::str::Utf8Error>;
}

#[cfg(feature = "quick-xml")]
impl<'a> XmlName for &'a [u8] {
    #[inline]
    fn local_name(&self) -> &[u8] {
        self
    }
}

#[cfg(feature = "quick-xml")]
impl<'a> XmlAttribute for (&'a [u8], &'a [u8]) {
    type Name = &'a [u8];

    #[inline]
    fn name(&self) -> &Self::Name {
        &self.0
    }

    #[inline]
    fn borrowed_value(&self) -> Result<&str, ::std::str::Utf8Error> {
        ::std::str::from_utf8(self.1)
    }

    #[inline]
    fn owned_value(self) -> Result<String, ::std::str::Utf8Error> {
        ::std::str::from_utf8(self.1).map(|s| ::std::borrow::ToOwned::to_owned(s))
    }
}

#[cfg(feature = "xml-rs")]
impl XmlName for ::xml::name::OwnedName {
    #[inline]
    fn local_name(&self) -> &[u8] {
        self.local_name.as_bytes()
    }

    #[inline]
    fn prefix(&self) -> Option<&[u8]> {
        self.prefix.as_ref().map(|s| s.as_bytes())
    }
}

#[cfg(feature = "xml-rs")]
impl XmlAttribute for ::xml::attribute::OwnedAttribute {
    type Name = ::xml::name::OwnedName;

    #[inline]
    fn name(&self) -> &Self::Name {
        &self.name
    }

    #[inline]
    fn borrowed_value(&self) -> Result<&str, ::std::str::Utf8Error> {
        Ok(::std::borrow::Borrow::borrow(&self.value))
    }

    #[inline]
    fn owned_value(self) -> Result<String, ::std::str::Utf8Error> {
        Ok(self.value)
    }
}

pub trait FromXml {
    #[allow(unused_variables)]
    fn consume_content(&mut self, content: String) {}

    #[allow(unused_variables)]
    fn consume_named<T: XmlName>(&mut self, name: T, content: String) {}

    #[allow(unused_variables)]
    fn consume_attribute<T: XmlAttribute>(&mut self, attr: T) -> Result<(), Error> {
        Ok(())
    }
}

