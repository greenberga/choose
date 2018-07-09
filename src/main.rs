extern crate clap;
extern crate rand;

use clap::{Arg, App};
use rand::{thread_rng, Rng};
use std::io::{self, Read};

fn main() {
    let matches = App::new("choose")
        .version("0.1.0")
        .author("Aaron Greenberg <p@aaronjgreenberg.com>")
        .about("a command line tool for random selection from a list")
        .arg(Arg::with_name("count")
            .takes_value(true)
            .default_value("1")
            .help("number of items to choose"))
        .arg(Arg::with_name("delimiter")
            .short("d")
            .takes_value(true)
            .default_value("\n")
            .help("list delimiter"))
        .get_matches();
    let count = matches.value_of("count").unwrap().parse::<i32>().unwrap();
    let delim = matches.value_of("delimiter").unwrap();

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let choices: Vec<&str> = buffer.split(delim).collect();

    let mut rng = thread_rng();
    for _ in 0..count {
      println!("{}", rng.choose(&choices).unwrap());
    }
}
