use xml::Element;

use ::ViaXml;


/// Category
///
/// [RSS 2.0 Specification § Category]
/// (http://cyber.law.harvard.edu/rss/rss.html#ltcategorygtSubelementOfLtitemgt)
#[derive(Default)]
pub struct Category {
    pub domain: Option<String>,
    pub value: String,
}

impl ViaXml for Category {
    fn to_xml(&self) -> Element {
        let mut category = match self.domain {
            Some(ref d) => Element::new("category".to_string(), None, vec![("domain".to_string(), None, d.clone())]),
            None => Element::new("category".to_string(), None, vec![]),
        };
        category.text(self.value.clone());
        category
    }

    fn from_xml(elem: Element) -> Result<Self, &'static str> {
        let domain = elem.get_attribute("domain", None).map(|s| s.to_string());
        let value = elem.content_str();

        Ok(Category {
            domain: domain,
            value: value,
        })
    }
}
