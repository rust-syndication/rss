extern crate rss;

use rss::{Channel, ChannelBuilder, ItemBuilder, extension};
use std::collections::HashMap;

macro_rules! test_write {
    ($channel:ident) => ({
        let output = $channel.to_string();
        let parsed = output.parse::<Channel>().expect("failed to parse xml");
        assert_eq!($channel, parsed);
    })
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
fn verify_write_format() {
    let item = ItemBuilder::new()
        .itunes_ext(Some(extension::itunes::ITunesItemExtensionBuilder::new()
                             .finalize()
                             .unwrap()))
        .dublin_core_ext(Some(extension::dublincore::DublinCoreExtensionBuilder::new()
                                  .finalize()
                                  .unwrap()))
        .finalize()
        .unwrap();

    let mut namespaces: HashMap<String, String> = HashMap::new();
    namespaces.insert("ext".to_string(), "http://example.com/".to_string());

    let channel = ChannelBuilder::new()
        .title("Title")
        .link("http://example.com/")
        .description("Description")
        .items(vec![item])
        .namespaces(namespaces)
        .finalize()
        .unwrap();

    let output = include_str!("data/verify_write_format.xml")
        .replace("\n", "")
        .replace("\t", "");

    assert_eq!(channel.to_string(), output);
}
