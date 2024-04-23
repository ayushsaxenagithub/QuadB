// Implement a function that checks whether a given number is prime or not.

use std::io;

fn is_prime(num: i32) -> bool {
    if num <= 1 {
        return false;
    }
    if num <= 3 {
        return true;
    }
    if num % 2 == 0 || num % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= num {
        if num % i == 0 || num % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn main() {
    let mut input = String::new();
    println!("Enter a number to check if it is prime:");
    io::stdin().read_line(&mut input).unwrap();
    let num: i32 = input.trim().parse().expect("Please type a number!");

    if is_prime(num) {
        println!("{} is a prime number.", num);
    } else {
        println!("{} is not a prime number.", num);
    }
}