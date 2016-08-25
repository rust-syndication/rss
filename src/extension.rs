use std::collections::HashMap;

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
    /// The children of the extension element. This is a map of qualified names to child
    /// elements.
    pub children: HashMap<String, Vec<Extension>>,
}
