/// This function takes a 64-bit unsigned integer and checks if it is a prime.
///
/// If the number is prime, `true` is returned, and vice-versa.
///
/// #Example
///
/// ```rust
/// use prime_util::*;
///
/// let result = prime::is_prime(31);
/// assert_eq!(result, true);
/// ```
pub fn is_prime(num: u64) -> bool {
    // Simple divisor counting apprrach
    let mut count = 0;
    for divisor in 1..(num + 1) {
        if num % divisor == 0 {
            count += 1;
        }
    }
    count == 2
}
