use std::env::args;

mod lib;
use lib::fizzbuzz;

fn main() {
    let n: usize = args().nth(1)
        .and_then(|n| n.parse().ok())
        .unwrap_or(100);

    for result in (1..=n).map(fizzbuzz) {
        println!("{}", result);
    }
}
