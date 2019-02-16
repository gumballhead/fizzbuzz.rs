mod lib;
use lib::fizzbuzz;

fn main() {
    for n in 1..=100 {
        println!("{}", fizzbuzz(n))
    }
}
