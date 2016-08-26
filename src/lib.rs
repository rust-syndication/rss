#![warn(missing_docs)]
#![allow(unknown_lints, while_let_on_iterator)]

//! A fast RSS feed parser.
//!
//! # Reading
//!
//! Reading can be done using any object that implements the `BufRead` trait.
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

extern crate quick_xml;

#[macro_use]
mod fromxml;

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

/// Types and functions for namespace extensions.
pub mod extension;
pub use extension::Extension;

mod error;
pub use error::Error;

mod parser;
