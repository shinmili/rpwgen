extern crate rand;

use rand::prelude::*;

fn generate(len: usize, choices: &[char]) -> String {
    let mut rng = thread_rng();
    (0..len)
        .map(|_| choices.choose(&mut rng).unwrap())
        .cloned()
        .collect()
}

fn main() {
    let mut args = std::env::args();
    args.next();
    let len = args.next().unwrap().parse().unwrap();
    let choices: Vec<char> = args.next().unwrap().chars().collect();
    println!("{}", generate(len, &choices[..]));
}
