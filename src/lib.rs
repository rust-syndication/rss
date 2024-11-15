// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/rss/")]

//! Library for serializing the RSS web content syndication format.
//!
//! # Reading
//!
//! A channel can be read from any object that implements the `BufRead` trait.
//!
//! ## From a file
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
//! ### From a buffer
//!
//! **Note**: This example requires [reqwest](https://crates.io/crates/reqwest) crate.
//!
//! ```rust,ignore
//! use std::error::Error;
//! use rss::Channel;
//!
//! async fn example_feed() -> Result<Channel, Box<dyn Error>> {
//!     let content = reqwest::get("http://example.com/feed.xml")
//!         .await?
//!         .bytes()
//!         .await?;
//!     let channel = Channel::read_from(&content[..])?;
//!     Ok(channel)
//! }
//! ```
//!
//! # Writing
//!
//! A channel can be written to any object that implements the `Write` trait or converted to an
//! XML string using the `ToString` trait.
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
//! **Note**: This requires the `builders` feature, which is enabled by default.
//!
//! ```
//! use rss::ChannelBuilder;
//!
//! let channel = ChannelBuilder::default()
//!     .title("Channel Title")
//!     .link("http://example.com")
//!     .description("An RSS feed.")
//!     .build();
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

#[cfg(feature = "builders")]
#[macro_use]
extern crate derive_builder;

extern crate quick_xml;

#[cfg(feature = "serde")]
#[cfg(feature = "validation")]
extern crate chrono;
#[cfg(feature = "validation")]
extern crate mime;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;
#[cfg(feature = "validation")]
extern crate url;

mod category;
mod channel;
mod cloud;
mod enclosure;
mod guid;
mod image;
mod item;
mod source;
mod textinput;

mod error;
mod toxml;
mod util;

/// Types and methods for namespaced extensions.
pub mod extension;

/// Methods for validating RSS feeds.
#[cfg(feature = "validation")]
pub mod validation;

pub use crate::category::Category;
#[cfg(feature = "builders")]
pub use crate::category::CategoryBuilder;
pub use crate::channel::Channel;
#[cfg(feature = "builders")]
pub use crate::channel::ChannelBuilder;
pub use crate::cloud::Cloud;
#[cfg(feature = "builders")]
pub use crate::cloud::CloudBuilder;
pub use crate::enclosure::Enclosure;
#[cfg(feature = "builders")]
pub use crate::enclosure::EnclosureBuilder;
pub use crate::guid::Guid;
#[cfg(feature = "builders")]
pub use crate::guid::GuidBuilder;
pub use crate::image::Image;
#[cfg(feature = "builders")]
pub use crate::image::ImageBuilder;
pub use crate::item::Item;
#[cfg(feature = "builders")]
pub use crate::item::ItemBuilder;
pub use crate::source::Source;
#[cfg(feature = "builders")]
pub use crate::source::SourceBuilder;
pub use crate::textinput::TextInput;
#[cfg(feature = "builders")]
pub use crate::textinput::TextInputBuilder;

pub use crate::error::Error;
