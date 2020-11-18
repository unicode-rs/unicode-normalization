use unicode_normalization::{
    char::canonical_combining_class, is_nfc, is_nfc_stream_safe, UnicodeNormalization,
};

#[test]
fn test_streamsafe_regression(){
    let input = "\u{342}".repeat(55) + &"\u{344}".repeat(3);
    let nfc_ss = input.chars().nfc().stream_safe().collect::<String>();

    // The result should be NFC:
    assert!(is_nfc(&nfc_ss));
    // and should be stream-safe:
    assert!(is_nfc_stream_safe(&nfc_ss))
}
