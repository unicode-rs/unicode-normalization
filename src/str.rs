//! Methods for applying composition and decomposition to strings and char iterators.

pub use super::decompose::Decompositions;
pub use super::recompose::Recompositions;


/// Methods for iterating over strings while applying Unicode normalizations
/// as described in
/// [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/).
pub trait UnicodeNormalization<I: Iterator<Item=char>> {
    /// Returns an iterator over the string in Unicode Normalization Form D
    /// (canonical decomposition).
    #[inline]
    fn nfd_chars(self) -> Decompositions<I>;

    /// Returns an iterator over the string in Unicode Normalization Form KD
    /// (compatibility decomposition).
    #[inline]
    fn nfkd_chars(self) -> Decompositions<I>;

    /// An Iterator over the string in Unicode Normalization Form C
    /// (canonical decomposition followed by canonical composition).
    #[inline]
    fn nfc_chars(self) -> Recompositions<I>;

    /// An Iterator over the string in Unicode Normalization Form KC
    /// (compatibility decomposition followed by canonical composition).
    #[inline]
    fn nfkc_chars(self) -> Recompositions<I>;
}

impl<'a> UnicodeNormalization<Chars<'a>> for &'a str {
    #[inline]
    fn nfd_chars(self) -> Decompositions<Chars<'a>> {
        super::decompose::new_canonical(self.chars())
    }

    #[inline]
    fn nfkd_chars(self) -> Decompositions<Chars<'a>> {
        super::decompose::new_compatible(self.chars())
    }

    #[inline]
    fn nfc_chars(self) -> Recompositions<Chars<'a>> {
        super::recompose::new_canonical(self.chars())
    }

    #[inline]
    fn nfkc_chars(self) -> Recompositions<Chars<'a>> {
        super::recompose::new_compatible(self.chars())
    }
}

impl<I: Iterator<Item=char>> UnicodeNormalization<I> for I {
    #[inline]
    fn nfd_chars(self) -> Decompositions<I> {
        super::decompose::new_canonical(self)
    }

    #[inline]
    fn nfkd_chars(self) -> Decompositions<I> {
        super::decompose::new_compatible(self)
    }

    #[inline]
    fn nfc_chars(self) -> Recompositions<I> {
        super::recompose::new_canonical(self)
    }

    #[inline]
    fn nfkc_chars(self) -> Recompositions<I> {
        super::recompose::new_compatible(self)
    }
}
