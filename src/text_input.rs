use xml::Element;

use ::{ElementUtils, ViaXml};


/// [RSS 2.0 Specification § `<textInput>` sub-element of `<channel>`]
/// (http://cyber.law.harvard.edu/rss/rss.html#lttextinputgtSubelementOfLtchannelgt)
pub struct TextInput {
    pub title: String,
    pub description: String,
    pub name: String,
    pub link: String,
}

impl ViaXml for TextInput {
    fn to_xml(&self) -> Element {
        let mut elem = Element::new("textInput".to_string(), None, vec![]);
        elem.tag_with_text("title", &self.title);
        elem.tag_with_text("description", &self.description);
        elem.tag_with_text("name", &self.name);
        elem.tag_with_text("link", &self.link);
        elem
    }

    fn from_xml(elem: Element) -> Result<Self, &'static str> {
        let title = match elem.get_child("title", None) {
            Some(elem) => elem.content_str(),
            None => return Err("<textInput> is missing required <title> element"),
        };

        let description = match elem.get_child("description", None) {
            Some(elem) => elem.content_str(),
            None => return Err("<textInput> is missing required <description> element"),
        };

        let name = match elem.get_child("name", None) {
            Some(elem) => elem.content_str(),
            None => return Err("<textInput> is missing required <name> element"),
        };

        let link = match elem.get_child("link", None) {
            Some(elem) => elem.content_str(),
            None => return Err("<textInput> is missing required <link> element"),
        };

        Ok(TextInput {
            title: title,
            description: description,
            name: name,
            link: link,
        })
    }
}
