// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

#![warn(missing_docs)]
#![allow(unknown_lints, while_let_on_iterator)]
#![doc(html_root_url = "https://docs.rs/rss/")]

//! Library for serializing the RSS web content syndication format.
//!
//! # Building
//!
//! A channel can be built using the Builder functions.
//!
//! ## Example
//!
//! ```
//! extern crate rss;
//!
//! use rss::{ChannelBuilder, ImageBuilder};
//!
//! fn main() {
//!     let image = ImageBuilder::default()
//!         .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg")
//!         .title("LAS 300 Logo")
//!         .link("http://www.jupiterbroadcasting.com")
//!         .finalize();
//!
//!     let description = "Ogg Vorbis audio versions of The Linux Action Show! A show that \
//!         covers everything geeks care about in the computer industry. Get a solid dose of \
//!         Linux, gadgets, news events and much more!";
//!
//!     let channel = ChannelBuilder::default()
//!         .title("The Linux Action Show! OGG")
//!         .link("http://www.jupiterbroadcasting.com")
//!         .description(description.as_ref())
//!         .image(image)
//!         .finalize();
//! }
//! ```
//!
//! # Reading
//!
//! ## From a `BufRead`
//!
//! A channel can be read from any object that implements the `BufRead` trait.
//!
//! ```rust,no_run
//! use std::fs::File;
//! use std::io::BufReader;
//! use rss::Channel;
//!
//! let file = File::open("example.xml").unwrap();
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
//! extern crate rss;
//!
//! use rss::Channel;
//!
//! fn main() {
//!     let url = "https://feedpress.me/usererror.xml";
//!     let channel = Channel::from_url(url).unwrap();
//! }
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
//! ```rust,no_run
//! use std::fs::File;
//! use std::io::{BufReader, sink};
//! use rss::Channel;
//!
//! let file = File::open("example.xml").unwrap();
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
//! # Validation
//!
//! A channel can be validated by means of the channel or by the builders.
//!
//! ## Example
//!
//! ```
//! extern crate rss;
//!
//! use rss::Channel;
//!
//! fn main() {
//!     let input = include_str!("tests/data/rss2sample.xml");
//!
//!     let channel = input.parse::<Channel>().unwrap();
//!     channel.validate().unwrap();
//! }
//! ```
//!
//! ## Example
//!
//! ```
//! extern crate rss;
//!
//! use rss::ImageBuilder;
//!
//! fn main() {
//!     let builder = ImageBuilder::default()
//!             .url("http://jupiterbroadcasting.com/images/LAS-300-Badge.jpg")
//!             .title("LAS 300 Logo")
//!             .link("http://www.jupiterbroadcasting.com")
//!             .validate()
//!             .unwrap();
//! }
//! ```
extern crate quick_xml;
extern crate chrono;
extern crate url;
extern crate mime;

#[cfg(feature = "from_url")]
extern crate reqwest;

#[macro_use]
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
