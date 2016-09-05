#![warn(missing_docs)]
#![allow(unknown_lints, while_let_on_iterator)]

//! Library for serializing the RSS web content syndication format.
//!
//! # Reading
//!
//! A channel can be read from any object that implements the `BufRead` trait.
//!
//! ## Example
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

extern crate quick_xml;

#[macro_use]
mod fromxml;
mod toxml;

mod channel;
pub use channel::Channel;

mod item;
pub use item::Item;

mod category;
pub use category::Category;

mod guid;
pub use guid::Guid;

mod enclosure;
pub use enclosure::Enclosure;

mod source;
pub use source::Source;

mod cloud;
pub use cloud::Cloud;

mod image;
pub use image::Image;

mod textinput;
pub use textinput::TextInput;

/// Types and functions for namespaced extensions.
pub mod extension;
pub use extension::Extension;

mod error;
pub use error::Error;
