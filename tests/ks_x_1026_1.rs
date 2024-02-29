#![cfg(feature = "ks_x_1026-1")]

use unicode_normalization::UnicodeNormalization;

macro_rules! norm_string {
    ($method: ident, $input: expr) => {
        $input.$method().collect::<String>()
    };
}

/// § 6.2
#[test]
fn compatibility_and_halfwidth_hangul_letters() {
    // Compatibility
    let orig = "\u{3131}\u{314F}";
    assert_eq!(norm_string!(nfkd, orig), "\u{1100}\u{1161}");
    assert_eq!(norm_string!(nfkc, orig), "\u{AC00}");
    assert_eq!(
        norm_string!(nfkd_ks_x_1026_1, orig),
        "\u{1100}\u{1160}\u{115F}\u{1161}"
    );
    assert_eq!(
        norm_string!(nfkc_ks_x_1026_1, orig),
        "\u{1100}\u{1160}\u{115F}\u{1161}"
    );

    // Halfwidth
    let orig = "\u{FFA1}\u{FFC6}";
    assert_eq!(norm_string!(nfd, orig), "\u{FFA1}\u{FFC6}");
    assert_eq!(norm_string!(nfc, orig), "\u{FFA1}\u{FFC6}");
    assert_eq!(norm_string!(nfkd, orig), "\u{1100}\u{1165}");
    assert_eq!(norm_string!(nfkc, orig), "\u{AC70}");
    assert_eq!(norm_string!(nfc_ks_x_1026_1, orig), "\u{FFA1}\u{FFC6}");
    assert_eq!(
        norm_string!(nfkd_ks_x_1026_1, orig),
        "\u{1100}\u{1160}\u{115F}\u{1165}"
    );
    assert_eq!(
        norm_string!(nfkc_ks_x_1026_1, orig),
        "\u{1100}\u{1160}\u{115F}\u{1165}"
    );
}

/// § 6.3
#[test]
fn hangul_embedded_symbols() {
    // Circled
    let orig = "\u{3260}";
    assert_eq!(norm_string!(nfd, orig), "\u{3260}");
    assert_eq!(norm_string!(nfc, orig), "\u{3260}");
    assert_eq!(norm_string!(nfkd, orig), "\u{1100}");
    assert_eq!(norm_string!(nfkc, orig), "\u{1100}");
    assert_eq!(norm_string!(nfc_ks_x_1026_1, orig), "\u{3260}");
    assert_eq!(norm_string!(nfkd_ks_x_1026_1, orig), "\u{1100}\u{1160}");
    assert_eq!(norm_string!(nfkc_ks_x_1026_1, orig), "\u{1100}\u{1160}");

    // Parenthesized
    let orig = "\u{3200}";
    assert_eq!(norm_string!(nfd, orig), "\u{3200}");
    assert_eq!(norm_string!(nfc, orig), "\u{3200}");
    assert_eq!(norm_string!(nfkd, orig), "(\u{1100})");
    assert_eq!(norm_string!(nfkc, orig), "(\u{1100})");
    assert_eq!(norm_string!(nfc_ks_x_1026_1, orig), "\u{3200}");
    assert_eq!(norm_string!(nfkd_ks_x_1026_1, orig), "(\u{1100}\u{1160})");
    assert_eq!(norm_string!(nfkc_ks_x_1026_1, orig), "(\u{1100}\u{1160})");
}

/// § 6.4
#[test]
fn hangul_syllable_blocks() {
    let orig = "\u{1100}\u{1161}\u{11EB}";
    assert_eq!(norm_string!(nfd, orig), "\u{1100}\u{1161}\u{11EB}");
    assert_eq!(norm_string!(nfc, orig), "\u{AC00}\u{11EB}");
    assert_eq!(norm_string!(nfkd, orig), "\u{1100}\u{1161}\u{11EB}");
    assert_eq!(norm_string!(nfkc, orig), "\u{AC00}\u{11EB}");
    assert_eq!(
        norm_string!(nfc_ks_x_1026_1, orig),
        "\u{1100}\u{1161}\u{11EB}"
    );
    assert_eq!(
        norm_string!(nfkd_ks_x_1026_1, orig),
        "\u{1100}\u{1161}\u{11EB}"
    );
    assert_eq!(
        norm_string!(nfkc_ks_x_1026_1, orig),
        "\u{1100}\u{1161}\u{11EB}"
    );
}

#[test]
fn non_hangul() {
    let orig = "ab\u{010D}de\u{0301}";
    assert_eq!(norm_string!(nfd, orig), "abc\u{030C}de\u{0301}");
    assert_eq!(norm_string!(nfc, orig), "ab\u{010D}d\u{00E9}");
    assert_eq!(norm_string!(nfkd, orig), "abc\u{030C}de\u{0301}");
    assert_eq!(norm_string!(nfkc, orig), "ab\u{010D}d\u{00E9}");
    assert_eq!(norm_string!(nfc_ks_x_1026_1, orig), "ab\u{010D}d\u{00E9}");
    assert_eq!(
        norm_string!(nfkd_ks_x_1026_1, orig),
        "abc\u{030C}de\u{0301}"
    );
    assert_eq!(norm_string!(nfkc_ks_x_1026_1, orig), "ab\u{010D}d\u{00E9}");
}
