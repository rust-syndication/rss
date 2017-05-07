// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use error::Error;

use fromxml::FromXml;
use quick_xml::{Element, Event, XmlReader, XmlWriter};
use quick_xml::error::Error as XmlError;
use string_utils;
use toxml::{ToXml, XmlWriterExt};

/// A representation of the `<textInput>` element.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TextInput
{
    /// The label of the Submit button for the text input.
    title: String,
    /// A description of the text input.
    description: String,
    /// The name of the text object.
    name: String,
    /// The URL of the CGI script that processes the text input request.
    link: String,
}

impl TextInput
{
    /// Get the title that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{TextInputBuilder, TextInput};
    ///
    /// let title = "Enter Comment";
    ///
    /// let text_input = TextInputBuilder::new()
    ///     .title(title)
    ///     .link("http://www.example.com/feedback")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(title.to_owned(), text_input.title());
    /// ```
    pub fn title(&self) -> String
    {
        self.title.clone()
    }

    /// Get the description that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{TextInputBuilder, TextInput};
    ///
    /// let description = "Provided Feedback";
    ///
    /// let text_input = TextInputBuilder::new()
    ///     .description(description)
    ///     .link("http://www.example.com/feedback")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(description.to_owned(), text_input.description());
    /// ```
    pub fn description(&self) -> String
    {
        self.description.clone()
    }

    /// Get the name that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{TextInputBuilder, TextInput};
    ///
    /// let name = "Comment";
    ///
    /// let text_input = TextInputBuilder::new()
    ///     .name(name)
    ///     .link("http://www.example.com/feedback")
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(name.to_owned(), text_input.name());
    /// ```
    pub fn name(&self) -> String
    {
        self.name.clone()
    }

    /// Get the link that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{TextInputBuilder, TextInput};
    ///
    /// let link = "http://www.example.com/feedback";
    ///
    /// let text_input = TextInputBuilder::new()
    ///     .link(link)
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(link.to_owned(), text_input.link());
    /// ```
    pub fn link(&self) -> String
    {
        self.link.clone()
    }
}


impl FromXml for TextInput
{
    /// TODO
    fn from_xml<R: ::std::io::BufRead>(mut reader: XmlReader<R>,
                                       _: Element)
        -> Result<(Self, XmlReader<R>), Error>
    {
        let mut title = None;
        let mut description = None;
        let mut name = None;
        let mut link = None;

        while let Some(e) = reader.next() {
            match e {
                Ok(Event::Start(element)) => {
                    match element.name() {
                        b"title" => title = element_text!(reader),
                        b"description" => description = element_text!(reader),
                        b"name" => name = element_text!(reader),
                        b"link" => link = element_text!(reader),
                        _ => skip_element!(reader),
                    }
                },
                Ok(Event::End(_)) => {
                    let title = title.unwrap_or_default();
                    let description = description.unwrap_or_default();
                    let name = name.unwrap_or_default();
                    let link = link.unwrap_or_default();

                    return Ok((TextInput { title: title,
                                           description: description,
                                           name: name,
                                           link: link, },
                               reader));
                },
                Err(err) => return Err(err.into()),
                _ => {},
            }
        }

        Err(Error::EOF)
    }
}

impl ToXml for TextInput
{
    /// TODO
    fn to_xml<W: ::std::io::Write>(&self,
                                   writer: &mut XmlWriter<W>)
        -> Result<(), XmlError>
    {
        let element = Element::new("textInput");

        writer.write(Event::Start(element.clone()))?;

        writer.write_text_element(b"title",
                                  &self.title)?;
        writer.write_text_element(b"description",
                                  &self.description)?;
        writer.write_text_element(b"name",
                                  &self.name)?;
        writer.write_text_element(b"link",
                                  &self.link)?;

        writer.write(Event::End(element))
    }
}

/// This `TextInputBuilder` struct creates the `TextInput`.
#[derive(Debug, Clone, Default)]
pub struct TextInputBuilder
{
    title: String,
    description: String,
    name: String,
    link: String,
}

impl TextInputBuilder
{
    /// Construct a new `TextInputBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::TextInputBuilder;
    ///
    /// let text_input_builder = TextInputBuilder::new();
    /// ```
    pub fn new() -> TextInputBuilder
    {
        TextInputBuilder::default()
    }

    /// Set the title that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::TextInputBuilder;
    ///
    /// let mut text_input_builder = TextInputBuilder::new();
    /// text_input_builder.title("Title");
    /// ```
    pub fn title(&mut self,
                 title: &str)
        -> &mut TextInputBuilder
    {
        self.title = title.to_owned();
        self
    }

    /// Set the description that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::TextInputBuilder;
    ///
    /// let mut text_input_builder = TextInputBuilder::new();
    /// text_input_builder.description("This is a test description.");
    /// ```
    pub fn description(&mut self,
                       description: &str)
        -> &mut TextInputBuilder
    {
        self.description = description.to_owned();
        self
    }

    /// Set the name that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::TextInputBuilder;
    ///
    /// let mut text_input_builder = TextInputBuilder::new();
    /// text_input_builder.name("Comments");
    /// ```
    pub fn name(&mut self,
                name: &str)
        -> &mut TextInputBuilder
    {
        self.name = name.to_owned();
        self
    }

    /// Set the link that exists under `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::TextInputBuilder;
    ///
    /// let mut text_input_builder = TextInputBuilder::new();
    /// text_input_builder.link("http://www.example.com/feedback");
    /// ```
    pub fn link(&mut self,
                link: &str)
        -> &mut TextInputBuilder
    {
        self.link = link.to_owned();
        self
    }

    /// Validate the contents of `TextInput`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::TextInputBuilder;
    ///
    /// let text_input = TextInputBuilder::new()
    ///         .title("Title")
    ///         .description("This is a test description.")
    ///         .name("Comments")
    ///         .link("http://www.example.com/feedback")
    ///         .validate().unwrap()
    ///         .finalize().unwrap();
    /// ```
    pub fn validate(&mut self) -> Result<&mut TextInputBuilder, String>
    {
        string_utils::str_to_url(self.link.clone().as_str())?;

        Ok(self)
    }

    /// Construct the `TextInput` from the `TextInputBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::TextInputBuilder;
    ///
    /// let text_input = TextInputBuilder::new()
    ///         .title("Title")
    ///         .description("This is a test description.")
    ///         .name("Comments")
    ///         .link("http://www.example.com/feedback")
    ///         .finalize()
    ///         .unwrap();
    /// ```
    pub fn finalize(&self) -> Result<TextInput, String>
    {
        Ok(TextInput { title: self.title.clone(),
                       description: self.description.clone(),
                       name: self.name.clone(),
                       link: self.link.clone(), })
    }
}
