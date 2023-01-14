// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::io::{BufRead, Write};

use quick_xml::events::{BytesStart, Event};
use quick_xml::Error as XmlError;
use quick_xml::Reader;
use quick_xml::Writer;

use crate::error::Error;
use crate::toxml::ToXml;
use crate::util::{attr_value, decode, skip};

/// Represents a cloud in an RSS feed.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[cfg_attr(
    feature = "builders",
    builder(
        setter(into),
        default,
        build_fn(name = "build_impl", private, error = "never::Never")
    )
)]
pub struct Cloud {
    /// The domain to register with.
    pub domain: String,
    /// The port to register with.
    pub port: String,
    /// The path to register with.
    pub path: String,
    /// The procedure to register with.
    pub register_procedure: String,
    /// The protocol to register with.
    pub protocol: String,
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
    pub fn from_xml<'s, R: BufRead>(
        reader: &mut Reader<R>,
        element: &'s BytesStart<'s>,
    ) -> Result<Self, Error> {
        let mut cloud = Cloud::default();

        for att in element.attributes().with_checks(false).flatten() {
            match decode(att.key.as_ref(), reader)?.as_ref() {
                "domain" => cloud.domain = attr_value(&att, reader)?.to_string(),
                "port" => cloud.port = attr_value(&att, reader)?.to_string(),
                "path" => cloud.path = attr_value(&att, reader)?.to_string(),
                "registerProcedure" => {
                    cloud.register_procedure = attr_value(&att, reader)?.to_string()
                }
                "protocol" => cloud.protocol = attr_value(&att, reader)?.to_string(),
                _ => {}
            }
        }

        skip(element.name(), reader)?;

        Ok(cloud)
    }
}

impl ToXml for Cloud {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = "cloud";
        let mut element = BytesStart::new(name);

        element.push_attribute(("domain", self.domain.as_str()));
        element.push_attribute(("port", self.port.as_str()));
        element.push_attribute(("path", self.path.as_str()));
        element.push_attribute(("registerProcedure", self.register_procedure.as_str()));
        element.push_attribute(("protocol", self.protocol.as_str()));

        writer.write_event(Event::Empty(element))?;
        Ok(())
    }
}

#[cfg(feature = "builders")]
impl CloudBuilder {
    /// Builds a new `Cloud`.
    pub fn build(&self) -> Cloud {
        self.build_impl().unwrap()
    }
}
