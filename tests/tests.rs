extern crate unicode_normalization;
use unicode_normalization::UnicodeNormalization;
use unicode_normalization::__test_api::{
     stream_safe,
};

mod data {
    pub mod normalization_tests;
}
use crate::data::normalization_tests::NORMALIZATION_TESTS;

#[test]
fn test_normalization_tests_unaffected() {
    for test in NORMALIZATION_TESTS {
        for &s in &[test.source, test.nfc, test.nfd, test.nfkc, test.nfkd] {
            assert_eq!(stream_safe(s), s);
        }
    }
}

#[test]
fn test_official() {
    macro_rules! normString {
        ($method: ident, $input: expr) => { $input.$method().collect::<String>() }
    }

    for test in NORMALIZATION_TESTS {
        // these invariants come from the CONFORMANCE section of
        // http://www.unicode.org/Public/UNIDATA/NormalizationTest.txt
        {
            let r1 = normString!(nfc, test.source);
            let r2 = normString!(nfc, test.nfc);
            let r3 = normString!(nfc, test.nfd);
            let r4 = normString!(nfc, test.nfkc);
            let r5 = normString!(nfc, test.nfkd);
            assert_eq!(test.nfc, &r1[..]);
            assert_eq!(test.nfc, &r2[..]);
            assert_eq!(test.nfc, &r3[..]);
            assert_eq!(test.nfkc, &r4[..]);
            assert_eq!(test.nfkc, &r5[..]);
        }

        {
            let r1 = normString!(nfd, test.source);
            let r2 = normString!(nfd, test.nfc);
            let r3 = normString!(nfd, test.nfd);
            let r4 = normString!(nfd, test.nfkc);
            let r5 = normString!(nfd, test.nfkd);
            assert_eq!(test.nfd, &r1[..]);
            assert_eq!(test.nfd, &r2[..]);
            assert_eq!(test.nfd, &r3[..]);
            assert_eq!(test.nfkd, &r4[..]);
            assert_eq!(test.nfkd, &r5[..]);
        }

        {
            let r1 = normString!(nfkc, test.source);
            let r2 = normString!(nfkc, test.nfc);
            let r3 = normString!(nfkc, test.nfd);
            let r4 = normString!(nfkc, test.nfkc);
            let r5 = normString!(nfkc, test.nfkd);
            assert_eq!(test.nfkc, &r1[..]);
            assert_eq!(test.nfkc, &r2[..]);
            assert_eq!(test.nfkc, &r3[..]);
            assert_eq!(test.nfkc, &r4[..]);
            assert_eq!(test.nfkc, &r5[..]);
        }

        {
            let r1 = normString!(nfkd, test.source);
            let r2 = normString!(nfkd, test.nfc);
            let r3 = normString!(nfkd, test.nfd);
            let r4 = normString!(nfkd, test.nfkc);
            let r5 = normString!(nfkd, test.nfkd);
            assert_eq!(test.nfkd, &r1[..]);
            assert_eq!(test.nfkd, &r2[..]);
            assert_eq!(test.nfkd, &r3[..]);
            assert_eq!(test.nfkd, &r4[..]);
            assert_eq!(test.nfkd, &r5[..]);
        }
    }
}

#[test]
fn test_quick_check() {
    use unicode_normalization::__test_api::quick_check;
    for test in NORMALIZATION_TESTS {
        assert!(quick_check::is_nfc(test.nfc));
        assert!(quick_check::is_nfd(test.nfd));
        assert!(quick_check::is_nfkc(test.nfkc));
        assert!(quick_check::is_nfkd(test.nfkd));
        if test.nfc != test.nfd {
            assert!(!quick_check::is_nfc(test.nfd));
            assert!(!quick_check::is_nfd(test.nfc));
        }
        if test.nfkc != test.nfc {
            assert!(!quick_check::is_nfkc(test.nfc));
            assert!(quick_check::is_nfc(test.nfkc));
        }
        if test.nfkd != test.nfd {
            assert!(!quick_check::is_nfkd(test.nfd));
            assert!(quick_check::is_nfd(test.nfkd));
        }
    }
}
