// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::collections::HashMap;

use extension::Extension;

mod itunes_category;
mod itunes_channel_extension;
mod itunes_item_extension;
mod itunes_owner;

pub use self::itunes_category::*;
pub use self::itunes_channel_extension::*;
pub use self::itunes_item_extension::*;
pub use self::itunes_owner::*;

/// The iTunes XML namespace.
pub const NAMESPACE: &str = "http://www.itunes.com/dtds/podcast-1.0.dtd";

fn parse_image(map: &mut HashMap<String, Vec<Extension>>) -> Option<String> {
    let mut element = match map.remove("image").map(|mut v| v.remove(0)) {
        Some(element) => element,
        None => return None,
    };

    element.attrs.remove("href")
}

fn parse_categories(map: &mut HashMap<String, Vec<Extension>>) -> Vec<ITunesCategory> {
    let mut elements = match map.remove("category") {
        Some(elements) => elements,
        None => return Vec::new(),
    };

    let mut categories = Vec::with_capacity(elements.len());

    for elem in &mut elements {
        let text = elem.attrs.remove("text").unwrap_or_default();

        let child = {
            if let Some(mut child) = elem.children.remove("category").map(|mut v| v.remove(0)) {
                let text = child.attrs.remove("text").unwrap_or_default();
                let mut category = ITunesCategory::default();
                category.set_text(text);
                Some(Box::new(category))
            } else {
                None
            }
        };

        let mut category = ITunesCategory::default();
        category.set_text(text);
        category.set_subcategory(child);
        categories.push(category);
    }

    categories
}

fn parse_owner(map: &mut HashMap<String, Vec<Extension>>) -> Option<ITunesOwner> {
    if let Some(mut element) = map.remove("owner").map(|mut v| v.remove(0)) {
        let name = element
            .children
            .remove("name")
            .and_then(|mut v| v.remove(0).value);

        let email = element
            .children
            .remove("email")
            .and_then(|mut v| v.remove(0).value);

        let mut owner = ITunesOwner::default();
        owner.set_name(name);
        owner.set_email(email);
        Some(owner)
    } else {
        None
    }
}
