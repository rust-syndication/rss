use std::collections::HashMap;

use extension::Extension;
use extension::remove_extension_values;

/// A Dublin Core element extension.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DublinCoreExtension {
    /// An entity responsible for making contributions to the resource.
    pub contributor: Option<Vec<String>>,
    /// The spatial or temporal topic of the resource, the spatial applicability of the resource,
    /// or the jurisdiction under which the resource is relevant.
    pub coverage: Option<Vec<String>>,
    /// An entity primarily responsible for making the resource.
    pub creator: Option<Vec<String>>,
    /// A point or period of time associated with an event in the lifecycle of the resource.
    pub date: Option<Vec<String>>,
    /// An account of the resource.
    pub description: Option<Vec<String>>,
    /// The file format, physical medium, or dimensions of the resource.
    pub format: Option<Vec<String>>,
    /// An unambiguous reference to the resource within a given context.
    pub identifier: Option<Vec<String>>,
    /// A language of the resource.
    pub language: Option<Vec<String>>,
    /// An entity responsible for making the resource available.
    pub publisher: Option<Vec<String>>,
    /// A related resource.
    pub relation: Option<Vec<String>>,
    /// Information about rights held in and over the resource.
    pub rights: Option<Vec<String>>,
    /// A related resource from which the described resource is derived.
    pub source: Option<Vec<String>>,
    /// The topic of the resource.
    pub subject: Option<Vec<String>>,
    /// A name given to the resource.
    pub title: Option<Vec<String>>,
    /// The nature or genre of the resource.
    pub resource_type: Option<Vec<String>>,
}

impl DublinCoreExtension {
    /// Creates a DublinCoreExtension using the specified hashmap.
    pub fn from_map(mut map: HashMap<String, Vec<Extension>>) -> Self {
        let mut ext = DublinCoreExtension::default();
        ext.contributor = remove_extension_values(&mut map, "contributor");
        ext.coverage = remove_extension_values(&mut map, "coverage");
        ext.creator = remove_extension_values(&mut map, "creator");
        ext.date = remove_extension_values(&mut map, "date");
        ext.description = remove_extension_values(&mut map, "description");
        ext.format = remove_extension_values(&mut map, "format");
        ext.identifier = remove_extension_values(&mut map, "identifier");
        ext.language = remove_extension_values(&mut map, "language");
        ext.publisher = remove_extension_values(&mut map, "publisher");
        ext.relation = remove_extension_values(&mut map, "relation");
        ext.rights = remove_extension_values(&mut map, "rights");
        ext.source = remove_extension_values(&mut map, "source");
        ext.subject = remove_extension_values(&mut map, "subject");
        ext.title = remove_extension_values(&mut map, "title");
        ext.resource_type = remove_extension_values(&mut map, "type");
        ext
    }
}
