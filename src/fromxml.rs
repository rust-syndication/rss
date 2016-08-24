use quick_xml::{XmlReader, Element};

use error::Error;

pub trait FromXml: Sized {
    fn from_xml<R: ::std::io::BufRead>(mut reader: XmlReader<R>,
                                       element: Element)
                                       -> Result<(Self, XmlReader<R>), Error>;
}

pub fn element_text<R: ::std::io::BufRead>(mut reader: XmlReader<R>)
                                           -> (Result<Option<String>, Error>, XmlReader<R>) {
    use quick_xml::Event;

    macro_rules! try {
        ($e:expr, $reader:ident) => (match $e {
            Ok(v) => v,
            Err(err) => return (Err(err.into()), $reader),
        })
    }

    let mut content: Option<String> = None;
    let mut depth = 0;

    while let Some(e) = reader.next() {
        match e {
            Ok(Event::Start(_)) => {
                depth += 1;
            }
            Ok(Event::End(_)) => {
                depth -= 1;
                if depth == -1 {
                    break;
                }
            }
            Ok(Event::CData(element)) => {
                if depth == 0 {
                    let text_content = element.content();
                    let text = try!(::std::str::from_utf8(text_content), reader);

                    if content.is_some() {
                        content.as_mut().unwrap().push_str(text);
                    } else {
                        content = Some(text.to_string());
                    }
                }
            }
            Ok(Event::Text(element)) => {
                if depth == 0 {
                    use std::borrow::Cow;

                    let text_content = try!(element.unescaped_content().map_err(|e| e.0), reader);
                    let text = match text_content {
                        Cow::Owned(owned) => Cow::Owned(try!(String::from_utf8(owned), reader)),
                        Cow::Borrowed(borrowed) => {
                            Cow::Borrowed(try!(::std::str::from_utf8(borrowed), reader))
                        }
                    };

                    if content.is_some() {
                        content.as_mut().unwrap().push_str(&text);
                    } else {
                        content = Some(text.into_owned());
                    }
                }
            }
            Err(err) => return (Err(err.0.into()), reader),
            _ => {}
        }
    }

    (Ok(content), reader)
}

macro_rules! element_text {
    ($reader:ident) => ({
        let text = ::fromxml::element_text($reader);
        $reader = text.1;
        try!(text.0)
    })
}

macro_rules! close_element {
    ($reader:ident) => ({
        let mut depth = 0;

        while let Some(e) = $reader.next() {
            match e {
                Ok(Event::Start(_)) => {
                    depth += 1;
                }
                Ok(Event::End(_)) => {
                    depth -= 1;
                    if depth == -1 {
                        break;
                    }
                }
                Err(err) => return Err(err.0.into()),
                _ => {}
            }
        }
    })
}
