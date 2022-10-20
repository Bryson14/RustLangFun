#![cfg_attr(debug_assertions, allow(unused))]
use regex::Regex;

pub struct RegexBuddy {
    complete_pattern: String,
}

pub enum Quantity {
    NTimes(isize),
    OneOrMore,
    ZeroOrMore,
    NTimesOrMore(isize),
    ZeroOrOne,
    Range(isize, isize),
    Once,
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

    pub fn show_pattern(self) -> RegexFactory {
        println!("pattern>>{}<<", self.complete_pattern);
        self
    }

    fn push_pattern(&mut self, pattern: String) -> RegexFactory {
        self.complete_pattern.push_str(&pattern);
        RegexFactory {
            complete_pattern: self.complete_pattern.clone(),
        }
    }

    /// Adds a repeating sign on the end of the string. This is for DRY on special characters but putting just a
    /// plan word like `people` will cause unwanted errors. For example, if `pattern=people` and `quantity=Quantity::OneOrMore`
    /// will add `people+` to the complete pattern. this will match `people` and `peopleeeee` but not `peoplepeople`
    fn push_pattern_repeat(&mut self, pattern: String, quantity: Quantity) -> RegexFactory {
        let mut s = String::new();
        match quantity {
            Quantity::NTimes(n) => s.push_str(&format!("{}{{{}}}", pattern, n)),
            Quantity::OneOrMore => s.push_str(&format!("{}+", pattern)),
            Quantity::ZeroOrMore => s.push_str(&format!("{}*", pattern)),
            Quantity::NTimesOrMore(n) => s.push_str(&format!("{}{{{},}}", pattern, n)),
            Quantity::Once => s.push_str(&format!("{}", pattern)),
            Quantity::ZeroOrOne => s.push_str(&format!("{}?", pattern)),
            Quantity::Range(low, high) => {
                assert!(low < high);
                s.push_str(&format!("{}{{{},{}}}", pattern, low, high))
            }
        }
        self.complete_pattern.push_str(&s);
        RegexFactory {
            complete_pattern: self.complete_pattern.clone(),
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

    pub fn required_repeat(&mut self, pattern: &str, quantity: Quantity) -> RegexFactory {
        self.push_pattern_repeat(format!("({})", pattern), quantity)
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

    pub fn whitespace(&mut self, quantity: Quantity) -> RegexFactory {
        self.push_pattern_repeat("\\s".into(), quantity)
    }

    pub fn any_digit(&mut self, quantity: Quantity) -> RegexFactory {
        self.push_pattern_repeat("\\d".into(), quantity)
    }

    pub fn any_non_digit(&mut self, quantity: Quantity) -> RegexFactory {
        self.push_pattern_repeat("\\D".into(), quantity)
    }

    pub fn any_word_char(&mut self, quantity: Quantity) -> RegexFactory {
        self.push_pattern_repeat("\\w".into(), quantity)
    }

    pub fn any_non_word_char(&mut self, quantity: Quantity) -> RegexFactory {
        self.push_pattern_repeat("\\W".into(), quantity)
    }

    /// for a word boundary example, see [stackoverflow](https://stackoverflow.com/questions/1324676/what-is-a-word-boundary-in-regex)
    pub fn word_boundary(&mut self, quantity: Quantity) -> RegexFactory {
        self.push_pattern_repeat("\\b".into(), quantity)
    }

    pub fn non_word_boundary(&mut self, quantity: Quantity) -> RegexFactory {
        self.push_pattern_repeat("\\B".into(), quantity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn beginning_1_test() {
        let re = RegexBuddy::new()
            .begin_line()
            .required("adam")
            .compile()
            .unwrap();
        assert!(re.is_match("adam is cool"));
        assert!(!re.is_match("what adam is cool"));
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
            .required("a")
            .optional("1")
            .required("b")
            .compile()
            .unwrap();
        assert!(re.is_match("ab"));
        assert!(re.is_match("1ab1"));
        assert!(re.is_match("a1b"));
        assert!(!re.is_match("a1e"));
    }

    #[test]
    fn whitespace_1_test() {
        let re = RegexBuddy::new()
            .required("a")
            .whitespace(Quantity::NTimesOrMore(3))
            .required("b")
            .compile()
            .unwrap();
        assert!(!re.is_match("ab"));
        assert!(!re.is_match("a b"));
        assert!(!re.is_match("a  b"));
        assert!(re.is_match("a   b"));
        assert!(re.is_match("a    b"));
    }

    #[test]
    fn whitespace_2_test() {
        let re = RegexBuddy::new()
            .required("a")
            .whitespace(Quantity::NTimes((6)))
            .required("b")
            .compile()
            .unwrap();
        assert!(!re.is_match("ab"));
        assert!(!re.is_match("a b"));
        assert!(!re.is_match("a  b"));
        assert!(re.is_match("a      b"));
        assert!(!re.is_match("a       b"));
    }

    #[test]
    fn whitespace_3_test() {
        let re = RegexBuddy::new()
            .required("a")
            .whitespace(Quantity::OneOrMore)
            .required("b")
            .compile()
            .unwrap();
        assert!(!re.is_match("ab"));
        assert!(re.is_match("a b"));
        assert!(re.is_match("a  b"));
        assert!(re.is_match("a      b"));
        assert!(re.is_match("a       b"));
    }

    #[test]
    fn whitespace_4_test() {
        let re = RegexBuddy::new()
            .required("a")
            .whitespace(Quantity::ZeroOrMore)
            .required("b")
            .compile()
            .unwrap();
        assert!(re.is_match("ab"));
        assert!(re.is_match("a b"));
        assert!(re.is_match("a  b"));
        assert!(re.is_match("a      b"));
        assert!(re.is_match("a       b"));
    }

    #[test]
    fn whitespace_5_test() {
        let re = RegexBuddy::new()
            .required("a")
            .whitespace(Quantity::NTimesOrMore(0))
            .required("b")
            .compile()
            .unwrap();
        assert!(re.is_match("ab"));
        assert!(re.is_match("a b"));
        assert!(re.is_match("a  b"));
        assert!(re.is_match("a      b"));
        assert!(re.is_match("a       b"));
    }

    #[test]
    fn whitespace_6_test() {
        let re = RegexBuddy::new()
            .required("a")
            .whitespace(Quantity::Once)
            .required("b")
            .compile()
            .unwrap();
        assert!(!re.is_match("ab"));
        assert!(re.is_match("a b"));
        assert!(!re.is_match("a  b"));
    }

    #[test]
    fn required_2_test() {
        let re = RegexBuddy::new()
            .required_repeat("a", Quantity::Range(2, 4))
            .show_pattern()
            .required("b")
            .show_pattern()
            .compile()
            .unwrap();
        assert!(!re.is_match("ab"));
        assert!(re.is_match("aab"));
        assert!(re.is_match("aaab"));
        assert!(re.is_match("aaaab"));
        assert!(!re.is_match("aaaa"));
    }

    #[test]
    fn email_1_test() {
        let re = RegexBuddy::new()
            .required_repeat("a", Quantity::Range(2, 4))
            .show_pattern()
            .required("b")
            .show_pattern()
            .compile()
            .unwrap();
        assert!(!re.is_match("apples@abc.com"));
        assert!(re.is_match("aab"));
        assert!(re.is_match("aaab"));
        assert!(re.is_match("aaaab"));
        assert!(!re.is_match("aaaa"));
    }
}
