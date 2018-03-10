extern crate rss;

use std::collections::HashMap;

use rss::Channel;
use rss::extension::Extension;
use rss::extension::dublincore::DublinCoreExtension;

fn get_extension_values<'a>(
    map: &'a HashMap<String, Vec<Extension>>,
    key: &str,
) -> Option<Vec<&'a str>> {
    map.get(key)
        .map(|v| v.iter().filter_map(|ext| ext.value()).collect::<Vec<_>>())
}

#[test]
fn read_rss090() {
    let input = include_str!("data/rss090.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    assert_eq!(channel.title(), "Mozilla Dot Org");
    assert_eq!(channel.link(), "http://www.mozilla.org");
    assert_eq!(
        channel.description(),
        "the Mozilla Organization\n      web site"
    );

    let image = channel.image().unwrap();
    assert_eq!(image.title(), "Mozilla");
    assert_eq!(image.url(), "http://www.mozilla.org/images/moz.gif");
    assert_eq!(image.link(), "http://www.mozilla.org");

    assert_eq!(channel.items().len(), 5);

    let item = channel.items().get(0).unwrap();
    assert_eq!(item.title(), Some("New Status Updates"));
    assert_eq!(item.link(), Some("http://www.mozilla.org/status/"));
}

#[test]
fn read_rss091() {
    let input = include_str!("data/rss091.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    assert_eq!(channel.title(), "WriteTheWeb");
    assert_eq!(channel.link(), "http://writetheweb.com");
    assert_eq!(channel.description(), "News for web users that write back");
    assert_eq!(channel.language(), Some("en-us"));
    assert_eq!(
        channel.copyright(),
        Some("Copyright 2000, WriteTheWeb team.")
    );
    assert_eq!(channel.managing_editor(), Some("editor@writetheweb.com"));
    assert_eq!(channel.webmaster(), Some("webmaster@writetheweb.com"));

    let image = channel.image().unwrap();
    assert_eq!(image.title(), "WriteTheWeb");
    assert_eq!(
        image.url(),
        "http://writetheweb.com/images/mynetscape88.gif"
    );
    assert_eq!(image.link(), "http://writetheweb.com");
    assert_eq!(image.width(), Some("88"));
    assert_eq!(image.height(), Some("31"));
    assert_eq!(
        image.description(),
        Some("News for web users that write back")
    );

    assert_eq!(channel.items().len(), 6);

    let item = channel.items().get(0).unwrap();
    assert_eq!(item.title(), Some("Giving the world a pluggable Gnutella"));
    assert_eq!(item.link(), Some("http://writetheweb.com/read.php?item=24"));
    assert_eq!(
        item.description(),
        Some(
            "WorldOS is a framework on which to build programs that work like Freenet or \
             Gnutella -allowing distributed applications using peer-to-peer routing.",
        )
    );
}

#[test]
fn read_rss092() {
    let input = include_str!("data/rss092.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    assert_eq!(channel.title(), "Dave Winer: Grateful Dead");
    assert_eq!(
        channel.link(),
        "http://www.scripting.com/blog/categories/gratefulDead.html"
    );
    assert_eq!(
        channel.description(),
        "A high-fidelity Grateful Dead song every day. This is where we're experimenting \
         with enclosures on RSS news items that download when you're not using your \
         computer. If it works (it will) it will be the end of the Click-And-Wait \
         multimedia experience on the Internet."
    );
    assert_eq!(
        channel.last_build_date(),
        Some("Fri, 13 Apr 2001 19:23:02 GMT")
    );
    assert_eq!(channel.docs(), Some("http://backend.userland.com/rss092"));
    assert_eq!(
        channel.managing_editor(),
        Some("dave@userland.com (Dave Winer)")
    );
    assert_eq!(channel.webmaster(), Some("dave@userland.com (Dave Winer)"));

    let cloud = channel.cloud().unwrap();
    assert_eq!(cloud.domain(), "data.ourfavoritesongs.com");
    assert_eq!(cloud.port(), "80");
    assert_eq!(cloud.path(), "/RPC2");
    assert_eq!(
        cloud.register_procedure(),
        "ourFavoriteSongs.rssPleaseNotify"
    );
    assert_eq!(cloud.protocol(), "xml-rpc");

    assert_eq!(channel.items().len(), 22);

    let item = channel.items().get(0).unwrap();
    assert_eq!(
        item.description(),
        Some(
            "It's been a few days since I added a song to the Grateful Dead channel. Now \
             that there are all these new Radio users, many of whom are tuned into this \
             channel (it's #16 on the hotlist of upstreaming Radio users, there's no way \
             of knowing how many non-upstreaming users are subscribing, have to \
             do something about this..). Anyway, tonight's song is a live \
             version of Weather Report Suite from Dick's Picks Volume 7. It's wistful \
             music. Of course a beautiful song, oft-quoted here on Scripting News. <i>A \
             little change, the wind and rain.</i>",
        )
    );

    let enclosure = item.enclosure().unwrap();
    assert_eq!(
        enclosure.url(),
        "http://www.scripting.com/mp3s/weatherReportDicksPicsVol7.mp3"
    );
    assert_eq!(enclosure.length(), "6182912");
    assert_eq!(enclosure.mime_type(), "audio/mpeg");
}

#[test]
fn read_rss1() {
    let input = include_str!("data/rss1.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    assert_eq!(channel.title(), "XML.com");
    assert_eq!(channel.link(), "http://xml.com/pub");
    assert_eq!(
        channel.description(),
        "XML.com features a rich mix of information and services \n      \
         for the XML community."
    );

    let image = channel.image().unwrap();
    assert_eq!(image.title(), "XML.com");
    assert_eq!(image.url(), "http://xml.com/universal/images/xml_tiny.gif");
    assert_eq!(image.link(), "http://www.xml.com");

    let text_input = channel.text_input().unwrap();
    assert_eq!(text_input.title(), "Search XML.com");
    assert_eq!(text_input.description(), "Search XML.com's XML collection");
    assert_eq!(text_input.name(), "s");
    assert_eq!(text_input.link(), "http://search.xml.com");

    assert_eq!(channel.items().len(), 2);

    let item = channel.items().get(0).unwrap();
    assert_eq!(item.title(), Some("Processing Inclusions with XSLT"));
    assert_eq!(
        item.link(),
        Some("http://xml.com/pub/2000/08/09/xslt/xslt.html")
    );
    assert_eq!(
        item.description(),
        Some(
            "Processing document inclusions with general XML tools can be \n     \
             problematic. This article proposes a way of preserving inclusion \n     \
             information through SAX-based processing.",
        )
    );
}

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
    assert_eq!(
        channel.last_build_date(),
        Some("Sat, 27 Aug 2016 09:00:00 GMT")
    );
    assert_eq!(channel.generator(), Some("Generator"));
    assert_eq!(
        channel.docs(),
        Some("http://blogs.law.harvard.edu/tech/rss")
    );
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
    assert_eq!(
        channel.items().get(0).unwrap().link(),
        Some("http://example.com/")
    );
    assert_eq!(
        channel.items().get(0).unwrap().description(),
        Some("Description")
    );
    assert_eq!(
        channel.items().get(0).unwrap().author(),
        Some("author@example.com")
    );
    assert_eq!(channel.items().get(0).unwrap().comments(), Some("Comments"));
    assert_eq!(
        channel.items().get(0).unwrap().pub_date(),
        Some("Sat, 27 Aug 2016 00:00:00 GMT")
    );
}

#[test]
fn read_content() {
    let input = include_str!("data/content.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    assert_eq!(
        channel.items().get(0).unwrap().content(),
        Some("An example <a href=\"http://example.com/\">link</a>.")
    );
}

#[test]
fn read_source() {
    let input = include_str!("data/source.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    assert_eq!(
        channel
            .items()
            .get(0,)
            .unwrap()
            .source()
            .as_ref()
            .map(|v| v.url(),),
        Some("http://example.com/feed/")
    );
    assert_eq!(
        channel
            .items()
            .get(0,)
            .unwrap()
            .source()
            .as_ref()
            .and_then(|v| v.title(),),
        Some("Feed")
    );
}

#[test]
fn read_guid() {
    let input = include_str!("data/guid.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    assert_eq!(
        channel
            .items()
            .get(0)
            .unwrap()
            .guid()
            .as_ref()
            .map(|v| v.is_permalink()),
        Some(false)
    );
    assert_eq!(
        channel
            .items()
            .get(0,)
            .unwrap()
            .guid()
            .as_ref()
            .map(|v| v.value(),),
        Some("abc")
    );

    assert_eq!(
        channel
            .items()
            .get(1)
            .unwrap()
            .guid()
            .as_ref()
            .map(|v| v.is_permalink()),
        Some(true)
    );
    assert_eq!(
        channel
            .items()
            .get(1,)
            .unwrap()
            .guid()
            .as_ref()
            .map(|v| v.value(),),
        Some("def")
    );
}

#[test]
fn read_enclosure() {
    let input = include_str!("data/enclosure.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    assert_eq!(
        channel
            .items()
            .get(0,)
            .unwrap()
            .enclosure()
            .as_ref()
            .map(|v| v.url(),),
        Some("http://example.com/media.mp3")
    );
    assert_eq!(
        channel
            .items()
            .get(0,)
            .unwrap()
            .enclosure()
            .as_ref()
            .map(|v| v.length(),),
        Some("4992349")
    );
    assert_eq!(
        channel
            .items()
            .get(0,)
            .unwrap()
            .enclosure()
            .as_ref()
            .map(|v| v.mime_type(),),
        Some("audio/mpeg")
    );
}

#[test]
fn read_category() {
    let input = include_str!("data/category.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    assert_eq!(channel.categories().get(0).unwrap().domain(), None);
    assert_eq!(channel.categories().get(0).unwrap().name(), "Category 1");

    assert_eq!(
        channel.categories().get(1).unwrap().domain(),
        Some("http://example.com/")
    );
    assert_eq!(channel.categories().get(1).unwrap().name(), "Category 2");

    assert_eq!(
        channel
            .items()
            .get(0)
            .unwrap()
            .categories()
            .get(0)
            .unwrap()
            .domain(),
        None
    );
    assert_eq!(
        channel
            .items()
            .get(0)
            .unwrap()
            .categories()
            .get(0)
            .unwrap()
            .name(),
        "Category 1"
    );

    assert_eq!(
        channel
            .items()
            .get(0)
            .unwrap()
            .categories()
            .get(1)
            .unwrap()
            .domain(),
        Some("http://example.com/")
    );
    assert_eq!(
        channel
            .items()
            .get(0)
            .unwrap()
            .categories()
            .get(1)
            .unwrap()
            .name(),
        "Category 2"
    );
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

    assert_eq!(
        channel.namespaces().get("ext").unwrap(),
        "http://example.com/"
    );
    assert_eq!(channel.namespaces().len(), 1);

    assert_eq!(
        get_extension_values(
            channel
                .items()
                .get(0)
                .unwrap()
                .extensions()
                .get("ext")
                .unwrap(),
            "creator",
        ),
        Some(vec!["Creator Name"])
    );
    assert_eq!(
        get_extension_values(
            channel
                .items()
                .get(0)
                .unwrap()
                .extensions()
                .get("ext")
                .unwrap(),
            "contributor",
        ),
        Some(vec!["Contributor 1", "Contributor 2"])
    );
    assert_eq!(
        channel
            .items()
            .get(0)
            .unwrap()
            .extensions()
            .get("ext")
            .unwrap()
            .get("parent")
            .map(|v| v.iter()
                .find(|v| v.children().contains_key("child"))
                .expect("failed to find child elements")
                .children()
                .get("child")
                .unwrap()
                .iter()
                .map(|v| v.value())
                .collect::<Vec<_>>()),
        Some(vec![Some("Child 1"), Some("Child 2")])
    );
}

#[test]
fn read_itunes() {
    let input = include_str!("data/itunes.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    assert_eq!(channel.itunes_ext().unwrap().author(), Some("Author"));
    assert_eq!(channel.itunes_ext().unwrap().block(), Some("yes"));
    assert_eq!(channel.itunes_ext().unwrap().categories().len(), 2);

    assert_eq!(
        channel
            .itunes_ext()
            .unwrap()
            .categories()
            .get(0)
            .unwrap()
            .text(),
        "Category 1"
    );
    assert_eq!(
        channel
            .itunes_ext()
            .unwrap()
            .categories()
            .get(0)
            .unwrap()
            .subcategory()
            .as_ref()
            .map(|v| v.text()),
        Some("Subcategory")
    );

    assert_eq!(
        channel
            .itunes_ext()
            .unwrap()
            .categories()
            .get(1)
            .unwrap()
            .text(),
        "Category 2"
    );
    assert_eq!(
        channel
            .itunes_ext()
            .unwrap()
            .categories()
            .get(1)
            .unwrap()
            .subcategory(),
        None
    );

    assert_eq!(
        channel.itunes_ext().unwrap().image(),
        Some("http://example.com/image.jpg")
    );
    assert_eq!(channel.itunes_ext().unwrap().explicit(), Some("no"));
    assert_eq!(channel.itunes_ext().unwrap().complete(), Some("yes"));
    assert_eq!(
        channel.itunes_ext().unwrap().new_feed_url(),
        Some("http://example.com/feed/")
    );
    assert_eq!(
        channel
            .itunes_ext()
            .unwrap()
            .owner()
            .as_ref()
            .and_then(|v| v.name(),),
        Some("Name")
    );
    assert_eq!(
        channel
            .itunes_ext()
            .unwrap()
            .owner()
            .as_ref()
            .and_then(|v| v.email(),),
        Some("example@example.com")
    );
    assert_eq!(channel.itunes_ext().unwrap().subtitle(), Some("Subtitle"));
    assert_eq!(channel.itunes_ext().unwrap().summary(), Some("Summary"));
    assert_eq!(
        channel.itunes_ext().unwrap().keywords(),
        Some("key1,key2,key3")
    );
    assert_eq!(
        channel
            .items()
            .get(0)
            .unwrap()
            .itunes_ext()
            .unwrap()
            .author(),
        Some("Author")
    );
    assert_eq!(
        channel
            .items()
            .get(0)
            .unwrap()
            .itunes_ext()
            .unwrap()
            .block(),
        Some("yes")
    );
    assert_eq!(
        channel
            .items()
            .get(0)
            .unwrap()
            .itunes_ext()
            .unwrap()
            .image(),
        Some("http://example.com/image.jpg")
    );
    assert_eq!(
        channel
            .items()
            .get(0)
            .unwrap()
            .itunes_ext()
            .unwrap()
            .duration(),
        Some("01:22:33")
    );
    assert_eq!(
        channel
            .items()
            .get(0)
            .unwrap()
            .itunes_ext()
            .unwrap()
            .explicit(),
        Some("yes")
    );
    assert_eq!(
        channel
            .items()
            .get(0)
            .unwrap()
            .itunes_ext()
            .unwrap()
            .closed_captioned(),
        Some("no")
    );
    assert_eq!(
        channel
            .items()
            .get(0)
            .unwrap()
            .itunes_ext()
            .unwrap()
            .order(),
        Some("1")
    );
    assert_eq!(
        channel
            .items()
            .get(0)
            .unwrap()
            .itunes_ext()
            .unwrap()
            .subtitle(),
        Some("Subtitle")
    );
    assert_eq!(
        channel
            .items()
            .get(0)
            .unwrap()
            .itunes_ext()
            .unwrap()
            .summary(),
        Some("Summary")
    );
    assert_eq!(
        channel
            .items()
            .get(0)
            .unwrap()
            .itunes_ext()
            .unwrap()
            .keywords(),
        Some("key1,key2,key3")
    );
}

#[test]
fn read_dublincore() {
    let input = include_str!("data/dublincore.xml");
    let channel = input.parse::<Channel>().expect("failed to parse xml");

    fn test_ext(dc: &DublinCoreExtension) {
        assert_eq!(
            dc.contributors()
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>(),
            vec!["Contributor 1", "Contributor 2"]
        );
        assert_eq!(
            dc.coverages()
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>(),
            vec!["Coverage"]
        );
        assert_eq!(
            dc.creators().iter().map(|s| s.as_str()).collect::<Vec<_>>(),
            vec!["Creator"]
        );
        assert_eq!(
            dc.dates().iter().map(|s| s.as_str()).collect::<Vec<_>>(),
            vec!["2016-08-27"]
        );
        assert_eq!(
            dc.descriptions()
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>(),
            vec!["Description"]
        );
        assert_eq!(
            dc.formats().iter().map(|s| s.as_str()).collect::<Vec<_>>(),
            vec!["text/plain"]
        );
        assert_eq!(
            dc.identifiers()
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>(),
            vec!["Identifier"]
        );
        assert_eq!(
            dc.languages()
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>(),
            vec!["en-US"]
        );
        assert_eq!(
            dc.publishers()
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>(),
            vec!["Publisher"]
        );
        assert_eq!(
            dc.relations()
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>(),
            vec!["Relation"]
        );
        assert_eq!(
            dc.rights().iter().map(|s| s.as_str()).collect::<Vec<_>>(),
            vec!["Company"]
        );
        assert_eq!(
            dc.sources().iter().map(|s| s.as_str()).collect::<Vec<_>>(),
            vec!["Source"]
        );
        assert_eq!(
            dc.subjects().iter().map(|s| s.as_str()).collect::<Vec<_>>(),
            vec!["Subject"]
        );
        assert_eq!(
            dc.titles().iter().map(|s| s.as_str()).collect::<Vec<_>>(),
            vec!["Title"]
        );
        assert_eq!(
            dc.types().iter().map(|s| s.as_str()).collect::<Vec<_>>(),
            vec!["Type"]
        );
    }

    test_ext(
        channel
            .dublin_core_ext()
            .as_ref()
            .expect("dc extension missing"),
    );
    test_ext(
        channel
            .items()
            .get(0)
            .unwrap()
            .dublin_core_ext()
            .as_ref()
            .expect("ds extension missing"),
    );
}

#[test]
fn read_escaped() {
    let input = r#"
        <rss version="2.0">
            <channel>
                <title>My &lt;feed&gt;</title>
            </channel>
        </rss>
    "#;
    let channel = input.parse::<Channel>().unwrap();
    assert_eq!("My <feed>", channel.title());
    let output = channel.to_string();
    let parsed_channel = output.parse::<Channel>().unwrap();
    assert_eq!(channel, parsed_channel);
}
