// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use error::Error;
use fromxml::{FromXml, element_text};
use quick_xml::errors::Error as XmlError;
use quick_xml::events::{Event, BytesStart, BytesEnd};
use quick_xml::events::attributes::Attributes;
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use toxml::{ToXml, WriterExt};
use url::Url;

/// A representation of the `<textInput>` element.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TextInput {
    /// The label of the Submit button for the text input.
    title: String,
    /// A description of the text input.
    description: String,
    /// The name of the text object.
    name: String,
    /// The URL of the CGI script that processes the text input request.
    link: String,
}

impl TextInput {
    /// Return the title for this `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::TextInputBuilder;
    ///
    /// let title = "Enter Comment";
    ///
    /// let text_input = TextInputBuilder::default()
    ///     .title(title)
    ///     .finalize();
    ///
    /// assert_eq!(title, text_input.title());
    /// ```
    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    /// Return the description of this `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::TextInputBuilder;
    ///
    /// let description = "Provided Feedback";
    ///
    /// let text_input = TextInputBuilder::default()
    ///     .description(description)
    ///     .finalize();
    ///
    /// assert_eq!(description, text_input.description());
    /// ```
    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    /// Return the name of this `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::TextInputBuilder;
    ///
    /// let name = "Comment";
    ///
    /// let text_input = TextInputBuilder::default()
    ///     .name(name)
    ///     .finalize();
    ///
    /// assert_eq!(name, text_input.name());
    /// ```
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Return the submission URL for this `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::TextInputBuilder;
    ///
    /// let link = "http://www.example.com/feedback";
    ///
    /// let text_input = TextInputBuilder::default()
    ///     .link(link)
    ///     .finalize();
    ///
    /// assert_eq!(link, text_input.link());
    /// ```
    pub fn link(&self) -> &str {
        self.link.as_str()
    }
}

impl FromXml for TextInput {
    fn from_xml<R: ::std::io::BufRead>(reader: &mut Reader<R>,
                                       _: Attributes)
                                       -> Result<Self, Error> {
        let mut title = None;
        let mut description = None;
        let mut name = None;
        let mut link = None;
        let mut buf = Vec::new();
        let mut skip_buf = Vec::new();

        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(element) => {
                    match element.name() {
                        b"title" => title = element_text(reader)?,
                        b"description" => description = element_text(reader)?,
                        b"name" => name = element_text(reader)?,
                        b"link" => link = element_text(reader)?,
                        n => reader.read_to_end(n, &mut skip_buf)?,
                    }
                }
                Event::End(_) => {
                    let title = title.unwrap_or_default();
                    let description = description.unwrap_or_default();
                    let name = name.unwrap_or_default();
                    let link = link.unwrap_or_default();

                    return Ok(TextInput {
                                  title: title,
                                  description: description,
                                  name: name,
                                  link: link,
                              });
                }
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }

        Err(Error::EOF)
    }
}

impl ToXml for TextInput {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = b"textInput";

        writer
            .write_event(Event::Start(BytesStart::borrowed(name, name.len())))?;

        writer.write_text_element(b"title", &self.title)?;
        writer
            .write_text_element(b"description", &self.description)?;
        writer.write_text_element(b"name", &self.name)?;
        writer.write_text_element(b"link", &self.link)?;

        writer.write_event(Event::End(BytesEnd::borrowed(name)))?;
        Ok(())
    }
}

/// A builder used to create a `TextInput`.
#[derive(Debug, Clone, Default)]
pub struct TextInputBuilder {
    title: String,
    description: String,
    name: String,
    link: String,
}

impl TextInputBuilder {
    /// Construct a new `TextInputBuilder` using the values from an existing `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{Channel, TextInputBuilder};
    ///
    /// let input = include_str!("tests/data/textinput.xml");
    /// let channel = input.parse::<Channel>().unwrap();
    /// let text_input = channel.text_input().unwrap().clone();
    /// let builder = TextInputBuilder::from_text_input(text_input);
    /// ```
    pub fn from_text_input(text_input: TextInput) -> Self {
        TextInputBuilder {
            title: text_input.title,
            description: text_input.description,
            name: text_input.name,
            link: text_input.link,
        }
    }


    /// Set the title for the `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::TextInputBuilder;
    ///
    /// let builder = TextInputBuilder::default()
    ///     .title("Title");
    /// ```
    pub fn title<S>(mut self, title: S) -> TextInputBuilder
        where S: Into<String>
    {
        self.title = title.into();
        self
    }

    /// Set the description of the `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::TextInputBuilder;
    ///
    /// let builder = TextInputBuilder::default()
    ///     .description("This is a test description.");
    /// ```
    pub fn description<S>(mut self, description: S) -> TextInputBuilder
        where S: Into<String>
    {
        self.description = description.into();
        self
    }

    /// Set the name of the `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::TextInputBuilder;
    ///
    /// let builder = TextInputBuilder::default()
    ///     .name("Comments");
    /// ```
    pub fn name<S>(mut self, name: S) -> TextInputBuilder
        where S: Into<String>
    {
        self.name = name.into();
        self
    }

    /// Set the submission URL for the `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::TextInputBuilder;
    ///
    /// let builder = TextInputBuilder::default()
    ///     .link("http://www.example.com/feedback");
    /// ```
    pub fn link<S>(mut self, link: S) -> TextInputBuilder
        where S: Into<String>
    {
        self.link = link.into();
        self
    }

    /// Validate the contents of this `TextInputBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::TextInputBuilder;
    ///
    /// let text_input = TextInputBuilder::default()
    ///         .title("Title")
    ///         .description("This is a test description.")
    ///         .name("Comments")
    ///         .link("http://www.example.com/feedback")
    ///         .validate()
    ///         .unwrap();
    /// ```
    pub fn validate(self) -> Result<TextInputBuilder, Error> {
        Url::parse(self.link.as_str())?;

        Ok(self)
    }

    /// Construct the `TextInput` from this `TextInputBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::TextInputBuilder;
    ///
    /// let text_input = TextInputBuilder::default()
    ///         .title("Title")
    ///         .description("This is a test description.")
    ///         .name("Comments")
    ///         .link("http://www.example.com/feedback")
    ///         .finalize();
    /// ```
    pub fn finalize(self) -> TextInput {
        TextInput {
            title: self.title,
            description: self.description,
            name: self.name,
            link: self.link,
        }
    }
}
