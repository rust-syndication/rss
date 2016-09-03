#![feature(test)]

extern crate test;
extern crate rss;

use test::Bencher;
use rss::Channel;

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
