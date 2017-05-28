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

/// A representation of the `<image>` element.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Image {
    /// The URL of the channel image.
    url: String,
    /// A description of the image. This is used in the HTML `alt` attribute.
    title: String,
    /// The URL that the image links to.
    link: String,
    /// The width of the image.
    width: Option<String>,
    /// The height of the image.
    height: Option<String>,
    /// The text for the HTML `title` attribute.
    description: Option<String>,
}

impl Image {
    /// Get the url that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ImageBuilder, Image};
    ///
    /// let url = "http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg";
    ///
    /// let link = "http://www.jupiterbroadcasting.com";
    ///
    /// let image = ImageBuilder::new()
    ///     .url(url)
    ///     .link(link)
    ///     .finalize();
    ///
    /// assert_eq!(url, image.url());
    /// ```
    pub fn url(&self) -> &str() {
        self.url.as_str()
    }


    /// Get the title that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ImageBuilder, Image};
    ///
    /// let title = "LAS 300 Logo";
    ///
    /// let url = "http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg";
    ///
    /// let link = "http://www.jupiterbroadcasting.com";
    ///
    /// let image = ImageBuilder::new()
    ///     .title(title)
    ///     .url(url)
    ///     .link(link)
    ///     .finalize();
    ///
    /// assert_eq!(title, image.title());
    /// ```
    pub fn title(&self) -> &str {
        self.title.as_str()
    }


    /// Get the link that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ImageBuilder, Image};
    ///
    /// let url = "http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg";
    ///
    /// let link = "http://www.jupiterbroadcasting.com/";
    ///
    /// let image = ImageBuilder::new()
    ///     .url(url)
    ///     .link(link)
    ///     .finalize();
    ///
    /// assert_eq!(link, image.link());
    /// ```
    pub fn link(&self) -> &str {
        self.link.as_str()
    }


    /// Get the width that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ImageBuilder, Image};
    ///
    /// let default: i64 = 88;
    ///
    /// let url = "http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg";
    ///
    /// let link = "http://www.jupiterbroadcasting.com";
    ///
    /// let image = ImageBuilder::new()
    ///     .width(None)
    ///     .url(url)
    ///     .link(link)
    ///     .finalize();
    ///
    /// assert_eq!(default.to_string(), image.width().unwrap());
    /// ```
    ///
    /// ```
    /// use rss::{ImageBuilder, Image};
    ///
    /// let width: i64 = 60;
    ///
    /// let url = "http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg";
    ///
    /// let link = "http://www.jupiterbroadcasting.com";
    ///
    /// let image = ImageBuilder::new()
    ///     .width(Some(width))
    ///     .url(url)
    ///     .link(link)
    ///     .finalize();
    ///
    /// assert_eq!(Some(width.to_string().as_str()), image.width());
    /// ```
    ///
    /// ```
    pub fn width(&self) -> Option<&str> {
        self.width.as_ref().map(|s| s.as_str())
    }


    /// Get the height that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ImageBuilder, Image};
    ///
    /// let default: i64 = 31;
    ///
    /// let url = "http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg";
    ///
    /// let link = "http://www.jupiterbroadcasting.com";
    ///
    /// let image = ImageBuilder::new()
    ///     .height(None)
    ///     .url(url)
    ///     .link(link)
    ///     .finalize();
    ///
    /// assert_eq!(default.to_string(), image.height().unwrap());
    /// ```
    ///
    /// ```
    /// use rss::{ImageBuilder, Image};
    ///
    /// let height: i64 = 60;
    ///
    /// let url = "http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg";
    ///
    /// let link = "http://www.jupiterbroadcasting.com";
    ///
    /// let image = ImageBuilder::new()
    ///     .height(Some(height))
    ///     .url(url)
    ///     .link(link)
    ///     .finalize();
    ///
    /// assert_eq!(Some(height.to_string().as_str()), image.height());
    /// ```
    pub fn height(&self) -> Option<&str> {
        self.height.as_ref().map(|s| s.as_str())
    }


    /// Get the description that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{ImageBuilder, Image};
    ///
    /// let url = "http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg";
    ///
    /// let link = "http://www.jupiterbroadcasting.com";
    ///
    /// let image = ImageBuilder::new()
    ///     .description(None)
    ///     .url(url)
    ///     .link(link)
    ///     .finalize();
    ///
    /// assert!(image.description().is_none());
    /// ```
    ///
    /// ```
    /// use rss::{ImageBuilder, Image};
    ///
    /// let description_string = "This is a test";
    ///
    /// let url = "http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg";
    ///
    /// let link = "http://www.jupiterbroadcasting.com";
    ///
    /// let image = ImageBuilder::new()
    ///     .description(Some(description_string.to_string()))
    ///     .url(url)
    ///     .link(link)
    ///     .finalize();
    ///
    /// let description_option = image.description();
    /// assert!(description_option.is_some());
    ///
    /// assert_eq!(Some(description_string), image.description());
    /// ```
    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|s| s.as_str())
    }
}

impl FromXml for Image {
    fn from_xml<R: ::std::io::BufRead>(reader: &mut Reader<R>,
                                       _: Attributes)
                                       -> Result<Self, Error> {
        let mut url = None;
        let mut title = None;
        let mut link = None;
        let mut width = None;
        let mut height = None;
        let mut description = None;
        let mut buf = Vec::new();
        let mut skip_buf = Vec::new();

        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(element) => {
                    match element.name() {
                        b"url" => url = element_text(reader)?,
                        b"title" => title = element_text(reader)?,
                        b"link" => link = element_text(reader)?,
                        b"width" => width = element_text(reader)?,
                        b"height" => height = element_text(reader)?,
                        b"description" => description = element_text(reader)?,
                        n => reader.read_to_end(n, &mut skip_buf)?,
                    }
                }
                Event::End(_) => {
                    return Ok(Image {
                                  url: url.unwrap_or_default(),
                                  title: title.unwrap_or_default(),
                                  link: link.unwrap_or_default(),
                                  width: width,
                                  height: height,
                                  description: description,
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

impl ToXml for Image {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = b"image";

        writer
            .write_event(Event::Start(BytesStart::borrowed(name, name.len())))?;

        writer.write_text_element(b"url", &self.url)?;
        writer.write_text_element(b"title", &self.title)?;
        writer.write_text_element(b"link", &self.link)?;

        if let Some(width) = self.width.as_ref() {
            writer.write_text_element(b"width", width)?;
        }

        if let Some(height) = self.height.as_ref() {
            writer.write_text_element(b"height", height)?;
        }

        if let Some(description) = self.description.as_ref() {
            writer.write_text_element(b"description", description)?;
        }

        writer.write_event(Event::End(BytesEnd::borrowed(name)))?;
        Ok(())
    }
}

/// This `ImageBuilder` struct creates the `Image`.
#[derive(Debug, Clone, Default)]
pub struct ImageBuilder {
    url: String,
    title: String,
    link: String,
    width: Option<i64>,
    height: Option<i64>,
    description: Option<String>,
}

impl ImageBuilder {
    /// Construct a new `ImageBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ImageBuilder;
    ///
    /// let image_builder = ImageBuilder::new();
    /// ```
    pub fn new() -> ImageBuilder {
        ImageBuilder::default()
    }


    /// Set the url that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ImageBuilder;
    ///
    /// let image_builder = ImageBuilder::new()
    ///     .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg");
    /// ```
    pub fn url<S: Into<String>>(mut self, url: S) -> ImageBuilder {
        self.url = url.into();
        self
    }


    /// Set the title that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ImageBuilder;
    ///
    /// let image_builder = ImageBuilder::new()
    ///     .title("LAS 300 Logo");
    /// ```
    pub fn title<S: Into<String>>(mut self, title: S) -> ImageBuilder {
        self.title = title.into();
        self
    }


    /// Set the link that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ImageBuilder;
    ///
    /// let image_builder = ImageBuilder::new()
    ///     .link("http://www.jupiterbroadcasting.com/");
    /// ```
    pub fn link<S: Into<String>>(mut self, link: S) -> ImageBuilder {
        self.link = link.into();
        self
    }


    /// Set the width that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ImageBuilder;
    ///
    /// let image_builder = ImageBuilder::new()
    ///     .width(Some(88));
    /// ```
    pub fn width(mut self, width: Option<i64>) -> ImageBuilder {
        self.width = width;
        self
    }


    /// Set the height that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ImageBuilder;
    ///
    /// let image_builder = ImageBuilder::new()
    ///     .height(Some(88));
    /// ```
    pub fn height(mut self, height: Option<i64>) -> ImageBuilder {
        self.height = height;
        self
    }


    /// Set the description that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ImageBuilder;
    ///
    /// let image_builder = ImageBuilder::new()
    ///     .description(Some("This is a test".to_string()));
    /// ```
    pub fn description(mut self, description: Option<String>) -> ImageBuilder {
        self.description = description;
        self
    }


    /// Validate the contents of `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ImageBuilder;
    ///
    /// let image = ImageBuilder::new()
    ///         .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg")
    ///         .title("LAS 300 Logo")
    ///         .link("http://www.jupiterbroadcasting.com")
    ///         .width(Some(88))
    ///         .height(Some(88))
    ///         .description(Some("This is a test".to_string()))
    ///         .validate()
    ///         .unwrap()
    ///         .finalize();
    /// ```
    pub fn validate(self) -> Result<ImageBuilder, Error> {
        if !self.url.ends_with(".jpeg") && !self.url.ends_with(".jpg") &&
           !self.url.ends_with(".png") && !self.url.ends_with(".gif") {
            return Err(Error::Validation("Image Url must end with .jpeg, .png, or .gif"
                                             .to_string()));
        }

        Url::parse(self.url.as_str())?;
        Url::parse(self.link.as_str())?;

        if let Some(width) = self.width {
            if width > 144 {
                return Err(Error::Validation("Image Width cannot be greater than 144."
                                                 .to_string()));
            } else if width < 0 {
                return Err(Error::Validation("Image Width cannot be a negative value."
                                                 .to_string()));
            }
        }

        if let Some(height) = self.height {
            if height > 144 {
                return Err(Error::Validation("Image Height cannot be greater than 400."
                                                 .to_string()));
            } else if height < 0 {
                return Err(Error::Validation("Image Height cannot be a negative value."
                                                 .to_string()));
            }
        }

        Ok(self)
    }


    /// Construct the `Image` from the `ImageBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ImageBuilder;
    ///
    /// let image = ImageBuilder::new()
    ///         .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg")
    ///         .title("LAS 300 Logo")
    ///         .link("http://www.jupiterbroadcasting.com")
    ///         .width(Some(88))
    ///         .height(Some(88))
    ///         .description(Some("This is a test".to_string()))
    ///         .finalize();
    /// ```
    pub fn finalize(self) -> Image {
        Image {
            url: self.url,
            title: self.title,
            link: self.link,
            width: Some(self.width.unwrap_or(88).to_string()),
            height: Some(self.height.unwrap_or(31).to_string()),
            description: self.description,
        }
    }
}
