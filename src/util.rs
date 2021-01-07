// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::io::BufRead;

use quick_xml::events::Event;
use quick_xml::Reader;

use crate::error::Error;

pub fn element_text<R: BufRead>(reader: &mut Reader<R>) -> Result<Option<String>, Error> {
    let mut content: Option<String> = None;
    let mut buf = Vec::new();
    let mut skip_buf = Vec::new();

    loop {
        match reader.read_event(&mut buf)? {
            Event::Start(element) => {
                reader.read_to_end(element.name(), &mut skip_buf)?;
            }
            Event::CData(element) => {
                let text = reader.decode(&*element).into();
                content = Some(text);
            }
            Event::Text(element) => {
                let text = element.unescape_and_decode(reader)?;
                content = Some(text);
            }
            Event::End(_) | Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(content)
}
