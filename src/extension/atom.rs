// This file is part of rss.
//
// Copyright © 2015-2020 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::collections::BTreeMap;
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
#[cfg_attr(
    feature = "builders",
    builder(
        setter(into),
        default,
        build_fn(name = "build_impl", private, error = "never::Never")
    )
)]
pub struct AtomExtension {
    /// Links
    #[cfg_attr(feature = "builders", builder(setter(each = "link")))]
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
    /// Creates an `AtomExtension` using the specified `BTreeMap`.
    pub fn from_map(mut map: BTreeMap<String, Vec<Extension>>) -> Self {
        let links = map
            .remove("link")
            .unwrap_or_default()
            .into_iter()
            .filter_map(|mut link_ext| {
                Some(Link {
                    href: link_ext.attrs.remove("href")?,
                    rel: link_ext
                        .attrs
                        .remove("rel")
                        .unwrap_or_else(|| Link::default().rel),
                    hreflang: link_ext.attrs.remove("hreflang"),
                    mime_type: link_ext.attrs.remove("type"),
                    title: link_ext.attrs.remove("title"),
                    length: link_ext.attrs.remove("length"),
                })
            })
            .collect();

        Self { links }
    }
}

impl ToXml for AtomExtension {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        for link in &self.links {
            let mut element = BytesStart::new("atom:link");
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

    fn used_namespaces(&self) -> BTreeMap<String, String> {
        let mut namespaces = BTreeMap::new();
        namespaces.insert("atom".to_owned(), NAMESPACE.to_owned());
        namespaces
    }
}

#[cfg(feature = "builders")]
impl AtomExtensionBuilder {
    /// Builds a new `AtomExtension`.
    pub fn build(&self) -> AtomExtension {
        self.build_impl().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "builders")]
    #[cfg(feature = "atom")]
    fn test_builder() {
        use atom_syndication::LinkBuilder;
        assert_eq!(
            AtomExtensionBuilder::default()
                .link(
                    LinkBuilder::default()
                        .rel("self")
                        .href("http://example.com/feed")
                        .build(),
                )
                .link(
                    LinkBuilder::default()
                        .rel("alternate")
                        .href("http://example.com")
                        .build(),
                )
                .build(),
            AtomExtension {
                links: vec![
                    Link {
                        rel: "self".to_string(),
                        href: "http://example.com/feed".to_string(),
                        ..Default::default()
                    },
                    Link {
                        rel: "alternate".to_string(),
                        href: "http://example.com".to_string(),
                        ..Default::default()
                    }
                ]
            }
        );
    }
}
