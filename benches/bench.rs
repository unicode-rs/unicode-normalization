#![feature(test)]
#![feature(iterator_step_by)]
extern crate unicode_normalization;
extern crate test;

use test::Bencher;
use unicode_normalization::UnicodeNormalization;

#[bench]
fn bench_is_nfc_ascii(b: &mut Bencher) {
    b.iter(|| unicode_normalization::is_nfc("all types of normalized"));
}

#[bench]
fn bench_is_nfc_normalized(b: &mut Bencher) {
    b.iter(|| unicode_normalization::is_nfc("Introducci\u{00f3}n a Unicode.pdf"));
}

#[bench]
fn bench_is_nfc_not_normalized(b: &mut Bencher) {
    b.iter(|| unicode_normalization::is_nfc("Introduccio\u{0301}n a Unicode.pdf"));
}

#[bench]
fn bench_is_nfd_ascii(b: &mut Bencher) {
    b.iter(|| unicode_normalization::is_nfd("an easy string to check"));
}

#[bench]
fn bench_is_nfd_normalized(b: &mut Bencher) {
    b.iter(|| unicode_normalization::is_nfd("Introduccio\u{0301}n a Unicode.pdf"));
}

#[bench]
fn bench_is_nfd_not_normalized(b: &mut Bencher) {
    b.iter(|| unicode_normalization::is_nfd("Introducci\u{00f3}n a Unicode.pdf"));
}

#[bench]
fn bench_nfc_ascii(b: &mut Bencher) {
    let s = "normalize me please";
    b.iter(|| s.nfc().count());
}

#[bench]
fn bench_nfd_ascii(b: &mut Bencher) {
    let s = "decompose me entirely";
    b.iter(|| s.nfd().count());
}

#[bench]
fn bench_streamsafe_ascii(b: &mut Bencher) {
    let s = "quite nonthreatening";
    b.iter(|| s.stream_safe().count());
}

#[bench]
fn bench_streamsafe_adversarial(b: &mut Bencher) {
    let s = "bo\u{0300}\u{0301}\u{0302}\u{0303}\u{0304}\u{0305}\u{0306}\u{0307}\u{0308}\u{0309}\u{030a}\u{030b}\u{030c}\u{030d}\u{030e}\u{030f}\u{0310}\u{0311}\u{0312}\u{0313}\u{0314}\u{0315}\u{0316}\u{0317}\u{0318}\u{0319}\u{031a}\u{031b}\u{031c}\u{031d}\u{032e}oom";
    b.iter(|| s.stream_safe().count());
}
