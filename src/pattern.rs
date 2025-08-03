#[derive(Debug)]
pub enum Pattern {
    Literal(char),
    Digit,
    Alphanumeric,
    Any,
    PositiveGroup(Vec<Pattern>),
    NegativeGroup(Vec<Pattern>),
    Alternate(Vec<Vec<Pattern>>),
    StartOfString,
    EndOfString,
    OneOrMore,
    ZeroOrOne,
    ZeroOrMore,
    LiteralQuantifier(usize),
    PatternWithQuantifier(Box<Pattern>, Quantifier),
}

#[derive(Debug)]
pub enum Quantifier {
    ZeroOrOne,
    ZeroOrMore,
    OneOrMore,
    Literal(usize),
}

impl Pattern {
    pub fn matches(&self, s: &str) -> Option<usize> {
        match self {
            Self::PatternWithQuantifier(p, quantifier) => p.handle_quantifier(quantifier, s),
            _ => (self.char_matches(s) > 0).then_some(1),
        }
    }

    fn char_matches(&self, s: &str) -> usize {
        let Some(c) = s.chars().next() else {
            return 0;
        };
        if match self {
            Self::Literal(p) => c == *p,
            Self::Digit => c.is_ascii_digit(),
            Self::Alphanumeric => c.is_ascii_alphanumeric() || c == '_',
            Self::Any => true,
            Self::PositiveGroup(p) => p.iter().any(|x| x.char_matches(s) > 0),
            Self::NegativeGroup(p) => !p.iter().any(|x| x.char_matches(s) > 0),
            _ => false,
        } {
            return 1;
        }
        0
    }

    fn handle_quantifier(&self, quantifier: &Quantifier, s: &str) -> Option<usize> {
        match quantifier {
            Quantifier::ZeroOrOne => Some(self.char_matches(s)),
            Quantifier::ZeroOrMore => Some(self.match_count(s)),
            Quantifier::OneOrMore => {
                let match_count = self.match_count(s);
                (match_count > 0).then_some(match_count)
            }
            Quantifier::Literal(n) => {
                let match_count = self.match_count(s);
                (match_count == *n).then_some(match_count)
            }
        }
    }

    fn match_count(&self, s: &str) -> usize {
        let mut count = 0;
        for (idx, _) in s.char_indices() {
            if self.char_matches(&s[idx..]) == 0 {
                break;
            }
            count += 1;
        }
        count
    }
}
