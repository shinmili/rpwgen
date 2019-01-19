extern crate clap;
extern crate rand;

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
            choices: (b'0'..=b'9')
                .chain(b'A'..=b'Z')
                .chain(b'a'..=b'z')
                .map(char::from)
                .collect(),
            length: 16,
        }
    }
}

impl<'a> From<ArgMatches<'a>> for Config {
    fn from(matches: ArgMatches) -> Config {
        let mut config = Config::default();
        if let Some(choices) = matches.value_of("chars") {
            if !choices.is_empty() {
                config.choices = choices.chars().collect();
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
