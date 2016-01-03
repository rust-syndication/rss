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
    pub title: String,
    pub description: String,
    pub name: String,
    pub link: String,
}

impl ViaXml for TextInput {
    fn to_xml(&self) -> Element {
        let mut elem = Element::new("textInput".to_owned(), None, vec![]);
        elem.tag_with_text("title", self.title.clone());
        elem.tag_with_text("description", self.description.clone());
        elem.tag_with_text("name", self.name.clone());
        elem.tag_with_text("link", self.link.clone());
        elem
    }

    fn from_xml(elem: Element) -> Result<Self, ReadError> {
        let title = match elem.get_child("title", None) {
            Some(elem) => elem.content_str(),
            None => return Err(ReadError::TextInputMissingTitle),
        };

        let description = match elem.get_child("description", None) {
            Some(elem) => elem.content_str(),
            None => return Err(ReadError::TextInputMissingDescription),
        };

        let name = match elem.get_child("name", None) {
            Some(elem) => elem.content_str(),
            None => return Err(ReadError::TextInputMissingName),
        };

        let link = match elem.get_child("link", None) {
            Some(elem) => elem.content_str(),
            None => return Err(ReadError::TextInputMissingLink),
        };

        Ok(TextInput {
            title: title,
            description: description,
            name: name,
            link: link,
        })
    }
}
