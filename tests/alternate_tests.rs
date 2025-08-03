#[cfg(test)]
mod tests {
    use codecrafters_grep::regex::Regex;

    #[test]
    fn test_simple_alternate_match() {
        let regex = Regex::parse("(cat|dog)");
        assert!(regex.matches("cat"));
        assert!(regex.matches("dog"));
        assert!(!regex.matches("bat"));
    }

    #[test]
    fn test_alternate_with_literals() {
        let regex = Regex::parse("(foo|bar|baz)");
        assert!(regex.matches("foo"));
        assert!(regex.matches("bar"));
        assert!(regex.matches("baz"));
        assert!(!regex.matches("qux"));
    }

    #[test]
    fn test_alternate_with_groups() {
        let regex = Regex::parse("(abc|def|g)");
        assert!(regex.matches("abc"));
        assert!(regex.matches("def"));
        assert!(regex.matches("g"));
        assert!(!regex.matches("ab"));
    }

    #[test]
    fn test_alternate_with_quantifiers() {
        let regex = Regex::parse("(a+|b{2}|c?)");
        assert!(regex.matches("aaa"));
        assert!(regex.matches("bb"));
        assert!(regex.matches("c"));
        assert!(regex.matches("b"));
    }

    #[test]
    fn test_alternate_actual_case() {
        let regex = Regex::parse(r"^I see (\d (cat|dog|cow)s?(, | and )?)+$");
        let input = "I see 1 cat, 2 dogs and 3 cows";
        assert!(regex.matches(input));
    }
}
