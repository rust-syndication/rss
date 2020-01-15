// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::collections::HashMap;
use std::fmt;
use std::io::Write;
use std::str::FromStr;

use quick_xml::Error as XmlError;
use quick_xml::Writer;

use crate::extension::Extension;
use crate::toxml::WriterExt;

/// The Syndication XML namespace.
pub const NAMESPACE: &str = "http://purl.org/rss/1.0/modules/syndication/";

/// The unit of time between updates/refreshes
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum UpdatePeriod {
    /// refresh hourly
    HOURLY,
    /// refresh daily
    DAILY,
    /// refresh weekly
    WEEKLY,
    /// refresh monthly
    MONTHLY,
    /// refresh yearly
    YEARLY,
}

impl FromStr for UpdatePeriod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hourly" => Ok(UpdatePeriod::HOURLY),
            "daily" => Ok(UpdatePeriod::DAILY),
            "weekly" => Ok(UpdatePeriod::WEEKLY),
            "monthly" => Ok(UpdatePeriod::MONTHLY),
            "yearly" => Ok(UpdatePeriod::YEARLY),
            _ => Err(())
        }
    }
}

impl fmt::Display for UpdatePeriod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdatePeriod::HOURLY => write!(f, "hourly"),
            UpdatePeriod::DAILY => write!(f, "daily"),
            UpdatePeriod::WEEKLY => write!(f, "weekly"),
            UpdatePeriod::MONTHLY => write!(f, "monthly"),
            UpdatePeriod::YEARLY => write!(f, "yearly"),
        }
    }
}

/// An RSS syndication element extension.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[cfg_attr(feature = "builders", builder(setter(into), default))]
pub struct SyndicationExtension {
    /// The refresh period for this channel
    period: UpdatePeriod,
    /// Number of periods between refreshes
    frequency: u32,
    /// Timestamp from which the refresh periods are calculated
    base: String,
}

impl SyndicationExtension {
    /// Retrieve the base timestamp from which the refresh periods are calculated
    pub fn base(&self) -> &str {
        &self.base
    }

    /// Set the base from which the refresh periods are calculated
    pub fn set_base(&mut self, base: &str) {
        self.base = base.to_owned();
    }

    /// Retrieve the number of periods between refreshes
    pub fn frequency(&self) -> u32 {
        self.frequency
    }

    /// Set the number of periods between refreshes
    pub fn set_frequency(&mut self, frequency: u32) {
        self.frequency = frequency;
    }

    /// Retrieve the refresh period for this channel
    pub fn period(&self) -> &UpdatePeriod {
        &self.period
    }

    /// Set the refresh period for this channel
    pub fn set_period(&mut self, period: UpdatePeriod) {
        self.period = period;
    }

    /// Serialises this extension to the nominated writer
    pub fn to_xml<W: Write>(&self, namespaces: &HashMap<String, String>, writer: &mut Writer<W>) -> Result<(), XmlError> {
        for (prefix, namespace) in namespaces {
            if NAMESPACE == namespace {
                writer.write_text_element(format!("{}:updatePeriod", prefix), &self.period.to_string())?;
                writer.write_text_element(format!("{}:updateFrequency", prefix), &format!("{}", self.frequency))?;
                writer.write_text_element(format!("{}:updateBase", prefix), &self.base)?;
            }
        }
        Ok(())
    }
}

impl Default for SyndicationExtension {
    fn default() -> Self {
        SyndicationExtension { period: UpdatePeriod::DAILY, frequency: 1, base: String::from("1970-01-01T00:00+00:00") }
    }
}

/// Retrieves the extensions for the nominated field and runs the callback if there is at least 1 extension value
fn with_first_ext_value<'a, F>(map: &'a HashMap<String, Vec<Extension>>, field: &str, f: F)
    where F: FnOnce(&'a str) {
    if let Some(extensions) = map.get(field) {
        if !extensions.is_empty() {
            if let Some(v) = extensions[0].value.as_ref() {
                f(v);
            }
        }
    }
}

impl SyndicationExtension {
    /// Creates a `SyndicationExtension` using the specified `HashMap`.
    pub fn from_map(map: HashMap<String, Vec<Extension>>) -> Self {
        let mut syn = SyndicationExtension::default();

        with_first_ext_value(&map, "updatePeriod", |value| syn.period = value.parse().unwrap());
        with_first_ext_value(&map, "updateFrequency", |value| syn.frequency = value.parse().unwrap());
        with_first_ext_value(&map, "updateBase", |value| syn.base = value.to_owned());

        syn
    }
}
