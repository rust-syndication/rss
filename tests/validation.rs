#![cfg(feature = "validation")]

use std::error::Error;

use rss::validation::{Validate, ValidationError};
use rss::{Enclosure, Image};

#[test]
fn image_height_limit() {
    let mut image = Image {
        url: "https://example.com/image.png".into(),
        link: "https://example.com/".into(),
        title: "Image".into(),
        height: Some("400".into()),
        ..Default::default()
    };

    image.validate().expect("image height 400 should be valid");

    image.height = Some("401".into());
    image
        .validate()
        .expect_err("image height 401 should exceed the RSS limit");
}

#[test]
fn enclosure_mime_type_error_keeps_source() {
    let enclosure = Enclosure {
        url: "https://example.com/audio.mp3".into(),
        length: "1".into(),
        mime_type: "audio".into(),
    };

    let err = enclosure
        .validate()
        .expect_err("a MIME type without a subtype should be invalid");
    assert!(matches!(err, ValidationError::MimeParsing(_)));
    assert_eq!(err.to_string(), "Unable to parse MIME type");
    let source = err.source().expect("MIME parse error should be the source");
    assert!(source.is::<mime::FromStrError>());
}
