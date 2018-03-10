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
//! ## From a Reader
//!
//! A channel can be read from any object that implements the `BufRead` trait.
//!
//! ```rust,no_run
//! use std::fs::File;
//! use std::io::BufReader;
//! use rss::Channel;
//!
//! let file = File::open("example.xml").unwrap();
//! let channel = Channel::read_from(BufReader::new(file)).unwrap();
//! ```
//!
//! ## From a URL
//!
//! A channel can also be read from a URL.
//!
//! **Note**: This requires enabling the `from_url` feature.
//!
//! ```ignore
//! use rss::Channel;
//!
//! let channel = Channel::from_url("http://example.com/feed.xml").unwrap();
//! ```
//!
//! # Writing
//!
//! A channel can be written to any object that implements the `Write` trait or converted to an
//! XML string using the `ToString` trait.
//!
//! **Note**: Writing a channel does not perform any escaping of XML entities.
//!
//! ```rust
//! use rss::Channel;
//!
//! let channel = Channel::default();
//! channel.write_to(::std::io::sink()).unwrap(); // write to the channel to a writer
//! let string = channel.to_string(); // convert the channel to a string
//! ```
//!
//! # Creation
//!
//! Builder methods are provided to assist in the creation of channels.
//!
//! ```
//! use rss::ChannelBuilder;
//!
//! let channel = ChannelBuilder::default()
//!     .title("Channel Title")
//!     .link("http://example.com")
//!     .description("An RSS feed.")
//!     .build()
//!     .unwrap();
//! ```
//!
//! ## Validation
//!
//! Validation methods are provided to validate the contents of a channel against the
//! RSS specification.
//!
//! **Note**: This requires enabling the `validation` feature.
//!
//! ```rust,ignore
//! use rss::Channel;
//! use rss::validation::Validate;
//!
//! let channel = Channel::default();
//! channel.validate().unwrap();
//! ```

#[macro_use]
extern crate derive_builder;

extern crate failure;
extern crate quick_xml;

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;
#[cfg(feature = "serde")]
#[cfg(feature = "validation")]
extern crate chrono;
#[cfg(feature = "validation")]
extern crate url;
#[cfg(feature = "validation")]
extern crate mime;

#[cfg(feature = "from_url")]
extern crate reqwest;

mod channel;
mod category;
mod cloud;
mod enclosure;
mod guid;
mod image;
mod item;
mod source;
mod textinput;

mod error;
mod fromxml;
mod toxml;
mod util;

/// Types and methods for namespaced extensions.
pub mod extension;

/// Methods for validating RSS feeds.
#[cfg(feature = "validation")]
pub mod validation;

pub use channel::{Channel, ChannelBuilder};
pub use category::{Category, CategoryBuilder};
pub use cloud::{Cloud, CloudBuilder};
pub use enclosure::{Enclosure, EnclosureBuilder};
pub use guid::{Guid, GuidBuilder};
pub use image::{Image, ImageBuilder};
pub use item::{Item, ItemBuilder};
pub use source::{Source, SourceBuilder};
pub use textinput::{TextInput, TextInputBuilder};

pub use error::Error;
