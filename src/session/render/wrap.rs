use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// Wrap `line` into possibly multiple lines according to `width`.
///
/// `width` is supposed to be unicode width, and is required to be positive.
pub fn wrap_line(line: &str, width: usize) -> Vec<&str> {
    let mut wrapped_lines = Vec::new();

    let mut line_width = 0;
    let mut start_idx = 0;

    for (idx, grapheme) in line.grapheme_indices(true) {
        let grapheme_width = grapheme.width();

        if line_width + grapheme_width > width {
            // Last wrapped line ranges from the start grapheme to last grapheme.
            wrapped_lines.push(&line[start_idx..idx]);

            // This wrapped line starts from current grapheme.
            start_idx = idx;
            line_width = grapheme_width;
        } else {
            line_width += grapheme_width;
        }
    }

    // Trailing graphemes
    wrapped_lines.push(&line[start_idx..]);

    wrapped_lines
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_wrap_ascii() {
        let line = "Hello World";
        let width = 11;

        let wrapped_lines = wrap_line(line, width);

        assert_eq!(wrapped_lines.len(), 1);
        assert_eq!(wrapped_lines[0], "Hello World");
    }

    #[test]
    fn one_wrap_ascii() {
        let line = "Hello World";
        let width = 10;

        let wrapped_lines = wrap_line(line, width);

        assert_eq!(wrapped_lines.len(), 2);
        assert_eq!(wrapped_lines[0], "Hello Worl");
        assert_eq!(wrapped_lines[1], "d");
    }

    #[test]
    fn two_wrap_ascii() {
        let line = "Hello World";
        let width = 5;

        let wrapped_lines = wrap_line(line, width);

        assert_eq!(wrapped_lines.len(), 3);
        assert_eq!(wrapped_lines[0], "Hello");
        assert_eq!(wrapped_lines[1], " Worl");
        assert_eq!(wrapped_lines[2], "d");
    }

    #[test]
    fn no_wrap_cjk() {
        let line = "你好 世界";
        let width = 9;

        let wrapped_lines = wrap_line(line, width);

        assert_eq!(wrapped_lines.len(), 1);
        assert_eq!(wrapped_lines[0], "你好 世界");
    }

    #[test]
    fn one_wrap_cjk() {
        let line = "你好 世界";
        let width = 8;

        let wrapped_lines = wrap_line(line, width);

        assert_eq!(wrapped_lines.len(), 2);
        assert_eq!(wrapped_lines[0], "你好 世");
        assert_eq!(wrapped_lines[1], "界");
    }

    #[test]
    fn two_wrap_cjk() {
        let line = "你好 世界";
        let width = 4;

        let wrapped_lines = wrap_line(line, width);

        assert_eq!(wrapped_lines.len(), 3);
        assert_eq!(wrapped_lines[0], "你好");
        assert_eq!(wrapped_lines[1], " 世");
        assert_eq!(wrapped_lines[2], "界");
    }
}
