//! <http://std.dkuug.dk/jtc1/sc2/wg2/docs/n3422.pdf> Annex B

use core::{
    convert::{TryFrom, TryInto},
    iter::FusedIterator,
};

use tinyvec::ArrayVec;

// § B.1.1

use crate::normalize::hangul_constants::{
    L_BASE, L_LAST, N_COUNT, S_BASE, S_COUNT, T_BASE, T_COUNT, T_LAST, V_BASE, V_LAST,
};

// § B.1.2

fn is_old_jongseong(t: char) -> bool {
    match t {
        '\u{11C3}'..='\u{11FF}' | '\u{D7CB}'..='\u{D7FB}' => true,
        _ => false,
    }
}

/// Iterator that decomposes modern Hangul LV syllables immediately followed by old Hangul T jamo
/// into a 3-character L V T sequences, as specified in KS X 1026-1 annex B.1.5.
#[derive(Clone, Debug)]
pub struct RecomposeHangul<I> {
    /// Medial vowel of a decomposed LV syllable
    v: Option<char>,
    /// Character yielded by inner iterator in last call to its `next()`
    last: Option<char>,
    inner: I,
}

impl<I: Iterator<Item = char>> Iterator for RecomposeHangul<I> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.v {
            // If an LV syllable was decomposed in the last call to `next`,
            // yield its medial vowel.
            self.v = None;
            Some(v)
        } else {
            let prev = self.last;
            self.last = self.inner.next();

            if let (Some(prev), Some(next)) = (prev, self.last) {
                let s_index = u32::from(prev).wrapping_sub(S_BASE);
                if s_index < S_COUNT && s_index % T_COUNT == 0 && is_old_jongseong(next) {
                    // We have an LV syllable followed by an old jongseong, decompose into L V
                    let l: char = (L_BASE + s_index / N_COUNT).try_into().unwrap();
                    self.v = Some((V_BASE + (s_index % N_COUNT) / T_COUNT).try_into().unwrap());
                    return Some(l);
                }
            }

            prev
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (inner_lo, inner_hi) = self.inner.size_hint();
        let add_factor: usize = self.v.map_or(0, |_| 1) + self.last.map_or(0, |_| 1);
        (
            inner_lo.saturating_add(add_factor),
            inner_hi
                .and_then(|h| h.checked_mul(2))
                .and_then(|h| h.checked_add(add_factor)),
        )
    }
}

impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for RecomposeHangul<I> {}

impl<I: Iterator<Item = char>> RecomposeHangul<I> {
    #[inline]
    pub(crate) fn new(mut iter: I) -> Self {
        RecomposeHangul {
            v: None,
            last: iter.next(),
            inner: iter,
        }
    }
}

// B.2.1

static CP_JAMO: [char; 94] = [
    '\u{1100}', '\u{1101}', '\u{11AA}', '\u{1102}', '\u{11AC}', '\u{11AD}', '\u{1103}', '\u{1104}',
    '\u{1105}', '\u{11B0}', '\u{11B1}', '\u{11B2}', '\u{11B3}', '\u{11B4}', '\u{11B5}', '\u{111A}',
    '\u{1106}', '\u{1107}', '\u{1108}', '\u{1121}', '\u{1109}', '\u{110A}', '\u{110B}', '\u{110C}',
    '\u{110D}', '\u{110E}', '\u{110F}', '\u{1110}', '\u{1111}', '\u{1112}', '\u{1161}', '\u{1162}',
    '\u{1163}', '\u{1164}', '\u{1165}', '\u{1166}', '\u{1167}', '\u{1168}', '\u{1169}', '\u{116A}',
    '\u{116B}', '\u{116C}', '\u{116D}', '\u{116E}', '\u{116F}', '\u{1170}', '\u{1171}', '\u{1172}',
    '\u{1173}', '\u{1174}', '\u{1175}', '\u{1160}', '\u{1114}', '\u{1115}', '\u{11C7}', '\u{11C8}',
    '\u{11CC}', '\u{11CE}', '\u{11D3}', '\u{11D7}', '\u{11D9}', '\u{111C}', '\u{11DD}', '\u{11DF}',
    '\u{111D}', '\u{111E}', '\u{1120}', '\u{1122}', '\u{1123}', '\u{1127}', '\u{1129}', '\u{112B}',
    '\u{112C}', '\u{112D}', '\u{112E}', '\u{112F}', '\u{1132}', '\u{1136}', '\u{1140}', '\u{1147}',
    '\u{114C}', '\u{11F1}', '\u{11F2}', '\u{1157}', '\u{1158}', '\u{1159}', '\u{1184}', '\u{1185}',
    '\u{1188}', '\u{1191}', '\u{1192}', '\u{1194}', '\u{119E}', '\u{11A1}',
];

// § B.2.2

static HW_JAMO: [char; 64] = [
    '\u{1160}', '\u{1100}', '\u{1101}', '\u{11AA}', '\u{1102}', '\u{11AC}', '\u{11AD}', '\u{1103}',
    '\u{1104}', '\u{1105}', '\u{11B0}', '\u{11B1}', '\u{11B2}', '\u{11B3}', '\u{11B4}', '\u{11B5}',
    '\u{111A}', '\u{1106}', '\u{1107}', '\u{1108}', '\u{1121}', '\u{1109}', '\u{110A}', '\u{110B}',
    '\u{110C}', '\u{110D}', '\u{110E}', '\u{110F}', '\u{1110}', '\u{1111}', '\u{1112}', '\u{FFBF}',
    '\u{FFC0}', '\u{FFC1}', '\u{1161}', '\u{1162}', '\u{1163}', '\u{1164}', '\u{1165}', '\u{1166}',
    '\u{FFC8}', '\u{FFC9}', '\u{1167}', '\u{1168}', '\u{1169}', '\u{116A}', '\u{116B}', '\u{116C}',
    '\u{FFD0}', '\u{FFD1}', '\u{116D}', '\u{116E}', '\u{116F}', '\u{1170}', '\u{1171}', '\u{1172}',
    '\u{FFD8}', '\u{FFD9}', '\u{1173}', '\u{1174}', '\u{1175}', '\u{FFDD}', '\u{FFDE}', '\u{FFDF}',
];

// § B.2.3

static PC_JAMO: [char; 14] = [
    '\u{1100}', '\u{1102}', '\u{1103}', '\u{1105}', '\u{1106}', '\u{1107}', '\u{1109}', '\u{110B}',
    '\u{110C}', '\u{110E}', '\u{110F}', '\u{1110}', '\u{1111}', '\u{1112}',
];

// § B.2.4

/// Iterator that decomposes compatibility characters containing Hangul jamo
/// in a manner that avoids introducing new nonstandard jamo sequences,
/// as specified in KS X 1026-1 annex B.2.4.
#[derive(Clone, Debug)]
pub struct NormalizeJamoKdkc<I> {
    inner: I,
    // Buffer for when a character normalizes into multiple.
    // Characters are pushed to and popped from the end.
    // Length 3 is sufficient, as the longest possible expansion
    // is for a parenthesized choseong like U+3200,
    // which expands into ['(', <choseong>, '\u{1160}', ')'] (length 4).
    // (There are no parenthesized jungseong or jongseong.)
    buf: ArrayVec<[char; 3]>,
}

impl<I: Iterator<Item = char>> Iterator for NormalizeJamoKdkc<I> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.buf.pop() {
            // Empty buffer before yielding from underlying iterator.
            Some(c)
        } else {
            let ch = self.inner.next()?;
            // Whether ch is a parenthesized Hangul letter
            let mut pf = false;

            let uch: u32 = ch.into();
            let base_jamo: char = match uch {
                // Hangul compatibility letter
                0x3131..=0x318E => CP_JAMO[usize::try_from(uch - 0x3131).unwrap()],

                // Parenthesized Hangul letter
                0x3200..=0x320D => {
                    pf = true;
                    self.buf.push(')');
                    PC_JAMO[usize::try_from(uch - 0x3200).unwrap()]
                }

                // Circled Hangul letter
                0x3260..=0x326D => PC_JAMO[usize::try_from(uch - 0x3260).unwrap()],

                // Halfwidth Hangul letter
                0xFFA0..=0xFFDF => HW_JAMO[usize::try_from(uch - 0xFFA0).unwrap()],

                _ => return Some(ch),
            };

            // Insert fillers
            let first_ret: char = match base_jamo.into() {
                // `base_jamo` is choseong, yield a jungseong filler after
                L_BASE..=L_LAST => {
                    self.buf.push('\u{1160}');
                    base_jamo
                }

                // `base_jamo` is jungseong, yield a choseong filler before
                V_BASE..=V_LAST => {
                    self.buf.push(base_jamo);
                    '\u{115F}'
                }

                // `base_jamo` is jongseong, yield a choseong and a jungseong filler before
                T_BASE..=T_LAST => {
                    self.buf.push(base_jamo);
                    self.buf.push('\u{1160}');
                    '\u{115F}'
                }

                _ => unreachable!("`base_jamo` shluld be a jamo, but is not"),
            };

            if pf {
                // Parenthesized Hangul letter, yield open paren before
                self.buf.push(first_ret);
                Some('(')
            } else {
                Some(first_ret)
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (inner_lo, inner_hi) = self.inner.size_hint();
        let add_factor: usize = self.buf.len();
        (
            inner_lo.saturating_add(add_factor),
            inner_hi
                .and_then(|h| h.checked_mul(4)) // Why 4? See comment on `buf` field
                .and_then(|h| h.checked_add(add_factor)),
        )
    }
}

impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for NormalizeJamoKdkc<I> {}

impl<I: Iterator<Item = char>> NormalizeJamoKdkc<I> {
    #[inline]
    pub(crate) fn new(iter: I) -> Self {
        NormalizeJamoKdkc {
            inner: iter,
            buf: ArrayVec::new(),
        }
    }
}
