#[cfg(test)]
mod tests {
    use codecrafters_grep::regex::Regex;

    #[test]
    fn test_zero_or_one_quantifier() {
        let regex = Regex::parse("a?");
        assert!(regex.matches("a"));
        assert!(regex.matches("ab"));
        assert!(regex.matches("bb"));
    }

    #[test]
    fn test_zero_or_more_quantifier() {
        let regex = Regex::parse("a*");
        assert!(regex.matches("a"));
        assert!(regex.matches("aaaa"));
        assert!(regex.matches("aaab"));
    }

    #[test]
    fn test_one_or_more_quantifier() {
        let regex = Regex::parse("ca+at");
        assert!(regex.matches("caat"));
        assert!(regex.matches("caaats"));
    }

    #[test]
    fn test_literal_quantifier() {
        let regex = Regex::parse("a{3}");
        assert!(!regex.matches("a"));
        assert!(!regex.matches("aa"));
        assert!(regex.matches("aaa"));
        assert!(regex.matches("aaab"));
    }
    #[test]
    fn test_two_byte_characters() {
        let regex = Regex::parse("g.+gol");
        assert!(regex.matches("goøö0Ogol"));
    }
}
