// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use error::Error;
use quick_xml::{Element, Event, XmlWriter};
use quick_xml::error::Error as XmlError;
use toxml::{ToXml, XmlWriterExt};

/// The contact information for the owner of an iTunes podcast.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ITunesOwner {
    /// The name of the owner.
    name: Option<String>,
    /// The email of the email.
    email: Option<String>,
}

impl ITunesOwner {
    /// Get the optional name that exists under `ITunesOwner`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesOwnerBuilder, ITunesOwner};
    ///
    /// let name = "name";
    ///
    /// let owner = ITunesOwnerBuilder::new()
    ///     .name(Some(name.to_string()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(Some(name), owner.name());
    /// ```
    ///
    // ```
    /// use feed::extension::itunes::{ITunesOwnerBuilder, ITunesOwner};
    ///
    /// let owner = ITunesOwnerBuilder::new()
    ///     .name(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let name_opt = owner.name();
    /// assert!(name_opt.is_none());
    /// ```
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|s| s.as_str())
    }


    /// Get the optional email that exists under `ITunesOwner`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::{ITunesOwnerBuilder, ITunesOwner};
    ///
    /// let email = "email@example.com";
    ///
    /// let owner = ITunesOwnerBuilder::new()
    ///     .email(Some(email.to_string()))
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(Some(email), owner.email());
    /// ```
    ///
    // ```
    /// use feed::extension::itunes::{ITunesOwnerBuilder, ITunesOwner};
    ///
    /// let owner = ITunesOwnerBuilder::new()
    ///     .email(None)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let email_opt = owner.email();
    /// assert!(email_opt.is_none());
    /// ```
    pub fn email(&self) -> Option<&str> {
        self.email.as_ref().map(|s| s.as_str())
    }
}

impl ToXml for ITunesOwner {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut XmlWriter<W>) -> Result<(), XmlError> {
        let element = Element::new(b"itunes:owner");

        writer.write(Event::Start(element.clone()))?;

        if let Some(name) = self.name.as_ref() {
            writer.write_text_element(b"name", name)?;
        }

        if let Some(email) = self.email.as_ref() {
            writer.write_text_element(b"email", email)?;
        }

        writer.write(Event::End(element))
    }
}

/// This `ITunesOwnerBuilder` struct creates the `ITunesOwner`.
#[derive(Debug, Clone, Default)]
pub struct ITunesOwnerBuilder {
    name: Option<String>,
    email: Option<String>,
}

impl ITunesOwnerBuilder {
    /// Construct a new `ITunesOwnerBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesOwnerBuilder;
    ///
    /// let owner_builder = ITunesOwnerBuilder::new();
    /// ```
    pub fn new() -> ITunesOwnerBuilder {
        ITunesOwnerBuilder::default()
    }

    /// Set the optional name that exists under `ITunesOwner`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesOwnerBuilder;
    ///
    /// let mut owner_builder = ITunesOwnerBuilder::new();
    /// owner_builder.name(Some("name".to_string()));
    /// ```
    pub fn name(mut self, name: Option<String>) -> ITunesOwnerBuilder {
        self.name = name;
        self
    }

    /// Set the optional email that exists under `ITunesOwner`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesOwnerBuilder;
    ///
    /// let mut owner_builder = ITunesOwnerBuilder::new();
    /// owner_builder.email(Some("email@example.com".to_string()));
    /// ```
    pub fn email(mut self, email: Option<String>) -> ITunesOwnerBuilder {
        self.email = email;
        self
    }

    /// Construct the `ITunesOwner` from the `ITunesOwnerBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesOwnerBuilder;
    ///
    /// let owner = ITunesOwnerBuilder::new()
    ///     .email(Some("email@example.com".to_string()))
    ///     .name(Some("name".to_string()))
    ///     .finalize()
    ///     .unwrap();
    /// ```
    pub fn finalize(self) -> Result<ITunesOwner, Error> {
        Ok(ITunesOwner {
               name: self.name,
               email: self.email,
           })
    }
}
