pub mod prime;

use std::env;
use std::io::stdin;

fn take_input() {
    println!("Prime cheker utility.\n=====================\n");
    loop {
        process_single_line();
        if user_wants_to_exit() {
            break;
        }
    }
}

fn process_single_line() {
    let mut num_str: String = String::new();

    println!("Enter the number to check : ");

    stdin().read_line(&mut num_str).unwrap();

    process_string(num_str.trim());
}

fn user_wants_to_exit() -> bool {
    let mut usr_str = String::new();

    println!("Do you want to exit? (y/n) : ");
    stdin()
        .read_line(&mut usr_str)
        .expect("Error while reading input.");

    let trimmed = usr_str.trim();

    trimmed == "y" || trimmed == "Y" || trimmed.to_lowercase() == "yes"
}

fn process_string(num_str: &str) {
    let num = num_str.parse::<u64>().expect(INVALID_NUMBER);

    println!(
        "The integer {} is{} a prime.",
        num,
        match prime::is_prime(num) {
            true => "",
            false => " not",
        }
    );
}

const HELP_TEXT: &str = "USAGE:\n\n1. prime\n2. prime [unsigned integer]\n";
const INVALID_NUMBER: &str = "Please enter a valid unsigned integer.";

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => take_input(),
        2 => process_string(args[1].trim()),
        _ => {
            println!("{}", HELP_TEXT);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::prime::*;
    #[test]
    fn first_10_numbers_tested() {
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(6), false);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(8), false);
        assert_eq!(is_prime(9), false);
        assert_eq!(is_prime(10), false);
    }
}
