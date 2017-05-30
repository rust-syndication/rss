// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/rss/")]

//! Library for serializing the RSS web content syndication format.
//!
//! # Reading
//!
//! ## From a `BufRead`
//!
//! A channel can be read from any object that implements the `BufRead` trait.
//!
//! ```rust
//! use std::fs::File;
//! use std::io::BufReader;
//! use rss::Channel;
//!
//! let file = File::open("tests/data/rss2sample.xml").unwrap();
//! let reader = BufReader::new(file);
//! let channel = Channel::read_from(reader).unwrap();
//! ```
//! ## From a URL
//!
//! A channel can also be read from a URL.
//!
//! To enable this functionality you must enable the `from_url` feature in your Cargo.toml.
//!
//! ```toml
//! [dependencies]
//! rss = { version = "*", features = ["from_url"] }
//! ```
//!
//! ```ignore
//! use rss::Channel;
//!
//! let channel = Channel::from_url("https://feedpress.me/usererror.xml").unwrap();
//! ```
//!
//! # Writing
//!
//! A channel can be written to any object that implements the `Write` trait or converted to an
//! XML string using the `ToString` trait.
//!
//! **Note**: Writing a channel does not perform any escaping of XML entities.
//!
//! ## Example
//!
//! ```rust
//! use std::fs::File;
//! use std::io::{BufReader, sink};
//! use rss::Channel;
//!
//! let file = File::open("tests/data/rss2sample.xml").unwrap();
//! let reader = BufReader::new(file);
//! let channel = Channel::read_from(reader).unwrap();
//!
//! // write to the channel to a writer
//! channel.write_to(sink()).unwrap();
//!
//! // convert the channel to a string
//! let string = channel.to_string();
//! ```
//!
//! # Creation
//!
//! A channel can be created using the Builder functions.
//!
//! ## Example
//!
//! ```
//! use rss::{ChannelBuilder, ImageBuilder};
//!
//! let image = ImageBuilder::default()
//!     .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg")
//!     .title("LAS 300 Logo")
//!     .link("http://www.jupiterbroadcasting.com")
//!     .finalize();
//!
//! let channel = ChannelBuilder::default()
//!     .title("The Linux Action Show! OGG")
//!     .link("http://www.jupiterbroadcasting.com")
//!     .description("Ogg Vorbis audio versions of The Linux Action Show!")
//!     .image(image)
//!     .finalize();
//! ```
//!
//! # Validation
//!
//! Validation can be performed using either a `Channel` or a builder.
//!
//! The the following checks are performed during validation:
//!
//! * Ensures that integer properties can be parsed from their string representaiton in to
//! integers
//! * Ensures that the integer properties are within their valid range according to the RSS 2.0
//! specification
//! * Ensures that URL properties can be parsed
//! * Ensures that string properties where only certain values are allowed fall within those
//! valid values
//!
//! ## Example
//!
//! ```
//! use rss::Channel;
//!
//! let input = include_str!("tests/data/rss2sample.xml");
//! let channel = input.parse::<Channel>().unwrap();
//! channel.validate().unwrap();
//! ```
//!
//! ## Example
//!
//! ```
//! use rss::ImageBuilder;
//!
//! let builder = ImageBuilder::default()
//!     .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg")
//!     .title("LAS 300 Logo")
//!     .link("http://www.jupiterbroadcasting.com")
//!     .validate()
//!     .unwrap();
//! ```
extern crate quick_xml;
extern crate chrono;
extern crate url;
extern crate mime;

#[cfg(feature = "from_url")]
extern crate reqwest;

mod fromxml;
mod toxml;

mod channel;
pub use channel::Channel;
pub use channel::ChannelBuilder;

mod item;
pub use item::Item;
pub use item::ItemBuilder;

mod category;
pub use category::Category;
pub use category::CategoryBuilder;

mod guid;
pub use guid::Guid;
pub use guid::GuidBuilder;

mod enclosure;
pub use enclosure::Enclosure;
pub use enclosure::EnclosureBuilder;

mod source;
pub use source::Source;
pub use source::SourceBuilder;

mod cloud;
pub use cloud::Cloud;
pub use cloud::CloudBuilder;

mod image;
pub use image::Image;
pub use image::ImageBuilder;

mod textinput;
pub use textinput::TextInput;
pub use textinput::TextInputBuilder;

/// Types and functions for namespaced extensions.
pub mod extension;
pub use extension::Extension;

mod error;
pub use error::Error;
