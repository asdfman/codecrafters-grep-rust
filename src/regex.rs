use crate::pattern::{Pattern, Quantifier};

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
    for (idx, pattern) in patterns.iter().enumerate() {
        if input.is_empty() {
            if let Pattern::EndOfString = pattern {
                return true;
            } else {
                return false;
            }
        }
        let Some(mut match_len) = pattern.matches(input) else {
            return false;
        };
        if let Pattern::PatternWithQuantifier(_, q) = pattern {
            while match_len > 0 {
                if try_match(&patterns[idx + 1..], &input[match_len..]) {
                    return true;
                } else {
                    match_len -= 1;
                }
            }
            match q {
                Quantifier::OneOrMore => return false,
                Quantifier::Literal(count) if *count > 0 => return false,
                _ => {} // Zero matches ok - just continue
            }
        }
        input = &input[match_len..];
    }
    true
}
