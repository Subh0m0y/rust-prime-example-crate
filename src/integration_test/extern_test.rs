extern crate prime_util;

use prime_util::prime::is_prime;

#[cfg(test)]
mod integration_test {
    #[test]
    fn example() {
        assert_eq!(is_prime(11), true);
        assert_eq!(is_prime(12), false);
        assert_eq!(is_prime(13), true);
    }
}
