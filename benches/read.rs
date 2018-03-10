// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

#![feature(test)]

extern crate rss;
extern crate test;

use rss::Channel;
use test::Bencher;

#[bench]
fn read_rss2sample(b: &mut Bencher) {
    let input: &[u8] = include_bytes!("../tests/data/rss2sample.xml");
    b.iter(|| {
        let _ = Channel::read_from(input).expect("failed to parse feed");
    });
}

#[bench]
fn read_itunes(b: &mut Bencher) {
    let input: &[u8] = include_bytes!("../tests/data/itunes.xml");
    b.iter(|| {
        let _ = Channel::read_from(input).expect("failed to parse feed");
    });
}

#[bench]
fn read_dublincore(b: &mut Bencher) {
    let input: &[u8] = include_bytes!("../tests/data/dublincore.xml");
    b.iter(|| {
        let _ = Channel::read_from(input).expect("failed to parse feed");
    });
}
