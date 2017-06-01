// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use extension::Extension;
use extension::remove_extension_values;
use quick_xml::Writer;
use quick_xml::errors::Error as XmlError;
use std::collections::HashMap;
use toxml::{ToXml, WriterExt};

/// The Dublin Core XML namespace.
pub static NAMESPACE: &'static str = "http://purl.org/dc/elements/1.1/";

/// A Dublin Core element extension.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DublinCoreExtension {
    /// An entity responsible for making contributions to the resource.
    contributors: Vec<String>,
    /// The spatial or temporal topic of the resource, the spatial applicability of the resource,
    /// or the jurisdiction under which the resource is relevant.
    coverages: Vec<String>,
    /// An entity primarily responsible for making the resource.
    creators: Vec<String>,
    /// A point or period of time associated with an event in the lifecycle of the resource.
    dates: Vec<String>,
    /// An account of the resource.
    descriptions: Vec<String>,
    /// The file format, physical medium, or dimensions of the resource.
    formats: Vec<String>,
    /// An unambiguous reference to the resource within a given context.
    identifiers: Vec<String>,
    /// A language of the resource.
    languages: Vec<String>,
    /// An entity responsible for making the resource available.
    publishers: Vec<String>,
    /// A related resource.
    relations: Vec<String>,
    /// Information about rights held in and over the resource.
    rights: Vec<String>,
    /// A related resource from which the described resource is derived.
    sources: Vec<String>,
    /// The topic of the resource.
    subjects: Vec<String>,
    /// A name given to the resource.
    titles: Vec<String>,
    /// The nature or genre of the resource.
    resource_types: Vec<String>,
}

impl DublinCoreExtension {
    /// Return the contributors for this `DublinCoreExtension`.
    pub fn contributors(&self) -> &[String] {
        &self.contributors
    }

    /// Return the coverages for this `DublinCoreExtension`.
    pub fn coverages(&self) -> &[String] {
        &self.coverages
    }

    /// Return the creators for this `DublinCoreExtension`.
    pub fn creators(&self) -> &[String] {
        &self.creators
    }

    /// Return the dates for this `DublinCoreExtension`.
    pub fn dates(&self) -> &[String] {
        &self.dates
    }

    /// Return the descriptions for this `DublinCoreExtension`.
    pub fn descriptions(&self) -> &[String] {
        &self.descriptions
    }

    /// Return the formats for this `DublinCoreExtension`.
    pub fn formats(&self) -> &[String] {
        &self.formats
    }

    /// Return the identifiers for this `DublinCoreExtension`.
    pub fn identifiers(&self) -> &[String] {
        &self.identifiers
    }

    /// Return the languages for this `DublinCoreExtension`.
    pub fn languages(&self) -> &[String] {
        &self.languages
    }

    /// Return the publishers for this `DublinCoreExtension`.
    pub fn publishers(&self) -> &[String] {
        &self.publishers
    }

    /// Return the relations for this `DublinCoreExtension`.
    pub fn relations(&self) -> &[String] {
        &self.relations
    }

    /// Return the rights for this `DublinCoreExtension`.
    pub fn rights(&self) -> &[String] {
        &self.rights
    }

    /// Return the sources for this `DublinCoreExtension`.
    pub fn sources(&self) -> &[String] {
        &self.sources
    }

    /// Return the subjects for this `DublinCoreExtension`.
    pub fn subjects(&self) -> &[String] {
        &self.subjects
    }

    /// Return the titles for this `DublinCoreExtension`.
    pub fn titles(&self) -> &[String] {
        &self.titles
    }

    /// Return the resource_types for this `DublinCoreExtension`.
    pub fn resource_types(&self) -> &[String] {
        &self.resource_types
    }
}

impl DublinCoreExtension {
    /// Creates a DublinCoreExtension using the specified hashmap.
    pub fn from_map(mut map: HashMap<String, Vec<Extension>>) -> Self {
        let mut ext = DublinCoreExtension::default();
        ext.contributors = remove_extension_values(&mut map, "contributor").unwrap_or_default();
        ext.coverages = remove_extension_values(&mut map, "coverage").unwrap_or_default();
        ext.creators = remove_extension_values(&mut map, "creator").unwrap_or_default();
        ext.dates = remove_extension_values(&mut map, "date").unwrap_or_default();
        ext.descriptions = remove_extension_values(&mut map, "description").unwrap_or_default();
        ext.formats = remove_extension_values(&mut map, "format").unwrap_or_default();
        ext.identifiers = remove_extension_values(&mut map, "identifier").unwrap_or_default();
        ext.languages = remove_extension_values(&mut map, "language").unwrap_or_default();
        ext.publishers = remove_extension_values(&mut map, "publisher").unwrap_or_default();
        ext.relations = remove_extension_values(&mut map, "relation").unwrap_or_default();
        ext.rights = remove_extension_values(&mut map, "rights").unwrap_or_default();
        ext.sources = remove_extension_values(&mut map, "source").unwrap_or_default();
        ext.subjects = remove_extension_values(&mut map, "subject").unwrap_or_default();
        ext.titles = remove_extension_values(&mut map, "title").unwrap_or_default();
        ext.resource_types = remove_extension_values(&mut map, "type").unwrap_or_default();
        ext
    }
}

impl ToXml for DublinCoreExtension {
    fn to_xml<W: ::std::io::Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        writer
            .write_text_elements(b"dc:contributor", &self.contributors)?;
        writer.write_text_elements(b"dc:coverage", &self.coverages)?;
        writer.write_text_elements(b"dc:creator", &self.creators)?;
        writer.write_text_elements(b"dc:date", &self.dates)?;
        writer
            .write_text_elements(b"dc:description", &self.descriptions)?;
        writer.write_text_elements(b"dc:format", &self.formats)?;
        writer
            .write_text_elements(b"dc:identifier", &self.identifiers)?;
        writer.write_text_elements(b"dc:language", &self.languages)?;
        writer
            .write_text_elements(b"dc:publisher", &self.publishers)?;
        writer.write_text_elements(b"dc:relation", &self.relations)?;
        writer.write_text_elements(b"dc:rights", &self.rights)?;
        writer.write_text_elements(b"dc:source", &self.sources)?;
        writer.write_text_elements(b"dc:subject", &self.subjects)?;
        writer.write_text_elements(b"dc:title", &self.titles)?;
        writer.write_text_elements(b"dc:type", &self.resource_types)
    }
}

/// A builder used to create a `DublinCoreExtension`.
#[derive(Debug, Default, Clone)]
pub struct DublinCoreExtensionBuilder {
    /// An entity responsible for making contributions to the resource.
    contributors: Vec<String>,
    /// The spatial or temporal topic of the resource, the spatial applicability of the resource,
    /// or the jurisdiction under which the resource is relevant.
    coverages: Vec<String>,
    /// An entity primarily responsible for making the resource.
    creators: Vec<String>,
    /// A point or period of time associated with an event in the lifecycle of the resource.
    dates: Vec<String>,
    /// An account of the resource.
    descriptions: Vec<String>,
    /// The file format, physical medium, or dimensions of the resource.
    formats: Vec<String>,
    /// An unambiguous reference to the resource within a given context.
    identifiers: Vec<String>,
    /// A language of the resource.
    languages: Vec<String>,
    /// An entity responsible for making the resource available.
    publishers: Vec<String>,
    /// A related resource.
    relations: Vec<String>,
    /// Information about rights held in and over the resource.
    rights: Vec<String>,
    /// A related resource from which the described resource is derived.
    sources: Vec<String>,
    /// The topic of the resource.
    subjects: Vec<String>,
    /// A name given to the resource.
    titles: Vec<String>,
    /// The nature or genre of the resource.
    resource_types: Vec<String>,
}

impl DublinCoreExtensionBuilder {
    /// Construct a new `DublinCoreExtensionBuilder` using the values from an existing
    /// `DublinCoreExtension`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rss::Channel;
    /// use rss::extension::dublincore::DublinCoreExtensionBuilder;
    ///
    /// let input = include_str!("tests/data/dublincore.xml");
    /// let channel = input.parse::<Channel>().unwrap();
    /// let extension = channel.dublin_core_ext().unwrap().clone();
    /// let builder = DublinCoreExtensionBuilder::from_extension(extension);
    /// ```
    pub fn from_extension(extension: DublinCoreExtension) -> Self {
        DublinCoreExtensionBuilder {
            contributors: extension.contributors,
            coverages: extension.coverages,
            creators: extension.creators,
            dates: extension.dates,
            descriptions: extension.descriptions,
            formats: extension.formats,
            identifiers: extension.identifiers,
            languages: extension.languages,
            publishers: extension.publishers,
            relations: extension.relations,
            rights: extension.rights,
            sources: extension.sources,
            subjects: extension.subjects,
            titles: extension.titles,
            resource_types: extension.resource_types,
        }
    }

    /// Set the contributors for the `DublinCoreExtension`.
    pub fn contributors<V>(mut self, contributors: V) -> DublinCoreExtensionBuilder
        where V: Into<Vec<String>>
    {
        self.contributors = contributors.into();
        self
    }

    /// Set the coverages for the `DublinCoreExtension`.
    pub fn coverages<V>(mut self, coverages: V) -> DublinCoreExtensionBuilder
        where V: Into<Vec<String>>
    {
        self.coverages = coverages.into();
        self
    }

    /// Set the creators for the `DublinCoreExtension`.
    pub fn creators<V>(mut self, creators: V) -> DublinCoreExtensionBuilder
        where V: Into<Vec<String>>
    {
        self.creators = creators.into();
        self
    }

    /// Set the dates for the `DublinCoreExtension`.
    pub fn dates<V>(mut self, dates: V) -> DublinCoreExtensionBuilder
        where V: Into<Vec<String>>
    {
        self.dates = dates.into();
        self
    }

    /// Set the descriptions for the `DublinCoreExtension`.
    pub fn descriptions<V>(mut self, descriptions: V) -> DublinCoreExtensionBuilder
        where V: Into<Vec<String>>
    {
        self.descriptions = descriptions.into();
        self
    }

    /// Set the formats for the `DublinCoreExtension`.
    pub fn formats<V>(mut self, formats: V) -> DublinCoreExtensionBuilder
        where V: Into<Vec<String>>
    {
        self.formats = formats.into();
        self
    }

    /// Set the identifiers for the `DublinCoreExtension`.
    pub fn identifiers<V>(mut self, identifiers: V) -> DublinCoreExtensionBuilder
        where V: Into<Vec<String>>
    {
        self.identifiers = identifiers.into();
        self
    }

    /// Set the languages for the `DublinCoreExtension`.
    pub fn languages<V>(mut self, languages: V) -> DublinCoreExtensionBuilder
        where V: Into<Vec<String>>
    {
        self.languages = languages.into();
        self
    }

    /// Set the publishers for the `DublinCoreExtension`.
    pub fn publishers<V>(mut self, publishers: V) -> DublinCoreExtensionBuilder
        where V: Into<Vec<String>>
    {
        self.publishers = publishers.into();
        self
    }

    /// Set the relations for the `DublinCoreExtension`.
    pub fn relations<V>(mut self, relations: V) -> DublinCoreExtensionBuilder
        where V: Into<Vec<String>>
    {
        self.relations = relations.into();
        self
    }

    /// Set the rights for the `DublinCoreExtension`.
    pub fn rights<V>(mut self, rights: V) -> DublinCoreExtensionBuilder
        where V: Into<Vec<String>>
    {
        self.rights = rights.into();
        self
    }

    /// Set the sources for the `DublinCoreExtension`.
    pub fn sources<V>(mut self, sources: V) -> DublinCoreExtensionBuilder
        where V: Into<Vec<String>>
    {
        self.sources = sources.into();
        self
    }

    /// Set the subjects for the `DublinCoreExtension`.
    pub fn subjects<V>(mut self, subjects: V) -> DublinCoreExtensionBuilder
        where V: Into<Vec<String>>
    {
        self.subjects = subjects.into();
        self
    }

    /// Set the titles for the `DublinCoreExtension`.
    pub fn titles<V>(mut self, titles: V) -> DublinCoreExtensionBuilder
        where V: Into<Vec<String>>
    {
        self.titles = titles.into();
        self
    }

    /// Set the resource_types for the `DublinCoreExtension`.
    pub fn resource_types<V>(mut self, resource_types: V) -> DublinCoreExtensionBuilder
        where V: Into<Vec<String>>
    {
        self.resource_types = resource_types.into();
        self
    }

    /// Construct the `DublinCoreExtension` from this `DublinCoreExtensionBuilder`.
    pub fn finalize(self) -> DublinCoreExtension {
        DublinCoreExtension {
            contributors: self.contributors,
            coverages: self.coverages,
            creators: self.creators,
            dates: self.dates,
            descriptions: self.descriptions,
            formats: self.formats,
            identifiers: self.identifiers,
            languages: self.languages,
            publishers: self.publishers,
            relations: self.relations,
            rights: self.rights,
            sources: self.sources,
            subjects: self.subjects,
            titles: self.titles,
            resource_types: self.resource_types,
        }
    }
}
