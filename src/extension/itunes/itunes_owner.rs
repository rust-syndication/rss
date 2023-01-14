// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::io::Write;

use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::Error as XmlError;
use quick_xml::Writer;

use crate::toxml::{ToXml, WriterExt};

/// The contact information for the owner of an iTunes podcast.
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
pub struct ITunesOwner {
    /// The name of the owner.
    pub name: Option<String>,
    /// The email of the owner.
    pub email: Option<String>,
}

impl ITunesOwner {
    /// Return the name of this person.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesOwner;
    ///
    /// let mut owner = ITunesOwner::default();
    /// owner.set_name("John Doe".to_string());
    /// assert_eq!(owner.name(), Some("John Doe"));
    /// ```
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set the name of this person.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesOwner;
    ///
    /// let mut owner = ITunesOwner::default();
    /// owner.set_name("John Doe".to_string());
    /// ```
    pub fn set_name<V>(&mut self, name: V)
    where
        V: Into<Option<String>>,
    {
        self.name = name.into();
    }

    /// Return the email of this person.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesOwner;
    ///
    /// let mut owner = ITunesOwner::default();
    /// owner.set_email("johndoe@example.com".to_string());
    /// assert_eq!(owner.email(), Some("johndoe@example.com"));
    /// ```
    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    /// Set the email of this person.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::itunes::ITunesOwner;
    ///
    /// let mut owner = ITunesOwner::default();
    /// owner.set_email("johndoe@example.com".to_string());
    /// ```
    pub fn set_email<V>(&mut self, email: V)
    where
        V: Into<Option<String>>,
    {
        self.email = email.into();
    }
}

impl ToXml for ITunesOwner {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = "itunes:owner";

        writer.write_event(Event::Start(BytesStart::new(name)))?;

        if let Some(name) = self.name.as_ref() {
            writer.write_text_element("itunes:name", name)?;
        }

        if let Some(email) = self.email.as_ref() {
            writer.write_text_element("itunes:email", email)?;
        }

        writer.write_event(Event::End(BytesEnd::new(name)))?;
        Ok(())
    }
}

#[cfg(feature = "builders")]
impl ITunesOwnerBuilder {
    /// Builds a new `ITunesOwner`.
    pub fn build(&self) -> ITunesOwner {
        self.build_impl().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "builders")]
    fn test_builder() {
        assert_eq!(
            ITunesOwnerBuilder::default()
                .name("John Doe".to_string())
                .build(),
            ITunesOwner {
                name: Some("John Doe".to_string()),
                email: None,
            }
        );
        assert_eq!(
            ITunesOwnerBuilder::default()
                .name("John Doe".to_string())
                .email("johndoe@example.com".to_string())
                .build(),
            ITunesOwner {
                name: Some("John Doe".to_string()),
                email: Some("johndoe@example.com".to_string()),
            }
        );
    }
}
