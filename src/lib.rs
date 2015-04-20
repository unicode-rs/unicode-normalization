// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Unicode character composition and decomposition utilities
//! as described in
//! [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/).
//!
//! ```rust
//! extern crate unicode_normalization;
//!
//! use unicode_normalization::char::compose;
//! use unicode_normalization::nfc;
//!
//! fn main() {
//!     assert_eq!(compose('A','\u{30a}'), Some('Å'));
//!
//!     let s = "ÅΩ";
//!     let c = nfc(s.chars()).collect::<String>();
//!     assert_eq!(c, "ÅΩ");
//! }
//! ```
//!
//! # crates.io
//!
//! You can use this package in your project by adding the following
//! to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! unicode-normalization = "0.0.3"
//! ```

#![deny(missing_docs, unsafe_code)]
#![doc(html_logo_url = "https://unicode-rs.github.io/unicode-rs_sm.png",
       html_favicon_url = "https://unicode-rs.github.io/unicode-rs_sm.png")]

pub use tables::UNICODE_VERSION;
pub use decompose::{nfd, nfkd, Decompositions};
pub use recompose::{nfc, nfkc, Recompositions};

mod decompose;
mod normalize;
mod recompose;
mod tables;

#[cfg(test)]
mod test;
#[cfg(test)]
mod testdata;

/// Methods for composing and decomposing characters.
pub mod char {
    pub use normalize::{decompose_canonical, decompose_compatible, compose};

    /// Look up the canonical combining class of a character.
    pub use tables::normalization::canonical_combining_class;
}
