use xml::Element;

use ::{Category, ElementUtils, Item, TextInput, ViaXml};


/// [RSS 2.0 Specification § Required channel elements]
/// (http://cyber.law.harvard.edu/rss/rss.html#requiredChannelElements)
///
/// ## Examples
///
/// ```
/// use rss::Channel;
///
/// let channel = Channel {
///     title: String::from("My Blog"),
///     link: String::from("http://myblog.com"),
///     description: String::from("My thoughts on life, the universe, and everything"),
///     items: vec![],
///     ..Default::default()
/// };
/// ```
#[derive(Default)]
pub struct Channel {
    pub title: String,
    pub link: String,
    pub description: String,
    pub items: Vec<Item>,
    pub language: Option<String>,
    pub copyright: Option<String>,
    pub managing_editor: Option<String>,
    pub web_master: Option<String>,
    pub pub_date: Option<String>,
    pub last_build_date: Option<String>,
    pub categories: Vec<Category>,
    pub generator: Option<String>,
    pub docs: Option<String>,
    // pub cloud:
    pub ttl: Option<String>,  // TODO: change this to Option<i32>?
    pub image: Option<String>,
    pub rating: Option<String>,
    pub text_input: Option<TextInput>,
    pub skip_hours: Option<String>,
    pub skip_days: Option<String>,
}

impl ViaXml for Channel {
    fn to_xml(&self) -> Element {
        let mut channel = Element::new("channel".to_string(), None, vec![]);

        channel.tag_with_text("title", &self.title);
        channel.tag_with_text("link", &self.link);
        channel.tag_with_text("description", &self.description);

        for item in &self.items {
            channel.tag(item.to_xml());
        }

        channel.tag_with_optional_text("language", &self.language);
        channel.tag_with_optional_text("copyright", &self.copyright);
        channel.tag_with_optional_text("managingEditor", &self.managing_editor);
        channel.tag_with_optional_text("webMaster", &self.web_master);
        channel.tag_with_optional_text("pubDate", &self.pub_date);
        channel.tag_with_optional_text("lastBuildDate", &self.last_build_date);
        channel.tag_with_optional_text("generator", &self.generator);
        channel.tag_with_optional_text("docs", &self.docs);
        channel.tag_with_optional_text("ttl", &self.ttl);
        channel.tag_with_optional_text("image", &self.image);
        channel.tag_with_optional_text("rating", &self.rating);

        if let Some(ref text_input) = self.text_input {
            channel.tag(text_input.to_xml());
        }

        channel.tag_with_optional_text("skipHours", &self.skip_hours);
        channel.tag_with_optional_text("skipDays", &self.skip_days);

        for category in &self.categories {
            channel.tag(category.to_xml());
        }

        channel
    }

    fn from_xml(elem: Element) -> Result<Self, &'static str> {
        let title = match elem.get_child("title", None) {
            Some(elem) => elem.content_str(),
            None => return Err("<channel> is missing required <title> element"),
        };

        let link = match elem.get_child("link", None) {
            Some(elem) => elem.content_str(),
            None => return Err("<channel> is missing required <link> element"),
        };

        let description = match elem.get_child("description", None) {
            Some(elem) => elem.content_str(),
            None => return Err("<channel> is missing required <description> element"),
        };

        let items = elem.get_children("item", None)
            .map(|e| ViaXml::from_xml(e.clone()).unwrap())
            .collect();

        let language = elem.get_child("language", None).map(Element::content_str);
        let copyright = elem.get_child("copyright", None).map(Element::content_str);
        let managing_editor = elem.get_child("managing_editor", None).map(Element::content_str);
        let web_master = elem.get_child("managing_editor", None).map(Element::content_str);
        let pub_date = elem.get_child("pub_date", None).map(Element::content_str);
        let last_build_date = elem.get_child("last_build_date", None).map(Element::content_str);

        let categories = elem.get_children("category", None)
            .map(|e| ViaXml::from_xml(e.clone()).unwrap())
            .collect();

        let generator = elem.get_child("generator", None).map(Element::content_str);
        let docs = elem.get_child("docs", None).map(Element::content_str);
        let ttl = elem.get_child("ttl", None).map(Element::content_str);
        let image = elem.get_child("image", None).map(Element::content_str);
        let rating = elem.get_child("rating", None).map(Element::content_str);

        let text_input = elem.get_child("textInput", None).map(|e| ViaXml::from_xml(e.clone()).unwrap());

        let skip_hours = elem.get_child("skip_hours", None).map(Element::content_str);
        let skip_days = elem.get_child("skip_days", None).map(Element::content_str);

        Ok(Channel {
            title: title,
            link: link,
            description: description,
            items: items,
            language: language,
            copyright: copyright,
            managing_editor: managing_editor,
            web_master: web_master,
            pub_date: pub_date,
            last_build_date: last_build_date,
            categories: categories,
            generator: generator,
            docs: docs,
            ttl: ttl,
            image: image,
            rating: rating,
            text_input: text_input,
            skip_hours: skip_hours,
            skip_days: skip_days,
        })
    }
}
