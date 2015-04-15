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
//! use unicode_normalization::str::UnicodeNormalization;
//!
//! fn main() {
//!     assert_eq!(compose('A','\u{30a}'), Some('Å'));
//!     
//!     let s = "ÅΩ";
//!     let c = UnicodeNormalization::nfc_chars(s).collect::<String>();
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
//! unicode-normalization = "0.0.2"
//! ```

#![deny(missing_docs, unsafe_code)]

pub use tables::UNICODE_VERSION;

mod decompose;
mod normalize;
mod recompose;
mod tables;

#[cfg(test)]
mod test;

/// Methods for composing and decomposing characters.
pub mod char {
    pub use normalize::{decompose_canonical, decompose_compatible, compose};

    /// Look up the canonical combining class of a character.
    pub use tables::normalization::canonical_combining_class;
}

/// Methods for applying composition and decomposition to strings.
pub mod str {
    pub use super::decompose::Decompositions;
    pub use super::recompose::Recompositions;

    /// Methods for iterating over strings while applying Unicode normalizations
    /// as described in 
    /// [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/).
    pub trait UnicodeNormalization {
        /// Returns an iterator over the string in Unicode Normalization Form D
        /// (canonical decomposition).
        #[inline]
        fn nfd_chars(&self) -> Decompositions;

        /// Returns an iterator over the string in Unicode Normalization Form KD
        /// (compatibility decomposition).
        #[inline]
        fn nfkd_chars(&self) -> Decompositions;

        /// An Iterator over the string in Unicode Normalization Form C
        /// (canonical decomposition followed by canonical composition).
        #[inline]
        fn nfc_chars(&self) -> Recompositions;

        /// An Iterator over the string in Unicode Normalization Form KC
        /// (compatibility decomposition followed by canonical composition).
        #[inline]
        fn nfkc_chars(&self) -> Recompositions;
    }

    impl UnicodeNormalization for str {
        #[inline]
        fn nfd_chars(&self) -> Decompositions {
            super::decompose::new_canonical(self)
        }

        #[inline]
        fn nfkd_chars(&self) -> Decompositions {
            super::decompose::new_compatible(self)
        }

        #[inline]
        fn nfc_chars(&self) -> Recompositions {
            super::recompose::new_canonical(self)
        }

        #[inline]
        fn nfkc_chars(&self) -> Recompositions {
            super::recompose::new_compatible(self)
        }
    }
}
