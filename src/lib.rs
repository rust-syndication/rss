#![warn(missing_docs)]
#![allow(unknown_lints, while_let_on_iterator)]

//! ## Reading
//!
//! See [`Channel::read_from`](struct.Channel.html#method.read_from).

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

mod error;
pub use error::Error;

mod parser;
