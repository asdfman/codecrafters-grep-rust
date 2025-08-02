use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, regex: Regex) -> bool {
    regex.matches(input_line)
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    let regex: Regex = pattern.as_str().into();
    dbg!(&regex);

    if match_pattern(&input_line, regex) {
        println!("Match found: {}", input_line.trim());
        process::exit(0)
    } else {
        println!("Match not found: {}", input_line.trim());
        process::exit(1)
    }
}

#[derive(Debug)]
enum Regex {
    Exact(String),
    Digit,
    Alphanumeric,
}

impl From<&str> for Regex {
    fn from(s: &str) -> Self {
        match s {
            r"\d" => Self::Digit,
            r"\w" => Self::Alphanumeric,
            _ => Self::Exact(s.into()),
        }
    }
}

impl Regex {
    fn matches(&self, input: &str) -> bool {
        match self {
            Self::Exact(s) => input.contains(s),
            Self::Digit => input.chars().any(|c| c.is_ascii_digit()),
            Self::Alphanumeric => input.chars().any(|c| c.is_ascii_alphanumeric()),
        }
    }
}
