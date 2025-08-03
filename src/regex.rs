use crate::pattern::Pattern;

#[derive(Debug)]
pub struct Regex {
    pub patterns: Vec<Pattern>,
}

impl Regex {
    pub fn parse(s: &str) -> Self {
        Self {
            patterns: Pattern::parse(s),
        }
    }

    pub fn matches(&self, input: &str) -> bool {
        if let Some(Pattern::StartOfString) = self.patterns.first() {
            return try_match(&self.patterns[1..], input);
        }
        for (cur, _) in input.char_indices() {
            if try_match(&self.patterns, &input[cur..]) {
                return true;
            }
        }
        false
    }
}

fn try_match(patterns: &[Pattern], mut input: &str) -> bool {
    for pattern in patterns {
        if input.is_empty() {
            if let Pattern::EndOfString = pattern {
                return true;
            } else {
                return false;
            }
        }
        let match_len = pattern.matches(input);
        if match_len == 0 {
            return false;
        }
        input = &input[match_len..];
    }
    true
}
