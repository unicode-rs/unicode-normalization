use unicode_normalization::UnicodeNormalization;

macro_rules! check_ccs {
    ($input: expr, $expected_out: expr) => {
        assert_eq!(
            $input.correct_defective_ccs().collect::<String>(),
            $expected_out
        )
    };
}

#[test]
fn defective_css() {
    check_ccs!("", "");
    check_ccs!("abcde", "abcde");
    check_ccs!("a\u{0301}bcde", "a\u{0301}bcde");
    check_ccs!("\u{0301}bcde", "\u{00A0}\u{0301}bcde");
    check_ccs!("\u{200C}\u{0301}bcde", "\u{00A0}\u{200C}\u{0301}bcde");
    check_ccs!("\u{200C}bcde", "\u{200C}bcde");
    check_ccs!("\u{180F}bcde", "\u{180F}bcde");
    check_ccs!("\u{FFFF}\u{0301}bcde", "\u{FFFF}\u{00A0}\u{0301}bcde");
    check_ccs!("\u{10FFFD}\u{0301}bcde", "\u{10FFFD}\u{0301}bcde");
    check_ccs!("\u{180F}\u{180F}\u{180F}", "\u{180F}\u{180F}\u{180F}");
    check_ccs!("\u{180F}\u{180F}\u{180F}a", "\u{180F}\u{180F}\u{180F}a");
    check_ccs!(
        "\u{180F}\u{180F}\u{180F}\u{0301}",
        "\u{00A0}\u{180F}\u{180F}\u{180F}\u{0301}"
    );
}
