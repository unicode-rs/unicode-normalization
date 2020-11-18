//! Test that the NFC iterator doesn't run needlessly further ahead of its
//! underlying iterator.
//!
//! The NFC iterator is wrapped around the NFD iterator, and it buffers
//! up combining characters so that it can sort them once it knows it has
//! seen the complete sequence. At that point, it should drain its own
//! buffer before consuming more characters from its inner iterator.
//! This fuzz target defines a custom iterator which records how many
//! times it's called so we can detect if NFC called it too many times.

#![no_main]

#[macro_use]
extern crate libfuzzer_sys;

use std::str::Chars;
use std::cell::RefCell;
use std::rc::Rc;
use unicode_normalization::UnicodeNormalization;

const MAX_NONSTARTERS: u32 = 30;

#[derive(Debug)]
struct Counter<'a> {
    iter: Chars<'a>,
    value: Rc<RefCell<u32>>,
}

impl<'a> Iterator for Counter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        *self.value.borrow_mut() += 1;
        self.iter.next()
    }
}

fuzz_target!(|input: String| {
    let stream_safe = input.chars().stream_safe().collect::<String>();

    let mut value = Rc::new(RefCell::new(0));
    let counter = Counter { iter: stream_safe.chars(), value: Rc::clone(&mut value) };
    for _ in counter.nfc() {
        // Plus 2: one for the starter at the beginning of a sequence, and
        // one for a starter that begins the following sequence.
        assert!(*value.borrow() <= MAX_NONSTARTERS + 2);
        *value.borrow_mut() = 0;
    }
});
