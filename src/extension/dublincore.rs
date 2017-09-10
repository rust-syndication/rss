// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::collections::HashMap;
use std::io::Write;

use quick_xml::errors::Error as XmlError;
use quick_xml::writer::Writer;

use extension::Extension;
use extension::util::remove_extension_values;

use toxml::{ToXml, WriterExt};

/// The Dublin Core XML namespace.
pub static NAMESPACE: &'static str = "http://purl.org/dc/elements/1.1/";

/// A Dublin Core element extension.
#[derive(Debug, Default, Clone, PartialEq, Builder)]
#[builder(setter(into), default)]
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
    types: Vec<String>,
}

impl DublinCoreExtension {
    /// Return the contributors to the resource.
    pub fn contributors(&self) -> &[String] {
        &self.contributors
    }

    /// Return a mutable slice of the contributors to the resource.
    pub fn contributors_mut(&mut self) -> &mut [String] {
        &mut self.contributors
    }

    /// Set the contributors to the resource.
    pub fn set_contributors<V>(&mut self, contributors: V)
    where
        V: Into<Vec<String>>,
    {
        self.contributors = contributors.into();
    }

    /// Return the spatial or temporal topics of the resource, the spatial applicabilities of the
    /// resource, or the jurisdictions under which the resource is relevant.
    pub fn coverages(&self) -> &[String] {
        &self.coverages
    }

    /// Return a mutable slice of the spatial or temporal topics of the resource, the spatial
    /// applicabilities of the resource, or the jurisdictions under which the resource is relevant.
    pub fn coverages_mut(&mut self) -> &mut [String] {
        &mut self.coverages
    }

    /// Set the spatial or temporal topics of the resource, the spatial applicabilities of the
    /// resource, or the jurisdictions under which the resource is relevant.
    pub fn set_coverages<V>(&mut self, coverages: V)
    where
        V: Into<Vec<String>>,
    {
        self.coverages = coverages.into();
    }

    /// Return the creators of the resource.
    pub fn creators(&self) -> &[String] {
        &self.creators
    }

    /// Return a mutable slice of the creators of the resource.
    pub fn creators_mut(&mut self) -> &mut [String] {
        &mut self.creators
    }

    /// Set the creators of the resource.
    pub fn set_creators<V>(&mut self, creators: V)
    where
        V: Into<Vec<String>>,
    {
        self.creators = creators.into();
    }

    /// Return the times associated with the resource.
    pub fn dates(&self) -> &[String] {
        &self.dates
    }

    /// Return a mutable slice of the times associated with the resource.
    pub fn dates_mut(&mut self) -> &mut [String] {
        &mut self.dates
    }

    /// Set the times associated with the resource.
    pub fn set_dates<V>(&mut self, dates: V)
    where
        V: Into<Vec<String>>,
    {
        self.dates = dates.into();
    }

    /// Return the descriptions of the resource.
    pub fn descriptions(&self) -> &[String] {
        &self.descriptions
    }

    /// Return a mutable slice of the descriptions of the resource.
    pub fn descriptions_mut(&mut self) -> &mut [String] {
        &mut self.descriptions
    }

    /// Set the descriptions of the resource.
    pub fn set_descriptions<V>(&mut self, descriptions: V)
    where
        V: Into<Vec<String>>,
    {
        self.descriptions = descriptions.into();
    }

    /// Return the file formats, physical mediums, or dimensions of the resource.
    pub fn formats(&self) -> &[String] {
        &self.formats
    }

    /// Return a mutable slice of the file formats, physical mediums, or
    /// dimensions of the resource.
    pub fn formats_mut(&mut self) -> &mut [String] {
        &mut self.formats
    }

    /// Set the file formats, physical mediums, or dimensions of the resource.
    pub fn set_formats<V>(&mut self, formats: V)
    where
        V: Into<Vec<String>>,
    {
        self.formats = formats.into();
    }

    /// Return the identifiers of the resource.
    pub fn identifiers(&self) -> &[String] {
        &self.identifiers
    }

    /// Return a mutable slice of the identifiers of the resource.
    pub fn identifiers_mut(&mut self) -> &mut [String] {
        &mut self.identifiers
    }

    /// Set the identifiers of the resource.
    pub fn set_identifiers<V>(&mut self, identifiers: V)
    where
        V: Into<Vec<String>>,
    {
        self.identifiers = identifiers.into();
    }

    /// Return the languages of the resource.
    pub fn languages(&self) -> &[String] {
        &self.languages
    }

    /// Return a mutable slice of the languages of the resource.
    pub fn languages_mut(&mut self) -> &mut [String] {
        &mut self.languages
    }

    /// Set the languages of the resource.
    pub fn set_languages<V>(&mut self, languages: V)
    where
        V: Into<Vec<String>>,
    {
        self.languages = languages.into();
    }

    /// Return the publishers of the resource.
    pub fn publishers(&self) -> &[String] {
        &self.publishers
    }

    /// Return a mutable slice of the publishers of the resource.
    pub fn publishers_mut(&mut self) -> &mut [String] {
        &mut self.publishers
    }

    /// Set the publishers of the resource.
    pub fn set_publishers<V>(&mut self, publishers: V)
    where
        V: Into<Vec<String>>,
    {
        self.publishers = publishers.into();
    }

    /// Return the related resources.
    pub fn relations(&self) -> &[String] {
        &self.relations
    }

    /// Return a mutable slice of the related resources.
    pub fn relations_mut(&mut self) -> &mut [String] {
        &mut self.relations
    }

    /// Set the related resources.
    pub fn set_relations<V>(&mut self, relations: V)
    where
        V: Into<Vec<String>>,
    {
        self.relations = relations.into();
    }

    /// Return the information about rights held in and over the resource.
    pub fn rights(&self) -> &[String] {
        &self.rights
    }

    /// Return a mutable slice of the information about rights held in and over
    /// the resource.
    pub fn rights_mut(&mut self) -> &mut [String] {
        &mut self.rights
    }

    /// Set the information about rights held in and over the resource.
    pub fn set_rights<V>(&mut self, rights: V)
    where
        V: Into<Vec<String>>,
    {
        self.rights = rights.into();
    }

    /// Return the sources of the resource.
    pub fn sources(&self) -> &[String] {
        &self.sources
    }

    /// Return a mutable slice of the sources of the resource.
    pub fn sources_mut(&mut self) -> &mut [String] {
        &mut self.sources
    }

    /// Set the sources of the resource.
    pub fn set_sources<V>(&mut self, sources: V)
    where
        V: Into<Vec<String>>,
    {
        self.sources = sources.into();
    }

    /// Return the topics of the resource.
    pub fn subjects(&self) -> &[String] {
        &self.subjects
    }

    /// Return a mutable slice of the subjects of the resource.
    pub fn subjects_mut(&mut self) -> &mut [String] {
        &mut self.subjects
    }

    /// Set the topics of the resource.
    pub fn set_subjects<V>(&mut self, subjects: V)
    where
        V: Into<Vec<String>>,
    {
        self.subjects = subjects.into();
    }

    /// Return the titles of the resource.
    pub fn titles(&self) -> &[String] {
        &self.titles
    }

    /// Return a mutable slice of the titles of the resource.
    pub fn titles_mut(&mut self) -> &mut [String] {
        &mut self.titles
    }

    /// Set the titles of the resource.
    pub fn set_titles<V>(&mut self, titles: V)
    where
        V: Into<Vec<String>>,
    {
        self.titles = titles.into();
    }

    /// Return the natures or genres of the resource.
    pub fn types(&self) -> &[String] {
        &self.types
    }

    /// Return a mutable slice of the natures or genres of the resource.
    pub fn types_mut(&mut self) -> &mut [String] {
        &mut self.types
    }

    /// set the natures or genres of the resource.
    pub fn set_types<V>(&mut self, types: V)
    where
        V: Into<Vec<String>>,
    {
        self.types = types.into();
    }
}

impl DublinCoreExtension {
    /// Creates a `DublinCoreExtension` using the specified `HashMap`.
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
        ext.types = remove_extension_values(&mut map, "type").unwrap_or_default();
        ext
    }
}

impl ToXml for DublinCoreExtension {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
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
        writer.write_text_elements(b"dc:type", &self.types)?;
        Ok(())
    }
}
