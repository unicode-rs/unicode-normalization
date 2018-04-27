#![feature(test)]
#![feature(iterator_step_by)]
extern crate unicode_normalization;
extern crate test;

use test::Bencher;
use unicode_normalization::UnicodeNormalization;

macro_rules! bench_check {
    ($b: ident, $s: expr, $form: ident) => {
        let s = $s;
        $b.iter(|| s.chars().eq(s.$form()));
    }
}

#[bench]
fn bench_is_nfc_ascii(b: &mut Bencher) {
    bench_check!(b, "all types of normalized", nfc);
}

#[bench]
fn bench_is_nfc_normalized(b: &mut Bencher) {
    bench_check!(b, "Introducci\u{00f3}n a Unicode.pdf", nfc);
}

#[bench]
fn bench_is_nfc_not_normalized(b: &mut Bencher) {
    bench_check!(b, "Introduccio\u{0301}n a Unicode.pdf", nfc);
}

#[bench]
fn bench_is_nfd_ascii(b: &mut Bencher) {
    bench_check!(b, "an easy string to check", nfd);
}

#[bench]
fn bench_is_nfd_normalized(b: &mut Bencher) {
    bench_check!(b, "Introduccio\u{0301}n a Unicode.pdf", nfd);
}

#[bench]
fn bench_is_nfd_not_normalized(b: &mut Bencher) {
    bench_check!(b, "Introducci\u{00f3}n a Unicode.pdf", nfd);
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
