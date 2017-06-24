// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use quick_xml::errors::Error as XmlError;
use quick_xml::events::{Event, BytesStart, BytesEnd, BytesText};
use quick_xml::writer::Writer;

use std::collections::HashMap;
use toxml::ToXml;

/// Types and functions for
/// [iTunes](https://help.apple.com/itc/podcasts_connect/#/itcb54353390) extensions.
pub mod itunes;

/// Types and functions for [Dublin Core](http://dublincore.org/documents/dces/) extensions.
pub mod dublincore;

/// A map of extension namespace prefixes to local names to elements.
pub type ExtensionMap = HashMap<String, HashMap<String, Vec<Extension>>>;

/// A namespaced extension such as iTunes or Dublin Core.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Extension {
    /// The qualified name of the extension element.
    name: String,
    /// The content of the extension element.
    value: Option<String>,
    /// The attributes for the extension element.
    attrs: HashMap<String, String>,
    /// The children of the extension element. This is a map of local names to child
    /// elements.
    children: HashMap<String, Vec<Extension>>,
}

impl Extension {
    /// Return the qualified name of the extension element.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Return the content of the extension element.
    pub fn value(&self) -> Option<&str> {
        self.value.as_ref().map(|s| s.as_str())
    }

    /// Return the attributes for the extension element.
    pub fn attrs(&self) -> &HashMap<String, String> {
        &self.attrs
    }

    /// Return the children of the extension element.
    ///
    /// This is a map of local names to child elements.
    pub fn children(&self) -> &HashMap<String, Vec<Extension>> {
        &self.children
    }
}

impl ToXml for Extension {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {

        let name_len = self.name.len();
        let mut element = BytesStart::borrowed(self.name.as_bytes(), name_len);
        element.extend_attributes(self.attrs.iter().map(|a| (a.0.as_bytes(), a.1.as_bytes())));
        writer.write_event(Event::Start(element))?;

        if let Some(value) = self.value.as_ref() {
            writer.write_event(
                Event::Text(BytesText::borrowed(value.as_bytes())),
            )?;
        }

        for extension in self.children.values().flat_map(|extensions| extensions) {
            extension.to_xml(writer)?;
        }

        writer.write_event(
            Event::End(BytesEnd::borrowed(self.name.as_bytes())),
        )?;
        Ok(())
    }
}

/// A namespaced extension such as iTunes or Dublin Core.
#[derive(Debug, Default, Clone)]
pub struct ExtensionBuilder {
    /// The qualified name of the extension element.
    name: String,
    /// The content of the extension element.
    value: Option<String>,
    /// The attributes for the extension element.
    attrs: HashMap<String, String>,
    /// The children of the extension element. This is a map of local names to child
    /// elements.
    children: HashMap<String, Vec<Extension>>,
}

impl ExtensionBuilder {
    /// Set the qualified name of the extension element.
    pub fn name<S>(mut self, name: S) -> ExtensionBuilder
    where
        S: Into<String>,
    {
        self.name = name.into();
        self
    }

    /// Set the content of the extension element.
    pub fn value<V>(mut self, value: V) -> ExtensionBuilder
    where
        V: Into<Option<String>>,
    {
        self.value = value.into();
        self
    }

    /// Set the attributes for the extension element.
    pub fn attrs<V>(mut self, attrs: V) -> ExtensionBuilder
    where
        V: Into<HashMap<String, String>>,
    {
        self.attrs = attrs.into();
        self
    }

    /// Set the children of the extension element.
    pub fn children<V>(mut self, children: V) -> ExtensionBuilder
    where
        V: Into<HashMap<String, Vec<Extension>>>,
    {
        self.children = children.into();
        self
    }

    /// Construct the `ExtensionBuilder` from this `ExtensionBuilderBuilder`.
    pub fn finalize(self) -> Extension {
        Extension {
            name: self.name,
            value: self.value,
            attrs: self.attrs,
            children: self.children,
        }
    }
}

/// Get a reference to the value for the first extension with the specified key.
pub fn get_extension_value<'a>(
    map: &'a HashMap<String, Vec<Extension>>,
    key: &str,
) -> Option<&'a str> {
    map.get(key)
        .and_then(|v| v.first())
        .and_then(|ext| ext.value.as_ref())
        .map(|s| s.as_str())
}

/// Remove and return the value for the first extension with the specified key.
pub fn remove_extension_value(
    map: &mut HashMap<String, Vec<Extension>>,
    key: &str,
) -> Option<String> {
    map.remove(key).map(|mut v| v.remove(0)).and_then(
        |ext| ext.value,
    )
}

/// Get a reference to all values for the extensions with the specified key.
pub fn get_extension_values<'a>(
    map: &'a HashMap<String, Vec<Extension>>,
    key: &str,
) -> Option<Vec<&'a str>> {
    map.get(key).map(|v| {
        v.iter()
            .filter_map(|ext| ext.value.as_ref().map(|s| s.as_str()))
            .collect::<Vec<_>>()
    })
}

/// Remove and return all values for the extensions with the specified key.
pub fn remove_extension_values(
    map: &mut HashMap<String, Vec<Extension>>,
    key: &str,
) -> Option<Vec<String>> {
    map.remove(key).map(|v| {
        v.into_iter()
            .filter_map(|ext| ext.value)
            .collect::<Vec<_>>()
    })
}
