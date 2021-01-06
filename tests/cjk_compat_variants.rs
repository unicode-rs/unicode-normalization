//! Test the standard variation sequence replacements.

use unicode_normalization::UnicodeNormalization;

#[test]
fn test_cjk_compat_variants() {
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
    let mut var_iter = s.chars().cjk_compat_variants();
    assert_eq!(var_iter.next(), Some('\u{831d}'));
    assert_eq!(var_iter.next(), Some('\u{fe00}'));
    assert_eq!(var_iter.next(), Some('\u{6148}'));
    assert_eq!(var_iter.next(), Some('\u{fe00}'));
    assert_eq!(var_iter.next(), None);

    // The standardized variants are normalization-stable.
    let mut var_nfc_iter = s.chars().cjk_compat_variants().nfc();
    assert_eq!(var_nfc_iter.next(), Some('\u{831d}'));
    assert_eq!(var_nfc_iter.next(), Some('\u{fe00}'));
    assert_eq!(var_nfc_iter.next(), Some('\u{6148}'));
    assert_eq!(var_nfc_iter.next(), Some('\u{fe00}'));
    assert_eq!(var_nfc_iter.next(), None);

    let mut var_nfd_iter = s.chars().cjk_compat_variants().nfd();
    assert_eq!(var_nfd_iter.next(), Some('\u{831d}'));
    assert_eq!(var_nfd_iter.next(), Some('\u{fe00}'));
    assert_eq!(var_nfd_iter.next(), Some('\u{6148}'));
    assert_eq!(var_nfd_iter.next(), Some('\u{fe00}'));
    assert_eq!(var_nfd_iter.next(), None);

    let mut var_nfkc_iter = s.chars().cjk_compat_variants().nfkc();
    assert_eq!(var_nfkc_iter.next(), Some('\u{831d}'));
    assert_eq!(var_nfkc_iter.next(), Some('\u{fe00}'));
    assert_eq!(var_nfkc_iter.next(), Some('\u{6148}'));
    assert_eq!(var_nfkc_iter.next(), Some('\u{fe00}'));
    assert_eq!(var_nfkc_iter.next(), None);

    let mut var_nfkd_iter = s.chars().cjk_compat_variants().nfkd();
    assert_eq!(var_nfkd_iter.next(), Some('\u{831d}'));
    assert_eq!(var_nfkd_iter.next(), Some('\u{fe00}'));
    assert_eq!(var_nfkd_iter.next(), Some('\u{6148}'));
    assert_eq!(var_nfkd_iter.next(), Some('\u{fe00}'));
    assert_eq!(var_nfkd_iter.next(), None);
}

/// `cjk_compat_variants` shouldn't decompose Hangul.
#[test]
fn test_cjk_compat_variants_with_hangul() {
    assert_eq!(
        "중국어 (홍콩)"
            .chars()
            .cjk_compat_variants()
            .collect::<String>(),
        "중국어 (홍콩)"
    );
}
