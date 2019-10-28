# unicode-normalization

[![Build Status](https://travis-ci.org/unicode-rs/unicode-normalization.svg)](https://travis-ci.org/unicode-rs/unicode-normalization)
[![Docs](https://docs.rs/unicode-normalization/badge.svg)](https://docs.rs/unicode-normalization/)

Unicode character composition and decomposition utilities
as described in
[Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/).

This crate requires Rust 1.21+.

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

You can use this package in your project by adding the following
to your `Cargo.toml`:

```toml
[dependencies]
unicode-normalization = "0.1.8"
```

## Linux Vendors / Downstream
As is, tests won't work on the published crate, as important
corpus data required for fully testing functionality otherwise
bloats the size of the crate.

Tests aren't hugely meaningful without this, but there are two
workarounds:

```bash
RUSTFLAGS="--cfg minimal_tests" cargo test
```

This will make the crate compile, and some arbitrary set of lower
quality tests pass.

```bash
python scripts/unicode.py
cp ./normalization_tests.rs src/
```

This will generate the full corpus required for tests to work,
without needing to pass any special flags.
