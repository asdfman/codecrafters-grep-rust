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
}
