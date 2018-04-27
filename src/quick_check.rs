use UnicodeNormalization;
use tables;

pub enum IsNormalized {
    Yes,
    No,
    Maybe,
}

// https://unicode.org/reports/tr15/#Detecting_Normalization_Forms
#[inline]
fn quick_check<F, I>(s: I, is_allowed: F) -> IsNormalized
    where I: Iterator<Item=char>, F: Fn(char) -> IsNormalized
{
    let mut last_cc = 0u8;
    let mut result = IsNormalized::Yes;
    for ch in s {
        // For ASCII we know it's always allowed and a starter
        if ch <= '\x7f' {
            last_cc = 0;
            continue;
        }
        // Otherwise, lookup the combining class and QC property
        let cc = tables::canonical_combining_class(ch);
        if last_cc > cc && cc != 0 {
            return IsNormalized::No;
        }
        match is_allowed(ch) {
            IsNormalized::Yes => (),
            IsNormalized::No => return IsNormalized::No,
            IsNormalized::Maybe => {
                result = IsNormalized::Maybe;
            },
        }
        last_cc = cc;
    }
    result
}

#[inline]
pub fn is_nfc_quick<I: Iterator<Item=char>>(s: I) -> IsNormalized {
    quick_check(s, tables::qc_nfc)
}

#[inline]
pub fn is_nfd_quick<I: Iterator<Item=char>>(s: I) -> IsNormalized {
    quick_check(s, tables::qc_nfd)
}

pub fn is_nfc(s: &str) -> bool {
    match is_nfc_quick(s.chars()) {
        IsNormalized::Yes => true,
        IsNormalized::No => false,
        IsNormalized::Maybe => s.chars().eq(s.chars().nfc()),
    }
}

pub fn is_nfd(s: &str) -> bool {
    match is_nfd_quick(s.chars()) {
        IsNormalized::Yes => true,
        IsNormalized::No => false,
        IsNormalized::Maybe => s.chars().eq(s.chars().nfd()),
    }
}
