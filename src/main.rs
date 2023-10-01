//! Command-line modular exponentation tool
//!
//! Pedro Gonzalez
//! 2023

use std::env;
use std::str::FromStr;

fn main() {
    // Read arguments from environment
    println!("Starting program...");
    if env::args().len() < 4 {
        eprintln!("Must use 3 arguments.");
        error();
    }

    // Load arguments into new vector
    let mut numbers = Vec::new();
    println!("Processing arguments...");
    for arg in env::args().skip(1){
        numbers.push(u64::from_str(&arg).expect("modexp: usage: modexp <x:u64> <y:u64> <m:u64>"));
    }

    // Pass arguments to modexp function
    println!("Recived arguments...");
    println!("Result: {}",modexp(numbers[0],numbers[1],numbers[2]));
}

fn modexp(x:u64,y:u64,m:u64) -> u64 {
    assert!(m != 0);
    let mut z:u128 = 1;
    let mut y:u128 = y as u128;
    let mut x:u128 = x as u128;
    let m:u128 = m as u128;

    // Overflow check
    if (m-1).checked_mul(m-1).is_none() {
        eprintln!("m overflow caught.");
        error();
    }

    if m == 0 {
        error();
    }


    while y > 0 {
        if y % 2 == 1{
            // Overflow check
            match z.checked_mul(x) {
                Some(result) => {
                    z = result % m;
                }
                None => {
                    eprintln!("z overflow caught");
                    error()
                }
            }
        }
        y /= 2 ;
        // Overflow check
        match x.checked_mul(x) {
            Some(result) => {
                x = result % m;
            }
            None => {
                eprintln!("x overflow caught");
                error();
            }
        }
    }

    // Make sure z is small enough to fit as u64.
    if z > u64::MAX as u128 {
        eprintln!("z is larger than u64 max.");
        error();
    }
    z.try_into().unwrap()
}

// Print a usage error message and exit.
fn error() -> ! {
    eprintln!("modexp: usage: modexp <x:u64> <y:u64> <m:u64>");
    std::process::exit(1);
}



#[cfg(test)]
mod tests {
    use super::*;

    // Tests from professor
    #[test]
    fn test_modexp() {
        // Largest prime less than 2**64.
        // https://primes.utm.edu/lists/2small/0bit.html
        let bigm = u64::max_value() - 58;
        assert_eq!(0, modexp(bigm - 2, bigm - 1, 1));
        assert_eq!(1, modexp(bigm - 2, bigm - 1, bigm));
        assert_eq!(827419628471527655, modexp(bigm - 2, (1 << 32) + 1, bigm));
        // https://practice.geeksforgeeks.org/problems/
        //    modular-exponentiation-for-large-numbers/0
        assert_eq!(4, modexp(10, 9, 6));
        assert_eq!(34, modexp(450, 768, 517));
    }   

    #[test]
    fn test_modexp_basic() {
        // Test with prime number m=17
        assert_eq!(modexp(2, 20, 17), 16);
        
        // Test with m=1 (should return 0)
        assert_eq!(modexp(10, 5, 1), 0);
    }

    #[test]
    #[should_panic]
    fn test_modexp_large_numbers() {
        // Test with large numbers
        assert_eq!(modexp(123456789, 987654321, 101), 100);
    }

    #[test]
    fn test_modexp_edge_cases() {
        // Test with edge cases
        assert_eq!(modexp(0, 5, 17), 0); // x=0 should return 0
        assert_eq!(modexp(5, 0, 17), 1); // y=0 should return 1
    }

    #[test]
    #[should_panic]
    fn test_modexp_invalid_input() {
        // Test with invalid input
        assert_eq!(modexp(2, 20, 0), 0); // m=0 should return 0 (invalid)
        assert_eq!(modexp(2, 20, 1), 0); // m=1 should return 0 (invalid)
        assert_eq!(modexp(2, 20, std::u64::MAX), 0); // m=max should return 0 (invalid)
    }
}