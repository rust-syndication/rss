# rust-rss

[![Build Status](https://travis-ci.org/frewsxcv/rust-rss.svg?branch=master)](https://travis-ci.org/frewsxcv/rust-rss)
[![rss on Crates.io](https://meritbadge.herokuapp.com/rss)](https://crates.io/crates/rss)

[Documentation](https://frewsxcv.github.io/rust-rss/)

Library for serializing the RSS web content syndication format.

## Examples

### Writing

```rust
use rss::{Channel, Item, Rss};

let item = Item {
    title: Some(String::from("Ford hires Elon Musk as CEO")),
    pub_date: Some(String::from("01 Apr 2019 07:30:00 GMT")),
    description: Some(String::from("In an unprecedented move, Ford hires Elon Musk.")),
    ..Default::default()
};

let channel = Channel {
    title: String::from("TechCrunch"),
    link: String::from("http://techcrunch.com"),
    description: String::from("The latest technology news and information on startups"),
    items: vec![item],
    ..Default::default()
};

let rss = Rss(channel);

let rss_string = rss.to_string();
```

### Reading

```rust
use rss::Rss;

let rss_str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0">
  <channel>
    <title>TechCrunch</title>
    <link>http://techcrunch.com</link>
    <description>The latest technology news and information on startups</description>
    <item>
      <title>Ford hires Elon Musk as CEO</title>
      <pubDate>01 Apr 2019 07:30:00 GMT</pubDate>
      <description>In an unprecedented move, Ford hires Elon Musk.</description>
    </item>
  </channel>
</rss>
"#;

let rss = rss_str.parse::<Rss>().unwrap();
```

### Partial Feeds

In some cases, the RSS source may not return a standards-compliant RSS such as a missing description tag. The library
is designed to return an error in such cases, however this behaviour can be loosened by using the feature
flag `rss_loose`.

Using this flag changes what would normally be a `String` type to a `Option<String>`, just like other fields.

In your `Cargo.toml`, add the following:
```toml
[dependencies.rss]
features = ["rss_loose"]
```

## Contributors & License

- Michael Yoo [GitHub](https://github.com/sekjun9878) [Web](https://www.michael.yoo.id.au/)

Released under The Apache License 2.0

