// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::io::{BufRead, Write};

use quick_xml::events::attributes::Attributes;
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::Error as XmlError;
use quick_xml::Reader;
use quick_xml::Writer;

use crate::error::Error;
use crate::toxml::{ToXml, WriterExt};
use crate::util::element_text;

/// Represents a text input for an RSS channel.
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
pub struct TextInput {
    /// The label of the Submit button for the text input.
    pub title: String,
    /// A description of the text input.
    pub description: String,
    /// The name of the text object.
    pub name: String,
    /// The URL of the CGI script that processes the text input request.
    pub link: String,
}

impl TextInput {
    /// Return the title for this text field.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::TextInput;
    ///
    /// let mut text_input = TextInput::default();
    /// text_input.set_title("Input Title");
    /// assert_eq!(text_input.title(), "Input Title");
    /// ```
    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    /// Set the title for this text field.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::TextInput;
    ///
    /// let mut text_input = TextInput::default();
    /// text_input.set_title("Input Title");
    /// ```
    pub fn set_title<V>(&mut self, title: V)
    where
        V: Into<String>,
    {
        self.title = title.into();
    }

    /// Return the description of this text field.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::TextInput;
    ///
    /// let mut text_input = TextInput::default();
    /// text_input.set_description("Input description");
    /// assert_eq!(text_input.description(), "Input description");
    /// ```
    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    /// Set the description of this text field.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::TextInput;
    ///
    /// let mut text_input = TextInput::default();
    /// text_input.set_description("Input description");
    /// ```
    pub fn set_description<V>(&mut self, description: V)
    where
        V: Into<String>,
    {
        self.description = description.into();
    }

    /// Return the name of the text object in this input.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::TextInput;
    ///
    /// let mut text_input = TextInput::default();
    /// text_input.set_name("Input name");
    /// assert_eq!(text_input.name(), "Input name");
    /// ```
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Set the name of the text object in this input.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::TextInput;
    ///
    /// let mut text_input = TextInput::default();
    /// text_input.set_name("Input name");;
    /// ```
    pub fn set_name<V>(&mut self, name: V)
    where
        V: Into<String>,
    {
        self.name = name.into();
    }

    /// Return the URL of the GCI script that processes the text input request.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::TextInput;
    ///
    /// let mut text_input = TextInput::default();
    /// text_input.set_link("http://example.com/submit");
    /// assert_eq!(text_input.link(), "http://example.com/submit");
    /// ```
    pub fn link(&self) -> &str {
        self.link.as_str()
    }

    /// Set the URL of the GCI script that processes the text input request.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::TextInput;
    ///
    /// let mut text_input = TextInput::default();
    /// text_input.set_link("http://example.com/submit");
    /// ```
    pub fn set_link<V>(&mut self, link: V)
    where
        V: Into<String>,
    {
        self.link = link.into();
    }
}

impl TextInput {
    /// Builds a TextInput from source XML
    pub fn from_xml<R: BufRead>(reader: &mut Reader<R>, _: Attributes) -> Result<Self, Error> {
        let mut text_input = TextInput::default();
        let mut buf = Vec::new();

        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(element) => match element.name() {
                    b"title" => text_input.title = element_text(reader)?.unwrap_or_default(),
                    b"description" => {
                        text_input.description = element_text(reader)?.unwrap_or_default()
                    }
                    b"name" => text_input.name = element_text(reader)?.unwrap_or_default(),
                    b"link" => text_input.link = element_text(reader)?.unwrap_or_default(),
                    n => reader.read_to_end(n, &mut Vec::new())?,
                },
                Event::End(_) => break,
                Event::Eof => return Err(Error::Eof),
                _ => {}
            }

            buf.clear();
        }

        Ok(text_input)
    }
}

impl ToXml for TextInput {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = b"textInput";

        writer.write_event(Event::Start(BytesStart::borrowed(name, name.len())))?;

        writer.write_text_element(b"title", &self.title)?;
        writer.write_text_element(b"description", &self.description)?;
        writer.write_text_element(b"name", &self.name)?;
        writer.write_text_element(b"link", &self.link)?;

        writer.write_event(Event::End(BytesEnd::borrowed(name)))?;
        Ok(())
    }
}

#[cfg(feature = "builders")]
impl TextInputBuilder {
    /// Builds a new `TextInput`.
    pub fn build(&self) -> TextInput {
        self.build_impl().unwrap()
    }
}
