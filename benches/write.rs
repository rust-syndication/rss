#![feature(test)]

extern crate test;
extern crate rss;

use std::io::sink;
use test::Bencher;
use rss::Channel;

#[bench]
fn write_rss2sample(b: &mut Bencher) {
    let input: &[u8] = include_bytes!("../tests/data/rss2sample.xml");
    let channel = Channel::read_from(input).expect("failed to parse feed");
    b.iter(|| {
        let _ = channel.write_to(sink()).expect("failed to write");
    });
}

#[bench]
fn write_itunes(b: &mut Bencher) {
    let input: &[u8] = include_bytes!("../tests/data/itunes.xml");
    let channel = Channel::read_from(input).expect("failed to parse feed");
    b.iter(|| {
        let _ = channel.write_to(sink()).expect("failed to write");
    });
}

#[bench]
fn write_dublincore(b: &mut Bencher) {
    let input: &[u8] = include_bytes!("../tests/data/dublincore.xml");
    let channel = Channel::read_from(input).expect("failed to parse feed");
    b.iter(|| {
        let _ = channel.write_to(sink()).expect("failed to write");
    });
}
