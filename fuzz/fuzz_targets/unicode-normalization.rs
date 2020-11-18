#![no_main]

#[macro_use]
extern crate libfuzzer_sys;

use unicode_normalization::{
    is_nfc, is_nfc_quick, is_nfc_stream_safe, is_nfc_stream_safe_quick, is_nfd, is_nfd_quick,
    is_nfd_stream_safe, is_nfd_stream_safe_quick, is_nfkc, is_nfkc_quick, is_nfkd, is_nfkd_quick,
    IsNormalized, UnicodeNormalization,
};

fn from_bool(is_normalized: bool) -> IsNormalized {
    if is_normalized {
        IsNormalized::Yes
    } else {
        IsNormalized::No
    }
}

fuzz_target!(|input: String| {
    // The full predicates imply the quick predicates.
    assert_ne!(is_nfc_quick(input.chars()), from_bool(!is_nfc(&input)));
    assert_ne!(is_nfd_quick(input.chars()), from_bool(!is_nfd(&input)));
    assert_ne!(is_nfkc_quick(input.chars()), from_bool(!is_nfkc(&input)));
    assert_ne!(is_nfkd_quick(input.chars()), from_bool(!is_nfkd(&input)));
    assert_ne!(
        is_nfc_stream_safe_quick(input.chars()),
        from_bool(!is_nfc_stream_safe(&input))
    );
    assert_ne!(
        is_nfd_stream_safe_quick(input.chars()),
        from_bool(!is_nfd_stream_safe(&input))
    );

    // Check NFC, NFD, NFKC, and NFKD normalization.
    let nfc = input.chars().nfc().collect::<String>();
    assert_eq!(nfc.is_empty(), input.is_empty());
    assert_ne!(is_nfc_quick(nfc.chars()), IsNormalized::No);
    assert!(is_nfc(&nfc));

    let nfd = input.chars().nfd().collect::<String>();
    assert!(nfd.len() >= nfc.len());
    assert_ne!(is_nfd_quick(nfd.chars()), IsNormalized::No);
    assert!(is_nfd(&nfd));

    let nfkc = input.chars().nfkc().collect::<String>();
    assert_eq!(nfkc.is_empty(), input.is_empty());
    assert_ne!(is_nfkc_quick(nfkc.chars()), IsNormalized::No);
    assert!(is_nfkc(&nfkc));

    let nfkd = input.chars().nfkd().collect::<String>();
    assert!(nfkd.len() >= nfkc.len());
    assert_ne!(is_nfkd_quick(nfkd.chars()), IsNormalized::No);
    assert!(is_nfkd(&nfkd));

    // Check stream-safe.
    let nfc_ss = nfc.chars().stream_safe().collect::<String>();
    assert!(nfc_ss.len() >= nfc.len());
    assert_ne!(is_nfc_stream_safe_quick(nfc_ss.chars()), IsNormalized::No);
    assert!(is_nfc_stream_safe(&nfc_ss));

    let nfd_ss = nfd.chars().stream_safe().collect::<String>();
    assert!(nfd_ss.len() >= nfd.len());
    assert_ne!(is_nfd_stream_safe_quick(nfd_ss.chars()), IsNormalized::No);
    assert!(is_nfd_stream_safe(&nfd_ss));

    // Check that NFC and NFD preserve stream-safe.
    let ss_nfc = input.chars().stream_safe().nfc().collect::<String>();
    assert_eq!(ss_nfc.is_empty(), input.is_empty());
    assert_ne!(is_nfc_stream_safe_quick(ss_nfc.chars()), IsNormalized::No);
    assert!(is_nfc_stream_safe(&ss_nfc));

    let ss_nfd = input.chars().stream_safe().nfd().collect::<String>();
    assert_eq!(ss_nfd.is_empty(), input.is_empty());
    assert_ne!(is_nfd_stream_safe_quick(ss_nfd.chars()), IsNormalized::No);
    assert!(is_nfd_stream_safe(&ss_nfd));
});
