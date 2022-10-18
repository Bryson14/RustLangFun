use regex::Regex;

pub struct RegexBuddy {
    complete_pattern: String,
}

pub struct RegexFactory {
    complete_pattern: String,
}

impl RegexBuddy {
    pub fn new() -> RegexFactory {
        RegexFactory {
            complete_pattern: "".into(),
        }
    }
}

impl RegexFactory {
    pub fn compile(self) -> Result<Regex, regex::Error> {
        Regex::new(&self.complete_pattern)
    }

    fn push_pattern(&mut self, pattern: String) -> RegexFactory {
        let s = &mut self.complete_pattern;
        s.push_str(&pattern);
        RegexFactory {
            complete_pattern: s.to_string(),
        }
    }

    fn begin_pattern_with(&mut self, pattern: String) -> RegexFactory {
        let s = &mut self.complete_pattern;
        s.insert_str(0, &pattern);
        RegexFactory {
            complete_pattern: s.to_string(),
        }
    }

    pub fn optional(&mut self, pattern: &str) -> RegexFactory {
        self.push_pattern(format!("({})?", pattern))
    }

    pub fn required(&mut self, pattern: &str) -> RegexFactory {
        self.push_pattern(format!("{}", pattern))
    }

    pub fn capture_group(&mut self, pattern: &str) -> RegexFactory {
        self.push_pattern(format!("(?{})", pattern))
    }

    pub fn begin_line(&mut self) -> RegexFactory {
        self.begin_pattern_with("^".into())
    }

    pub fn end_line(&mut self) -> RegexFactory {
        self.push_pattern(format!("$"))
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn beginning_1_test() {
        let re = RegexBuddy::new()
            .begin_line()
            .required("bryson")
            .compile()
            .unwrap();
        assert!(re.is_match("bryson is cool"));
        assert!(!re.is_match("what bryson is cool"));
    }

    #[test]
    fn ending_1_test() {
        let re = RegexBuddy::new()
            .required("bryson")
            .end_line()
            .compile()
            .unwrap();
        assert!(!re.is_match("bryson is cool"));
        assert!(re.is_match("what bryson"));
    }

    #[test]
    fn optional_1_test() {
        let re = RegexBuddy::new()
            .required("abc")
            .optional("123")
            .required("def")
            .compile()
            .unwrap();
        assert!(re.is_match("abcdef"));
        assert!(re.is_match("123abcdef123"));
        assert!(re.is_match("abc123def"));
        assert!(!re.is_match("abc123ef"));
    }
}
