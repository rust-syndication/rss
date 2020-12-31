extern crate rss;

use rss::{extension, Channel, ChannelBuilder, CloudBuilder, EnclosureBuilder, ItemBuilder};
use std::collections::HashMap;

macro_rules! test_write {
    ($channel: ident) => {{
        let output = $channel.to_string();
        let parsed = output.parse::<Channel>().expect("failed to parse xml");
        assert_eq!($channel, parsed);
    }};
}

#[test]
fn write_channel() {
    let input = include_str!("data/channel.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");
    test_write!(channel);
}

#[test]
fn write_item() {
    let input = include_str!("data/item.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");
    test_write!(channel);
}

#[test]
fn write_content() {
    let input = include_str!("data/content.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");
    test_write!(channel);
}

#[test]
fn write_source() {
    let input = include_str!("data/source.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");
    test_write!(channel);
}

#[test]
fn write_guid() {
    let input = include_str!("data/guid.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");
    test_write!(channel);
}

#[test]
fn write_enclosure() {
    let input = include_str!("data/enclosure.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");
    test_write!(channel);
}

#[test]
fn write_category() {
    let input = include_str!("data/category.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");
    test_write!(channel);
}

#[test]
fn write_image() {
    let input = include_str!("data/image.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");
    test_write!(channel);
}

#[test]
fn write_mixed_content() {
    let input = include_str!("data/mixed_content.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");
    test_write!(channel);
}

#[test]
fn write_cloud() {
    let input = include_str!("data/cloud.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");
    test_write!(channel);
}

#[test]
fn write_textinput() {
    let input = include_str!("data/textinput.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");
    test_write!(channel);
}

#[test]
fn write_extension() {
    let input = include_str!("data/extension.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");
    test_write!(channel);
}

#[test]
fn write_itunes() {
    let input = include_str!("data/itunes.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");
    test_write!(channel);
}

#[test]
fn write_dublincore() {
    let input = include_str!("data/dublincore.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");
    test_write!(channel);
}

#[test]
fn write_syndication() {
    let input = include_str!("data/syndication.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");
    test_write!(channel);
}

#[test]
fn verify_write_format() {
    let item = ItemBuilder::default()
        .itunes_ext(extension::itunes::ITunesItemExtension::default())
        .dublin_core_ext(extension::dublincore::DublinCoreExtension::default())
        .build()
        .unwrap();

    let mut namespaces: HashMap<String, String> = HashMap::new();
    namespaces.insert("ext".to_string(), "http://example.com/".to_string());

    let channel = ChannelBuilder::default()
        .title("Title")
        .link("http://example.com/")
        .description("Description")
        .items(vec![item])
        .namespaces(namespaces)
        .build()
        .unwrap();

    let output = include_str!("data/verify_write_format.xml")
        .replace("\n", "")
        .replace("\r", "")
        .replace("\t", "");

    assert_eq!(channel.to_string(), output);
}

#[test]
fn test_content_namespace() {
    let channel = ChannelBuilder::default()
        .items(vec![ItemBuilder::default()
            .content("Lorem ipsum dolor sit amet".to_owned())
            .build()
            .unwrap()])
        .build()
        .unwrap();
    let xml = channel.to_string();

    assert!(xml.contains("xmlns:content="));
    assert!(!xml.contains("xmlns:dc="));
    assert!(!xml.contains("xmlns:itunes="));
}

#[test]
fn test_namespaces() {
    let channel = ChannelBuilder::default()
        .items(vec![ItemBuilder::default()
            .content("Lorem ipsum dolor sit amet".to_owned())
            .itunes_ext(
                extension::itunes::ITunesItemExtensionBuilder::default()
                    .author("Anonymous".to_owned())
                    .build()
                    .unwrap(),
            )
            .dublin_core_ext(
                extension::dublincore::DublinCoreExtensionBuilder::default()
                    .languages(vec!["English".to_owned(), "Deutsch".to_owned()])
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap()])
        .build()
        .unwrap();
    let xml = channel.to_string();

    assert!(xml.contains("xmlns:content="));
    assert!(xml.contains("xmlns:dc="));
    assert!(xml.contains("xmlns:itunes="));
}

#[test]
fn test_escape() {
    let mut attrs = HashMap::new();
    attrs.insert("key1".to_owned(), "value 1&2".to_owned());
    attrs.insert("key2".to_owned(), "value 2&3".to_owned());

    let mut extension_tag = HashMap::new();
    extension_tag.insert(
        "n1".to_owned(),
        vec![extension::ExtensionBuilder::default()
            .name("ext")
            .attrs(attrs)
            .build()
            .unwrap()],
    );

    let mut extensions = HashMap::new();
    extensions.insert("e1".to_owned(), extension_tag);

    let channel = ChannelBuilder::default()
        .cloud(
            CloudBuilder::default()
                .domain("example.com")
                .port("80")
                .path("/rpc?r=1&p=2&c=3")
                .register_procedure("notify")
                .protocol("xml-rpc")
                .build()
                .unwrap(),
        )
        .extensions(extensions)
        .items(vec![ItemBuilder::default()
            .content("Lorem ipsum dolor sit amet".to_owned())
            .enclosure(
                EnclosureBuilder::default()
                    .url("http://example.com?test=1&another=true")
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap()])
        .build()
        .unwrap();
    let xml = channel.to_string();

    assert!(xml.contains("value 1&amp;2"));
    assert!(xml.contains("value 2&amp;3"));
    assert!(xml.contains("r=1&amp;p=2&amp;c=3"));
    assert!(xml.contains("http://example.com?test=1&amp;another=true"));
}
