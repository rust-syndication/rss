// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::collections::BTreeMap;
use std::io::Write;

use quick_xml::Error as XmlError;
use quick_xml::Writer;

use crate::extension::util::get_extension_values;
use crate::extension::Extension;

use crate::toxml::{ToXml, WriterExt};

/// The Dublin Core XML namespace.
pub const NAMESPACE: &str = "http://purl.org/dc/elements/1.1/";

/// A Dublin Core element extension.
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
pub struct DublinCoreExtension {
    /// An entity responsible for making contributions to the resource.
    #[cfg_attr(feature = "builders", builder(setter(each = "contributor")))]
    pub contributors: Vec<String>,
    /// The spatial or temporal topic of the resource, the spatial applicability of the resource,
    /// or the jurisdiction under which the resource is relevant.
    #[cfg_attr(feature = "builders", builder(setter(each = "coverage")))]
    pub coverages: Vec<String>,
    /// An entity primarily responsible for making the resource.
    #[cfg_attr(feature = "builders", builder(setter(each = "creator")))]
    pub creators: Vec<String>,
    /// A point or period of time associated with an event in the lifecycle of the resource.
    #[cfg_attr(feature = "builders", builder(setter(each = "date")))]
    pub dates: Vec<String>,
    /// An account of the resource.
    #[cfg_attr(feature = "builders", builder(setter(each = "description")))]
    pub descriptions: Vec<String>,
    /// The file format, physical medium, or dimensions of the resource.
    #[cfg_attr(feature = "builders", builder(setter(each = "format")))]
    pub formats: Vec<String>,
    /// An unambiguous reference to the resource within a given context.
    #[cfg_attr(feature = "builders", builder(setter(each = "identifier")))]
    pub identifiers: Vec<String>,
    /// A language of the resource.
    #[cfg_attr(feature = "builders", builder(setter(each = "language")))]
    pub languages: Vec<String>,
    /// An entity responsible for making the resource available.
    #[cfg_attr(feature = "builders", builder(setter(each = "publisher")))]
    pub publishers: Vec<String>,
    /// A related resource.
    #[cfg_attr(feature = "builders", builder(setter(each = "relation")))]
    pub relations: Vec<String>,
    /// Information about rights held in and over the resource.
    #[cfg_attr(feature = "builders", builder(setter(each = "right")))]
    pub rights: Vec<String>,
    /// A related resource from which the described resource is derived.
    #[cfg_attr(feature = "builders", builder(setter(each = "source")))]
    pub sources: Vec<String>,
    /// The topic of the resource.
    #[cfg_attr(feature = "builders", builder(setter(each = "subject")))]
    pub subjects: Vec<String>,
    /// A name given to the resource.
    #[cfg_attr(feature = "builders", builder(setter(each = "title")))]
    pub titles: Vec<String>,
    /// The nature or genre of the resource.
    #[cfg_attr(feature = "builders", builder(setter(each = "r#type")))]
    pub types: Vec<String>,
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
    /// Creates a `DublinCoreExtension` using the specified `BTreeMap`.
    pub fn from_map(map: BTreeMap<String, Vec<Extension>>) -> Self {
        let mut ext = DublinCoreExtension::default();
        for (key, v) in map {
            match key.as_str() {
                "contributor" => ext.contributors = get_extension_values(v),
                "coverage" => ext.coverages = get_extension_values(v),
                "creator" => ext.creators = get_extension_values(v),
                "date" => ext.dates = get_extension_values(v),
                "description" => ext.descriptions = get_extension_values(v),
                "format" => ext.formats = get_extension_values(v),
                "identifier" => ext.identifiers = get_extension_values(v),
                "language" => ext.languages = get_extension_values(v),
                "publisher" => ext.publishers = get_extension_values(v),
                "relation" => ext.relations = get_extension_values(v),
                "rights" => ext.rights = get_extension_values(v),
                "source" => ext.sources = get_extension_values(v),
                "subject" => ext.subjects = get_extension_values(v),
                "title" => ext.titles = get_extension_values(v),
                "type" => ext.types = get_extension_values(v),
                _ => {}
            }
        }
        ext
    }
}

impl ToXml for DublinCoreExtension {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        writer.write_text_elements("dc:contributor", &self.contributors)?;
        writer.write_text_elements("dc:coverage", &self.coverages)?;
        writer.write_text_elements("dc:creator", &self.creators)?;
        writer.write_text_elements("dc:date", &self.dates)?;
        writer.write_text_elements("dc:description", &self.descriptions)?;
        writer.write_text_elements("dc:format", &self.formats)?;
        writer.write_text_elements("dc:identifier", &self.identifiers)?;
        writer.write_text_elements("dc:language", &self.languages)?;
        writer.write_text_elements("dc:publisher", &self.publishers)?;
        writer.write_text_elements("dc:relation", &self.relations)?;
        writer.write_text_elements("dc:rights", &self.rights)?;
        writer.write_text_elements("dc:source", &self.sources)?;
        writer.write_text_elements("dc:subject", &self.subjects)?;
        writer.write_text_elements("dc:title", &self.titles)?;
        writer.write_text_elements("dc:type", &self.types)?;
        Ok(())
    }

    fn used_namespaces(&self) -> BTreeMap<String, String> {
        let mut namespaces = BTreeMap::new();
        namespaces.insert("dc".to_owned(), NAMESPACE.to_owned());
        namespaces
    }
}

#[cfg(feature = "builders")]
impl DublinCoreExtensionBuilder {
    /// Builds a new `DublinCoreExtension`.
    pub fn build(&self) -> DublinCoreExtension {
        self.build_impl().unwrap()
    }
}
