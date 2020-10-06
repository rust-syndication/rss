// This file is part of rss.
//
// Copyright © 2015-2020 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::collections::HashMap;
use std::io::Write;

pub use atom_syndication::Link;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Error as XmlError;
use quick_xml::Writer;

use crate::extension::Extension;
use crate::toxml::ToXml;

/// The Atom XML namespace.
pub const NAMESPACE: &str = "http://www.w3.org/2005/Atom";

/// An Atom element extension.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[cfg_attr(feature = "builders", builder(setter(into), default))]
pub struct AtomExtension {
    /// Links
    pub links: Vec<Link>,
}

impl AtomExtension {
    /// Retrieve links
    pub fn links(&self) -> &[Link] {
        &self.links
    }

    /// Set links
    pub fn set_links<V>(&mut self, links: V)
    where
        V: Into<Vec<Link>>,
    {
        self.links = links.into();
    }
}

impl AtomExtension {
    /// Creates an `AtomExtension` using the specified `HashMap`.
    pub fn from_map(mut map: HashMap<String, Vec<Extension>>) -> Self {
        let mut ext = Self::default();

        ext.links = map
            .remove("link")
            .unwrap_or_default()
            .into_iter()
            .filter_map(|mut link_ext| {
                let href = link_ext.attrs.remove("href")?;

                let mut link = Link::default();
                link.href = href;
                if let Some(rel) = link_ext.attrs.remove("rel") {
                    link.rel = rel;
                }
                link.hreflang = link_ext.attrs.remove("hreflang");
                link.mime_type = link_ext.attrs.remove("type");
                link.title = link_ext.attrs.remove("title");
                link.length = link_ext.attrs.remove("length");
                Some(link)
            })
            .collect();

        ext
    }
}

impl ToXml for AtomExtension {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        for link in &self.links {
            let name = b"link";
            let mut element = BytesStart::borrowed(name, name.len());
            element.push_attribute(("href", &*link.href));
            element.push_attribute(("rel", &*link.rel));

            if let Some(ref hreflang) = link.hreflang {
                element.push_attribute(("hreflang", &**hreflang));
            }

            if let Some(ref mime_type) = link.mime_type {
                element.push_attribute(("type", &**mime_type));
            }

            if let Some(ref title) = link.title {
                element.push_attribute(("title", &**title));
            }

            if let Some(ref length) = link.length {
                element.push_attribute(("length", &**length));
            }

            writer.write_event(Event::Empty(element))?;
        }
        Ok(())
    }
}
