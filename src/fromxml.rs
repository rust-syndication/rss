// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::io::BufRead;

use quick_xml::events::attributes::Attributes;
use quick_xml::Reader;

use error::Error;

pub trait FromXml: Sized {
    fn from_xml<R: BufRead>(reader: &mut Reader<R>, atts: Attributes) -> Result<Self, Error>;
}
