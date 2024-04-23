// Implement a function that checks whether a given string is a palindrome or not.


use std::io::{self, Write};

fn is_palindrome(s: &str) -> bool {
    s.bytes().eq(s.bytes().rev())
}

fn main() {
    let mut input = String::new();
    print!("Enter a string:");
    io::stdout().flush().unwrap();  // Move this line here before reading input
    io::stdin().read_line(&mut input).unwrap();
    let trimmed_input = input.trim();

    if is_palindrome(trimmed_input) {
        println!("'{}' is a palindrome.", trimmed_input);
    } else {
        println!("'{}' is not a palindrome.", trimmed_input);
    }
}
