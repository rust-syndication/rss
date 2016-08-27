extern crate rss;

use rss::Channel;
use rss::extension::dublincore::DublinCoreExtension;
use rss::extension::get_extension_values;

#[test]
fn test_rss2sample() {
    let input = include_str!("data/rss2sample.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");
    assert_eq!(channel.title, "Liftoff News");
    assert_eq!(channel.link, "http://liftoff.msfc.nasa.gov/");
    assert_eq!(channel.description, "Liftoff to Space Exploration.");
    assert_eq!(channel.language.unwrap(), "en-us");
    assert_eq!(channel.pub_date.unwrap(), "Tue, 10 Jun 2003 04:00:00 GMT");
    assert_eq!(channel.last_build_date.unwrap(),
               "Tue, 10 Jun 2003 09:41:01 GMT");
    assert!(channel.categories.is_empty());

    assert_eq!(channel.items[0].title.as_ref().map(|s| s.as_str()),
               Some("Star City"));
    assert_eq!(channel.items[0].link.as_ref().map(|s| s.as_str()),
               Some("http://liftoff.msfc.nasa.gov/news/2003/news-starcity.asp"));
    assert_eq!(channel.items[0].description.as_ref().map(|s| s.as_str()),
               Some("How do Americans get ready to work with Russians aboard the \
                        International Space Station? They take a crash course in culture, \
                        language and protocol at Russia's \
                        <a href=\"http://howe.iki.rssi.ru/GCTC/gctc_e.htm\">Star City</a>."));
    assert_eq!(channel.items[0].pub_date.as_ref().map(|s| s.as_str()),
               Some("Tue, 03 Jun 2003 09:39:21 GMT"));
    assert_eq!(channel.items[0].guid.as_ref().map(|v| v.value.as_str()),
               Some("http://liftoff.msfc.nasa.gov/2003/06/03.html#item573"));
}

#[test]
fn test_content() {
    let input = include_str!("data/content.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");
    assert_eq!(channel.title, "Example");
    assert_eq!(channel.description, "An RSS Example with Slash");
    assert_eq!(channel.last_build_date.as_ref().map(|s| s.as_str()),
               Some("Sun, 15 May 2005 13:02:08 -0500"));
    assert_eq!(channel.link, "http://www.example.com");

    assert_eq!(channel.items[0].title.as_ref().map(|s| s.as_str()),
               Some("A Link in Here"));
    assert_eq!(channel.items[0].guid.as_ref().map(|v| v.value.as_str()),
               Some("d77d2e80-0487-4e8c-a35d-a93f12a0ff7d:2005/05/15/114"));
    assert_eq!(channel.items[0].pub_date.as_ref().map(|s| s.as_str()),
               Some("Sun, 15 May 2005 13:02:08 -0500"));
    assert_eq!(channel.items[0].link.as_ref().map(|s| s.as_str()),
               Some("http://www.example.com/blog/2005/05/15/114"));
    assert_eq!(channel.items[0].content.as_ref().map(|s| s.as_str()),
               Some("This is a <a href=\"http://example.com/\">link</a>."));
}

#[test]
fn test_source() {
    let input = include_str!("data/source.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");
    assert_eq!(channel.title, "Source Test");

    assert_eq!(channel.items[0].source.as_ref().map(|v| v.url.as_str()),
               Some("http://example.com/feed/"));
    assert_eq!(channel.items[0].source.as_ref().and_then(|v| v.title.as_ref().map(|s| s.as_str())),
               Some("Feed"));
}

#[test]
fn test_enclosure() {
    let input = include_str!("data/enclosure.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");
    assert_eq!(channel.title, "Enclosure Test");

    assert_eq!(channel.items[0].enclosure.as_ref().map(|v| v.url.as_str()),
               Some("http://example.com/media.mp3"));
    assert_eq!(channel.items[0].enclosure.as_ref().map(|v| v.length.as_str()),
               Some("4992349"));
    assert_eq!(channel.items[0].enclosure.as_ref().map(|v| v.mime_type.as_str()),
               Some("audio/mpeg"));
}

#[test]
fn test_extension() {
    let input = include_str!("data/extension.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");
    assert_eq!(channel.title, "Extension Test");

    let ext = channel.items[0].extensions.get("ext").expect("failed to find extension");
    assert_eq!(get_extension_values(&ext, "creator"),
               Some(vec!["Creator Name"]));
    assert_eq!(get_extension_values(&ext, "contributor"),
               Some(vec!["Contributor 1", "Contributor 2"]));
    assert_eq!(ext.get("parent")
                   .map(|v| {
            v.iter()
                .find(|v| v.children.contains_key("child"))
                .expect("failed to find child elements")
                .children
                .get("child")
                .unwrap()
                .iter()
                .map(|v| v.value.as_ref().map(|s| s.as_str()))
                .collect::<Vec<_>>()
        }),
               Some(vec![Some("Child 1"), Some("Child 2")]));
}

#[test]
fn test_itunes() {
    let input = include_str!("data/itunes.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    let itunes = channel.itunes_ext.expect("itunes extension missing");
    assert_eq!(itunes.author.as_ref().map(|s| s.as_str()),
               Some("Example author"));
    assert_eq!(itunes.block.as_ref().map(|s| s.as_str()), Some("yes"));
    assert_eq!(itunes.categories.as_ref().map(|v| v.len()), Some(1));
    assert_eq!(itunes.categories.as_ref().map(|v| v[0].text.as_str()),
               Some("Example Category"));
    assert_eq!(itunes.categories
                   .as_ref()
                   .and_then(|v| v[0].subcategory.as_ref())
                   .map(|v| v.text.as_str()),
               Some("Example Subcategory"));
    assert_eq!(itunes.image.as_ref().map(|s| s.as_str()),
               Some("http://example.com/image.jpg"));
    assert_eq!(itunes.explicit.as_ref().map(|s| s.as_str()), Some("no"));
    assert_eq!(itunes.complete.as_ref().map(|s| s.as_str()), Some("yes"));
    assert_eq!(itunes.new_feed_url.as_ref().map(|s| s.as_str()),
               Some("http://example.com/feed/"));
    assert_eq!(itunes.owner.as_ref().and_then(|v| v.name.as_ref()).map(|s| s.as_str()),
               Some("Example name"));
    assert_eq!(itunes.owner.as_ref().and_then(|v| v.email.as_ref()).map(|s| s.as_str()),
               Some("example@example.com"));
    assert_eq!(itunes.subtitle.as_ref().map(|s| s.as_str()),
               Some("Example subtitle"));
    assert_eq!(itunes.summary.as_ref().map(|s| s.as_str()),
               Some("Example summary"));
    assert_eq!(itunes.keywords.as_ref().map(|s| s.as_str()),
               Some("key1,key2,key3"));

    let itunes = &channel.items[0].itunes_ext.as_ref().expect("itunes extension missing");
    assert_eq!(itunes.author.as_ref().map(|s| s.as_str()),
               Some("Example author"));
    assert_eq!(itunes.block.as_ref().map(|s| s.as_str()), Some("yes"));
    assert_eq!(itunes.image.as_ref().map(|s| s.as_str()),
               Some("http://example.com/image.jpg"));
    assert_eq!(itunes.duration.as_ref().map(|s| s.as_str()),
               Some("01:22:33"));
    assert_eq!(itunes.explicit.as_ref().map(|s| s.as_str()), Some("yes"));
    assert_eq!(itunes.closed_captioned.as_ref().map(|s| s.as_str()),
               Some("no"));
    assert_eq!(itunes.order.as_ref().map(|s| s.as_str()), Some("1"));
    assert_eq!(itunes.subtitle.as_ref().map(|s| s.as_str()),
               Some("Example subtitle"));
    assert_eq!(itunes.summary.as_ref().map(|s| s.as_str()),
               Some("Example summary"));
    assert_eq!(itunes.keywords.as_ref().map(|s| s.as_str()),
               Some("key1,key2,key3"));
}

#[test]
fn test_dublincore() {
    let input = include_str!("data/dublincore.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    fn test_ext(dc: &DublinCoreExtension) {
        assert_eq!(dc.contributor.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
        Some(vec!["Contributor 1", "Contributor 2"]));
        assert_eq!(dc.coverage.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
        Some(vec!["Example coverage"]));
        assert_eq!(dc.creator.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
        Some(vec!["Example creator"]));
        assert_eq!(dc.date.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
        Some(vec!["2016-08-27"]));
        assert_eq!(dc.description.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
        Some(vec!["Example description"]));
        assert_eq!(dc.format.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
        Some(vec!["text/plain"]));
        assert_eq!(dc.identifier.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
        Some(vec!["Example identifier"]));
        assert_eq!(dc.language.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
        Some(vec!["en-US"]));
        assert_eq!(dc.publisher.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
        Some(vec!["Example publisher"]));
        assert_eq!(dc.relation.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
        Some(vec!["Example relation"]));
        assert_eq!(dc.rights.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
        Some(vec!["Example company"]));
        assert_eq!(dc.source.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
        Some(vec!["Example source"]));
        assert_eq!(dc.subject.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
        Some(vec!["Example subject"]));
        assert_eq!(dc.title.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
        Some(vec!["Example title"]));
        assert_eq!(dc.resource_type.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
        Some(vec!["Example type"]));
    }
    
    test_ext(&channel.dublin_core_ext.expect("dc extension missing"));
    test_ext(&channel.items[0].dublin_core_ext.as_ref().expect("ds extension missing"));
}
