extern crate unicode_normalization;
#[cfg(bloaty_test)]
#[path = "../src/normalization_tests.rs"]
mod normalization_tests;
#[cfg(bloaty_test)]
use unicode_normalization::UnicodeNormalization;

#[cfg(bloaty_test)]
#[test]
fn test_official() {
    use normalization_tests::NORMALIZATION_TESTS;
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

