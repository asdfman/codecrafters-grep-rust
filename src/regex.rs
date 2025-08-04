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
            return try_match(&self.patterns[1..], input).is_some();
        }
        for (cur, _) in input.char_indices() {
            if try_match(&self.patterns, &input[cur..]).is_some() {
                return true;
            }
        }
        false
    }
}

fn try_match(patterns: &[Pattern], mut input: &str) -> Option<usize> {
    let mut total_matched_len = 0;
    for (idx, pattern) in patterns.iter().enumerate() {
        if input.is_empty() {
            let mut rest = &patterns[idx..];
            while let Some(pat) = rest.first() {
                match pat {
                    Pattern::EndOfString => return Some(total_matched_len),
                    _ if pat.is_optional() => {
                        rest = &rest[1..];
                        continue;
                    }
                    _ => return None,
                }
            }
            return None;
        }
        if let Pattern::CaptureGroup(p) = pattern {
            let mut matched_len = 0;
            let mut group_input = input;
            for sub_pattern in p {
                if let Some(len) = try_match(std::slice::from_ref(sub_pattern), group_input) {
                    matched_len += len;
                    group_input = &group_input[len..];
                } else {
                    if sub_pattern.is_optional() {
                        continue;
                    }
                    return None;
                }
            }
            total_matched_len += matched_len;
            input = &input[matched_len..];
            continue;
        }
        if let Pattern::Alternate(p) = pattern {
            for sub_pattern in p {
                if let Some(match_len) = try_match(sub_pattern, input) {
                    let rest = &patterns[idx + 1..];
                    if let Some(rest_len) = try_match(rest, &input[match_len..]) {
                        return Some(total_matched_len + match_len + rest_len);
                    }
                }
            }
            return None;
        }
        if let Pattern::PatternWithQuantifier(inner, quant) = pattern {
            let mut count = 0;
            let mut match_lengths = vec![];
            let mut rest = input;
            while let Some(len) = try_match(std::slice::from_ref(inner), rest) {
                if len == 0 {
                    break;
                }
                match_lengths.push(len);
                rest = &rest[len..];
                count += 1;
                match quant {
                    Quantifier::ZeroOrOne => break,
                    Quantifier::Literal(n) if count >= *n => break,
                    _ => {}
                }
            }
            let mut min_required_match_count = 0;
            match quant {
                Quantifier::ZeroOrOne => {
                    let rest_len = try_match(&patterns[idx + 1..], rest)?;
                    return Some(match_lengths.iter().sum::<usize>() + rest_len);
                }
                Quantifier::Literal(n) => {
                    if count != *n {
                        return None;
                    }
                    let rest_len = try_match(&patterns[idx + 1..], rest)?;
                    return Some(match_lengths.iter().sum::<usize>() + rest_len);
                }
                Quantifier::OneOrMore => min_required_match_count = 1,
                _ => {}
            }
            if count < min_required_match_count {
                return None;
            }
            let mut rest_len = try_match(&patterns[idx + 1..], rest);
            while rest_len.is_none() && count > min_required_match_count {
                count -= 1;
                match_lengths.pop().unwrap();
                rest = &input[match_lengths.iter().sum()..];
                rest_len = try_match(&patterns[idx + 1..], rest);
            }
            return Some(match_lengths.iter().sum::<usize>() + rest_len?);
        }
        let match_len = pattern.matches(input)?;
        total_matched_len += match_len;
        input = &input[match_len..];
    }
    Some(total_matched_len)
}
