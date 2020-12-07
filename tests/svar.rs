//! Test the standard variation sequence replacements.

use unicode_normalization::UnicodeNormalization;

#[test]
fn test_standardized_variations_for_cjk_singleton_decompositions() {
    // These codepoints have singleton decompositions in the canonical
    // decomposition, and can use standardized variations.
    let s = "\u{2f999}\u{2f8a6}";

    // These codepoints have canonical decompositions.
    let mut nfd_iter = s.chars().nfd();
    assert_eq!(nfd_iter.next(), Some('\u{831d}'));
    assert_eq!(nfd_iter.next(), Some('\u{6148}'));
    assert_eq!(nfd_iter.next(), None);

    let mut nfkd_iter = s.chars().nfkd();
    assert_eq!(nfkd_iter.next(), Some('\u{831d}'));
    assert_eq!(nfkd_iter.next(), Some('\u{6148}'));
    assert_eq!(nfkd_iter.next(), None);

    let mut nfc_iter = s.chars().nfc();
    assert_eq!(nfc_iter.next(), Some('\u{831d}'));
    assert_eq!(nfc_iter.next(), Some('\u{6148}'));
    assert_eq!(nfc_iter.next(), None);

    let mut nfkc_iter = s.chars().nfkc();
    assert_eq!(nfkc_iter.next(), Some('\u{831d}'));
    assert_eq!(nfkc_iter.next(), Some('\u{6148}'));
    assert_eq!(nfkc_iter.next(), None);

    // However they also have standardized variants.
    let mut svar_iter = s.chars().svar();
    assert_eq!(svar_iter.next(), Some('\u{831d}'));
    assert_eq!(svar_iter.next(), Some('\u{fe00}'));
    assert_eq!(svar_iter.next(), Some('\u{6148}'));
    assert_eq!(svar_iter.next(), Some('\u{fe00}'));
    assert_eq!(svar_iter.next(), None);

    // The standardized variants are normalization-stable.
    let mut svar_nfc_iter = s.chars().svar().nfc();
    assert_eq!(svar_nfc_iter.next(), Some('\u{831d}'));
    assert_eq!(svar_nfc_iter.next(), Some('\u{fe00}'));
    assert_eq!(svar_nfc_iter.next(), Some('\u{6148}'));
    assert_eq!(svar_nfc_iter.next(), Some('\u{fe00}'));
    assert_eq!(svar_nfc_iter.next(), None);

    let mut svar_nfd_iter = s.chars().svar().nfd();
    assert_eq!(svar_nfd_iter.next(), Some('\u{831d}'));
    assert_eq!(svar_nfd_iter.next(), Some('\u{fe00}'));
    assert_eq!(svar_nfd_iter.next(), Some('\u{6148}'));
    assert_eq!(svar_nfd_iter.next(), Some('\u{fe00}'));
    assert_eq!(svar_nfd_iter.next(), None);

    let mut svar_nfkc_iter = s.chars().svar().nfkc();
    assert_eq!(svar_nfkc_iter.next(), Some('\u{831d}'));
    assert_eq!(svar_nfkc_iter.next(), Some('\u{fe00}'));
    assert_eq!(svar_nfkc_iter.next(), Some('\u{6148}'));
    assert_eq!(svar_nfkc_iter.next(), Some('\u{fe00}'));
    assert_eq!(svar_nfkc_iter.next(), None);

    let mut svar_nfkd_iter = s.chars().svar().nfkd();
    assert_eq!(svar_nfkd_iter.next(), Some('\u{831d}'));
    assert_eq!(svar_nfkd_iter.next(), Some('\u{fe00}'));
    assert_eq!(svar_nfkd_iter.next(), Some('\u{6148}'));
    assert_eq!(svar_nfkd_iter.next(), Some('\u{fe00}'));
    assert_eq!(svar_nfkd_iter.next(), None);
}

/// `svar` shouldn't decompose Hangul.
#[test]
fn test_svar_hangul() {
    assert_eq!(
        "중국어 (홍콩)".chars().svar().collect::<String>(),
        "중국어 (홍콩)"
    );
}
