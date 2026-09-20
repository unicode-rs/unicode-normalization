#![no_main]

#[macro_use]
extern crate libfuzzer_sys;

use unicode_normalization::{
    check_nfc_quick, check_nfc_stream_safe_quick, check_nfd_quick, check_nfd_stream_safe_quick,
    check_nfkc_quick, check_nfkd_quick, is_nfc, is_nfc_stream_safe, is_nfd, is_nfd_stream_safe,
    is_nfkc, is_nfkd, UnicodeNormalization,
};

fuzz_target!(|input: String| {
    // The full predicates imply the quick predicates.
    assert_eq!(check_nfc_quick(&input).is_ok(), is_nfc(&input));
    assert_eq!(check_nfd_quick(&input).is_ok(), is_nfd(&input));
    assert_eq!(check_nfkc_quick(&input).is_ok(), is_nfkc(&input));
    assert_eq!(check_nfkd_quick(&input).is_ok(), is_nfkd(&input));
    assert_eq!(
        check_nfc_stream_safe_quick(&input).is_ok(),
        is_nfc_stream_safe(&input)
    );
    assert_eq!(
        check_nfd_stream_safe_quick(&input).is_ok(),
        is_nfd_stream_safe(&input)
    );

    // Check NFC, NFD, NFKC, and NFKD normalization.
    let nfc = input.nfc().collect::<String>();
    assert_eq!(nfc.is_empty(), input.is_empty());
    assert!(check_nfc_quick(&nfc).is_ok());
    assert!(is_nfc(&nfc));

    let nfd = input.nfd().collect::<String>();
    assert!(nfd.len() >= nfc.len());
    assert!(check_nfd_quick(&nfd).is_ok());
    assert!(is_nfd(&nfd));

    let nfkc = input.nfkc().collect::<String>();
    assert_eq!(nfkc.is_empty(), input.is_empty());
    assert!(check_nfkc_quick(&nfkc).is_ok());
    assert!(is_nfkc(&nfkc));

    let nfkd = input.nfkd().collect::<String>();
    assert!(nfkd.len() >= nfkc.len());
    assert!(check_nfkd_quick(&nfkd).is_ok());
    assert!(is_nfkd(&nfkd));

    // Check stream-safe.
    let nfc_ss = nfc.stream_safe().collect::<String>();
    assert!(nfc_ss.len() >= nfc.len());
    assert!(check_nfc_stream_safe_quick(&nfc_ss).is_ok());
    assert!(is_nfc_stream_safe(&nfc_ss));

    let nfd_ss = nfd.stream_safe().collect::<String>();
    assert!(nfd_ss.len() >= nfd.len());
    assert!(check_nfd_stream_safe_quick(&nfd_ss).is_ok());
    assert!(is_nfd_stream_safe(&nfd_ss));

    // Check that NFC and NFD preserve stream-safe.
    let ss_nfc = input.stream_safe().nfc().collect::<String>();
    assert_eq!(ss_nfc.is_empty(), input.is_empty());
    assert!(check_nfc_stream_safe_quick(&ss_nfc).is_ok());
    assert!(is_nfc_stream_safe(&ss_nfc));

    let ss_nfd = input.stream_safe().nfd().collect::<String>();
    assert_eq!(ss_nfd.is_empty(), input.is_empty());
    assert!(check_nfd_stream_safe_quick(&ss_nfd).is_ok());
    assert!(is_nfd_stream_safe(&ss_nfd));
});
