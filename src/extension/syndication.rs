// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::collections::BTreeMap;
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
    Hourly,
    /// refresh daily
    Daily,
    /// refresh weekly
    Weekly,
    /// refresh monthly
    Monthly,
    /// refresh yearly
    Yearly,
}

impl FromStr for UpdatePeriod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hourly" => Ok(UpdatePeriod::Hourly),
            "daily" => Ok(UpdatePeriod::Daily),
            "weekly" => Ok(UpdatePeriod::Weekly),
            "monthly" => Ok(UpdatePeriod::Monthly),
            "yearly" => Ok(UpdatePeriod::Yearly),
            _ => Err(()),
        }
    }
}

impl fmt::Display for UpdatePeriod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdatePeriod::Hourly => write!(f, "hourly"),
            UpdatePeriod::Daily => write!(f, "daily"),
            UpdatePeriod::Weekly => write!(f, "weekly"),
            UpdatePeriod::Monthly => write!(f, "monthly"),
            UpdatePeriod::Yearly => write!(f, "yearly"),
        }
    }
}

/// An RSS syndication element extension.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[cfg_attr(
    feature = "builders",
    builder(
        setter(into),
        default,
        build_fn(name = "build_impl", private, error = "never::Never")
    )
)]
pub struct SyndicationExtension {
    /// The refresh period for this channel
    pub period: UpdatePeriod,
    /// Number of periods between refreshes
    pub frequency: u32,
    /// Timestamp from which the refresh periods are calculated
    pub base: String,
}

impl SyndicationExtension {
    /// Retrieve the base timestamp from which the refresh periods are calculated
    pub fn base(&self) -> &str {
        &self.base
    }

    /// Set the base from which the refresh periods are calculated
    pub fn set_base(&mut self, base: &str) {
        base.clone_into(&mut self.base);
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

    /// Serializes this extension to the nominated writer
    pub fn to_xml<W: Write>(
        &self,
        namespaces: &BTreeMap<String, String>,
        writer: &mut Writer<W>,
    ) -> Result<(), XmlError> {
        for (prefix, namespace) in namespaces {
            if NAMESPACE == namespace {
                writer.write_text_element(
                    format!("{}:updatePeriod", prefix),
                    &self.period.to_string(),
                )?;
                writer.write_text_element(
                    format!("{}:updateFrequency", prefix),
                    &format!("{}", self.frequency),
                )?;
                writer.write_text_element(format!("{}:updateBase", prefix), &self.base)?;
            }
        }
        Ok(())
    }
}

impl Default for SyndicationExtension {
    fn default() -> Self {
        SyndicationExtension {
            period: UpdatePeriod::Daily,
            frequency: 1,
            base: String::from("1970-01-01T00:00+00:00"),
        }
    }
}

/// Retrieves the extensions for the nominated field and runs the callback if there is at least 1 extension value
fn with_first_ext_value<'a, F>(map: &'a BTreeMap<String, Vec<Extension>>, field: &str, f: F)
where
    F: FnOnce(&'a str),
{
    if let Some(extensions) = map.get(field) {
        if !extensions.is_empty() {
            if let Some(v) = extensions[0].value.as_ref() {
                f(v);
            }
        }
    }
}

impl SyndicationExtension {
    /// Creates a `SyndicationExtension` using the specified `BTreeMap`.
    pub fn from_map(map: BTreeMap<String, Vec<Extension>>) -> Self {
        let mut syn = SyndicationExtension::default();

        with_first_ext_value(&map, "updatePeriod", |value| {
            if let Ok(update_period) = value.parse() {
                syn.period = update_period
            }
        });
        with_first_ext_value(&map, "updateFrequency", |value| {
            if let Ok(frequency) = value.parse() {
                syn.frequency = frequency
            }
        });
        with_first_ext_value(&map, "updateBase", |value| value.clone_into(&mut syn.base));

        syn
    }
}

#[cfg(feature = "builders")]
impl SyndicationExtensionBuilder {
    /// Builds a new `SyndicationExtension`.
    pub fn build(&self) -> SyndicationExtension {
        self.build_impl().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "builders")]
    fn test_builder() {
        assert_eq!(
            SyndicationExtensionBuilder::default()
                .period(UpdatePeriod::Weekly)
                .frequency(2_u32)
                .base("2021-01-01T00:00+00:00")
                .build(),
            SyndicationExtension {
                period: UpdatePeriod::Weekly,
                frequency: 2,
                base: "2021-01-01T00:00+00:00".to_string(),
            }
        );
    }
}
