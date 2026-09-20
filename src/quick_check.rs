use crate::lookups::canonical_combining_class;
use crate::stream_safe;
use crate::tables;
use crate::UnicodeNormalization;

use core::error::Error;
use core::fmt;

/// Error returned when a string is not properly normalized.
#[derive(Clone, Debug)]
pub struct NormalizationError {
    /// String was normal up to this position.
    normal_up_to: usize,
}
impl NormalizationError {
    /// Returns the index in the given string up to which it was properly normalized.
    pub const fn normal_up_to(&self) -> usize {
        self.normal_up_to
    }
}
impl fmt::Display for NormalizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "string was not normalized at position {}",
            self.normal_up_to
        )
    }
}
impl Error for NormalizationError {}

/// Whether additional checking is necessary to verify normalization.
///
/// The QuickCheck algorithm can quickly determine if a text is or isn't
/// normalized without any allocations in many cases, but it has to be able to
/// return `Maybe` when a full decomposition and recomposition is necessary.
#[derive(Debug, Eq, PartialEq)]
pub enum QuickCheck {
    /// The text is definitely normalized.
    Yes,
    /// The text may be normalized.
    Maybe,
}

/// Normalization status of single character.
pub enum IsNormalized {
    /// The text is definitely normalized.
    Yes,
    /// The text is definitely not normalized.
    No,
    /// The text may be normalized.
    Maybe,
}

// https://unicode.org/reports/tr15/#Detecting_Normalization_Forms
#[inline]
fn quick_check<F, I>(
    s: I,
    is_allowed: F,
    stream_safe: bool,
) -> Result<QuickCheck, NormalizationError>
where
    I: Iterator<Item = (usize, char)>,
    F: Fn(char) -> IsNormalized,
{
    let mut last_cc = 0u8;
    let mut nonstarter_count = 0;
    let mut result = QuickCheck::Yes;
    for (idx, ch) in s {
        // For ASCII we know it's always allowed and a starter
        if ch <= '\x7f' {
            last_cc = 0;
            nonstarter_count = 0;
            continue;
        }

        // Otherwise, lookup the combining class and QC property
        let cc = canonical_combining_class(ch);
        if last_cc > cc && cc != 0 {
            return Err(NormalizationError { normal_up_to: idx });
        }
        match is_allowed(ch) {
            IsNormalized::Yes => (),
            IsNormalized::No => return Err(NormalizationError { normal_up_to: idx }),
            IsNormalized::Maybe => {
                result = QuickCheck::Maybe;
            }
        }
        if stream_safe {
            let decomp = stream_safe::classify_nonstarters(ch);

            // If we're above `MAX_NONSTARTERS`, we're definitely *not*
            // stream-safe normalized.
            if nonstarter_count + decomp.leading_nonstarters > stream_safe::MAX_NONSTARTERS {
                return Err(NormalizationError { normal_up_to: idx });
            }
            if decomp.leading_nonstarters == decomp.decomposition_len {
                nonstarter_count += decomp.decomposition_len;
            } else {
                nonstarter_count = decomp.trailing_nonstarters;
            }
        }
        last_cc = cc;
    }
    Ok(result)
}

fn full_check<I: Iterator<Item = (usize, char)>, J: Iterator<Item = char>>(
    check: I,
    normalized: J,
) -> Result<(), NormalizationError> {
    check.zip(normalized).try_for_each(|((idx, lhs), rhs)| {
        if lhs == rhs {
            Ok(())
        } else {
            Err(NormalizationError { normal_up_to: idx })
        }
    })
}

/// Quickly check if a string is in NFC.
#[inline]
pub fn check_nfc_quick(s: &str) -> Result<QuickCheck, NormalizationError> {
    quick_check(s.char_indices(), tables::qc_nfc, false)
}

/// Quickly check if a string is in NFKC.
#[inline]
pub fn check_nfkc_quick(s: &str) -> Result<QuickCheck, NormalizationError> {
    quick_check(s.char_indices(), tables::qc_nfkc, false)
}

/// Quickly check if a string is in NFD.
#[inline]
pub fn check_nfd_quick(s: &str) -> Result<QuickCheck, NormalizationError> {
    quick_check(s.char_indices(), tables::qc_nfd, false)
}

/// Quickly check if a string is in NFKD.
#[inline]
pub fn check_nfkd_quick(s: &str) -> Result<QuickCheck, NormalizationError> {
    quick_check(s.char_indices(), tables::qc_nfkd, false)
}

/// Quickly check if a string is Stream-Safe NFC.
#[inline]
pub fn check_nfc_stream_safe_quick(s: &str) -> Result<QuickCheck, NormalizationError> {
    quick_check(s.char_indices(), tables::qc_nfc, true)
}

/// Quickly check if a string is Stream-Safe NFD.
#[inline]
pub fn check_nfd_stream_safe_quick(s: &str) -> Result<QuickCheck, NormalizationError> {
    quick_check(s.char_indices(), tables::qc_nfd, true)
}

/// Authoritatively check if a string is in NFC.
#[inline]
pub fn check_nfc(s: &str) -> Result<(), NormalizationError> {
    match check_nfc_quick(s)? {
        QuickCheck::Yes => Ok(()),
        QuickCheck::Maybe => full_check(s.char_indices(), s.chars().nfc()),
    }
}

/// Return whether a string is in NFC.
#[inline]
pub fn is_nfc(s: &str) -> bool {
    check_nfc(s).is_ok()
}

/// Authoritatively check if a string is in NFKC.
#[inline]
pub fn check_nfkc(s: &str) -> Result<(), NormalizationError> {
    match check_nfkc_quick(s)? {
        QuickCheck::Yes => Ok(()),
        QuickCheck::Maybe => full_check(s.char_indices(), s.chars().nfkc()),
    }
}

/// Return whether a string is in NFKC.
#[inline]
pub fn is_nfkc(s: &str) -> bool {
    check_nfkc(s).is_ok()
}

/// Authoritatively check if a string is in NFD.
#[inline]
pub fn check_nfd(s: &str) -> Result<(), NormalizationError> {
    match check_nfd_quick(s)? {
        QuickCheck::Yes => Ok(()),
        QuickCheck::Maybe => full_check(s.char_indices(), s.chars().nfd()),
    }
}

/// Return whether a string is in NFD.
#[inline]
pub fn is_nfd(s: &str) -> bool {
    check_nfd(s).is_ok()
}

/// Authoritatively check if a string is in NFKD.
#[inline]
pub fn check_nfkd(s: &str) -> Result<(), NormalizationError> {
    match check_nfkd_quick(s)? {
        QuickCheck::Yes => Ok(()),
        QuickCheck::Maybe => full_check(s.char_indices(), s.chars().nfkd()),
    }
}

/// Return whether a string is in NFKD.
#[inline]
pub fn is_nfkd(s: &str) -> bool {
    check_nfkd(s).is_ok()
}

/// Authoritatively check if a string is Stream-Safe NFC.
#[inline]
pub fn check_nfc_stream_safe(s: &str) -> Result<(), NormalizationError> {
    match check_nfc_stream_safe_quick(s)? {
        QuickCheck::Yes => Ok(()),
        QuickCheck::Maybe => full_check(s.char_indices(), s.chars().stream_safe().nfc()),
    }
}

/// Return whether a string is Stream-Safe NFC.
#[inline]
pub fn is_nfc_stream_safe(s: &str) -> bool {
    check_nfc_stream_safe(s).is_ok()
}

/// Authoritatively check if a string is Stream-Safe NFD.
#[inline]
pub fn check_nfd_stream_safe(s: &str) -> Result<(), NormalizationError> {
    match check_nfd_stream_safe_quick(s)? {
        QuickCheck::Yes => Ok(()),
        QuickCheck::Maybe => full_check(s.char_indices(), s.chars().stream_safe().nfd()),
    }
}

/// Return whether a string is Stream-Safe NFD.
#[inline]
pub fn is_nfd_stream_safe(s: &str) -> bool {
    check_nfd_stream_safe(s).is_ok()
}

#[cfg(test)]
mod tests {
    use super::{check_nfc_stream_safe_quick, check_nfd_stream_safe_quick, QuickCheck};

    #[test]
    fn test_stream_safe_nfd() {
        let okay = "Da\u{031b}\u{0316}\u{0317}\u{0318}\u{0319}\u{031c}\u{031d}\u{0300}\u{0301}\u{0302}\u{0303}\u{0304}\u{0305}\u{0306}\u{0307}\u{0308}\u{0309}\u{030a}\u{030b}\u{030c}\u{030d}\u{030e}\u{030f}\u{0310}\u{0311}\u{0312}\u{0313}\u{0314}\u{0315}\u{031a}ngerzone";
        assert_eq!(check_nfd_stream_safe_quick(okay).unwrap(), QuickCheck::Yes);

        let too_much = "Da\u{031b}\u{0316}\u{0317}\u{0318}\u{0319}\u{031c}\u{031d}\u{031e}\u{0300}\u{0301}\u{0302}\u{0303}\u{0304}\u{0305}\u{0306}\u{0307}\u{0308}\u{0309}\u{030a}\u{030b}\u{030c}\u{030d}\u{030e}\u{030f}\u{0310}\u{0311}\u{0312}\u{0313}\u{0314}\u{0315}\u{031a}ngerzone";
        assert!(check_nfd_stream_safe_quick(too_much).is_err());
    }

    #[test]
    fn test_stream_safe_nfc() {
        let okay = "ok\u{e0}\u{031b}\u{0316}\u{0317}\u{0318}\u{0319}\u{031c}\u{031d}\u{0301}\u{0302}\u{0303}\u{0304}\u{0305}\u{0306}\u{0307}\u{0308}\u{0309}\u{030a}\u{030b}\u{030c}\u{030d}\u{030e}\u{030f}\u{0310}\u{0311}\u{0312}\u{0313}\u{0314}\u{0315}\u{031a}y";
        assert_eq!(
            check_nfc_stream_safe_quick(okay).unwrap(),
            QuickCheck::Maybe
        );

        let too_much = "not ok\u{e0}\u{031b}\u{0316}\u{0317}\u{0318}\u{0319}\u{031c}\u{031d}\u{031e}\u{0301}\u{0302}\u{0303}\u{0304}\u{0305}\u{0306}\u{0307}\u{0308}\u{0309}\u{030a}\u{030b}\u{030c}\u{030d}\u{030e}\u{030f}\u{0310}\u{0311}\u{0312}\u{0313}\u{0314}\u{0315}\u{031a}y";
        assert!(check_nfc_stream_safe_quick(too_much).is_err());
    }
}
