use std::collections::HashMap;

use extension::Extension;
use extension::remove_extension_value;

/// An iTunes channel element extension.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ITunesChannelExtension {
    /// The author of the podcast.
    pub author: Option<String>,
    /// Specifies if the podcast should be prevented from appearing in the iTunes Store. A value of
    /// `Yes` indicates that the podcast should not show up in the iTunes Store. All other values
    /// are ignored.
    pub block: Option<String>,
    /// The iTunes categories the podcast belongs to.
    pub categories: Option<Vec<ITunesCategory>>,
    /// The artwork for the podcast.
    pub image: Option<String>,
    /// Specifies whether the podcast contains explicit content. A value of `Yes`, `Explicit`, or
    /// `True` indicates that the podcast contains explicit content. A value of `Clean`, `No`,
    /// `False` inidicates that none of the episodes contain explicit content.
    pub explicit: Option<String>,
    /// Specifies whether the podcast is complete and no new episodes will be posted. A value of
    /// `Yes` indicates that the podcast is complete.
    pub complete: Option<String>,
    /// The new URL where the podcast is located.
    pub new_feed_url: Option<String>,
    /// The contact information for the owner of the podcast.
    pub owner: Option<ITunesOwner>,
    /// A description of the podcast.
    pub subtitle: Option<String>,
    /// A summary of the podcast.
    pub summary: Option<String>,
    /// Keywords for the podcast. The string contains a comma separated list of keywords.
    pub keywords: Option<String>,
}

impl ITunesChannelExtension {
    /// Creates an ITunesChannelExtension using the specified hashmap.
    pub fn from_map(mut map: HashMap<String, Vec<Extension>>) -> Self {
        let mut ext = ITunesChannelExtension::default();
        ext.author = remove_extension_value(&mut map, "author");
        ext.block = remove_extension_value(&mut map, "block");
        ext.categories = parse_categories(&mut map);
        ext.image = parse_image(&mut map);
        ext.explicit = remove_extension_value(&mut map, "explicit");
        ext.complete = remove_extension_value(&mut map, "complete");
        ext.new_feed_url = remove_extension_value(&mut map, "new-feed-url");
        ext.owner = parse_owner(&mut map);
        ext.subtitle = remove_extension_value(&mut map, "subtitle");
        ext.summary = remove_extension_value(&mut map, "summary");
        ext.keywords = remove_extension_value(&mut map, "keywords");
        ext
    }
}

/// An iTunes item element extension.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ITunesItemExtension {
    /// The author of the podcast episode.
    pub author: Option<String>,
    /// Specifies if the podcast episode should be prevented from appearing in the iTunes Store. A
    /// value of `Yes` indicates that the episode should not show up in the iTunes Store. All other
    /// values are ignored.
    pub block: Option<String>,
    /// The artwork for the podcast episode.
    pub image: Option<String>,
    /// The podcast episode duration in one of the following formats: HH:MM:SS, H:MM:SS, MM:SS,
    /// M:SS.
    pub duration: Option<String>,
    /// Specifies whether the podcast episode contains explicit content. A value of `Yes`,
    /// `Explicit`, or `True` indicates that the episode contains explicit content. A value of
    /// `Clean`, `No`, `False` inidicates that episode does not contain explicit content.
    pub explicit: Option<String>,
    /// Specifies whether the podcast episode contains embedded closed captioning. A value of `Yes`
    /// indicates that it does. Any other value indicates that it does not.
    pub closed_captioned: Option<String>,
    /// A value used to override the default sorting order for episodes.
    pub order: Option<String>,
    /// A description of the podcast episode.
    pub subtitle: Option<String>,
    /// A summary of the podcast episode.
    pub summary: Option<String>,
    /// Keywords for the podcast. The string contains a comma separated list of keywords.
    pub keywords: Option<String>,
}

impl ITunesItemExtension {
    /// Creates an ITunesChannelExtension using the specified hashmap.
    pub fn from_map(mut map: HashMap<String, Vec<Extension>>) -> Self {
        let mut ext = ITunesItemExtension::default();
        ext.author = remove_extension_value(&mut map, "author");
        ext.block = remove_extension_value(&mut map, "block");
        ext.image = parse_image(&mut map);
        ext.duration = remove_extension_value(&mut map, "duration");
        ext.explicit = remove_extension_value(&mut map, "explicit");
        ext.closed_captioned = remove_extension_value(&mut map, "isClosedCaptioned");
        ext.order = remove_extension_value(&mut map, "order");
        ext.subtitle = remove_extension_value(&mut map, "subtitle");
        ext.summary = remove_extension_value(&mut map, "summary");
        ext.keywords = remove_extension_value(&mut map, "keywords");
        ext
    }
}

/// A category for an iTunes podcast.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ITunesCategory {
    /// The name of the category.
    pub text: String,
    // This is contained within a Box to ensure it gets allocated on the heap to prevent an
    // infinite size.
    /// An optional subcategory for the cagetory.
    pub subcategory: Option<Box<ITunesCategory>>,
}

/// The contact information for the owner of an iTunes podcast.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ITunesOwner {
    /// The name of the owner.
    pub name: Option<String>,
    /// The email of the email.
    pub email: Option<String>,
}

fn parse_image(map: &mut HashMap<String, Vec<Extension>>) -> Option<String> {
    let mut element = match map.remove("image").map(|mut v| v.remove(0)) {
        Some(element) => element,
        None => return None,
    };

    element.attrs.remove("href")
}

fn parse_categories(map: &mut HashMap<String, Vec<Extension>>) -> Option<Vec<ITunesCategory>> {
    let mut elements = match map.remove("category") {
        Some(elements) => elements,
        None => return None,
    };

    let mut categories = Vec::with_capacity(elements.len());

    for elem in &mut elements {
        let text = elem.attrs.remove("text").unwrap_or_default();

        let child = {
            if let Some(mut child) = elem.children.remove("category").map(|mut v| v.remove(0)) {
                let text = child.attrs.remove("text").unwrap_or_default();
                Some(Box::new(ITunesCategory {
                    text: text,
                    subcategory: None,
                }))
            } else {
                None
            }
        };

        categories.push(ITunesCategory {
            text: text,
            subcategory: child,
        })
    }

    Some(categories)
}

fn parse_owner(map: &mut HashMap<String, Vec<Extension>>) -> Option<ITunesOwner> {
    let mut element = match map.remove("owner").map(|mut v| v.remove(0)) {
        Some(element) => element,
        None => return None,
    };

    let name = element.children.remove("name").and_then(|mut v| v.remove(0).value);
    let email = element.children.remove("email").and_then(|mut v| v.remove(0).value);

    Some(ITunesOwner {
        name: name,
        email: email,
    })
}
