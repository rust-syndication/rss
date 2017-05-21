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
use url::Url;
use toxml::{ToXml, XmlWriterExt};

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
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(url, image.url());
    /// ```
    pub fn url(&self) -> &str() {
        self.url
            .as_str()
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
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(title, image.title());
    /// ```
    pub fn title(&self) -> &str {
        self.title
            .as_str()
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
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(link, image.link());
    /// ```
    pub fn link(&self) -> &str {
        self.link
            .as_str()
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
    ///     .finalize()
    ///     .unwrap();
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
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(Some(width.to_string().as_str()), image.width());
    /// ```
    ///
    /// ```
    pub fn width(&self) -> Option<&str> {
        self.width
            .as_ref()
            .map(|s| s.as_str())
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
    ///     .finalize()
    ///     .unwrap();
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
    ///     .finalize()
    ///     .unwrap();
    ///
    /// assert_eq!(Some(height.to_string().as_str()), image.height());
    /// ```
    pub fn height(&self) -> Option<&str> {
        self.height
            .as_ref()
            .map(|s| s.as_str())
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
    ///     .finalize()
    ///     .unwrap();
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
    ///     .finalize()
    ///     .unwrap();
    ///
    /// let description_option = image.description();
    /// assert!(description_option.is_some());
    ///
    /// assert_eq!(Some(description_string), image.description());
    /// ```
    pub fn description(&self) -> Option<&str> {
        self.description
            .as_ref()
            .map(|s| s.as_str())
    }
}

impl FromXml for Image {
    fn from_xml<R: ::std::io::BufRead>(mut reader: XmlReader<R>,
                                       _: Element)
        -> Result<(Self, XmlReader<R>), Error> {
        let mut url = None;
        let mut title = None;
        let mut link = None;
        let mut width = None;
        let mut height = None;
        let mut description = None;

        while let Some(e) = reader.next() {
            match e {
                Ok(Event::Start(element)) => {
                    match element.name() {
                        b"url" => url = element_text!(reader),
                        b"title" => title = element_text!(reader),
                        b"link" => link = element_text!(reader),
                        b"width" => width = element_text!(reader),
                        b"height" => height = element_text!(reader),
                        b"description" => description = element_text!(reader),
                        _ => skip_element!(reader),
                    }
                },
                Ok(Event::End(_)) => {
                    let url = url.unwrap_or_default();
                    let title = title.unwrap_or_default();
                    let link = link.unwrap_or_default();

                    return Ok((Image { url: url,
                                       title: title,
                                       link: link,
                                       width: width,
                                       height: height,
                                       description: description, },
                               reader));
                },
                Err(err) => return Err(err.into()),
                _ => {},
            }
        }

        Err(Error::EOF)
    }
}

impl ToXml for Image {
    fn to_xml<W: ::std::io::Write>(&self,
                                   writer: &mut XmlWriter<W>)
        -> Result<(), XmlError> {
        let element = Element::new(b"image");

        writer.write(Event::Start(element.clone()))?;

        writer.write_text_element(b"url",
                                  &self.url)?;
        writer.write_text_element(b"title",
                                  &self.title)?;
        writer.write_text_element(b"link",
                                  &self.link)?;

        if let Some(width) = self.width
                                 .as_ref() {
            writer.write_text_element(b"width",
                                      width)?;
        }

        if let Some(height) = self.height
                                  .as_ref() {
            writer.write_text_element(b"height",
                                      height)?;
        }

        if let Some(description) =
            self.description
                .as_ref() {
            writer.write_text_element(b"description",
                                      description)?;
        }

        writer.write(Event::End(element))
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
    /// let mut image_builder = ImageBuilder::new();
    /// image_builder.url("http://jupiterbroadcasting.com/images/LAS-300-Badge.
    /// jpg");
    /// ```
    pub fn url(mut self,
               url: &str)
        -> ImageBuilder {
        self.url = url.to_string();
        self
    }


    /// Set the title that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ImageBuilder;
    ///
    /// let mut image_builder = ImageBuilder::new();
    /// image_builder.title("LAS 300 Logo");
    /// ```
    pub fn title(mut self,
                 title: &str)
        -> ImageBuilder {
        self.title = title.to_string();
        self
    }


    /// Set the link that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ImageBuilder;
    ///
    /// let mut image_builder = ImageBuilder::new();
    /// image_builder.link("http://www.jupiterbroadcasting.com/");
    /// ```
    pub fn link(mut self,
                link: &str)
        -> ImageBuilder {
        self.link = link.to_string();
        self
    }


    /// Set the width that exists under `Image`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::ImageBuilder;
    ///
    /// let mut image_builder = ImageBuilder::new();
    /// image_builder.width(Some(88));
    /// ```
    pub fn width(mut self,
                 width: Option<i64>)
        -> ImageBuilder {
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
    /// let mut image_builder = ImageBuilder::new();
    /// image_builder.height(Some(88));
    /// ```
    pub fn height(mut self,
                  height: Option<i64>)
        -> ImageBuilder {
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
    /// let mut image_builder = ImageBuilder::new();
    /// image_builder.description(Some("This is a test".to_string()));
    /// ```
    pub fn description(mut self,
                       description: Option<String>)
        -> ImageBuilder {
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
    ///         .validate().unwrap()
    ///         .finalize().unwrap();
    /// ```
    pub fn validate(self) -> Result<ImageBuilder, Error> {
        if !self.url
                .ends_with(".jpeg") &&
           !self.url
                .ends_with(".jpg") &&
           !self.url
                .ends_with(".png") &&
           !self.url
                .ends_with(".gif") {
            return Err(Error::Validation("Image Url must end with .jpeg, .png, or .gif".to_string()));
        }

        Url::parse(self.url
                       .as_str())?;
        Url::parse(self.link
                       .as_str())?;

        let width_opt = self.width;
        if width_opt.is_some() {
            let width = width_opt.unwrap();
            if width > 144 {
                return Err(Error::Validation("Image width cannot be greater than 144.".to_string()));
            } else if width < 0 {
                return Err(Error::Validation("Image width cannot be a negative value.".to_string()));
            }
        }

        let height_opt = self.height;
        if height_opt.is_some() {
            let height = height_opt.unwrap();
            if height > 144 {
                return Err(Error::Validation("Image height cannot be greater than 400.".to_string()));
            } else if height < 0 {
                return Err(Error::Validation("Image height cannot be a negative value.".to_string()));
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
    pub fn finalize(self) -> Result<Image, Error> {

        let width = match self.width {
            Some(val) => Some(val.to_string()),
            None => Some(88.to_string()),
        };

        let height = match self.height {
            Some(val) => Some(val.to_string()),
            None => Some(31.to_string()),
        };

        Ok(Image { url: self.url,
                   title: self.title,
                   link: self.link,
                   width: width,
                   height: height,
                   description: self.description, })
    }
}
