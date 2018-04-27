// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Functions for computing canonical and compatible decompositions for Unicode characters.
use std::char;
use std::ops::FnMut;
use tables;

/// Compute canonical Unicode decomposition for character.
/// See [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/)
/// for more information.
#[inline]
pub fn decompose_canonical<F>(c: char, mut emit_char: F) where F: FnMut(char) {
    // 7-bit ASCII never decomposes
    if c <= '\x7f' {
        emit_char(c);
        return;
    }

    // Perform decomposition for Hangul
    if (c as u32) >= S_BASE && (c as u32) < (S_BASE + S_COUNT) {
        decompose_hangul(c, emit_char);
        return;
    }

    if let Some(decomposed) = tables::canonical_fully_decomposed(c) {
        for &d in decomposed {
            emit_char(d);
        }
        return;
    }

    // Finally bottom out.
    emit_char(c);
}

/// Compute canonical or compatible Unicode decomposition for character.
/// See [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/)
/// for more information.
#[inline]
pub fn decompose_compatible<F: FnMut(char)>(c: char, mut emit_char: F) {
    // 7-bit ASCII never decomposes
    if c <= '\x7f' {
        emit_char(c);
        return;
    }

    // Perform decomposition for Hangul
    if (c as u32) >= S_BASE && (c as u32) < (S_BASE + S_COUNT) {
        decompose_hangul(c, emit_char);
        return;
    }

    if let Some(decomposed) = tables::compatibility_fully_decomposed(c) {
        for &d in decomposed {
            emit_char(d);
        }
        return;
    }

    if let Some(decomposed) = tables::canonical_fully_decomposed(c) {
        for &d in decomposed {
            emit_char(d);
        }
        return;
    }

    // Finally bottom out.
    emit_char(c);
}

/// Compose two characters into a single character, if possible.
/// See [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/)
/// for more information.
pub fn compose(a: char, b: char) -> Option<char> {
    compose_hangul(a, b).or_else(|| tables::composition_table(a, b))
}

// Constants from Unicode 9.0.0 Section 3.12 Conjoining Jamo Behavior
// http://www.unicode.org/versions/Unicode9.0.0/ch03.pdf#M9.32468.Heading.310.Combining.Jamo.Behavior
const S_BASE: u32 = 0xAC00;
const L_BASE: u32 = 0x1100;
const V_BASE: u32 = 0x1161;
const T_BASE: u32 = 0x11A7;
const L_COUNT: u32 = 19;
const V_COUNT: u32 = 21;
const T_COUNT: u32 = 28;
const N_COUNT: u32 = (V_COUNT * T_COUNT);
const S_COUNT: u32 = (L_COUNT * N_COUNT);

// Decompose a precomposed Hangul syllable
#[allow(unsafe_code)]
#[inline(always)]
fn decompose_hangul<F>(s: char, mut emit_char: F) where F: FnMut(char) {
    let si = s as u32 - S_BASE;

    let li = si / N_COUNT;
    unsafe {
        emit_char(char::from_u32_unchecked(L_BASE + li));

        let vi = (si % N_COUNT) / T_COUNT;
        emit_char(char::from_u32_unchecked(V_BASE + vi));

        let ti = si % T_COUNT;
        if ti > 0 {
            emit_char(char::from_u32_unchecked(T_BASE + ti));
        }
    }
}

// Compose a pair of Hangul Jamo
#[allow(unsafe_code)]
#[inline(always)]
fn compose_hangul(a: char, b: char) -> Option<char> {
    let l = a as u32;
    let v = b as u32;
    // Compose an LPart and a VPart
    if L_BASE <= l && l < (L_BASE + L_COUNT) // l should be an L choseong jamo
        && V_BASE <= v && v < (V_BASE + V_COUNT) { // v should be a V jungseong jamo
        let r = S_BASE + (l - L_BASE) * N_COUNT + (v - V_BASE) * T_COUNT;
        return unsafe { Some(char::from_u32_unchecked(r)) };
    }
    // Compose an LVPart and a TPart
    if S_BASE <= l && l <= (S_BASE+S_COUNT-T_COUNT) // l should be a syllable block
        && T_BASE <= v && v < (T_BASE+T_COUNT) // v should be a T jongseong jamo
        && (l - S_BASE) % T_COUNT == 0 { // l should be an LV syllable block (not LVT)
        let r = l + (v - T_BASE);
        return unsafe { Some(char::from_u32_unchecked(r)) };
    }
    None
}
