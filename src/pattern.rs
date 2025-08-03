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
    LiteralQuantifier(u64),
    PatternWithQuantifier(Box<Pattern>, Quantifier),
}

#[derive(Debug)]
pub enum Quantifier {
    ZeroOrOne,
    ZeroOrMore,
    OneOrMore,
    Literal(u64),
}

impl Pattern {
    pub fn matches(&self, s: &str) -> usize {
        match self {
            Self::PatternWithQuantifier(..) => todo!(),
            _ => self.char_matches(s),
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
}
