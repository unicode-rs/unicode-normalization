//! Test that the NFC iterator flushes its buffer immediately after seeting a
//! '\n' or '\u{34f}' (CGJ), as those characters never compose with anything.

use unicode_normalization::UnicodeNormalization;

/// Iterate through a given data buffer, expecting to stop at a given stop index.
struct Limited {
    data: Vec<char>,
    stop: usize,
    i: usize,
}

impl Limited {
    fn new(data: Vec<char>, stop: usize) -> Self {
        Self { data, stop, i: 0 }
    }
}

impl Iterator for Limited {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        assert!(self.i < self.stop, "next() called too many times");
        let result = self.data.get(self.i);
        self.i += 1;
        result.copied()
    }
}

#[test]
fn test_nfc_nobuffer_newline() {
    let mut s = Limited::new(vec!['a', 'b', 'c', '\n', 'd', 'e', 'f'], 4).nfc();
    assert_eq!(s.next(), Some('a'));
    assert_eq!(s.next(), Some('b'));
    assert_eq!(s.next(), Some('c'));
    assert_eq!(s.next(), Some('\n'));
}

#[test]
fn test_nfc_nobuffer_newline_at_start() {
    let mut s = Limited::new(vec!['\n', 'd', 'e', 'f'], 1).nfc();
    assert_eq!(s.next(), Some('\n'));
}

#[test]
fn test_nfc_nobuffer_newline_after_combine() {
    let mut s = Limited::new(vec!['A', '\u{30a}', '\n', 'd', 'e', 'f'], 3).nfc();
    assert_eq!(s.next(), Some('\u{c5}'));
    assert_eq!(s.next(), Some('\n'));
}

#[test]
fn test_nfc_nobuffer_newline_after_block() {
    let mut s = Limited::new(vec![',', ')', '\u{30f}', '\n', '\u{30f}'], 4).nfc();
    assert_eq!(s.next(), Some(','));
    assert_eq!(s.next(), Some(')'));
    assert_eq!(s.next(), Some('\u{30f}'));
    assert_eq!(s.next(), Some('\n'));
}

#[test]
fn test_nfc_nobuffer_cgj() {
    let mut s = Limited::new(vec!['a', 'b', 'c', '\u{34f}', 'd', 'e', 'f'], 4).nfc();
    assert_eq!(s.next(), Some('a'));
    assert_eq!(s.next(), Some('b'));
    assert_eq!(s.next(), Some('c'));
    assert_eq!(s.next(), Some('\u{34f}'));
}

#[test]
fn test_nfc_nobuffer_cgj_at_start() {
    let mut s = Limited::new(vec!['\u{34f}', 'd', 'e', 'f'], 1).nfc();
    assert_eq!(s.next(), Some('\u{34f}'));
}

#[test]
fn test_nfc_nobuffer_cgj_after_combine() {
    let mut s = Limited::new(vec!['A', '\u{30a}', '\u{34f}', 'd', 'e', 'f'], 3).nfc();
    assert_eq!(s.next(), Some('\u{c5}'));
    assert_eq!(s.next(), Some('\u{34f}'));
}

#[test]
fn test_nfc_nobuffer_cgj_after_block() {
    let mut s = Limited::new(vec![',', ')', '\u{30f}', '\u{34f}', '\u{30f}'], 4).nfc();
    assert_eq!(s.next(), Some(','));
    assert_eq!(s.next(), Some(')'));
    assert_eq!(s.next(), Some('\u{30f}'));
    assert_eq!(s.next(), Some('\u{34f}'));
}
