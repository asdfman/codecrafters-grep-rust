#[cfg(test)]
mod tests {
    use codecrafters_grep::regex::Regex;

    #[test]
    fn test_backreference_pass() {
        let pattern = Regex::parse(r"(b..s|c..e) here and \1 there");
        assert!(pattern.matches("bugs here and bugs there"));
    }

    #[test]
    fn test_backreference_fail() {
        let pattern = Regex::parse("(cat) and \\1");
        assert!(!pattern.matches("cat and dog"));
    }

    #[test]
    fn test_multiple_backreferences() {
        let pattern = Regex::parse(r"('(cat) and \2') is the same as \1");
        assert!(pattern.matches("'cat and cat' is the same as 'cat and cat'"));
    }

    #[test]
    fn test_final_boss() {
        let pattern = Regex::parse(r"(([abc]+)-([def]+)) is \1, not ([^xyz]+), \2, or \3");
        assert!(pattern.matches("abc-def is abc-def, not efg, abc, or def"));
    }

    #[test]
    fn test_backref_capture_with_quantifier() {
        let pattern = Regex::parse(r"once a (drea+mer), alwaysz? a \1");
        assert!(pattern.matches("once a dreaaamer, always a dreaaamer"));
    }
}
