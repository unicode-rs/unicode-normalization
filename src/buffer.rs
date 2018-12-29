// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// There's no particular science behind this number, but it seems to work
// relatively well in ad-hoc benchmarks on a 2012 Macbook Pro.
const INLINE_MAX: usize = 4;

// A struct optimized for FIFO queuing of very small amounts of data, supporting
// `push_back` and `pop_front` operations as well as a `mutate_in_place`
// operation used in practice for sorting.
//
// Stores the first INLINE_MAX entries inline, and then spills to a Vec;
// will only provide any advantage if spilling is unlikely.
//
// The implementation is a sort of compromise between a ring buffer and a plain
// array. We increment a `front` pointer on `pop_front` rather than move
// everything, but we don't use a true ring buffer because that would make
// sorting pretty complex. Instead we just compact `front` back to zero
// periodically. (Since sorting is only used for decomposition, not
// recomposition, this means we're more optimized for the former currently.)
#[derive(Clone)]
pub struct Buffer<T> {
    back: usize,
    front: usize,
    data: [T; INLINE_MAX],
    spill: Vec<T>,
}

impl<T: Copy + Default> Buffer<T> {
    #[inline]
    pub fn new() -> Buffer<T> {
        Buffer {
            // Inline storage, used for fast path.
            data: [T::default(); INLINE_MAX],

            // Logical extension of inline storage. If the number of items is,
            // and has always been, <= INLINE_MAX, `spill` is empty.
            // Otherwise, `spill` contains all items, and `data` contains a copy
            // of between 1 and INLINE_MAX of them (where values before `front`
            // are garbage in both `spill` and `data`).
            //
            // (We need the redundancy in the spilled case so that we can make a
            // single slice available for sorting).
            spill: Vec::new(),

            // Index of first item in buffer (or zero if buffer is empty).
            // Always found in `data`.
            front: 0,

            // Index where next item will be inserted in buffer, either in `data`
            // or in `spill`.
            back: 0,
        }
    }

    #[inline]
    pub fn mutate_in_place<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut [T]),
    {
        if self.back <= INLINE_MAX {
            // Not spilled (fast path)
            f(&mut self.data[self.front..self.back]);
        } else if self.size() <= INLINE_MAX {
            // Spilled, can unspill (might as well since it's a copy either way)
            self.unspill();
            f(&mut self.data[0..self.back]);
        } else {
            // Spilled, have to stay that way
            f(&mut self.spill[self.front..self.back]);
            self.data
                .copy_from_slice(&self.spill[self.front..self.front + INLINE_MAX]);
        }
    }

    pub fn push_back(&mut self, item: T) {
        if self.back < INLINE_MAX {
            // Fast path
            self.data[self.back] = item;
        } else if self.size() < INLINE_MAX && self.back == INLINE_MAX {
            // Non-allocating fallback
            self.shrink_inline();
            self.data[self.back] = item;
        } else if self.back == INLINE_MAX {
            // Allocating fallback
            self.spill(item);
        } else {
            // Already spilled
            self.spill.push(item);
        }

        self.back += 1;
    }

    #[inline]
    pub fn mutate_and_push_back<F>(&mut self, item: T, mut f: F)
    where
        F: FnMut(&mut [T]),
    {
        if self.back < INLINE_MAX {
            // Fast path
            f(&mut self.data[self.front..self.back]);
            self.data[self.back] = item;
        } else if self.size() < INLINE_MAX && self.back == INLINE_MAX {
            // Non-allocating fallback
            self.shrink_inline();
            f(&mut self.data[0..self.back]);
            self.data[self.back] = item;
        } else if self.size() < INLINE_MAX {
            // Spilled, can unspill (might as well since it's a copy either way)
            self.unspill();
            f(&mut self.data[0..self.back]);
            self.data[self.back] = item;
        } else if self.back == INLINE_MAX {
            // Allocating fallback
            f(&mut self.data);
            self.spill(item);
        } else {
            // Already spilled
            f(&mut self.spill[self.front..self.back]);
            self.data
                .copy_from_slice(&self.spill[self.front..self.front + INLINE_MAX]);
            self.spill.push(item);
        }

        self.back += 1;
    }

    #[inline]
    pub fn pop_front(&mut self) -> Option<T> {
        match self.size() {
            0 => None,
            1 => {
                // Fast path when the buffer has exactly one item - avoid
                // moving the front pointer forward to avoid unnecessary
                // compaction
                self.back = self.front;
                Some(self.data[self.front])
            }
            _ => {
                // Potentially slower path, if we need to compact
                if self.front == INLINE_MAX - 1 {
                    self.compact();
                }
                let result = self.data[self.front];
                self.front += 1;
                Some(result)
            }
        }
    }

    #[inline]
    fn spill(&mut self, item: T) {
        debug_assert!(self.back == INLINE_MAX && self.size() == INLINE_MAX);
        self.spill.clear();
        self.spill.reserve(INLINE_MAX + 1);
        self.spill.extend_from_slice(&self.data);
        self.spill.push(item);
    }

    #[inline]
    fn compact(&mut self) {
        debug_assert!(self.front > 0 && self.back >= INLINE_MAX);
        if self.back == INLINE_MAX {
            self.shrink_inline();
        } else if self.size() <= INLINE_MAX {
            self.unspill();
        } else {
            self.shrink_spilled();
        }
    }

    #[inline]
    fn shrink_inline(&mut self) {
        debug_assert!(self.back <= INLINE_MAX && self.size() <= INLINE_MAX);
        for i in self.front..self.back {
            self.data[i - self.front] = self.data[i];
        }
        self.back -= self.front;
        self.front = 0;
    }

    #[inline]
    fn unspill(&mut self) {
        debug_assert!(self.back > INLINE_MAX && self.size() <= INLINE_MAX);
        let size = self.size();
        self.data[0..size].copy_from_slice(&self.spill[self.front..self.back]);
        self.spill.clear();
        self.back = size;
        self.front = 0;
    }

    #[inline]
    fn shrink_spilled(&mut self) {
        debug_assert!(self.back > INLINE_MAX && self.size() > INLINE_MAX);
        self.data
            .copy_from_slice(&self.spill[self.front..self.front + INLINE_MAX]);
        self.spill.drain(0..self.front);
        self.back -= self.front;
        self.front = 0;
    }

    #[inline]
    fn size(&self) -> usize {
        self.back - self.front
    }
}

#[cfg(test)]
mod tests {
    use super::{Buffer, INLINE_MAX};

    #[test]
    fn test_buffer_is_fifo() {
        let mut buffer: Buffer<usize> = Buffer::new();
        assert_eq!(None, buffer.pop_front());
        for i in 0..INLINE_MAX * 3 {
            buffer.push_back(i);
        }
        for i in 0..INLINE_MAX * 3 {
            assert_eq!(Some(i), buffer.pop_front());
        }
        assert_eq!(None, buffer.pop_front());
    }

    #[test]
    fn test_buffer_sort_in_place() {
        let mut buffer: Buffer<usize> = Buffer::new();
        for i in 0..INLINE_MAX * 3 {
            for j in 0..i {
                buffer.push_back(i - j - 1);
            }
            buffer.mutate_in_place(|data| data.sort());
            for j in 0..i {
                assert_eq!(Some(j), buffer.pop_front());
            }
        }
    }

    #[test]
    fn test_buffer_sort_before_push() {
        let mut buffer: Buffer<usize> = Buffer::new();
        for i in 0..INLINE_MAX * 3 {
            let inputs = (0..i).map(|j| i - j - 1).collect::<Vec<_>>();

            for j in &*inputs {
                buffer.mutate_and_push_back(*j, |data| data.sort());
            }
            let results = (0..i)
                .map(|_| buffer.pop_front().unwrap())
                .collect::<Vec<_>>();

            let mut expected = inputs.clone();
            if i > 1 {
                expected[0..i - 1].sort();
            }
            assert_eq!(expected, results);
        }
    }
}
