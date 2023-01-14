// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::collections::BTreeMap;
use std::io::Write;
use std::str;

use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Error as XmlError;
use quick_xml::Writer;

use crate::toxml::ToXml;

/// Types and methods for [Atom](https://www.rssboard.org/rss-profile#namespace-elements-atom) extensions.
#[cfg(feature = "atom")]
pub mod atom;

/// Types and methods for
/// [iTunes](https://help.apple.com/itc/podcasts_connect/#/itcb54353390) extensions.
pub mod itunes;

/// Types and methods for [Dublin Core](http://dublincore.org/documents/dces/) extensions.
pub mod dublincore;

/// Types and methods for [Syndication](http://web.resource.org/rss/1.0/modules/syndication/) extensions.
pub mod syndication;

pub(crate) mod util;

/// A map of extension namespace prefixes to local names to elements.
pub type ExtensionMap = BTreeMap<String, BTreeMap<String, Vec<Extension>>>;

/// A namespaced extension such as iTunes or Dublin Core.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[cfg_attr(
    feature = "builders",
    builder(
        setter(into),
        default,
        build_fn(name = "build_impl", private, error = "never::Never")
    )
)]
pub struct Extension {
    /// The qualified name of the extension element.
    pub name: String,
    /// The content of the extension element.
    pub value: Option<String>,
    /// The attributes for the extension element.
    #[cfg_attr(feature = "builders", builder(setter(each = "attr")))]
    pub attrs: BTreeMap<String, String>,
    /// The children of the extension element. This is a map of local names to child
    /// elements.
    #[cfg_attr(feature = "builders", builder(setter(each = "child")))]
    pub children: BTreeMap<String, Vec<Extension>>,
}

impl Extension {
    /// Return the qualified name of this extension.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Set the qualified name of this extension.
    pub fn set_name<V>(&mut self, name: V)
    where
        V: Into<String>,
    {
        self.name = name.into();
    }

    /// Return the text content of this extension.
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    /// Set the text content of this extension.
    pub fn set_value<V>(&mut self, value: V)
    where
        V: Into<Option<String>>,
    {
        self.value = value.into();
    }

    /// Return the attributes for the extension element.
    pub fn attrs(&self) -> &BTreeMap<String, String> {
        &self.attrs
    }

    /// Return the children of the extension element.
    ///
    /// This is a map of local names to child elements.
    pub fn children(&self) -> &BTreeMap<String, Vec<Extension>> {
        &self.children
    }
}

impl ToXml for Extension {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let mut element = BytesStart::new(&self.name);
        element.extend_attributes(self.attrs.iter().map(|a| (a.0.as_str(), a.1.as_str())));
        writer.write_event(Event::Start(element))?;

        if let Some(ref value) = self.value {
            writer.write_event(Event::Text(BytesText::new(value)))?;
        }

        for extension in self.children.values().flatten() {
            extension.to_xml(writer)?;
        }

        writer.write_event(Event::End(BytesEnd::new(&self.name)))?;
        Ok(())
    }
}

#[cfg(feature = "builders")]
impl ExtensionBuilder {
    /// Builds a new `Extension`.
    pub fn build(&self) -> Extension {
        self.build_impl().unwrap()
    }
}
