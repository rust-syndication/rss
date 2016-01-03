use std::str::FromStr;
use xml::Element;

use ::{ElementUtils, ReadError, ViaXml};


/// [RSS 2.0 Specification § `<image>` sub-element of `<channel>`]
/// (http://cyber.law.harvard.edu/rss/rss.html#ltimagegtSubelementOfLtchannelgt)
#[derive(Default, Debug, Clone)]
pub struct Image {
    pub url: String,
    pub title: String,
    pub link: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl ViaXml for Image {
    fn to_xml(&self) -> Element {
        let mut elem = Element::new("image".to_owned(), None, vec![]);
        elem.tag_with_text("url", self.url.clone());
        elem.tag_with_text("title", self.title.clone());
        elem.tag_with_text("link", self.link.clone());
        if let Some(ref n) = self.width {
            elem.tag_with_text("width", n.to_string());
        }
        if let Some(ref n) = self.height {
            elem.tag_with_text("height", n.to_string());
        }
        elem
    }

    fn from_xml(elem: Element) -> Result<Self, ReadError> {
        let url = match elem.get_child("url", None) {
            Some(elem) => elem.content_str(),
            None => return Err(ReadError::ImageMissingUrl),
        };

        let title = match elem.get_child("title", None) {
            Some(elem) => elem.content_str(),
            None => return Err(ReadError::ImageMissingTitle),
        };

        let link = match elem.get_child("link", None) {
            Some(elem) => elem.content_str(),
            None => return Err(ReadError::ImageMissingLink),
        };

        let height = match elem.get_child("height", None)
                               .map(|h| u32::from_str(&h.content_str()))
        {
            Some(Ok(height)) => Some(height),
            Some(Err(_)) => return Err(ReadError::ImageHeightInvalid),
            None => None,
        };

        let width = match elem.get_child("width", None)
                              .map(|h| u32::from_str(&h.content_str()))
        {
            Some(Ok(width)) => Some(width),
            Some(Err(_)) => return Err(ReadError::ImageWidthInvalid),
            None => None,
        };

        Ok(Image {
            url: url,
            title: title,
            link: link,
            height: height,
            width: width,
        })
    }
}
