// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::VecDeque;
use std::fmt::{self, Write};
use std::iter::Peekable;

const COMBINING_GRAPHEME_JOINER: char = '\u{034F}';

// Helper functions used for Unicode normalization
fn canonical_sort(comb: &mut VecDeque<(char, u8)>) {
    let len = comb.len();
    for i in 0..len {
        let mut swapped = false;
        for j in 1..len-i {
            let class_a = comb[j-1].1;
            let class_b = comb[j].1;
            if class_a != 0 && class_b != 0 && class_a > class_b {
                comb.swap(j-1, j);
                swapped = true;
            }
        }
        if !swapped { break; }
    }
}

#[derive(Clone)]
enum DecompositionType {
    Canonical,
    Compatible
}

/// External iterator for a string decomposition's characters.
#[derive(Clone)]
pub struct Decompositions<I: Iterator<Item=char>> {
    kind: DecompositionType,
    iter: Peekable<I>,
    buffer: VecDeque<(char, u8)>,
    // True to use the http://unicode.org/reports/tr15/#UAX15-D4 variant of decomposition.
    stream_safe: bool
}

#[inline]
pub fn new_canonical<I: Iterator<Item=char>>(iter: I, stream_safe: bool) -> Decompositions<I> {
    Decompositions {
        iter: iter.peekable(),
        buffer: VecDeque::with_capacity(32),
        kind: self::DecompositionType::Canonical,
        stream_safe: stream_safe,
    }
}

#[inline]
pub fn new_compatible<I: Iterator<Item=char>>(iter: I, stream_safe: bool) -> Decompositions<I> {
    Decompositions {
        iter: iter.peekable(),
        buffer: VecDeque::with_capacity(32),
        kind: self::DecompositionType::Compatible,
        stream_safe: stream_safe,
    }
}

impl<I: Iterator<Item=char>> Iterator for Decompositions<I> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        use self::DecompositionType::*;

        match self.buffer.pop_front() {
            Some((c, _)) => {
                return Some(c);
            }
            _ => {}
        };

        let mut non_starter_count = 0;
        let stream_safe = self.stream_safe;

        while self.iter.peek().is_some() {
            let ch = self.iter.next().unwrap();
            let ch_next = self.iter.peek();

            let buffer = &mut self.buffer;
            {
                let callback = |d| {
                    let class = super::char::canonical_combining_class(d);

                    buffer.push_back((d, class));

                    if stream_safe {
                        if class == 0 {
                            non_starter_count = 0;
                        } else {
                            non_starter_count += 1;
                        }

                        if non_starter_count == 30 {
                            match ch_next {
                                Some(c) => {
                                    if super::char::canonical_combining_class(*c) != 0 {
                                        canonical_sort(buffer);
                                        buffer.push_back((COMBINING_GRAPHEME_JOINER, 0));
                                        non_starter_count = 0;
                                    }
                                }
                                None => {}
                            };
                        }
                    }
                };
                match self.kind {
                    Canonical => {
                        super::char::decompose_canonical(ch, callback)
                    }
                    Compatible => {
                        super::char::decompose_compatible(ch, callback)
                    }
                }
            }

            if match ch_next {
                Some(c) => super::char::canonical_combining_class(*c) == 0,
                None => true,
            } {
                canonical_sort(buffer);
                break
            }
        }

        match self.buffer.pop_front() {
            Some((c, _)) => {
                Some(c)
            }
            _ => None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, _) = self.iter.size_hint();
        (lower, None)
    }
}

impl<I: Iterator<Item=char> + Clone> fmt::Display for Decompositions<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for c in self.clone() {
            f.write_char(c)?;
        }
        Ok(())
    }
}
