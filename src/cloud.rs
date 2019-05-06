// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::io::{BufRead, Write};

use quick_xml::Error as XmlError;
use quick_xml::events::{BytesStart, Event};
use quick_xml::events::attributes::Attributes;
use quick_xml::Reader;
use quick_xml::Writer;

use error::Error;
use toxml::ToXml;

/// Represents a cloud in an RSS feed.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, Builder)]
#[builder(setter(into), default)]
pub struct Cloud {
    /// The domain to register with.
    domain: String,
    /// The port to register with.
    port: String,
    /// The path to register with.
    path: String,
    /// The procedure to register with.
    register_procedure: String,
    /// The protocol to register with.
    protocol: String,
}

impl Cloud {
    /// Return the domain for this cloud.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Cloud;
    ///
    /// let mut cloud = Cloud::default();
    /// cloud.set_domain("http://example.com");
    /// assert_eq!(cloud.domain(), "http://example.com");
    /// ```
    pub fn domain(&self) -> &str {
        self.domain.as_str()
    }

    /// Set the domain for this cloud.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Cloud;
    ///
    /// let mut cloud = Cloud::default();
    /// cloud.set_domain("http://example.com");
    /// ```
    pub fn set_domain<V>(&mut self, domain: V)
    where
        V: Into<String>,
    {
        self.domain = domain.into();
    }

    /// Return the port for this cloud.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Cloud;
    ///
    /// let mut cloud = Cloud::default();
    /// cloud.set_port("80");
    /// assert_eq!(cloud.port(), "80");
    /// ```
    pub fn port(&self) -> &str {
        self.port.as_str()
    }

    /// Set the port for this cloud.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Cloud;
    ///
    /// let mut cloud = Cloud::default();
    /// cloud.set_port("80");
    /// ```
    pub fn set_port<V>(&mut self, port: V)
    where
        V: Into<String>,
    {
        self.port = port.into();
    }

    /// Return the path for this cloud.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Cloud;
    ///
    /// let mut cloud = Cloud::default();
    /// cloud.set_port("/rpc");
    /// assert_eq!(cloud.port(), "/rpc");
    /// ```
    pub fn path(&self) -> &str {
        self.path.as_str()
    }

    /// Set the path for this cloud.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Cloud;
    ///
    /// let mut cloud = Cloud::default();
    /// cloud.set_path("/rpc");
    /// ```
    pub fn set_path<V>(&mut self, path: V)
    where
        V: Into<String>,
    {
        self.path = path.into();
    }

    /// Return the register procedure for this cloud.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Cloud;
    ///
    /// let mut cloud = Cloud::default();
    /// cloud.set_register_procedure("pingMe");
    /// assert_eq!(cloud.register_procedure(), "pingMe");
    /// ```
    pub fn register_procedure(&self) -> &str {
        self.register_procedure.as_str()
    }

    /// Set the register procedure for this cloud.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Cloud;
    ///
    /// let mut cloud = Cloud::default();
    /// cloud.set_register_procedure("pingMe");
    /// ```
    pub fn set_register_procedure<V>(&mut self, register_procedure: V)
    where
        V: Into<String>,
    {
        self.register_procedure = register_procedure.into();
    }

    /// Return the protocol for this cloud.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Cloud;
    ///
    /// let mut cloud = Cloud::default();
    /// cloud.set_protocol("xml-rpc");
    /// assert_eq!(cloud.protocol(), "xml-rpc");
    /// ```
    pub fn protocol(&self) -> &str {
        self.protocol.as_str()
    }

    /// Set the protocol for this cloud.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Cloud;
    ///
    /// let mut cloud = Cloud::default();
    /// cloud.set_protocol("xml-rpc");
    /// ```
    pub fn set_protocol<V>(&mut self, protocol: V)
    where
        V: Into<String>,
    {
        self.protocol = protocol.into();
    }
}

impl Cloud {
    /// Builds a Cloud from source XML
    pub fn from_xml<R: BufRead>(reader: &mut Reader<R>, mut atts: Attributes) -> Result<Self, Error> {
        let mut cloud = Cloud::default();

        for attr in atts.with_checks(false) {
            if let Ok(att) = attr {
                match att.key {
                    b"domain" => {
                        cloud.domain = att.unescape_and_decode_value(reader)?;
                    }
                    b"port" => {
                        cloud.port = att.unescape_and_decode_value(reader)?;
                    }
                    b"path" => {
                        cloud.path = att.unescape_and_decode_value(reader)?;
                    }
                    b"registerProcedure" => {
                        cloud.register_procedure = att.unescape_and_decode_value(reader)?;
                    }
                    b"protocol" => {
                        cloud.protocol = att.unescape_and_decode_value(reader)?;
                    }
                    _ => {}
                }
            }
        }

        reader.read_to_end(b"cloud", &mut Vec::new())?;

        Ok(cloud)
    }
}

impl ToXml for Cloud {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = b"cloud";
        let mut element = BytesStart::borrowed(name, name.len());

        let attrs = &[
            (b"domain" as &[u8], self.domain.as_bytes()),
            (b"port", self.port.as_bytes()),
            (b"path", self.path.as_bytes()),
            (b"registerProcedure", self.register_procedure.as_bytes()),
            (b"protocol", self.protocol.as_bytes()),
        ];
        element.extend_attributes(attrs.iter().cloned());

        writer.write_event(Event::Empty(element))?;
        Ok(())
    }
}
