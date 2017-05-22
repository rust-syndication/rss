extern crate rss;

use rss::Channel;
use rss::extension::dublincore::DublinCoreExtension;
use rss::extension::get_extension_values;

#[test]
fn read_channel() {
    let input = include_str!("data/channel.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    assert_eq!(channel.title(), "Title");
    assert_eq!(channel.link(), "http://example.com/");
    assert_eq!(channel.description(), "Description");
    assert_eq!(channel.language(), Some("en-US"));
    assert_eq!(channel.managing_editor(), Some("editor@example.com"));
    assert_eq!(channel.webmaster(), Some("webmaster@example.com"));
    assert_eq!(channel.pub_date(), Some("Sat, 27 Aug 2016 00:00:00 GMT"));
    assert_eq!(channel.last_build_date(),
               Some("Sat, 27 Aug 2016 09:00:00 GMT"));
    assert_eq!(channel.generator(), Some("Generator"));
    assert_eq!(channel.docs(),
               Some("http://blogs.law.harvard.edu/tech/rss"));
    assert_eq!(channel.ttl(), Some("60"));
    assert_eq!(channel.skip_hours().get(0).unwrap().as_str(), "6");
    assert_eq!(channel.skip_hours().get(1).unwrap().as_str(), "8");
    assert_eq!(channel.skip_days().get(0).unwrap().as_str(), "Tuesday");
    assert_eq!(channel.skip_days().get(1).unwrap().as_str(), "Thursday");
}

#[test]
fn read_item() {
    let input = include_str!("data/item.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    assert_eq!(channel.items().get(0).unwrap().title(), Some("Title"));
    assert_eq!(channel.items().get(0).unwrap().link(),
               Some("http://example.com/"));
    assert_eq!(channel.items().get(0).unwrap().description(),
               Some("Description"));
    assert_eq!(channel.items().get(0).unwrap().author(),
               Some("author@example.com"));
    assert_eq!(channel.items().get(0).unwrap().comments(), Some("Comments"));
    assert_eq!(channel.items().get(0).unwrap().pub_date(),
               Some("Sat, 27 Aug 2016 00:00:00 GMT"));
}

#[test]
fn read_content() {
    let input = include_str!("data/content.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    assert_eq!(channel.items().get(0).unwrap().content(),
               Some("An example <a href=\"http://example.com/\">link</a>."));
}

#[test]
fn read_source() {
    let input = include_str!("data/source.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    assert_eq!(channel
                   .items()
                   .get(0)
                   .unwrap()
                   .source()
                   .as_ref()
                   .map(|v| v.url()),
               Some("http://example.com/feed/"));
    assert_eq!(channel
                   .items()
                   .get(0)
                   .unwrap()
                   .source()
                   .as_ref()
                   .and_then(|v| v.title()),
               Some("Feed"));
}

#[test]
fn read_guid() {
    let input = include_str!("data/guid.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    assert_eq!(channel
                   .items()
                   .get(0)
                   .unwrap()
                   .guid()
                   .as_ref()
                   .map(|v| v.is_permalink()),
               Some(false));
    assert_eq!(channel
                   .items()
                   .get(0)
                   .unwrap()
                   .guid()
                   .as_ref()
                   .map(|v| v.value()),
               Some("abc"));

    assert_eq!(channel
                   .items()
                   .get(1)
                   .unwrap()
                   .guid()
                   .as_ref()
                   .map(|v| v.is_permalink()),
               Some(true));
    assert_eq!(channel
                   .items()
                   .get(1)
                   .unwrap()
                   .guid()
                   .as_ref()
                   .map(|v| v.value()),
               Some("def"));
}

#[test]
fn read_enclosure() {
    let input = include_str!("data/enclosure.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    assert_eq!(channel
                   .items()
                   .get(0)
                   .unwrap()
                   .enclosure()
                   .as_ref()
                   .map(|v| v.url()),
               Some("http://example.com/media.mp3"));
    assert_eq!(channel
                   .items()
                   .get(0)
                   .unwrap()
                   .enclosure()
                   .as_ref()
                   .map(|v| v.length()),
               Some("4992349"));
    assert_eq!(channel
                   .items()
                   .get(0)
                   .unwrap()
                   .enclosure()
                   .as_ref()
                   .map(|v| v.mime_type()),
               Some("audio/mpeg"));
}

#[test]
fn read_category() {
    let input = include_str!("data/category.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    assert_eq!(channel.categories().get(0).unwrap().domain(), None);
    assert_eq!(channel.categories().get(0).unwrap().name(), "Category 1");

    assert_eq!(channel.categories().get(1).unwrap().domain(),
               Some("http://example.com/"));
    assert_eq!(channel.categories().get(1).unwrap().name(), "Category 2");

    assert_eq!(channel
                   .items()
                   .get(0)
                   .unwrap()
                   .categories()
                   .get(0)
                   .unwrap()
                   .domain(),
               None);
    assert_eq!(channel
                   .items()
                   .get(0)
                   .unwrap()
                   .categories()
                   .get(0)
                   .unwrap()
                   .name(),
               "Category 1");

    assert_eq!(channel
                   .items()
                   .get(0)
                   .unwrap()
                   .categories()
                   .get(1)
                   .unwrap()
                   .domain(),
               Some("http://example.com/"));
    assert_eq!(channel
                   .items()
                   .get(0)
                   .unwrap()
                   .categories()
                   .get(1)
                   .unwrap()
                   .name(),
               "Category 2");
}

#[test]
fn read_image() {
    let input = include_str!("data/image.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    assert_eq!(channel.image().unwrap().title(), "Title");
    assert_eq!(channel.image().unwrap().url(), "http://example.org/url");
    assert_eq!(channel.image().unwrap().link(), "http://example.org/link");
    assert_eq!(channel.image().unwrap().width(), Some("100"));
    assert_eq!(channel.image().unwrap().height(), Some("200"));
    assert_eq!(channel.image().unwrap().description(), Some("Description"));
}

#[test]
fn read_mixed_content() {
    let input = include_str!("data/mixed_content.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    assert_eq!(channel.title(), "Title");
}

#[test]
fn read_cloud() {
    let input = include_str!("data/cloud.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");
    let cloud = channel.cloud().expect("cloud missing");

    assert_eq!(cloud.domain(), "example.com");
    assert_eq!(cloud.port(), "80");
    assert_eq!(cloud.path(), "/rpc");
    assert_eq!(cloud.register_procedure(), "notify");
    assert_eq!(cloud.protocol(), "xml-rpc");
}

#[test]
fn read_textinput() {
    let input = include_str!("data/textinput.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");
    let text_input = channel.text_input().expect("textinput missing");

    assert_eq!(text_input.title(), "Title");
    assert_eq!(text_input.name(), "Name");
    assert_eq!(text_input.link(), "http://example.com/");
    assert_eq!(text_input.description(), "Description");
}

#[test]
fn read_extension() {
    let input = include_str!("data/extension.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    assert_eq!(channel.namespaces().get("ext").unwrap(),
               "http://example.com/");
    assert_eq!(channel.namespaces().len(), 1);

    assert_eq!(get_extension_values(&channel
                                         .items()
                                         .get(0)
                                         .unwrap()
                                         .extensions()
                                         .get("ext")
                                         .unwrap(),
                                    "creator"),
               Some(vec!["Creator Name"]));
    assert_eq!(get_extension_values(&channel
                                         .items()
                                         .get(0)
                                         .unwrap()
                                         .extensions()
                                         .get("ext")
                                         .unwrap(),
                                    "contributor"),
               Some(vec!["Contributor 1", "Contributor 2"]));
    assert_eq!(channel
                   .items()
                   .get(0)
                   .unwrap()
                   .extensions()
                   .get("ext")
                   .unwrap()
                   .get("parent")
                   .map(|v| {
        v.iter()
            .find(|v| v.children().contains_key("child"))
            .expect("failed to find child elements")
            .children()
            .get("child")
            .unwrap()
            .iter()
            .map(|v| v.value())
            .collect::<Vec<_>>()
    }),
               Some(vec![Some("Child 1"), Some("Child 2")]));
}


#[test]
fn read_itunes() {
    let input = include_str!("data/itunes.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    assert_eq!(channel.itunes_ext().unwrap().author(), Some("Author"));
    assert_eq!(channel.itunes_ext().unwrap().block(), Some("yes"));
    assert_eq!(channel.itunes_ext().unwrap().categories().len(), 2);

    assert_eq!(channel
                   .itunes_ext()
                   .unwrap()
                   .categories()
                   .get(0)
                   .unwrap()
                   .text(),
               "Category 1");
    assert_eq!(channel
                   .itunes_ext()
                   .unwrap()
                   .categories()
                   .get(0)
                   .unwrap()
                   .subcategory()
                   .as_ref()
                   .map(|v| v.text()),
               Some("Subcategory"));

    assert_eq!(channel
                   .itunes_ext()
                   .unwrap()
                   .categories()
                   .get(1)
                   .unwrap()
                   .text(),
               "Category 2");
    assert_eq!(channel
                   .itunes_ext()
                   .unwrap()
                   .categories()
                   .get(1)
                   .unwrap()
                   .subcategory(),
               None);

    assert_eq!(channel.itunes_ext().unwrap().image(),
               Some("http://example.com/image.jpg"));
    assert_eq!(channel.itunes_ext().unwrap().explicit(), Some("no"));
    assert_eq!(channel.itunes_ext().unwrap().complete(), Some("yes"));
    assert_eq!(channel.itunes_ext().unwrap().new_feed_url(),
               Some("http://example.com/feed/"));
    assert_eq!(channel
                   .itunes_ext()
                   .unwrap()
                   .owner()
                   .as_ref()
                   .and_then(|v| v.name()),
               Some("Name"));
    assert_eq!(channel
                   .itunes_ext()
                   .unwrap()
                   .owner()
                   .as_ref()
                   .and_then(|v| v.email()),
               Some("example@example.com"));
    assert_eq!(channel.itunes_ext().unwrap().subtitle(), Some("Subtitle"));
    assert_eq!(channel.itunes_ext().unwrap().summary(), Some("Summary"));
    assert_eq!(channel.itunes_ext().unwrap().keywords(),
               Some("key1,key2,key3"));
    assert_eq!(channel
                   .items()
                   .get(0)
                   .unwrap()
                   .itunes_ext()
                   .unwrap()
                   .author(),
               Some("Author"));
    assert_eq!(channel
                   .items()
                   .get(0)
                   .unwrap()
                   .itunes_ext()
                   .unwrap()
                   .block(),
               Some("yes"));
    assert_eq!(channel
                   .items()
                   .get(0)
                   .unwrap()
                   .itunes_ext()
                   .unwrap()
                   .image(),
               Some("http://example.com/image.jpg"));
    assert_eq!(channel
                   .items()
                   .get(0)
                   .unwrap()
                   .itunes_ext()
                   .unwrap()
                   .duration(),
               Some("01:22:33"));
    assert_eq!(channel
                   .items()
                   .get(0)
                   .unwrap()
                   .itunes_ext()
                   .unwrap()
                   .explicit(),
               Some("yes"));
    assert_eq!(channel
                   .items()
                   .get(0)
                   .unwrap()
                   .itunes_ext()
                   .unwrap()
                   .closed_captioned(),
               Some("no"));
    assert_eq!(channel
                   .items()
                   .get(0)
                   .unwrap()
                   .itunes_ext()
                   .unwrap()
                   .order(),
               Some("1"));
    assert_eq!(channel
                   .items()
                   .get(0)
                   .unwrap()
                   .itunes_ext()
                   .unwrap()
                   .subtitle(),
               Some("Subtitle"));
    assert_eq!(channel
                   .items()
                   .get(0)
                   .unwrap()
                   .itunes_ext()
                   .unwrap()
                   .summary(),
               Some("Summary"));
    assert_eq!(channel
                   .items()
                   .get(0)
                   .unwrap()
                   .itunes_ext()
                   .unwrap()
                   .keywords(),
               Some("key1,key2,key3"));
}

#[test]
fn read_dublincore() {
    let input = include_str!("data/dublincore.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    fn test_ext(dc: &DublinCoreExtension) {
        assert_eq!(dc.contributors()
                       .iter()
                       .map(|s| s.as_str())
                       .collect::<Vec<_>>(),
                   vec!["Contributor 1", "Contributor 2"]);
        assert_eq!(dc.coverages()
                       .iter()
                       .map(|s| s.as_str())
                       .collect::<Vec<_>>(),
                   vec!["Coverage"]);
        assert_eq!(dc.creators().iter().map(|s| s.as_str()).collect::<Vec<_>>(),
                   vec!["Creator"]);
        assert_eq!(dc.dates().iter().map(|s| s.as_str()).collect::<Vec<_>>(),
                   vec!["2016-08-27"]);
        assert_eq!(dc.descriptions()
                       .iter()
                       .map(|s| s.as_str())
                       .collect::<Vec<_>>(),
                   vec!["Description"]);
        assert_eq!(dc.formats().iter().map(|s| s.as_str()).collect::<Vec<_>>(),
                   vec!["text/plain"]);
        assert_eq!(dc.identifiers()
                       .iter()
                       .map(|s| s.as_str())
                       .collect::<Vec<_>>(),
                   vec!["Identifier"]);
        assert_eq!(dc.languages()
                       .iter()
                       .map(|s| s.as_str())
                       .collect::<Vec<_>>(),
                   vec!["en-US"]);
        assert_eq!(dc.publishers()
                       .iter()
                       .map(|s| s.as_str())
                       .collect::<Vec<_>>(),
                   vec!["Publisher"]);
        assert_eq!(dc.relations()
                       .iter()
                       .map(|s| s.as_str())
                       .collect::<Vec<_>>(),
                   vec!["Relation"]);
        assert_eq!(dc.rights().iter().map(|s| s.as_str()).collect::<Vec<_>>(),
                   vec!["Company"]);
        assert_eq!(dc.sources().iter().map(|s| s.as_str()).collect::<Vec<_>>(),
                   vec!["Source"]);
        assert_eq!(dc.subjects().iter().map(|s| s.as_str()).collect::<Vec<_>>(),
                   vec!["Subject"]);
        assert_eq!(dc.titles().iter().map(|s| s.as_str()).collect::<Vec<_>>(),
                   vec!["Title"]);
        assert_eq!(dc.resource_types()
                       .iter()
                       .map(|s| s.as_str())
                       .collect::<Vec<_>>(),
                   vec!["Type"]);
    }

    test_ext(&channel
                  .dublin_core_ext()
                  .as_ref()
                  .expect("dc extension missing"));
    test_ext(&channel
                  .items()
                  .get(0)
                  .unwrap()
                  .dublin_core_ext()
                  .as_ref()
                  .expect("ds extension missing"));
}
