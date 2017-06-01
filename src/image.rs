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
    /// Return the URL for this `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ImageBuilder;
    ///
    /// let url = "http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg";
    ///
    /// let image = ImageBuilder::default()
    ///     .url(url)
    ///     .finalize();
    ///
    /// assert_eq!(url, image.url());
    /// ```
    pub fn url(&self) -> &str() {
        self.url.as_str()
    }

    /// Return the title for this `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ImageBuilder;
    ///
    /// let title = "LAS 300 Logo";
    ///
    /// let image = ImageBuilder::default()
    ///     .title(title)
    ///     .finalize();
    ///
    /// assert_eq!(title, image.title());
    /// ```
    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    /// Return the link that this `Image` directs to.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ImageBuilder;
    ///
    /// let link = "http://www.jupiterbroadcasting.com/";
    ///
    /// let image = ImageBuilder::default()
    ///     .link(link)
    ///     .finalize();
    ///
    /// assert_eq!(link, image.link());
    /// ```
    pub fn link(&self) -> &str {
        self.link.as_str()
    }

    /// Return the width of this `Image`.
    ///
    /// If this is `None` the default value should be considered to be `80`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ImageBuilder;
    ///
    /// let width = 60;
    ///
    /// let image = ImageBuilder::default()
    ///     .width(width)
    ///     .finalize();
    ///
    /// assert_eq!(Some(width.to_string().as_str()), image.width());
    /// ```
    pub fn width(&self) -> Option<&str> {
        self.width.as_ref().map(|s| s.as_str())
    }

    /// Return the height of this `Image`.
    ///
    /// If this is `None` the default value should be considered to be `31`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ImageBuilder;
    ///
    /// let height = 60;
    ///
    /// let image = ImageBuilder::default()
    ///     .height(height)
    ///     .finalize();
    ///
    /// assert_eq!(Some(height.to_string().as_str()), image.height());
    /// ```
    pub fn height(&self) -> Option<&str> {
        self.height.as_ref().map(|s| s.as_str())
    }

    /// Return the description of this `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ImageBuilder;
    ///
    /// let image = ImageBuilder::default()
    ///     .description(None)
    ///     .finalize();
    ///
    /// assert!(image.description().is_none());
    /// ```
    ///
    /// ```
    /// use rss::ImageBuilder;
    ///
    /// let description = "This is a test";
    ///
    /// let image = ImageBuilder::default()
    ///     .description(description.to_string())
    ///     .finalize();
    ///
    /// assert_eq!(Some(description), image.description());
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

/// A builder used to create an `Image`.
#[derive(Debug, Default, Clone)]
pub struct ImageBuilder {
    url: String,
    title: String,
    link: String,
    width: Option<i64>,
    height: Option<i64>,
    description: Option<String>,
}

impl ImageBuilder {
    /// Construct a new `ImageBuilder` using the values from an existing `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::{Channel, ImageBuilder};
    ///
    /// let input = include_str!("tests/data/image.xml");
    /// let channel = input.parse::<Channel>().unwrap();
    /// let image = channel.image().unwrap().clone();
    /// let builder = ImageBuilder::from_image(image).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// If this function encounters an error while parsing `width` or `height` from a `String` to
    /// an `i64` it will return an
    /// [`IntParsing`](/rss/enum.Error.html#variant.IntParsing) error.
    pub fn from_image(image: Image) -> Result<Self, Error> {
        let width = match image.width {
            Some(width) => Some(width.parse::<i64>()?),
            None => None,
        };

        let height = match image.height {
            Some(height) => Some(height.parse::<i64>()?),
            None => None,
        };

        Ok(ImageBuilder {
               url: image.url,
               title: image.title,
               link: image.link,
               width: width,
               height: height,
               description: image.description,
           })
    }

    /// Set URL for the `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ImageBuilder;
    ///
    /// let builder = ImageBuilder::default()
    ///     .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg");
    /// ```
    pub fn url<S>(mut self, url: S) -> ImageBuilder
        where S: Into<String>
    {
        self.url = url.into();
        self
    }

    /// Set the title for the `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ImageBuilder;
    ///
    /// let builder = ImageBuilder::default()
    ///     .title("LAS 300 Logo");
    /// ```
    pub fn title<S>(mut self, title: S) -> ImageBuilder
        where S: Into<String>
    {
        self.title = title.into();
        self
    }

    /// Set the link that the `Image` directs to.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ImageBuilder;
    ///
    /// let builder = ImageBuilder::default()
    ///     .link("http://www.jupiterbroadcasting.com/");
    /// ```
    pub fn link<S>(mut self, link: S) -> ImageBuilder
        where S: Into<String>
    {
        self.link = link.into();
        self
    }

    /// Set width of the `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ImageBuilder;
    ///
    /// let builder = ImageBuilder::default()
    ///     .width(88);
    /// ```
    pub fn width<V>(mut self, width: V) -> ImageBuilder
        where V: Into<Option<i64>>
    {
        self.width = width.into();
        self
    }

    /// Set the height of the `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ImageBuilder;
    ///
    /// let builder = ImageBuilder::default()
    ///     .height(88);
    /// ```
    pub fn height<V>(mut self, height: V) -> ImageBuilder
        where V: Into<Option<i64>>
    {
        self.height = height.into();
        self
    }

    /// Set the description of the `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ImageBuilder;
    ///
    /// let builder = ImageBuilder::default()
    ///     .description("This is a test".to_string());
    /// ```
    pub fn description<V>(mut self, description: V) -> ImageBuilder
        where V: Into<Option<String>>
    {
        self.description = description.into();
        self
    }


    /// Validate the contents of this `ImageBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ImageBuilder;
    ///
    /// let image = ImageBuilder::default()
    ///         .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg")
    ///         .title("LAS 300 Logo")
    ///         .link("http://www.jupiterbroadcasting.com")
    ///         .validate()
    ///         .unwrap();
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


    /// Construct the `Image` from this `ImageBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ImageBuilder;
    ///
    /// let image = ImageBuilder::default()
    ///         .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg")
    ///         .title("LAS 300 Logo")
    ///         .link("http://www.jupiterbroadcasting.com")
    ///         .finalize();
    /// ```
    pub fn finalize(self) -> Image {
        Image {
            url: self.url,
            title: self.title,
            link: self.link,
            width: self.width.map(|n| n.to_string()),
            height: self.height.map(|n| n.to_string()),
            description: self.description,
        }
    }
}
