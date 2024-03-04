use unicode_normalization::UnicodeNormalization;

const L: char = '\u{1100}';
const L_F: char = '\u{115F}';
const V: char = '\u{1161}';
const V_F: char = '\u{1160}';
const T: char = '\u{11AE}';
const LV: char = '\u{AC00}';
const LVT: char = '\u{AC01}';

macro_rules! standardize {
    ($input: expr) => {
        IntoIterator::into_iter($input)
            .standard_korean_syllables()
            .collect::<Vec<char>>()
    };
}

/// <https://www.unicode.org/reports/tr29/#Korean_Syllable_Break_Examples>
#[test]
fn korean_syllable_break_examples() {
    // LVT LV LV LVf LfV LfVfT
    let orig = [LVT, L, V, LV, L, V_F, L_F, V, L_F, V_F, T];
    assert_eq!(standardize!(orig), orig);

    // LL TT VV TT VV LLVV
    let orig = [L, L, T, T, V, V, T, T, V, V, L, LV, V];
    assert_eq!(
        standardize!(orig),
        [L, L, V_F, L_F, V_F, T, T, L_F, V, V, T, T, L_F, V, V, L, LV, V]
    );
}

#[cfg(feature = "ks_x_1026-1")]
mod ks_x_1026_1 {
    use super::*;
    macro_rules! standardize_ks_x_1026_1 {
        ($input: expr) => {
            IntoIterator::into_iter($input)
                .standard_korean_syllables_ks_x_1026_1()
                .collect::<Vec<char>>()
        };
    }

    /// <http://std.dkuug.dk/jtc1/sc2/wg2/docs/n3422.pdf> § 7.8
    #[test]
    fn korean_syllable_break_examples_ks_x_1026_1() {
        // LVT LV LV LVf LfV LfVfT
        let orig = [LVT, L, V, LV, L, V_F, L_F, V, L_F, V_F, T];
        assert_eq!(standardize_ks_x_1026_1!(orig), orig);

        // L L T T V VT T V V L LV V
        let orig = [L, L, T, T, V, V, T, T, V, V, L, LV, V];
        assert_eq!(
            standardize_ks_x_1026_1!(orig),
            [
                L, V_F, L, V_F, L_F, V_F, T, L_F, V_F, T, L_F, V, L_F, V, T, L_F, V_F, T, L_F, V,
                L_F, V, L, V_F, LV, L_F, V
            ]
        );

        //L LVf LfVfT T LfV VT T LfV V L LV V
        let orig = [
            L, L, V_F, L_F, V_F, T, T, L_F, V, V, T, T, L_F, V, V, L, LV, V,
        ];
        assert_eq!(
            standardize_ks_x_1026_1!(orig),
            [
                L, V_F, L, V_F, L_F, V_F, T, L_F, V_F, T, L_F, V, L_F, V, T, L_F, V_F, T, L_F, V,
                L_F, V, L, V_F, LV, L_F, V
            ]
        );
    }
}
