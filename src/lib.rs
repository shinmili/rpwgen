extern crate clap;
extern crate rand;
#[macro_use]
extern crate unic_char_range;

use clap::ArgMatches;
use rand::prelude::*;

pub struct Config {
    choices: Vec<char>,
    length: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            choices: unfold_range_notation("0-9A-Za-z").unwrap(),
            length: 16,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum UnfoldError {
    StartsWithHyphen,
    EndsWithHyphen,
    EndsWithBackSlash,
}

fn unfold_range_notation(s: &str) -> Result<Vec<char>, UnfoldError> {
    let mut choices = vec![];
    let mut chars = s.chars().peekable();

    if chars.peek() == Some(&'-') {
        return Err(UnfoldError::StartsWithHyphen);
    }

    while let Some(c) = Escape::from(chars.by_ref()).next() {
        if c.is_err() {
            return Err(UnfoldError::EndsWithBackSlash);
        }
        let c = c.unwrap();
        if let Some(&maybe_hyphen) = chars.peek() {
            if maybe_hyphen == '-' {
                if let Some(d) = Escape::from(chars.by_ref()).nth(1) {
                    if d.is_err() {
                        return Err(UnfoldError::EndsWithBackSlash);
                    }
                    let d = d.unwrap();
                    // /.-./
                    for c in chars!(c..=d) {
                        choices.push(c);
                    }
                } else {
                    // /.-$/
                    return Err(UnfoldError::EndsWithHyphen);
                }
            } else {
                // /.[^-]/
                choices.push(c);
            }
        } else {
            // /.$/
            choices.push(c);
        }
    }
    return Ok(choices);
}

impl<'a> From<ArgMatches<'a>> for Config {
    fn from(matches: ArgMatches) -> Config {
        let mut config = Config::default();
        if let Some(choices) = matches.value_of("chars") {
            if !choices.is_empty() {
                if let Ok(choices) = unfold_range_notation(choices) {
                    config.choices = choices;
                }
                // TODO: Invalid notation should be reported. Currently it's just ignored.
            }
        }
        if let Some(length_str) = matches.value_of("length") {
            if let Ok(length) = length_str.parse() {
                config.length = length;
            }
        }
        config
    }
}

pub fn generate(config: &Config) -> String {
    let mut rng = thread_rng();
    (0..config.length)
        .map(|_| config.choices.choose(&mut rng).unwrap())
        .cloned()
        .collect()
}

struct Escape<T> {
    inner: T,
}

impl<T> From<T> for Escape<T> {
    fn from(inner: T) -> Escape<T> {
        Escape { inner }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct EscapeError;
impl<T> Iterator for Escape<T>
where
    T: Iterator<Item = char>,
{
    type Item = Result<char, EscapeError>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some('\\') => self.inner.next().map(Ok).or(Some(Err(EscapeError))),
            c => c.map(Ok),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_vec_char(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    #[test]
    fn test_successful_unfold() {
        assert_eq!(
            unfold_range_notation("abcABC012"),
            Ok(to_vec_char("abcABC012"))
        );
        assert_eq!(unfold_range_notation("a-g"), Ok(to_vec_char("abcdefg")));
        assert_eq!(
            unfold_range_notation("a-gA-G"),
            Ok(to_vec_char("abcdefgABCDEFG"))
        );
        assert_eq!(unfold_range_notation("a-cde-g"), Ok(to_vec_char("abcdefg")));
        assert_eq!(unfold_range_notation("e-gda-c"), Ok(to_vec_char("efgdabc")));
    }

    #[test]
    fn test_successful_unfold_with_escape() {
        assert_eq!(unfold_range_notation(r"\\"), Ok(vec!['\\']));
        assert_eq!(unfold_range_notation(r"\-"), Ok(vec!['-']));
        assert_eq!(
            unfold_range_notation(r#"0-2A-Ca-c\-_"#),
            Ok(to_vec_char("012ABCabc-_"))
        );
    }

    #[test]
    fn test_syntax_error() {
        assert_eq!(
            unfold_range_notation("-Za-z0-9"),
            Err(UnfoldError::StartsWithHyphen)
        );
        assert_eq!(
            unfold_range_notation("A-Za-z0-"),
            Err(UnfoldError::EndsWithHyphen)
        );
        assert_eq!(
            unfold_range_notation(r"A-Z/\"),
            Err(UnfoldError::EndsWithBackSlash)
        );
    }
}
