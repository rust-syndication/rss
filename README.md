# rss-rs

[![Build Status](https://travis-ci.org/jameshurst/rss-rs.svg?branch=master)](https://travis-ci.org/jameshurst/rss-rs)
[![Crates.io Status](http://meritbadge.herokuapp.com/rss-rs)](https://crates.io/crates/rss-rs)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/jameshurst/rss-rs/master/LICENSE)

[Documentation](https://jameshurst.github.io/rss-rs/rss/)

A fast RSS feed parser written in Rust. 

rss-rs provides a complete implementation of the RSS 2.0 specification.

## Usage

To use rss-rs just add the dependency to your `Cargo.toml`.

```toml
[dependencies]
rss-rs = "0.3"
```

The package includes a single crate named `rss`.

```rust
extern crate rss;
```

## Reading

Reading can be done using any object that implements the `BufRead` trait. 

```rust
let reader: BufRead = ...;
let channel = Channel::read_from(reader).unwrap();
```

## Extensions

Elements which have non-default namespaces will be considered extensions. Extensions are stored in `Channel.extensions` and `Item.extensions`. 

For conveninence, [Dublin Core](http://dublincore.org/documents/dces/) and [iTunes](https://help.apple.com/itc/podcasts_connect/#/itcb54353390) extensions are extracted to structs and stored in `Channel.itunes_ext`, `Channel.dublin_core_ext`, `Item.itunes_ext`, and `Item.dublin_core_ext`.

## Invalid Feeds

As a best effort to parse invalid feeds rss-rs will default elements declared as "required" by the RSS 2.0 specification to an empty string.

## Todo

* Writing support
