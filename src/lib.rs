pub fn is_fizz(n: usize) -> bool {
    n % 3 == 0
}

pub fn is_buzz(n: usize) -> bool {
    n % 5 == 0
}

pub fn fizzbuzz(n: usize) -> String {
    let fizz = is_fizz(n);
    let buzz = is_buzz(n);

    if fizz && buzz {
        "fizzbuzz".to_string()
    } else if fizz {
        "fizz".to_string()
    } else if buzz {
        "buzz".to_string()
    } else {
        n.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::is_fizz;
    use super::is_buzz;

    #[test]
    fn test_fizz() {
        assert!(is_fizz(3))
    }

    #[test]
    fn test_not_fizz() {
        assert!(!is_fizz(4))
    }


    #[test]
    fn test_buzz() {
        assert!(is_buzz(5))
    }


    #[test]
    fn test_not_buzz() {
        assert!(!is_buzz(4))
    }
}
