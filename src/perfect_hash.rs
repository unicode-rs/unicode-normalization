// Copyright 2019 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Support for lookups based on minimal perfect hashing.

use tables::*;

// This function is based on multiplication being fast and is "good enough". Also
// it can share some work between the unsalted and salted versions.
#[inline]
fn my_hash(key: u32, salt: u32, n: usize) -> usize {
    let y = key.wrapping_add(salt).wrapping_mul(2654435769);
    let y = y ^ key.wrapping_mul(0x31415926);
    (((y as u64) * (n as u64)) >> 32) as usize
}

/// Do a lookup using minimal perfect hashing.
/// 
/// The table is stored as a sequence of "salt" values, then a sequence of
/// values that contain packed key/value pairs. The strategy is to hash twice.
/// The first hash retrieves a salt value that makes the second hash unique.
/// The hash function doesn't have to be very good, just good enough that the
/// resulting map is unique.
#[inline]
fn mph_lookup<KV, V, FK, FV>(x: u32, salt: &[u16], kv: &[KV], fk: FK, fv: FV,
    default: V) -> V
    where KV: Copy, FK: Fn(KV) -> u32, FV: Fn(KV) -> V
{
    let s = salt[my_hash(x, 0, salt.len())] as u32;
    let key_val = kv[my_hash(x, s, salt.len())];
    if x == fk(key_val) {
        fv(key_val)
    } else {
        default
    }
}

/// Extract the key in a 24 bit key and 8 bit value packed in a u32.
#[inline]
fn u8_lookup_fk(kv: u32) -> u32 {
    kv >> 8
}

/// Extract the value in a 24 bit key and 8 bit value packed in a u32.
#[inline]
fn u8_lookup_fv(kv: u32) -> u8 {
    (kv & 0xff) as u8
}

/// Extract the key for a boolean lookup.
#[inline]
fn bool_lookup_fk(kv: u32) -> u32 {
    kv
}

/// Extract the value for a boolean lookup.
#[inline]
fn bool_lookup_fv(_kv: u32) -> bool {
    true
}

/// Extract the key in a pair.
#[inline]
fn pair_lookup_fk<T>(kv: (u32, T)) -> u32 {
    kv.0
}

/// Extract the value in a pair, returning an option.
#[inline]
fn pair_lookup_fv_opt<T>(kv: (u32, T)) -> Option<T> {
    Some(kv.1)
}

/// Look up the canonical combining class for a codepoint.
/// 
/// The value returned is as defined in the Unicode Character Database.
pub fn canonical_combining_class(c: char) -> u8 {
    mph_lookup(c.into(), CANONICAL_COMBINING_CLASS_SALT, CANONICAL_COMBINING_CLASS_KV,
        u8_lookup_fk, u8_lookup_fv, 0)
}

pub(crate) fn composition_table(c1: char, c2: char) -> Option<char> {
    if c1 < '\u{10000}' && c2 < '\u{10000}' {
        mph_lookup((c1 as u32) << 16 | (c2 as u32),
        COMPOSITION_TABLE_SALT, COMPOSITION_TABLE_KV,
        pair_lookup_fk, pair_lookup_fv_opt, None)
    } else {
        composition_table_astral(c1, c2)
    }
}

pub(crate) fn canonical_fully_decomposed(c: char) -> Option<&'static [char]> {
    mph_lookup(c.into(), CANONICAL_DECOMPOSED_SALT, CANONICAL_DECOMPOSED_KV,
        pair_lookup_fk, pair_lookup_fv_opt, None)
}

pub(crate) fn compatibility_fully_decomposed(c: char) -> Option<&'static [char]> {
    mph_lookup(c.into(), COMPATIBILITY_DECOMPOSED_SALT, COMPATIBILITY_DECOMPOSED_KV,
        pair_lookup_fk, pair_lookup_fv_opt, None)
}

/// Return whether the given character is a combining mark (`General_Category=Mark`)
pub fn is_combining_mark(c: char) -> bool {
    mph_lookup(c.into(), COMBINING_MARK_SALT, COMBINING_MARK_KV,
        bool_lookup_fk, bool_lookup_fv, false)
}

pub fn stream_safe_trailing_nonstarters(c: char) -> usize {
    mph_lookup(c.into(), TRAILING_NONSTARTERS_SALT, TRAILING_NONSTARTERS_KV,
        u8_lookup_fk, u8_lookup_fv, 0) as usize
}
