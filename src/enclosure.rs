// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use error::Error;
use fromxml::FromXml;
use mime::Mime;
use quick_xml::errors::Error as XmlError;
use quick_xml::events::{Event, BytesStart};
use quick_xml::events::attributes::Attributes;
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use toxml::ToXml;
use url::Url;

/// A representation of the `<enclosure>` element.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Enclosure {
    /// The URL of the enclosure.
    url: String,
    /// The length of the enclosure in bytes.
    length: String,
    /// The MIME type of the enclosure.
    mime_type: String,
}

impl Enclosure {
    /// Return the URL for this `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::EnclosureBuilder;
    ///
    /// let url = "http://www.podtrac.com/pts/redirect.ogg/traffic.libsyn.com/jnite/\
    ///     linuxactionshowep408.ogg";
    ///
    /// let enclosure = EnclosureBuilder::default()
    ///     .url(url)
    ///     .finalize();
    ///
    /// assert_eq!(url, enclosure.url())
    /// ```
    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    /// Return the content length for this `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::EnclosureBuilder;
    ///
    /// let length = 70772893;
    ///
    /// let enclosure = EnclosureBuilder::default()
    ///     .length(length)
    ///     .finalize();
    ///
    /// assert_eq!(length.to_string(), enclosure.length())
    /// ```
    pub fn length(&self) -> &str {
        self.length.as_str()
    }

    /// Return the content MIME type for this `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::EnclosureBuilder;
    ///
    /// let mime_type = "audio/ogg";
    ///
    /// let enclosure = EnclosureBuilder::default()
    ///     .mime_type(mime_type)
    ///     .finalize();
    ///
    /// assert_eq!(mime_type, enclosure.mime_type())
    /// ```
    pub fn mime_type(&self) -> &str {
        self.mime_type.as_str()
    }
}

impl FromXml for Enclosure {
    fn from_xml<R: ::std::io::BufRead>(reader: &mut Reader<R>,
                                       mut atts: Attributes)
                                       -> Result<Self, Error> {
        let mut url = None;
        let mut length = None;
        let mut mime_type = None;

        for attr in atts.with_checks(false) {
            if let Ok(attr) = attr {
                match attr.key {
                    b"url" if url.is_none() => {
                        url = Some(attr.unescape_and_decode_value(reader)?);
                    }
                    b"length" if length.is_none() => {
                        length = Some(attr.unescape_and_decode_value(reader)?);
                    }
                    b"type" if mime_type.is_none() => {
                        mime_type = Some(attr.unescape_and_decode_value(reader)?);
                    }
                    _ => {}
                }
            }
        }

        let mut depth = 1;
        let mut buf = Vec::new();
        while depth > 0 {
            match reader.read_event(&mut buf)? {
                Event::Start(_) => depth += 1,
                Event::End(_) => depth -= 1,
                Event::Eof => break,
                _ => {}
            }
        }

        Ok(Enclosure {
               url: url.unwrap_or_default(),
               length: length.unwrap_or_default(),
               mime_type: mime_type.unwrap_or_default(),
           })
    }
}

impl ToXml for Enclosure {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = b"enclosure";

        let mut element = BytesStart::borrowed(name, name.len());

        let attrs = &[(b"url" as &[u8], self.url.as_bytes()),
                      (b"length", self.length.as_bytes()),
                      (b"type", self.mime_type.as_bytes())];
        element.extend_attributes(attrs.into_iter().map(|v| *v));

        writer.write_event(Event::Empty(element))?;
        Ok(())
    }
}

/// A builder used to create an `Enclosure`.
#[derive(Debug, Clone, Default)]
pub struct EnclosureBuilder {
    url: String,
    length: i64,
    mime_type: String,
}

impl EnclosureBuilder {
    /// Construct a new `EnclosureBuilder` using the values from an existing `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{Channel, EnclosureBuilder};
    ///
    /// let input = include_str!("tests/data/enclosure.xml");
    /// let channel = input.parse::<Channel>().unwrap();
    /// let enclosure = channel.items()[0].enclosure().unwrap().clone();
    /// let builder = EnclosureBuilder::from_enclosure(enclosure).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// If this function encounters an error while parsing `length` from a `String` to an `i64` it
    /// will return an [`IntParsing`](/rss/enum.Error.html#variant.IntParsing) error.
    pub fn from_enclosure(enclosure: Enclosure) -> Result<Self, Error> {
        Ok(EnclosureBuilder {
               url: enclosure.url,
               length: enclosure.length.parse()?,
               mime_type: enclosure.mime_type,
           })
    }

    /// Set the URL for the `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::EnclosureBuilder;
    ///
    /// let url = "http://www.podtrac.com/pts/redirect.ogg/traffic.libsyn.com/jnite\
    ///     /linuxactionshowep408.ogg";
    ///
    /// let builder = EnclosureBuilder::default()
    ///     .url(url);
    /// ```
    pub fn url<S>(mut self, url: S) -> EnclosureBuilder
        where S: Into<String>
    {
        self.url = url.into();
        self
    }

    /// Set the content length for the `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::EnclosureBuilder;
    ///
    /// let builder = EnclosureBuilder::default()
    ///     .length(70772893);
    /// ```
    pub fn length(mut self, length: i64) -> EnclosureBuilder {
        self.length = length;
        self
    }

    /// Set the content MIME type for the `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::EnclosureBuilder;
    ///
    /// let builder = EnclosureBuilder::default()
    ///     .mime_type("audio/ogg");
    /// ```
    pub fn mime_type<S>(mut self, mime_type: S) -> EnclosureBuilder
        where S: Into<String>
    {
        self.mime_type = mime_type.into();
        self
    }

    /// Validate the contents of this `EnclosureBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::EnclosureBuilder;
    ///
    /// let url = "http://www.podtrac.com/pts/redirect.ogg/traffic.libsyn.com/jnite/\
    ///     linuxactionshowep408.ogg";
    ///
    /// let enclosure = EnclosureBuilder::default()
    ///         .url(url)
    ///         .length(70772893)
    ///         .mime_type("audio/ogg")
    ///         .validate()
    ///         .unwrap();
    /// ```
    pub fn validate(self) -> Result<EnclosureBuilder, Error> {
        Url::parse(self.url.as_str())?;

        let mime = self.mime_type.parse::<Mime>();

        if mime.is_err() {
            return Err(Error::Validation(format!("Enclosure Mime Type is invalid: {:?}",
                                                 mime.unwrap_err())));
        }

        if self.length < 0 {
            return Err(Error::Validation("Enclosure Length cannot be a negative value"
                                             .to_string()));
        }

        Ok(self)
    }

    /// Construct the `Enclosure` from this `EnclosureBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::EnclosureBuilder;
    ///
    /// let url = "http://www.podtrac.com/pts/redirect.ogg/traffic.libsyn.com/jnite/\
    ///     linuxactionshowep408.ogg";
    ///
    /// let enclosure = EnclosureBuilder::default()
    ///         .url(url)
    ///         .length(70772893)
    ///         .mime_type("audio/ogg")
    ///         .finalize();
    /// ```
    pub fn finalize(self) -> Enclosure {
        Enclosure {
            url: self.url,
            length: self.length.to_string(),
            mime_type: self.mime_type,
        }
    }
}
