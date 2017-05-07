use category::Category;
use cloud::Cloud;
use error::Error;
use extension::{self, ExtensionMap};
use extension::dublincore::DublinCoreExtension;
use extension::itunes::ITunesChannelExtension;

use fromxml::{self, FromXml};
use image::Image;
use item::Item;

use quick_xml::{Element, Event, XmlReader, XmlWriter};
use quick_xml::error::Error as XmlError;
use std::collections::HashMap;
use std::str::{self, FromStr};
use textinput::TextInput;
use toxml::{ToXml, XmlWriterExt};

/// A representation of the `<channel>` element.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Channel
{
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
    /// The namespaces present in the RSS tag.
    pub namespaces: HashMap<String, String>,
}

impl Channel
{
    /// Attempt to read the RSS channel from the speficied reader.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let reader: BufRead = ...;
    /// let channel = Channel::read_from(reader).unwrap();
    /// ```
    pub fn read_from<R: ::std::io::BufRead>(reader: R) -> Result<Channel, Error>
    {
        let mut reader = XmlReader::from_reader(reader).trim_text(true);
        let mut in_rss = false;
        let mut namespaces = HashMap::new();

        while let Some(e) = reader.next() {
            match e {
                Ok(Event::Start(element)) => {
                    match element.name() {
                        b"rss" if !in_rss => {
                            for attr in element.attributes().with_checks(false) {
                                if let Ok(attr) = attr {
                                    let split = attr.0
                                                    .splitn(2,
                                                            |b| *b == b':')
                                                    .collect::<Vec<_>>();
                                    if split.len() != 2 {
                                        continue;
                                    }

                                    let ns = unsafe { split.get_unchecked(0) };
                                    if ns != b"xmlns" {
                                        continue;
                                    }

                                    let name = unsafe { split.get_unchecked(1) };
                                    if name == b"itunes" || name == b"dc" {
                                        continue;
                                    }

                                    let key = str::from_utf8(name)?.to_string();
                                    let value = str::from_utf8(attr.1)?.to_string();
                                    namespaces.insert(key,
                                                      value);
                                }
                            }

                            in_rss = true;
                        },
                        b"channel" if in_rss => {
                            let mut channel = Channel::from_xml(reader,
                                                                element)
                                .map(|v| v.0)?;
                            channel.namespaces = namespaces;
                            return Ok(channel);
                        },
                        _ => skip_element!(reader),
                    }
                },
                Ok(Event::End(_)) => in_rss = false,
                Err(err) => return Err(err.into()),
                _ => {},
            }
        }

        Err(Error::EOF)
    }

    /// Attempt to write the RSS channel as XML to the speficied writer.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let channel: Channel = ...;
    /// let writer: Write = ...;
    /// channel.write_to(writer).unwrap();
    /// ```
    pub fn write_to<W: ::std::io::Write>(&self,
                                         writer: W)
        -> Result<W, Error>
    {
        let mut writer = ::quick_xml::XmlWriter::new(writer);

        let element = Element::new(b"rss");

        writer.write(Event::Start({
                                      let mut element = element.clone();
                                      element.extend_attributes(::std::iter::once((b"version", b"2.0")));

                                      let mut itunes_ns = self.itunes_ext.is_some();
                                      let mut dc_ns = self.dublin_core_ext.is_some();

                                      if !itunes_ns || dc_ns {
                                          for item in &self.items {
                                              if !itunes_ns {
                                                  itunes_ns = item.itunes_ext.is_some();
                                              }

                                              if !dc_ns {
                                                  dc_ns = item.dublin_core_ext.is_some();
                                              }

                                              if itunes_ns && dc_ns {
                                                  break;
                                              }
                                          }
                                      }

                                      if itunes_ns {
                                          element.extend_attributes(::std::iter::once((b"xmlns:itunes",
                                                             extension::itunes::NAMESPACE)));
                                      }

                                      if dc_ns {
                                          element.extend_attributes(::std::iter::once((b"xmlns:dc",
                                                             extension::dublincore::NAMESPACE)));
                                      }

                                      element.extend_attributes(self.namespaces.iter().map(|(name, url)| {
                                                                                               (format!("xmlns:{}",
                                                                                                        name),
                                                                                                url)
                                                                                           }));

                                      element
                                  }))?;

        self.to_xml(&mut writer)?;

        writer.write(Event::End(element))?;

        Ok(writer.into_inner())
    }
}

impl ToString for Channel
{
    fn to_string(&self) -> String
    {
        let buf = self.write_to(Vec::new()).unwrap_or(Vec::new());
        // this unwrap should be safe since the bytes written from the Channel are all valid utf8
        String::from_utf8(buf).unwrap()
    }
}

impl FromXml for Channel
{
    fn from_xml<R: ::std::io::BufRead>(mut reader: XmlReader<R>,
                                       _: Element)
        -> Result<(Self, XmlReader<R>), Error>
    {
        let mut channel = Channel::default();

        while let Some(e) = reader.next() {
            match e {
                Ok(Event::Start(element)) => {
                    match element.name() {
                        b"category" => {
                            let (category, reader_) = Category::from_xml(reader,
                                                                         element)?;
                            reader = reader_;
                            channel.categories.push(category);
                        },
                        b"cloud" => {
                            let (cloud, reader_) = Cloud::from_xml(reader,
                                                                   element)?;
                            reader = reader_;
                            channel.cloud = Some(cloud);
                        },
                        b"image" => {
                            let (image, reader_) = Image::from_xml(reader,
                                                                   element)?;
                            reader = reader_;
                            channel.image = Some(image);
                        },
                        b"textInput" => {
                            let (text_input, reader_) = TextInput::from_xml(reader,
                                                                            element)?;
                            reader = reader_;
                            channel.text_input = Some(text_input);
                        },
                        b"item" => {
                            let (item, reader_) = Item::from_xml(reader,
                                                                 element)?;
                            reader = reader_;
                            channel.items.push(item);
                        },
                        b"title" => {
                            if let Some(content) = element_text!(reader) {
                                channel.title = content;
                            }
                        },
                        b"link" => {
                            if let Some(content) = element_text!(reader) {
                                channel.link = content;
                            }
                        },
                        b"description" => {
                            if let Some(content) = element_text!(reader) {
                                channel.description = content;
                            }
                        },
                        b"language" => channel.language = element_text!(reader),
                        b"copyright" => channel.copyright = element_text!(reader),
                        b"managingEditor" => {
                            channel.managing_editor = element_text!(reader);
                        },
                        b"webMaster" => channel.webmaster = element_text!(reader),
                        b"pubDate" => channel.pub_date = element_text!(reader),
                        b"lastBuildDate" => {
                            channel.last_build_date = element_text!(reader);
                        },
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
                                    },
                                    Ok(Event::End(_)) => {
                                        break;
                                    },
                                    Err(err) => return Err(err.into()),
                                    _ => {},
                                }
                            }
                        },
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
                                    },
                                    Ok(Event::End(_)) => {
                                        break;
                                    },
                                    Err(err) => return Err(err.into()),
                                    _ => {},
                                }
                            }
                        },
                        _ => {
                            if let Some((ns, name)) = fromxml::extension_name(&element) {
                                parse_extension!(reader,
                                                 element,
                                                 ns,
                                                 name,
                                                 channel.extensions);
                            } else {
                                skip_element!(reader);
                            }
                        },
                    }
                },
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
                },
                Err(err) => return Err(err.into()),
                _ => {},
            }
        }

        Err(Error::EOF)
    }
}

impl ToXml for Channel
{
    fn to_xml<W: ::std::io::Write>(&self,
                                   writer: &mut XmlWriter<W>)
        -> Result<(), XmlError>
    {
        let element = Element::new(b"channel");

        writer.write(Event::Start(element.clone()))?;

        writer.write_text_element(b"title",
                                  &self.title)?;
        writer.write_text_element(b"link",
                                  &self.link)?;
        writer.write_text_element(b"description",
                                  &self.description)?;

        if let Some(language) = self.language.as_ref() {
            writer.write_text_element(b"language",
                                      language)?;
        }

        if let Some(copyright) = self.copyright.as_ref() {
            writer.write_text_element(b"copyright",
                                      copyright)?;
        }

        if let Some(managing_editor) = self.managing_editor.as_ref() {
            writer.write_text_element(b"managingEditor",
                                      managing_editor)?;
        }

        if let Some(webmaster) = self.webmaster.as_ref() {
            writer.write_text_element(b"webMaster",
                                      webmaster)?;
        }

        if let Some(pub_date) = self.pub_date.as_ref() {
            writer.write_text_element(b"pubDate",
                                      pub_date)?;
        }

        if let Some(last_build_date) = self.last_build_date.as_ref() {
            writer.write_text_element(b"lastBuildDate",
                                      last_build_date)?;
        }

        writer.write_objects(&self.categories)?;

        if let Some(generator) = self.generator.as_ref() {
            writer.write_text_element(b"generator",
                                      generator)?;
        }

        if let Some(docs) = self.docs.as_ref() {
            writer.write_text_element(b"docs",
                                      docs)?;
        }

        if let Some(cloud) = self.cloud.as_ref() {
            writer.write_object(cloud)?;
        }

        if let Some(ttl) = self.ttl.as_ref() {
            writer.write_text_element(b"ttl",
                                      ttl)?;
        }

        if let Some(image) = self.image.as_ref() {
            writer.write_object(image)?;
        }

        if let Some(text_input) = self.text_input.as_ref() {
            writer.write_object(text_input)?;
        }

        if !self.skip_hours.is_empty() {
            let element = Element::new(b"skipHours");
            writer.write(Event::Start(element.clone()))?;
            for hour in &self.skip_hours {
                writer.write_text_element(b"hour",
                                          hour)?;
            }
            writer.write(Event::End(element))?;
        }

        if !self.skip_days.is_empty() {
            let element = Element::new(b"skipDays");
            writer.write(Event::Start(element.clone()))?;
            for day in &self.skip_days {
                writer.write_text_element(b"day",
                                          day)?;
            }
            writer.write(Event::End(element))?;
        }

        writer.write_objects(&self.items)?;

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

impl FromStr for Channel
{
    type Err = Error;
    #[inline]
    /// Attempt to read the RSS channel from the speficied str.
    fn from_str(s: &str) -> Result<Channel, Error>
    {
        Channel::read_from(s.as_bytes())
    }
}
