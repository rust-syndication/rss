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

use ::{ReadError, ViaXml};


/// [RSS 2.0 Specification § `<category>` sub-element of `<item>`]
/// (http://cyber.law.harvard.edu/rss/rss.html#ltcategorygtSubelementOfLtitemgt)
#[derive(Default, Debug, Clone)]
pub struct Category {
    pub domain: Option<String>,
    pub value: String,
}

impl ViaXml for Category {
    fn to_xml(&self) -> Element {
        let mut category = match self.domain {
            Some(ref d) => Element::new("category".to_owned(), None, vec![("domain".to_owned(), None, d.clone())]),
            None => Element::new("category".to_owned(), None, vec![]),
        };
        category.text(self.value.clone());
        category
    }

    fn from_xml(elem: Element) -> Result<Self, ReadError> {
        let domain = elem.get_attribute("domain", None).map(|s| s.to_owned());
        let value = elem.content_str();

        Ok(Category {
            domain: domain,
            value: value,
        })
    }
}
