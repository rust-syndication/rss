use xml::Element;

use ::{Category, ElementUtils, ViaXml};


/// Item
///
/// [RSS 2.0 Specification § Item]
/// (http://cyber.law.harvard.edu/rss/rss.html#hrelementsOfLtitemgt)
#[derive(Default)]
pub struct Item {
    pub title: Option<String>,
    pub link: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub categories: Vec<Category>,
    pub comments: Option<String>,
    // pub enclosure
    // pub guid
    pub pub_date: Option<String>,  // add a custom String type to parse this date?
    // pub source
}


impl ViaXml for Item {
    fn to_xml(&self) -> Element {
        let mut item = Element::new("item".to_string(), None, vec![]);

        item.tag_with_optional_text("title", &self.title);
        item.tag_with_optional_text("link", &self.link);
        item.tag_with_optional_text("description", &self.description);
        item.tag_with_optional_text("author", &self.author);
        item.tag_with_optional_text("comments", &self.comments);
        item.tag_with_optional_text("pubDate", &self.pub_date);

        for category in &self.categories {
            item.tag(category.to_xml());
        }

        item
    }

    fn from_xml(elem: Element) -> Result<Self, &'static str> {
        let title = elem.get_child("title", None).map(Element::content_str);
        let link = elem.get_child("link", None).map(Element::content_str);
        let description = elem.get_child("description", None).map(Element::content_str);
        let author = elem.get_child("author", None).map(Element::content_str);
        let comments = elem.get_child("comments", None).map(Element::content_str);
        let pub_date = elem.get_child("pubDate", None).map(Element::content_str);

        let categories = elem.get_children("category", None)
            .map(|e| ViaXml::from_xml(e.clone()).unwrap())
            .collect();

        Ok(Item {
            title: title,
            link: link,
            description: description,
            categories: categories,
            author: author,
            comments: comments,
            pub_date: pub_date,
        })
    }
}
