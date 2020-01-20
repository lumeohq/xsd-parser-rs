use super::*;
use utils::split_comment_line;

#[test]
fn comment_splitting_test() {
    assert_eq!(split_comment_line("hello", 80, 0), "// hello\n");
    assert_eq!(split_comment_line("hello world", 10, 0), "// hello\n// world\n");

    // Limitation can con be satisfied, splitting should do its best.
    assert_eq!(split_comment_line("hello world", 3, 0), "// hello\n// world\n");

    assert_eq!(split_comment_line("hello", 80, 2), "  // hello\n");
    assert_eq!(split_comment_line("hello world", 10, 2), "  // hello\n  // world\n");
    assert_eq!(split_comment_line("foo bar", 10, 4), "    // foo\n    // bar\n");
}
