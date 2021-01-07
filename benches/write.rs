// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use bencher::{benchmark_group, benchmark_main, Bencher};
use rss::Channel;
use std::io::sink;

fn write_rss2sample(b: &mut Bencher) {
    let input: &[u8] = include_bytes!("../tests/data/rss2sample.xml");
    let channel = Channel::read_from(input).expect("failed to parse feed");
    b.iter(|| {
        let _ = channel.write_to(sink()).expect("failed to write");
    });
}

fn write_itunes(b: &mut Bencher) {
    let input: &[u8] = include_bytes!("../tests/data/itunes.xml");
    let channel = Channel::read_from(input).expect("failed to parse feed");
    b.iter(|| {
        let _ = channel.write_to(sink()).expect("failed to write");
    });
}

fn write_dublincore(b: &mut Bencher) {
    let input: &[u8] = include_bytes!("../tests/data/dublincore.xml");
    let channel = Channel::read_from(input).expect("failed to parse feed");
    b.iter(|| {
        let _ = channel.write_to(sink()).expect("failed to write");
    });
}

fn write_syndication(b: &mut Bencher) {
    let input: &[u8] = include_bytes!("../tests/data/syndication.xml");
    let channel = Channel::read_from(input).expect("failed to parse feed");
    b.iter(|| {
        let _ = channel.write_to(sink()).expect("failed to write");
    });
}

benchmark_group!(
    benches,
    write_rss2sample,
    write_itunes,
    write_dublincore,
    write_syndication,
);
benchmark_main!(benches);
