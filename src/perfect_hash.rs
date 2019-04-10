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

// This function is based on multiplication being fast and is "good enough". Also
// it can share some work between the unsalted and salted versions.
#[inline]
fn my_hash(key: u32, salt: u32, n: usize) -> usize {
    let y = key.wrapping_add(salt).wrapping_mul(2654435769);
    let y = y ^ key.wrapping_mul(0x31415926);
    (((y as u64) * (n as u64)) >> 32) as usize
}

#[inline]
pub(crate) fn mph_lookup<KV, V, FK, FV>(x: u32, salt: &[u16], kv: &[KV], fk: FK, fv: FV,
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
pub(crate) fn u8_lookup_fk(kv: u32) -> u32 {
    kv >> 8
}

/// Extract the value in a 24 bit key and 8 bit value packed in a u32.
#[inline]
pub(crate) fn u8_lookup_fv(kv: u32) -> u8 {
    (kv & 0xff) as u8
}

/// Extract the key for a boolean lookup.
#[inline]
pub(crate) fn bool_lookup_fk(kv: u32) -> u32 {
    kv
}

/// Extract the value for a boolean lookup.
#[inline]
pub(crate) fn bool_lookup_fv(_kv: u32) -> bool {
    true
}

/// Extract the key in a pair.
#[inline]
pub(crate) fn pair_lookup_fk<T>(kv: (u32, T)) -> u32 {
    kv.0
}

/// Extract the value in a pair, returning an option.
#[inline]
pub(crate) fn pair_lookup_fv_opt<T>(kv: (u32, T)) -> Option<T> {
    Some(kv.1)
}

