//! Test the extended versions of `nfd`, `nfc`, `nfkc`, and `nfkd`.

use unicode_normalization::UnicodeNormalization;

#[test]
fn test_standardized_variations_for_cjk_singleton_decompositions() {
    // These codepoints have singleton decompositions in the canonical
    // decomposition, and can use standardized variations in the extended
    // decomposition.
    let s = "\u{2f999}\u{2f8a6}";

    let mut nfd_iter = s.chars().nfd();
    assert_eq!(nfd_iter.next(), Some('\u{831d}'));
    assert_eq!(nfd_iter.next(), Some('\u{6148}'));
    assert_eq!(nfd_iter.next(), None);

    let mut nfd_ext_iter = s.chars().nfd_ext();
    assert_eq!(nfd_ext_iter.next(), Some('\u{831d}'));
    assert_eq!(nfd_ext_iter.next(), Some('\u{fe00}'));
    assert_eq!(nfd_ext_iter.next(), Some('\u{6148}'));
    assert_eq!(nfd_ext_iter.next(), Some('\u{fe00}'));
    assert_eq!(nfd_ext_iter.next(), None);

    let mut nfkd_iter = s.chars().nfkd();
    assert_eq!(nfkd_iter.next(), Some('\u{831d}'));
    assert_eq!(nfkd_iter.next(), Some('\u{6148}'));
    assert_eq!(nfkd_iter.next(), None);

    let mut nfkd_ext_iter = s.chars().nfkd_ext();
    assert_eq!(nfkd_ext_iter.next(), Some('\u{831d}'));
    assert_eq!(nfkd_ext_iter.next(), Some('\u{fe00}'));
    assert_eq!(nfkd_ext_iter.next(), Some('\u{6148}'));
    assert_eq!(nfkd_ext_iter.next(), Some('\u{fe00}'));
    assert_eq!(nfkd_ext_iter.next(), None);

    let mut nfc_iter = s.chars().nfc();
    assert_eq!(nfc_iter.next(), Some('\u{831d}'));
    assert_eq!(nfc_iter.next(), Some('\u{6148}'));
    assert_eq!(nfc_iter.next(), None);

    let mut nfc_ext_iter = s.chars().nfc_ext();
    assert_eq!(nfc_ext_iter.next(), Some('\u{831d}'));
    assert_eq!(nfc_ext_iter.next(), Some('\u{fe00}'));
    assert_eq!(nfc_ext_iter.next(), Some('\u{6148}'));
    assert_eq!(nfc_ext_iter.next(), Some('\u{fe00}'));
    assert_eq!(nfc_ext_iter.next(), None);

    let mut nfkc_iter = s.chars().nfkc();
    assert_eq!(nfkc_iter.next(), Some('\u{831d}'));
    assert_eq!(nfkc_iter.next(), Some('\u{6148}'));
    assert_eq!(nfkc_iter.next(), None);

    let mut nfkc_ext_iter = s.chars().nfkc_ext();
    assert_eq!(nfkc_ext_iter.next(), Some('\u{831d}'));
    assert_eq!(nfkc_ext_iter.next(), Some('\u{fe00}'));
    assert_eq!(nfkc_ext_iter.next(), Some('\u{6148}'));
    assert_eq!(nfkc_ext_iter.next(), Some('\u{fe00}'));
    assert_eq!(nfkc_ext_iter.next(), None);
}

/// Test that the ext iterators incude the usual NFC/NFD/NFKC/NKFD normalizations.
#[test]
fn test_underlying_nfd_nfc_nfkd_nfkc() {
    let s = "hi\u{212b}\u{4d}\u{3a9}\u{2156}\u{31}\u{2044}\u{33}";

    let mut nfd_ext_iter = s.chars().nfd_ext();
    assert_eq!(nfd_ext_iter.next(), Some('h'));
    assert_eq!(nfd_ext_iter.next(), Some('i'));
    assert_eq!(nfd_ext_iter.next(), Some('\u{41}'));
    assert_eq!(nfd_ext_iter.next(), Some('\u{30a}'));
    assert_eq!(nfd_ext_iter.next(), Some('\u{4d}'));
    assert_eq!(nfd_ext_iter.next(), Some('\u{3a9}'));
    assert_eq!(nfd_ext_iter.next(), Some('\u{2156}'));
    assert_eq!(nfd_ext_iter.next(), Some('\u{31}'));
    assert_eq!(nfd_ext_iter.next(), Some('\u{2044}'));
    assert_eq!(nfd_ext_iter.next(), Some('\u{33}'));
    assert_eq!(nfd_ext_iter.next(), None);

    let mut nfkd_ext_iter = s.chars().nfkd_ext();
    assert_eq!(nfkd_ext_iter.next(), Some('h'));
    assert_eq!(nfkd_ext_iter.next(), Some('i'));
    assert_eq!(nfkd_ext_iter.next(), Some('\u{41}'));
    assert_eq!(nfkd_ext_iter.next(), Some('\u{30a}'));
    assert_eq!(nfkd_ext_iter.next(), Some('\u{4d}'));
    assert_eq!(nfkd_ext_iter.next(), Some('\u{3a9}'));
    assert_eq!(nfkd_ext_iter.next(), Some('\u{32}'));
    assert_eq!(nfkd_ext_iter.next(), Some('\u{2044}'));
    assert_eq!(nfkd_ext_iter.next(), Some('\u{35}'));
    assert_eq!(nfkd_ext_iter.next(), Some('\u{31}'));
    assert_eq!(nfkd_ext_iter.next(), Some('\u{2044}'));
    assert_eq!(nfkd_ext_iter.next(), Some('\u{33}'));
    assert_eq!(nfkd_ext_iter.next(), None);

    let mut nfc_ext_iter = s.chars().nfc_ext();
    assert_eq!(nfc_ext_iter.next(), Some('h'));
    assert_eq!(nfc_ext_iter.next(), Some('i'));
    assert_eq!(nfc_ext_iter.next(), Some('\u{c5}'));
    assert_eq!(nfc_ext_iter.next(), Some('\u{4d}'));
    assert_eq!(nfc_ext_iter.next(), Some('\u{3a9}'));
    assert_eq!(nfc_ext_iter.next(), Some('\u{2156}'));
    assert_eq!(nfc_ext_iter.next(), Some('\u{31}'));
    assert_eq!(nfc_ext_iter.next(), Some('\u{2044}'));
    assert_eq!(nfc_ext_iter.next(), Some('\u{33}'));
    assert_eq!(nfc_ext_iter.next(), None);

    let mut nfkc_ext_iter = s.chars().nfkc_ext();
    assert_eq!(nfkc_ext_iter.next(), Some('h'));
    assert_eq!(nfkc_ext_iter.next(), Some('i'));
    assert_eq!(nfkc_ext_iter.next(), Some('\u{c5}'));
    assert_eq!(nfkc_ext_iter.next(), Some('\u{4d}'));
    assert_eq!(nfkc_ext_iter.next(), Some('\u{3a9}'));
    assert_eq!(nfkc_ext_iter.next(), Some('\u{32}'));
    assert_eq!(nfkc_ext_iter.next(), Some('\u{2044}'));
    assert_eq!(nfkc_ext_iter.next(), Some('\u{35}'));
    assert_eq!(nfkc_ext_iter.next(), Some('\u{31}'));
    assert_eq!(nfkc_ext_iter.next(), Some('\u{2044}'));
    assert_eq!(nfkc_ext_iter.next(), Some('\u{33}'));
    assert_eq!(nfkc_ext_iter.next(), None);
}
