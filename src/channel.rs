use std::str::FromStr;

use quick_xml::{XmlReader, Event, Element};

use fromxml::{self, FromXml};
use error::Error;
use category::Category;
use cloud::Cloud;
use image::Image;
use textinput::TextInput;
use item::Item;
use extension::ExtensionMap;
use extension::itunes::ITunesChannelExtension;
use extension::dublincore::DublinCoreExtension;

/// A representation of the `<channel>` element.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Channel {
    /// The name of the channel.
    pub title: String,
    /// The URL for the website corresponding to the channel.
    pub link: String,
    /// A description of the channel.
    pub description: String,
    /// The language of the channel.
    pub language: Option<String>,
    /// The copyright notice for the channel.
    pub copyright: Option<String>,
    /// The email address for the managing editor.
    pub managing_editor: Option<String>,
    /// The email address for the webmaster.
    pub webmaster: Option<String>,
    /// The publication date for the content of the channel.
    pub pub_date: Option<String>,
    /// The date that the contents of the channel last changed.
    pub last_build_date: Option<String>,
    /// The categories the channel belongs to.
    pub categories: Vec<Category>,
    /// The program used to generate the channel.
    pub generator: Option<String>,
    /// A URL that points to the documentation for the RSS format.
    pub docs: Option<String>,
    /// The cloud to register with to be notified of updates to the channel.
    pub cloud: Option<Cloud>,
    /// The number of minutes the channel can be cached before refreshing.
    pub ttl: Option<String>,
    /// An image that can be displayed with the channel.
    pub image: Option<Image>,
    /// A text input box that can be displayed with the channel.
    pub text_input: Option<TextInput>,
    /// A hint to tell the aggregator which hours it can skip.
    pub skip_hours: Vec<String>,
    /// A hint to tell the aggregator which days it can skip.
    pub skip_days: Vec<String>,
    /// The items in the channel.
    pub items: Vec<Item>,
    /// The extensions for the channel.
    pub extensions: ExtensionMap,
    /// The iTunes extension for the channel.
    pub itunes_ext: Option<ITunesChannelExtension>,
    /// The Dublin Core extension for the channel.
    pub dublin_core_ext: Option<DublinCoreExtension>,
}

impl Channel {
    #[inline]
    /// Attempt to read the RSS channel from the speficied reader.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let reader: BufRead = ...;
    /// let channel = Channel::read_from(reader).unwrap();
    /// ```
    pub fn read_from<R: ::std::io::BufRead>(reader: R) -> Result<Channel, Error> {
        ::parser::parse(::quick_xml::XmlReader::from_reader(reader))
    }
}

impl FromXml for Channel {
    fn from_xml<R: ::std::io::BufRead>(mut reader: XmlReader<R>,
                                       _: Element)
                                       -> Result<(Self, XmlReader<R>), Error> {
        let mut channel = Channel::default();

        while let Some(e) = reader.next() {
            match e {
                Ok(Event::Start(element)) => {
                    match element.name() {
                        b"category" => {
                            let (category, reader_) = try!(Category::from_xml(reader, element));
                            reader = reader_;
                            channel.categories.push(category);
                        }
                        b"cloud" => {
                            let (cloud, reader_) = try!(Cloud::from_xml(reader, element));
                            reader = reader_;
                            channel.cloud = Some(cloud);
                        }
                        b"image" => {
                            let (image, reader_) = try!(Image::from_xml(reader, element));
                            reader = reader_;
                            channel.image = Some(image);
                        }
                        b"textInput" => {
                            let (text_input, reader_) = try!(TextInput::from_xml(reader, element));
                            reader = reader_;
                            channel.text_input = Some(text_input);
                        }
                        b"item" => {
                            let (item, reader_) = try!(Item::from_xml(reader, element));
                            reader = reader_;
                            channel.items.push(item);
                        }
                        b"title" => {
                            if let Some(content) = element_text!(reader) {
                                channel.title = content;
                            }
                        }
                        b"link" => {
                            if let Some(content) = element_text!(reader) {
                                channel.link = content;
                            }
                        }
                        b"description" => {
                            if let Some(content) = element_text!(reader) {
                                channel.description = content;
                            }
                        }
                        b"language" => channel.language = element_text!(reader),
                        b"copyright" => channel.copyright = element_text!(reader),
                        b"managingEditor" => {
                            channel.managing_editor = element_text!(reader);
                        }
                        b"webMaster" => channel.webmaster = element_text!(reader),
                        b"pubDate" => channel.pub_date = element_text!(reader),
                        b"lastBuildDate" => {
                            channel.last_build_date = element_text!(reader);
                        }
                        b"generator" => channel.generator = element_text!(reader),
                        b"docs" => channel.docs = element_text!(reader),
                        b"ttl" => channel.ttl = element_text!(reader),
                        b"skipHours" => {
                            while let Some(e) = reader.next() {
                                match e {
                                    Ok(Event::Start(element)) => {
                                        if element.name() == b"hour" {
                                            if let Some(content) = element_text!(reader) {
                                                channel.skip_hours.push(content);
                                            }
                                        } else {
                                            skip_element!(reader);
                                        }
                                    }
                                    Ok(Event::End(_)) => {
                                        break;
                                    }
                                    Err(err) => return Err(err.into()),
                                    _ => {}
                                }
                            }
                        }
                        b"skipDays" => {
                            while let Some(e) = reader.next() {
                                match e {
                                    Ok(Event::Start(element)) => {
                                        if element.name() == b"day" {
                                            if let Some(content) = element_text!(reader) {
                                                channel.skip_days.push(content);
                                            }
                                        } else {
                                            skip_element!(reader);
                                        }
                                    }
                                    Ok(Event::End(_)) => {
                                        break;
                                    }
                                    Err(err) => return Err(err.into()),
                                    _ => {}
                                }
                            }
                        }
                        _ => {
                            if let Some((ns, name)) = fromxml::extension_name(&element) {
                                parse_extension!(reader, element, ns, name, channel.extensions);
                            } else {
                                skip_element!(reader);
                            }
                        }
                    }
                }
                Ok(Event::End(_)) => {
                    if !channel.extensions.is_empty() {
                        if let Some(map) = channel.extensions.remove("itunes") {
                            channel.itunes_ext = Some(ITunesChannelExtension::from_map(map));
                        }

                        if let Some(map) = channel.extensions.remove("dc") {
                            channel.dublin_core_ext = Some(DublinCoreExtension::from_map(map));
                        }
                    }

                    return Ok((channel, reader));
                }
                Err(err) => return Err(err.into()),
                _ => {}
            }
        }

        Err(Error::EOF)
    }
}

impl FromStr for Channel {
    type Err = Error;
    #[inline]
    /// Attempt to read the RSS channel from the speficied str.
    fn from_str(s: &str) -> Result<Channel, Error> {
        Channel::read_from(s.as_bytes())
    }
}
