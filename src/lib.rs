pub fn fizzbuzz(n: usize) -> String {
    let fizz = n % 3 == 0;
    let buzz = n % 5 == 0;

    if fizz && buzz {
        return "fizzbuzz".to_string()
    } else if fizz {
        return "fizz".to_string()
    } else if buzz {
        return "buzz".to_string()
    } else {
        return n.to_string()
    }
}
