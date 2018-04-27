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
