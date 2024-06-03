# unicode-normalization

[![Build Status](https://travis-ci.org/unicode-rs/unicode-normalization.svg)](https://travis-ci.org/unicode-rs/unicode-normalization)
[![Docs](https://docs.rs/unicode-normalization/badge.svg)](https://docs.rs/unicode-normalization/)

Unicode character composition and decomposition utilities
as described in
[Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/).

This crate requires Rust 1.36+.

```rust
extern crate unicode_normalization;

use unicode_normalization::char::compose;
use unicode_normalization::UnicodeNormalization;

fn main() {
    assert_eq!(compose('A','\u{30a}'), Some('Å'));

    let s = "ÅΩ";
    let c = s.nfc().collect::<String>();
    assert_eq!(c, "ÅΩ");
}
```

## crates.io

You can use this package in your project by adding the following to your
`Cargo.toml`:

```toml
[dependencies]
unicode-normalization = "0.1.23"
```

## `no_std` + `alloc` support

This crate is completely `no_std` + `alloc` compatible. This can be enabled by
disabling the `std` feature, i.e. specifying `default-features = false` for this
crate on your `Cargo.toml`.

## KS X 1026-1

Korean Standard KS X 1026-1 ([Korean](https://standard.go.kr/KSCI/standardIntro/getStandardSearchView.do?ksNo=KSX1026-1),
[English](http://std.dkuug.dk/jtc1/sc2/wg2/docs/n3422.pdf)) is an ROK government
standard that corrects some defects and makes some changes to the Unicode NFC,
NFKC, and NFKD normalization forms for certain Korean characters. The
`ks_x_1026-1` crate feature (disabled by default) adds methods to support these
alternate normalizations.
