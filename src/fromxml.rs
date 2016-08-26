use std::io::BufRead;

use quick_xml::{XmlReader, Element, Event};

use error::Error;

pub trait FromXml: Sized {
    fn from_xml<R: ::std::io::BufRead>(mut reader: XmlReader<R>,
                                       element: Element)
                                       -> Result<(Self, XmlReader<R>), Error>;
}

macro_rules! try_reader {
    ($e:expr, $reader:ident) => (match $e {
        Ok(v) => v,
        Err(err) => return (Err(err.into()), $reader),
    })
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
        let result = ::fromxml::close_element($reader);
        $reader = result.1;
        try!(result.0)
    })
}

pub fn element_text<R: BufRead>(mut reader: XmlReader<R>)
                                -> (Result<Option<String>, Error>, XmlReader<R>) {
    let mut content: Option<String> = None;

    while let Some(e) = reader.next() {
        match e {
            Ok(Event::Start(_)) => {
                let result = close_element(reader);
                reader = result.1;
                try_reader!(result.0, reader);
            }
            Ok(Event::End(_)) => {
                break;
            }
            Ok(Event::CData(element)) => {
                let text_content = element.content();
                let text = try_reader!(::std::str::from_utf8(text_content), reader);
                content = Some(text.to_string());
            }
            Ok(Event::Text(element)) => {
                let text_content = try_reader!(element.unescaped_content().map_err(|e| e.0),
                reader);
                let text = try_reader!(String::from_utf8(text_content.into_owned()), reader);
                content = Some(text);
            }
            Err(err) => return (Err(err.0.into()), reader),
            _ => {}
        }
    }

    (Ok(content), reader)
}

pub fn close_element<R: BufRead>(mut reader: XmlReader<R>) -> (Result<(), Error>, XmlReader<R>) {
    let mut depth = 0;

    while let Some(e) = reader.next() {
        match e {
            Ok(Event::Start(_)) => {
                depth += 1;
            }
            Ok(Event::End(_)) => {
                if depth == 0 {
                    return (Ok(()), reader);
                }
                
                depth -= 1;
            }
            Err(err) => return (Err(err.0.into()), reader),
            _ => {}
        }
    }

    (Err(Error::EOF), reader)
}
