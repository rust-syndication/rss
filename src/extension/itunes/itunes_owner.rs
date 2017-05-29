// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use quick_xml::errors::Error as XmlError;
use quick_xml::events::{Event, BytesStart, BytesEnd};
use quick_xml::writer::Writer;
use toxml::{ToXml, WriterExt};

/// The contact information for the owner of an iTunes podcast.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ITunesOwner {
    /// The name of the owner.
    name: Option<String>,
    /// The email of the owner.
    email: Option<String>,
}

impl ITunesOwner {
    /// Return the name of this owner.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesOwnerBuilder;
    ///
    /// let name = "name";
    ///
    /// let owner = ITunesOwnerBuilder::default()
    ///     .name(name.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(name), owner.name());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::ITunesOwnerBuilder;
    ///
    /// let owner = ITunesOwnerBuilder::default()
    ///     .name(None)
    ///     .finalize();
    ///
    /// assert!(owner.name().is_none());
    /// ```
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|s| s.as_str())
    }

    /// Return the email of this owner.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesOwnerBuilder;
    ///
    /// let email = "email@example.com";
    ///
    /// let owner = ITunesOwnerBuilder::default()
    ///     .email(email.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(email), owner.email());
    /// ```
    ///
    /// ```
    /// use rss::extension::itunes::ITunesOwnerBuilder;
    ///
    /// let owner = ITunesOwnerBuilder::default()
    ///     .email(None)
    ///     .finalize();
    ///
    /// assert!(owner.email().is_none());
    /// ```
    pub fn email(&self) -> Option<&str> {
        self.email.as_ref().map(|s| s.as_str())
    }
}

impl ToXml for ITunesOwner {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = b"itunes:owner";

        writer
            .write_event(Event::Start(BytesStart::borrowed(name, name.len())))?;

        if let Some(name) = self.name.as_ref() {
            writer.write_text_element(b"name", name)?;
        }

        if let Some(email) = self.email.as_ref() {
            writer.write_text_element(b"email", email)?;
        }

        writer.write_event(Event::End(BytesEnd::borrowed(name)))?;
        Ok(())
    }
}

/// A builder used to create an `ITunesOwner`.
#[derive(Debug, Clone, Default)]
pub struct ITunesOwnerBuilder {
    name: Option<String>,
    email: Option<String>,
}

impl ITunesOwnerBuilder {
    /// Set the name of the owner.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesOwnerBuilder;
    ///
    /// let builder = ITunesOwnerBuilder::default()
    ///     .name("name".to_string());
    /// ```
    pub fn name<V: Into<Option<String>>>(mut self, name: V) -> ITunesOwnerBuilder {
        self.name = name.into();
        self
    }

    /// Set the email of the owner.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesOwnerBuilder;
    ///
    /// let builder = ITunesOwnerBuilder::default()
    ///     .email("email@example.com".to_string());
    /// ```
    pub fn email<V: Into<Option<String>>>(mut self, email: V) -> ITunesOwnerBuilder {
        self.email = email.into();
        self
    }

    /// Construct the `ITunesOwner` from this `ITunesOwnerBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesOwnerBuilder;
    ///
    /// let owner = ITunesOwnerBuilder::default()
    ///     .name("name".to_string())
    ///     .email("email@example.com".to_string())
    ///     .finalize();
    /// ```
    pub fn finalize(self) -> ITunesOwner {
        ITunesOwner {
            name: self.name,
            email: self.email,
        }
    }
}
