use xml::Element;

use ::{ReadError, ViaXml};

/// [RSS 2.0 Specification § `<guid>` sub-element of `<item>`]
/// (http://cyber.law.harvard.edu/rss/rss.html#ltguidgtSubelementOfLtitemgt)
#[derive(Debug, Clone)]
pub struct Guid {
    pub is_perma_link: bool,
    pub value: String,
}

impl ViaXml for Guid {
    fn to_xml(&self) -> Element {
        let mut guid = if !self.is_perma_link {
            Element::new("guid".to_owned(), None, vec![("isPermaLink".to_owned(), None, "false".to_owned())])
        } else {
            Element::new("guid".to_owned(), None, vec![])
        };

        guid.text(self.value.clone());
        guid
    }

    fn from_xml(elem: Element) -> Result<Self, ReadError> {
        let is_perma_link = match elem.get_attribute("isPermaLink", None) {
            Some("false") => false,
            _ => true
        };

        let value = elem.content_str();

        Ok(Guid {
            is_perma_link: is_perma_link,
            value: value,
        })
    }
}
