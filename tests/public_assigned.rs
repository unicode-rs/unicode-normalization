//! Tests for `is_public_assigned`. These embed knowledge of the current
//! version of Unicode and may need to be updated for future versions.

use unicode_normalization::char::is_public_assigned;

#[test]
fn test_public_assigned() {
    // Misc assigned.
    assert!(is_public_assigned('\0'));
    assert!(is_public_assigned('a'));
    assert!(is_public_assigned('\u{7f}'));
    assert!(is_public_assigned('\u{80}'));
    assert!(!is_public_assigned('\u{9e4}'));
    assert!(!is_public_assigned('\u{fdcf}'));

    // Around the first unassigned non-private-use.
    assert!(is_public_assigned('\u{377}'));
    assert!(!is_public_assigned('\u{378}'));
    assert!(!is_public_assigned('\u{379}'));
    assert!(is_public_assigned('\u{37a}'));
    assert!(is_public_assigned('\u{37f}'));

    // Around the last assigned non-private-use.
    assert!(!is_public_assigned('\u{e00ff}'));
    assert!(is_public_assigned('\u{e0100}'));
    assert!(is_public_assigned('\u{e01ef}'));
    assert!(!is_public_assigned('\u{e01f0}'));

    // Private-Use areas
    assert!(!is_public_assigned('\u{e000}'));
    assert!(!is_public_assigned('\u{f8ff}'));
    assert!(!is_public_assigned('\u{f0000}'));
    assert!(!is_public_assigned('\u{ffffd}'));
    assert!(!is_public_assigned('\u{100000}'));
    assert!(!is_public_assigned('\u{10fffd}'));

    // Noncharacters are considered unassigned.
    assert!(!is_public_assigned('\u{fdd0}'));
    assert!(!is_public_assigned('\u{fdef}'));
    assert!(!is_public_assigned('\u{fffe}'));
    assert!(!is_public_assigned('\u{ffff}'));
    assert!(!is_public_assigned('\u{1fffe}'));
    assert!(!is_public_assigned('\u{1ffff}'));
    assert!(!is_public_assigned('\u{2fffe}'));
    assert!(!is_public_assigned('\u{2ffff}'));
    assert!(!is_public_assigned('\u{3fffe}'));
    assert!(!is_public_assigned('\u{3ffff}'));
    assert!(!is_public_assigned('\u{4fffe}'));
    assert!(!is_public_assigned('\u{4ffff}'));
    assert!(!is_public_assigned('\u{5fffe}'));
    assert!(!is_public_assigned('\u{5ffff}'));
    assert!(!is_public_assigned('\u{6fffe}'));
    assert!(!is_public_assigned('\u{6ffff}'));
    assert!(!is_public_assigned('\u{7fffe}'));
    assert!(!is_public_assigned('\u{7ffff}'));
    assert!(!is_public_assigned('\u{8fffe}'));
    assert!(!is_public_assigned('\u{8ffff}'));
    assert!(!is_public_assigned('\u{9fffe}'));
    assert!(!is_public_assigned('\u{9ffff}'));
    assert!(!is_public_assigned('\u{afffe}'));
    assert!(!is_public_assigned('\u{affff}'));
    assert!(!is_public_assigned('\u{bfffe}'));
    assert!(!is_public_assigned('\u{bffff}'));
    assert!(!is_public_assigned('\u{cfffe}'));
    assert!(!is_public_assigned('\u{cffff}'));
    assert!(!is_public_assigned('\u{dfffe}'));
    assert!(!is_public_assigned('\u{dffff}'));
    assert!(!is_public_assigned('\u{efffe}'));
    assert!(!is_public_assigned('\u{effff}'));
    assert!(!is_public_assigned('\u{ffffe}'));
    assert!(!is_public_assigned('\u{fffff}'));
    assert!(!is_public_assigned('\u{10fffe}'));
    assert!(!is_public_assigned('\u{10ffff}'));
}
