use quick_xml::{XmlReader, Event, Element};

use fromxml::FromXml;
use error::Error;
use category::Category;
use guid::Guid;
use enclosure::Enclosure;
use source::Source;

/// A representation of the `<item>` element.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Item {
    /// The title of the item.
    pub title: Option<String>,
    /// The URL of the item.
    pub link: Option<String>,
    /// The item synopsis.
    pub description: Option<String>,
    /// The email address of author of the item.
    pub author: Option<String>,
    /// The categories the item belongs to.
    pub categories: Vec<Category>,
    /// The URL for the comments page of the item.
    pub comments: Option<String>,
    /// The description of a media object that is attached to the item.
    pub enclosure: Option<Enclosure>,
    /// A unique identifier for the item.
    pub guid: Option<Guid>,
    /// The date the item was published.
    pub pub_date: Option<String>,
    /// The RSS channel the item came from.
    pub source: Option<Source>,
    /// The HTML contents of the item.
    pub content: Option<String>,
}

impl FromXml for Item {
    fn from_xml<R: ::std::io::BufRead>(mut reader: XmlReader<R>,
                                       _: Element)
                                       -> Result<(Self, XmlReader<R>), Error> {
        let mut item = Item::default();

        while let Some(e) = reader.next() {
            match e {
                Ok(Event::Start(element)) => {
                    match element.name() {
                        b"category" => {
                            let (category, reader_) = try!(Category::from_xml(reader, element));
                            reader = reader_;
                            item.categories.push(category);
                        }
                        b"guid" => {
                            let (guid, reader_) = try!(Guid::from_xml(reader, element));
                            reader = reader_;
                            item.guid = Some(guid);
                        }
                        b"enclosure" => {
                            let (enclosure, reader_) = try!(Enclosure::from_xml(reader, element));
                            reader = reader_;
                            item.enclosure = Some(enclosure);
                        }
                        b"source" => {
                            let (source, reader_) = try!(Source::from_xml(reader, element));
                            reader = reader_;
                            item.source = Some(source);
                        }
                        b"title" => item.title = element_text!(reader),
                        b"link" => item.link = element_text!(reader),
                        b"description" => item.description = element_text!(reader),
                        b"author" => item.author = element_text!(reader),
                        b"comments" => item.comments = element_text!(reader),
                        b"pubDate" => item.pub_date = element_text!(reader),
                        b"content:encoded" => item.content = element_text!(reader),
                        _ => skip_element!(reader),
                    }
                }
                Ok(Event::End(_)) => {
                    return Ok((item, reader));
                }
                Err(err) => return Err(err.0.into()),
                _ => {}
            }
        }

        Err(Error::EOF)
    }
}
