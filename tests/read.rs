extern crate rss;

use rss::Channel;
use rss::extension::dublincore::DublinCoreExtension;
use rss::extension::get_extension_values;

#[test]
fn test_channel() {
    let input = include_str!("data/channel.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    assert_eq!(channel.title, "<Title>");
    assert_eq!(channel.link, "http://example.com/");
    assert_eq!(channel.description, "Description");
    assert_eq!(channel.language.as_ref().map(|s| s.as_str()), Some("en-US"));
    assert_eq!(channel.managing_editor.as_ref().map(|s| s.as_str()),
               Some("editor@example.com"));
    assert_eq!(channel.webmaster.as_ref().map(|s| s.as_str()),
               Some("webmaster@example.com"));
    assert_eq!(channel.pub_date.as_ref().map(|s| s.as_str()),
               Some("Sat, 27 Aug 2016 00:00:00 GMT"));
    assert_eq!(channel.last_build_date.as_ref().map(|s| s.as_str()),
               Some("Sat, 27 Aug 2016 09:00:00 GMT"));
    assert_eq!(channel.generator.as_ref().map(|s| s.as_str()),
               Some("Generator"));
    assert_eq!(channel.docs.as_ref().map(|s| s.as_str()),
               Some("http://blogs.law.harvard.edu/tech/rss"));
    assert_eq!(channel.ttl.as_ref().map(|s| s.as_str()), Some("60"));
    assert_eq!(channel.skip_hours[0].as_str(), "6");
    assert_eq!(channel.skip_hours[1].as_str(), "8");
    assert_eq!(channel.skip_days[0].as_str(), "Tuesday");
    assert_eq!(channel.skip_days[1].as_str(), "Thursday");
}

#[test]
fn test_item() {
    let input = include_str!("data/item.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");
    let item = &channel.items[0];

    assert_eq!(item.title.as_ref().map(|s| s.as_str()), Some("<Title>"));
    assert_eq!(item.link.as_ref().map(|s| s.as_str()), Some("http://example.com/"));
    assert_eq!(item.description.as_ref().map(|s| s.as_str()), Some("Description"));
    assert_eq!(item.author.as_ref().map(|s| s.as_str()), Some("author@example.com"));
    assert_eq!(item.comments.as_ref().map(|s| s.as_str()), Some("Comments"));
    assert_eq!(item.pub_date.as_ref().map(|s| s.as_str()),
               Some("Sat, 27 Aug 2016 00:00:00 GMT"));
}

#[test]
fn test_content() {
    let input = include_str!("data/content.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    assert_eq!(channel.items[0].content.as_ref().map(|s| s.as_str()),
               Some("An example <a href=\"http://example.com/\">link</a>."));
}

#[test]
fn test_source() {
    let input = include_str!("data/source.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    assert_eq!(channel.items[0].source.as_ref().map(|v| v.url.as_str()),
               Some("http://example.com/feed/"));
    assert_eq!(channel.items[0].source.as_ref().and_then(|v| v.title.as_ref().map(|s| s.as_str())),
               Some("Feed"));
}

#[test]
fn test_guid() {
    let input = include_str!("data/guid.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    assert_eq!(channel.items[0].guid.as_ref().map(|v| v.is_permalink),
               Some(false));
    assert_eq!(channel.items[0].guid.as_ref().map(|v| v.value.as_str()),
               Some("abc"));

    assert_eq!(channel.items[1].guid.as_ref().map(|v| v.is_permalink),
               Some(true));
    assert_eq!(channel.items[1].guid.as_ref().map(|v| v.value.as_str()),
               Some("def"));
}

#[test]
fn test_enclosure() {
    let input = include_str!("data/enclosure.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    assert_eq!(channel.items[0].enclosure.as_ref().map(|v| v.url.as_str()),
               Some("http://example.com/media.mp3"));
    assert_eq!(channel.items[0].enclosure.as_ref().map(|v| v.length.as_str()),
               Some("4992349"));
    assert_eq!(channel.items[0].enclosure.as_ref().map(|v| v.mime_type.as_str()),
               Some("audio/mpeg"));
}

#[test]
fn test_category() {
    let input = include_str!("data/category.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    assert_eq!(channel.categories[0].domain, None);
    assert_eq!(channel.categories[0].name, "Category 1");

    assert_eq!(channel.categories[1].domain.as_ref().map(|s| s.as_str()),
               Some("http://example.com/"));
    assert_eq!(channel.categories[1].name, "Category 2");

    assert_eq!(channel.items[0].categories[0].domain, None);
    assert_eq!(channel.items[0].categories[0].name, "Category 1");

    assert_eq!(channel.items[0].categories[1].domain.as_ref().map(|s| s.as_str()),
               Some("http://example.com/"));
    assert_eq!(channel.items[0].categories[1].name, "Category 2");
}

#[test]
fn test_image() {
    let input = include_str!("data/image.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");
    let image = channel.image.expect("image missing");

    assert_eq!(image.title, "Title");
    assert_eq!(image.url, "http://example.org/url");
    assert_eq!(image.link, "http://example.org/link");
    assert_eq!(image.width.as_ref().map(|s| s.as_str()), Some("100"));
    assert_eq!(image.height.as_ref().map(|s| s.as_str()), Some("200"));
    assert_eq!(image.description.as_ref().map(|s| s.as_str()), Some("Description"));
}

#[test]
fn test_mixed_content() {
    let input = include_str!("data/mixed_content.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    assert_eq!(channel.title, "Title");
}

#[test]
fn test_cloud() {
    let input = include_str!("data/cloud.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");
    let cloud = channel.cloud.expect("cloud missing");

    assert_eq!(cloud.domain, "example.com");
    assert_eq!(cloud.port, "80");
    assert_eq!(cloud.path, "/rpc");
    assert_eq!(cloud.register_procedure, "notify");
    assert_eq!(cloud.protocol, "xml-rpc");
}

#[test]
fn test_textinput() {
    let input = include_str!("data/textinput.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");
    let text_input = channel.text_input.expect("textinput missing");

    assert_eq!(text_input.title, "Title");
    assert_eq!(text_input.name, "Name");
    assert_eq!(text_input.link, "http://example.com/");
    assert_eq!(text_input.description, "Description");
}


#[test]
fn test_extension() {
    let input = include_str!("data/extension.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

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
    assert_eq!(itunes.author.as_ref().map(|s| s.as_str()), Some("Author"));
    assert_eq!(itunes.block.as_ref().map(|s| s.as_str()), Some("yes"));
    assert_eq!(itunes.categories.as_ref().map(|v| v.len()), Some(2));
   
    assert_eq!(itunes.categories.as_ref().map(|v| v[0].text.as_str()),
               Some("Category 1"));
    assert_eq!(itunes.categories
                   .as_ref()
                   .and_then(|v| v[0].subcategory.as_ref())
                   .map(|v| v.text.as_str()),
               Some("Subcategory"));
   
    assert_eq!(itunes.categories.as_ref().map(|v| v[1].text.as_str()),
    Some("Category 2"));
    assert_eq!(itunes.categories
               .as_ref()
               .and_then(|v| v[1].subcategory.as_ref()),
               None);
    
    assert_eq!(itunes.image.as_ref().map(|s| s.as_str()),
    Some("http://example.com/image.jpg"));
    assert_eq!(itunes.explicit.as_ref().map(|s| s.as_str()), Some("no"));
    assert_eq!(itunes.complete.as_ref().map(|s| s.as_str()), Some("yes"));
    assert_eq!(itunes.new_feed_url.as_ref().map(|s| s.as_str()),
               Some("http://example.com/feed/"));
    assert_eq!(itunes.owner.as_ref().and_then(|v| v.name.as_ref()).map(|s| s.as_str()),
               Some("Name"));
    assert_eq!(itunes.owner.as_ref().and_then(|v| v.email.as_ref()).map(|s| s.as_str()),
               Some("example@example.com"));
    assert_eq!(itunes.subtitle.as_ref().map(|s| s.as_str()),
               Some("Subtitle"));
    assert_eq!(itunes.summary.as_ref().map(|s| s.as_str()), Some("Summary"));
    assert_eq!(itunes.keywords.as_ref().map(|s| s.as_str()),
               Some("key1,key2,key3"));

    let itunes = &channel.items[0].itunes_ext.as_ref().expect("itunes extension missing");
    assert_eq!(itunes.author.as_ref().map(|s| s.as_str()), Some("Author"));
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
               Some("Subtitle"));
    assert_eq!(itunes.summary.as_ref().map(|s| s.as_str()), Some("Summary"));
    assert_eq!(itunes.keywords.as_ref().map(|s| s.as_str()),
               Some("key1,key2,key3"));
}

#[test]
fn test_dublincore() {
    let input = include_str!("data/dublincore.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    fn test_ext(dc: &DublinCoreExtension) {
        assert_eq!(dc.contributor
                       .as_ref()
                       .map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
                   Some(vec!["Contributor 1", "Contributor 2"]));
        assert_eq!(dc.coverage.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
                   Some(vec!["Coverage"]));
        assert_eq!(dc.creator.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
                   Some(vec!["Creator"]));
        assert_eq!(dc.date.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
                   Some(vec!["2016-08-27"]));
        assert_eq!(dc.description
                       .as_ref()
                       .map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
                   Some(vec!["Description"]));
        assert_eq!(dc.format.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
                   Some(vec!["text/plain"]));
        assert_eq!(dc.identifier
                       .as_ref()
                       .map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
                   Some(vec!["Identifier"]));
        assert_eq!(dc.language.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
                   Some(vec!["en-US"]));
        assert_eq!(dc.publisher.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
                   Some(vec!["Publisher"]));
        assert_eq!(dc.relation.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
                   Some(vec!["Relation"]));
        assert_eq!(dc.rights.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
                   Some(vec!["Company"]));
        assert_eq!(dc.source.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
                   Some(vec!["Source"]));
        assert_eq!(dc.subject.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
                   Some(vec!["Subject"]));
        assert_eq!(dc.title.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
                   Some(vec!["Title"]));
        assert_eq!(dc.resource_type
                       .as_ref()
                       .map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
                   Some(vec!["Type"]));
    }

    test_ext(&channel.dublin_core_ext.expect("dc extension missing"));
    test_ext(&channel.items[0].dublin_core_ext.as_ref().expect("ds extension missing"));
}
