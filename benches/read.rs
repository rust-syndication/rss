#![feature(test)]

extern crate test;
extern crate rss;

use test::Bencher;
use rss::Channel;

#[bench]
fn bench_rss2sample(b: &mut Bencher) {
    let input: &[u8] = include_bytes!("../tests/data/rss2sample.xml");
    b.iter(|| {
        let _ = Channel::read_from(input).expect("failed to parse feed");
    });
}

#[bench]
fn bench_extensions(b: &mut Bencher) {
    let input: &[u8] = include_bytes!("../tests/data/extension.xml");
    b.iter(|| {
        let _ = Channel::read_from(input).expect("failed to parse feed");
    });
}
