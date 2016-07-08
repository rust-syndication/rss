// Copyright 2015 Corey Farwell
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use xml::Element;

use ::{ElementUtils, ReadError, ViaXml};


/// [RSS 2.0 Specification § `<textInput>` sub-element of `<channel>`]
/// (http://cyber.law.harvard.edu/rss/rss.html#lttextinputgtSubelementOfLtchannelgt)
#[derive(Debug, Clone)]
pub struct TextInput {
    #[cfg(not(feature = "rss_loose"))]
    pub title: String,
    #[cfg(not(feature = "rss_loose"))]
    pub description: String,
    #[cfg(not(feature = "rss_loose"))]
    pub name: String,
    #[cfg(not(feature = "rss_loose"))]
    pub link: String,

    #[cfg(feature = "rss_loose")]
    pub title: Option<String>,
    #[cfg(feature = "rss_loose")]
    pub description: Option<String>,
    #[cfg(feature = "rss_loose")]
    pub name: Option<String>,
    #[cfg(feature = "rss_loose")]
    pub link: Option<String>,
}

impl ViaXml for TextInput {
    fn to_xml(&self) -> Element {
        let mut elem = Element::new("textInput".to_owned(), None, vec![]);

        #[cfg(not(feature = "rss_loose"))]
        elem.tag_with_text("title", self.title.clone());
        #[cfg(not(feature = "rss_loose"))]
        elem.tag_with_text("description", self.description.clone());
        #[cfg(not(feature = "rss_loose"))]
        elem.tag_with_text("name", self.name.clone());
        #[cfg(not(feature = "rss_loose"))]
        elem.tag_with_text("link", self.link.clone());

        #[cfg(feature = "rss_loose")]
        elem.tag_with_optional_text("title", self.title.clone());
        #[cfg(feature = "rss_loose")]
        elem.tag_with_optional_text("description", self.description.clone());
        #[cfg(feature = "rss_loose")]
        elem.tag_with_optional_text("name", self.name.clone());
        #[cfg(feature = "rss_loose")]
        elem.tag_with_optional_text("link", self.link.clone());

        elem
    }

    fn from_xml(elem: Element) -> Result<Self, ReadError> {
        #[cfg(not(feature = "rss_loose"))]
        let title = match elem.get_child("title", None) {
            Some(elem) => elem.content_str(),
            None => return Err(ReadError::TextInputMissingTitle),
        };

        #[cfg(not(feature = "rss_loose"))]
        let description = match elem.get_child("description", None) {
            Some(elem) => elem.content_str(),
            None => return Err(ReadError::TextInputMissingDescription),
        };

        #[cfg(not(feature = "rss_loose"))]
        let name = match elem.get_child("name", None) {
            Some(elem) => elem.content_str(),
            None => return Err(ReadError::TextInputMissingName),
        };

        #[cfg(not(feature = "rss_loose"))]
        let link = match elem.get_child("link", None) {
            Some(elem) => elem.content_str(),
            None => return Err(ReadError::TextInputMissingLink),
        };

        #[cfg(feature = "rss_loose")]
        let title = elem.get_child("title", None).map(Element::content_str);
        #[cfg(feature = "rss_loose")]
        let description = elem.get_child("description", None).map(Element::content_str);
        #[cfg(feature = "rss_loose")]
        let name = elem.get_child("name", None).map(Element::content_str);
        #[cfg(feature = "rss_loose")]
        let link = elem.get_child("link", None).map(Element::content_str);

        Ok(TextInput {
            title: title,
            description: description,
            name: name,
            link: link,
        })
    }
}
