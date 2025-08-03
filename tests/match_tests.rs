#[cfg(test)]
mod tests {
    use codecrafters_grep::regex::Regex;

    #[test]
    fn test_literal_match() {
        let regex = Regex::parse("abc");
        assert!(regex.matches("abc"));
        assert!(regex.matches("123abc456"));
        assert!(!regex.matches("ab"));
    }

    #[test]
    fn test_digit_match() {
        let regex = Regex::parse(r"\d");
        assert!(regex.matches("a1b"));
        assert!(!regex.matches("abc"));
    }

    #[test]
    fn test_alphanumeric_match() {
        let regex = Regex::parse(r"\w");
        assert!(regex.matches("abc"));
        assert!(regex.matches("_"));
        assert!(!regex.matches("!!!"));
        assert!(regex.matches("×=#_=×%"));
    }

    #[test]
    fn test_any_match() {
        let regex = Regex::parse(".");
        assert!(regex.matches("a"));
        assert!(regex.matches("1"));
        assert!(!regex.matches(""));
    }

    #[test]
    fn test_start_of_line_anchor() {
        let regex = Regex::parse(r"^dog");
        assert!(regex.matches("dog"));
        assert!(regex.matches("dog house"));
        assert!(!regex.matches("my dog"));
        assert!(!regex.matches("a dog house"));
    }

    #[test]
    fn test_end_of_line_anchor() {
        let regex = Regex::parse(r"dog$");
        assert!(regex.matches("dog"));
        assert!(regex.matches("my dog"));
        assert!(!regex.matches("dog house"));
        assert!(!regex.matches("a dog house"));
    }

    #[test]
    fn test_positive_group_match() {
        let regex = Regex::parse("[abc]");
        assert!(regex.matches("a"));
        assert!(regex.matches("b"));
        assert!(regex.matches("c"));
        assert!(!regex.matches("d"));
        assert!(regex.matches("1a2"));
        assert!(!regex.matches("xyz"));
    }

    #[test]
    fn test_negative_group_match() {
        let regex = Regex::parse("[^abc]");
        assert!(regex.matches("d"));
        assert!(regex.matches("x"));
        assert!(!regex.matches("a"));
        assert!(!regex.matches("b"));
        assert!(!regex.matches("c"));
        assert!(regex.matches("dog"));
        assert!(!regex.matches("cab"));
    }

    #[test]
    fn test_digit_and_word_matching() {
        let regex1 = Regex::parse(r"\d apple");
        assert!(regex1.matches("1 apple"));
        assert!(!regex1.matches("1 orange"));

        let regex2 = Regex::parse(r"\d\d\d apples");
        assert!(regex2.matches("100 apples"));
        assert!(!regex2.matches("1 apple"));

        let regex3 = Regex::parse(r"\d \w\w\ws");
        assert!(regex3.matches("3 dogs"));
        assert!(regex3.matches("4 cats"));
        assert!(!regex3.matches("1 dog"));
    }
}
