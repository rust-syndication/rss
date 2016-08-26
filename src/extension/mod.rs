use std::collections::HashMap;

/// Types and functions for
/// [iTunes extensions](https://help.apple.com/itc/podcasts_connect/#/itcb54353390).
pub mod itunes;

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

/// Get a reference to the value for the first extension with the specified key.
pub fn get_extension_text<'a>(map: &'a HashMap<String, Vec<Extension>>,
                              key: &str)
                              -> Option<&'a str> {
    map.get(key).and_then(|v| v.first()).and_then(|ext| ext.value.as_ref()).map(|s| s.as_str())
}

/// Remove and return the value for the first extension with the specified key.
pub fn remove_extension_text(map: &mut HashMap<String, Vec<Extension>>,
                             key: &str)
                             -> Option<String> {
    map.remove(key).map(|mut v| v.remove(0)).and_then(|ext| ext.value)
}

/// Get a reference to the values for the extensions with the specified key.
pub fn get_extension_array<'a>(map: &'a HashMap<String, Vec<Extension>>,
                               key: &str)
                               -> Option<Vec<&'a str>> {
    map.get(key).map(|v| {
        v.iter().filter_map(|ext| ext.value.as_ref().map(|s| s.as_str())).collect::<Vec<_>>()
    })
}

/// Remove and return the values for the extensions with the specified key.
pub fn remove_extension_array(map: &mut HashMap<String, Vec<Extension>>,
                              key: &str)
                              -> Option<Vec<String>> {
    map.remove(key).map(|v| v.into_iter().filter_map(|ext| ext.value).collect::<Vec<_>>())
}
