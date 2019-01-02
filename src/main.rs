#[macro_use]
extern crate clap;
extern crate rand;

use clap::App;
use rand::prelude::*;

fn generate(len: usize, choices: &[char]) -> String {
    let mut rng = thread_rng();
    (0..len)
        .map(|_| choices.choose(&mut rng).unwrap())
        .cloned()
        .collect()
}

fn main() {
    let matches = App::new("Rpwgen")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("Generates some passwords")
        .args_from_usage(
            "-c --chars=[CHARS] 'Characters used in passwords'
             [length] 'Password\'s length (in Unicode code points)'",
        )
        .get_matches();
    let len = matches
        .value_of("length")
        .map(|s| s.parse().unwrap())
        .unwrap();
    let choices: Vec<char> = matches.value_of("chars").unwrap().chars().collect();
    println!("{}", generate(len, &choices[..]));
}
