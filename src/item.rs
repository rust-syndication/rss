
use category::Category;
use enclosure::Enclosure;
use error::Error;
use extension::ExtensionMap;
use extension::dublincore::DublinCoreExtension;
use extension::itunes::ITunesItemExtension;

use fromxml::{self, FromXml};
use guid::Guid;
use quick_xml::{Element, Event, XmlReader, XmlWriter};
use quick_xml::error::Error as XmlError;
use source::Source;
use toxml::{ToXml, XmlWriterExt};

/// A representation of the `<item>` element.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Item
{
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
    /// The extensions for the item.
    pub extensions: ExtensionMap,
    /// The iTunes extension for the item.
    pub itunes_ext: Option<ITunesItemExtension>,
    /// The Dublin Core extension for the item.
    pub dublin_core_ext: Option<DublinCoreExtension>,
}

impl FromXml for Item
{
    fn from_xml<R: ::std::io::BufRead>(mut reader: XmlReader<R>,
                                       _: Element)
        -> Result<(Self, XmlReader<R>), Error>
    {
        let mut item = Item::default();

        while let Some(e) = reader.next() {
            match e {
                Ok(Event::Start(element)) => {
                    match element.name() {
                        b"category" => {
                            let (category, reader_) = Category::from_xml(reader,
                                                                         element)?;
                            reader = reader_;
                            item.categories.push(category);
                        },
                        b"guid" => {
                            let (guid, reader_) = Guid::from_xml(reader,
                                                                 element)?;
                            reader = reader_;
                            item.guid = Some(guid);
                        },
                        b"enclosure" => {
                            let (enclosure, reader_) = Enclosure::from_xml(reader,
                                                                           element)?;
                            reader = reader_;
                            item.enclosure = Some(enclosure);
                        },
                        b"source" => {
                            let (source, reader_) = Source::from_xml(reader,
                                                                     element)?;
                            reader = reader_;
                            item.source = Some(source);
                        },
                        b"title" => item.title = element_text!(reader),
                        b"link" => item.link = element_text!(reader),
                        b"description" => item.description = element_text!(reader),
                        b"author" => item.author = element_text!(reader),
                        b"comments" => item.comments = element_text!(reader),
                        b"pubDate" => item.pub_date = element_text!(reader),
                        b"content:encoded" => item.content = element_text!(reader),
                        _ => {
                            if let Some((ns, name)) = fromxml::extension_name(&element) {
                                parse_extension!(reader,
                                                 element,
                                                 ns,
                                                 name,
                                                 item.extensions);
                            } else {
                                skip_element!(reader);
                            }
                        },
                    }
                },
                Ok(Event::End(_)) => {
                    if !item.extensions.is_empty() {
                        if let Some(map) = item.extensions.remove("itunes") {
                            item.itunes_ext = Some(ITunesItemExtension::from_map(map));
                        }

                        if let Some(map) = item.extensions.remove("dc") {
                            item.dublin_core_ext = Some(DublinCoreExtension::from_map(map));
                        }
                    }

                    return Ok((item, reader));
                },
                Err(err) => return Err(err.into()),
                _ => {},
            }
        }

        Err(Error::EOF)
    }
}

impl ToXml for Item
{
    fn to_xml<W: ::std::io::Write>(&self,
                                   writer: &mut XmlWriter<W>)
        -> Result<(), XmlError>
    {
        let element = Element::new(b"item");

        writer.write(Event::Start(element.clone()))?;

        if let Some(title) = self.title.as_ref() {
            writer.write_text_element(b"title",
                                      title)?;
        }

        if let Some(link) = self.link.as_ref() {
            writer.write_text_element(b"link",
                                      link)?;
        }

        if let Some(description) = self.description.as_ref() {
            writer.write_text_element(b"description",
                                      description)?;
        }

        if let Some(author) = self.author.as_ref() {
            writer.write_text_element(b"author",
                                      author)?;
        }

        writer.write_objects(&self.categories)?;

        if let Some(comments) = self.comments.as_ref() {
            writer.write_text_element(b"comments",
                                      comments)?;
        }

        if let Some(enclosure) = self.enclosure.as_ref() {
            writer.write_object(enclosure)?;
        }

        if let Some(guid) = self.guid.as_ref() {
            writer.write_object(guid)?;
        }

        if let Some(pub_date) = self.pub_date.as_ref() {
            writer.write_text_element(b"pubDate",
                                      pub_date)?;
        }

        if let Some(source) = self.source.as_ref() {
            writer.write_object(source)?;
        }

        if let Some(content) = self.content.as_ref() {
            writer.write_cdata_element(b"content:encoded",
                                       content)?;
        }

        for map in self.extensions.values() {
            for extensions in map.values() {
                for extension in extensions {
                    extension.to_xml(writer)?;
                }
            }
        }

        if let Some(ext) = self.itunes_ext.as_ref() {
            ext.to_xml(writer)?;
        }

        if let Some(ext) = self.dublin_core_ext.as_ref() {
            ext.to_xml(writer)?;
        }

        writer.write(Event::End(element))
    }
}
