extern crate quick_xml;

use quick_xml::{XmlWriter, Element, Event};
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
    pub name: String,
    /// The content of the extension element.
    pub value: Option<String>,
    /// The attributes for the extension element.
    pub attrs: HashMap<String, String>,
    /// The children of the extension element. This is a map of local names to child
    /// elements.
    pub children: HashMap<String, Vec<Extension>>,
}

impl ToXml for Extension {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut XmlWriter<W>) -> Result<(), XmlError> {
        let element = Element::new(&self.name);

        try!(writer.write(Event::Start({
            let mut element = element.clone();
            element.extend_attributes(&self.attrs);
            element
        })));

        if let Some(value) = self.value.as_ref() {
            try!(writer.write(Event::Text(Element::new(value))));
        }

        for extensions in self.children.values() {
            for extension in extensions {
                try!(extension.to_xml(writer));
            }
        }

        writer.write(Event::End(element))
    }
}

/// Get a reference to the value for the first extension with the specified key.
pub fn get_extension_value<'a>(map: &'a HashMap<String, Vec<Extension>>,
                               key: &str)
                               -> Option<&'a str> {
    map.get(key).and_then(|v| v.first()).and_then(|ext| ext.value.as_ref()).map(|s| s.as_str())
}

/// Remove and return the value for the first extension with the specified key.
pub fn remove_extension_value(map: &mut HashMap<String, Vec<Extension>>,
                              key: &str)
                              -> Option<String> {
    map.remove(key).map(|mut v| v.remove(0)).and_then(|ext| ext.value)
}

/// Get a reference to all values for the extensions with the specified key.
pub fn get_extension_values<'a>(map: &'a HashMap<String, Vec<Extension>>,
                                key: &str)
                                -> Option<Vec<&'a str>> {
    map.get(key).map(|v| {
        v.iter().filter_map(|ext| ext.value.as_ref().map(|s| s.as_str())).collect::<Vec<_>>()
    })
}

/// Remove and return all values for the extensions with the specified key.
pub fn remove_extension_values(map: &mut HashMap<String, Vec<Extension>>,
                               key: &str)
                               -> Option<Vec<String>> {
    map.remove(key).map(|v| v.into_iter().filter_map(|ext| ext.value).collect::<Vec<_>>())
}
