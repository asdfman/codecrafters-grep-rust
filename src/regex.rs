use crate::{
    match_context::{Captures, MatchContext},
    parse::assign_capture_indices,
    pattern::{Pattern, Quantifier},
};

#[derive(Debug)]
pub struct Regex {
    pub patterns: Vec<Pattern>,
    pub capture_group_count: usize,
}

impl Regex {
    pub fn parse(s: &str) -> Self {
        let mut next_index = 1;
        let mut patterns = Pattern::parse(s);
        for p in patterns.iter_mut() {
            assign_capture_indices(p, &mut next_index);
        }
        Self {
            patterns,
            capture_group_count: next_index - 1,
        }
    }

    pub fn matches(&self, input: &str) -> bool {
        let mut ctx = MatchContext::new(input);
        let captures = Captures::new(self.capture_group_count);
        if let Some(Pattern::StartOfString) = self.patterns.first() {
            return try_match(&mut ctx, &self.patterns[1..], captures).is_some();
        }
        for (cur, _) in input.char_indices() {
            if cur > 0 {
                return false;
            }
            ctx.reset_to(cur);
            if try_match(&mut ctx, &self.patterns, captures.clone()).is_some() {
                return true;
            }
        }
        false
    }
}

fn try_match(
    ctx: &mut MatchContext,
    patterns: &[Pattern],
    captures: Captures,
) -> Option<(usize, Captures)> {
    let mut total_matched_len = 0;
    for (idx, pattern) in patterns.iter().enumerate() {
        if ctx.input.is_empty() {
            let mut rest = &patterns[idx..];
            while let Some(pat) = rest.first() {
                match pat {
                    Pattern::EndOfString => return Some((total_matched_len, captures)),
                    _ if pat.is_optional() => {
                        rest = &rest[1..];
                        continue;
                    }
                    _ => return None,
                }
            }
            return None;
        }
        if let Pattern::CaptureGroup(c_idx, p) = pattern {
            let cur_offset = ctx.input_offset;
            if let Some((len, mut temp_captures)) = try_match(ctx, p, captures.clone()) {
                temp_captures.capture(cur_offset, cur_offset + len, *c_idx);
                if let Some((rest_len, temp_captures)) =
                    try_match(ctx, &patterns[idx + 1..], temp_captures)
                {
                    return Some((total_matched_len + len + rest_len, temp_captures));
                }
            }
            return None;
        }
        if let Pattern::BackReference(index) = pattern {
            captures.debug_print(ctx.original_input);
            let ref_str = captures.get_capture(*index, ctx.original_input)?;
            if ctx.input.starts_with(ref_str) {
                total_matched_len += ref_str.len();
                ctx.advance(ref_str.len());
                continue;
            } else {
                return None;
            }
        }
        if let Pattern::Alternate(c_idx, p) = pattern {
            let cur_offset = ctx.input_offset;
            for sub_pattern in p {
                if let Some((match_len, mut temp_captures)) =
                    try_match(ctx, sub_pattern, captures.clone())
                {
                    temp_captures.capture(cur_offset, cur_offset + match_len, *c_idx);
                    let rest = &patterns[idx + 1..];
                    if let Some((rest_len, temp_captures)) = try_match(ctx, rest, temp_captures) {
                        return Some((total_matched_len + match_len + rest_len, temp_captures));
                    }
                }
                ctx.reset_to(cur_offset);
            }
            return None;
        }
        if let Pattern::PatternWithQuantifier(inner, quant) = pattern {
            let mut count = 0;
            let mut match_lengths = vec![];
            let cur_offset = ctx.input_offset;
            let min_required_match_count = match quant {
                Quantifier::Literal(n) => *n,
                Quantifier::OneOrMore => 1,
                _ => 0,
            };
            let mut temp_captures = Captures::default();
            loop {
                let prev_offset = ctx.input_offset;
                if let Some((len, sub_captures)) =
                    try_match(ctx, std::slice::from_ref(inner), captures.clone())
                {
                    if len == 0 {
                        ctx.reset_to(prev_offset);
                        break;
                    }
                    match_lengths.push(len);
                    count += 1;
                    temp_captures = sub_captures;
                    match quant {
                        Quantifier::ZeroOrOne => break,
                        Quantifier::Literal(n) if count >= *n => break,
                        _ => {}
                    }
                } else {
                    ctx.reset_to(prev_offset);
                    break;
                }
            }
            match quant {
                Quantifier::ZeroOrOne => {
                    println!("ZeroOrOne quantifier matched {} times", count);
                    let (rest_len, temp_captures) =
                        try_match(ctx, &patterns[idx + 1..], temp_captures)?;
                    dbg!(&patterns[idx + 1..]);
                    return Some((
                        match_lengths.iter().sum::<usize>() + rest_len,
                        temp_captures,
                    ));
                }
                Quantifier::Literal(n) => {
                    if count != *n {
                        return None;
                    }
                    let (rest_len, temp_captures) =
                        try_match(ctx, &patterns[idx + 1..], temp_captures)?;
                    return Some((
                        match_lengths.iter().sum::<usize>() + rest_len,
                        temp_captures,
                    ));
                }
                _ => {}
            }
            while count > min_required_match_count {
                if let Some((rest_len, sub_captures)) =
                    try_match(ctx, &patterns[idx + 1..], temp_captures.clone())
                {
                    return Some((match_lengths.iter().sum::<usize>() + rest_len, sub_captures));
                }
                count -= 1;
                match_lengths.pop().unwrap();
                ctx.reset_to(cur_offset + match_lengths.iter().sum::<usize>());
            }
            return None;
        }
        let match_len = pattern.matches(ctx.input)?;
        total_matched_len += match_len;
        ctx.advance(match_len);
    }
    Some((total_matched_len, captures))
}
