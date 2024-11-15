# rss

[![Build status](https://github.com/rust-syndication/rss/workflows/Build/badge.svg)](https://github.com/rust-syndication/rss/actions?query=branch%3Amaster)
[![Codecov](https://codecov.io/gh/rust-syndication/rss/branch/master/graph/badge.svg)](https://codecov.io/gh/rust-syndication/rss)
[![crates.io Status](https://img.shields.io/crates/v/rss.svg)](https://crates.io/crates/rss)
[![Docs](https://docs.rs/rss/badge.svg)](https://docs.rs/rss)

Library for deserializing and serializing the RSS web content syndication format.

### Supported Versions

Reading from the following RSS versions is supported:

* RSS 0.90
* RSS 0.91
* RSS 0.92
* RSS 1.0
* RSS 2.0

Writing support is limited to RSS 2.0.

### Documentation

- [Released](https://docs.rs/rss/)
- [Master](https://rust-syndication.github.io/rss/rss/)

## Usage

Add the dependency to your `Cargo.toml`.

```toml
[dependencies]
rss = "2.0"
```

## Reading

A channel can be read from any object that implements the `BufRead` trait.

### From a file

```rust
use std::fs::File;
use std::io::BufReader;
use rss::Channel;

let file = File::open("example.xml").unwrap();
let channel = Channel::read_from(BufReader::new(file)).unwrap();
```

### From a buffer

**Note**: This example requires [reqwest](https://crates.io/crates/reqwest) crate.

```rust
use std::error::Error;
use rss::Channel;

async fn example_feed() -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get("http://example.com/feed.xml")
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}
```

## Writing

A channel can be written to any object that implements the `Write` trait or converted to an XML string using the `ToString` trait.

```rust
use rss::Channel;

let channel = Channel::default();
channel.write_to(::std::io::sink()).unwrap(); // // write to the channel to a writer
let string = channel.to_string(); // convert the channel to a string
```

## Creation

Builder methods are provided to assist in the creation of channels.

**Note**: This requires the `builders` feature, which is enabled by default.

```rust
use rss::ChannelBuilder;

let channel = ChannelBuilder::default()
    .title("Channel Title")
    .link("http://example.com")
    .description("An RSS feed.")
    .build()
    .unwrap();
```

## Validation

Validation methods are provided to validate the contents of a channel against the RSS specification.

**Note**: This requires enabling the `validation` feature.

```rust
use rss::Channel;
use rss::validation::Validate;

let channel = Channel::default();
channel.validate().unwrap();
```

## Extensions

Elements which have non-default namespaces will be considered extensions. Extensions are stored in `Channel.extensions` and `Item.extensions`. 

For convenience, [Dublin Core](http://dublincore.org/documents/dces/), [Syndication](http://web.resource.org/rss/1.0/modules/syndication/) and [iTunes](https://help.apple.com/itc/podcasts_connect/#/itcb54353390) extensions are extracted to structs and stored in as properties on channels and items.

## Invalid Feeds

As a best effort to parse invalid feeds `rss` will default elements declared as "required" by the RSS 2.0 specification to an empty string.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
