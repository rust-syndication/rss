// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use error::Error;
use extension::Extension;
use std::collections::HashMap;

mod itunes_category;
pub use extension::itunes::itunes_category::ITunesCategory;
pub use extension::itunes::itunes_category::ITunesCategoryBuilder;

mod itunes_owner;
pub use extension::itunes::itunes_owner::ITunesOwner;
pub use extension::itunes::itunes_owner::ITunesOwnerBuilder;

mod itunes_item_extension;
pub use extension::itunes::itunes_item_extension::ITunesItemExtension;
pub use extension::itunes::itunes_item_extension::ITunesItemExtensionBuilder;

mod itunes_channel_extension;
pub use extension::itunes::itunes_channel_extension::ITunesChannelExtension;
pub use extension::itunes::itunes_channel_extension::ITunesChannelExtensionBuilder;

/// The iTunes XML namespace.
pub static NAMESPACE: &'static str = "http://www.itunes.com/dtds/podcast-1.0.dtd";

fn parse_image(map: &mut HashMap<String, Vec<Extension>>) -> Option<String> {
    let mut element = match map.remove("image")
                               .map(|mut v| v.remove(0)) {
        Some(element) => element,
        None => return None,
    };

    element.attrs
           .remove("href")
}

fn parse_categories(map: &mut HashMap<String, Vec<Extension>>) -> Result<Vec<ITunesCategory>, Error> {
    let mut elements = match map.remove("category") {
        Some(elements) => elements,
        None => return Ok(Vec::new()),
    };

    let mut categories = Vec::with_capacity(elements.len());

    for elem in &mut elements {
        let text = elem.attrs
                       .remove("text")
                       .unwrap_or_default();

        let child = {
            if let Some(mut child) = elem.children
                                         .remove("category")
                                         .map(|mut v| v.remove(0)) {
                let text = child.attrs
                                .remove("text")
                                .unwrap_or_default();
                Some(Box::new(ITunesCategoryBuilder::new()
                                  .text(text.as_str())
                                  .subcategory(None)
                                  .finalize()?))
            } else {
                None
            }
        };

        categories.push(ITunesCategoryBuilder::new()
                            .text(text.as_str())
                            .subcategory(child)
                            .finalize()?);
    }

    Ok(categories)
}

fn parse_owner(map: &mut HashMap<String, Vec<Extension>>) -> Result<Option<ITunesOwner>, Error> {
    let mut element = match map.remove("owner")
                               .map(|mut v| v.remove(0)) {
        Some(element) => element,
        None => return Ok(None),
    };

    let name = element.children
                      .remove("name")
                      .and_then(|mut v| {
                                    v.remove(0)
                                     .value
                                });
    let email = element.children
                       .remove("email")
                       .and_then(|mut v| {
                                     v.remove(0)
                                      .value
                                 });

    Ok(Some(ITunesOwnerBuilder::new()
                .name(name)
                .email(email)
                .finalize()?))
}
