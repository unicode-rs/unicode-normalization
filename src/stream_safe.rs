use core::iter::FusedIterator;

use crate::lookups::{
    canonical_combining_class, canonical_fully_decomposed, compatibility_fully_decomposed,
    stream_safe_trailing_nonstarters,
};
use crate::normalize::{hangul_decomposition_length, is_hangul_syllable};
use crate::tables::stream_safe_leading_nonstarters;

pub(crate) const MAX_NONSTARTERS: usize = 30;
const COMBINING_GRAPHEME_JOINER: char = '\u{034F}';

/// [UAX15-D4]: This iterator keeps track of how many non-starters there have been
/// since the last starter in *NFKD* and will emit a Combining Grapheme Joiner
/// (U+034F) if the count exceeds 30.
///
/// To process input that arrives in several pieces, see [`StreamSafeState`].
///
/// [UAX15-D4]: https://www.unicode.org/reports/tr15/#UAX15-D4
#[derive(Debug)]
pub struct StreamSafe<I> {
    iter: I,
    nonstarter_count: usize,
    buffer: Option<char>,
}

/// Saved state of a [`StreamSafe`] iterator, used to continue the
/// Stream-Safe Text Process across input that arrives in several pieces.
///
/// A run of non-starters can cross the boundary between two pieces. If each
/// piece got a fresh [`StreamSafe`], the combined output could differ from
/// a single pass over the concatenated input and not be stream-safe. Take
/// the state from the iterator that consumed one piece with
/// [`StreamSafe::try_into_state_if_exhausted`] and pass it to
/// [`StreamSafe::resume`] for the next piece. The combined output is then
/// the same as a single pass.
///
/// This only applies to [`StreamSafe`] itself. Other iterators
/// applied to each piece are not resumed.
///
/// The contents are private and may change between versions. The
/// [`Default`] value is the state of a freshly created iterator.
#[derive(Clone, Debug, Default)]
pub struct StreamSafeState {
    nonstarter_count: usize,
}

impl<I: Iterator<Item = char>> StreamSafe<I> {
    /// Create a new stream safe iterator.
    ///
    /// Note that this iterator can also be obtained by directly calling [`.stream_safe()`](crate::UnicodeNormalization::stream_safe)
    /// on the iterator.
    #[inline]
    pub fn new(iter: I) -> Self {
        Self::resume(iter, StreamSafeState::default())
    }

    /// Create a stream safe iterator that continues from a state previously
    /// returned by [`StreamSafe::try_into_state_if_exhausted`], instead of
    /// starting fresh. This allows for continuing processing across input
    /// that arrives in several pieces.
    ///
    /// `StreamSafe::resume(iter, StreamSafeState::default())` is equivalent
    /// to [`StreamSafe::new(iter)`](StreamSafe::new).
    ///
    /// # Examples
    ///
    /// ```
    /// use unicode_normalization::{StreamSafe, StreamSafeState};
    ///
    /// // 40 combining marks after a starter, split across two pieces.
    /// let marks = "\u{0301}".repeat(20);
    /// let pieces = [format!("a{}", marks), marks];
    ///
    /// let mut state = StreamSafeState::default();
    /// let mut out = String::new();
    /// for piece in pieces.iter() {
    ///     let mut it = StreamSafe::resume(piece.chars(), state);
    ///     out.extend(&mut it);
    ///     state = it.try_into_state_if_exhausted()
    ///         .expect("iterator is exhausted");
    /// }
    ///
    /// let whole: String = StreamSafe::new(pieces.concat().chars()).collect();
    /// assert_eq!(out, whole);
    /// assert!(out.contains('\u{034F}'));
    /// ```
    #[inline]
    pub fn resume(iter: I, state: StreamSafeState) -> Self {
        Self {
            iter,
            nonstarter_count: state.nonstarter_count,
            buffer: None,
        }
    }

    /// Converts this iterator into the state to pass to
    /// [`StreamSafe::resume`] to continue processing the next piece of input.
    ///
    /// Succeeds only if the iterator has no more characters to yield.
    /// Otherwise, returns `Err(self)`: the iterator is handed back and yields
    /// the same remaining output as before. To check, this may call `next`
    /// on the underlying iterator once. A character it returns is still
    /// yielded by the returned iterator.
    #[inline]
    pub fn try_into_state_if_exhausted(mut self) -> Result<StreamSafeState, Self> {
        // A buffered char means there is more output. Otherwise, check the
        // source: `next()` takes the buffer first and classifies the char just
        // as if it came straight from `self.iter`, so the output is unchanged.
        if self.buffer.is_none() {
            self.buffer = self.iter.next();
        }
        match self.buffer {
            None => Ok(StreamSafeState {
                nonstarter_count: self.nonstarter_count,
            }),
            Some(_) => Err(self),
        }
    }
}

impl<I: Iterator<Item = char>> Iterator for StreamSafe<I> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        let next_ch = self.buffer.take().or_else(|| self.iter.next())?;
        let d = classify_nonstarters(next_ch);
        if self.nonstarter_count + d.leading_nonstarters > MAX_NONSTARTERS {
            // Since we're emitting a CGJ, the suffix of the emitted string in NFKD has no trailing
            // nonstarters, so we can reset the counter to zero. Put `next_ch` back into the
            // iterator (via `self.buffer`), and we'll reclassify it next iteration.
            self.nonstarter_count = 0;
            self.buffer = Some(next_ch);
            return Some(COMBINING_GRAPHEME_JOINER);
        }

        // Is the character all nonstarters in NFKD? If so, increment our counter of contiguous
        // nonstarters in NKFD.
        if d.leading_nonstarters == d.decomposition_len {
            self.nonstarter_count += d.decomposition_len;
        }
        // Otherwise, reset the counter to the decomposition's number of trailing nonstarters.
        else {
            self.nonstarter_count = d.trailing_nonstarters;
        }
        Some(next_ch)
    }
}

impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for StreamSafe<I> {}

#[derive(Debug)]
pub(crate) struct Decomposition {
    pub(crate) leading_nonstarters: usize,
    pub(crate) trailing_nonstarters: usize,
    pub(crate) decomposition_len: usize,
}

#[inline]
pub(crate) fn classify_nonstarters(c: char) -> Decomposition {
    // As usual, fast path for ASCII (which is always a starter)
    if c <= '\x7f' {
        return Decomposition {
            leading_nonstarters: 0,
            trailing_nonstarters: 0,
            decomposition_len: 1,
        };
    }
    // Next, special case Hangul, since it's not handled by our tables.
    if is_hangul_syllable(c) {
        return Decomposition {
            leading_nonstarters: 0,
            trailing_nonstarters: 0,
            decomposition_len: hangul_decomposition_length(c),
        };
    }
    let decomp = compatibility_fully_decomposed(c).or_else(|| canonical_fully_decomposed(c));
    match decomp {
        Some(decomp) => Decomposition {
            leading_nonstarters: stream_safe_leading_nonstarters(c),
            trailing_nonstarters: stream_safe_trailing_nonstarters(c),
            decomposition_len: decomp.len(),
        },
        None => {
            let is_nonstarter = canonical_combining_class(c) != 0;
            let nonstarter = if is_nonstarter { 1 } else { 0 };
            Decomposition {
                leading_nonstarters: nonstarter,
                trailing_nonstarters: nonstarter,
                decomposition_len: 1,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{classify_nonstarters, StreamSafe, StreamSafeState, MAX_NONSTARTERS};
    use crate::lookups::canonical_combining_class;
    use crate::normalize::decompose_compatible;

    #[cfg(not(feature = "std"))]
    use alloc::{format, string::String, vec::Vec};

    use core::char;

    fn stream_safe(s: &str) -> String {
        StreamSafe::new(s.chars()).collect()
    }

    fn stream_safe_pieces(pieces: &[&str]) -> String {
        let (out, _) = pieces.iter().fold(
            (String::new(), StreamSafeState::default()),
            |(mut acc, state), piece| {
                let mut it = StreamSafe::resume(piece.chars(), state);
                acc.extend(&mut it);
                (
                    acc,
                    it.try_into_state_if_exhausted()
                        .expect("iterator is exhausted"),
                )
            },
        );
        out
    }

    #[test]
    fn test_simple() {
        let technically_okay = "Da\u{0300}\u{0301}\u{0302}\u{0303}\u{0304}\u{0305}\u{0306}\u{0307}\u{0308}\u{0309}\u{030a}\u{030b}\u{030c}\u{030d}\u{030e}\u{030f}\u{0310}\u{0311}\u{0312}\u{0313}\u{0314}\u{0315}\u{0316}\u{0317}\u{0318}\u{0319}\u{031a}\u{031b}\u{031c}\u{031d}ngerzone";
        assert_eq!(stream_safe(technically_okay), technically_okay);

        let too_much = "Da\u{0300}\u{0301}\u{0302}\u{0303}\u{0304}\u{0305}\u{0306}\u{0307}\u{0308}\u{0309}\u{030a}\u{030b}\u{030c}\u{030d}\u{030e}\u{030f}\u{0310}\u{0311}\u{0312}\u{0313}\u{0314}\u{0315}\u{0316}\u{0317}\u{0318}\u{0319}\u{031a}\u{031b}\u{031c}\u{031d}\u{032e}ngerzone";
        let fixed_it = "Da\u{0300}\u{0301}\u{0302}\u{0303}\u{0304}\u{0305}\u{0306}\u{0307}\u{0308}\u{0309}\u{030a}\u{030b}\u{030c}\u{030d}\u{030e}\u{030f}\u{0310}\u{0311}\u{0312}\u{0313}\u{0314}\u{0315}\u{0316}\u{0317}\u{0318}\u{0319}\u{031a}\u{031b}\u{031c}\u{031d}\u{034f}\u{032e}ngerzone";
        assert_eq!(stream_safe(too_much), fixed_it);

        let woah_nelly = "Da\u{0300}\u{0301}\u{0302}\u{0303}\u{0304}\u{0305}\u{0306}\u{0307}\u{0308}\u{0309}\u{030a}\u{030b}\u{030c}\u{030d}\u{030e}\u{030f}\u{0310}\u{0311}\u{0312}\u{0313}\u{0314}\u{0315}\u{0316}\u{0317}\u{0318}\u{0319}\u{031a}\u{031b}\u{031c}\u{031d}\u{032e}\u{0300}\u{0301}\u{0302}\u{0303}\u{0304}\u{0305}\u{0306}\u{0307}\u{0308}\u{0309}\u{030a}\u{030b}\u{030c}\u{030d}\u{030e}\u{030f}\u{0310}\u{0311}\u{0312}\u{0313}\u{0314}\u{0315}\u{0316}\u{0317}\u{0318}\u{0319}\u{031a}\u{031b}\u{031c}\u{031d}\u{032e}ngerzone";
        let its_cool = "Da\u{0300}\u{0301}\u{0302}\u{0303}\u{0304}\u{0305}\u{0306}\u{0307}\u{0308}\u{0309}\u{030a}\u{030b}\u{030c}\u{030d}\u{030e}\u{030f}\u{0310}\u{0311}\u{0312}\u{0313}\u{0314}\u{0315}\u{0316}\u{0317}\u{0318}\u{0319}\u{031a}\u{031b}\u{031c}\u{031d}\u{034f}\u{032e}\u{0300}\u{0301}\u{0302}\u{0303}\u{0304}\u{0305}\u{0306}\u{0307}\u{0308}\u{0309}\u{030a}\u{030b}\u{030c}\u{030d}\u{030e}\u{030f}\u{0310}\u{0311}\u{0312}\u{0313}\u{0314}\u{0315}\u{0316}\u{0317}\u{0318}\u{0319}\u{031a}\u{031b}\u{031c}\u{034f}\u{031d}\u{032e}ngerzone";
        assert_eq!(stream_safe(woah_nelly), its_cool);
    }

    #[test]
    fn test_all_nonstarters() {
        let s = "\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}\u{0300}";
        let expected = "\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}\u{034F}\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}\u{300}";
        assert_eq!(stream_safe(s), expected);
    }

    #[test]
    fn test_classify_nonstarters() {
        // Highest character in the `compat_fully_decomp` table is 2FA1D
        for ch in 0..0x2FA1E {
            let ch = match char::from_u32(ch) {
                Some(c) => c,
                None => continue,
            };
            let c = classify_nonstarters(ch);
            let mut s = Vec::new();
            decompose_compatible(ch, |c| s.push(c));

            assert_eq!(s.len(), c.decomposition_len);

            let num_leading = s
                .iter()
                .take_while(|&c| canonical_combining_class(*c) != 0)
                .count();
            let num_trailing = s
                .iter()
                .rev()
                .take_while(|&c| canonical_combining_class(*c) != 0)
                .count();

            assert_eq!(num_leading, c.leading_nonstarters);
            assert_eq!(num_trailing, c.trailing_nonstarters);
        }
    }

    #[test]
    fn test_resume_any_split() {
        let marks: String = (0x0300..0x0320)
            .map(|c| char::from_u32(c).unwrap())
            .collect();
        let inputs = [
            // A run of combining marks long enough to need a CGJ.
            format!("a{}{}b", marks, marks),
            // U+FF9E has ccc 0, but its NFKD is the non-starter U+3099.
            format!("a{}", "\u{FF9E}".repeat(MAX_NONSTARTERS + 10)),
            // U+0344 decomposes to two non-starters; U+01D5 ends with two.
            format!("\u{01D5}{}", "\u{0344}".repeat((MAX_NONSTARTERS + 10) / 2)),
        ];
        for s in &inputs {
            let whole = stream_safe(s);
            assert!(whole.contains('\u{034F}'));
            let bounds: Vec<usize> = s
                .char_indices()
                .map(|(i, _)| i)
                .chain(core::iter::once(s.len()))
                .collect();
            for (k, &i) in bounds.iter().enumerate() {
                for &j in &bounds[k..] {
                    let pieces = [&s[..i], &s[i..j], &s[j..]];
                    assert_eq!(
                        stream_safe_pieces(&pieces),
                        whole,
                        "split at {} and {}",
                        i,
                        j
                    );
                }
            }
        }
    }

    #[test]
    fn test_try_into_state_not_exhausted() {
        let mut it = StreamSafe::new("ab".chars());
        assert_eq!(it.next(), Some('a'));
        let it = it.try_into_state_if_exhausted().unwrap_err();
        assert_eq!(it.collect::<String>(), "b");
    }

    #[test]
    fn test_try_into_state_every_position() {
        let s = format!("a{}", "\u{0301}".repeat(MAX_NONSTARTERS + 10));
        let whole = stream_safe(&s);
        for n in 0..whole.chars().count() {
            let mut it = StreamSafe::new(s.chars());
            let mut out: String = it.by_ref().take(n).collect();
            let it = it.try_into_state_if_exhausted().unwrap_err();
            out.extend(it);
            assert_eq!(out, whole, "stopped after {}", n);
        }
    }
}
