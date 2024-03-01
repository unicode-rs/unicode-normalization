use core::iter::FusedIterator;

use tinyvec::ArrayVec;

use crate::normalize::hangul_constants::{N_COUNT, S_BASE, T_COUNT};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum JamoKind {
    L,
    V,
    T,
}

impl JamoKind {
    fn of(c: char) -> (Option<Self>, Option<Self>) {
        match c {
            // L
            '\u{1100}'..='\u{115F}' | '\u{A960}'..='\u{A97C}' => {
                (Some(JamoKind::L), Some(JamoKind::L))
            }
            // V
            '\u{1160}'..='\u{11A7}' | '\u{D7B0}'..='\u{D7C6}' => {
                (Some(JamoKind::V), Some(JamoKind::V))
            }
            // T
            '\u{11A8}'..='\u{11FF}' | '\u{D7CB}'..='\u{D7FB}' => {
                (Some(JamoKind::T), Some(JamoKind::T))
            }
            // LV or LVT
            '\u{AC00}'..='\u{D7A3}' => (
                Some(JamoKind::L),
                Some(if ((u32::from(c) - S_BASE) % N_COUNT) % T_COUNT == 0 {
                    // LV
                    JamoKind::V
                } else {
                    // LVT
                    JamoKind::T
                }),
            ),
            _ => (None, None),
        }
    }
}

/// Iterator over a string's characters, with '\u{115F}' and '\u{1160}' inserted
/// where needed to ensure all Korean syllable blocks are in standard form
/// by [UAX29 rules](https://www.unicode.org/reports/tr29/#Standard_Korean_Syllables).
#[derive(Clone, Debug)]
pub struct StandardKoreanSyllables<I> {
    prev_end_jamo_kind: Option<JamoKind>,
    buf: ArrayVec<[Option<char>; 3]>,
    inner: I,
}

impl<I: Iterator<Item = char>> Iterator for StandardKoreanSyllables<I> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.buf.pop() {
            c
        } else {
            let next_c = self.inner.next();
            let prev_end_jamo_kind = self.prev_end_jamo_kind;
            let (next_start_jamo_kind, next_end_jamo_kind) =
                next_c.map_or((None, None), JamoKind::of);
            self.prev_end_jamo_kind = next_end_jamo_kind;

            insert_fillers(
                next_c,
                prev_end_jamo_kind,
                next_start_jamo_kind,
                &mut self.buf,
            )
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (inner_lo, inner_hi) = self.inner.size_hint();
        let add_factor: usize = self.buf.len();
        (
            inner_lo.saturating_add(add_factor),
            inner_hi
                .and_then(|h| h.checked_mul(3)) // T → Lf Vf T
                .and_then(|h| h.checked_add(add_factor)),
        )
    }
}

impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for StandardKoreanSyllables<I> {}

#[inline]
fn insert_fillers(
    next_c: Option<char>,
    prev_end_jamo_kind: Option<JamoKind>,
    next_start_jamo_kind: Option<JamoKind>,
    buf: &mut ArrayVec<[Option<char>; 3]>,
) -> Option<char> {
    match (prev_end_jamo_kind, next_start_jamo_kind) {
        // Insert choseong filler before V not preceded by L or V
        (None, Some(JamoKind::V)) | (Some(JamoKind::T), Some(JamoKind::V)) => {
            buf.push(next_c);
            Some('\u{115F}')
        }
        // Insert choseong and jungseong fillers before T preceded non-jamo
        (None, Some(JamoKind::T)) => {
            buf.push(next_c);
            buf.push(Some('\u{1160}'));
            Some('\u{115F}')
        }
        // Insert V filler between L and non-jamo
        (Some(JamoKind::L), None) => {
            buf.push(next_c);
            Some('\u{1160}')
        }
        // For L followed by T, insert V filler, L filler, then another V filler
        (Some(JamoKind::L), Some(JamoKind::T)) => {
            buf.push(next_c);
            buf.push(Some('\u{1160}'));
            buf.push(Some('\u{115F}'));
            Some('\u{1160}')
        }
        _ => next_c,
    }
}

impl<I> StandardKoreanSyllables<I> {
    #[inline]
    pub(crate) fn new(iter: I) -> Self {
        Self {
            prev_end_jamo_kind: None,
            buf: ArrayVec::new(),
            inner: iter,
        }
    }
}
