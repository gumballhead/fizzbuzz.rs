extern crate fizzbuzz;

use fizzbuzz::fizzbuzz;

#[test]
fn test_one_is_one() {
    assert_eq!(fizzbuzz(1), "1")
}

#[test]
fn test_two_is_two() {
    assert_eq!(fizzbuzz(2), "2")
}

#[test]
fn test_three_is_fizz() {
    assert_eq!(fizzbuzz(3), "fizz")
}

#[test]
fn test_five_is_buzz() {
    assert_eq!(fizzbuzz(5), "buzz")
}

#[test]
fn test_fifteen_is_fizz_buzz() {
    assert_eq!(fizzbuzz(15), "fizzbuzz")
}

#[test]
fn test_fizzes() {
    let fizzes = (1..=100)
        .filter(|&n| n % 3 == 0 && n % 5 != 0)
        .map(fizzbuzz)
        .all(|it| it == "fizz");

    assert!(fizzes)
}

#[test]
fn test_buzzes() {
    let buzzes = (1..=100)
        .filter(|&n| n % 5 == 0 && n % 3 != 0)
        .map(fizzbuzz)
        .all(|it| it == "buzz");

    assert!(buzzes)
}

#[test]
fn test_fizzbuzzes() {
    let fizzbuzzes = (1..=100)
        .filter(|&n| n % 3 == 0 && n % 5 == 0)
        .map(fizzbuzz)
        .all(|it| it == "fizzbuzz");

    assert!(fizzbuzzes)
}
