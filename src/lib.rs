#![warn(missing_docs)]

//! ## Reading
//!
//! See [`Channel::read_from`](struct.Channel.html#method.read_from) and [`parse`](fn.parse.html).

#[cfg(feature = "quick-xml")]
extern crate quick_xml;
#[cfg(feature = "xml-rs")]
extern crate xml;

mod channel;
pub use channel::Channel;

mod item;
pub use item::Item;

mod guid;
pub use guid::Guid;

mod enclosure;
pub use enclosure::Enclosure;

mod source;
pub use source::Source;

#[cfg(feature = "quick-xml")]
mod parser_quickxml;
#[cfg(feature = "quick-xml")]
pub use parser_quickxml::parse;

#[cfg(feature = "xml-rs")]
mod parser_xmlrs;
#[cfg(feature = "xml-rs")]
pub use parser_xmlrs::parse;

mod error;
pub use error::Error;

mod fromxml;
