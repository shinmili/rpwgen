#[macro_use]
extern crate clap;
extern crate rpwgen;

use clap::App;
use rpwgen::generate;

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
    println!("{}", generate(&matches.into()));
}
