# rss-rs

[![Build Status](https://travis-ci.org/jameshurst/rss-rs.svg?branch=master)](https://travis-ci.org/jameshurst/rss-rs)
[![Crates.io Status](http://meritbadge.herokuapp.com/rss-rs)](https://crates.io/crates/rss-rs)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/jameshurst/rss-rs/master/LICENSE)

[Documentation](https://jameshurst.github.io/rss-rs/rss/)

RSS feed parser written in Rust. 

Two XML parsing backends are currently supported,
[xml-rs](https://github.com/netvl/xml-rs) and [quick-xml](https://github.com/tafia/quick-xml). By default rss-rs uses xml-rs as its parser.

## Usage

To use rss-rs just add the dependency to your `Cargo.toml`.

```toml
[dependencies]
rss-rs = "0.1"
```

If you would like to use rss-rs with quick-xml as the XML parser then enable the `quick-xml` feature and disable default features.

```toml
[dependencies]
rss-rs = {version = "0.1", features = ["quick-xml"], default-features = false}
```
The package includes a single crate named `rss`.

```rust
extern crate rss;
```

## Reading

### xml-rs

Reading can be done using any object that implements the `Read` trait. 

```rust
let reader: Read = ...;
let channel = Channel::read_from(reader).unwrap();
```
### quick-xml

Reading can be done using any object that implements the `BufRead` trait. 

```rust
let reader: BufRead = ...;
let channel = Channel::read_from(reader).unwrap();
```

## Todo

* Writing support
  
