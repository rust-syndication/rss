extern crate quick_xml;

use quick_xml::XmlWriter;
use quick_xml::error::Error as XmlError;

use std::collections::HashMap;

use extension::Extension;
use extension::remove_extension_values;

use toxml::{ToXml, XmlWriterExt};

/// The Dublin Core XML namespace.
pub static NAMESPACE: &'static str = "http://purl.org/dc/elements/1.1/";

/// A Dublin Core element extension.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DublinCoreExtension {
    /// An entity responsible for making contributions to the resource.
    pub contributor: Vec<String>,
    /// The spatial or temporal topic of the resource, the spatial applicability of the resource,
    /// or the jurisdiction under which the resource is relevant.
    pub coverage: Vec<String>,
    /// An entity primarily responsible for making the resource.
    pub creator: Vec<String>,
    /// A point or period of time associated with an event in the lifecycle of the resource.
    pub date: Vec<String>,
    /// An account of the resource.
    pub description: Vec<String>,
    /// The file format, physical medium, or dimensions of the resource.
    pub format: Vec<String>,
    /// An unambiguous reference to the resource within a given context.
    pub identifier: Vec<String>,
    /// A language of the resource.
    pub language: Vec<String>,
    /// An entity responsible for making the resource available.
    pub publisher: Vec<String>,
    /// A related resource.
    pub relation: Vec<String>,
    /// Information about rights held in and over the resource.
    pub rights: Vec<String>,
    /// A related resource from which the described resource is derived.
    pub source: Vec<String>,
    /// The topic of the resource.
    pub subject: Vec<String>,
    /// A name given to the resource.
    pub title: Vec<String>,
    /// The nature or genre of the resource.
    pub resource_type: Vec<String>,
}

impl DublinCoreExtension {
    /// Creates a DublinCoreExtension using the specified hashmap.
    pub fn from_map(mut map: HashMap<String, Vec<Extension>>) -> Self {
        let mut ext = DublinCoreExtension::default();
        ext.contributor = remove_extension_values(&mut map, "contributor").unwrap_or_default();
        ext.coverage = remove_extension_values(&mut map, "coverage").unwrap_or_default();
        ext.creator = remove_extension_values(&mut map, "creator").unwrap_or_default();
        ext.date = remove_extension_values(&mut map, "date").unwrap_or_default();
        ext.description = remove_extension_values(&mut map, "description").unwrap_or_default();
        ext.format = remove_extension_values(&mut map, "format").unwrap_or_default();
        ext.identifier = remove_extension_values(&mut map, "identifier").unwrap_or_default();
        ext.language = remove_extension_values(&mut map, "language").unwrap_or_default();
        ext.publisher = remove_extension_values(&mut map, "publisher").unwrap_or_default();
        ext.relation = remove_extension_values(&mut map, "relation").unwrap_or_default();
        ext.rights = remove_extension_values(&mut map, "rights").unwrap_or_default();
        ext.source = remove_extension_values(&mut map, "source").unwrap_or_default();
        ext.subject = remove_extension_values(&mut map, "subject").unwrap_or_default();
        ext.title = remove_extension_values(&mut map, "title").unwrap_or_default();
        ext.resource_type = remove_extension_values(&mut map, "type").unwrap_or_default();
        ext
    }
}

impl ToXml for DublinCoreExtension {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut XmlWriter<W>) -> Result<(), XmlError> {
        try!(writer.write_text_elements(b"dc:contributor", &self.contributor));
        try!(writer.write_text_elements(b"dc:coverage", &self.coverage));
        try!(writer.write_text_elements(b"dc:creator", &self.creator));
        try!(writer.write_text_elements(b"dc:date", &self.date));
        try!(writer.write_text_elements(b"dc:description", &self.description));
        try!(writer.write_text_elements(b"dc:format", &self.format));
        try!(writer.write_text_elements(b"dc:identifier", &self.identifier));
        try!(writer.write_text_elements(b"dc:language", &self.language));
        try!(writer.write_text_elements(b"dc:publisher", &self.publisher));
        try!(writer.write_text_elements(b"dc:relation", &self.relation));
        try!(writer.write_text_elements(b"dc:rights", &self.rights));
        try!(writer.write_text_elements(b"dc:source", &self.source));
        try!(writer.write_text_elements(b"dc:subject", &self.subject));
        try!(writer.write_text_elements(b"dc:title", &self.title));
        writer.write_text_elements(b"dc:type", &self.resource_type)
    }
}
