#[cfg(not(feature = "std"))]
use alloc::collections::VecDeque;
use core::iter::FusedIterator;
#[cfg(feature = "std")]
use std::collections::VecDeque;

use crate::{lookups, tables};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CcsKind {
    /// A CCS base character (graphic character other than combining mark).
    Base,

    /// A combining character other than a `Default_Ignorable_Code_Point`.
    NonIgnorableCombining,

    /// A default-ignorable combining character, ZWJ, or ZWNJ.
    IgnorableCombining,
}

impl CcsKind {
    fn of(c: char) -> Option<Self> {
        if c == '\u{200C}' || c == '\u{200D}' {
            // ZWNJ || ZWJ
            Some(CcsKind::IgnorableCombining)
        } else if lookups::is_combining_mark(c) {
            if tables::is_default_ignorable_mark(c) {
                Some(CcsKind::IgnorableCombining)
            } else {
                Some(CcsKind::NonIgnorableCombining)
            }
        } else if tables::not_in_ccs(c) {
            None
        } else {
            Some(CcsKind::Base)
        }
    }
}

/// An iterator over the string that corrects
/// [defective combining character sequences](https://www.unicode.org/versions/Unicode15.0.0/UnicodeStandard-15.0.pdf#I6.1.36487)
/// by inserting U+00A0 NO-BREAK SPACE in front of them.
///
/// For the purposes of this iterator, private use characters,
/// as well as unassigned codepoints other than noncharacters,
/// are considered valid base characters,
/// so combining character sequences that start with such will not be modified.
///
/// In addition, combining character sequences that consist entirely of `Default_Ignorable_Code_Point`s
/// will not be modified. (Because of this, this iterator may buffer up to the entire length of its input;
/// it is *not* "stream-safe" *even if* used with [`StreamSafe`][crate::StreamSafe]).
#[derive(Clone, Debug)]
pub struct CorrectDefectiveCcs<I> {
    /// Whether the last character emitted was part of a CCS.
    in_ccs: bool,
    buffer: VecDeque<Option<char>>,
    /// Whether the last character in `buffer` is part of a CCS.
    /// (Updated only when `is_ccs` is set from false to true).
    end_of_buffer_in_ccs: bool,
    iter: I,
}

impl<I: Iterator<Item = char>> Iterator for CorrectDefectiveCcs<I> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.in_ccs {
            if let Some(c) = self.buffer.pop_front() {
                // Empty buffer

                if self.buffer.is_empty() {
                    self.in_ccs = self.end_of_buffer_in_ccs;
                }
                c
            } else {
                // Forward from inner iterator

                let c = self.iter.next();
                if c.map_or(true, tables::not_in_ccs) {
                    self.in_ccs = false;
                }
                c
            }
        } else {
            if self.buffer.is_empty() {
                // We don't have a buffer of default ignorable combining characters built up

                let c = self.iter.next()?;
                match CcsKind::of(c) {
                    // Character not in CCS, just forward it
                    None => return Some(c),

                    // Character starts non-defective CCS,
                    // label ourselves as in CCS and forward it
                    Some(CcsKind::Base) => {
                        self.in_ccs = true;
                        return Some(c);
                    }

                    // Character starts defective CCS and is not default-ignorable.
                    // Put it in the buffer to emit on next iteration,
                    // mark ourselves as in CCS,
                    // and emit NO-BREAK SPACE
                    Some(CcsKind::NonIgnorableCombining) => {
                        self.in_ccs = true;
                        self.end_of_buffer_in_ccs = true;
                        self.buffer.push_back(Some(c));
                        return Some('\u{00A0}'); // NO-BREAK SPACE
                    }

                    // Character starts defective CCS and is default-ignorable.
                    // Put it in the buffer, and fall through to loop below
                    // to find out whether we emit a NO-BREAK SPACE first.
                    Some(CcsKind::IgnorableCombining) => {
                        self.buffer.push_back(Some(c));
                    }
                }
            }

            loop {
                // We do have a buffer of default ignorable combining characters built up,
                // and we need to figure out whether to emit a NO-BREAK SPACE first.

                let c = self.iter.next();
                match c.and_then(CcsKind::of) {
                    // Inner iterator yielded character outside CCS (or `None`).
                    // Emit the built-up buffer with no leading NO-BREAK SPACE.
                    None => {
                        self.in_ccs = true;
                        self.end_of_buffer_in_ccs = false;
                        let ret = self.buffer.pop_front().unwrap();
                        self.buffer.push_back(c);
                        return ret;
                    }

                    // Inner iterator yielded character that starts a new CCS.
                    // Emit the built-up buffer with no leading NO-BREAK SPACE.
                    Some(CcsKind::Base) => {
                        self.in_ccs = true;
                        self.end_of_buffer_in_ccs = true;
                        let ret = self.buffer.pop_front().unwrap();
                        self.buffer.push_back(c);
                        return ret;
                    }

                    // Inner iterator yielded non-ignorable combining character.
                    // Emit the built-up buffer with leading NO-BREAK SPACE.
                    Some(CcsKind::NonIgnorableCombining) => {
                        self.in_ccs = true;
                        self.end_of_buffer_in_ccs = true;
                        self.buffer.push_back(c);
                        return Some('\u{00A0}'); // NO-BREAK SPACE
                    }

                    // Inner iterator yielded ignorable combining character.
                    // Add it to the buffer, don't emit anything.
                    Some(CcsKind::IgnorableCombining) => {
                        self.buffer.push_back(c);
                    }
                }
            }
        }
    }
}

impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for CorrectDefectiveCcs<I> {}

impl<I> CorrectDefectiveCcs<I> {
    pub(crate) fn new(iter: I) -> Self {
        Self {
            in_ccs: false,
            buffer: VecDeque::new(),
            end_of_buffer_in_ccs: false,
            iter,
        }
    }
}
