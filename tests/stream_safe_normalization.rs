extern crate unicode_normalization;
use unicode_normalization::__test_api::{
     stream_safe,
};

mod normalization_tests;
use normalization_tests::NORMALIZATION_TESTS;

#[test]
fn test_normalization_tests_unaffected() {
    for test in NORMALIZATION_TESTS {
        for &s in &[test.source, test.nfc, test.nfd, test.nfkc, test.nfkd] {
            assert_eq!(stream_safe(s), s);
        }
    }
}

