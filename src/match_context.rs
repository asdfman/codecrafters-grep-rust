pub struct MatchContext<'a> {
    pub original_input: &'a str,
    pub input: &'a str,
    pub input_offset: usize,
}

impl<'a> MatchContext<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            original_input: input,
            input,
            input_offset: 0,
        }
    }

    pub fn set_input(&mut self, start: usize, end: usize) {
        self.input = &self.original_input[start..=end];
        self.input_offset = start;
    }

    pub fn reset_to(&mut self, offset: usize) {
        self.input_offset = offset;
        self.input = &self.original_input[offset..];
    }

    pub fn advance(&mut self, len: usize) {
        self.input = &self.input[len..];
        self.input_offset += len;
    }
}

#[derive(Debug, Clone, Default)]
pub struct Captures(pub Vec<Option<(usize, usize)>>);

impl Captures {
    pub fn new(capture_count: usize) -> Self {
        let captures = vec![None; capture_count];
        Self(captures)
    }
    pub fn capture(&mut self, start: usize, end: usize, index: usize) {
        if !self.0.is_empty() {
            self.0[index - 1] = Some((start, end));
        }
    }

    pub fn get_capture<'a>(&self, idx: usize, orig_input: &'a str) -> Option<&'a str> {
        if let Some((start, end)) = self.0[idx - 1] {
            Some(&orig_input[start..end])
        } else {
            None
        }
    }

    pub fn debug_print(&self, input: &str) {
        for (i, capture) in self.0.iter().enumerate() {
            match capture {
                Some((start, end)) => {
                    println!(
                        "Capture group {}: '{}' at [{}..{}]",
                        i + 1,
                        &input[*start..*end],
                        start,
                        end
                    );
                }
                None => {
                    println!("Capture group {}: None", i + 1);
                }
            }
        }
    }
}
