extern crate clap;
extern crate rand;
#[macro_use]
extern crate unic_char_range;

use clap::ArgMatches;
use rand::prelude::*;
use std::convert::From;

pub struct Config {
    choices: Vec<char>,
    length: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            choices: unfold_range_notation("0-9A-Za-z"),
            length: 16,
        }
    }
}

fn unfold_range_notation(s: &str) -> Vec<char> {
    let mut choices = vec![];
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if let Some(&maybe_hyphen) = chars.peek() {
            if maybe_hyphen == '-' {
                if let Some(d) = chars.nth(1) {
                    // /.-./
                    for c in chars!(c..=d) {
                        choices.push(c);
                    }
                } else {
                    // /.-$/
                    choices.push(c);
                    choices.push('-');
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
    return choices;
}

impl<'a> From<ArgMatches<'a>> for Config {
    fn from(matches: ArgMatches) -> Config {
        let mut config = Config::default();
        if let Some(choices) = matches.value_of("chars") {
            if !choices.is_empty() {
                config.choices = unfold_range_notation(choices)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unfold_range_notation() {
        assert_eq!(
            unfold_range_notation("abcABC012"),
            "abcABC012".chars().collect::<Vec<char>>()
        );
        assert_eq!(
            unfold_range_notation("a-g"),
            "abcdefg".chars().collect::<Vec<char>>()
        );
        assert_eq!(
            unfold_range_notation("a-gA-G"),
            "abcdefgABCDEFG".chars().collect::<Vec<char>>()
        );
        assert_eq!(
            unfold_range_notation("a-cde-g"),
            "abcdefg".chars().collect::<Vec<char>>()
        );
        assert_eq!(
            unfold_range_notation("e-gda-c"),
            "efgdabc".chars().collect::<Vec<char>>()
        );
    }
}
