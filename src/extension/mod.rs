// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use error::Error;
use quick_xml::{Element, Event, XmlWriter};
use quick_xml::error::Error as XmlError;
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
    /// Get the name that exists under `Extension`.
    pub fn name(&self) -> String {
        self.name
            .clone()
    }

    /// Get the value that exists under `Extension`.
    pub fn value(&self) -> Option<String> {
        self.value
            .clone()
    }

    /// Get the attrs that exists under `Extension`.
    pub fn attrs(&self) -> HashMap<String, String> {
        self.attrs
            .clone()
    }

    /// Get the children that exists under `Extension`.
    pub fn children(&self) -> HashMap<String, Vec<Extension>> {
        self.children
            .clone()
    }
}

impl ToXml for Extension {
    fn to_xml<W: ::std::io::Write>(&self,
                                   writer: &mut XmlWriter<W>)
        -> Result<(), XmlError> {
        let element = Element::new(&self.name);

        writer.write(Event::Start({
                                      let mut element = element.clone();
                                      element.extend_attributes(&self.attrs);
                                      element
                                  }))?;

        if let Some(value) = self.value
                                 .as_ref() {
            writer.write(Event::Text(Element::new(value)))?;
        }

        for extensions in self.children
                              .values() {
            for extension in extensions {
                extension.to_xml(writer)?;
            }
        }

        writer.write(Event::End(element))
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
    // Construct a new `DublinCoreExtensionBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::extension::ExtensionBuilder;
    ///
    /// let extension_builder = ExtensionBuilder::new();
    /// ```
    pub fn new() -> ExtensionBuilder {
        ExtensionBuilder::default()
    }

    /// Get the name that exists under `Extension`.
    pub fn name(&mut self,
                name: &str)
        -> &mut ExtensionBuilder {
        self.name = String::from(name);
        self
    }

    /// Get the value that exists under `Extension`.
    pub fn value(&mut self,
                 value: Option<String>)
        -> &mut ExtensionBuilder {
        self.value = value;
        self
    }

    /// Get the attrs that exists under `Extension`.
    pub fn attrs(&mut self,
                 attrs: HashMap<String, String>)
        -> &mut ExtensionBuilder {
        self.attrs = attrs;
        self
    }

    /// Get the children that exists under `Extension`.
    pub fn children(&mut self,
                    children: HashMap<String, Vec<Extension>>)
        -> &mut ExtensionBuilder {
        self.children = children;
        self
    }

    /// Construct the `ExtensionBuilder` from the `ExtensionBuilderBuilder`.
    pub fn finalize(&self) -> Result<Extension, Error> {
        Ok(Extension { name: self.name
                                 .clone(),
                       value: self.value
                                  .clone(),
                       attrs: self.attrs
                                  .clone(),
                       children: self.children
                                     .clone(), })
    }
}

/// Get a reference to the value for the first extension with the specified key.
pub fn get_extension_value<'a>(map: &'a HashMap<String, Vec<Extension>>,
                               key: &str)
    -> Option<&'a str> {
    map.get(key)
       .and_then(|v| v.first())
       .and_then(|ext| {
                     ext.value
                        .as_ref()
                 })
       .map(|s| s.as_str())
}

/// Remove and return the value for the first extension with the specified key.
pub fn remove_extension_value(map: &mut HashMap<String, Vec<Extension>>,
                              key: &str)
    -> Option<String> {
    map.remove(key)
       .map(|mut v| v.remove(0))
       .and_then(|ext| ext.value)
}

/// Get a reference to all values for the extensions with the specified key.
pub fn get_extension_values<'a>(map: &'a HashMap<String, Vec<Extension>>,
                                key: &str)
    -> Option<Vec<&'a str>> {
    map.get(key)
       .map(|v| {
                v.iter()
                 .filter_map(|ext| {
                                 ext.value
                                    .as_ref()
                                    .map(|s| s.as_str())
                             })
                 .collect::<Vec<_>>()
            })
}

/// Remove and return all values for the extensions with the specified key.
pub fn remove_extension_values(map: &mut HashMap<String, Vec<Extension>>,
                               key: &str)
    -> Option<Vec<String>> {
    map.remove(key)
       .map(|v| {
                v.into_iter()
                 .filter_map(|ext| ext.value)
                 .collect::<Vec<_>>()
            })
}
