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

use ::{Category, ElementUtils, Item, Image, ReadError, TextInput, ViaXml};


/// [RSS 2.0 Specification § Required channel elements]
/// (http://cyber.law.harvard.edu/rss/rss.html#requiredChannelElements)
///
/// # Examples
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
#[derive(Default, Debug, Clone)]
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
    pub image: Option<Image>,
    pub rating: Option<String>,
    pub text_input: Option<TextInput>,
    pub skip_hours: Option<String>,
    pub skip_days: Option<String>,
}

impl ViaXml for Channel {
    fn to_xml(&self) -> Element {
        let mut channel = Element::new("channel".to_owned(), None, vec![]);

        channel.tag_with_text("title", self.title.clone());
        channel.tag_with_text("link", self.link.clone());
        channel.tag_with_text("description", self.description.clone());

        for item in &self.items {
            channel.tag(item.to_xml());
        }

        channel.tag_with_optional_text("language", self.language.clone());
        channel.tag_with_optional_text("copyright", self.copyright.clone());
        channel.tag_with_optional_text("managingEditor", self.managing_editor.clone());
        channel.tag_with_optional_text("webMaster", self.web_master.clone());
        channel.tag_with_optional_text("pubDate", self.pub_date.clone());
        channel.tag_with_optional_text("lastBuildDate", self.last_build_date.clone());
        channel.tag_with_optional_text("generator", self.generator.clone());
        channel.tag_with_optional_text("docs", self.docs.clone());
        channel.tag_with_optional_text("ttl", self.ttl.clone());
        channel.tag_with_optional_text("rating", self.rating.clone());

        if let Some(ref text_input) = self.text_input {
            channel.tag(text_input.to_xml());
        }

        channel.tag_with_optional_text("skipHours", self.skip_hours.clone());
        channel.tag_with_optional_text("skipDays", self.skip_days.clone());

        for category in &self.categories {
            channel.tag(category.to_xml());
        }

        channel
    }

    fn from_xml(elem: Element) -> Result<Self, ReadError> {
        let title = match elem.get_child("title", None) {
            Some(elem) => elem.content_str(),
            None => return Err(ReadError::ChannelMissingTitle),
        };

        let link = match elem.get_child("link", None) {
            Some(elem) => elem.content_str(),
            None => return Err(ReadError::ChannelMissingLink),
        };

        let description = match elem.get_child("description", None) {
            Some(elem) => elem.content_str(),
            None => return Err(ReadError::ChannelMissingDescription),
        };

        let items = match elem.get_children("item", None)
                              .map(|e| ViaXml::from_xml(e.clone()))
                              .collect::<Result<Vec<_>, _>>()
        {
            Ok(items) => items,
            Err(err) => return Err(err),
        };

        let language = elem.get_child("language", None).map(Element::content_str);
        let copyright = elem.get_child("copyright", None).map(Element::content_str);
        let managing_editor = elem.get_child("managingEditor", None).map(Element::content_str);
        let web_master = elem.get_child("webMaster", None).map(Element::content_str);
        let pub_date = elem.get_child("pubDate", None).map(Element::content_str);
        let last_build_date = elem.get_child("lastBuildDate", None).map(Element::content_str);

        let categories = match elem.get_children("category", None)
                                   .map(|e| ViaXml::from_xml(e.clone()))
                                   .collect::<Result<Vec<_>, _>>()
        {
            Ok(categories) => categories,
            Err(err) => return Err(err),
        };

        let generator = elem.get_child("generator", None).map(Element::content_str);
        let docs = elem.get_child("docs", None).map(Element::content_str);
        let ttl = elem.get_child("ttl", None).map(Element::content_str);

        let image = match elem.get_child("image", None).map(|e| Image::from_xml(e.clone())) {
            Some(Ok(image)) => Some(image),
            Some(Err(err)) => return Err(err),
            None => None,
        };

        let rating = elem.get_child("rating", None).map(Element::content_str);

        let text_input = match elem.get_child("textInput", None).map(|e| ViaXml::from_xml(e.clone())) {
            Some(Ok(text_input)) => Some(text_input),
            Some(Err(err)) => return Err(err),
            None => None,
        };

        let skip_hours = elem.get_child("skipHours", None).map(Element::content_str);
        let skip_days = elem.get_child("skipDays", None).map(Element::content_str);

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
