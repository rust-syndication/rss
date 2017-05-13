// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use error::Error;
use fromxml::FromXml;
use mime::Mime;
use quick_xml::{Element, Event, XmlReader, XmlWriter};
use quick_xml::error::Error as XmlError;
use toxml::ToXml;
use url::Url;

/// A representation of the `<enclosure>` element.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Enclosure
{
    /// The URL of the enclosure.
    url: String,
    /// The length of the enclosure in bytes.
    length: String,
    /// The MIME type of the enclosure.
    mime_type: String,
}

impl Enclosure
{
    /// Get the url that exists under `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{EnclosureBuilder, Enclosure};
    ///
    /// let url = "http://www.podtrac.com/pts/redirect.ogg/".to_owned()
    /// + "traffic.libsyn.com/jnite/linuxactionshowep408.ogg";
    ///
    /// let enclosure = EnclosureBuilder::new()
    ///     .url(url.as_ref())
    ///     .mime_type("audio/ogg")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(url.to_owned(), enclosure.url())
    /// ```
    pub fn url(&self) -> String
    {
        self.url
            .clone()
    }


    /// Get the length that exists under `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{EnclosureBuilder, Enclosure};
    ///
    /// let length: i64 = 70772893;
    ///
    /// let url = "http://www.podtrac.com/pts/redirect.ogg/".to_owned()
    /// + "traffic.libsyn.com/jnite/linuxactionshowep408.ogg";
    ///
    /// let enclosure = EnclosureBuilder::new()
    ///     .url(url.as_str())
    ///     .length(length)
    ///     .mime_type("audio/ogg")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(length.to_string(), enclosure.length())
    /// ```
    pub fn length(&self) -> String
    {
        self.length
            .clone()
    }


    /// Get the enclosure type that exists under `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{EnclosureBuilder, Enclosure};
    ///
    /// let enclosure_type = "audio/ogg";
    ///
    /// let url = "http://www.podtrac.com/pts/redirect.ogg/".to_owned()
    /// + "traffic.libsyn.com/jnite/linuxactionshowep408.ogg";
    ///
    /// let enclosure = EnclosureBuilder::new()
    ///     .url(url.as_str())
    ///     .mime_type(enclosure_type)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(enclosure_type.to_owned(), enclosure.mime_type())
    /// ```
    pub fn mime_type(&self) -> String
    {
        self.mime_type
            .clone()
    }
}

impl FromXml for Enclosure
{
    /// TODO: document from xml
    fn from_xml<R: ::std::io::BufRead>(mut reader: XmlReader<R>,
                                       element: Element)
        -> Result<(Self, XmlReader<R>), Error>
    {
        let mut url = None;
        let mut length = None;
        let mut mime_type = None;

        for attr in element.attributes()
                           .with_checks(false)
                           .unescaped() {
            if let Ok(attr) = attr {
                match attr.0 {
                    b"url" if url.is_none() => {
                        url = Some(String::from_utf8(attr.1
                                                         .into_owned())?);
                    },
                    b"length" if length.is_none() => {
                        length = Some(String::from_utf8(attr.1
                                                            .into_owned())?);
                    },
                    b"type" if mime_type.is_none() => {
                        mime_type = Some(String::from_utf8(attr.1
                                                               .into_owned())?);
                    },
                    _ => {},
                }
            }
        }

        skip_element!(reader);

        let url = url.unwrap_or_default();
        let length = length.unwrap_or_default();
        let mime_type = mime_type.unwrap_or_default();

        Ok((Enclosure { url: url,
                        length: length,
                        mime_type: mime_type, },
            reader))
    }
}

impl ToXml for Enclosure
{
    /// TODO: document to xml
    fn to_xml<W: ::std::io::Write>(&self,
                                   writer: &mut XmlWriter<W>)
        -> Result<(), XmlError>
    {
        let element = Element::new(b"enclosure");

        writer.write(Event::Start({
                                      let mut element = element.clone();

                                      let attrs = &[(b"url" as &[u8], &self.url),
                                                    (b"length", &self.length),
                                                    (b"type", &self.mime_type)];
                                      element.extend_attributes(attrs.into_iter()
                                                                     .map(|v| *v));

                                      element
                                  }))?;

        writer.write(Event::End(element))
    }
}

/// This `EnclosureBuilder` struct creates the `Enclosure`.
#[derive(Debug, Clone, Default)]
pub struct EnclosureBuilder
{
    url: String,
    length: i64,
    mime_type: String,
}

impl EnclosureBuilder
{
    /// Construct a new `EnclosureBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::EnclosureBuilder;
    ///
    /// let enclosure_builder = EnclosureBuilder::new();
    /// ```
    pub fn new() -> EnclosureBuilder
    {
        EnclosureBuilder::default()
    }


    /// Set the url that exists under `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::EnclosureBuilder;
    ///
    /// let url = "http://www.podtrac.com/pts/".to_owned()
    /// + "redirect.ogg/traffic.libsyn.com/jnite/linuxactionshowep408.ogg";
    ///
    /// let mut enclosure_builder = EnclosureBuilder::new();
    /// enclosure_builder.url(url.as_ref());
    /// ```
    pub fn url(&mut self,
               url: &str)
        -> &mut EnclosureBuilder
    {
        self.url = url.to_owned();
        self
    }


    /// Set the length that exists under `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::EnclosureBuilder;
    ///
    /// let mut enclosure_builder = EnclosureBuilder::new();
    /// enclosure_builder.length(70772893);
    /// ```
    pub fn length(&mut self,
                  length: i64)
        -> &mut EnclosureBuilder
    {
        self.length = length;
        self
    }


    /// Set the enclosure_type that exists under `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::EnclosureBuilder;
    ///
    /// let mut enclosure_builder = EnclosureBuilder::new();
    /// enclosure_builder.mime_type("audio/ogg");
    /// ```
    pub fn mime_type(&mut self,
                     mime_type: &str)
        -> &mut EnclosureBuilder
    {
        self.mime_type = mime_type.to_owned();
        self
    }


    /// Validate the contents of `Enclosure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::EnclosureBuilder;
    ///
    /// let url = "http://www.podtrac.com/pts/redirect.ogg/".to_owned()
    /// + "traffic.libsyn.com/jnite/linuxactionshowep408.ogg";
    ///
    /// let enclosure = EnclosureBuilder::new()
    ///         .url(url.as_ref())
    ///         .length(70772893)
    ///         .mime_type("audio/ogg")
    ///         .validate().unwrap()
    ///         .finalize().unwrap();
    /// ```
    pub fn validate(&mut self) -> Result<&mut EnclosureBuilder, Error>
    {
        Url::parse(self.url
                       .as_str())?;

        let mime = self.mime_type
                       .parse::<Mime>();
        if mime.is_err() {
            return Err(Error::Validation(String::from(format!("Error: {:?}",
                                                              mime.unwrap_err()))));
        }

        if self.length < 0 {
            return Err(Error::Validation(String::from("Enclosure Length cannot be a negative value")));
        }

        Ok(self)
    }


    /// Construct the `Enclosure` from the `EnclosureBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::EnclosureBuilder;
    ///
    /// let url = "http://www.podtrac.com/pts/redirect.ogg/".to_owned()
    /// + "traffic.libsyn.com/jnite/linuxactionshowep408.ogg";
    ///
    /// let enclosure = EnclosureBuilder::new()
    ///         .url(url.as_ref())
    ///         .length(70772893)
    ///         .mime_type("audio/ogg")
    ///         .finalize();
    /// ```
    pub fn finalize(&self) -> Result<Enclosure, Error>
    {
        let length = self.length
                         .to_string();

        Ok(Enclosure { url: self.url
                                .clone(),
                       length: length,
                       mime_type: self.mime_type
                                      .clone(), })
    }
}
